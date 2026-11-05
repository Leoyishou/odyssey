//! 设置: preferences (`preferences.json` in the library root) and the
//! settings view — a category list in the sidebar, one page per category,
//! each page a column of grouped rows. Only what a
//! user really needs to change lives here.

use crate::{app::{View, Workspace}, theme::{self, craft, Theme}, themes, ui};
use crate::ui::{TitleBarExt, TipExt};
use crate::sidebar as sb;
use gpui::{prelude::FluentBuilder, *};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock, Mutex, RwLock};

// ---- preferences

static FEISHU: AtomicBool = AtomicBool::new(false);
static ENGLISH: AtomicBool = AtomicBool::new(true);
static PREFS: RwLock<Option<Preferences>> = RwLock::new(None);

/// The active document theme draws blocks the 飞书 way (else Craft's).
pub fn feishu() -> bool { FEISHU.load(Ordering::Relaxed) }
pub(crate) fn set_base(feishu: bool) { FEISHU.store(feishu, Ordering::Relaxed); }
pub fn english() -> bool { ENGLISH.load(Ordering::Relaxed) }
pub fn tr(zh: &'static str, en: &'static str) -> &'static str { if english() { en } else { zh } }

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Preferences {
    /// Document theme id (themes.rs); empty in files from before themes, see `feishu`.
    pub theme: String,
    /// The old two-way switch, still written so older builds keep the base style.
    pub feishu: bool,
    pub english: bool,
    /// Fetch `http(s)` images referenced by documents.
    pub remote_images: bool,
    /// The system-wide quick note hotkey is registered.
    pub quick_hotkey: bool,
    /// Look for a new version now and then (update.rs).
    pub auto_update: bool,
    /// Light or dark look: "light", "dark", or empty to follow the system.
    pub appearance: String,
    /// The last version that opened this library (changelog.rs).
    pub seen_version: String,
}

impl Default for Preferences {
    fn default() -> Self { Preferences { theme: String::new(), feishu: true, english: true, remote_images: true, quick_hotkey: true, auto_update: true, appearance: String::new(), seen_version: String::new() } }
}

impl Preferences {
    pub fn theme_id(&self) -> String {
        if !self.theme.is_empty() { self.theme.clone() } else if self.feishu { "feishu".into() } else { "craft".into() }
    }
}

fn path() -> std::path::PathBuf { crate::store::Store::root_dir().join("preferences.json") }

pub fn load() {
    let p = std::fs::read(path()).ok().and_then(|b| serde_json::from_slice::<Preferences>(&b).ok()).unwrap_or_default();
    apply(p);
}

fn apply(p: Preferences) {
    FEISHU.store(p.feishu, Ordering::Relaxed);
    ENGLISH.store(p.english, Ordering::Relaxed);
    // Before the themes are read (startup) the base comes from `feishu`.
    themes::select(&p.theme_id());
    *PREFS.write().unwrap() = Some(p);
}

/// Tell the OS which look the app wants (macOS: the window material, menus
/// and dialogs follow it); empty follows the system.
pub fn apply_appearance(pref: &str) {
    #[cfg(target_os = "macos")]
    unsafe {
        use objc::{class, msg_send, sel, sel_impl};
        use objc::runtime::Object;
        let app: *mut Object = msg_send![class!(NSApplication), sharedApplication];
        let name = match pref { "light" => Some(c"NSAppearanceNameAqua"), "dark" => Some(c"NSAppearanceNameDarkAqua"), _ => None };
        let look: *mut Object = match name {
            Some(n) => { let s: *mut Object = msg_send![class!(NSString), stringWithUTF8String: n.as_ptr()]; msg_send![class!(NSAppearance), appearanceNamed: s] }
            None => std::ptr::null_mut(),
        };
        let _: () = msg_send![app, setAppearance: look];
    }
    #[cfg(not(target_os = "macos"))]
    let _ = pref;
}

/// Draw light or dark as the preference and the system say; true when the look changed.
pub fn sync_dark(cx: &App) -> bool {
    let on = match prefs().appearance.as_str() {
        "light" => false,
        "dark" => true,
        _ => matches!(cx.window_appearance(), WindowAppearance::Dark | WindowAppearance::VibrantDark),
    };
    let changed = theme::set_dark(on);
    if changed { themes::touch(); }
    changed
}

/// The current preferences (loaded on first use).
pub fn prefs() -> Preferences {
    if PREFS.read().unwrap().is_none() { load(); }
    PREFS.read().unwrap().clone().unwrap_or_default()
}

pub(crate) fn save(p: &Preferences) -> anyhow::Result<()> {
    use std::io::Write;
    let root = crate::store::Store::root_dir();
    std::fs::create_dir_all(&root)?;
    let tmp = root.join("preferences.json.tmp");
    let mut f = std::fs::File::create(&tmp)?;
    f.write_all(&serde_json::to_vec_pretty(p)?)?;
    f.sync_all()?;
    std::fs::rename(tmp, path())?;
    apply(p.clone());
    Ok(())
}

/// Redraw every window's document after the theme or language changed.
pub(crate) fn refresh_all(this: &mut Workspace, cx: &mut Context<Workspace>) {
    fn refresh(ws: &mut Workspace, cx: &mut Context<Workspace>) {
        ws.settings_filter.update(cx, |f, cx| { f.placeholder_color = Theme::text_disabled(); f.set_placeholder(tr("搜索快捷键", "Search shortcuts"), cx) });
        ws.search.update(cx, |f, _| f.placeholder_color = Theme::text_disabled());
        if let Some(editor) = &ws.editor {
            editor.update(cx, |e, cx| { e.cache.borrow_mut().clear(); e.refresh_theme(); cx.notify(); });
        }
        cx.notify();
    }
    refresh(this, cx);
    // The other windows. Skip this workspace, and never read a window that is
    // mid-update (the one a click came from): GPUI panics on that, so ask
    // `is_active` first, which is None for a window on the update stack.
    let me = cx.entity_id();
    for w in cx.windows() {
        if let Some(h) = w.downcast::<Workspace>()
            && h.is_active(cx).is_some()
            && h.entity(cx).is_ok_and(|e| e.entity_id() != me)
        {
            let _ = h.update(cx, |ws, _, cx| refresh(ws, cx));
        }
    }
    crate::app::set_menus(cx);
    cx.refresh_windows();
}

/// A theme file in the user's editor; scripted runs only log it (they must
/// not start other apps).
fn open_in_editor(p: &std::path::Path, cx: &mut App) {
    if std::env::var_os("FDL_SCRIPT").is_some() { eprintln!("devtools: open {}", p.display()); return }
    cx.open_url(&format!("file://{}", p.display()));
}

/// Theme files are watched: saving one in an editor redraws at once.
pub(crate) fn watch_themes(cx: &mut Context<Workspace>) {
    static LAST: Mutex<Option<Vec<(std::path::PathBuf, std::time::SystemTime, u64)>>> = Mutex::new(None);
    *LAST.lock().unwrap() = Some(themes::stamp());
    cx.spawn(async |this, cx| loop {
        cx.background_executor().timer(std::time::Duration::from_millis(800)).await;
        let now = cx.background_executor().spawn(async { themes::stamp() }).await;
        let changed = { let mut l = LAST.lock().unwrap(); let c = l.as_ref() != Some(&now); *l = Some(now); c };
        if changed && this.update(cx, |w, cx| w.reload_themes(cx)).is_err() { break }
        if this.upgrade().is_none() { break }
    }).detach();
}

// ---- the view

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Page { #[default] General, Appearance, Shortcuts, Library, About }

impl Page {
    fn title(self) -> &'static str {
        match self {
            Page::General => tr("通用", "General"),
            Page::Appearance => tr("外观", "Appearance"),
            Page::Shortcuts => tr("快捷键", "Shortcuts"),
            Page::Library => tr("文档库", "Documents"),
            Page::About => tr("关于", "About"),
        }
    }
    fn icon(self) -> &'static str {
        match self {
            Page::General => "view-settings",
            Page::Appearance => "palette",
            Page::Shortcuts => "keyboard",
            Page::Library => "storage",
            Page::About => "info",
        }
    }
}

/// The sidebar list, one evenly spaced run (six rows need no unlabelled
/// groups); `None` is 回收站 (its own view, shown with this sidebar).
const NAV: [Option<Page>; 6] = [Some(Page::General), Some(Page::Appearance), Some(Page::Shortcuts), Some(Page::Library), None, Some(Page::About)];

// ---- controls (switch on the accent, 30pt buttons, grouped rows)

const COLUMN_W: f32 = 640.;

/// A titled group: the caption above, then the rows in one rounded card,
/// split by hairlines.
pub(crate) fn group(title: Option<&'static str>, rows: Vec<AnyElement>) -> Div {
    let n = rows.len();
    div().w_full().flex().flex_col().gap(px(8.))
        .children(title.map(caption))
        .child(div().w_full().flex().flex_col().rounded(px(12.)).border_1().border_color(Theme::line_border()).bg(crate::theme::raised(Theme::bg_body())).overflow_hidden()
            .children(rows.into_iter().enumerate().map(move |(i, r)| div().w_full().flex().flex_col().when(i + 1 < n, |d| d.border_b_1().border_color(ui::list_line())).child(r))))
}

pub(crate) fn caption(t: &'static str) -> Div {
    div().px(px(4.)).text_size(px(13.)).line_height(px(18.)).font_weight(FontWeight::SEMIBOLD).text_color(Theme::sidebar_section()).child(t)
}

/// One setting: title and an optional description on the left, the control on the right.
fn row(title: impl Into<SharedString>, desc: Option<SharedString>, control: impl IntoElement) -> Div {
    div().w_full().min_h(px(48.)).px(px(16.)).py(px(10.)).flex().items_center().justify_between().gap(px(24.))
        .child(div().flex_1().min_w_0().flex().flex_col().gap(px(2.))
            .child(div().text_size(px(13.)).line_height(px(20.)).text_color(Theme::text()).child(title.into()))
            .children(desc.map(|d| div().text_size(px(12.)).line_height(px(17.)).text_color(Theme::text_caption()).child(d))))
        .child(div().flex_none().flex().items_center().gap(px(8.)).child(control))
}

