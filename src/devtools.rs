//! Scripted UI driver for development: `FDL_SCRIPT=steps.txt` replays input
//! into the window and `shot <name>` saves PNGs of the window to `FDL_SHOTS`
//! (default /tmp). One step per line:
//!   wait <ms> | key <keystroke> | type <text> | move x y | click x y |
//!   dclick x y | rclick x y | drag x1 y1 x2 y2 | shot <name> | burst <name> n |
//!   mark <text> (IME composition) | commit <text> | nkey <letter> (a real
//!   key press through AppKit, so the active input method sees it) |
//!   activate (make the window key; takes focus) | quit |
//!   pick <path>[|<path>...] (answers the next file dialog instead of showing it)
//! For profiling: bench <n> <step> (runs the step n times, drawing a frame
//!   synchronously after each, and prints step / frame / editor-render times) |
//!   wheel x y dy (one scroll-wheel event).
//! For demo videos: glide x y ms (eased pointer move from the last position) |
//!   scroll x y dy ms (eased wheel scroll) | record <file.mp4> | record stop |
//!   caption <text> (a timed note in the pointer track, for titles).
//!   A recording is 30 fps through ffmpeg (`FDL_RECORD_FPS` overrides). Frames are rendered offscreen, so
//!   they have no cursor, traffic lights or rounded corners; the pointer track
//!   goes to `<file>.cursor` for compositing those in afterwards.

use gpui::*;
use std::sync::Mutex;
use std::time::{Duration, Instant};

fn fps() -> f64 {
    std::env::var("FDL_RECORD_FPS").ok().and_then(|v| v.parse().ok()).filter(|&f: &f64| f > 0.).unwrap_or(30.)
}

/// Time spent in the editor's `render` during the last benched frame.
pub static RENDER_NS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Last pointer position, and the cursor log of the running recording.
static POINTER: Mutex<(f32, f32)> = Mutex::new((640., 400.));
static CURSOR_LOG: Mutex<Option<(Instant, std::fs::File)>> = Mutex::new(None);
static PICK: Mutex<Option<Vec<std::path::PathBuf>>> = Mutex::new(None);

/// The scripted answer to the next open / save dialog, if any.
pub fn take_pick() -> Option<Vec<std::path::PathBuf>> {
    PICK.lock().unwrap().take()
}

fn log_pointer(x: f32, y: f32, what: &str) {
    use std::io::Write;
    *POINTER.lock().unwrap() = (x, y);
    if let Some((start, f)) = CURSOR_LOG.lock().unwrap().as_mut() {
        let _ = writeln!(f, "{:.4} {x:.1} {y:.1} {what}", start.elapsed().as_secs_f64());
    }
}

fn ease(t: f32) -> f32 {
    if t < 0.5 { 4. * t * t * t } else { 1. - (-2. * t + 2.).powi(3) / 2. }
}

