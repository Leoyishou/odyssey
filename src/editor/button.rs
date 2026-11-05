//! 按钮 (`<button>`): an inline object painted by `RichTextElement`, with
//! a hover popover (编辑) and the 按钮设置 panel.

use crate::editor_theme::{c, craft};
use gpui::*;

/// A button colour's text and fill at rest, hovered, pressed and disabled
/// (`light-x` / `deep-x` names, painted from the palette).
pub struct Palette {
    pub text: Hsla,
    pub bg: Hsla,
    pub hover_text: Hsla,
    pub hover_bg: Hsla,
    pub active_text: Hsla,
    pub active_bg: Hsla,
    pub disabled_bg: Hsla,
}

/// The color picker's two rows, in order.
pub const COLORS: [[&str; 8]; 2] = [
    ["light-red", "light-orange", "light-green", "light-wathet", "light-blue", "light-carmine", "light-purple", "light-neutral"],
    ["deep-red", "deep-orange", "deep-green", "deep-wathet", "deep-blue", "deep-carmine", "deep-purple", "deep-neutral"],
];

/// Existing documents write orange as `origin`; read both.
pub fn normalize_color(name: &str) -> &str {
    match name {
        "light-origin" => "light-orange",
        "deep-origin" => "deep-orange",
        n if COLORS.iter().flatten().any(|c| *c == n) => n,
        _ => crate::model::BUTTON_DEFAULT_COLOR,
    }
}

pub fn palette(name: &str) -> Palette {
    // (100, 200, 300, 400, 500, 600, 700) of each hue: the content palette
    // (Tailwind v4 ramps, the `contentPalette` tokens; wathet is cyan,
    // carmine pink, green emerald) and, for blue, the ramp around the accent.
    let hue = |h: &str| -> [u32; 7] {
        match h {
            "red" => [craft::R100, craft::R200, craft::R300, craft::R350, craft::R400, craft::R500, craft::R600],
            "orange" => [craft::O100, craft::O200, craft::O300, craft::O350, craft::O400, craft::O500, craft::O600],
            "green" => [0xd0fae5, 0xa4f4cf, 0x5ee9b5, 0x00d492, 0x00bc7d, 0x009966, 0x007a55],
            "wathet" => [0xcefafe, 0xa2f4fd, 0x53eafd, 0x00d3f3, 0x00b8db, 0x0092b8, 0x007595],
            "carmine" => [0xfce7f3, 0xfccee8, 0xfda5d5, 0xfb64b6, 0xf6339a, 0xe60076, 0xc6005c],
            "purple" => [0xede9fe, 0xddd6ff, 0xc4b4ff, 0xa684ff, 0x8e51ff, 0x7f22fe, 0x7008e7],
            _ => [craft::B100, craft::B200, craft::B300, craft::B400, craft::B500, craft::B600, craft::B700],
        }
    };
    let name = normalize_color(name);
    let (deep, h) = name.split_once('-').map(|(a, b)| (a == "deep", b)).unwrap_or((false, "blue"));
    let v = hue(h);
    if h == "neutral" {
        return if deep {
            Palette { text: c(craft::N00), bg: c(craft::N800), hover_text: c(craft::N00), hover_bg: c(craft::N650), active_text: c(craft::N00), active_bg: c(craft::N950), disabled_bg: c(craft::N200) }
        } else {
            Palette { text: c(craft::N900), bg: c(craft::N200), hover_text: c(craft::N900), hover_bg: c(craft::N300), active_text: c(craft::N900), active_bg: c(craft::N400), disabled_bg: c(craft::N200) }
        };
    }
    if deep {
        // The 500 fill, 400 under the pointer, 600 pressed; white label.
        let w = c(craft::N00);
        return Palette { text: w, bg: c(v[4]), hover_text: w, hover_bg: c(v[3]), active_text: w, active_bg: c(v[5]), disabled_bg: c(v[0]) };
    }
    // A 100 tint with the 600 label, a shade deeper per state; the light
    // hues (orange, wathet) darken their label on hover and press.
    let text = c(v[5]);
    let deeper = if matches!(h, "orange" | "wathet") { c(v[6]) } else { text };
    Palette { text, bg: c(v[0]), hover_text: deeper, hover_bg: c(v[1]), active_text: deeper, active_bg: c(v[2]), disabled_bg: c(v[0]) }
}