fn desc(s: &'static str) -> Option<SharedString> { Some(s.into()) }

/// The switch: the accent when on, black 16% when off, a white knob.
pub(crate) fn switch(id: impl Into<ElementId>, on: bool) -> Stateful<Div> {
    div().id(id).flex_none().w(px(32.)).h(px(19.)).p(px(2.)).rounded_full().cursor_pointer().flex().items_center()
        .when(on, |d| d.bg(Theme::primary()).justify_end())
        .when(!on, |d| d.bg(theme::ca(craft::N1000, 0.16)).justify_start())
        .child(div().size(px(15.)).rounded_full().bg(gpui::white()).shadow(theme::shadow_button()))
}

/// A segmented control: a grey groove and a white selected pill.
fn segmented(id: &'static str, labels: &[&'static str], selected: usize, on: impl Fn(&mut Workspace, usize, &mut Context<Workspace>) + Clone + 'static, cx: &mut Context<Workspace>) -> Div {
    div().flex_none().h(px(28.)).p(px(2.)).flex().gap(px(2.)).rounded(px(9.)).bg(Theme::fill_hover_strong())
        .children(labels.iter().enumerate().map(|(i, label)| {
            let on = on.clone();
            div().id((id, i)).h_full().px(px(12.)).flex().items_center().justify_center().rounded(px(7.)).cursor_pointer()
                .text_size(px(13.)).line_height(px(18.))
                // In dark mode the picked segment is lighter than the groove (macOS: #636366).
                .when(i == selected, |d| d.bg(if theme::dark() { theme::rgbaf(0x63, 0x63, 0x66, 1.) } else { Theme::bg_body() }).shadow(theme::shadow_button()).text_color(Theme::text()))
                .when(i != selected, |d| d.text_color(Theme::text_caption()).hover(|s| s.text_color(Theme::text())))
                .child(*label)
                .on_click(cx.listener(move |w, _, _, cx| on(w, i, cx)))
        }))
}

/// A keyboard shortcut chip; `recording` shows it waiting for keys.
pub(crate) fn keycap(id: impl Into<ElementId>, key: &str, recording: bool) -> Stateful<Div> {
    div().id(id).flex_none().min_w(px(144.)).min_h(px(28.)).px(px(14.)).py(px(4.)).flex().items_center().justify_center().rounded(px(7.)).cursor_pointer()
        .text_size(px(12.)).line_height(px(16.))
        .when(recording, |d| d.bg(crate::theme::raised(Theme::bg_body())).border_1().border_color(Theme::primary()).text_color(Theme::primary()).child(if key.is_empty() { tr("按下新的组合键…", "Press keys…").to_string() } else { crate::shortcuts::pretty(key) }))
        .when(!recording, |d| d.bg(Theme::fill_hover()).border_1().border_color(transparent_black()).text_color(Theme::text()).hover(|s| s.bg(Theme::fill_hover_strong())).child(crate::shortcuts::pretty(key)))
}

/// A small text button (恢复默认 …).
pub(crate) fn link_button(id: impl Into<ElementId>, label: &'static str) -> Stateful<Div> {
    div().id(id).flex_none().h(px(26.)).px(px(10.)).flex().items_center().rounded(px(7.)).cursor_pointer()
        .text_size(px(12.)).line_height(px(16.)).text_color(Theme::primary()).hover(|s| s.bg(Theme::fill_hover()))
        .child(label)
}

pub(crate) fn button(id: impl Into<ElementId>, label: &'static str) -> Stateful<Div> {
    div().id(id).flex_none().h(px(28.)).px(px(12.)).flex().items_center().justify_center().rounded(px(8.)).cursor_pointer()
        .bg(crate::theme::raised(Theme::bg_body())).border_1().border_color(Theme::button_border()).shadow(theme::shadow_button())
        .hover(|s| s.bg(theme::c(craft::N100))).active(|s| s.bg(theme::c(craft::N200)))
        .text_size(px(13.)).line_height(px(18.)).text_color(Theme::text()).child(label)
}

// ---- theme preview

/// Preview scale against the real document. Its frame clips to a rectangle
/// (GPUI masks have no radius), so the page is rounded inside the frame.
const PAGE_S: f32 = 0.78;

/// 外观's sample page: a short document with most of what a theme styles
/// (headings, inline styles, lists, a task, quote, callout, table, divider,
/// highlighted code), at scale `s`, in two columns so it reads at a glance.
fn sample_page(t: &themes::Theme, s: f32) -> Div {
    let st = t.style;
    let sc = |v: f32| px((v * s).max(0.5));
    let text = st.color("page.color").unwrap_or_else(|| st.token("text"));
    let (body, line) = (st.num("page.size"), st.num("page.line"));
    let (font, mono): (SharedString, SharedString) = (st.font("page.font").into(), st.font("pre.font").into());
    let gap = sc(st.num("p.gap")).clamp(px(4.), px(9.));
    let p = |c: Div| c.text_size(sc(body)).line_height(sc(line));
    let heading = |l: u8, s: &'static str| {
        let h = format!("h{l}");
        div().text_size(sc(st.num(&format!("{h}.size")))).line_height(sc(st.num(&format!("{h}.line")))).font_weight(FontWeight(st.num(&format!("{h}.weight"))))
            .text_color(st.color(&format!("{h}.color")).unwrap_or(text)).child(s)
    };
    let link = st.color("a.color").unwrap_or(text);
    let chip = |s: &'static str| div().px(sc(st.num("code.pad")).max(px(2.))).rounded(sc(st.num("code.radius"))).bg(st.color("code.bg").unwrap_or_else(|| st.token("code_chip")))
        .border_1().border_color(st.color("code.border").unwrap_or_else(|| st.token("code_chip_border"))).font_family(mono.clone()).text_color(st.color("code.color").unwrap_or(text)).child(s);
    let para = p(div().flex().flex_wrap().items_center())
        .child(tr("Odyssey 是一款", "Odyssey is a "))
        .child(div().font_weight(FontWeight(st.num("strong.weight"))).text_color(st.color("strong.color").unwrap_or(text)).child(tr("安静的本地笔记应用", "quiet, local notes app")))
        .child(tr("，", ", "))
        .child(tr("文档就是电脑上的文件。", "your notes are files. "))
        .child(tr("输入 ", "Type "))
        .child(chip("/"))
        .child(tr(" 插入内容，", " to insert, "))
        .child(tr("详见", "see "))
        // The full stop rides with the link, so it never wraps alone.
        .child(div().flex().child(div().text_color(link).when(st.flag("a.underline"), |d| d.underline()).child(tr("写作基础", "Writing basics"))).child(tr("。", ".")));
    let indent = sc(st.num("list.indent")).max(px(10.));
    let marker = |m: Div| div().flex_none().w(indent).flex().justify_center().child(m);
    let bullet_c = st.color("bullet.color").unwrap_or(text);
    let bullet = |s: &'static str| div().flex().items_center().child(marker(div().size(sc(6.).max(px(3.))).rounded_full().bg(bullet_c))).child(p(div()).child(s));
    let num_c = st.color("number.color").unwrap_or(text);
    let number = |n: &'static str, s: &'static str| div().flex().items_center().child(marker(p(div()).text_color(num_c).child(n))).child(p(div()).child(s));
    let task_size = st.num("task.size");
    let task = |done: bool, label: &'static str| div().flex().items_center().child(marker(
            div().size(sc(task_size)).rounded(sc(st.num("task.radius"))).flex().items_center().justify_center()
                .when(done, |d| d.bg(st.color("task.checked").unwrap_or_else(|| st.token("primary")))
                    .child(ui::icon("todo-check", (task_size * s * 0.75).round(), st.color("task.check").unwrap_or_else(gpui::white))))
                .when(!done, |d| d.border_1().border_color(st.color("task.border").unwrap_or_else(|| st.token("todo_box"))))))
        .child(p(div()).child(label).when(done, |d| d.text_color(st.color("done.color").unwrap_or_else(|| st.token("todo_done"))).when(st.flag("done.strike"), |d| d.line_through())));
    let quote = div().flex().gap(sc((st.num("quote.indent") - st.num("quote.bar")).max(4.)))
        .child(div().flex_none().w(sc(st.num("quote.bar")).max(px(1.))).rounded(px(1.)).bg(st.color("quote.bar-color").unwrap_or_else(|| st.token("quote_bar"))))
        .child(p(div()).text_color(st.color("quote.color").unwrap_or(text)).when(st.flag("quote.italic"), |d| d.italic()).child(tr("每一篇指南都是用 Odyssey 写成的，放心修改。", "Every guide is written in Odyssey. Edit them freely.")));
    let callout = div().px(sc(st.num("callout.pad-x"))).py(sc(st.num("callout.pad-y"))).rounded(sc(st.num("callout.radius")))
        .bg(st.named("callout", "light-yellow").unwrap_or_else(|| st.token("code_chip")))
        .when(st.num("callout.border") > 0., |d| d.border(sc(st.num("callout.border")).max(px(1.))).border_color(st.named("border", "yellow").unwrap_or_else(|| st.token("code_chip_border"))))
        .child(p(div().flex().gap(px(6.))).child("💡").child(tr("一边读一边动手试，记得最快。", "Try things as you read; it sticks faster.")));
    let line_c = st.color("table.line").unwrap_or_else(|| st.token("table_line"));
    let cell = |s: &'static str, head: bool| div().flex_1().px(px(6.)).py(px(3.)).when(head, |d| d.bg(st.color("th.bg").unwrap_or_else(|| st.token("table_header"))).font_weight(FontWeight::SEMIBOLD)).child(s);
    let row = |a: &'static str, b: &'static str, head: bool, last: bool| div().flex().when(!last, |d| d.border_b_1().border_color(line_c))
        .child(cell(a, head).border_r_1().border_color(line_c)).child(cell(b, head));
    let table = div().w_full().rounded(sc(st.num("table.radius"))).border_1().border_color(line_c).overflow_hidden()
        .text_size(sc(st.num("table.size"))).line_height(sc(st.num("table.line-height")))
        .child(row(tr("按键", "Keys"), tr("作用", "Does"), true, false)).child(row(crate::shortcuts::keys("⌘ N"), tr("新建文档", "New document"), false, false)).child(row(crate::shortcuts::keys("⌘ J"), tr("搜索所有文档", "Search documents"), false, true));
    let (hr_c, hr_w) = (st.color("hr.color").unwrap_or_else(|| st.token("separator_dots")), (st.num("hr.width") * s).max(0.6));
    // Painted to the column's width: a row of fixed-size dots would set the
    // column's minimum width and squeeze its neighbour.
    let hr_style = st.line("hr.style");
    let hr = canvas(|_, _, _| {}, move |b, _, window, _| {
        let (step, dash) = match hr_style { themes::LineStyle::Solid => return window.paint_quad(fill(Bounds::new(b.origin, size(b.size.width, px(hr_w))), hr_c)), themes::LineStyle::Dashed => (8., 4.), themes::LineStyle::Dotted => (hr_w + 2.5, hr_w) };
        let mut x = 0.;
        while x + dash <= f32::from(b.size.width) {
            window.paint_quad(fill(Bounds::new(point(b.origin.x + px(x), b.origin.y), size(px(dash), px(hr_w))), hr_c).corner_radii(px(if hr_style == themes::LineStyle::Dotted { hr_w / 2. } else { 0. })));
            x += step;
        }
    }).w_full().h(px(hr_w));
    let syn = |k: &str| st.color(&format!("syntax.{k}")).unwrap_or(text);
    let tok = |s: &'static str, c: Hsla| div().text_color(c).child(s);
    let code_line = |n: &'static str, parts: Vec<Div>| div().flex()
        .when(st.flag("pre.numbers"), |d| d.child(div().flex_none().w(px(16.)).mr(px(8.)).text_color(st.color("pre.number-color").unwrap_or_else(|| st.token("code_line_number"))).child(n)))
        .children(parts);
    let code = div().w_full().overflow_hidden().px(px(10.)).py(px(7.)).rounded(sc(st.num("pre.radius"))).bg(st.color("pre.bg").unwrap_or_else(|| st.token("code_bg")))
        .border_1().border_color(st.color("pre.border").unwrap_or_else(|| st.token("code_border")))
        .font_family(mono.clone()).text_size(sc(st.num("pre.size"))).line_height(sc(st.num("pre.line"))).text_color(st.color("pre.color").unwrap_or_else(|| st.token("code_text")))
        .child(code_line("1", vec![tok("# ", syn("comment")), tok(tr("备份文档库", "back up your documents"), syn("comment"))]))
        .child(code_line("2", vec![tok("cd ", syn("function")), tok("~/Documents ", text), tok("&& ", syn("keyword")), tok("tar ", syn("function")), tok("-czf ", syn("attr")), tok("\"Quiet.tar.gz\" ", syn("string")), tok("Odyssey", text)]))
        .child(code_line("3", vec![tok("echo ", syn("function")), tok("\"done\"", syn("string"))]));
    // One column, like a page: a centred text column (the theme's width, scaled,
    // up to 760 so a wide frame isn't mostly margin) whose bottom fades into
    // the paper. Blocks come in the order the 开始指南 itself uses.
    let paper = st.color("page.bg").unwrap_or_else(|| st.token("bg_body"));
    // Its own bottom corners too: nothing clips it to the page's radius.
    let fade = div().absolute().left_0().right_0().bottom_0().h(px(56.)).rounded_b(px(7.)).bg(linear_gradient(180., linear_color_stop(paper.opacity(0.), 0.), linear_color_stop(paper, 1.)));
    // items_start: a column stretched to the frame would shrink its blocks to fit.
    div().relative().size_full().flex().justify_center().items_start().bg(paper).font_family(font).text_color(text).overflow_hidden()
        // min_w_0: the column follows the frame, not its widest unbreakable line.
        .child(div().w_full().min_w_0().overflow_hidden().max_w((sc(st.num("page.width")) * 1.35).min(px(760.))).px(px(32.)).pt(px(22.)).flex().flex_col().gap(gap)
            .child(div().text_size(sc(st.num("title.size"))).line_height(sc(st.num("title.size") * 1.25)).font_weight(FontWeight(st.num("title.weight")))
                .font_family(SharedString::from(st.font("title.font"))).text_color(st.color("title.color").unwrap_or(text)).child(tr("开始指南", "Getting started")))
            .child(para)
            .child(callout)
            .child(heading(1, tr("五分钟上手", "Five minutes")))
            .child(div().flex().flex_col().child(task(true, tr("打开这份指南", "Open this guide"))).child(task(false, crate::shortcuts::keys(tr("按 ⌘ N 新建一篇文档", "Press ⌘ N for a new document")))))
            .child(heading(2, tr("三种输入方式", "Three ways to write")))
            .child(div().flex().flex_col().child(bullet(tr("斜杠菜单：输入 / 插入任意内容块", "Slash menu: type / to insert any block"))).child(bullet(tr("Markdown：# 加空格变标题", "Markdown: # and a space make a heading"))))
            .child(div().flex().flex_col().child(number("1.", tr("先读「写作基础」", "Read Writing basics first"))).child(number("2.", tr("再看「内容块」", "Then Content blocks"))))
            .child(quote)
            .child(heading(3, tr("最常用的快捷键", "Shortcuts to know")))
            .child(table)
            .child(div().w_full().py(px(2.)).child(hr))
            .child(code))
        .child(fade)
}