pub fn start(window: &mut Window, cx: &mut App) {
    let Ok(path) = std::env::var("FDL_SCRIPT") else { return };
    let Ok(script) = std::fs::read_to_string(&path) else {
        eprintln!("devtools: cannot read {path}");
        return;
    };
    let handle = window.window_handle();
    #[cfg(target_os = "macos")]
    let Some(ns_view) = ns_view_of(window) else { return };
    #[cfg(target_os = "macos")]
    native::no_app_nap();
    cx.spawn(async move |cx| {
        cx.background_executor().timer(Duration::from_millis(800)).await;
        #[cfg(target_os = "macos")]
        if std::env::var_os("FDL_NOPARK").is_none() {
            let mut parked = false;
            for origin in native::park_candidates(ns_view) {
                native::place(ns_view, origin);
                cx.background_executor().timer(Duration::from_millis(250)).await;
                if native::is_visible(ns_view) {
                    parked = true;
                    break;
                }
            }
            if !parked {
                eprintln!("devtools: no visible off-screen spot; window may stop rendering");
            }
        }
        let mut recording: Option<std::rc::Rc<std::cell::Cell<bool>>> = None;
        // Only leading whitespace is insignificant: `type ab ` types a trailing space.
        for line in script.lines().map(str::trim_start).filter(|l| !l.trim().is_empty() && !l.starts_with('#')) {
            let (cmd, rest) = line.split_once(' ').unwrap_or((line, ""));
            if cmd == "wait" {
                // Draw invalidated frames ourselves: the display link stops
                // while the screen is locked, and nothing would render.
                let end = Instant::now() + Duration::from_millis(rest.trim().parse().unwrap_or(100));
                while Instant::now() < end {
                    cx.background_executor().timer(Duration::from_millis(16).min(end - Instant::now())).await;
                    let _ = handle.update(cx, |_, window, cx| {
                        if window.needs_draw() {
                            window.draw(cx).clear();
                        }
                    });
                }
                continue;
            }
            // `burst name n`: n captures ~8ms apart, to catch transient frames.
            if cmd == "burst" {
                let (name, n) = rest.trim().split_once(' ').unwrap_or((rest.trim(), "10"));
                let dir = std::env::var("FDL_SHOTS").unwrap_or_else(|_| "/tmp".into());
                for i in 0..n.trim().parse().unwrap_or(10) {
                    let path = format!("{dir}/{name}-{i:02}.png");
                    let _ = handle.update(cx, |_, window, cx| capture(window, cx, &path));
                    cx.background_executor().timer(Duration::from_millis(8)).await;
                }
                continue;
            }
            if cmd == "bench" {
                let (n, step) = rest.trim().split_once(' ').unwrap_or((rest.trim(), "noop"));
                let (scmd, srest) = step.split_once(' ').unwrap_or((step, ""));
                let mut samples = vec![];
                for _ in 0..n.parse().unwrap_or(20) {
                    let _ = handle.update(cx, |_, window, cx| {
                        let t0 = Instant::now();
                        run(scmd, srest, window, cx);
                        let t1 = Instant::now();
                        RENDER_NS.store(0, std::sync::atomic::Ordering::Relaxed);
                        window.draw(cx).clear();
                        let render = RENDER_NS.load(std::sync::atomic::Ordering::Relaxed) as f64 / 1e6;
                        samples.push(((t1 - t0).as_secs_f64() * 1e3, t1.elapsed().as_secs_f64() * 1e3, render));
                    });
                    cx.background_executor().timer(Duration::from_millis(10)).await;
                }
                let stat = |f: fn(&(f64, f64, f64)) -> f64| {
                    let mut v: Vec<f64> = samples.iter().map(f).collect();
                    v.sort_by(f64::total_cmp);
                    let at = |q: f64| v.get(((v.len() as f64 - 1.) * q).round() as usize).copied().unwrap_or(0.);
                    format!("p50 {:.2} p95 {:.2} max {:.2}", at(0.5), at(0.95), at(1.))
                };
                eprintln!(
                    "bench {step:?} x{}: step [{}] frame [{}] editor render [{}] ms",
                    samples.len(),
                    stat(|s| s.0),
                    stat(|s| s.1),
                    stat(|s| s.2)
                );
                continue;
            }
            let quit = cmd == "quit";
            if cmd == "record" || (quit && recording.is_some()) {
                if let Some(stop) = recording.take() {
                    stop.set(true);
                    *CURSOR_LOG.lock().unwrap() = None;
                    // Lets the writer drain and ffmpeg finish the file.
                    cx.background_executor().timer(Duration::from_millis(300)).await;
                }
                if cmd == "record" && rest.trim() != "stop" {
                    recording = Some(start_recording(handle, rest.trim(), cx));
                }
                if !quit {
                    continue;
                }
            }
            if cmd == "caption" {
                let (x, y) = *POINTER.lock().unwrap();
                log_pointer(x, y, &format!("caption {}", rest.trim()));
                continue;
            }
            #[cfg(target_os = "macos")]
            if cmd == "glide" {
                let n = nums(rest);
                if n.len() >= 3 {
                    let (x0, y0) = *POINTER.lock().unwrap();
                    let steps = (n[2] / 16.).max(1.) as usize;
                    for i in 1..=steps {
                        let t = ease(i as f32 / steps as f32);
                        let (x, y) = (x0 + (n[0] - x0) * t, y0 + (n[1] - y0) * t);
                        native::send(ns_view, native::MOVED, x, y, 0);
                        log_pointer(x, y, "move");
                        cx.background_executor().timer(Duration::from_millis(16)).await;
                    }
                }
                continue;
            }
            if cmd == "scroll" {
                let n = nums(rest);
                if n.len() >= 4 {
                    let steps = (n[3] / 16.).max(1.) as usize;
                    let mut done = 0.;
                    for i in 1..=steps {
                        let to = n[2] * ease(i as f32 / steps as f32);
                        let event = ScrollWheelEvent {
                            position: point(px(n[0]), px(n[1])),
                            // Positive dy scrolls the content up (reading on).
                            delta: ScrollDelta::Pixels(point(px(0.), px(done - to))),
                            ..Default::default()
                        };
                        done = to;
                        let _ = handle.update(cx, |_, window, cx| {
                            window.dispatch_event(PlatformInput::ScrollWheel(event), cx);
                        });
                        cx.background_executor().timer(Duration::from_millis(16)).await;
                    }
                }
                continue;
            }
            #[cfg(target_os = "macos")]
            if cmd == "activate" {
                native::activate(ns_view);
                cx.background_executor().timer(Duration::from_millis(300)).await;
                continue;
            }
            #[cfg(target_os = "macos")]
            if cmd == "nkey" {
                native::key(ns_view, rest.trim());
                cx.background_executor().timer(Duration::from_millis(120)).await;
                continue;
            }
            #[cfg(target_os = "macos")]
            if cmd == "mark" || cmd == "commit" {
                native::ime(ns_view, cmd == "commit", rest);
                cx.background_executor().timer(Duration::from_millis(120)).await;
                continue;
            }
            #[cfg(target_os = "macos")]
            let handled = mouse_step(ns_view, cmd, &nums(rest));
            #[cfg(not(target_os = "macos"))]
            let handled = false;
            if !handled {
                let _ = handle.update(cx, |_, window, cx| run(cmd, rest, window, cx));
            }
            if quit {
                let _ = cx.update(|cx| cx.quit());
                return;
            }
            // Let the frame settle before the next step.
            cx.background_executor().timer(Duration::from_millis(120)).await;
        }
    })
    .detach();
}

