use gpui::*;

actions!(quick_note, [QuickNew]);

#[derive(Default)]
struct QuickWindows(Vec<WindowHandle<crate::app::Workspace>>);
impl Global for QuickWindows {}

pub fn open(cx: &mut App) { open_document(None, cx); }

#[cfg(target_os = "macos")]
#[derive(Default)]
struct LastDocument(Option<String>);
#[cfg(target_os = "macos")]
impl Global for LastDocument {}
#[cfg(target_os = "macos")]
pub fn used(id: &str, cx: &mut App) { cx.set_global(LastDocument(Some(id.into()))); }
#[cfg(not(target_os = "macos"))]
pub fn used(_: &str, _: &mut App) {}
#[cfg(target_os = "macos")]
pub fn open_last(cx: &mut App) {
    let Some(mut store) = cx.try_global::<crate::shared_store::SharedStore>().cloned() else { return };
    let entries = store.list();
    let id = cx.try_global::<LastDocument>().and_then(|s| s.0.clone()).filter(|id| entries.iter().any(|e| &e.id == id))
        .or_else(|| entries.iter().max_by_key(|e| e.meta.opened).map(|e| e.id.clone()));
    let Some(id) = id else { return };
    for handle in cx.windows() {
        if let Some(handle) = handle.downcast::<crate::app::Workspace>()
            && handle.update(cx, |w, window, _| {
                if w.current.as_ref() != Some(&id) { return false; }
                window.activate_window(); true
            }).unwrap_or(false)
        { cx.activate(true); return; }
    }
    open_document(Some(id), cx);
}

pub fn open_document(document: Option<String>, cx: &mut App) {
    let stored = cx.try_global::<crate::shared_store::SharedStore>().and_then(|s| s.settings().quick_note_size);
    let remembered = cx.try_global::<Sizes>().and_then(|s| s.quick).or(stored).unwrap_or([640., 800.]);
    let mut previous = None;
    let handles = cx.try_global::<QuickWindows>().map(|w| w.0.clone()).unwrap_or_default();
    let mut alive = Vec::new();
    for handle in handles {
        if let Ok(b) = handle.update(cx, |_, window, cx| (window.bounds(), window.display(cx))) {
            previous = Some(b);
            alive.push(handle);
        }
    }
    cx.set_global(QuickWindows(alive));
    let display = previous.as_ref().and_then(|(_, d)| d.clone()).or_else(|| cx.primary_display());
    let mut bounds = Bounds::centered(display.as_ref().map(|d| d.id()), size(px(remembered[0].max(480.)), px(remembered[1].max(480.))), cx);
    if let Some(display) = display {
        let screen = display.bounds();
        bounds.size.width = bounds.size.width.min(screen.size.width - px(32.));
        bounds.size.height = bounds.size.height.min(screen.size.height - px(64.));
        bounds.origin = screen.origin + point((screen.size.width - bounds.size.width) / 2., (screen.size.height - bounds.size.height) / 2.);
        if let Some((last, _)) = previous {
            let next = last.origin + point(px(28.), px(28.));
            if next.x + bounds.size.width <= screen.right() - px(8.) && next.y + bounds.size.height <= screen.bottom() - px(8.) { bounds.origin = next; }
        }
    }
    persist(cx);
    let result = cx.open_window(WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        window_background: crate::theme::window_background(),
        titlebar: Some(TitlebarOptions {
            title: Some(crate::settings::text("Odyssey · 快速笔记").into()),
            appears_transparent: cfg!(any(target_os = "macos", target_os = "windows")),
            traffic_light_position: Some(point(px(20.), px(crate::app::HEADER_H / 2. - 7.))),
        }),
        window_min_size: Some(size(px(480.), px(480.))),
        ..Default::default()
    }, |window, cx| cx.new(|cx| crate::app::Workspace::new_document_window(document, window, cx)));
    match result {
        Ok(handle) => { cx.global_mut::<QuickWindows>().0.push(handle); cx.activate(true); },
        Err(e) => eprintln!("无法打开快速笔记：{e}"),
    }
}