fn badge(t: &themes::Theme) -> Option<&'static str> {
    if t.customized() { Some(tr("已修改", "Edited")) } else if t.bundled.is_none() { Some(tr("自定义", "Custom")) } else { None }
}

/// A pop-up button (macOS): the current choice and a chevron on `button`'s white pill.
fn popup_button(id: impl Into<ElementId>, label: impl Into<SharedString>, open: bool) -> Stateful<Div> {
    div().id(id).flex_none().max_w(px(220.)).h(px(28.)).pl(px(12.)).pr(px(8.)).flex().items_center().gap(px(6.)).rounded(px(8.)).cursor_pointer()
        .bg(if open { theme::c(craft::N100) } else { Theme::bg_body() }).border_1().border_color(Theme::button_border()).shadow(theme::shadow_button())
        .hover(|s| s.bg(theme::c(craft::N100))).active(|s| s.bg(theme::c(craft::N200)))
        .text_size(px(13.)).line_height(px(18.)).text_color(Theme::text())
        .child(div().min_w_0().truncate().child(label.into()))
        .child(ui::icon("chevron-down", 12., Theme::text_caption()))
}

fn apply_theme(w: &mut Workspace, id: String, cx: &mut Context<Workspace>) {
    let feishu = themes::themes().iter().find(|t| t.id == id).is_some_and(|t| t.style.feishu);
    w.theme_menu = None;
    w.set_pref(move |p| { p.theme = id; p.feishu = feishu; }, cx)
}

impl Workspace {
    /// A compact theme list; applying a choice updates the page preview below.
    pub(crate) fn render_theme_menu(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let focused = self.theme_menu.as_ref()?;
        if self.view != View::Settings || self.settings_page != Page::Appearance { return None }
        let b = self.theme_anchor.get()?;
        let current = prefs().theme_id();
        let list = themes::themes();
        let rows: Vec<AnyElement> = list.iter().map(|t| {
            let g = SharedString::from(format!("theme-opt-{}", t.id));
            let (id, hover_id) = (t.id.clone(), t.id.clone());
            ui::menu_row(g.clone(), g.clone())
                .when(&t.id == focused, |d| d.bg(Theme::fill_hover()))
                .child(div().flex_1().min_w_0().overflow_hidden().child(ui::menu_label(t.name().to_string(), g.clone(), gpui::white())))
                .children(badge(t).map(|b| div().flex_none().text_size(px(11.)).text_color(Theme::text_caption()).child(ui::menu_label(b, g.clone(), gpui::white()))))
                .child(div().flex_none().w(px(14.)).when(t.id == current, |d| d.child(ui::menu_glyph("check", 14., g.clone()))))
                .on_hover(cx.listener(move |w, hovered: &bool, _, cx| if *hovered && w.theme_menu.as_ref() != Some(&hover_id) { w.theme_menu = Some(hover_id.clone()); cx.notify(); }))
                .on_click(cx.listener(move |w, _, _, cx| apply_theme(w, id.clone(), cx)))
                .into_any_element()
        }).collect();
        drop(list);
        let anchor = self.theme_anchor.clone();
        let menu = ui::popover().id("theme-menu").occlude().flex()
            .on_mouse_down_out(cx.listener(move |w, ev: &MouseDownEvent, _, cx| {
                // A press on the button itself: its click closes the menu.
                if anchor.get().is_some_and(|b| b.contains(&ev.position)) { return }
                w.theme_menu = None;
                cx.notify();
            }))
            .child(div().id("theme-menu-list").w(px(200.)).max_h(px(302.)).overflow_y_scroll().py(px(ui::MENU_PAD_Y)).flex().flex_col().children(rows));
        Some(deferred(anchored().anchor(Corner::TopRight).position(point(b.right(), b.bottom() + px(4.))).snap_to_window_with_margin(px(8.)).child(menu)).with_priority(2).into_any_element())
    }

    /// Keys while the theme dropdown is open; true when used.
    pub(crate) fn theme_menu_key(&mut self, key: &str, cx: &mut Context<Self>) -> bool {
        let ids: Vec<String> = themes::themes().iter().map(|t| t.id.clone()).collect();
        let at = self.theme_menu.as_ref().and_then(|p| ids.iter().position(|id| id == p)).unwrap_or(0);
        match key {
            "escape" => self.theme_menu = None,
            "up" => self.theme_menu = Some(ids[at.saturating_sub(1)].clone()),
            "down" => self.theme_menu = Some(ids[(at + 1).min(ids.len().saturating_sub(1))].clone()),
            "enter" | "space" => { apply_theme(self, ids[at].clone(), cx); return true }
            _ => return false,
        }
        cx.notify();
        true
    }
}

