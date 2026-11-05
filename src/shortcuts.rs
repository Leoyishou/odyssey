use gpui::*;
use gpui::prelude::FluentBuilder;
use std::collections::BTreeMap;
use crate::{app::Workspace, settings::{english, tr}, theme::Theme};

pub struct Shortcuts { base: Vec<KeyBinding>, custom: BTreeMap<String, String> }
impl Global for Shortcuts {}
fn path() -> std::path::PathBuf { crate::store::Store::root_dir().join("shortcuts.json") }
/// The system-wide quick note hotkey unless the user picked another.
pub const GLOBAL_DEFAULT: &str = "cmd-space";
pub fn global_key() -> String {
    std::fs::read(path()).ok().and_then(|b| serde_json::from_slice::<BTreeMap<String,String>>(&b).ok()).and_then(|m| m.get("global").cloned()).unwrap_or_else(|| GLOBAL_DEFAULT.into())
}
pub fn init(cx: &mut App) {
    let base = cx.key_bindings().borrow().bindings().cloned().collect::<Vec<_>>();
    #[cfg(target_os = "windows")]
    let base = base.into_iter().filter_map(windows_binding).collect();
    let custom = std::fs::read(path()).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default();
    cx.set_global(Shortcuts { base, custom });
    rebuild(cx);
}

#[cfg(target_os = "windows")]
fn windows_binding(binding: KeyBinding) -> Option<KeyBinding> {
    let keys = binding.keystrokes().iter().map(|k| k.unparse().replace("win-", "cmd-")).collect::<Vec<_>>().join(" ");
    // Cocoa's control-key navigation conflicts with Windows editing shortcuts.
    if matches!(keys.as_str(), "ctrl-a" | "ctrl-b" | "ctrl-d" | "ctrl-e" | "ctrl-f" | "ctrl-h" | "ctrl-n" | "ctrl-p" | "ctrl-backspace") { return None; }
    let keys = if binding.action().name().ends_with("::InlineCode") { "ctrl-shift-c".to_string() } else { keys.split(' ').map(|key| match key {
        "cmd-left" => "home".into(), "cmd-right" => "end".into(),
        "cmd-shift-left" => "shift-home".into(), "cmd-shift-right" => "shift-end".into(),
        "cmd-up" => "ctrl-home".into(), "cmd-down" => "ctrl-end".into(),
        "cmd-shift-up" => "ctrl-shift-home".into(), "cmd-shift-down" => "ctrl-shift-end".into(),
        "alt-left" | "alt-right" | "alt-shift-left" | "alt-shift-right" | "alt-backspace" | "alt-delete" => key.replacen("alt-", "ctrl-", 1),
        _ => key.replace("cmd-", "ctrl-"),
    }).collect::<Vec<String>>().join(" ") };
    KeyBinding::load(&keys, binding.action().boxed_clone(), binding.predicate(), false, binding.action_input(), &DummyKeyboardMapper).ok()
}