fn nums(s: &str) -> Vec<f32> {
    s.split_whitespace().filter_map(|n| n.parse().ok()).collect()
}

/// Mouse input goes through AppKit (the GPUI view's NSResponder methods) so
/// it takes the same path as real events; it must run outside of any `App` borrow.
#[cfg(target_os = "macos")]
mod native {
    use core_graphics::geometry::{CGPoint, CGRect};
    use objc::runtime::Object;
    use objc::{class, msg_send, sel, sel_impl};

    pub const MOVED: u64 = 5;
    pub const LEFT_DOWN: u64 = 1;
    pub const LEFT_UP: u64 = 2;
    pub const RIGHT_DOWN: u64 = 3;
    pub const RIGHT_UP: u64 = 4;
    pub const LEFT_DRAGGED: u64 = 6;

    /// Places scripted windows almost entirely past a screen edge that has no
    /// neighbouring display, so they neither cover the user's windows nor get
    /// covered (fully occluded windows stop rendering). Returns the candidate
    /// origins, best first; the caller tries them until one stays visible
    /// (a sliver behind the Dock counts as occluded).
    pub fn park_candidates(ns_view: usize) -> Vec<CGPoint> {
        use core_graphics::geometry::CGSize;
        const SLIVER: f64 = 24.;
        fn intersects(a: &CGRect, b: &CGRect) -> bool {
            a.origin.x < b.origin.x + b.size.width
                && b.origin.x < a.origin.x + a.size.width
                && a.origin.y < b.origin.y + b.size.height
                && b.origin.y < a.origin.y + a.size.height
        }
        unsafe {
            let view = ns_view as *mut Object;
            let win: *mut Object = msg_send![view, window];
            let w: CGRect = msg_send![win, frame];
            let (ww, wh) = (w.size.width, w.size.height);
            let list: *mut Object = msg_send![class!(NSScreen), screens];
            let n: usize = msg_send![list, count];
            let screens: Vec<CGRect> = (0..n)
                .map(|i| {
                    let sc: *mut Object = msg_send![list, objectAtIndex: i];
                    let f: CGRect = msg_send![sc, frame];
                    f
                })
                .collect();
            let mut out = vec![];
            for (i, s) in screens.iter().enumerate() {
                let (sx, sy, sw, sh) = (s.origin.x, s.origin.y, s.size.width, s.size.height);
                let mid_y = sy + (sh - wh) / 2.;
                for o in [
                    CGPoint::new(sx + sw - SLIVER, mid_y),
                    CGPoint::new(sx - ww + SLIVER, mid_y),
                    // Bottom corners first: a centred Dock hides the middle.
                    CGPoint::new(sx, sy - wh + SLIVER),
                    CGPoint::new(sx + sw - ww, sy - wh + SLIVER),
                    CGPoint::new(sx + (sw - ww) / 2., sy - wh + SLIVER),
                ] {
                    let r = CGRect::new(&o, &CGSize::new(ww, wh));
                    if !screens.iter().enumerate().any(|(j, f)| j != i && intersects(&r, f)) {
                        out.push(o);
                    }
                }
            }
            out
        }
    }