/// The button at rest: a white-button drop (1px, soft).
pub fn shadow_s1() -> Vec<BoxShadow> {
    crate::editor_theme::shadow_button()
}

/// The button lifted (hovered, pressed, being edited): the menu drop.
pub fn shadow_s4() -> Vec<BoxShadow> {
    crate::editor_theme::shadow_menu()
}

/// Font size, line height and weight of the button inside a paragraph
/// laid out with `size` / `weight`: in body text and table cells a 13pt
/// label as tall as the line; in a heading (bold or semibold)
/// a semibold label scaled with the heading, as tall as its line.
pub fn metrics(size: f32, weight: FontWeight) -> (f32, f32, FontWeight, bool) {
    let heading = weight != FontWeight::NORMAL;
    if !heading {
        return (13., if size < 15. { 20. } else { 22. }, FontWeight::NORMAL, false);
    }
    let (fs, lh) = match size as i32 {
        21 => (16., 27.),
        19 => (15., 25.),
        18 => (14., 24.),
        16 => (13., 24.),
        _ => (13., 22.),
    };
    (fs, lh, FontWeight::SEMIBOLD, true)
}

// ---- UI: hover popover, 按钮设置 panel, clicks

use super::core::{EQUATION, Pos, Selection};
use super::link::{is_link_input, normalize};
use super::overlays::{Probe, at};
use super::view::{Editor, EditorEvent, Menu};
use crate::model::{ButtonAction, InlineButton};
use crate::input::{TextField, TextFieldEvent};
use crate::model::BlockId;
use crate::editor_theme::{self as theme, Theme};
use crate::editor_ui::{ROW_RADIUS, TipExt, ToastKind, icon, panel, popover};
use gpui::prelude::FluentBuilder;
use std::{cell::Cell, rc::Rc, time::Duration};

/// The popover shows 200ms after the pointer reaches a button and hides
/// 300ms after it leaves both.
const SHOW_DELAY: Duration = Duration::from_millis(200);
const HIDE_DELAY: Duration = Duration::from_millis(300);
const PANEL_W: f32 = 270.;
const MAX_TEXT: usize = 200;
/// The type submenu of the slash / "+" menu, built like the insert panel it
/// hangs off: 8 + its 32pt title + two 32pt rows + 8.
pub const TYPE_MENU_H: f32 = 8. + 32. + 32. * 2. + 8.;
/// Rows of the 执行操作 list.
const ACTION_ROW_H: f32 = 26.;
/// The actions a new button can take (关注文档更新 needs the cloud).
pub const ACTIONS: [ButtonAction; 2] = [ButtonAction::OpenLink, ButtonAction::DuplicatePage];

fn action_icon(a: ButtonAction) -> &'static str {
    match a {
        ButtonAction::OpenLink => "button-link",
        _ => "copy",
    }
}

pub(super) struct Panel {
    at: Pos,
    orig: InlineButton,
    /// The values being edited, drawn in place of the button.
    preview: InlineButton,
    text_err: Option<&'static str>,
    link_err: bool,
    /// The 执行操作 list is open.
    drop: bool,
}

pub(super) struct BtnUi {
    /// The button under the pointer.
    target: Option<Pos>,
    /// The button the popover is showing.
    card: Option<Pos>,
    pub(super) card_over: bool,
    pending: Option<(Option<Pos>, Task<()>)>,
    /// A press on a button: its action runs on release over it.
    press: Option<Pos>,
    panel: Option<Panel>,
    text: Entity<TextField>,
    url: Entity<TextField>,
    /// The 执行操作 select (its list hangs under it).
    select: Rc<Cell<Option<Bounds<Pixels>>>>,
}