impl Workspace {
    /// Change preferences and refresh what depends on them.
    pub(crate) fn set_pref(&mut self, f: impl FnOnce(&mut Preferences), cx: &mut Context<Self>) {
        let old = prefs();
        let mut p = old.clone();
        f(&mut p);
        if let Err(e) = save(&p) {
            self.show_toast_kind(ui::ToastKind::Error, &format!("{}: {e}", tr("设置保存失败", "Could not save settings")), cx);
            return;
        }
        if p.quick_hotkey != old.quick_hotkey {
            if p.quick_hotkey { crate::quick_note::register(cx); } else { crate::quick_note::suspend_hotkey(); }
        }
        if p.appearance != old.appearance { apply_appearance(&p.appearance); }
        if p.theme_id() != old.theme_id() || p.english != old.english || sync_dark(cx) {
            refresh_all(self, cx);
        }
        cx.notify();
    }

    pub(crate) fn reload_themes(&mut self, cx: &mut Context<Self>) {
        themes::load(&prefs().theme_id(), themes::all_loaded(), || cx.text_system().all_font_names());
        refresh_all(self, cx);
    }

    /// Open the theme's stylesheet in the user's editor; a built-in theme is
    /// first copied in full into the themes folder (the copy replaces it).
    fn edit_theme(&mut self, id: &str, cx: &mut Context<Self>) {
        match themes::editable_file(id) {
            Ok(p) => {
                self.reload_themes(cx);
                open_in_editor(&p, cx);
            }
            Err(e) => self.show_toast_kind(ui::ToastKind::Error, &e.to_string(), cx),
        }
    }

    fn duplicate_theme(&mut self, id: &str, cx: &mut Context<Self>) {
        match themes::duplicate(id) {
            Ok((new_id, p)) => {
                themes::load(&new_id, themes::all_loaded(), || cx.text_system().all_font_names());
                let feishu = themes::themes().iter().find(|t| t.id == new_id).is_some_and(|t| t.style.feishu);
                self.set_pref(move |pr| { pr.theme = new_id; pr.feishu = feishu; }, cx);
                open_in_editor(&p, cx);
            }
            Err(e) => self.show_toast_kind(ui::ToastKind::Error, &e.to_string(), cx),
        }
    }

    fn remove_theme(&mut self, id: &str, cx: &mut Context<Self>) {
        if let Err(e) = themes::remove(id) { return self.show_toast_kind(ui::ToastKind::Error, &e.to_string(), cx) }
        self.reload_themes(cx);
        // A deleted theme of the user's own: back to Craft.
        if !themes::themes().iter().any(|t| t.id == id) { self.set_pref(|p| { p.theme = "craft".into(); p.feishu = false; }, cx); }
    }

    pub(crate) fn open_settings(&mut self, page: Page, cx: &mut Context<Self>) {
        self.cancel_shortcut_recording(cx);
        self.settings_page = page;
        self.theme_menu = None;
        self.settings_scroll.set_offset(point(px(0.), px(0.)));
        self.shortcut_error = None;
        if self.view != View::Settings { self.show_view(View::Settings, cx); }
        cx.notify();
    }

    pub(crate) fn render_settings_sidebar(&self, window: &Window, cx: &mut Context<Self>) -> AnyElement {
        let current = (self.view == View::Settings).then_some(self.settings_page);
        let in_trash = self.view == View::Trash;
        let nav_row = |page: Option<Page>, cx: &mut Context<Self>| {
            let on = if page.is_some() { page == current } else { in_trash };
            let (title, icon) = page.map_or((tr("回收站", "Trash"), "bin"), |p| (p.title(), p.icon()));
            div().id(SharedString::from(format!("settings-nav-{page:?}"))).h(px(sb::ROW_H)).flex().items_center().pl(px(6.)).pr(px(4.)).rounded(px(sb::ROW_R))
                .text_size(px(13.)).line_height(px(20.)).cursor_pointer()
                .when(on, |d| d.bg(Theme::sidebar_selected()).text_color(Theme::text()))
                .when(!on, |d| d.text_color(Theme::sidebar_text()).hover(|s| s.bg(Theme::sidebar_hover())))
                .child(div().size(px(20.)).flex_none().flex().items_center().justify_center().mr(px(9.)).child(ui::icon(icon, sb::ROW_ICON, if on { Theme::text_code() } else { Theme::icon() })))
                .child(title)
                .on_click(cx.listener(move |w, _, _, cx| match page { Some(p) => w.open_settings(p, cx), None => w.show_view(View::Trash, cx) }))
        };
        div().w(px(self.sidebar_w(window))).h_full().flex_none().flex().flex_col().text_color(Theme::text()).font_family(theme::UI_FONT)
            .child(div().h(px(crate::app::HEADER_H)).flex_none().flex().items_center().pl(px(theme::TRAFFIC_LIGHTS_W)).pr(px(12.)).title_bar().child(self.header_navigation(cx))
                .child(div().flex_1()).children(update_pill(cx)))
            // The document sidebar's rhythm: 主页 sits 8 under the bar, a section
            // header like 我的文档's, rows on a 31 pitch. 设置's line sits on
            // 主页's, not a section header's: the 34pt box keeps the rows below.
            .child(div().pt(px(8.)).px(px(sb::SB_X)).flex().flex_col()
                .child(div().h(px(sb::SECTION_H)).pl(px(4.)).pt(px((sb::SECTION_TOP + sb::SECTION_H) / 2. - sb::ROW_H / 2.))
                    .child(div().px(px(4.)).text_size(px(13.)).line_height(px(20.)).font_weight(FontWeight::SEMIBOLD).text_color(Theme::sidebar_section()).child(tr("设置", "Settings"))))
                .child(div().flex().flex_col().gap(px(sb::ROW_GAP)).children(NAV.iter().map(|p| nav_row(*p, cx)).collect::<Vec<_>>())))
            // Where 设置 was: 返回, back to the page 设置 was opened from.
            .child(div().flex_1())
            .child(sb::footer_slot(sb::footer_row("ft-back", "arrow-left", tr("返回", "Back")).tip(tr("返回", "Back"), Some("Esc"))
                .on_click(cx.listener(|w, _, _, cx| w.close_settings(cx)))))
            .into_any_element()
    }

    /// 收起 / 展开侧栏 and 主页, in the very box the document sidebar's
    /// 收起侧栏 uses (28pt, 4pt past the traffic lights, glyph 17), so the
    /// buttons don't move when switching between the document list and 设置.
    pub(crate) fn header_navigation(&self, cx: &mut Context<Self>) -> AnyElement {
        let open = self.sidebar_open;
        let bar_button = |id: &'static str, glyph: &'static str| {
            div().id(id).no_drag().flex_none().size(px(28.)).flex().items_center().justify_center().rounded(px(8.)).cursor_pointer()
                .hover(move |s| s.bg(if open { Theme::sidebar_hover() } else { Theme::fill_hover_strong() }))
                .child(ui::icon(glyph, 17., Theme::icon()))
        };
        div().flex().items_center().ml(px(4.)).gap(px(4.))
            .child(bar_button("settings-collapse", if open { "panel-left-close" } else { "panel-left-open" })
                .tip(if open { tr("收起侧栏", "Hide sidebar") } else { tr("展开侧栏", "Show sidebar") }, Some(crate::shortcuts::hint("workspace::ToggleSidebar", "⌘ + ]")))
                .on_click(cx.listener(|w, _, _, cx| { w.sidebar_open = !w.sidebar_open; cx.notify(); })))
            .child(bar_button("settings-home", "home").tip(tr("返回主页", "Home"), Some(crate::shortcuts::hint("workspace::GoHome", "⌘ + ⌥ + Home")))
                .on_click(cx.listener(|w, _, _, cx| w.show_view(View::Home, cx))))
            .into_any_element()
    }

    /// Back to where 设置 was opened from.
    pub(crate) fn close_settings(&mut self, cx: &mut Context<Self>) {
        let back = if self.editor.is_some() && self.current.is_some() { View::Doc } else { View::Home };
        self.show_view(back, cx);
    }

    pub(crate) fn render_settings(&self, cx: &mut Context<Self>) -> AnyElement {
        let page = self.settings_page;
        let body = match page {
            Page::General => self.general_page(cx),
            Page::Appearance => self.appearance_page(cx),
            Page::Shortcuts => self.shortcut_settings(cx),
            Page::Library => self.library_page(cx),
            Page::About => self.about_page(cx),
        };
        div().flex_1().min_w_0().h_full().flex().flex_col().bg(Theme::bg_body()).text_color(Theme::text()).font_family(theme::UI_FONT)
            // The bar only drags the window; the page title heads the centred column.
            .child(div().h(px(crate::app::HEADER_H)).flex_none().flex().items_center().title_bar()
                .when(!self.sidebar_open, |d| d.pl(px(theme::TRAFFIC_LIGHTS_W)).child(self.header_navigation(cx))))
            .child(div().id("settings-content").flex_1().min_h_0().overflow_y_scroll().track_scroll(&self.settings_scroll)
                .child(div().w_full().flex().flex_col().items_center().px(px(32.)).pb(px(64.))
                    .child(div().w_full().max_w(px(if page == Page::Shortcuts { 760. } else { COLUMN_W })).flex().flex_col().gap(px(28.))
                        .child(div().flex().items_center().gap(px(16.))
                            .child(div().flex_1().min_w_0().px(px(4.)).text_size(px(22.)).line_height(px(28.)).font_weight(FontWeight::SEMIBOLD).child(page.title()))
                            .when(page == Page::Shortcuts, |d| d.child(self.shortcut_actions(cx))))
                        .child(body))))
            .into_any_element()
    }

