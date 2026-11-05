//! Links: the URL rules, the link panel, the hover card
//! with its 更多 menu, and opening links on click.

use super::core::{EditKind, Pos, Selection, TITLE};
use super::overlays::at;
use super::view::{Editor, Menu};
use crate::model::*;
use crate::editor_theme::{self as theme, Theme, craft};
use crate::editor_ui::{ROW_RADIUS, TipExt, ToastKind, icon, menu_glyph, menu_row, panel, popover};
use gpui::{prelude::FluentBuilder, *};
use regex::Regex;
use std::ops::Range;
use std::sync::LazyLock;
use std::time::Duration;

// ---- URL rules
//
// Ported from the docx bundle; `{1,256}`-style repeats are `+` here (same
// language for any sane input, without blowing up the automaton) and the
// email rule's `(?!data:)` is checked by hand.

const PROTO: &str = "(?:https?|s?ftp|ftps|nfs|vscode|svn|git)";
const PCT: &str = "%[0-9a-f]{2}";
const PORT: &str = "(?::[0-9]{2,5})?";
const TLD: &str = "[a-z\\-]{2,15}";
const IPV4: &str = r"(?:(?:[0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\.){3}(?:2[0-4][0-9]|25[0-5]|1[0-9]{2}|[1-9][0-9]|[0-9])";
const LABELS: &str = r"(?:[\-A-Z0-9_+~#@]+\.)+";
const HOST: &str = r"[\-A-Z0-9_+#@]+";
const EMAIL: &str = r"[0-9A-Za-z_.!#$%&'*+,\-./=?^`{|}~]+@[A-Za-z0-9_.\-]+\.";
const EMAIL_TLDS: &str = "com|org|net|int|edu|gov|mil|ac|ad|ae|af|ag|ai|al|am|an|ao|aq|ar|as|at|au|aw|ax|az|ba|bb|bd|be|bf|bg|bh|bi|bj|bm|bn|bo|br|bs|bt|bv|bw|by|bz|ca|cc|cf|cg|ch|ci|ck|cl|cm|cn|co|cr|cu|cv|cw|cx|cy|cz|de|dj|dk|dm|do|dz|ec|ee|eg|eh|er|es|et|eu|fi|fj|fk|fm|fo|fr|ga|gb|gd|ge|gf|gh|gi|gl|gm|gn|gp|gr|gs|gt|gu|gw|gy|hk|hm|hn|hr|ht|hu|id|ie|il|im|in|io|iq|ir|is|it|jm|jo|jp|ke|kg|kh|ki|km|kn|kp|kr|kw|ky|kz|la|lb|lc|li|lk|lr|ls|lt|lu|lv|ly|ma|mc|md|me|mf|mg|mh|mk|ml|mm|mn|mo|mp|mq|mr|ms|mt|mu|mv|mw|mx|my|mz|na|nc|ne|nf|ng|ni|nl|no|np|nr|nu|nz|om|pa|pe|pf|pg|ph|pk|pl|pm|pn|pr|ps|pt|pw|py|qa|re|ro|rs|ru|rw|sa|sb|sc|sd|se|sg|sh|si|sj|sk|sl|sm|sn|so|sr|ss|st|su|sv|sx|sy|sz|tc|td|tf|tg|th|tj|tk|tl|tm|tn|to|tp|tr|tt|tv|tw|tz|ua|ug|uk|um|us|uy|uz|va|vc|ve|vg|vi|vn|vu|wf|ws|ye|yt|za|zm|zw|site|top|wtf|xxx|xyz|cloud|engineering|help|one";

/// Path / query / fragment (`c`).
fn tail() -> String {
    format!(r"(?:[/#?](?:[\-A-Z0-9@:_+.~#?&'/=;$,!*\[\](){{}}^|<>]|{PCT})*(?:[\-A-Z0-9&@#/=~()_|]|{PCT})?)?")
}