impl BtnUi {
    pub(super) fn new(window: &mut Window, cx: &mut Context<Editor>) -> (Self, Vec<Subscription>) {
        let field = |ph: &'static str, cx: &mut Context<Editor>| {
            cx.new(|cx| {
                let mut f = TextField::new(ph, cx);
                f.tab_events = true;
                f.placeholder_color = Theme::text_placeholder();
                f.size = 13.;
                f.line_height = 20.;
                f
            })
        };
        let text = field("请输入按钮", cx);
        let url = field("请输入链接地址", cx);
        let subs = [(&text, true), (&url, false)]
            .into_iter()
            .map(|(f, is_text)| cx.subscribe_in(f, window, move |e: &mut Editor, _, ev: &TextFieldEvent, window, cx| e.on_button_field(ev, is_text, window, cx)))
            .collect();
        let ui = BtnUi { target: None, card: None, card_over: false, pending: None, press: None, panel: None, text, url, select: Rc::default() };
        (ui, subs)
    }
}

fn text_error(t: &str) -> Option<&'static str> {
    if t.trim().is_empty() {
        Some(crate::settings::text("按钮不能为空"))
    } else if t.chars().count() > MAX_TEXT {
        Some(crate::settings::text("按钮文字不能超过 200 个字"))
    } else {
        None
    }
}

impl Editor {
    /// The button at window point `p`.
    fn button_under(&self, p: Point<Pixels>) -> Option<Pos> {
        self.cache.borrow().iter().find_map(|(id, l)| l.button_at(p).map(|ix| Pos::new(*id, ix)))
    }

    pub(super) fn button_rect(&self, p: Pos) -> Option<Bounds<Pixels>> {
        self.cache.borrow().get(&p.block)?.button_bounds(p.offset)
    }

    /// The edited button of `block` and its preview (drawn lifted).
    pub(super) fn button_preview(&self, block: BlockId) -> Option<(usize, InlineButton)> {
        let p = self.btn.panel.as_ref().filter(|p| p.at.block == block)?;
        matches!(self.menu, Some(Menu::Button { block: b, offset }) if b == block && offset == p.at.offset).then(|| (p.at.offset, p.preview.clone()))
    }

    /// Re-measures the row of a button whose preview changes size.
    pub(super) fn button_nonce(&self) -> Option<(BlockId, u64)> {
        let (ix, b) = self.button_preview(self.btn.panel.as_ref()?.at.block)?;
        let mut h = rustc_hash::FxHasher::default();
        std::hash::Hash::hash(&(ix, &b.text, b.action.name(), &b.color), &mut h);
        Some((self.btn.panel.as_ref()?.at.block, std::hash::Hasher::finish(&h) | 1))
    }

    // ---- click

    /// A plain press on a button: the caret stays, the action runs on release.
    pub(super) fn button_press(&mut self, p: Point<Pixels>) -> bool {
        let Some(at) = self.button_under(p) else { return false };
        self.btn.press = Some(at);
        true
    }

    pub(super) fn btn_press_reset(&mut self) {
        self.btn.press = None;
    }

    pub(super) fn button_release(&mut self, p: Point<Pixels>, cx: &mut Context<Self>) {
        let Some(at) = self.btn.press.take() else { return };
        if self.button_under(p) != Some(at) {
            return;
        }
        if let Some(f) = self.core.file_at(at) {
            return self.open_file(super::file::local_path(&f).as_deref(), cx);
        }
        if let Some(l) = self.core.doc_link_at(at) {
            if l.pending {
                cx.emit(EditorEvent::OpenPending(l.id, l.title));
            } else if super::text::DOC_TITLES.with(|t| t.borrow().contains_key(&l.id)) {
                cx.emit(EditorEvent::OpenDoc(l.id));
            }
            return;
        }
        if self.core.reminder_at(at).is_some() {
            return self.open_reminder(at, cx);
        }
        let Some(b) = self.core.button_at(at) else { return };
        match b.action {
            ButtonAction::OpenLink => {
                let src = b.src.trim();
                if is_link_input(src) {
                    cx.open_url(&normalize(src));
                } else {
                    self.toast_kind(ToastKind::Error, crate::settings::text("超链接不合法，请重新输入"), cx);
                }
            }
            ButtonAction::DuplicatePage => cx.emit(EditorEvent::Duplicate),
            ButtonAction::FollowPage => self.toast(crate::settings::text("当前位置暂不支持该功能"), cx),
        }
    }