    fn general_page(&self, cx: &mut Context<Self>) -> AnyElement {
        let p = prefs();
        div().w_full().flex().flex_col().gap(px(28.))
            .child(group(None, vec![
                row(tr("界面语言", "Interface language"), None,
                    segmented("lang", &["简体中文", "English"], p.english as usize, |w, i, cx| w.set_pref(|p| p.english = i == 1, cx), cx)).into_any_element(),
                row(tr("自动下载网络图片", "Download web images"), desc(tr("关闭后不联网，网络图片显示为占位，点击单独加载。", "When off the app stays offline; web images show a placeholder you can click to load.")),
                    switch("pref-remote", p.remote_images).on_click(cx.listener(|w, _, _, cx| w.set_pref(|p| p.remote_images = !p.remote_images, cx)))).into_any_element(),
                row(tr("自动检查更新", "Check for updates automatically"), desc(tr("只读取一份版本信息，不发送任何数据；新版本由你决定何时下载安装。", "Reads a version file and sends nothing about you; you decide when to download and install.")),
                    switch("pref-update", p.auto_update).on_click(cx.listener(|w, _, _, cx| w.set_pref(|p| p.auto_update = !p.auto_update, cx)))).into_any_element(),
            ]))
            .into_any_element()
    }

    fn appearance_page(&self, cx: &mut Context<Self>) -> AnyElement {
        let current = prefs().theme_id();
        let list = themes::themes();
        let sel = list.iter().find(|t| t.id == current).or_else(|| list.first());
        let (sel_id, sel_name, sel_path, customized, own, warnings) = match sel {
            Some(t) => (t.id.clone(), t.name().to_string(), t.path.clone(), t.customized(), t.bundled.is_none(), t.warnings.clone()),
            None => ("craft".into(), "Craft".into(), None, false, false, vec![]),
        };
        drop(list);
        let errors = themes::errors();
        let where_: SharedString = match &sel_path {
            // Inside the library: the short path (the full one wraps on narrow windows).
            Some(p) => p.strip_prefix(crate::store::Store::root_dir()).unwrap_or(p).display().to_string().into(),
            None => tr("内置主题", "Built in").into(),
        };
        let (edit_id, dup_id, rm_id, open_id) = (sel_id.clone(), sel_id.clone(), sel_id.clone(), sel_id);
        let anchor = self.theme_anchor.clone();
        let picker = popup_button("theme-picker", sel_name, self.theme_menu.is_some())
            .child(canvas(move |b, _, _| anchor.set(Some(b)), |_, _, _, _| {}).absolute().inset_0())
            .relative()
            .on_click(cx.listener(move |w, _, _, cx| {
                if w.theme_menu.is_none() && !themes::all_loaded() {
                    themes::load(&prefs().theme_id(), true, || cx.text_system().all_font_names());
                }
                w.theme_menu = if w.theme_menu.is_some() { None } else { Some(open_id.clone()) };
                cx.notify();
            }));
        // One group, top to bottom: the choice, how it looks, its stylesheet.
        // The preview is a page of the theme on a quiet band, like a sheet on a desk.
        let shown = { let list = themes::themes(); list.iter().find(|t| t.id == current).or_else(|| list.first()).map(|t| sample_page(t, PAGE_S).rounded(px(7.))) };
        let sample = div().w_full().px(px(16.)).py(px(16.)).flex().justify_center().bg(Theme::fill_hover())
            .child(div().w_full().h(px(360.)).rounded(px(8.)).overflow_hidden().border_1().border_color(Theme::line_border()).shadow(theme::shadow_button()).children(shown));
        div().w_full().flex().flex_col().gap(px(28.))
            .child(div().w_full().flex().flex_col().gap(px(8.))
                .child(group(None, vec![
                    row(tr("外观", "Appearance"), None, {
                        let at = match prefs().appearance.as_str() { "light" => 1, "dark" => 2, _ => 0 };
                        segmented("appearance", &[tr("跟随系统", "System"), tr("浅色", "Light"), tr("深色", "Dark")], at,
                            |w, i, cx| w.set_pref(|p| p.appearance = ["", "light", "dark"][i].into(), cx), cx)
                    }).into_any_element(),
                    row(tr("文档主题", "Document theme"), None, picker).into_any_element(),
                    sample.into_any_element(),
                    row(tr("主题 CSS", "Theme CSS"), Some(where_),
                        div().flex().items_center().gap(px(8.))
                            .when(customized || own, |d| d.child(link_button("theme-remove", if customized { tr("恢复默认", "Restore") } else { tr("删除", "Delete") })
                                .on_click(cx.listener(move |w, _, _, cx| w.remove_theme(&rm_id, cx)))))
                            .child(button("theme-duplicate", tr("复制为新主题", "Duplicate")).on_click(cx.listener(move |w, _, _, cx| w.duplicate_theme(&dup_id, cx))))
                            .child(button("theme-edit", tr("编辑 CSS", "Edit CSS")).on_click(cx.listener(move |w, _, _, cx| w.edit_theme(&edit_id, cx))))).into_any_element(),
                ]))
                .children(errors.into_iter().chain(warnings.into_iter().take(8).map(|w| format!("{}：{w}", tr("未生效", "Ignored")))).map(|e| div().px(px(4.)).text_size(px(12.)).line_height(px(18.)).text_color(Theme::danger()).child(e)))
                )
            .into_any_element()
    }

    fn library_page(&self, cx: &mut Context<Self>) -> AnyElement {
        let root = crate::store::Store::root_dir();
        let location = if cfg!(target_os = "windows") { root.display().to_string() } else {
            dirs::home_dir().and_then(|h| root.strip_prefix(h).ok().map(|r| format!("~/{}", r.display()))).unwrap_or_else(|| root.display().to_string())
        };
        let reveal = if cfg!(target_os = "macos") { tr("在访达中显示", "Show in Finder") } else if cfg!(target_os = "windows") {
            tr("在文件资源管理器中显示", "Show in File Explorer")
        } else { tr("在文件管理器中显示", "Show in File Manager") };
        div().w_full().flex().flex_col().gap(px(8.))
            .child(group(None, vec![
                row(tr("文档库文件夹", "Documents folder"), Some(location.into()),
                    button("reveal-library", reveal).on_click(|_, _, cx| cx.reveal_path(&crate::store::Store::root_dir()))).into_any_element(),
                row(tr("回收站", "Trash"), Some(if english() { format!("Deleted documents are kept for {} days.", crate::store::TRASH_DAYS) } else { format!("删除的文档保留 {} 天后自动清除。", crate::store::TRASH_DAYS) }.into()),
                    button("open-trash", tr("打开", "Open")).on_click(cx.listener(|w, _, _, cx| w.show_view(View::Trash, cx)))).into_any_element(),
            ]))
            .into_any_element()
    }

    fn about_page(&self, cx: &mut Context<Self>) -> AnyElement {
        div().w_full().flex().flex_col().gap(px(28.)).child(div().w_full().pt(px(24.)).flex().flex_col().items_center().gap(px(10.))
            .child(img(about_icon()).size(px(88.)))
            .child(div().text_size(px(20.)).line_height(px(26.)).font_weight(FontWeight::SEMIBOLD).child("Odyssey"))
            .child(div().text_size(px(13.)).line_height(px(18.)).text_color(Theme::text_caption()).child(format!("{} {}", tr("版本", "Version"), env!("CARGO_PKG_VERSION"))))
            .child(div().text_size(px(13.)).line_height(px(20.)).text_color(Theme::text_caption()).child(tr("本地优先的笔记应用：无需账号，文档就是你电脑上的文件。", "Local-first notes: no account, your documents are files on your Mac."))))
            .child(group(None, vec![update_row(cx).into_any_element()]))
            .into_any_element()
    }
}

/// The sidebar bar's 更新 pill, only while a new version is on offer:
/// 更新 downloads it (the pill then fills with its progress), 重启更新
/// installs it. Labels stay short: the bar keeps 72pt for it at the
/// narrowest sidebar, and a longer one ellipsizes rather than clips.
pub(crate) fn update_pill(cx: &mut Context<Workspace>) -> Option<AnyElement> {
    use crate::update::State;
    let state = crate::update::state(cx);
    let (label, tip) = match &state {
        State::Available(m) => (tr("更新", "Update").to_string(), crate::localized_format!("下载 Odyssey {}", "Download Odyssey {}", m.version)),
        State::Downloading(m) => (format!("{:.0}%", crate::update::progress(m) * 100.), crate::localized_format!("正在下载 {}…", "Downloading {}…", m.version)),
        State::Ready(m, _) => (tr("重启更新", "Restart").to_string(), crate::localized_format!("重启后更新到 {}，打开的文档会先保存", "Restart into {}; open documents are saved first", m.version)),
        State::Installing(m) => (tr("即将更新", "Updating").to_string(), crate::localized_format!("Odyssey 退出后安装 {}", "{} installs when Odyssey quits", m.version)),
        _ => return None,
    };
    let fill = if let State::Downloading(m) = &state { Some(crate::update::progress(m)) } else { None };
    let busy = matches!(state, State::Downloading(_) | State::Installing(_));
    Some(div().id("update-pill").no_drag().flex_shrink().min_w_0().h(px(24.)).px(px(10.)).relative().overflow_hidden().flex().items_center().justify_center().rounded_full()
        .text_size(px(12.)).line_height(px(16.)).font_weight(FontWeight::MEDIUM)
        .when(!busy, |d| d.cursor_pointer().bg(Theme::primary()).text_color(gpui::white()).hover(|s| s.opacity(0.9)))
        .when(busy, |d| d.bg(Theme::sidebar_hover()).text_color(Theme::text_caption()))
        // Downloading: a fixed width (no jitter as the digits change), filled left
        // to right behind a moving wave.
        .when_some(fill, |d, f| d.w(px(56.)).child(ui::wave_fill(f, Theme::primary().opacity(0.18))))
        .child(div().relative().min_w_0().truncate().child(label))
        .tip(&tip, None)
        .on_click(move |_, _, cx| match &state { State::Available(_) => crate::update::download(cx), State::Ready(..) => crate::update::install(cx), _ => {} })
        .into_any_element())
}

