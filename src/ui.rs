//! Small shared UI pieces: icons, tooltips, popover shells.

use crate::theme::{self, Theme, craft};
use crate::time::civil;
use gpui::{prelude::FluentBuilder, *};
use std::cell::RefCell;
use std::rc::Rc;

pub fn icon(name: &str, size: f32, color: Hsla) -> Svg {
    svg().path(SharedString::from(format!("icons/{name}.svg"))).size(px(size)).flex_none().text_color(color)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToastKind {
    Success,
    Error,
    Info,
}

/// Toast: a white rounded card 32px below the top, centred, with a filled
/// status circle (green check / red x / accent i). It lasts 3s and fades over
/// its last .3s; None once it is over.
pub fn toast_el(kind: ToastKind, msg: &SharedString, t0: std::time::Instant, window: &mut Window) -> Option<AnyElement> {
    let e = t0.elapsed().as_secs_f32();
    if e >= 3. {
        return None;
    }
    let op = if e > 2.7 {
        window.request_animation_frame();
        (3. - e) / 0.3
    } else {
        1.
    };
    let (fill, mark, mark_size) = match kind {
        ToastKind::Success => (theme::c(craft::old::G500), "check", 10.),
        ToastKind::Error => (Theme::danger(), "x", 10.),
        ToastKind::Info => (Theme::primary(), "info-mark", 12.),
    };
    let vw = window.viewport_size().width;
    let card = div()
        .flex()
        .items_center()
        .px(px(14.))
        .py(px(10.))
        .rounded(px(12.))
        .bg(crate::theme::raised(Theme::bg_body()))
        .border_1()
        .border_color(Theme::line_border())
        .shadow(theme::shadow_panel())
        .font_family(theme::UI_FONT)
        .text_size(px(13.))
        .line_height(px(20.))
        .text_color(Theme::text())
        .child(
            div()
                .relative()
                .size(px(16.))
                .mr(px(8.))
                .flex_none()
                .flex()
                .items_center()
                .justify_center()
                .child(icon("circle-filled", 16., fill).absolute().top_0().left_0())
                .child(icon(mark, mark_size, gpui::white())),
        )
        .child(ui_text(crate::settings::label(msg)));
    Some(
        deferred(
            anchored()
                .position_mode(AnchoredPositionMode::Window)
                .position(point(px(0.), px(32.)))
                .child(div().w(vw).flex().justify_center().opacity(op).child(card)),
        )
        .with_priority(20)
        .into_any_element(),
    )
}

/// Popover shell shared by the slash menu, block menu and toolbar dropdowns:
/// the light panel (macOS 26 menu material, near-white), hairline,
/// radius 12, soft drop.
pub fn popover() -> Div {
    div()
        .bg(crate::theme::raised(Theme::popover_bg()))
        .border_1()
        .border_color(Theme::line_border())
        .rounded(px(MENU_RADIUS))
        .shadow(theme::shadow_menu())
        .font_family(theme::UI_FONT)
        .text_color(Theme::text())
}

/// Second-level panels (color panel, link editor): white, larger drop.
/// The header button: a white pill on the note (hairline, 1px drop, radius 12).
pub fn pill() -> Div {
    div()
        .flex_none()
        .rounded(px(12.))
        .bg(crate::theme::raised(Theme::bg_body()))
        .border_1()
        .border_color(Theme::button_border())
        .shadow(theme::shadow_button())
}

/// Column headers of the page lists: tertiary text (measured #88898B on
/// the name / last-viewed / created columns).
pub fn list_header_text() -> Hsla {
    theme::c(0x88898b)
}

/// Hairline under a list header and between rows (black 4.5%).
pub fn list_line() -> Hsla {
    theme::ca(craft::N1000, 0.045)
}

/// The quiet list hover: a rounded fill that also hides the row's
/// hairline above it.
pub fn row_hover(s: StyleRefinement) -> StyleRefinement {
    s.bg(Theme::fill_hover()).border_color(transparent_black()).rounded(px(ROW_RADIUS))
}


/// The window dimmed under a modal dialog: a light black veil.
pub fn scrim() -> Hsla {
    theme::ca(craft::N1000, 0.25)
}

/// The modal dialog: white, radius 16, deep soft drop.
pub fn dialog(id: &'static str) -> Stateful<Div> {
    div().id(id).occlude().rounded(px(16.)).bg(crate::theme::raised(Theme::bg_body())).shadow(theme::shadow_dialog()).font_family(theme::UI_FONT).text_color(Theme::text())
}

/// The primary button: the accent, a white 13pt semibold label, 30 tall, radius 9.
pub fn primary_button(id: &'static str, label: &'static str) -> Stateful<Div> {
    div()
        .id(id)
        .flex_none()
        .h(px(30.))
        .px(px(16.))
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(9.))
        .cursor_pointer()
        .bg(Theme::primary())
        .shadow(theme::shadow_button())
        .hover(|s| s.bg(theme::c(craft::B600)))
        .active(|s| s.bg(theme::c(craft::B700)))
        .text_size(px(13.))
        .line_height(px(20.))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(gpui::white())
        .child(crate::settings::label(label))
}