    // ---- popover

    pub(super) fn button_hover(&mut self, p: Option<Point<Pixels>>, cx: &mut Context<Self>) {
        let target = p.and_then(|p| self.button_under(p));
        if target != self.btn.target {
            self.btn.target = target;
        }
        self.sync_button_card(cx);
    }

    pub(super) fn sync_button_card(&mut self, cx: &mut Context<Self>) {
        let busy = self.read_only || self.menu.is_some() || self.dragging || !self.core.sel.is_collapsed();
        let want = if busy {
            None
        } else if self.btn.card_over {
            self.btn.card
        } else {
            self.btn.target
        };
        // A 日期提醒's card waits longer.
        let delay = match want {
            Some(p) if self.core.reminder_at(p).is_some() => super::reminder::show_delay(),
            Some(_) => SHOW_DELAY,
            None => HIDE_DELAY,
        };
        let u = &mut self.btn;
        if want == u.card {
            u.pending = None;
            return;
        }
        if u.pending.as_ref().is_some_and(|(w, _)| *w == want) {
            return;
        }
        let task = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(delay).await;
            let _ = this.update(cx, |e, cx| {
                if e.btn.pending.as_ref().is_some_and(|(p, _)| *p == want) {
                    e.btn.pending = None;
                    e.btn.card = want;
                    e.btn.card_over = false;
                    cx.notify();
                }
            });
        });
        u.pending = Some((want, task));
    }

    pub(super) fn hide_button_card(&mut self, cx: &mut Context<Self>) {
        self.btn.pending = None;
        self.btn.card_over = false;
        if self.btn.card.take().is_some() {
            cx.notify();
        }
    }

    /// `.button-popover`: 编辑, centred 4px above the button.
    fn button_card(&self, window: &mut Window, cx: &mut Context<Self>) -> Option<AnyElement> {
        let at_ = self.btn.card?;
        if self.read_only || self.menu.is_some() || !self.core.sel.is_collapsed() {
            return None;
        }
        if let Some(f) = self.core.file_at(at_) {
            return self.file_card(at_, &f, cx);
        }
        if let Some(r) = self.core.reminder_at(at_) {
            return self.reminder_card(at_, &r, window, cx);
        }
        if let Some(link) = self.core.doc_link_at(at_) {
            let r = self.button_rect(at_)?;
            let doc = self.links.docs.iter().find(|d| d.id == link.id);
            let title = doc.map(|d| crate::settings::display_title(&d.title).to_string()).unwrap_or_else(|| "目标已删除或不可用".into());
            let body = doc.map(|d| d.preview.clone()).unwrap_or_else(|| link.title.clone());
            let id = link.id.clone();
            let pending = link.pending;
            let pending_title = link.title.clone();
            let card = popover().id("doc-preview").occlude().w(px(340.)).p(px(14.)).flex().flex_col().gap(px(8.))
                .on_hover(cx.listener(|e, h: &bool, _, cx| { e.btn.card_over = *h; e.sync_button_card(cx); }))
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .child(div().text_size(px(16.)).font_weight(FontWeight::SEMIBOLD).child(title))
                .child(div().max_h(px(180.)).overflow_hidden().text_size(px(13.)).line_height(px(21.)).child(body))
                .when(doc.is_some(), |d| d.child(div().id("doc-preview-open").cursor_pointer().text_color(Theme::link()).child("打开文档")
                    .on_click(cx.listener(move |_, _, _, cx| cx.emit(if pending { EditorEvent::OpenPending(id.clone(), pending_title.clone()) } else { EditorEvent::OpenDoc(id.clone()) })))));
            return Some(at(self.rel(point(r.left(), r.bottom() + px(5.))), Corner::TopLeft, card));
        }
        self.core.button_at(at_)?;
        let r = self.probed(Probe::Button(at_))?;
        let w = 1. + 8. + 24. + 8. + 1.;
        let el = popover()
            .id("btn-card")
            .occlude()
            .p(px(8.))
            .on_hover(cx.listener(|e, h: &bool, _, cx| {
                e.btn.card_over = *h;
                e.sync_button_card(cx);
            }))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(
                div()
                    .id("btn-edit")
                    .size(px(24.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(ROW_RADIUS))
                    .cursor_pointer()
                    .hover(|s| s.bg(Theme::fill_hover_strong()))
                    .active(|s| s.bg(Theme::fill_pressed()))
                    .tip_above(crate::settings::text("编辑"), None)
                    .child(icon("pen-line", 18., Theme::text_code()))
                    .on_click(cx.listener(move |e, _, window, cx| e.open_button_panel(at_, window, cx))),
            );
        Some(at(self.rel(point(r.center().x - px(w / 2.), r.top() - px(4.))), Corner::BottomLeft, el))
    }

    // ---- 按钮设置

    pub(super) fn insert_new_button(&mut self, action: ButtonAction, window: &mut Window, cx: &mut Context<Self>) {
        let mut button = InlineButton::new(action);
        button.text = crate::settings::label(action.label());
        if let Some(p) = self.core.insert_button(&button) {
            self.changed(window, cx);
            self.open_button_panel(p, window, cx);
        }
    }

    pub(super) fn open_button_panel(&mut self, p: Pos, window: &mut Window, cx: &mut Context<Self>) {
        let Some(b) = self.core.button_at(p) else { return };
        self.hide_button_card(cx);
        self.hide_link_card(cx);
        self.btn.text.update(cx, |f, cx| {
            f.set_text(&b.text, cx);
            f.select_all_text(cx);
        });
        self.btn.url.update(cx, |f, cx| f.set_text(&b.src, cx));
        self.btn.panel = Some(Panel { at: p, orig: b.clone(), preview: b, text_err: None, link_err: false, drop: false });
        self.core.clear_block_sel();
        self.slash = None;
        self.dragging = false;
        self.menu = Some(Menu::Button { block: p.block, offset: p.offset });
        let fh = self.btn.text.read(cx).focus_handle(cx);
        window.focus(&fh);
        cx.notify();
    }

    fn close_button_panel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(p) = self.btn.panel.take() else { return };
        if matches!(self.menu, Some(Menu::Button { .. })) {
            self.menu = None;
        }
        self.core.sel = Selection::caret(Pos::new(p.at.block, p.at.offset + EQUATION.len()));
        window.focus(&self.focus);
        self.restart_blink(window, cx);
        cx.notify();
    }

    fn submit_button(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(p) = &mut self.btn.panel else { return };
        let text = self.btn.text.read(cx).text.clone();
        let link = self.btn.url.read(cx).text.trim().to_string();
        p.text_err = text_error(&text);
        p.link_err = p.preview.action == ButtonAction::OpenLink && !is_link_input(&link);
        p.drop = false;
        if p.text_err.is_some() || p.link_err {
            return cx.notify();
        }
        let mut b = p.preview.clone();
        b.text = text;
        b.src = if b.action == ButtonAction::OpenLink { normalize(&link) } else { String::new() };
        let at_ = p.at;
        let changed = b != p.orig;
        self.close_button_panel(window, cx);
        if changed {
            self.core.set_button(at_, &b);
            self.changed(window, cx);
        }
    }

    fn on_button_field(&mut self, ev: &TextFieldEvent, is_text: bool, window: &mut Window, cx: &mut Context<Self>) {
        match ev {
            TextFieldEvent::Submit => self.submit_button(window, cx),
            TextFieldEvent::Cancel => self.button_escape(window, cx),
            TextFieldEvent::Tab | TextFieldEvent::ShiftTab => {
                let other = if is_text { self.btn.url.clone() } else { self.btn.text.clone() };
                if self.btn.panel.as_ref().is_some_and(|p| p.preview.action == ButtonAction::OpenLink) {
                    other.update(cx, |f, cx| f.select_all_text(cx));
                    let fh = other.read(cx).focus_handle(cx);
                    window.focus(&fh);
                }
                cx.notify();
            }
            TextFieldEvent::Changed => {
                let t = self.btn.text.read(cx).text.clone();
                if let Some(p) = &mut self.btn.panel {
                    if is_text {
                        p.text_err = text_error(&t);
                        p.preview.text = t.chars().take(MAX_TEXT).collect();
                    } else {
                        p.link_err = false;
                    }
                }
                cx.notify();
            }
            _ => cx.notify(),
        }
    }

    fn button_escape(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match &mut self.btn.panel {
            Some(p) if p.drop => {
                p.drop = false;
                cx.notify();
            }
            _ => self.close_button_panel(window, cx),
        }
    }

    fn set_button_action(&mut self, a: ButtonAction, window: &mut Window, cx: &mut Context<Self>) {
        let Some(p) = &mut self.btn.panel else { return };
        p.drop = false;
        let old = p.preview.action;
        if old != a {
            p.preview.action = a;
            p.link_err = false;
            // The default text follows the type.
            let text = self.btn.text.read(cx).text.clone();
            if text == old.label() || text == crate::settings::text(old.label()) {
                p.preview.text = crate::settings::label(a.label());
                self.btn.text.update(cx, |f, cx| f.set_text(crate::settings::text(a.label()), cx));
            }
            self.btn.url.update(cx, |f, cx| f.set_text("", cx));
        }
        let fh = self.btn.text.read(cx).focus_handle(cx);
        window.focus(&fh);
        cx.notify();
    }

    pub(super) fn button_panel(&self, block: BlockId, offset: usize, window: &mut Window, cx: &mut Context<Self>) -> Option<AnyElement> {
        let p = self.btn.panel.as_ref().filter(|p| p.at == Pos::new(block, offset))?;
        let r = self.probed(Probe::Button(p.at))?;
        let vh = f32::from(window.viewport_size().height);
        let view_top = f32::from(self.list.viewport_bounds().top());
        let below = vh - f32::from(r.bottom());
        let above = f32::from(r.top()) - view_top;
        let up = below < 400. && above > below;
        let red = Theme::danger();
        // The input: 30 tall, radius 9, white, hairline; the accent while
        // focused, red on an error.
        let input = |field: &Entity<TextField>, err: bool| {
            let focused = field.read(cx).is_focused(window);
            div()
                .h(px(30.))
                .px(px(10.))
                .flex()
                .items_center()
                .rounded(px(9.))
                .bg(crate::theme::raised(Theme::bg_body()))
                .border_1()
                .border_color(if err {
                    red
                } else if focused {
                    Theme::primary()
                } else {
                    Theme::line_border()
                })
                .when(!err && !focused, |d| d.hover(|s| s.border_color(theme::ca(craft::N1000, 0.16))))
                .child(field.clone())
        };
        let label = |t: &'static str| div().pb(px(6.)).text_size(px(13.)).line_height(px(20.)).child(crate::settings::text(t));
        let error = |t: &'static str| div().pt(px(4.)).text_size(px(12.)).line_height(px(18.)).text_color(red).child(crate::settings::text(t));
        let item = |last: bool| div().when(!last, |d| d.mb(px(16.)));
        let link = p.preview.action == ButtonAction::OpenLink;

        let mut swatches = div().flex().flex_col().gap(px(6.));
        for row in COLORS {
            let mut r = div().flex().justify_between();
            for name in row {
                let pal = palette(name);
                let on = normalize_color(&p.preview.color) == name;
                // Colour chips: radius 6, a hairline inside; the chosen
                // one ringed in the accent.
                r = r.child(
                    div()
                        .id(SharedString::from(format!("btn-color-{name}")))
                        .relative()
                        .size(px(23.))
                        .rounded(px(6.))
                        .border_1()
                        .border_color(Theme::swatch_border())
                        .bg(pal.bg)
                        .cursor_pointer()
                        .when(on, |d| d.child(div().absolute().top(px(-3.)).left(px(-3.)).size(px(27.)).rounded(px(8.)).border_2().border_color(Theme::primary())))
                        .on_click(cx.listener(move |e, _, _, cx| {
                            if let Some(p) = &mut e.btn.panel {
                                p.preview.color = name.into();
                                p.drop = false;
                            }
                            cx.notify();
                        })),
                );
            }
            swatches = swatches.child(r);
        }

        let select_cell = self.btn.select.clone();
        let select = div()
            .id("btn-action")
            .relative()
            .h(px(30.))
            .px(px(10.))
            .flex()
            .items_center()
            .rounded(px(9.))
            .bg(crate::theme::raised(Theme::bg_body()))
            .border_1()
            .border_color(if p.drop { Theme::primary() } else { Theme::line_border() })
            .when(!p.drop, |d| d.hover(|s| s.border_color(theme::ca(craft::N1000, 0.16))))
            .cursor_pointer()
            .child(canvas(move |b, _, _| select_cell.set(Some(b)), |_, _, _, _| {}).absolute().inset_0())
            .child(div().flex_1().text_size(px(13.)).line_height(px(20.)).child(crate::settings::text(p.preview.action.label())))
            .child(div().ml(px(8.)).child(icon("chevron-down", 12., Theme::text_caption())))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|e, _, _, cx| {
                    cx.stop_propagation();
                    if let Some(p) = &mut e.btn.panel {
                        p.drop = !p.drop;
                    }
                    cx.notify();
                }),
            );

        let content = div()
            .id("btn-panel-content")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .pt(px(12.))
            .px(px(12.))
            .child(item(false).child(label(crate::settings::text("按钮文字"))).child(input(&self.btn.text, p.text_err.is_some())).children(p.text_err.map(error)))
            .child(item(false).child(label(crate::settings::text("按钮颜色"))).child(swatches))
            .child(item(!link).child(label(crate::settings::text("执行操作"))).child(select))
            .when(link, |d| {
                d.child(item(true).child(label(crate::settings::text("超链接地址"))).child(input(&self.btn.url, p.link_err)).when(p.link_err, |d| d.child(error(crate::settings::text("超链接不合法，请重新输入")))))
            });
        // Buttons: 30 tall, radius 9; primary the accent in white
        // semibold, secondary white with a hairline and a 1px drop.
        let button = |id: &'static str, t: &'static str, primary: bool| {
            div()
                .id(id)
                .ml(px(8.))
                .h(px(30.))
                .px(px(14.))
                .flex()
                .items_center()
                .rounded(px(9.))
                .border_1()
                .cursor_pointer()
                .text_size(px(13.))
                .line_height(px(20.))
                .when(primary, |d| {
                    d.bg(Theme::primary())
                        .shadow(theme::shadow_button())
                        .border_color(Theme::primary())
                        .text_color(gpui::white())
                        .font_weight(FontWeight::SEMIBOLD)
                        .hover(|s| s.bg(c(craft::B600)).border_color(c(craft::B600)))
                        .active(|s| s.bg(c(craft::B700)).border_color(c(craft::B700)))
                })
                .when(!primary, |d| {
                    d.bg(crate::theme::raised(Theme::bg_body())).border_color(Theme::button_border()).shadow(theme::shadow_button()).hover(|s| s.bg(c(craft::N100))).active(|s| s.bg(c(craft::N200)))
                })
                .child(crate::settings::text(t))
        };
        let footer = div()
            .flex_none()
            .pt(px(16.))
            .px(px(12.))
            .pb(px(12.))
            .flex()
            .justify_end()
            .child(button("btn-cancel", "取消", false).on_click(cx.listener(|e, _, window, cx| e.close_button_panel(window, cx))))
            .child(button("btn-ok", "确定", true).on_click(cx.listener(|e, _, window, cx| e.submit_button(window, cx))));
        let el = panel()
            .id("btn-panel")
            .key_context("ButtonPanel")
            .occlude()
            .w(px(PANEL_W))
            .max_h(px(500.))
            .flex()
            .flex_col()
            .text_size(px(13.))
            .capture_any_mouse_down(cx.listener(|e, _, _, cx| {
                if let Some(p) = &mut e.btn.panel
                    && p.drop
                {
                    p.drop = false;
                    cx.notify();
                }
            }))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_key_down(cx.listener(|e, ev: &KeyDownEvent, window, cx| {
                if ev.keystroke.key == "escape" {
                    cx.stop_propagation();
                    e.button_escape(window, cx);
                }
            }))
            .child(content)
            .child(footer);
        let (pos, corner) = if up { (point(r.left(), r.top() - px(4.)), Corner::BottomLeft) } else { (point(r.left(), r.bottom() + px(4.)), Corner::TopLeft) };
        Some(at(self.rel(pos), corner, el))
    }

    /// The popover and the 执行操作 list.
    pub(super) fn button_layers(&self, window: &mut Window, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let mut out: Vec<AnyElement> = self.button_card(window, cx).into_iter().collect();
        let Some(p) = self.btn.panel.as_ref().filter(|p| p.drop && matches!(self.menu, Some(Menu::Button { .. }))) else { return out };
        let Some(a) = self.btn.select.get() else { return out };
        let cur = p.preview.action;
        // The select's list: a popover with 26pt rows, a grey hover,
        // the current choice in the accent with a check.
        let mut list = popover().w(a.size.width).py(px(5.)).text_size(px(13.)).occlude();
        for (i, act) in ACTIONS.into_iter().enumerate() {
            let on = act == cur;
            list = list.child(
                div()
                    .id(("btn-action-opt", i))
                    .mx(px(5.))
                    .h(px(ACTION_ROW_H))
                    .px(px(8.))
                    .flex()
                    .items_center()
                    .rounded(px(ROW_RADIUS))
                    .line_height(px(20.))
                    .cursor_pointer()
                    .hover(|s| s.bg(Theme::fill_hover()))
                    .when(on, |d| d.text_color(Theme::primary()))
                    .child(div().flex_1().child(crate::settings::text(act.label())))
                    .when(on, |d| d.child(icon("check", 12., Theme::primary())))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |e, _, window, cx| {
                            cx.stop_propagation();
                            e.set_button_action(act, window, cx);
                        }),
                    ),
            );
        }
        let h = 10. + 2. + ACTION_ROW_H * ACTIONS.len() as f32;
        let vh = f32::from(window.viewport_size().height);
        let below = f32::from(a.bottom()) + 4.;
        let y = if below + h > vh - 8. { f32::from(a.top()) - 4. - h } else { below };
        out.push(
            deferred(anchored().position_mode(AnchoredPositionMode::Window).position(point(a.left(), px(y))).child(list.on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())))
                .with_priority(2)
                .into_any_element(),
        );
        out
    }

    /// 选择按钮类型: the slash / "+" menu's submenu of 按钮.
    pub(super) fn button_type_menu(&self, cx: &mut Context<Self>) -> Stateful<Div> {
        // An editor list popover, like the insert panel: the menu shell,
        // a small secondary title, 32pt rows with a grey hover, 16pt glyphs.
        let mut el = popover()
            .id("btn-type-menu")
            .occlude()
            .w(px(200.))
            .px(px(8.)).py(px(crate::editor_ui::MENU_PAD_Y))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(div().h(px(32.)).px(px(8.)).flex().items_center().text_size(px(12.)).text_color(Theme::text_caption()).child(crate::settings::text("选择按钮类型")));
        for (i, act) in ACTIONS.into_iter().enumerate() {
            el = el.child(
                div()
                    .id(("btn-type", i))
                    .h(px(32.))
                    .px(px(8.))
                    .flex()
                    .items_center()
                    .gap(px(12.))
                    .rounded(px(ROW_RADIUS))
                    .cursor_pointer()
                    .hover(|s| s.bg(Theme::fill_hover()))
                    .child(icon(action_icon(act), 16., Theme::text_code()))
                    .child(div().text_size(px(13.)).line_height(px(20.)).child(crate::settings::text(act.label())))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |e, _, window, cx| {
                            cx.stop_propagation();
                            e.run_slash_button(act, window, cx);
                        }),
                    ),
            );
        }
        el
    }
}