/// 软件更新: where the updater stands and the one next step.
fn update_row(cx: &mut Context<Workspace>) -> Div {
    use crate::update::State;
    let state = crate::update::state(cx);
    let (title, detail): (SharedString, Option<SharedString>) = match &state {
        State::Idle => (tr("软件更新", "Software update").into(), None),
        State::Unsupported => (tr("软件更新", "Software update").into(), desc(tr("这个版本不能自动更新，请从网站下载新版本。", "This copy cannot update itself. Download new versions from the website."))),
        State::Checking => (tr("正在检查更新…", "Checking for updates…").into(), None),
        State::Current => (tr("已是最新版本", "Odyssey is up to date").into(), None),
        State::Available(m) => (crate::localized_format!("新版本 {} 可用", "Version {} is available", m.version).into(), None),
        State::Downloading(m) => (crate::localized_format!("正在下载 {}… {:.0}%", "Downloading {}… {:.0}%", m.version, crate::update::progress(m) * 100.).into(), None),
        State::Ready(m, _) => (crate::localized_format!("{} 已准备好", "{} is ready to install", m.version).into(), desc(tr("重启后完成更新，打开的文档会先保存。", "Odyssey restarts to finish; open documents are saved first."))),
        State::Installing(m) => (crate::localized_format!("{} 将在退出后安装", "{} installs when Odyssey quits", m.version).into(), None),
        State::Failed(e) => (tr("更新失败", "Update failed").into(), Some(e.clone().into())),
    };
    let control = match &state {
        State::Checking | State::Downloading(_) | State::Installing(_) => None,
        State::Available(_) => Some(button("update-download", tr("下载更新", "Download")).on_click(|_, _, cx| crate::update::download(cx))),
        State::Ready(..) => Some(button("update-install", tr("重启以更新", "Restart to Update")).on_click(|_, _, cx| crate::update::install(cx))),
        _ => Some(button("update-check", tr("检查更新", "Check Now")).on_click(|_, _, cx| crate::update::check(true, cx))),
    };
    row(title, detail, div().children(control))
}

/// The app icon resampled once to 176px, its size at 88pt on a 2x screen.
/// GPUI samples images bilinearly without mipmaps, so the 1024px file shrunk
/// 5.8× on the GPU skips most of its pixels and the edges come out jagged.
/// On a 1x screen the 2:1 step is still an exact box filter.
fn about_icon() -> ImageSource {
    const PATH: &str = "branding/app-icon.png";
    static ICON: LazyLock<Option<Arc<RenderImage>>> = LazyLock::new(|| {
        let bytes = crate::assets::Assets.load(PATH).ok()??;
        let mut buf = image::imageops::resize(&image::load_from_memory(&bytes).ok()?.into_rgba8(), 176, 176, image::imageops::FilterType::Lanczos3);
        for px in buf.pixels_mut() {
            px.0.swap(0, 2);
        }
        Some(Arc::new(RenderImage::new(vec![image::Frame::new(buf)])))
    });
    match &*ICON { Some(icon) => icon.clone().into(), None => PATH.into() }
}

pub fn label(text: impl AsRef<str>) -> String { localize(text.as_ref(), english()).to_string() }

pub fn text(text: &str) -> &str { localize(text, english()) }