/// The secondary button: white with a hairline and a 1px drop, 30 tall, radius 9.
pub fn secondary_button(id: &'static str, label: &'static str) -> Stateful<Div> {
    div()
        .id(id)
        .flex_none()
        .h(px(30.))
        .px(px(16.))
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(9.))
        .cursor_pointer()
        .bg(crate::theme::raised(Theme::bg_body()))
        .border_1()
        .border_color(Theme::button_border())
        .shadow(theme::shadow_button())
        .hover(|s| s.bg(theme::c(craft::N100)))
        .active(|s| s.bg(theme::c(craft::N200)))
        .text_size(px(13.))
        .line_height(px(20.))
        .text_color(Theme::text())
        .child(crate::settings::label(label))
}

/// The panel close button: a grey disc with a white cross (measured
/// #B6B7B8 on #F7F7F7, black at about 26%).
pub fn close_button(id: &'static str) -> Stateful<Div> {
    div()
        .id(id)
        .flex_none()
        .size(px(18.))
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .cursor_pointer()
        .bg(theme::ca(craft::N1000, 0.26))
        .hover(|s| s.bg(theme::ca(craft::N1000, 0.36)))
        .child(icon("x", 16., gpui::white()))
}

/// Menus and popovers round at 12 (the native menu panel); their
/// rows at 6, 5pt in from the edge.
pub const MENU_RADIUS: f32 = 12.;
pub const ROW_RADIUS: f32 = 6.;

/// A menu row like a native menu's: 26 tall, 13pt label, 16pt glyph;
/// hovering paints the accent. `group` names the row so its glyph
/// ([`menu_glyph`]) and label ([`menu_label`]) can turn white with it.
pub const MENU_ROW_H: f32 = 32.;
pub const MENU_PAD_Y: f32 = 6.;

pub fn menu_row(id: impl Into<ElementId>, group: SharedString) -> Stateful<Div> {
    div()
        .id(id)
        .group(group)
        .flex_none()
        .h(px(MENU_ROW_H))
        .mx(px(5.))
        .px(px(8.))
        .flex()
        .items_center()
        .gap(px(8.))
        .rounded(px(ROW_RADIUS))
        .cursor_pointer()
        .text_size(px(13.))
        .line_height(px(20.))
        .text_color(Theme::text())
        .hover(|s| s.bg(Theme::menu_highlight()))
}

/// A [`crate::ui::menu_row`] label that turns `hover` (white) while its row
/// is highlighted. GPUI fixes a text's colour when it lays the text out,
/// before hover is known, so a hover text colour on the row never reaches
/// the label; instead a `hover` copy lies over it and the two swap through
/// `group_hover` opacity, which is resolved at paint.
pub fn menu_label(label: impl Into<SharedString>, group: SharedString, hover: Hsla) -> Div {
    let label: SharedString = crate::settings::label(label.into()).into();
    div()
        .relative()
        .whitespace_nowrap()
        .child(div().group_hover(group.clone(), |s| s.opacity(0.)).child(label.clone()))
        .child(div().absolute().top_0().left_0().text_color(hover).opacity(0.).group_hover(group, |s| s.opacity(1.)).child(crate::settings::label(label)))
}

/// Glyph of a [`menu_row`]: secondary, white while its row is hovered.
pub fn menu_glyph(name: &str, size: f32, group: SharedString) -> Svg {
    icon(name, size, Theme::text_code()).group_hover(group, |s| s.text_color(gpui::white()))
}