    /// `place` with the origin given as the top-left corner in screen
    /// coordinates (y down from the main display's top).
    pub fn place_top_left(ns_view: usize, x: f64, y: f64) {
        unsafe {
            let win: *mut Object = msg_send![ns_view as *mut Object, window];
            let frame: CGRect = msg_send![win, frame];
            let list: *mut Object = msg_send![class!(NSScreen), screens];
            let main: *mut Object = msg_send![list, objectAtIndex: 0usize];
            let screen: CGRect = msg_send![main, frame];
            place(ns_view, CGPoint::new(x, screen.size.height - y - frame.size.height));
        }
    }

    pub fn place(ns_view: usize, origin: CGPoint) {
        unsafe {
            let win: *mut Object = msg_send![ns_view as *mut Object, window];
            let _: () = msg_send![win, setLevel: 3isize]; // NSFloatingWindowLevel
            // canJoinAllSpaces | stationary | ignoresCycle | fullScreenAuxiliary:
            // stays visible (so it keeps rendering) when the user is in
            // another Space or a full-screen app.
            let _: () = msg_send![win, setCollectionBehavior: (1usize | 1 << 4 | 1 << 6 | 1 << 8)];
            let _: () = msg_send![win, setFrameOrigin: origin];
        }
    }

    /// Opts out of App Nap, which otherwise stretches timers to ~1s while
    /// the (unfocused, mostly off-screen) window is scripted.
    pub fn no_app_nap() {
        unsafe {
            let info: *mut Object = msg_send![class!(NSProcessInfo), processInfo];
            let reason: *mut Object = msg_send![class!(NSString), alloc];
            let text = "scripted UI run";
            let reason: *mut Object = msg_send![reason, initWithBytes: text.as_ptr() length: text.len() encoding: 4usize];
            // NSActivityUserInitiatedAllowingIdleSystemSleep | NSActivityLatencyCritical
            let activity: *mut Object = msg_send![info, beginActivityWithOptions: 0x00EF_FFFFu64 | 0xFF_0000_0000u64 reason: reason];
            let _: *mut Object = msg_send![activity, retain]; // held for the process lifetime
        }
    }

    pub fn is_visible(ns_view: usize) -> bool {
        const NS_WINDOW_OCCLUSION_STATE_VISIBLE: u64 = 1 << 1;
        unsafe {
            let win: *mut Object = msg_send![ns_view as *mut Object, window];
            let state: u64 = msg_send![win, occlusionState];
            state & NS_WINDOW_OCCLUSION_STATE_VISIBLE != 0
        }
    }

    /// IME composition through the view's NSTextInputClient methods:
    /// `mark` = setMarkedText (pinyin in progress), `commit` = insertText.
    pub fn ime(ns_view: usize, commit: bool, text: &str) {
        #[repr(C)]
        struct NSRange {
            location: usize,
            length: usize,
        }
        unsafe impl objc::Encode for NSRange {
            fn encode() -> objc::Encoding {
                unsafe { objc::Encoding::from_str("{_NSRange=QQ}") }
            }
        }
        unsafe {
            let view = ns_view as *mut Object;
            let s: *mut Object = msg_send![class!(NSString), alloc];
            let s: *mut Object = msg_send![s,
                initWithBytes: text.as_ptr()
                length: text.len()
                encoding: 4usize]; // NSUTF8StringEncoding
            let none = NSRange { location: isize::MAX as usize, length: 0 }; // NSNotFound (NSIntegerMax)
            if commit {
                let _: () = msg_send![view, insertText: s replacementRange: none];
            } else {
                let n = text.encode_utf16().count();
                let sel = NSRange { location: n, length: 0 };
                let _: () = msg_send![view, setMarkedText: s selectedRange: sel replacementRange: none];
            }
        }
    }

    /// Makes the window key in the active app, so the input method serves it.
    /// Only for IME checks: it takes focus from whatever the user is doing.
    pub fn activate(ns_view: usize) {
        unsafe {
            let win: *mut Object = msg_send![ns_view as *mut Object, window];
            let app: *mut Object = msg_send![class!(NSApplication), sharedApplication];
            let _: () = msg_send![app, activateIgnoringOtherApps: true];
            let nil: *mut Object = std::ptr::null_mut();
            let _: () = msg_send![win, makeKeyAndOrderFront: nil];
        }
    }