#[cfg(target_os = "macos")]
mod hotkey {
    use super::*;
    use std::{cell::RefCell, ffi::c_void, ptr};
    type Ref = *mut c_void;
    #[repr(C)] struct EventType { class: u32, kind: u32 }
    #[repr(C)] struct HotkeyId { signature: u32, id: u32 }
    #[link(name = "Carbon", kind = "framework")]
    unsafe extern "C" {
        fn GetApplicationEventTarget() -> Ref;
        fn GetEventKind(event: Ref) -> u32;
        fn GetEventParameter(event: Ref, name: u32, kind: u32, actual: *mut u32, size: usize, actual_size: *mut usize, data: Ref) -> i32;
        fn InstallEventHandler(target: Ref, handler: unsafe extern "C" fn(Ref, Ref, Ref) -> i32, count: u32, types: *const EventType, data: Ref, out: *mut Ref) -> i32;
        fn RegisterEventHotKey(key: u32, modifiers: u32, id: HotkeyId, target: Ref, options: u32, out: *mut Ref) -> i32;
        fn RemoveEventHandler(handler: Ref) -> i32;
        fn UnregisterEventHotKey(key: Ref) -> i32;
    }
    thread_local! { static KEY: RefCell<(Ref, String)> = const { RefCell::new((ptr::null_mut(), String::new())) }; static LAST_KEY: RefCell<Ref> = const { RefCell::new(ptr::null_mut()) }; static HELD: RefCell<[bool; 2]> = const { RefCell::new([false; 2]) }; static APP: RefCell<Option<AsyncApp>> = const { RefCell::new(None) }; }
    unsafe extern "C" fn pressed(_: Ref, event: Ref, _: Ref) -> i32 {
        let mut id = HotkeyId { signature: 0, id: 0 };
        if unsafe { GetEventParameter(event, u32::from_be_bytes(*b"----"), u32::from_be_bytes(*b"hkid"), ptr::null_mut(), std::mem::size_of::<HotkeyId>(), ptr::null_mut(), &mut id as *mut _ as Ref) } != 0 || id.signature != u32::from_be_bytes(*b"FlNt") || !(1..=2).contains(&id.id) { return -9874; }
        let release = unsafe { GetEventKind(event) } == 6;
        let repeated = HELD.with(|held| { let mut held = held.borrow_mut(); let old = held[(id.id - 1) as usize]; held[(id.id - 1) as usize] = !release; old });
        if release || repeated { return 0; }
        APP.with(|app| {
            if let Some(cx) = app.borrow().clone() {
                cx.foreground_executor().clone().spawn(async move { let _ = cx.update(|cx| if id.id == 2 { super::open_last(cx) } else { super::open(cx) }); }).detach();
            }
        });
        0
    }
    pub fn suspend() { HELD.with(|h| *h.borrow_mut() = [false; 2]); LAST_KEY.with(|k| { let mut k = k.borrow_mut(); if !k.is_null() { unsafe { UnregisterEventHotKey(*k); } } *k = ptr::null_mut(); }); KEY.with(|k| { let mut k = k.borrow_mut(); if !k.0.is_null() { unsafe { UnregisterEventHotKey(k.0); } } *k = (ptr::null_mut(), String::new()); }); }
    pub fn install(cx: &mut App, code: u32, modifiers: u32, combo: &str) -> Result<(), i32> {
        if KEY.with(|k| k.borrow().1 == combo) { return Ok(()); }
        unsafe {
            let target = GetApplicationEventTarget();
            let events = [EventType { class: u32::from_be_bytes(*b"keyb"), kind: 5 }, EventType { class: u32::from_be_bytes(*b"keyb"), kind: 6 }];
            let mut handler = ptr::null_mut();
            let status = if APP.with(|a| a.borrow().is_none()) { InstallEventHandler(target, pressed, 2, events.as_ptr(), ptr::null_mut(), &mut handler) } else { 0 };
            if status != 0 { return Err(status); }
            let mut key = ptr::null_mut();
            // Carbon virtual key 49 = Space; optionKey = 1 << 11.
            let status = RegisterEventHotKey(code, modifiers, HotkeyId { signature: u32::from_be_bytes(*b"FlNt"), id: 1 }, target, 0, &mut key);
            if status != 0 { if !handler.is_null() { RemoveEventHandler(handler); } return Err(status); }
            KEY.with(|k| { let mut k = k.borrow_mut(); if !k.0.is_null() { UnregisterEventHotKey(k.0); } *k = (key, combo.into()); });
            APP.with(|app| *app.borrow_mut() = Some(cx.to_async()));
            let status = LAST_KEY.with(|k| {
                let mut k = k.borrow_mut();
                if !k.is_null() { return 0; }
                RegisterEventHotKey(49, (1 << 8) | (1 << 11), HotkeyId { signature: u32::from_be_bytes(*b"FlNt"), id: 2 }, target, 0, &mut *k)
            });
            if status != 0 { return Err(status); }
            Ok(())
        }
    }
}