/// Tooltip bubble: the dark rounded tip (12pt white on near-black, radius 7, arrow).
pub struct Tooltip {
    /// One or more lines.
    label: SharedString,
    /// Window bounds of the trigger, when it was registered through [`TipExt::tip`].
    anchor: Option<Bounds<Pixels>>,
    /// Where GPUI anchors the tooltip root (mouse + 1 at show time).
    origin: Point<Pixels>,
    side: TipSide,
}

/// Preferred side: chrome shows tooltips below, document controls above,
/// menu items to the right. Flipped when there is no room.
#[derive(Clone, Copy, PartialEq)]
pub enum TipSide {
    Below,
    Above,
    Right,
}

const TIP_GAP: f32 = 8.;
const TIP_ARROW: f32 = 5.;
const TIP_PAD: f32 = 12.;
const TIP_LINE: f32 = 18.;
const TIP_BG: u32 = 0x2a2c2f;
/// Tooltips stay dark in both looks (a step lighter on the dark page).
fn tip_bg() -> Hsla { if theme::dark() { theme::rgbaf(0x3a, 0x3b, 0x3d, 1.) } else { theme::rgbaf((TIP_BG >> 16) as u8, (TIP_BG >> 8) as u8, TIP_BG as u8, 1.) } }

impl Render for Tooltip {
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let lines: Vec<SharedString> = self.label.split('\n').map(|l| SharedString::from(l.to_string())).collect();
        let bubble = div()
            .px(px(10.))
            .py(px(6.))
            .rounded(px(7.))
            .bg(tip_bg())
            .text_color(gpui::white())
            .text_size(px(12.))
            .line_height(px(TIP_LINE))
            .font_family(theme::UI_FONT)
            .whitespace_nowrap()
            .flex()
            .flex_col()
            .children(lines.iter().map(|l| ui_text(l.clone())));
        let Some(anchor) = self.anchor else {
            return div().pl(px(8.)).pt(px(10.)).child(bubble).into_any_element();
        };
        // 10px from the trigger with an arrow pointing at it, on the preferred
        // side unless it doesn't fit, and kept inside the window.
        let mut style = window.text_style();
        style.font_family = theme::UI_FONT.into();
        let text_w = lines
            .iter()
            .map(|l| {
                let runs = ui_text(l.clone()).runs(&style);
                window.text_system().shape_line(l.clone(), px(12.), &runs, None).width
            })
            .fold(px(0.), |a, b| a.max(b));
        let w = text_w + px(20.);
        let h = px(TIP_PAD + TIP_LINE * lines.len() as f32);
        let vp = window.viewport_size();
        let gap = px(TIP_GAP);
        let s = px(TIP_ARROW);
        // (bubble origin, arrow base centre, arrow direction)
        let (x, y, arrow): (Pixels, Pixels, Box<dyn Fn(&mut PathBuilder)>) = if self.side == TipSide::Right
            && (anchor.right() + gap + w <= vp.width || anchor.left() - gap - w < px(0.))
        {
            let x = anchor.right() + gap;
            let cy = anchor.center().y;
            let y = (cy - h / 2.).min(vp.height - h).max(px(0.));
            let tip_y = cy.max(y + px(10.)).min(y + h - px(10.));
            (x, y, Box::new(move |p: &mut PathBuilder| {
                p.move_to(point(x, tip_y - s));
                p.line_to(point(x - s * 0.88, tip_y - px(0.8)));
                p.line_to(point(x - s * 0.88, tip_y + px(0.8)));
                p.line_to(point(x, tip_y + s));
            }))
        } else if self.side == TipSide::Right {
            let x = anchor.left() - gap - w;
            let cy = anchor.center().y;
            let y = (cy - h / 2.).min(vp.height - h).max(px(0.));
            let tip_y = cy.max(y + px(10.)).min(y + h - px(10.));
            let bx = x + w;
            (x, y, Box::new(move |p: &mut PathBuilder| {
                p.move_to(point(bx, tip_y - s));
                p.line_to(point(bx + s * 0.88, tip_y - px(0.8)));
                p.line_to(point(bx + s * 0.88, tip_y + px(0.8)));
                p.line_to(point(bx, tip_y + s));
            }))
        } else {
            let cx = anchor.center().x;
            let x = (cx - w / 2.).min(vp.width - w).max(px(0.));
            let fits_below = anchor.bottom() + gap + h <= vp.height;
            let fits_above = anchor.top() - gap - h >= px(0.);
            let below = if self.side == TipSide::Above { !fits_above } else { fits_below };
            let y = if below { anchor.bottom() + gap } else { anchor.top() - gap - h };
            let tip_x = cx.max(x + px(10.)).min(x + w - px(10.));
            let (base, apex) = if below { (y, y - s) } else { (y + h, y + h + s) };
            (x, y, Box::new(move |p: &mut PathBuilder| {
                p.move_to(point(tip_x - s, base));
                p.line_to(point(tip_x - px(0.8), apex + (base - apex) * 0.12));
                p.line_to(point(tip_x + px(0.8), apex + (base - apex) * 0.12));
                p.line_to(point(tip_x + s, base));
            }))
        };
        let o = self.origin;
        let arrow = canvas(
            |_, _, _| {},
            move |_, _, window, _| {
                let mut p = PathBuilder::fill();
                arrow(&mut p);
                p.close();
                if let Ok(path) = p.build() {
                    window.paint_path(path, tip_bg());
                }
            },
        )
        .absolute()
        .size(px(1.));
        div()
            .size(px(1.))
            .child(div().absolute().left(x - o.x).top(y - o.y).child(bubble))
            .child(arrow)
            .into_any_element()
    }
}