/// The URL alternatives of the input rule (`p|u|v|_`) and the
/// contains-a-URL rule (`g|u|v|_`).
fn url_rules(lenient_host: bool) -> String {
    let c = tail();
    let l = format!(r"(?-u:\b){c}");
    let u = format!(r"(?:(?:{PROTO}://)?localhost{PORT}|{PROTO}://{IPV4}{PORT}){l}");
    let host = if lenient_host {
        // p: any dotted name, protocol optional.
        format!(r"(?:{PROTO}://)?{LABELS}{TLD}{PORT}{l}")
    } else {
        // g: a protocol, or one of the common TLDs.
        format!(r"(?:{PROTO}://{LABELS}{TLD}|{LABELS}(?:com|cn|tk|de|net|org|uk|info|nl|ru|hk)){PORT}{l}")
    };
    let v = format!(r"{PROTO}://{HOST}{PORT}{c}");
    let w = format!(r"{PROTO}://{HOST}{PORT}{l}");
    format!("{host}|{u}|{v}|{w}")
}

/// `b`: an email address (or `mailto:`), anchored at the start.
static EMAIL_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(&format!(r"(?i)^(?:mailto:{EMAIL}{TLD}|{EMAIL}(?:{EMAIL_TLDS}))(?-u:\b)")).unwrap());
static EMAIL_FULL: LazyLock<Regex> = LazyLock::new(|| Regex::new(&format!(r"(?i)^(?:mailto:{EMAIL}{TLD}|{EMAIL}(?:{EMAIL_TLDS}))(?-u:\b)$")).unwrap());
static INPUT_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(&format!("(?i)^(?:{})$", url_rules(true))).unwrap());
static CONTAINS_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(&format!("(?i){}", url_rules(false))).unwrap());
static VCS_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new("(?i)(?:vscode|svn|git)://.").unwrap());
static SCHEME_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(&format!("(?i)^(?:{PROTO}|mailto):")).unwrap());

fn is_data(s: &str) -> bool {
    s.get(..5).is_some_and(|p| p.eq_ignore_ascii_case("data:"))
}

fn is_email(s: &str) -> bool {
    !is_data(s) && EMAIL_RE.is_match(s)
}

/// `lU(e, true, true)`: the whole input is a link (enables 确认).
pub fn is_link_input(s: &str) -> bool {
    (!is_data(s) && EMAIL_FULL.is_match(s)) || INPUT_RE.is_match(s)
}

/// `gU`: the text contains something URL-like.
pub fn contains_url(s: &str) -> bool {
    is_email(s) || CONTAINS_RE.is_match(s)
}

/// `zE(e, false)`: add the scheme a bare address lacks.
pub fn normalize(s: &str) -> String {
    if VCS_RE.is_match(s) || SCHEME_RE.is_match(s) {
        s.to_string()
    } else if is_email(s) {
        format!("mailto:{s}")
    } else {
        format!("http://{s}")
    }
}

/// The link around `offset` (adjacent runs with the same URL merge).
pub fn link_at(t: &RichText, offset: usize) -> Option<(Range<usize>, String)> {
    let mut cur: Option<(Range<usize>, String)> = None;
    for (r, m) in t.spans() {
        let joins = matches!((&cur, &m.link), (Some((cr, u)), Some(l)) if cr.end == r.start && u == l);
        if joins {
            cur.as_mut().unwrap().0.end = r.end;
            continue;
        }
        if cur.as_ref().is_some_and(|(cr, _)| cr.contains(&offset)) {
            break;
        }
        cur = m.link.clone().map(|l| (r, l));
    }
    cur.filter(|(r, _)| r.contains(&offset))
}

type LinkRef = (BlockId, Range<usize>);

/// Show / hide delay of the hover card.
const CARD_DELAY: Duration = Duration::from_millis(1000);
const CARD_W: f32 = 1. + 8. + 184. + 8. + 32. + 32. + 9. + 32. + 8. + 1.;
const CARD_H: f32 = 42.;
/// The 更多 button's left edge in the card.
const CARD_MORE_X: f32 = CARD_W - 1. - 8. - 24.;
/// The 更多 menus: native-style, 200 wide.
const SUBMENU_W: f32 = 200.;