#[cfg(all(test, target_os = "windows"))]
mod tests {
    use super::windows_binding;
    use gpui::KeyBinding;
    #[test]
    fn copy_and_inline_code_keep_distinct_windows_shortcuts() {
        use crate::editor::keymap::{Copy, InlineCode};
        let copy = windows_binding(KeyBinding::new("cmd-c", Copy, Some("Editor"))).unwrap();
        let code = windows_binding(KeyBinding::new("cmd-ctrl-c", InlineCode, Some("Editor"))).unwrap();
        assert_eq!(copy.keystrokes().iter().map(|k| k.unparse()).collect::<Vec<_>>(), ["ctrl-c"]);
        assert_eq!(code.keystrokes().iter().map(|k| k.unparse()).collect::<Vec<_>>(), ["ctrl-shift-c"]);
        assert_ne!(copy.keystrokes(), code.keystrokes());
    }
}
/// Mac key notation (⌘ ⌥ ⇧ ⌃, "⌘ + Shift + 7") as Windows writes it, the way
/// `windows_binding` maps the keys: ⌘ is Ctrl, ⌘ ⌃ C (inline code) is
/// Ctrl+Shift+C, ⌘ with an arrow is Home / End. "⌘ + B" stays spaced
/// ("Ctrl + B"), "⌘ B" becomes "Ctrl+B". A combination Windows has no twin
/// for (⌘ ⌃ ⇧ 4, the macOS screenshot) is left as written.
pub fn windows_keys(s: &str) -> String {
    const SYMBOLS: [(&str, usize); 4] = [("⌘", 0), ("⌃", 1), ("⌥", 2), ("⇧", 3)];
    let modifier = |t: &str, first: bool| -> Option<(usize, usize)> {
        SYMBOLS.iter().find(|(m, _)| t.starts_with(m)).map(|(m, i)| (*i, m.len()))
            .or_else(|| if first { None } else { [("Control", 1), ("Shift", 3)].iter().find(|(w, _)| t.starts_with(w) && !t[w.len()..].starts_with(|c: char| c.is_ascii_alphanumeric())).map(|(w, i)| (*i, w.len())) })
    };
    let mut out = String::new();
    let mut i = 0;
    while i < s.len() {
        let Some(_) = modifier(&s[i..], true) else {
            let c = s[i..].chars().next().unwrap();
            out.push(c);
            i += c.len_utf8();
            continue;
        };
        let (start, mut j, mut held, mut spaced) = (i, i, [false; 4], false);
        while let Some((m, n)) = modifier(&s[j..], j == start) {
            held[m] = true;
            j += n;
            let rest = &s[j..];
            let gap = rest.len() - rest.trim_start_matches(' ').len();
            // "⌘ + B" separates with " + "; in "⌘ +" the plus is the key.
            if rest[gap..].starts_with("+ ") { spaced = true; j += gap + 1; j += s[j..].len() - s[j..].trim_start_matches(' ').len(); } else { j += gap; }
        }
        let rest = &s[j..];
        let key_len = if rest.starts_with('&') { rest.find(';').map_or(0, |e| e + 1) } else {
            rest.chars().next().filter(|c| !c.is_whitespace() && *c != '<').map_or(0, |c| c.len_utf8() + rest[c.len_utf8()..].chars().take_while(|c| c.is_ascii_alphanumeric()).map(char::len_utf8).sum::<usize>())
        };
        let mut key = rest[..key_len].to_string();
        let [cmd, mut ctrl, alt, mut shift] = held;
        if key.is_empty() || cmd && ctrl && !(key.eq_ignore_ascii_case("c") && !alt && !shift) {
            out.push_str(&s[start..j + key_len]);
            i = j + key_len;
            continue;
        }
        if cmd && ctrl { shift = true; }
        else if cmd { match key.as_str() {
            "←" => key = "Home".into(), "→" => key = "End".into(),
            "↑" => { key = "Home".into(); ctrl = true; } "↓" => { key = "End".into(); ctrl = true; }
            _ => ctrl = true,
        } }
        let parts: Vec<&str> = [(ctrl, "Ctrl"), (alt, "Alt"), (shift, "Shift")].into_iter().filter(|(on, _)| *on).map(|(_, n)| n).chain(Some(key.as_str())).collect();
        out.push_str(&parts.join(if spaced { " + " } else { "+" }));
        i = j + key_len;
    }
    out
}

fn first_group_hint() -> &'static str {
    if cfg!(target_os = "windows") { tr("第一组请包含 Ctrl 或 Alt", "Start with Ctrl or Alt") } else { tr("第一组请包含 ⌘、⌃ 或 ⌥", "Start with ⌘, ⌃ or ⌥") }
}