/// Tooltip anchored on the element rather than the mouse.
pub trait TipExt: Sized {
    /// Window chrome: below, shortcut in full-width parens ("展开面板（⌘ + ]）").
    fn tip(self, text: &str, keys: Option<&str>) -> Self {
        let text = crate::settings::label(text);
        let label = match keys {
            Some(k) => format!("{text}（{k}）"),
            None => text.to_string(),
        };
        self.tip_label(label, TipSide::Below)
    }
    /// Document controls: above, in the editor's shortcut format
    /// ("粗体 (⌘ + B)"), optionally with a "Markdown: …" second line.
    fn tip_above(self, text: &str, keys: Option<&str>) -> Self {
        self.tip_label(doc_tip(text, keys, None), TipSide::Above)
    }
    fn tip_md(self, text: &str, keys: Option<&str>, md: Option<&str>) -> Self {
        self.tip_label(doc_tip(text, keys, md), TipSide::Above)
    }
    /// Menu items: to the right.
    fn tip_right(self, text: &str, keys: Option<&str>) -> Self {
        self.tip_label(doc_tip(text, keys, None), TipSide::Right)
    }
    fn tip_label(self, label: String, side: TipSide) -> Self;
}

/// Window chrome above the page's top line works like a native title bar: a
/// press drags the window, a double press zooms it (per the system setting).
/// Controls on it opt out with `no_drag`, so double-clicking them doesn't zoom.
pub trait TitleBarExt: InteractiveElement + Sized {
    fn title_bar(self) -> Self {
        // Keep the workspace's focus handler from preventing the native caption action.
        if cfg!(target_os = "windows") { return self.occlude().window_control_area(WindowControlArea::Drag); }
        self.on_mouse_down(MouseButton::Left, |ev, window, _| match ev.click_count {
            1 => window.start_window_move(),
            2 => window.titlebar_double_click(),
            _ => {}
        })
    }
    fn no_drag(self) -> Self {
        let element = if cfg!(target_os = "windows") { self.occlude() } else { self };
        element.on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
    }
}

impl<E: InteractiveElement> TitleBarExt for E {}

pub(crate) const WINDOW_CONTROLS_W: f32 = if cfg!(target_os = "windows") { 163. } else { 0. };
pub(crate) fn header_right_inset() -> f32 {
    if cfg!(target_os = "windows") { WINDOW_CONTROLS_W } else { (crate::app::HEADER_H - crate::app::HEADER_ACTIONS_H) / 2. }
}

pub(crate) fn window_controls(window: &Window) -> Option<Div> {
    if !cfg!(target_os = "windows") { return None; }
    let button = |id: &'static str, area: WindowControlArea, label: &'static str| {
        div().id(id).size(px(crate::app::HEADER_ACTIONS_H)).mx(px(7.)).rounded(px(8.)).flex_none().flex().items_center().justify_center().cursor_pointer()
            .occlude().window_control_area(area).hover(|s| s.bg(Theme::fill_hover())).active(|s| s.bg(Theme::fill_pressed())).tip(label, None)
    };
    let maximized = window.is_maximized();
    // Native hit-test areas preserve Snap Layouts and the normal WM_CLOSE save guard.
    Some(div().absolute().top_0().right_0().h(px(crate::app::HEADER_H)).flex().items_center().title_bar()
        .child(div().mx(px(12.)).w(px(1.)).h(px(18.)).bg(Theme::line_divider()))
        .child(button("window-minimize", WindowControlArea::Min, crate::settings::tr("最小化", "Minimize"))
            .child(icon("minus", 18., Theme::icon())))
        .child(button("window-maximize", WindowControlArea::Max, if maximized { crate::settings::tr("还原", "Restore") } else { crate::settings::tr("最大化", "Maximize") }).child(icon(if maximized { "copy" } else { "expand" }, 18., Theme::icon())))
        .child(button("window-close", WindowControlArea::Close, crate::settings::tr("关闭", "Close"))
            .child(icon("x", 18., Theme::icon()))))
}