#[derive(Default)]
pub struct LinkUi {
    /// The link under the pointer.
    pub(super) target: Option<LinkRef>,
    /// The link the hover card is showing.
    pub(super) card: Option<LinkRef>,
    /// Pointer over the card / its 更多 button / the 更多 menu.
    over: [bool; 3],
    /// The card's 更多 menu.
    pub(super) more: bool,
    /// A pending card change.
    pending: Option<(Option<LinkRef>, Task<()>)>,
    more_task: Option<Task<()>>,
    /// Counts card appearances (restarts the fade-in).
    shown: u64,
    /// The panel's text and link when it opened.
    orig: (String, String),
    /// A plain press on a link: opened on release unless it became a drag.
    pub(super) click: Option<String>,
    /// Parts of the open toolbar dropdown under the pointer: 0 its button,
    /// 1 the dropdown itself.
    tb_over: Vec<usize>,
    tb_task: Option<Task<()>>,
}

impl Editor {
    // ---- panel

    /// Open the link panel on `range` of `block` (`only_link`: the
    /// selection keeps its text, only the URL is asked for).
    pub fn open_link_panel(&mut self, block: BlockId, range: Range<usize>, only_link: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(t) = self.core.text_of(block) else { return };
        let range = range.start.min(t.len())..range.end.min(t.len());
        let text = t.text[range.clone()].to_string();
        let link = link_at(t, range.start).filter(|(r, _)| r.end >= range.end).map(|(_, l)| l).unwrap_or_default();
        self.hide_link_card(cx);
        self.link_ui.orig = (text.clone(), link.clone());
        // 13pt inputs (the fields are made with the 14pt default).
        for f in [&self.link_text_field, &self.link_field] {
            f.update(cx, |f, _| {
                f.size = 13.;
                f.line_height = 20.;
            });
        }
        self.link_text_field.update(cx, |f, cx| f.set_text(&text, cx));
        self.link_field.update(cx, |f, cx| f.set_text(&link, cx));
        self.menu = Some(Menu::Link { block, range, only_link });
        let field = if !only_link && text.is_empty() {
            &self.link_text_field
        } else if !only_link && text == link {
            self.link_text_field.update(cx, |f, cx| f.select_all_text(cx));
            &self.link_text_field
        } else {
            &self.link_field
        };
        let fh = field.read(cx).focus_handle(cx);
        window.focus(&fh);
        cx.notify();
    }