pub fn install(cx: &mut App) -> bool {
    cx.on_action(|_: &QuickNew, cx| open(cx));
    // Settings › 快速笔记 shows a failure; no alert on every launch.
    crate::settings::prefs().quick_hotkey && register(cx)
}

static HOTKEY_ERROR: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
/// Why the global hotkey isn't registered, if it failed.
pub fn hotkey_error() -> Option<String> { HOTKEY_ERROR.lock().unwrap().clone() }
/// Register the global hotkey from the shortcuts file.
pub fn register(cx: &mut App) -> bool {
    let r = set_hotkey(&crate::shortcuts::global_key(), cx);
    if let Err(e) = &r { eprintln!("全局快捷键注册失败：{e}"); }
    r.is_ok()
}

/// Window sizes seen this session (quick note and main), saved on quit.
#[derive(Clone, Copy, Default)]
struct Sizes { quick: Option<[f32; 2]>, main: Option<[f32; 2]> }
impl Global for Sizes {}
pub fn remember(window: &Window, quick: bool, cx: &mut App) {
    if let WindowBounds::Windowed(bounds) = window.window_bounds() {
        let s = [f32::from(bounds.size.width), f32::from(bounds.size.height)];
        let mut sizes = cx.try_global::<Sizes>().copied().unwrap_or_default();
        if quick { sizes.quick = Some(s) } else { sizes.main = Some(s) }
        cx.set_global(sizes);
    }
}
pub fn persist(cx: &mut App) {
    let (Some(sizes), Some(store)) = (cx.try_global::<Sizes>().copied(), cx.try_global::<crate::shared_store::SharedStore>()) else { return };
    let mut settings = store.settings();
    if sizes.quick.is_some() { settings.quick_note_size = sizes.quick }
    if sizes.main.is_some() { settings.window_size = sizes.main }
    if let Err(e) = store.save_settings(&settings) { eprintln!("无法保存窗口尺寸：{e}"); }
}

/// The main window's size: last used, or 1280 × 800.
pub fn main_window_size(cx: &App) -> gpui::Size<Pixels> {
    let s = cx.global::<crate::shared_store::SharedStore>().settings().window_size.unwrap_or([1280., 800.]);
    size(px(s[0].max(480.)), px(s[1].max(480.)))
}

#[cfg(target_os = "macos")]
pub fn set_hotkey(combo: &str, cx: &mut App) -> anyhow::Result<()> {
    let key = Keystroke::parse(combo)?;
    let keys = [("a",0),("s",1),("d",2),("f",3),("h",4),("g",5),("z",6),("x",7),("c",8),("v",9),("b",11),("q",12),("w",13),("e",14),("r",15),("y",16),("t",17),("1",18),("2",19),("3",20),("4",21),("6",22),("5",23),("9",25),("7",26),("8",28),("0",29),("o",31),("u",32),("i",34),("p",35),("l",37),("j",38),("k",40),("n",45),("m",46),("space",49)];
    let code = keys.iter().find(|(k,_)| *k == key.key).map(|(_,v)| *v).ok_or_else(|| anyhow::anyhow!(crate::settings::tr("全局快捷键请使用字母、数字或空格", "Use a letter, digit or Space for the global shortcut")))?;
    let m = key.modifiers;
    let modifiers = (if m.platform { 1 << 8 } else { 0 }) | (if m.shift { 1 << 9 } else { 0 }) | (if m.alt { 1 << 11 } else { 0 }) | (if m.control { 1 << 12 } else { 0 });
    #[cfg(target_os = "macos")]
    let r = hotkey::install(cx, code, modifiers, combo).map_err(|e| anyhow::anyhow!("{}（{e}）", crate::settings::tr("无法注册这个组合键，可能已被系统或其他应用占用", "This shortcut is taken by macOS or another app")));
    *HOTKEY_ERROR.lock().unwrap() = r.as_ref().err().map(|e| e.to_string());
    r
}

pub fn suspend_hotkey() { #[cfg(target_os = "macos")] hotkey::suspend(); *HOTKEY_ERROR.lock().unwrap() = None; }

#[cfg(not(target_os = "macos"))]
pub fn set_hotkey(_: &str, _: &mut App) -> anyhow::Result<()> {
    let error = crate::settings::tr("当前系统暂不支持全局快捷键", "Global shortcuts are not supported on this platform yet");
    *HOTKEY_ERROR.lock().unwrap() = Some(error.into());
    Err(anyhow::anyhow!(error))
}