/// "名称 (键 + 键)" plus an optional "Markdown: …" line.
pub fn doc_tip(text: &str, keys: Option<&str>, md: Option<&str>) -> String {
    let text = crate::settings::text(text);
    let mut s = match keys {
        Some(k) => format!("{text} ({k})"),
        None => text.to_string(),
    };
    if let Some(m) = md {
        s.push_str("\nMarkdown: ");
        s.push_str(crate::settings::text(m));
    }
    s
}

impl TipExt for Stateful<Div> {
    fn tip_label(self, label: String, side: TipSide) -> Self {
        let bounds = Rc::new(std::cell::Cell::new(None::<Bounds<Pixels>>));
        let record = bounds.clone();
        let label = SharedString::from(label);
        self.child(canvas(move |b, _, _| record.set(Some(b)), |_, _, _, _| {}).absolute().inset_0()).tooltip(move |window, cx| {
            let origin = window.mouse_position() + point(px(1.), px(1.));
            let (label, anchor) = (label.clone(), bounds.get());
            cx.new(|_| Tooltip { label, anchor, origin, side }).into()
        })
    }
}

/// Full date ("2025年12月2日"), as in 文档信息.
pub fn format_date(secs: u64) -> String {
    let (y, mo, d, _, _) = civil(secs);
    if crate::settings::english() { format!("{y}-{mo:02}-{d:02}") } else { format!("{y}年{mo}月{d}日") }
}

/// Relative time for 文档信息 ("几秒前", "3 分钟前", "2 小时前", then the date).
pub fn format_relative(secs: u64) -> String {
    let ago = crate::time::now().saturating_sub(secs);
    match ago {
        0..60 => crate::settings::text("几秒前").into(),
        60..3600 => crate::localized_format!("{} 分钟前", "{} min ago", ago / 60),
        3600..86_400 => crate::localized_format!("{} 小时前", "{} hours ago", ago / 3600),
        _ => format_date(secs),
    }
}

/// Search result times (`space.date_format.*`): "刚刚", "N 分钟前",
/// "今天 / 昨天 / 前天 HH:mm", "M月D日 HH:mm", across years "YYYY年M月D日".
pub fn format_visit(secs: u64) -> String {
    let now = crate::time::now();
    let ago = now.saturating_sub(secs);
    if ago < 60 {
        return crate::settings::tr("刚刚", "just now").into();
    }
    if ago < 3600 {
        return if crate::settings::english() { format!("{} min ago", ago / 60) } else { format!("{} 分钟前", ago / 60) };
    }
    let (y, mo, d, h, mi) = civil(secs);
    let day = |back: u64| {
        let (y, mo, d, _, _) = civil(now.saturating_sub(back * 86_400));
        (y, mo, d)
    };
    let names = if crate::settings::english() { ["Today", "Yesterday", "Two days ago"] } else { ["今天", "昨天", "前天"] };
    if let Some(name) = (0..3).find(|&b| day(b) == (y, mo, d)).map(|b| names[b as usize]) {
        format!("{name} {h:02}:{mi:02}")
    } else if y == civil(now).0 {
        if crate::settings::english() { format!("{mo:02}-{d:02} {h:02}:{mi:02}") } else { format!("{mo}月{d}日 {h:02}:{mi:02}") }
    } else {
        if crate::settings::english() { format!("{y}-{mo:02}-{d:02}") } else { format!("{y}年{mo}月{d}日") }
    }
}