/// A UI label's keys for this platform: `windows_keys` on Windows (cached,
/// labels are `'static`), the text itself elsewhere.
pub fn keys(s: &'static str) -> &'static str {
    if !cfg!(target_os = "windows") { return s }
    static CACHE: std::sync::Mutex<BTreeMap<&'static str, &'static str>> = std::sync::Mutex::new(BTreeMap::new());
    let mut cache = CACHE.lock().unwrap();
    cache.entry(s).or_insert_with(|| Box::leak(windows_keys(s).into_boxed_str()))
}

/// The user's keys, for `hint` (which runs where no `App` is at hand).
static CUSTOM: std::sync::RwLock<BTreeMap<String, String>> = std::sync::RwLock::new(BTreeMap::new());
static HINTS: std::sync::Mutex<BTreeMap<String, &'static str>> = std::sync::Mutex::new(BTreeMap::new());

/// The key hint the UI shows for `action`: `default` (its built-in key,
/// written as the UI writes it) unless the user changed it in 设置.
pub fn hint(action: &str, default: &'static str) -> &'static str {
    let Some(key) = CUSTOM.read().unwrap().get(action).cloned() else { return keys(default) };
    let mut cache = HINTS.lock().unwrap();
    if let Some(h) = cache.get(&key) { return h }
    let text = key.split(' ').map(|stroke| {
        let Ok(k) = Keystroke::parse(stroke) else { return stroke.to_string() };
        let m = k.modifiers;
        let names = if cfg!(target_os = "windows") { [(m.control, "Ctrl"), (m.alt, "Alt"), (m.shift, "Shift"), (m.platform, "Win")] } else { [(m.platform, "⌘"), (m.alt, "⌥"), (m.shift, "Shift"), (m.control, "Control")] };
        let mut parts: Vec<String> = names.into_iter().filter(|(on, _)| *on).map(|(_, s)| s.to_string()).collect();
        parts.push(match k.key.as_str() { "space" => "Space".into(), "enter" => "Enter".into(), k if k.chars().count() == 1 => k.to_uppercase(), k => { let mut c = k.chars(); c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default() } });
        parts.join(" + ")
    }).collect::<Vec<_>>().join(", ");
    let leaked: &'static str = Box::leak(text.into_boxed_str());
    cache.insert(key, leaked);
    leaked
}

pub fn rebuild(cx: &mut App) {
    *CUSTOM.write().unwrap() = cx.global::<Shortcuts>().custom.clone();
    let s = cx.global::<Shortcuts>();
    let bindings: Vec<_> = s.base.iter().filter(|b| !s.custom.get(b.action().name()).is_some_and(|key| key.is_empty())).map(|b| {
        s.custom.get(b.action().name()).and_then(|key| KeyBinding::load(key, b.action().boxed_clone(), b.predicate(), false, b.action_input(), &DummyKeyboardMapper).ok()).unwrap_or_else(|| b.clone())
    }).collect();
    cx.clear_key_bindings(); cx.bind_keys(bindings);
}

/// Settings groups, in order.
const GROUPS: [(&str, &str); 5] = [("应用与窗口", "App & windows"), ("文档", "Documents"), ("编辑", "Editing"), ("文字样式", "Text styles"), ("段落与块", "Paragraphs & blocks")];

/// (group, 中文, English) of the actions shown in Settings.
fn info(name: &str) -> Option<(usize, &'static str, &'static str)> {
    Some(match name.rsplit("::").next().unwrap_or(name) {
        "NewDoc" => (0, "新建文档", "New document"),
        "QuickNew" => (0, "新建笔记窗口", "New note window"),
        "CloseWindow" => (0, "关闭窗口", "Close window"),
        "HideApp" => (0, "隐藏 Odyssey", "Hide Odyssey"),
        "Minimize" => (0, "最小化窗口", "Minimize window"),
        "ToggleFullScreen" => (0, "进入 / 退出全屏", "Enter / exit full screen"),
        "OpenSettings" => (0, "打开设置", "Open Settings"),
        "GoHome" => (0, "返回主页", "Go to Home"),
        "ToggleSidebar" => (0, "展开 / 收起侧栏", "Show / hide sidebar"),
        "FocusSearch" => (0, "搜索文档", "Search documents"),
        "ToggleShortcuts" => (0, "查看快捷键", "Show shortcuts"),
        "ImportDoc" => (1, "导入文档", "Import documents"),
        "Save" => (1, "保存文档", "Save document"),
        "EditMode" => (1, "编辑模式", "Edit mode"),
        "ReadMode" => (1, "阅读模式", "Reading mode"),
        "FindInDoc" => (1, "文档内查找", "Find in document"),
        "ReplaceInDoc" => (1, "查找和替换", "Find and replace"),
        "Undo" => (2, "撤销", "Undo"),
        "Redo" => (2, "重做", "Redo"),
        "Copy" => (2, "复制", "Copy"),
        "Cut" => (2, "剪切", "Cut"),
        "Paste" => (2, "粘贴", "Paste"),
        "SelectAll" => (2, "全选", "Select all"),
        "PastePlain" => (2, "粘贴为纯文本", "Paste as plain text"),
        "Link" => (2, "插入链接", "Insert link"),
        "InsertReminder" => (2, "插入提醒", "Insert reminder"),
        "MoveBlockUp" => (2, "上移当前块", "Move block up"),
        "MoveBlockDown" => (2, "下移当前块", "Move block down"),
        "ToggleTodo" => (2, "切换任务完成状态", "Check / uncheck to-do"),
        "Bold" => (3, "加粗", "Bold"),
        "Italic" => (3, "斜体", "Italic"),
        "Underline" => (3, "下划线", "Underline"),
        "Strike" => (3, "删除线", "Strikethrough"),
        "InlineCode" => (3, "行内代码", "Inline code"),
        "ApplyLastColor" => (3, "应用上次的文字色和背景色", "Apply last colors"),
        "TurnText" => (4, "正文", "Text"),
        "TurnH1" => (4, "1 级标题", "Heading 1"),
        "TurnH2" => (4, "2 级标题", "Heading 2"),
        "TurnH3" => (4, "3 级标题", "Heading 3"),
        "TurnH4" => (4, "4 级标题", "Heading 4"),
        "TurnH5" => (4, "5 级标题", "Heading 5"),
        "TurnH6" => (4, "6 级标题", "Heading 6"),
        "TurnH7" => (4, "7 级标题", "Heading 7"),
        "TurnH8" => (4, "8 级标题", "Heading 8"),
        "TurnH9" => (4, "9 级标题", "Heading 9"),
        "TurnOrdered" => (4, "有序列表", "Numbered list"),
        "TurnBullet" => (4, "无序列表", "Bulleted list"),
        "TurnTodo" => (4, "任务列表", "To-do list"),
        "TurnCode" => (4, "代码块", "Code block"),
        "TurnQuote" => (4, "引用", "Quote"),
        "InsertDivider" => (4, "分隔线", "Divider"),
        "InsertTable" => (4, "插入表格", "Insert table"),
        "AlignLeft" => (4, "左对齐", "Align left"),
        "AlignCenter" => (4, "居中对齐", "Align center"),
        "AlignRight" => (4, "右对齐", "Align right"),
        _ => return None,
    })
}
pub fn label(name: &str) -> Option<&'static str> { info(name).map(|(_, zh, en)| if english() { en } else { zh }) }
fn default_key(s: &Shortcuts, id: &str) -> String {
    if id == "global" { return GLOBAL_DEFAULT.into(); }
    if id.ends_with("::NewDoc") { return if cfg!(target_os = "windows") { "ctrl-n" } else { "cmd-n" }.into(); }
    s.base.iter().find(|b| b.action().name() == id).map(|b| b.keystrokes().iter().map(|k| k.unparse()).collect::<Vec<_>>().join(" ")).unwrap_or_default()
}
/// The key an action runs on now.
pub fn key_of(cx: &App, id: &str) -> String {
    cx.try_global::<Shortcuts>().map(|s| s.custom.get(id).cloned().unwrap_or_else(|| default_key(s, id))).unwrap_or_default()
}
/// (action, label, key) of the shortcuts the user can change, in settings order (the global one first).
pub fn rows(cx: &App) -> Vec<(String, String, String)> {
    let Some(s) = cx.try_global::<Shortcuts>() else { return vec![] };
    let mut rows: Vec<(usize, String, String, String)> = vec![];
    for b in &s.base {
        let name = b.action().name();
        if let Some(title) = label(name) {
            if rows.iter().any(|(_, id, _, _)| id == name) { continue; }
            rows.push((info(name).unwrap().0, name.into(), title.into(), key_of(cx, name)));
        }
    }
    rows.sort_by_key(|r| r.0);
    std::iter::once(("global".to_string(), tr("全局快速新建笔记窗口", "Quick note from anywhere").to_string(), global_key()))
        .filter(|_| cfg!(target_os = "macos"))
        .chain(rows.into_iter().map(|(_, id, t, k)| (id, t, k))).collect()
}
pub fn pretty(key: &str) -> String {
    if key.is_empty() { return tr("未设置", "Not set").into(); }
    key.split(' ').map(|stroke| {
        let Ok(ks) = Keystroke::parse(stroke) else { return stroke.to_string() };
        let m = ks.modifiers;
        let win = cfg!(target_os = "windows");
        let names = if win { [(m.control, "Ctrl"), (m.alt, "Alt"), (m.shift, "Shift"), (m.platform, "Win")] } else { [(m.control, "⌃"), (m.alt, "⌥"), (m.shift, "⇧"), (m.platform, "⌘")] };
        let mut out: Vec<String> = names.into_iter().filter(|(on, _)| *on).map(|(_, s)| s.to_string()).collect();
        out.push(match ks.key.as_str() {
            "space" => "Space".into(), "escape" => "Esc".into(),
            "enter" | "backspace" | "delete" | "tab" if win => { let k = ks.key.as_str(); k[..1].to_uppercase() + &k[1..] }
            "enter" => "↩".into(), "backspace" => "⌫".into(), "delete" => "⌦".into(), "tab" => "⇥".into(),
            "up" => "↑".into(), "down" => "↓".into(), "left" => "←".into(), "right" => "→".into(),
            k if k.chars().count() == 1 => k.to_uppercase(),
            k => { let mut c = k.chars(); c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default() }
        });
        out.join(if win { "+" } else { " " })
    }).collect::<Vec<_>>().join("  →  ")
}
fn sequence(key: &str) -> anyhow::Result<Vec<Keystroke>> {
    key.split_whitespace().map(|k| Keystroke::parse(k).map_err(Into::into)).collect()
}
fn overlaps(a: &[Keystroke], b: &[Keystroke]) -> bool {
    !a.is_empty() && !b.is_empty() && (a.starts_with(b) || b.starts_with(a))
}
/// Prefix conflicts also need replacing: a single key must not consume a chord.
fn conflicts(s: &Shortcuts, id: &str, key: &str) -> Vec<String> {
    let Ok(parsed) = sequence(key) else { return vec![] };
    let contexts: Vec<_> = s.base.iter().filter(|b| b.action().name() == id).map(|b| b.predicate().map(|p| p.to_string())).collect();
    let mut found = vec![];
    for b in &s.base {
        let name = b.action().name();
        if name == id || found.iter().any(|n| n == name) { continue; }
        let other = s.custom.get(name).cloned().unwrap_or_else(|| b.keystrokes().iter().map(|k| k.unparse()).collect::<Vec<_>>().join(" "));
        let same_context = contexts.contains(&b.predicate().map(|p| p.to_string()));
        if (label(name).is_some() || same_context || id == "global") && sequence(&other).is_ok_and(|other| overlaps(&parsed, &other)) {
            found.push(name.to_string());
        }
    }
    found
}
fn change(id: &str, key: Option<String>, cx: &mut App) -> anyhow::Result<()> {
    let s = cx.global::<Shortcuts>();
    let mut custom = s.custom.clone();
    let key = key.unwrap_or_else(|| default_key(s, id));
    let parsed = sequence(&key)?;
    if !parsed.is_empty() {
        anyhow::ensure!(parsed.len() <= if id == "global" { 1 } else { 4 }, tr("应用快捷键最多四组，全局快捷键只支持一组", "Use up to four strokes, or one for a global shortcut"));
        let m = parsed[0].modifiers;
        anyhow::ensure!(m.platform || m.control || m.alt, first_group_hint());
        let global = s.custom.get("global").map(String::as_str).unwrap_or(GLOBAL_DEFAULT);
        if id != "global" {
            anyhow::ensure!(!overlaps(&parsed, &sequence(global)?), tr("与全局快速新建快捷键冲突，请先修改全局快捷键", "Change the global quick note shortcut first"));
        }
        anyhow::ensure!(!overlaps(&parsed, &sequence("cmd-alt-space")?), tr("⌘ ⌥ Space 用于全局打开上次的文档，请选择其他组合", "⌘ ⌥ Space is reserved for opening the last document"));
    }
    anyhow::ensure!(id != "global" || !parsed.is_empty(), tr("请使用开关关闭全局快捷键", "Use the switch to disable the global shortcut"));
    for other in conflicts(s, id, &key) { custom.insert(other, String::new()); }
    // Empty means explicitly unbound, rather than falling back to the default.
    if key == default_key(s, id) { custom.remove(id); } else { custom.insert(id.into(), key.clone()); }
    let old_global = s.custom.get("global").cloned().unwrap_or_else(|| GLOBAL_DEFAULT.into());
    if id == "global" && crate::settings::prefs().quick_hotkey { crate::quick_note::set_hotkey(&key, cx)?; }
    if let Err(e) = write(&custom) { if id == "global" { let _ = crate::quick_note::set_hotkey(&old_global, cx); } return Err(e); }
    cx.global_mut::<Shortcuts>().custom = custom;
    rebuild(cx); cx.refresh_windows(); Ok(())
}
fn write(custom: &BTreeMap<String, String>) -> anyhow::Result<()> {
    use std::io::Write;
    let tmp = path().with_extension("json.tmp");
    let mut f = std::fs::File::create(&tmp)?;
    f.write_all(&serde_json::to_vec_pretty(custom)?)?;
    f.sync_all()?;
    std::fs::rename(tmp, path())?;
    Ok(())
}
impl Workspace {
    pub(crate) fn cancel_shortcut_recording(&mut self, cx: &mut Context<Self>) {
        self.shortcut_strokes.clear();
        if self.shortcut_recording.take().is_some() {
            rebuild(cx);
            if crate::settings::prefs().quick_hotkey { crate::quick_note::register(cx); }
        }
    }

    pub(crate) fn start_recording(&mut self, id: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.cancel_shortcut_recording(cx);
        self.shortcut_recording = Some(id.to_string());
        self.shortcut_error = None;
        crate::quick_note::suspend_hotkey();
        window.focus(&self.focus);
        cx.clear_key_bindings();
        cx.notify();
    }

    pub(crate) fn reset_shortcut(&mut self, id: &str, cx: &mut Context<Self>) {
        self.cancel_shortcut_recording(cx);
        self.shortcut_error = change(id, None, cx).err().map(|e| (id.to_string(), e.to_string()));
        rebuild(cx);
        cx.notify();
    }

    fn reset_all_shortcuts(&mut self, cx: &mut Context<Self>) {
        self.cancel_shortcut_recording(cx);
        let global = cx.global::<Shortcuts>().custom.get("global").cloned();
        let mut custom: BTreeMap<String, String> = global.into_iter().map(|g| ("global".to_string(), g)).collect();
        // Keep the separately configured global key usable after resetting app keys.
        let defaults = Shortcuts { base: cx.global::<Shortcuts>().base.clone(), custom: custom.clone() };
        let global_key = custom.get("global").map(String::as_str).unwrap_or(GLOBAL_DEFAULT).to_string();
        for other in conflicts(&defaults, "global", &global_key) { custom.insert(other, String::new()); }
        match write(&custom) {
            Ok(()) => { cx.global_mut::<Shortcuts>().custom = custom; self.shortcut_error = None; }
            Err(e) => self.shortcut_error = Some((String::new(), e.to_string())),
        }
        rebuild(cx);
        cx.refresh_windows();
        cx.notify();
    }

    fn save_recorded_shortcut(&mut self, cx: &mut Context<Self>) {
        let Some(id) = self.shortcut_recording.clone() else { return };
        if self.shortcut_strokes.is_empty() { return; }
        match change(&id, Some(self.shortcut_strokes.join(" ")), cx) {
            Ok(()) => { self.cancel_shortcut_recording(cx); self.shortcut_error = None; }
            Err(e) => { self.shortcut_error = Some((id, e.to_string())); cx.clear_key_bindings(); }
        }
        cx.notify();
    }

    fn clear_shortcut(&mut self, id: &str, cx: &mut Context<Self>) {
        self.cancel_shortcut_recording(cx);
        self.shortcut_error = change(id, Some(String::new()), cx).err().map(|e| (id.to_string(), e.to_string()));
        cx.notify();
    }

    pub(crate) fn record_shortcut(&mut self, ev: &KeyDownEvent, cx: &mut Context<Self>) -> bool {
        let Some(id) = self.shortcut_recording.clone() else { return false };
        cx.stop_propagation();
        if ev.is_held { return true; }
        let plain = ev.keystroke.modifiers == Modifiers::default();
        if plain && ev.keystroke.key == "escape" { self.cancel_shortcut_recording(cx); cx.notify(); return true; }
        if plain && ev.keystroke.key == "enter" { self.save_recorded_shortcut(cx); return true; }
        if plain && ev.keystroke.key == "backspace" { self.shortcut_strokes.pop(); self.shortcut_error = None; cx.notify(); return true; }
        if matches!(ev.keystroke.key.as_str(), "shift" | "alt" | "ctrl" | "cmd" | "fn" | "control" | "platform" | "function") { return true; }
        self.shortcut_error = None;
        let m = ev.keystroke.modifiers;
        if self.shortcut_strokes.is_empty() && !(m.platform || m.control || m.alt) {
            self.shortcut_error = Some((id, first_group_hint().into()));
        } else if self.shortcut_strokes.len() >= if id == "global" { 1 } else { 4 } {
            self.shortcut_error = Some((id, tr("已达到组合键上限，请保存或按退格修改", "Save or press Backspace to edit the sequence").into()));
        } else { self.shortcut_strokes.push(ev.keystroke.unparse()); }
        cx.notify(); true
    }

    fn shortcut_recorder(&self, cx: &mut Context<Self>) -> AnyElement {
        let key = self.shortcut_strokes.join(" ");
        let id = self.shortcut_recording.as_deref().unwrap_or("");
        let replaced = conflicts(cx.global::<Shortcuts>(), id, &key).iter().map(|n| label(n).unwrap_or(n).to_string()).collect::<Vec<_>>();
        div().flex().flex_col().gap(px(6.)).py(px(6.))
            .child(div().text_size(px(12.)).text_color(Theme::text_caption()).child(if id == "global" {
                tr("按下组合键，回车保存 · Esc 取消 · 退格重录", "Press a shortcut, Enter to save · Esc to cancel · Backspace to edit")
            } else { tr("连续输入最多四组，回车保存 · Esc 取消 · 退格修改", "Record up to four strokes, Enter to save · Esc to cancel · Backspace to edit") }))
            .when(!replaced.is_empty(), |d| d.child(div().text_size(px(12.)).text_color(Theme::text_caption()).child(format!("{}{}", tr("保存后将取消以下快捷键绑定：", "Saving will unbind: "), replaced.join("、")))))
            .child(div().flex().gap(px(8.))
                .when(!key.is_empty(), |d| d.child(crate::settings::link_button("shortcut-save", tr("保存", "Save")).on_click(cx.listener(|w, _, _, cx| w.save_recorded_shortcut(cx)))))
                .child(crate::settings::link_button("shortcut-cancel", tr("取消", "Cancel")).on_click(cx.listener(|w, _, _, cx| { w.cancel_shortcut_recording(cx); cx.notify(); }))))
            .into_any_element()
    }

    /// 快捷键's controls on the page title's line: 全部恢复默认 (once something
    /// is changed) and a compact search field, like a toolbar search.
    pub(crate) fn shortcut_actions(&self, cx: &mut Context<Self>) -> AnyElement {
        let customized = cx.try_global::<Shortcuts>().is_some_and(|s| s.custom.keys().any(|k| k != "global"));
        div().flex_none().flex().items_center().gap(px(8.))
            .when(customized, |d| d.child(crate::settings::link_button("shortcuts-reset-all", tr("全部恢复默认", "Reset all")).on_click(cx.listener(|w, _, _, cx| w.reset_all_shortcuts(cx)))))
            .child(div().w(px(200.)).h(px(28.)).px(px(8.)).flex().items_center().gap(px(6.)).rounded(px(8.)).bg(Theme::fill_hover())
                .child(crate::ui::icon("search", 14., Theme::text_caption()))
                .child(div().flex_1().min_w_0().child(self.settings_filter.clone())))
            .into_any_element()
    }

    pub(crate) fn shortcut_settings(&self, cx: &mut Context<Self>) -> AnyElement {
        let q = self.settings_filter.read(cx).text.trim().to_lowercase();
        let customized = cx.try_global::<Shortcuts>().map(|s| s.custom.keys().filter(|k| *k != "global").cloned().collect::<Vec<_>>()).unwrap_or_default();
        let mut groups: Vec<Vec<AnyElement>> = (0..GROUPS.len()).map(|_| vec![]).collect();
        for (id, title, key) in rows(cx).into_iter().filter(|(id, _, _)| id != "global") {
            let Some((g, zh, en)) = info(&id) else { continue };
            if !q.is_empty() && ![zh.to_lowercase(), en.to_lowercase(), pretty(&key).to_lowercase(), key.to_lowercase()].iter().any(|s| s.contains(&q)) { continue; }
            let recording = self.shortcut_recording.as_ref() == Some(&id);
            let error = self.shortcut_error.as_ref().filter(|(e, _)| *e == id).map(|(_, m)| m.clone());
            let is_custom = customized.contains(&id);
            let (rec, reset, clear) = (id.clone(), id.clone(), id.clone());
            let shown_key = if recording { self.shortcut_strokes.join(" ") } else { key.clone() };
            groups[g].push(div().group("shortcut-row").w_full().px(px(16.)).py(px(7.)).flex().flex_col().gap(px(2.))
                .child(div().w_full().min_h(px(28.)).flex().items_center().gap(px(12.))
                    .child(div().flex_1().min_w_0().text_size(px(13.)).line_height(px(20.)).child(title))
                    .when(is_custom && !recording, |d| d.child(div().id(SharedString::from(format!("reset-{reset}"))).h(px(26.)).px(px(8.)).flex().items_center().rounded(px(7.)).cursor_pointer()
                        .text_size(px(12.)).text_color(Theme::primary()).hover(|s| s.bg(Theme::fill_hover())).child(tr("恢复默认", "Reset"))
                        .on_click(cx.listener(move |w, _, _, cx| w.reset_shortcut(&reset, cx)))))
                    .when(!recording && !key.is_empty(), |d| d.child(crate::settings::link_button(SharedString::from(format!("clear-{clear}")), tr("移除", "Remove")).invisible().group_hover("shortcut-row", |s| s.visible()).on_click(cx.listener(move |w, _, _, cx| w.clear_shortcut(&clear, cx)))))
                    .child(crate::settings::keycap(SharedString::from(id.clone()), &shown_key, recording)
                        .on_click(cx.listener(move |w, _, window, cx| w.start_recording(&rec, window, cx)))))
                .when(recording, |d| d.child(self.shortcut_recorder(cx)))
                .children(error.map(|e| div().text_size(px(12.)).line_height(px(17.)).text_color(Theme::danger()).child(e)))
                .into_any_element());
        }
        let empty = groups.iter().all(|g| g.is_empty());
        let general_error = self.shortcut_error.as_ref().filter(|(id, _)| id.is_empty()).map(|(_, e)| e.clone());
        // 快速笔记 from anywhere: the one system-wide shortcut, which can be off.
        let on = crate::settings::prefs().quick_hotkey;
        let global = global_key();
        let global_rec = self.shortcut_recording.as_deref() == Some("global");
        let global_err = self.shortcut_error.as_ref().filter(|(id, _)| id == "global").map(|(_, e)| e.clone())
            .or_else(|| on.then(crate::quick_note::hotkey_error).flatten());
        let matches = |zh: &str, en: &str, key: &str| q.is_empty() || [zh.to_lowercase(), en.to_lowercase(), key.to_lowercase()].iter().any(|s| s.contains(&q));
        let show_global = cfg!(target_os = "macos") && (matches("快速新建笔记", "Quick note", &pretty(&global)) || matches("打开上次使用的文档", "Open last used document", "⌥ ⌘ space"));
        let empty = empty && !show_global;
        let global_group = div().w_full().flex().flex_col().gap(px(8.))
            .child(crate::settings::group(Some(tr("全局", "Global")), vec![
                div().w_full().px(px(16.)).py(px(10.)).flex().items_center().gap(px(12.))
                    .child(div().flex_1().min_w_0().text_size(px(13.)).line_height(px(20.)).child(tr("快速新建笔记", "Quick note")))
                    .when(on && global != GLOBAL_DEFAULT && !global_rec, |d| d.child(crate::settings::link_button("global-reset", tr("恢复默认", "Reset")).on_click(cx.listener(|w, _, _, cx| w.reset_shortcut("global", cx)))))
                    .when(on, |d| d.child(crate::settings::keycap("global-key", &if global_rec { self.shortcut_strokes.join(" ") } else { global.clone() }, global_rec).on_click(cx.listener(|w, _, window, cx| w.start_recording("global", window, cx)))))
                    .child(crate::settings::switch("pref-hotkey", on).on_click(cx.listener(|w, _, _, cx| w.set_pref(|p| p.quick_hotkey = !p.quick_hotkey, cx))))
                    .into_any_element(),
                div().w_full().px(px(16.)).py(px(10.)).flex().items_center().gap(px(12.))
                    .child(div().flex_1().text_size(px(13.)).child(tr("打开上次使用的文档", "Open last used document")))
                    .child(div().text_size(px(13.)).text_color(Theme::text_caption()).child("⌘ ⌥ Space"))
                    .into_any_element(),
            ]))
            .when(global_rec, |d| d.child(self.shortcut_recorder(cx)))
            .children(global_err.map(|e| div().px(px(4.)).text_size(px(12.)).line_height(px(18.)).text_color(Theme::danger()).child(e)));
        div().w_full().flex().flex_col().gap(px(28.))
            .children(general_error.map(|e| div().px(px(4.)).text_size(px(12.)).line_height(px(18.)).text_color(Theme::danger()).child(e)))
            .when(show_global, |d| d.child(global_group))
            .when(empty, |d| d.child(div().py(px(40.)).flex().justify_center().text_size(px(13.)).text_color(Theme::text_caption()).child(tr("没有匹配的快捷键", "No matching shortcuts"))))
            .children(groups.into_iter().enumerate().filter(|(_, rows)| !rows.is_empty()).map(|(g, rows)| {
                let n = rows.len();
                div().w_full().flex().flex_col().gap(px(8.))
                    .child(div().px(px(4.)).text_size(px(13.)).line_height(px(18.)).font_weight(FontWeight::SEMIBOLD).text_color(Theme::sidebar_section()).child(if english() { GROUPS[g].1 } else { GROUPS[g].0 }))
                    .child(div().w_full().flex().flex_col().rounded(px(12.)).border_1().border_color(Theme::line_border()).overflow_hidden()
                        .children(rows.into_iter().enumerate().map(move |(i, r)| div().w_full().flex().flex_col().when(i + 1 < n, |d| d.border_b_1().border_color(crate::ui::list_line())).child(r))))
            }))
            .into_any_element()
    }
}

#[cfg(test)]
mod key_text_tests {
    use super::windows_keys as w;
    #[test]
    fn mac_key_text_reads_as_windows_keys() {
        assert_eq!(w("<code>⌘ ⌥ 1</code>"), "<code>Ctrl+Alt+1</code>");
        assert_eq!(w("<code>⌘ +</code> / <code>⌘ -</code>"), "<code>Ctrl++</code> / <code>Ctrl+-</code>");
        assert_eq!(w("<code>⌘ ⇧ &gt;</code>"), "<code>Ctrl+Shift+&gt;</code>");
        assert_eq!(w("<code>⌃ ⇧ ↑</code>, <code>⇧ Tab</code>, <code>⌘ Enter</code>"), "<code>Ctrl+Shift+↑</code>, <code>Shift+Tab</code>, <code>Ctrl+Enter</code>");
        assert_eq!(w("⌘ + Shift + 7"), "Ctrl + Shift + 7");
        assert_eq!(w("⌘ + Control + C"), "Ctrl + Shift + C");
        assert_eq!(w("⌘ ⌃ C"), "Ctrl+Shift+C");
        assert_eq!(w("搜索(⌘ + J)"), "搜索(Ctrl + J)");
        assert_eq!(w("⌘ + ⌥ + Home"), "Ctrl + Alt + Home");
        assert_eq!(w("⌘C / ⌘X / ⌘V"), "Ctrl+C / Ctrl+X / Ctrl+V");
        assert_eq!(w("⌘⇧← / ⌘⇧↓"), "Shift+Home / Ctrl+Shift+End");
        assert_eq!(w("按 ⌘ N 新建，⌘ ,"), "按 Ctrl+N 新建，Ctrl+,");
        assert_eq!(w("On macOS, <code>⌘ ⌃ ⇧ 4</code>"), "On macOS, <code>⌘ ⌃ ⇧ 4</code>");
        assert_eq!(w("Shift + Tab and Control"), "Shift + Tab and Control");
    }
}