pub(crate) fn localize(text: &str, english: bool) -> &str {
    if !english { return text; }
    match text {
        "主页" => "Home",
        "我的文档" => "My documents",
        "置顶" => "Pinned",
        "收藏" => "Favorites",
        "设置" => "Settings",
        "新建" => "New",
        "新建文档" => "New document",
        "搜索" => "Search",
        "搜索文档…" => "Search documents…",
        "搜索文档..." => "Search documents...",
        "最近访问" => "Recently visited",
        "最近访问时间" => "Last visited",
        "最近修改时间" => "Last modified",
        "创建时间" => "Date created",
        "修改时间" => "Date modified",
        "文档名称" => "Document name",
        "名称" => "Name",
        "回收站" => "Trash",
        "存储管理" => "Storage",
        "更多" => "More",
        "返回" => "Back",
        "取消" => "Cancel",
        "确定" => "OK",
        "删除" => "Delete",
        "移到回收站" => "Move to trash",
        "恢复" => "Restore",
        "清空回收站" => "Empty trash",
        "永久删除" => "Delete permanently",
        "彻底删除" => "Delete permanently",
        "重命名" => "Rename",
        "创建副本" => "Duplicate",
        "复制" => "Copy",
        "粘贴" => "Paste",
        "剪切" => "Cut",
        "全选" => "Select all",
        "导入" => "Import",
        "导出" => "Export",
        "下载" => "Download",
        "移动到" => "Move to",
        "添加到收藏" => "Add to favorites",
        "取消收藏" => "Remove from favorites",
        "取消置顶" => "Unpin",
        "文档信息" => "Document information",
        "字数统计" => "Word count",
        "快捷键" => "Keyboard shortcuts",
        "最近浏览" => "Recently viewed",
        "所有文档" => "All documents",
        "按名称" => "By name",
        "上次浏览" => "Last viewed",
        "创建" => "Created",
        "修改" => "Modified",
        "访问" => "Visited",
        "收藏时间" => "Date favorited",
        "文档标题" => "Document title",
        "暂无文档" => "No documents yet",
        "没有找到相关文档" => "No documents found",
        "暂无收藏文档" => "No favorites yet",
        "收藏的文档会显示在这里" => "Your favorites will appear here",
        "左对齐" => "Align left",
        "居中对齐" => "Align center",
        "右对齐" => "Align right",
        "正文" => "Text",
        "一级标题" => "Heading 1",
        "二级标题" => "Heading 2",
        "三级标题" => "Heading 3",
        "四级标题" => "Heading 4",
        "五级标题" => "Heading 5",
        "六级标题" => "Heading 6",
        "加粗" => "Bold",
        "斜体" => "Italic",
        "下划线" => "Underline",
        "删除线" => "Strikethrough",
        "行内代码" => "Inline code",
        "链接" => "Link",
        "代码块" => "Code block",
        "无序列表" => "Bulleted list",
        "有序列表" => "Numbered list",
        "任务" => "Task",
        "待办事项" => "To-do list",
        "引用" => "Quote",
        "分割线" => "Divider",
        "高亮块" => "Callout",
        "图片" => "Image",
        "表格" => "Table",
        "分栏" => "Columns",
        "基础" => "Basic",
        "常用" => "Common",
        "更多类型" => "More blocks",
        "文字颜色" => "Text color",
        "背景颜色" => "Background color",
        "边框颜色" => "Border color",
        "添加图标" => "Add icon",
        "添加封面" => "Add cover",
        "编辑封面" => "Edit cover",
        "移除封面" => "Remove cover",
        "自动换行" => "Wrap lines",
        "取消换行" => "Unwrap lines",
        "文档库内搜索" => "Search documents",
        "网页搜索" => "Search the web",
        "展开" => "Expand",
        "收起" => "Collapse",
        "选择语言" => "Choose language",
        "视频或文件" => "Video or file",
        "清除格式" => "Clear formatting",
        "转换为" => "Turn into",
        "向上插入" => "Insert above",
        "向下插入" => "Insert below",
        "保存" => "Save",
        "关闭" => "Close",
        "缩进和对齐" => "Indent and align",
        "缩进" => "Indent",
        "颜色" => "Color",
        "在下方添加" => "Add below",
        "分隔线" => "Divider",
        "团队协作" => "Collaboration",
        "小应用" => "Widgets",
        "按钮" => "Button",
        "子页面列表" => "Child pages",
        "日期提醒" => "Date reminder",
        "倒计时" => "Countdown",
        "公式" => "Equation",
        "Craft for macOS 的文档样式：SF Pro 15 / 22，虚点分隔线，蓝色强调。" => "Craft-style typography: SF Pro 15 / 22, dotted dividers and blue accents.",
        "飞书文档的正文样式：16 / 26，标题 Medium，蓝色链接。" => "Feishu-style typography: 16 / 26 body text, medium headings and blue links.",
        "Notion 的默认浅色页面：暖灰正文、紧凑的块、灰底红字的行内代码、深色引用竖线。" => "Warm gray text, compact blocks, red inline code and dark quote borders.",
        "Obsidian 默认主题的浅色模式：紫色强调、浅灰列表符号、更大的标题与段落间距。" => "Purple accents, subtle list markers and generous heading and paragraph spacing.",
        "阅读模式切回编辑" => "Switch from reading to editing",
        "复制 / 剪切 / 粘贴" => "Copy / Cut / Paste",
        "逐字扩选" => "Extend selection by character",
        "逐行扩选" => "Extend selection by line",
        "选到行首 / 行尾" => "Select to start / end of line",
        "选到文档开头 / 结尾" => "Select to start / end of document",
        "# 空格" => "# Space",
        "## 空格" => "## Space",
        "### 空格" => "### Space",
        "1. 空格" => "1. Space",
        "- 空格 或 * 空格" => "- Space or * Space",
        "任务列表" => "To-do list",
        "[] 空格" => "[] Space",
        "增加缩进" => "Increase indent",
        "减少缩进" => "Decrease indent",
        "> 空格" => "> Space",
        "代码" => "Code",
        "`代码`" => "`code`",
        "``` 空格 或 ``` 代码语言 空格" => "``` Space or ``` language Space",
        "**文字** 空格" => "**text** Space",
        "*文字* 空格" => "*text* Space",
        "~文字~ 空格" => "~text~ Space",
        "~~文字~~ 空格" => "~~text~~ Space",
        "$$文字$$" => "$$text$$",
        "--- 或 ***" => "--- or ***",
        "已切换为阅读模式" => "Switched to reading mode",
        "已切换为编辑模式" => "Switched to editing mode",
        "编辑" => "Edit",
        "可编辑文档" => "Edit this document",
        "阅读" => "Read",
        "仅查看或评论文档" => "View this document",
        "键盘快捷键" => "Keyboard shortcuts",
        "Markdown 是一种快速输入带格式文本的方式，让你书写文档时行云流水，一气呵成，例如输入#空格，即可输入一级标题。" => "Markdown lets you format text as you type. For example, type # followed by Space to create a heading.",
        "页宽设置" => "Page width",
        "查找和替换" => "Find and replace",
        "从“置顶”移除" => "Unpin",
        "添加到“置顶”" => "Pin",
        "下载为" => "Download as",
        "打印" => "Print",
        "显示引用列表" => "Show references",
        "为当前窗口选择合适页宽" => "Choose a page width for this window",
        "默认" => "Default",
        "较宽" => "Wide",
        "全宽" => "Full width",
        "几秒前" => "a few seconds ago",
        "今天" => "Today",
        "昨天" => "Yesterday",
        "前天" => "Two days ago",
        "天" => "d",
        "时" => "h",
        "分" => "min",
        "秒" => "s",
        "无效公式" => "Invalid equation",
        "目录下暂无子页面" => "No child pages yet",
        "创建时间（最新优先）" => "Created (newest first)",
        "标题（升序）" => "Title (A–Z)",
        "切换为宫格视图" => "Switch to grid view",
        "切换为列表视图" => "Switch to list view",
        "当前列表暂无内容" => "Nothing here yet",
        "标题" => "Title",
        "空白文档" => "Untitled",
        "退出" => "Exit",
        "退出批量操作" => "Exit selection mode",
        "点击展开" => "Click to expand",
        "点击折叠" => "Click to collapse",
        "批量操作" => "Select documents",
        "批量管理" => "Manage",
        "无法选中，选择的页面数已到达上限" => "Selection limit reached",
        "在新窗口中打开" => "Open in new window",
        "搜索目标位置" => "Search destinations",
        "移动成功" => "Moved successfully",
        "无搜索结果" => "No results",
        "从 我的文档 移动到" => "Move from My documents to",
        "信息与数据" => "Information and data",
        "概览" => "Overview",
        "最近修改" => "Last modified",
        "文档统计" => "Document statistics",
        "总字数" => "Words",
        "总字符数" => "Characters",
        "大小" => "Size",
        "当前系统不支持" => "Not supported on this platform",
        "打印超时，请稍后重试" => "Printing timed out. Please try again.",
        "打印失败，请重试" => "Printing failed. Please try again.",
        "需要 Microsoft Edge WebView2 运行时" => "Requires the Microsoft Edge WebView2 Runtime",
        "没有找到匹配的结果，尝试其他关键词进行搜索" => "No results. Try another search term.",
        "请先关闭正在编辑这些文档的其他窗口，再删除" => "Close other windows editing these documents before deleting them.",
        "删除成功" => "Deleted successfully",
        "已添加到“置顶”" => "Pinned",
        "已从“置顶”移除" => "Unpinned",
        "已添加到“收藏”" => "Added to favorites",
        "已从“收藏”中移除" => "Removed from favorites",
        "保存失败" => "Save failed",
        "返回主页" => "Back to Home",
        "搜索(⌘ + J)" => "Search (⌘ + J)",
        "导出失败，请重试" => "Export failed. Please try again.",
        "正在导出文档" => "Exporting document",
        "文档打印中" => "Printing document",
        "文件格式不支持" => "Unsupported file format",
        "导入失败，请检查文件格式" => "Import failed. Check the file format.",
        "导出 Markdown 设置" => "Export Markdown",
        "内容范围" => "Content",
        "仅文本（不含图片和附件）" => "Text only (no images or attachments)",
        "所有内容" => "All content",
        "即将彻底删除" => "Deleting soon",
        "回收站暂无内容" => "Trash is empty",
        "位置" => "Location",
        "剩余时间" => "Time remaining",
        "恢复成功。部分文档由于原位置不存在，已恢复到“我的文档”" => "Restored. Some documents were moved to My documents because their original location no longer exists.",
        "恢复成功" => "Restored successfully",
        "已彻底删除" => "Permanently deleted",
        "清空" => "Empty",
        "已清空回收站" => "Trash emptied",
        "确认清空回收站？" => "Empty the Trash?",
        "删除的内容将进入回收站，30 天后自动彻底删除" => "Deleted items will be moved to Trash and permanently deleted after 30 days.",
        "确认恢复？" => "Restore this document?",
        "确认后将恢复至原位置" => "The document will be restored to its original location.",
        "确认彻底删除？" => "Delete permanently?",
        "彻底删除后，该页面及其子页面将无法恢复" => "This page and its child pages cannot be recovered after permanent deletion.",
        "是否隐藏引用列表？" => "Hide references?",
        "选择隐藏后，协作者将无法查看本文的引用列表。如需重新显示，可点击文档右上角的更多按钮“...”，打开“显示引用列表”开关。" => "The reference list will be hidden. To show it again, open the document’s More menu and turn on Show references.",
        "隐藏" => "Hide",
        "是否显示引用列表？" => "Show references?",
        "选择显示后，协作者可再次查看本文的引用列表。" => "The document’s reference list will be visible again.",
        "显示" => "Show",
        "文档保存失败" => "Could not save document",
        "重试" => "Retry",
        "不再提示" => "Do not ask again",
        "放弃修改" => "Discard changes",
        "Odyssey · 快速笔记" => "Odyssey · Quick note",
        "没有匹配结果，换个关键词试试吧" => "No results. Try another search term.",
        "本文被以下文档所引用" => "Referenced by these documents",
        "隐藏引用列表" => "Hide references",
        "反向引用" => "Backlinks",
        "本文引用" => "Links from this document",
        "此文档已被删除" => "This document was deleted",
        "本文暂未被其它文档引用" => "No backlinks yet",
        "本文暂未引用其它文档" => "No linked documents yet",
        "查看更多" => "Show more",
        "当前为阅读模式，请切换为编辑模式后重试" => "Switch to editing mode to make changes.",
        "表情符号" => "Emoji",
        "动物" => "Animals",
        "食物" => "Food",
        "活动" => "Activities",
        "地点" => "Places",
        "物件" => "Objects",
        "符号" => "Symbols",
        "最近" => "Recent",
        "随机" => "Random",
        "移除" => "Remove",
        "输入“/”快速插入内容" => "Type / to insert content",
        "点击更换图标" => "Click to change icon",
        "请输入标题" => "Enter a title",
        "请输入有效的链接。" => "Enter a valid link.",
        "确认" => "Confirm",
        "文本" => "Text",
        "复制原始网页链接" => "Copy original link",
        "链接已复制到剪贴板" => "Link copied to clipboard",
        "编辑链接" => "Edit link",
        "移除链接" => "Remove link",
        "渐变" => "Gradients",
        "极光" => "Aurora",
        "山峦" => "Mountains",
        "几何" => "Geometry",
        "上传图片" => "Upload image",
        "展示文档信息" => "Show document information",
        "更换图标" => "Change icon",
        "拖拽以调整图片的位置" => "Drag to reposition the image",
        "选择图片" => "Choose image",
        "调整图片位置" => "Reposition image",
        "官方图库" => "Gallery",
        "本地上传" => "Upload",
        "推荐使用尺寸为 2,000 × 400 的图片，显示效果最佳" => "For best results, use a 2,000 × 400 image.",
        "新增分栏" => "Add column",
        "图片格式仅支持 jpeg、jpg、png、gif、webp、bmp" => "Supported image formats: JPEG, JPG, PNG, GIF, WebP and BMP",
        "识别图片宽高失败" => "Could not read image dimensions",
        "上传图片大小不能超过 20 MB" => "Images must be 20 MB or smaller",
        "文件不存在" => "File not found",
        "加载失败，" => "Could not load. ",
        "点击重试" => "Retry",
        "图片过小，请放大后重试" => "Image too small. Enlarge it and try again.",
        "保存裁剪" => "Save crop",
        "裁剪" => "Crop",
        "重置图片" => "Reset image",
        "编辑描述" => "Edit caption",
        "添加描述" => "Add caption",
        "上一张" => "Previous image",
        "下一张" => "Next image",
        "缩小" => "Zoom out",
        "放大" => "Zoom in",
        "适应页面" => "Fit to page",
        "原始尺寸" => "Actual size",
        "旋转" => "Rotate",
        "红色" => "Red",
        "橙色" => "Orange",
        "黄色" => "Yellow",
        "绿色" => "Green",
        "蓝色" => "Blue",
        "紫色" => "Purple",
        "灰色" => "Gray",
        "倒计时已结束" => "Countdown finished",
        "设置的倒计时时间必须晚于当前 GMT+8 的时间" => "Choose a time later than the current time in GMT+8.",
        "输入倒计时时长" => "Enter duration",
        "选择具体日期" => "Choose date and time",
        "倒计时结束时显示气泡提醒" => "Show a notification when the countdown ends",
        "倒计时设置" => "Countdown settings",
        "一" => "Mon",
        "二" => "Tue",
        "三" => "Wed",
        "四" => "Thu",
        "五" => "Fri",
        "六" => "Sat",
        "日" => "Sun",
        "事件发生时" => "At time of event",
        "提前 5 分钟" => "5 minutes before",
        "提前 15 分钟" => "15 minutes before",
        "提前 30 分钟" => "30 minutes before",
        "提前 1 小时" => "1 hour before",
        "提前 2 小时" => "2 hours before",
        "提前 1 天" => "1 day before",
        "提前 2 天" => "2 days before",
        "提前 1 周" => "1 week before",
        "当天 (09:00)" => "Same day (09:00)",
        "提前 1 天 (09:00)" => "1 day before (09:00)",
        "提前 2 天 (09:00)" => "2 days before (09:00)",
        "提前 1 周 (09:00)" => "1 week before (09:00)",
        "当天 (18:00)" => "Same day (18:00)",
        "明天" => "Tomorrow",
        "该提醒已被删除。" => "This reminder was deleted.",
        "设置时间" => "Set time",
        "提醒" => "Reminder",
        "收起目录" => "Hide table of contents",
        "展开目录" => "Show table of contents",
        "#### 空格" => "#### Space",
        "##### 空格" => "##### Space",
        "列" => "columns",
        "行" => "rows",
        "插入列" => "Insert column",
        "插入行" => "Insert row",
        "添加 LaTeX 公式" => "Add LaTeX equation",
        "上一处" => "Previous match",
        "下一处" => "Next match",
        "查找" => "Find",
        "替换为" => "Replace with",
        "全部替换" => "Replace all",
        "替换" => "Replace",
        "请输入代码块名称" => "Enter code block name",
        "输入图片描述" => "Enter image caption",
        "新编号为" => "Start at",
        "继续之前的编号" => "Continue numbering",
        "开始新列表" => "Start new list",
        "修改编号值" => "Change list number",
        "未能找到相关结果" => "No results found",
        "插入支持富文本的表格" => "Insert a rich text table",
        "选择栏数" => "Choose number of columns",
        "字体颜色" => "Text color",
        "恢复默认" => "Reset to default",
        "填充颜色" => "Fill color",
        "按 ESC 完成输入" => "Press Esc to finish",
        "无匹配结果" => "No matches",
        "粗体" => "Bold",
        "**文本** 空格" => "**text** Space",
        "~~文本~~ 空格" => "~~text~~ Space",
        "*文本* 空格" => "*text* Space",
        "~文本~ 空格" => "~text~ Space",
        "拆分单元格" => "Split cells",
        "合并单元格" => "Merge cells",
        "单元格背景颜色" => "Cell background",
        "删除列" => "Delete columns",
        "删除行" => "Delete rows",
        "顶部对齐" => "Align top",
        "垂直居中" => "Align middle",
        "底部对齐" => "Align bottom",
        "标题行" => "Header row",
        "标题列" => "Header column",
        "均分列宽" => "Distribute columns evenly",
        "粘贴或输入链接" => "Paste or enter a link",
        "输入文本" => "Enter text",
        "在文档内查找" => "Find in document",
        "请输入文字" => "Enter text",
        "编号" => "Number",
        "请输入公式" => "Enter an equation",
        "插入图片" => "Insert image",
        "请输入按钮" => "Enter button text",
        "请输入链接地址" => "Enter link URL",
        "按钮不能为空" => "Button text is required",
        "按钮文字不能超过 200 个字" => "Button text must be 200 characters or fewer",
        "超链接不合法，请重新输入" => "Invalid link. Please enter a valid URL.",
        "当前位置暂不支持该功能" => "This action is not available here.",
        "按钮文字" => "Button text",
        "按钮颜色" => "Button color",
        "执行操作" => "Action",
        "超链接地址" => "Link URL",
        "选择按钮类型" => "Choose button type",
        "无法播放" => "Cannot play",
        "暂停" => "Pause",
        "播放" => "Play",
        "取消静音" => "Unmute",
        "静音" => "Mute",
        "预览" => "Preview",
        "无法播放该音频" => "Cannot play this audio",
        "全屏" => "Full screen",
        "文本视图" => "Text view",
        "卡片视图" => "Card view",
        "预览视图" => "Preview view",
        "插入视频或文件" => "Insert video or file",
        "文件读取失败" => "Could not read file",
        "未命名文档" => "Untitled",
        "打开超链接" => "Open link",
        "关注文档更新" => "Follow document updates",
        "刚刚" => "just now",
        "文档不存在" => "Document not found",
        "文档已被另一个实例修改，未覆盖磁盘内容。请先将当前内容导出，再重新打开文档" => "Another instance modified this document. The file was not overwritten. Export your changes, then reopen the document.",
        "这个文档库由更新版本的 Odyssey 创建，部分内容可能无法正确显示，请升级后再编辑" => "These documents were created by a newer version of Odyssey. Update the app before editing them.",
        _ => text,
    }
}