/// Full-width CJK punctuation (、。〈〉《》「」『』【】〔〕（）：，… in U+3001–301B / U+FF01–FF65).
pub fn is_cjk_punct(ch: char) -> bool {
    matches!(ch, '\u{3001}'..='\u{3003}' | '\u{3008}'..='\u{3011}' | '\u{3014}'..='\u{301B}'
        | '\u{FF01}'..='\u{FF0F}' | '\u{FF1A}'..='\u{FF20}' | '\u{FF3B}'..='\u{FF40}' | '\u{FF5B}'..='\u{FF65}')
}

/// Font for full-width punctuation. When CoreText falls back from the
/// system UI font it squeezes these (an opening paren after a CJK
/// character loses ~5px). Browsers draw them full width with PingFang SC.
pub const CJK_PUNCT_FONT: &str = "PingFang SC";

/// Single-line UI text that keeps full-width punctuation at full width,
/// with an optional colored range (search hits). Inherits the parent text
/// style; honours `text_ellipsis()` against the final width (StyledText
/// caches its first, unconstrained measurement when nowrap).
pub struct UiText {
    text: SharedString,
    hit: Option<(std::ops::Range<usize>, Hsla)>,
}

pub fn ui_text(text: impl Into<SharedString>) -> UiText {
    UiText { text: text.into(), hit: None }
}

pub struct UiTextLayout {
    runs: Vec<TextRun>,
    font_size: Pixels,
    line_height: Pixels,
    ellipsis: Option<SharedString>,
    full: Rc<RefCell<Option<ShapedLine>>>,
}

impl UiText {
    pub fn hit(mut self, range: Option<std::ops::Range<usize>>, color: Hsla) -> Self {
        self.hit = range.map(|r| (r, color));
        self
    }

    fn runs(&self, style: &TextStyle) -> Vec<TextRun> {
        match &self.hit {
            Some((r, c)) => ui_runs(&self.text, style, std::slice::from_ref(r), *c),
            None => ui_runs(&self.text, style, &[], style.color),
        }
    }
}

/// Text runs that keep full-width punctuation at full width (see
/// [`CJK_PUNCT_FONT`]), with `hits` drawn in `hit_color`.
pub fn ui_runs(text: &str, style: &TextStyle, hits: &[std::ops::Range<usize>], hit_color: Hsla) -> Vec<TextRun> {
    let mut cuts = vec![0, text.len()];
    for (i, ch) in text.char_indices() {
        if is_cjk_punct(ch) {
            cuts.extend([i, i + ch.len_utf8()]);
        }
    }
    for r in hits {
        cuts.extend([r.start.min(text.len()), r.end.min(text.len())]);
    }
    cuts.sort();
    cuts.dedup();
    cuts.windows(2)
        .map(|w| {
            let mut run = style.to_run(w[1] - w[0]);
            if text[w[0]..w[1]].chars().all(is_cjk_punct) {
                run.font.family = CJK_PUNCT_FONT.into();
            }
            if hits.iter().any(|r| r.start <= w[0] && w[1] <= r.end) {
                run.color = hit_color;
            }
            run
        })
        .collect()
}

/// Longest prefix of `line` that fits `width` together with `suffix`.
fn truncated(line: &ShapedLine, runs: &[TextRun], suffix: &str, font_size: Pixels, width: Pixels, window: &mut Window) -> ShapedLine {
    let text = line.text.clone();
    let mut suffix_runs: Vec<TextRun> = runs.last().cloned().into_iter().collect();
    if let Some(r) = suffix_runs.first_mut() {
        r.len = suffix.len();
        r.font.family = theme::UI_FONT.into();
    }
    let suffix_w = window.text_system().shape_line(suffix.to_string().into(), font_size, &suffix_runs, None).width;
    let mut cut = 0;
    for (i, ch) in text.char_indices() {
        let end = i + ch.len_utf8();
        if line.x_for_index(end) + suffix_w > width {
            break;
        }
        cut = end;
    }
    let mut out = vec![];
    let mut at = 0;
    for r in runs {
        if at >= cut {
            break;
        }
        let mut r = r.clone();
        r.len = r.len.min(cut - at);
        at += r.len;
        out.push(r);
    }
    out.extend(suffix_runs);
    let s: SharedString = format!("{}{}", &text[..cut], suffix).into();
    window.text_system().shape_line(s, font_size, &out, None)
}

impl IntoElement for UiText {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}