    /// ⌘K / the toolbar / slash 链接: a selection asks for a URL only; a
    /// caret inside a link edits it; elsewhere a new link is inserted.
    pub fn link_from_selection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let (s, e) = self.core.ordered();
        if s.block == TITLE || self.core.is_code(s.block) {
            return;
        }
        self.core.clear_block_sel();
        if s == e {
            let r = self.core.text_of(s.block).and_then(|t| link_at(t, s.offset)).map(|(r, _)| r);
            return match r {
                Some(r) => self.open_link_panel(s.block, r, false, window, cx),
                None => self.open_link_panel(s.block, s.offset..s.offset, false, window, cx),
            };
        }
        let end = if s.block == e.block { e.offset } else { self.core.len_of(s.block) };
        self.open_link_panel(s.block, s.offset..end, true, window, cx);
    }

    pub(super) fn link_input_ok(&self, cx: &App) -> bool {
        is_link_input(self.link_field.read(cx).text.trim())
    }

    pub(super) fn submit_link(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(Menu::Link { block, range, only_link }) = self.menu.clone() else { return };
        let link = self.link_field.read(cx).text.trim().to_string();
        if !is_link_input(&link) {
            return;
        }
        let (orig_text, orig_link) = self.link_ui.orig.clone();
        let mut text = if only_link { orig_text.clone() } else { self.link_text_field.read(cx).text.clone() };
        if text.is_empty() {
            text = link.clone();
        }
        self.close_link_panel(window, cx);
        if text == orig_text && link == orig_link {
            return;
        }
        let url = normalize(&link);
        if !contains_url(&url) {
            return self.toast(crate::settings::text("请输入有效的链接。"), cx);
        }
        self.core.sel = Selection { anchor: Pos::new(block, range.start), head: Pos::new(block, range.end) };
        self.core.checkpoint(EditKind::Other);
        self.core.break_coalescing();
        let end = if text == orig_text {
            range.end
        } else {
            self.core.replace_in_block(block, range.clone(), &text);
            range.start + text.len()
        };
        if let Some(t) = self.core.text_mut(block) {
            t.update_marks(range.start..end, |m| m.link = Some(url.clone()));
        }
        self.core.break_coalescing();
        self.core.sel = Selection::caret(Pos::new(block, end));
        self.changed(window, cx);
    }

    pub(super) fn close_link_panel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if matches!(self.menu, Some(Menu::Link { .. })) {
            self.menu = None;
        }
        window.focus(&self.focus);
        self.restart_blink(window, cx);
        cx.notify();
    }

    /// The link panel: 文本 / 链接 rows, or just the URL, and 确认.
    pub(super) fn link_panel(&self, block: BlockId, range: Range<usize>, only_link: bool, window: &Window, cx: &mut Context<Self>) -> Option<AnyElement> {
        let (x, bottom) = {
            let cache = self.cache.borrow();
            let l = cache.get(&block)?;
            let p = l.position_for(range.start);
            let rects = l.rects_for(range.clone());
            let bottom = rects.last().map_or(p.y + l.line_height, |r| r.bottom());
            (p.x, bottom)
        };
        self.probed(super::overlays::Probe::Point(Pos::new(block, range.start)));
        // The input: 30 tall, radius 9, white with a hairline, the
        // accent rim only while focused.
        let input = |field: &Entity<TextField>| {
            let focused = field.read(cx).is_focused(window);
            div()
                .w(px(280.))
                .h(px(30.))
                .px(px(10.))
                .flex()
                .items_center()
                .rounded(px(9.))
                .bg(crate::theme::raised(Theme::bg_body()))
                .border_1()
                .border_color(if focused { Theme::primary() } else { Theme::line_border() })
                .child(field.clone())
        };
        let label = |t: &'static str| div().h(px(30.)).mr(px(8.)).flex().items_center().text_size(px(13.)).child(t);
        let ok = self.link_input_ok(cx);
        // The primary button: accent, white 13pt semibold, radius 9.
        let button = div()
            .id("link-ok")
            .ml(px(16.))
            .min_w(px(72.))
            .h(px(30.))
            .px(px(14.))
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(9.))
            .text_size(px(13.))
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(gpui::white())
            .bg(if ok { Theme::primary() } else { theme::ca(craft::B500, 0.4) })
            .when(ok, |d| {
                d.cursor_pointer()
                    .hover(|s| s.bg(theme::c(craft::B600)))
                    .active(|s| s.bg(theme::c(craft::B700)))
                    .on_click(cx.listener(|e, _, window, cx| e.submit_link(window, cx)))
            })
            .when(!ok, |d| d.cursor(CursorStyle::OperationNotAllowed))
            .child(crate::settings::text("确认"));
        let rows = if only_link {
            div().child(input(&self.link_field))
        } else {
            div()
                .flex()
                .child(div().flex().flex_col().gap(px(10.)).child(label(crate::settings::text("文本"))).child(label(crate::settings::text("链接"))))
                .child(div().flex().flex_col().gap(px(10.)).child(input(&self.link_text_field)).child(input(&self.link_field)))
        };
        let el = panel()
            .id("link-panel")
            .occlude()
            .p(px(16.))
            .flex()
            .items_end()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(rows)
            .child(button);
        Some(at(self.rel(point(x, bottom)), Corner::TopLeft, el))
    }

    // ---- hover card

    /// The link at window point `p` (only over its glyphs).
    pub(super) fn link_under(&self, p: Point<Pixels>) -> Option<LinkRef> {
        let cache = self.cache.borrow();
        for (id, l) in cache.iter() {
            if !l.bounds.contains(&p) {
                continue;
            }
            let Some(t) = self.core.text_of(*id) else { continue };
            for (r, m) in t.spans() {
                if m.link.is_some() && l.rects_for(r.clone()).iter().any(|b| b.contains(&p)) {
                    return link_at(t, r.start).map(|(r, _)| (*id, r));
                }
            }
        }
        None
    }

    /// Pointer moved over the page (no button pressed).
    pub(super) fn link_hover(&mut self, p: Option<Point<Pixels>>, cx: &mut Context<Self>) {
        let target = p.and_then(|p| self.link_under(p)).filter(|(b, _)| *b != TITLE);
        if target != self.link_ui.target {
            self.link_ui.target = target;
            cx.notify();
        }
        self.sync_card(cx);
    }

    /// Move the card towards what the pointer is over, after the delay.
    fn sync_card(&mut self, cx: &mut Context<Self>) {
        let u = &mut self.link_ui;
        let busy = matches!(self.menu, Some(Menu::Link { .. })) || self.dragging;
        let want = if busy {
            None
        } else if u.over[0] || u.over[2] {
            u.card.clone()
        } else {
            u.target.clone()
        };
        if want == u.card {
            u.pending = None;
            return;
        }
        if u.pending.as_ref().is_some_and(|(w, _)| *w == want) {
            return;
        }
        let w = want.clone();
        let task = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(CARD_DELAY).await;
            let _ = this.update(cx, |e, cx| {
                if e.link_ui.pending.as_ref().is_some_and(|(p, _)| *p == w) {
                    e.link_ui.pending = None;
                    e.set_card(w, cx);
                }
            });
        });
        u.pending = Some((want, task));
    }

    fn set_card(&mut self, card: Option<LinkRef>, cx: &mut Context<Self>) {
        let u = &mut self.link_ui;
        if card.is_some() {
            u.shown += 1;
        }
        u.card = card;
        u.more = false;
        u.over = [false; 3];
        cx.notify();
    }

    pub(super) fn hide_link_card(&mut self, cx: &mut Context<Self>) {
        self.link_ui.pending = None;
        if self.link_ui.card.is_some() {
            self.set_card(None, cx);
        }
    }

    fn card_over(&mut self, i: usize, on: bool, cx: &mut Context<Self>) {
        self.link_ui.over[i] = on;
        if i > 0 {
            // 更多 opens on hover and closes shortly after the pointer
            // leaves both the button and its menu.
            if on {
                self.link_ui.more_task = None;
                if !self.link_ui.more {
                    self.link_ui.more = true;
                    cx.notify();
                }
            } else if !self.link_ui.over[1] && !self.link_ui.over[2] {
                self.link_ui.more_task = Some(cx.spawn(async |this, cx| {
                    cx.background_executor().timer(Duration::from_millis(200)).await;
                    let _ = this.update(cx, |e, cx| {
                        if !e.link_ui.over[1] && !e.link_ui.over[2] {
                            e.link_ui.more = false;
                            cx.notify();
                        }
                    });
                }));
            }
        }
        self.sync_card(cx);
    }

    fn remove_link(&mut self, (block, range): LinkRef, window: &mut Window, cx: &mut Context<Self>) {
        self.hide_link_card(cx);
        self.core.checkpoint(EditKind::Other);
        if let Some(t) = self.core.text_mut(block) {
            t.update_marks(range, |m| m.link = None);
        }
        self.core.break_coalescing();
        self.changed(window, cx);
    }

    /// The URL of a link reference.
    fn link_url(&self, (block, range): &LinkRef) -> Option<String> {
        self.core.text_of(*block).and_then(|t| link_at(t, range.start)).map(|(_, l)| l)
    }

    /// The hover card: URL, 编辑链接, 移除链接 | 更多.
    pub(super) fn link_card(&self, window: &mut Window, cx: &mut Context<Self>) -> Option<AnyElement> {
        let lr = self.link_ui.card.clone()?;
        if self.menu.is_some() {
            return None;
        }
        let url = self.link_url(&lr)?;
        let rects = self.cache.borrow().get(&lr.0)?.rects_for(lr.1.clone());
        let first = *rects.first()?;
        let union = rects.iter().skip(1).fold(first, |a, b| a.union(b));
        let view_top = self.list.viewport_bounds().top();
        let x = union.center().x - px(CARD_W / 2.);
        let above = first.top() - px(7. + CARD_H) >= view_top;
        let (pos, corner) = if above { (point(x, first.top() - px(7.)), Corner::BottomLeft) } else { (point(x, union.bottom() + px(7.)), Corner::TopLeft) };
        let btn = |id: &'static str| {
            div()
                .id(id)
                .ml(px(8.))
                .size(px(24.))
                .flex()
                .flex_none()
                .items_center()
                .justify_center()
                .rounded(px(ROW_RADIUS))
                .cursor_pointer()
                .hover(|s| s.bg(Theme::fill_hover_strong()))
                .active(|s| s.bg(Theme::fill_pressed()))
        };
        let edit = lr.clone();
        let remove = lr.clone();
        let more = self.link_ui.more;
        let submenu = more.then(|| {
            let u = url.clone();
            popover()
                .id("link-card-more")
                .occlude()
                .absolute()
                .left(px(CARD_MORE_X - 9. - 1.))
                .top(px(1. + 8. + 24. + 2. - 1.))
                .w(px(SUBMENU_W))
                .py(px(crate::editor_ui::MENU_PAD_Y))
                .on_hover(cx.listener(|e, h: &bool, _, cx| e.card_over(2, *h, cx)))
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .child(menu_item("lc-copy", "globe", crate::settings::text("复制原始网页链接")).on_click(cx.listener(move |e, _, _, cx| {
                    cx.write_to_clipboard(ClipboardItem::new_string(u.clone()));
                    e.link_ui.more = false;
                    e.toast_kind(ToastKind::Success, crate::settings::text("链接已复制到剪贴板"), cx);
                })))
        });
        let el = popover()
            .id("link-card")
            .occlude()
            .relative()
            .h(px(CARD_H))
            .w(px(CARD_W))
            .p(px(8.))
            .flex()
            .items_center()
            .on_hover(cx.listener(|e, h: &bool, _, cx| e.card_over(0, *h, cx)))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(
                div()
                    .w(px(184.))
                    .h(px(24.))
                    .flex_none()
                    .px(px(6.))
                    .flex()
                    .items_center()
                    .child(div().ml(px(4.)).pr(px(4.)).flex_1().min_w_0().overflow_hidden().text_ellipsis().whitespace_nowrap().text_size(px(13.)).text_color(Theme::text_caption()).child(crate::editor_ui::ui_text(url))),
            )
            .child(
                btn("lc-edit")
                    .tip_above(crate::settings::text("编辑链接"), None)
                    .child(icon("pen-line", 18., Theme::text_code()))
                    .on_click(cx.listener(move |e, _, window, cx| {
                        let (b, r) = edit.clone();
                        e.open_link_panel(b, r, false, window, cx);
                    })),
            )
            .child(
                btn("lc-remove")
                    .tip_above(crate::settings::text("移除链接"), None)
                    .child(icon("unlink", 18., Theme::text_code()))
                    .on_click(cx.listener(move |e, _, window, cx| e.remove_link(remove.clone(), window, cx))),
            )
            .child(div().ml(px(8.)).w(px(1.)).h(px(18.)).flex_none().bg(Theme::line_divider()))
            .child(
                btn("lc-more")
                    .when(more, |d| d.bg(Theme::fill_hover_strong()))
                    .child(icon("layout-grid", 18., Theme::text_code()))
                    .on_hover(cx.listener(|e, h: &bool, _, cx| e.card_over(1, *h, cx)))
                    .on_click(cx.listener(|e, _, _, cx| {
                        e.link_ui.more = true;
                        cx.notify();
                    })),
            )
            .children(submenu)
            .with_animation(("link-card-fade", self.link_ui.shown), Animation::new(Duration::from_millis(150)), |el, k| el.opacity(k));
        let _ = window;
        Some(at(self.rel(pos), corner, el))
    }

    // ---- click

    /// A plain press on a link remembers it; the release opens it.
    pub(super) fn link_press(&mut self, p: Pos) {
        self.link_ui.click = self.core.text_of(p.block).and_then(|t| link_at(t, p.offset)).map(|(_, l)| l);
    }

    pub(super) fn link_release(&mut self, cx: &mut Context<Self>) {
        if let Some(url) = self.link_ui.click.take()
            && self.core.sel.is_collapsed()
        {
            cx.open_url(&normalize(&url));
        }
    }

    /// Window rects of the link under the pointer (pointer cursor).
    pub(super) fn pointer_rects(&self) -> Vec<Bounds<Pixels>> {
        let Some((b, r)) = &self.link_ui.target else { return vec![] };
        self.cache.borrow().get(b).map(|l| l.rects_for(r.clone())).unwrap_or_default()
    }

    // ---- toolbar 更多

    /// Hover only tracks an already open dropdown; clicks control its lifetime.
    pub(super) fn tb_drop_over(&mut self, m: Menu, i: usize, on: bool, _cx: &mut Context<Self>) {
        self.link_ui.tb_task = None;
        if self.menu.as_ref() != Some(&m) { return; }
        self.link_ui.tb_over.retain(|value| *value != i);
        if on { self.link_ui.tb_over.push(i); }
    }

    /// The toolbar's 更多 menu: 文档库内搜索, 网页搜索.
    pub(super) fn tb_more_menu(&self, cx: &mut Context<Self>) -> Stateful<Div> {
        popover()
            .id("tb-more-menu")
            .occlude()
            .w(px(SUBMENU_W))
            .py(px(crate::editor_ui::MENU_PAD_Y))
            .on_hover(cx.listener(|e, h: &bool, _, cx| e.tb_drop_over(Menu::TbMore, 1, *h, cx)))
            .child(menu_item("tm-search", "search", crate::settings::text("文档库内搜索")).on_click(cx.listener(|e, _, _, cx| {
                e.menu = None;
                let q = e.core.selected_text();
                cx.emit(super::view::EditorEvent::Search(q));
            })))
            .child(menu_item("tm-web", "globe", crate::settings::text("网页搜索")).on_click(cx.listener(|e, _, _, cx| {
                e.menu = None;
                let q = e.core.selected_text();
                cx.open_url(&format!("https://www.baidu.com/s?wd={}", encode_query(&q)));
                cx.notify();
            })))
    }
}