    /// Key down + up for a lowercase letter, delivered to the view's
    /// `keyDown:` so it goes through `interpretKeyEvents:` and the input method.
    pub fn key(ns_view: usize, letter: &str) {
        const CODES: &str = "asdfhgzxcv_bqweryt123465=97-80]ou[ip_lj'k;\\,/nm.";
        let Some(code) = CODES.find(letter) else { return };
        unsafe {
            let view = ns_view as *mut Object;
            let win: *mut Object = msg_send![view, window];
            let number: isize = msg_send![win, windowNumber];
            let nil: *mut Object = std::ptr::null_mut();
            let s: *mut Object = msg_send![class!(NSString), alloc];
            let s: *mut Object = msg_send![s, initWithBytes: letter.as_ptr() length: letter.len() encoding: 4usize];
            for (kind, sel) in [(10u64, true), (11u64, false)] {
                let ev: *mut Object = msg_send![class!(NSEvent),
                    keyEventWithType: kind
                    location: CGPoint::new(0., 0.)
                    modifierFlags: 0u64
                    timestamp: 0f64
                    windowNumber: number
                    context: nil
                    characters: s
                    charactersIgnoringModifiers: s
                    isARepeat: false
                    keyCode: code as u16];
                if sel {
                    let _: () = msg_send![view, keyDown: ev];
                } else {
                    let _: () = msg_send![view, keyUp: ev];
                }
            }
        }
    }

    pub fn send(ns_view: usize, kind: u64, x: f32, y: f32, clicks: i64) {
        unsafe {
            let view = ns_view as *mut Object;
            let win: *mut Object = msg_send![view, window];
            let frame: CGRect = msg_send![view, frame];
            let loc = CGPoint::new(x as f64, frame.size.height - y as f64);
            let number: isize = msg_send![win, windowNumber];
            let nil: *mut Object = std::ptr::null_mut();
            let ev: *mut Object = msg_send![class!(NSEvent),
                mouseEventWithType: kind
                location: loc
                modifierFlags: 0u64
                timestamp: 0f64
                windowNumber: number
                context: nil
                eventNumber: 0isize
                clickCount: clicks
                pressure: 1.0f32];
            // Deliver straight to the view: going through `-[NSWindow sendEvent:]`
            // would require the window to be key, i.e. steal focus from the user.
            match kind {
                LEFT_DOWN => msg_send![view, mouseDown: ev],
                LEFT_UP => msg_send![view, mouseUp: ev],
                RIGHT_DOWN => msg_send![view, rightMouseDown: ev],
                RIGHT_UP => msg_send![view, rightMouseUp: ev],
                LEFT_DRAGGED => msg_send![view, mouseDragged: ev],
                _ => msg_send![view, mouseMoved: ev],
            }
        }
    }
}

#[cfg(target_os = "macos")]
fn ns_view_of(window: &Window) -> Option<usize> {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    let RawWindowHandle::AppKit(h) = HasWindowHandle::window_handle(window).ok()?.as_raw() else { return None };
    Some(h.ns_view.as_ptr() as usize)
}

/// Returns true when the step was a mouse step (handled natively).
#[cfg(target_os = "macos")]
fn mouse_step(view: usize, cmd: &str, n: &[f32]) -> bool {
    use native::*;
    if n.len() >= 2 && matches!(cmd, "move" | "click" | "dclick" | "rclick") {
        log_pointer(n[0], n[1], if cmd == "move" { "move" } else { "click" });
    }
    match cmd {
        "move" if n.len() >= 2 => send(view, MOVED, n[0], n[1], 0),
        "click" if n.len() >= 2 => {
            send(view, MOVED, n[0], n[1], 0);
            send(view, LEFT_DOWN, n[0], n[1], 1);
            send(view, LEFT_UP, n[0], n[1], 1);
        }
        "dclick" if n.len() >= 2 => {
            send(view, LEFT_DOWN, n[0], n[1], 1);
            send(view, LEFT_UP, n[0], n[1], 1);
            send(view, LEFT_DOWN, n[0], n[1], 2);
            send(view, LEFT_UP, n[0], n[1], 2);
        }
        "drag" if n.len() >= 4 => {
            send(view, MOVED, n[0], n[1], 0);
            send(view, LEFT_DOWN, n[0], n[1], 1);
            for i in 1..=8 {
                let t = i as f32 / 8.;
                send(view, LEFT_DRAGGED, n[0] + (n[2] - n[0]) * t, n[1] + (n[3] - n[1]) * t, 1);
            }
            send(view, LEFT_UP, n[2], n[3], 1);
        }
        // Step-by-step drags, so frames render (and shots land) mid-drag. An
        // optional third number is the click count ("down x y 2": a double
        // click's second press, with frames in between).
        "down" if n.len() >= 2 => {
            send(view, MOVED, n[0], n[1], 0);
            send(view, LEFT_DOWN, n[0], n[1], n.get(2).map_or(1, |c| *c as i64));
        }
        "dragto" if n.len() >= 2 => send(view, LEFT_DRAGGED, n[0], n[1], 1),
        // `place x y`: the window's top-left at screen point (x, y) of the main
        // display, floating (for desktop shots; see tools/harness/README.md).
        "place" if n.len() >= 2 => place_top_left(view, n[0] as f64, n[1] as f64),
        "up" if n.len() >= 2 => send(view, LEFT_UP, n[0], n[1], n.get(2).map_or(1, |c| *c as i64)),
        _ => return false,
    }
    true
}