impl Element for UiText {
    type RequestLayoutState = UiTextLayout;
    type PrepaintState = Option<ShapedLine>;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        _: &mut App,
    ) -> (LayoutId, UiTextLayout) {
        let style = window.text_style();
        let font_size = style.font_size.to_pixels(window.rem_size());
        let line_height = style.line_height.to_pixels(font_size.into(), window.rem_size());
        let runs = if self.text.is_empty() { vec![] } else { self.runs(&style) };
        let ellipsis = style.text_overflow.as_ref().map(|TextOverflow::Truncate(s)| s.clone());
        let full = Rc::new(RefCell::new(None));
        let (text, r, f, clip) = (self.text.clone(), runs.clone(), full.clone(), ellipsis.is_some());
        let id = window.request_measured_layout(Style::default(), move |known, avail, window, _| {
            let mut slot = f.borrow_mut();
            let line = slot.get_or_insert_with(|| window.text_system().shape_line(text.clone(), font_size, &r, None));
            let mut w = line.width.ceil();
            if clip {
                let limit = known.width.or(match avail.width {
                    AvailableSpace::Definite(x) => Some(x),
                    _ => None,
                });
                if let Some(l) = limit {
                    w = w.min(l);
                }
            }
            size(w, line_height)
        });
        (id, UiTextLayout { runs, font_size, line_height, ellipsis, full })
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut UiTextLayout,
        window: &mut Window,
        _: &mut App,
    ) -> Option<ShapedLine> {
        let full = layout.full.borrow().clone()?;
        match &layout.ellipsis {
            Some(suffix) if full.width > bounds.size.width + px(0.5) => {
                Some(truncated(&full, &layout.runs, suffix, layout.font_size, bounds.size.width, window))
            }
            _ => Some(full),
        }
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut UiTextLayout,
        line: &mut Option<ShapedLine>,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(line) = line {
            let _ = line.paint(bounds.origin, layout.line_height, window, cx);
        }
    }
}

/// [`UiText`] that wraps: a paragraph keeping full-width punctuation at full
/// width, in the inherited text style.
pub fn ui_para(text: impl Into<SharedString>) -> UiPara {
    UiPara { text: text.into(), inner: None }
}

pub struct UiPara {
    text: SharedString,
    inner: Option<StyledText>,
}

impl IntoElement for UiPara {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}

impl Element for UiPara {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        let style = window.text_style();
        let runs = ui_runs(&self.text, &style, &[], style.color);
        self.inner.insert(StyledText::new(self.text.clone()).with_runs(runs)).request_layout(id, inspector_id, window, cx)
    }

    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(t) = &mut self.inner {
            t.prepaint(id, inspector_id, bounds, layout, window, cx)
        }
    }

    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut (),
        state: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(t) = &mut self.inner {
            t.paint(id, inspector_id, bounds, layout, state, window, cx)
        }
    }
}

/// Paint the outside of a continuous corner, including over cover images.
pub fn continuous_corner_mask(radius: f32, color: Hsla) -> impl IntoElement {
    canvas(|bounds, _, _| bounds, move |_, b, window, _| {
        let x = f32::from(b.left()); let y = f32::from(b.top());
        let w = f32::from(b.size.width); let h = f32::from(b.size.height);
        let r = radius.min(w / 2.).min(h / 2.);
        for (sx, sy, ux, uy, vx, vy) in [
            (x + w - r, y, 1., 0., 0., 1.),
            (x + w, y + h - r, 0., 1., -1., 0.),
            (x + r, y + h, -1., 0., 0., -1.),
            (x, y + r, 0., -1., 1., 0.),
        ] {
            let q = |u: f32, v: f32| point(px(sx + r * (u * ux + v * vx)), px(sy + r * (u * uy + v * vy)));
            let mut path = PathBuilder::fill();
            path.move_to(q(0., 0.));
            path.line_to(q(1., 0.));
            path.line_to(q(1., 1.));
            path.cubic_bezier_to(q(std::f32::consts::FRAC_1_SQRT_2, 0.2928932), q(1., 0.7928932), q(1., 0.5857864));
            path.cubic_bezier_to(q(0., 0.), q(0.4142136, 0.), q(0.2071068, 0.));
            path.close();
            if let Ok(path) = path.build() { window.paint_path(path, color); }
        }
    }).absolute().inset_0()
}