#[macro_export]
macro_rules! localized_format {
    ($zh:literal, $en:literal $(, $arg:expr)* $(,)?) => {
        if $crate::settings::english() { format!($en $(, $arg)*) } else { format!($zh $(, $arg)*) }
    };
}

/// User titles are content, not translation keys.
pub fn display_title(title: &str) -> String {
    let title = title.trim();
    if title.is_empty() { text(crate::model::UNTITLED).to_string() } else { title.to_string() }
}

/// Core diagnostics stay independent of UI language. Only known message templates
/// are translated here; captured paths, document IDs and OS errors are preserved.
pub fn diagnostic(message: &str) -> String { diagnostic_for(message, english()) }

fn diagnostic_for(message: &str, english: bool) -> String {
    if !english { return message.to_string(); }
    const TEMPLATES: &[(&str, &str)] = &[
        ("无法读取文档索引：{}", "Could not read document index: {}"),
        ("无法读取回收站索引：{}", "Could not read Trash index: {}"),
        ("文档索引已损坏，已从备份恢复（原文件保留为 {}）", "Document index was damaged and restored from backup (original retained as {})."),
        ("回收站索引已损坏，已从备份恢复（原文件保留为 {}）", "Trash index was damaged and restored from backup (original retained as {})."),
        ("文档索引已损坏且没有可用备份，部分文档的置顶、层级等信息已重置（原文件保留为 {}）", "Document index was damaged with no usable backup. Some pins and document hierarchy were reset (original retained as {})."),
        ("回收站索引已损坏且没有可用备份，部分文档的置顶、层级等信息已重置（原文件保留为 {}）", "Trash index was damaged with no usable backup. Some metadata was reset (original retained as {})."),
        ("回收站已存在同名文档，未覆盖：{}", "A document with this ID already exists in Trash and was not overwritten: {}"),
        ("文档库已存在同名文档，未覆盖：{}", "A document with this ID already exists in Documents and was not overwritten: {}"),
        ("文档已移入回收站，文档索引清理失败：{}", "Document moved to Trash, but the document index could not be cleaned up: {}"),
        ("回收站目录读取不完整：{}", "Could not fully read the Trash directory: {}"),
        ("无法清理过期回收站文档 {}：{}", "Could not remove expired Trash document {}: {}"),
        ("回收站索引保存失败：{}", "Could not save Trash index: {}"),
        ("恢复文档 {} 失败，已恢复的文档保留在文档库", "Could not restore document {}. Documents already restored remain in Documents."),
        ("文档已恢复，回收站索引清理失败：{}", "Document restored, but the Trash index could not be cleaned up: {}"),
    ];
    for (source, target) in TEMPLATES {
        let parts: Vec<_> = source.split("{}").collect();
        let Some(mut rest) = message.strip_prefix(parts[0]) else { continue };
        let mut values = Vec::new();
        let mut matched = true;
        for (i, suffix) in parts.iter().enumerate().skip(1) {
            let split = if i == parts.len() - 1 { rest.strip_suffix(suffix).map(|v| (v, "")) } else { rest.split_once(suffix) };
            let Some((value, next)) = split else { matched = false; break };
            values.push(value);
            rest = next;
        }
        if matched && rest.is_empty() {
            let mut result = String::new();
            let mut fragments = target.split("{}");
            result.push_str(fragments.next().unwrap_or_default());
            for (value, fragment) in values.iter().zip(fragments) { result.push_str(value); result.push_str(fragment); }
            return result;
        }
    }
    localize(message, true).to_string()
}

#[cfg(test)]
mod localization_tests {
    use super::{localize, diagnostic_for, display_title};

    #[test]
    fn ui_keys_and_menu_tables_have_english_translations() {
        fn check(key: &str) {
            if key.chars().any(|c| ('\u{4e00}'..='\u{9fff}').contains(&c)) {
                assert_ne!(localize(key, true), key, "Missing English UI translation: {key}");
                assert_eq!(localize(key, false), key);
            }
        }
        fn walk(dir: &std::path::Path, pattern: &regex::Regex) {
            for entry in std::fs::read_dir(dir).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() { walk(&path, pattern); }
                else if path.extension().is_some_and(|ext| ext == "rs") {
                    let source = std::fs::read_to_string(path).unwrap();
                    for hit in pattern.captures_iter(source.split("#[cfg(test)]").next().unwrap()) { check(&hit[1]); }
                }
            }
        }
        let pattern = regex::Regex::new(r#"settings::(?:text|label)\("([^"\\]*)""#).unwrap();
        walk(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), &pattern);
        for item in crate::editor::menus::SLASH_ITEMS { check(item.label); }
        for item in crate::editor::menus::TURN_ITEMS { check(item.label); check(item.tip); if let Some(md) = item.md { check(md); } }
        for group in crate::editor::menus::GROUPS { check(group.title); }
    }

    #[test]
    fn diagnostics_preserve_paths_and_identifiers() {
        let source = "无法清理过期回收站文档 我的笔记.xml：Permission denied";
        assert_eq!(diagnostic_for(source, false), source);
        assert_eq!(diagnostic_for(source, true), "Could not remove expired Trash document 我的笔记.xml: Permission denied");
        assert_eq!(diagnostic_for("文档索引已损坏，已从备份恢复（原文件保留为 index.json.bad）", true), "Document index was damaged and restored from backup (original retained as index.json.bad).");
        assert_eq!(diagnostic_for("unknown OS error", true), "unknown OS error");
    }

    #[test]
    fn user_titles_are_never_translation_keys() {
        for title in ["主页", "收藏", "未命名文档", "My notes", "我的中文笔记"] { assert_eq!(display_title(title), title); }
    }
}

#[cfg(test)]
mod preference_tests {
    use super::Preferences;

    #[test]
    fn new_preferences_use_feishu_without_overriding_saved_themes() {
        assert_eq!(Preferences::default().theme_id(), "feishu");
        for (json, expected) in [
            (r#"{}"#, "feishu"),
            (r#"{"feishu":false}"#, "craft"),
            (r#"{"feishu":true}"#, "feishu"),
            (r#"{"theme":"notion","feishu":false}"#, "notion"),
            (r#"{"theme":"craft","feishu":true}"#, "craft"),
            (r#"{"theme":"my-theme"}"#, "my-theme"),
        ] {
            let saved: Preferences = serde_json::from_str(json).unwrap();
            assert_eq!(saved.theme_id(), expected);
            let round_trip: Preferences = serde_json::from_str(&serde_json::to_string(&saved).unwrap()).unwrap();
            assert_eq!(round_trip.theme_id(), expected);
        }
    }
}