fn run(cmd: &str, rest: &str, window: &mut Window, cx: &mut App) {
    match cmd {
        // Synthesized right-button NSEvents report buttonNumber 0, so GPUI would
        // read them as left clicks; feed the right click to GPUI directly.
        "rclick" => {
            let n = nums(rest);
            if n.len() >= 2 {
                let position = point(px(n[0]), px(n[1]));
                let modifiers = Modifiers::default();
                window.dispatch_event(PlatformInput::MouseMove(MouseMoveEvent { position, pressed_button: None, modifiers }), cx);
                window.dispatch_event(
                    PlatformInput::MouseDown(MouseDownEvent { button: MouseButton::Right, position, modifiers, click_count: 1, first_mouse: false }),
                    cx,
                );
                window.dispatch_event(PlatformInput::MouseUp(MouseUpEvent { button: MouseButton::Right, position, modifiers, click_count: 1 }), cx);
            }
        }
        "key" => {
            for k in rest.split_whitespace() {
                match Keystroke::parse(k) {
                    Ok(ks) => {
                        let handled = window.dispatch_keystroke(ks, cx);
                        if std::env::var_os("FDL_TRACE").is_some() {
                            eprintln!("devtools: key {k} handled={handled} focused={:?}", window.focused(cx).is_some());
                        }
                    }
                    Err(e) => eprintln!("devtools: bad keystroke {k}: {e:?}"),
                }
            }
        }
        "type" => {
            for ch in rest.chars() {
                let s = ch.to_string();
                let ks = Keystroke { modifiers: Modifiers::default(), key: if ch == ' ' { "space".into() } else { s.clone() }, key_char: Some(s) };
                window.dispatch_keystroke(ks, cx);
            }
        }
        // `clip html|text <file>`: what the next paste reads instead of the
        // system clipboard (`clip text` with HTML keeps its plain fallback).
        "clip" => {
            let (kind, path) = rest.trim().split_once(' ').unwrap_or((rest.trim(), ""));
            let body = std::fs::read_to_string(path.trim()).unwrap_or_default();
            let mut fake = crate::clipboard::FAKE_CLIPBOARD.lock().unwrap();
            let cur = fake.take().unwrap_or((None, String::new()));
            *fake = Some(match kind {
                "html" => (Some(body), cur.1),
                _ => (cur.0, body),
            });
        }
        "pick" => *PICK.lock().unwrap() = Some(rest.trim().split('|').map(std::path::PathBuf::from).collect()),
        // `deskshot name margin`: the window as it sits on screen, with
        // `margin` pt of whatever is below it (a wallpaper backdrop for the
        // 开始指南 screenshots) and the system's own corners and shadow.
        #[cfg(target_os = "macos")]
        "deskshot" => {
            let mut it = rest.split_whitespace();
            let name = it.next().unwrap_or("desk").to_string();
            let margin: f64 = it.next().and_then(|m| m.parse().ok()).unwrap_or(48.);
            let dir = std::env::var("FDL_SHOTS").unwrap_or_else(|_| "/tmp".into());
            let path = format!("{dir}/{name}.png");
            match desk_grab(window, margin).and_then(|(w, h, mut px)| {
                for p in px.as_chunks_mut::<4>().0 { p.swap(0, 2); p[3] = 255; }
                image::save_buffer(&path, &px, w as u32, h as u32, image::ColorType::Rgba8).map_err(Into::into)
            }) {
                Ok(()) => eprintln!("devtools: saved {path}"),
                Err(e) => eprintln!("devtools: capture failed: {e}"),
            }
        }
        "shot" => {
            let name = rest.trim().to_string();
            let dir = std::env::var("FDL_SHOTS").unwrap_or_else(|_| "/tmp".into());
            let path = format!("{dir}/{name}.png");
            match capture(window, cx, &path) {
                Ok(()) => eprintln!("devtools: saved {path}"),
                Err(e) => eprintln!("devtools: capture failed: {e}"),
            }
        }
        "wheel" => {
            let n = nums(rest);
            if n.len() >= 3 {
                let event = ScrollWheelEvent {
                    position: point(px(n[0]), px(n[1])),
                    delta: ScrollDelta::Pixels(point(px(0.), px(-n[2]))),
                    ..Default::default()
                };
                window.dispatch_event(PlatformInput::ScrollWheel(event), cx);
            }
        }
        "resize" => {
            let n = nums(rest);
            if n.len() >= 2 {
                window.resize(size(px(n[0]), px(n[1])));
            }
        }
        "noop" => window.refresh(),
        "quit" => {}
        _ => eprintln!("devtools: unknown step {cmd} {rest}"),
    }
}