/// A capsule filled to `fraction` from the left, its edge a moving wave (the
/// update pill's download progress). Redraws every frame while shown.
pub fn wave_fill(fraction: f32, color: Hsla) -> impl IntoElement {
    static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    let t = START.get_or_init(std::time::Instant::now).elapsed().as_secs_f32();
    canvas(|bounds, _, _| bounds, move |_, b, window, _| {
        window.request_animation_frame();
        let f = fraction.clamp(0., 1.);
        if f <= 0. { return }
        let (x, y) = (f32::from(b.left()), f32::from(b.top()));
        let (w, h) = (f32::from(b.size.width), f32::from(b.size.height));
        let r = h / 2.;
        // The capsule's left and right walls at height v (0 = top).
        let wall = |v: f32| (r * r - (v - r).powi(2)).max(0.).sqrt();
        // A 2pt swell, one crest per pill height, drifting downwards; flat when full.
        let amp = if f >= 1. { 0. } else { 2. };
        let edge = |v: f32| (x + f * w + amp * (std::f32::consts::TAU * v / h - t * 5.).sin()).clamp(x + r - wall(v), x + w - r + wall(v));
        const N: usize = 24;
        let mut path = PathBuilder::fill();
        path.move_to(point(px(edge(0.)), px(y)));
        for i in 1..=N { let v = h * i as f32 / N as f32; path.line_to(point(px(edge(v)), px(y + v))); }
        for i in (0..=N).rev() { let v = h * i as f32 / N as f32; path.line_to(point(px(x + r - wall(v)), px(y + v))); }
        path.close();
        if let Ok(path) = path.build() { window.paint_path(path, color); }
    }).absolute().inset_0()
}

pub fn continuous_page(radius: f32) -> Div { continuous_surface(radius, Theme::bg_body()) }

pub fn continuous_surface(radius: f32, color: Hsla) -> Div {
    div().relative().child(canvas(|bounds, _, _| bounds, move |_, bounds, window, _| {
        let inset = 0.;
                let x = f32::from(bounds.left()) + inset;
                let y = f32::from(bounds.top()) + inset;
                let w = f32::from(bounds.size.width) - inset * 2.;
                let h = f32::from(bounds.size.height) - inset * 2.;
                let r = (radius - inset).max(0.).min(w / 2.).min(h / 2.);
                let pt = |a, b| point(px(a), px(b));
                let mut path = PathBuilder::fill();
                // Two cubics per corner: zero curvature at the straight joins,
                // a shared tangent/curvature at the midpoint, and a circular-size
                // diagonal inset rather than the previous overly square curve.
                let corner = |path: &mut PathBuilder, sx: f32, sy: f32, ux: f32, uy: f32, vx: f32, vy: f32| {
                    let q = |u: f32, v: f32| pt(sx + r * (u * ux + v * vx), sy + r * (u * uy + v * vy));
                    path.cubic_bezier_to(q(std::f32::consts::FRAC_1_SQRT_2, 0.2928932), q(0.2071068, 0.), q(0.4142136, 0.));
                    path.cubic_bezier_to(q(1., 1.), q(1., 0.5857864), q(1., 0.7928932));
                };
                path.move_to(pt(x + r, y));
                path.line_to(pt(x + w - r, y));
                corner(&mut path, x + w - r, y, 1., 0., 0., 1.);
                path.line_to(pt(x + w, y + h - r));
                corner(&mut path, x + w, y + h - r, 0., 1., -1., 0.);
                path.line_to(pt(x + r, y + h));
                corner(&mut path, x + r, y + h, -1., 0., 0., -1.);
                path.line_to(pt(x, y + r));
                corner(&mut path, x, y + r, 0., -1., 1., 0.);
                path.close();
                if let Ok(path) = path.build() { window.paint_path(path, color); }
    }).absolute().inset_0())
}

pub fn header_icon(id: &'static str, name: &'static str) -> Stateful<Div> {
    div().id(id).no_drag().size(px(crate::app::HEADER_ACTIONS_H)).rounded(px(8.)).flex_none().flex().items_center().justify_center().cursor_pointer()
        .hover(|s| s.bg(Theme::fill_hover())).active(|s| s.bg(Theme::fill_pressed())).child(icon(name, 18., Theme::icon()))
}

pub(crate) fn checkbox(on: bool) -> Div {
    div()
        .size(px(16.))
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(4.))
        .when(on, |d| d.bg(Theme::primary()).child(icon("todo-check", 12., Theme::bg_body())))
        .when(!on, |d| d.border_1().border_color(Theme::todo_box()).bg(crate::theme::raised(Theme::bg_body())))
}