/// Percent-encode for a query value (`encodeURIComponent`).
fn encode_query(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || b"-_.!~*'()".contains(&b) {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

/// A native-style menu row (`ui::menu_row`): 16pt glyph, 13pt label, the
/// accent with a white label and glyph on hover.
fn menu_item(id: &'static str, ic: &'static str, label: &'static str) -> Stateful<Div> {
    let g = SharedString::from(id);
    menu_row(id, g.clone()).child(menu_glyph(ic, 16., g)).child(div().whitespace_nowrap().child(label))
}

use crate::input::TextField;

#[cfg(test)]
mod tests {
    use super::{contains_url, is_link_input, link_at, normalize};
    use crate::model::{Marks, RichText};

    #[test]
    fn input_rule() {
        for ok in ["example.com", "https://example.com/a?b=c#d", "www.baidu.com", "localhost:8080", "http://127.0.0.1/x", "a@b.com", "mailto:a@b.cn", "ftp://host", "git://x"] {
            assert!(is_link_input(ok), "{ok}");
        }
        for bad in ["", "hello", "hello world", "一个链接", "data:a@b.com", "http://"] {
            assert!(!is_link_input(bad), "{bad}");
        }
    }

    #[test]
    fn normalizes() {
        assert_eq!(normalize("example.com"), "http://example.com");
        assert_eq!(normalize("HTTPS://x.cn"), "HTTPS://x.cn");
        assert_eq!(normalize("a@b.com"), "mailto:a@b.com");
        assert_eq!(normalize("svn://repo"), "svn://repo");
        assert!(contains_url("http://example.com"));
        assert!(contains_url("http://a.io"));
        assert!(!contains_url("hello"));
    }

    #[test]
    fn link_ranges() {
        let mut t = RichText::default();
        t.push("ab", Marks::default());
        t.push("cd", Marks { link: Some("u".into()), ..Default::default() });
        t.push("ef", Marks { link: Some("u".into()), bold: true, ..Default::default() });
        t.push("g", Marks::default());
        assert_eq!(link_at(&t, 3), Some((2..6, "u".into())));
        assert_eq!(link_at(&t, 5), Some((2..6, "u".into())));
        assert_eq!(link_at(&t, 1), None);
        assert_eq!(link_at(&t, 6), None);
    }
}