/// The window's content as tightly packed BGRA, rendered offscreen so it
/// works even while the window doesn't (occluded, screen locked). Falls back
/// to the window server's copy, which also has the system-drawn traffic lights.
fn frame(window: &mut Window, cx: &mut App) -> anyhow::Result<(usize, usize, Vec<u8>)> {
    if std::env::var_os("FDL_WINDOW_CAPTURE").is_none()
        && let Some(image) = window.render_offscreen(cx)
    {
        return Ok(image);
    }
    grab(window)
}

#[cfg(target_os = "macos")]
fn capture(window: &mut Window, cx: &mut App, path: &str) -> anyhow::Result<()> {
    let (w, h, mut px) = frame(window, cx)?;
    for p in px.as_chunks_mut::<4>().0 {
        // BGRA (little-endian premultiplied first) → opaque RGBA.
        p.swap(0, 2);
        p[3] = 255;
    }
    image::save_buffer(path, &px, w as u32, h as u32, image::ColorType::Rgba8)?;
    Ok(())
}

/// The screen around the window (`margin` pt each side): the window and
/// everything below it, as tightly packed BGRA.
#[cfg(target_os = "macos")]
fn desk_grab(window: &Window, margin: f64) -> anyhow::Result<(usize, usize, Vec<u8>)> {
    use core_graphics::geometry::{CGPoint, CGRect, CGSize};
    use core_graphics::window::{create_image, kCGWindowImageBestResolution, kCGWindowListOptionIncludingWindow, kCGWindowListOptionOnScreenBelowWindow};
    use objc::{class, msg_send, sel, sel_impl};
    let view = ns_view_of(window).ok_or_else(|| anyhow::anyhow!("not an AppKit window"))? as *mut objc::runtime::Object;
    let (number, frame, screen_h): (isize, CGRect, f64) = unsafe {
        let win: *mut objc::runtime::Object = msg_send![view, window];
        let list: *mut objc::runtime::Object = msg_send![class!(NSScreen), screens];
        let main: *mut objc::runtime::Object = msg_send![list, objectAtIndex: 0usize];
        let screen: CGRect = msg_send![main, frame];
        (msg_send![win, windowNumber], msg_send![win, frame], screen.size.height)
    };
    let top = screen_h - frame.origin.y - frame.size.height;
    let rect = CGRect::new(&CGPoint::new(frame.origin.x - margin, top - margin), &CGSize::new(frame.size.width + 2. * margin, frame.size.height + 2. * margin));
    let img = create_image(rect, kCGWindowListOptionOnScreenBelowWindow | kCGWindowListOptionIncludingWindow, number as u32, kCGWindowImageBestResolution)
        .ok_or_else(|| anyhow::anyhow!("CGWindowListCreateImage returned null"))?;
    let (w, h, stride) = (img.width(), img.height(), img.bytes_per_row());
    let data = img.data();
    let bytes = data.bytes();
    let mut out = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        out.extend_from_slice(&bytes[y * stride..y * stride + w * 4]);
    }
    Ok((w, h, out))
}

/// The window's pixels as tightly packed BGRA.
#[cfg(target_os = "macos")]
fn grab(window: &Window) -> anyhow::Result<(usize, usize, Vec<u8>)> {
    use core_graphics::geometry::{CGPoint, CGRect, CGSize};
    use core_graphics::window::{create_image, kCGWindowImageBoundsIgnoreFraming, kCGWindowListOptionIncludingWindow};
    use objc::{msg_send, sel, sel_impl};
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};

    let RawWindowHandle::AppKit(h) = HasWindowHandle::window_handle(window).map_err(|e| anyhow::anyhow!("{e:?}"))?.as_raw() else { anyhow::bail!("not an AppKit window") };
    let view = h.ns_view.as_ptr() as *mut objc::runtime::Object;
    let number: isize = unsafe {
        let ns_window: *mut objc::runtime::Object = msg_send![view, window];
        msg_send![ns_window, windowNumber]
    };
    let img = create_image(CGRect::new(&CGPoint::new(f64::INFINITY, f64::INFINITY), &CGSize::new(0., 0.)), kCGWindowListOptionIncludingWindow, number as u32, kCGWindowImageBoundsIgnoreFraming)
        .ok_or_else(|| anyhow::anyhow!("CGWindowListCreateImage returned null"))?;
    let (w, h, stride) = (img.width(), img.height(), img.bytes_per_row());
    let data = img.data();
    let bytes = data.bytes();
    let mut out = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        out.extend_from_slice(&bytes[y * stride..y * stride + w * 4]);
    }
    Ok((w, h, out))
}

/// Starts capturing frames into ffmpeg; set the returned flag to stop.
fn start_recording(handle: AnyWindowHandle, path: &str, cx: &mut AsyncApp) -> std::rc::Rc<std::cell::Cell<bool>> {
    let stop = std::rc::Rc::new(std::cell::Cell::new(false));
    let start = Instant::now();
    *CURSOR_LOG.lock().unwrap() = std::fs::File::create(format!("{path}.cursor")).ok().map(|f| (start, f));
    let (x, y) = *POINTER.lock().unwrap();
    log_pointer(x, y, "move");
    let (tx, rx) = std::sync::mpsc::channel::<(f64, usize, usize, Vec<u8>)>();
    let out = path.to_string();
    let fps = fps();
    // Encoding runs off the main thread; frames that arrive late are repeated
    // so the video keeps wall-clock time.
    std::thread::spawn(move || {
        use std::io::Write;
        let mut ffmpeg: Option<std::process::Child> = None;
        let mut next = 0u64;
        while let Ok((t, w, h, mut px)) = rx.recv() {
            let enc = ffmpeg.get_or_insert_with(|| {
                std::process::Command::new("ffmpeg")
                    .args(["-y", "-loglevel", "error", "-f", "rawvideo", "-pix_fmt", "bgra", "-s", &format!("{w}x{h}"), "-r", &fps.to_string(), "-i", "-"])
                    .args(["-c:v", "libx264", "-preset", "ultrafast", "-crf", "6", "-pix_fmt", "yuv444p", &out])
                    .stdin(std::process::Stdio::piped())
                    .spawn()
                    .expect("ffmpeg")
            });
            for p in px.as_chunks_mut::<4>().0 {
                p[3] = 255;
            }
            let stdin = enc.stdin.as_mut().unwrap();
            while next as f64 <= t * fps {
                if stdin.write_all(&px).is_err() {
                    return;
                }
                next += 1;
            }
        }
        if let Some(mut enc) = ffmpeg {
            drop(enc.stdin.take());
            let _ = enc.wait();
        }
        eprintln!("devtools: recorded {out} ({next} frames)");
    });
    let flag = stop.clone();
    cx.spawn(async move |cx| {
        while !flag.get() {
            let t = start.elapsed().as_secs_f64();
            if let Ok(Ok((w, h, px))) = handle.update(cx, |_, window, cx| frame(window, cx)) {
                let _ = tx.send((t, w, h, px));
            }
            cx.background_executor().timer(Duration::from_millis(12)).await;
        }
    })
    .detach();
    stop
}

#[cfg(not(target_os = "macos"))]
fn capture(_: &mut Window, _: &mut App, _: &str) -> anyhow::Result<()> {
    anyhow::bail!("capture is only implemented on macOS")
}

#[cfg(not(target_os = "macos"))]
fn grab(_: &Window) -> anyhow::Result<(usize, usize, Vec<u8>)> {
    anyhow::bail!("capture is only implemented on macOS")
}
