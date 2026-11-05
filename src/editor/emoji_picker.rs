//! The emoji picker: search
//! with 随机 beside it, 最近 plus seven categories under sticky titles, and
//! the category bar at the bottom. Rows are laid out by hand (every size is
//! fixed) so only the visible ones are built.

use super::emoji::{self, Emoji, PER_LINE};
use super::overlays::at;
use super::view::{Editor, Menu};
use crate::input::TextFieldEvent;
use crate::model::*;
use crate::feishu_theme::{self as theme, Theme};
use crate::editor_ui::{MENU_RADIUS, ROW_RADIUS, TipExt, icon};
use gpui::{prelude::FluentBuilder, *};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EmojiTarget {
    Callout(BlockId),
    /// 文档图标: a tab bar (表情符号 only — no 图标 tab) and 移除.
    DocIcon,
}

/// 456 wide; the picker's 380 plus 10 of padding above it (no tab bar).
const W: f32 = 456.;
const H: f32 = 390.;
/// Popover offset from the trigger, and the window inset it keeps.
const OFFSET: f32 = 9.;
const BOUNDARY: f32 = 20.;
const SCROLL_H: f32 = 262.;
/// The tab bar (pickers with tabs) replaces the 10px top padding.
const TABS_H: f32 = 54.;
const TITLE_H: f32 = 32.5;
/// The search results' section has an empty title (`padding-small` only).
const RESULTS_TITLE_H: f32 = 16.;
const ROW_H: f32 = 36.;
const CELL: f32 = 36.;
/// Rows start 24px in from the popover edge (23 inside its border).
const INSET: f32 = 23.;
const THUMB_MIN: f32 = 96.;

const NAV: [(&str, &str); 8] = [
    ("最近", "emoji-frequent"),
    ("表情符号", "emoji-people"),
    ("动物", "emoji-nature"),
    ("食物", "emoji-foods"),
    ("活动", "emoji-activity"),
    ("地点", "emoji-places"),
    ("物件", "emoji-objects"),
    ("符号", "emoji-symbols"),
];

struct Section {
    title: &'static str,
    emojis: Vec<&'static Emoji>,
    top: f32,
    title_h: f32,
}

impl Section {
    fn height(&self) -> f32 {
        self.title_h + self.emojis.len().div_ceil(PER_LINE) as f32 * ROW_H
    }
}

fn layout(groups: Vec<(&'static str, Vec<&'static Emoji>, f32)>) -> Vec<Section> {
    let mut top = 0.;
    groups
        .into_iter()
        .map(|(title, emojis, title_h)| {
            let s = Section { title, emojis, top, title_h };
            top += s.height();
            s
        })
        .collect()
}

/// 最近 and the categories; one untitled section of results while searching.
fn sections(query: &str) -> Vec<Section> {
    if query.is_empty() {
        let mut g = vec![(crate::settings::text("最近"), emoji::frequent(), TITLE_H)];
        for (i, t) in emoji::CATEGORIES.iter().enumerate() {
            g.push((*t, emoji::category(i).iter().collect(), TITLE_H));
        }
        layout(g)
    } else {
        layout(vec![("", emoji::search(query), RESULTS_TITLE_H)])
    }
}

/// Height of the callout's emoji box: it follows the first child's line so
/// the glyph sits on that line.
pub fn callout_emoji_height(first: Option<&BlockKind>) -> f32 {
    match first {
        Some(BlockKind::Heading { level: 1, .. }) => 36.,
        Some(BlockKind::Heading { level: 2, .. }) => 30.,
        Some(BlockKind::Heading { level: 3, .. }) => 28.,
        Some(BlockKind::Heading { level: 4, .. }) => 24.,
        _ => 26.,
    }
}

impl Editor {
    pub(super) fn emoji_query(&self, cx: &App) -> String {
        self.emoji_field.read(cx).text.trim().to_string()
    }

    /// Click on a trigger: open the picker, or close it if it is open there.
    pub fn toggle_emoji_picker(&mut self, target: EmojiTarget, window: &mut Window, cx: &mut Context<Self>) {
        if matches!(self.menu, Some(Menu::Emoji { target: t, .. }) if t == target) {
            return self.close_menu_focus(window, cx);
        }
        // A 13pt search field (it is made at 16pt).
        self.emoji_field.update(cx, |f, cx| {
            f.size = 13.;
            f.line_height = 20.;
            f.set_text("", cx)
        });
        self.emoji_scroll.set_offset(Point::default());
        self.set_emoji_search_active(false, cx);
        self.menu = Some(Menu::Emoji { target, selected: 0 });
        // Keys go to the search box (typing must not reach the block), but it
        // only looks focused once clicked or typed into.
        let fh = self.emoji_field.read(cx).focus_handle(cx);
        window.focus(&fh);
        cx.notify();
    }

    pub(super) fn on_emoji_field(&mut self, ev: &TextFieldEvent, window: &mut Window, cx: &mut Context<Self>) {
        let Some(Menu::Emoji { target, selected }) = self.menu.clone() else { return };
        let q = self.emoji_query(cx);
        let n = if q.is_empty() { 0 } else { emoji::search(&q).len() };
        let step = |d: isize| (selected as isize + d).clamp(0, n.saturating_sub(1) as isize) as usize;
        let sel = match ev {
            TextFieldEvent::Changed => {
                self.set_emoji_search_active(true, cx);
                self.emoji_scroll.set_offset(Point::default());
                0
            }
            TextFieldEvent::Up if n > 0 => step(-(PER_LINE as isize)),
            TextFieldEvent::Down if n > 0 => step(PER_LINE as isize),
            TextFieldEvent::LeftEdge if n > 0 => step(-1),
            TextFieldEvent::RightEdge if n > 0 => step(1),
            TextFieldEvent::Submit => {
                if let Some(e) = emoji::search(&q).get(selected).copied().filter(|_| n > 0) {
                    self.pick_emoji(target, e, true, window, cx);
                }
                return;
            }
            TextFieldEvent::Cancel => return self.close_menu_focus(window, cx),
            _ => return,
        };
        if sel != selected {
            // Keep the highlighted row in view.
            let y = RESULTS_TITLE_H + (sel / PER_LINE) as f32 * ROW_H;
            let off = -f32::from(self.emoji_scroll.offset().y);
            let off = if y < off { y } else if y + ROW_H > off + SCROLL_H { y + ROW_H - SCROLL_H } else { off };
            self.emoji_scroll.set_offset(point(px(0.), px(-off)));
        }
        self.menu = Some(Menu::Emoji { target, selected: sel });
        cx.notify();
    }

    /// The search box takes keys from the start but shows its caret and
    /// focus ring only once clicked or typed into.
    fn set_emoji_search_active(&mut self, on: bool, cx: &mut Context<Self>) {
        self.emoji_search_active = on;
        self.emoji_field.update(cx, |f, _| f.hide_caret = !on);
    }

    /// `record`: picked from the grid (counts for 最近); 随机 doesn't.
    fn pick_emoji(&mut self, target: EmojiTarget, e: &'static Emoji, record: bool, window: &mut Window, cx: &mut Context<Self>) {
        if record {
            emoji::add_frequent(e.id);
        }
        self.close_menu_focus(window, cx);
        match target {
            EmojiTarget::Callout(block) => self.set_callout(block, |_, _, _, em| *em = e.char.to_string(), window, cx),
            EmojiTarget::DocIcon => self.pick_doc_icon(Some(e), cx),
        }
    }

    /// Where the trigger is (window coordinates, last frame).
    fn emoji_anchor(&self, target: EmojiTarget) -> Option<Bounds<Pixels>> {
        match target {
            EmojiTarget::Callout(block) => {
                let b = self.block_rect(block)?;
                let h = callout_emoji_height(self.core.block(block).and_then(|b| b.children.first()).map(|c| &c.kind));
                // 1px border + 16 padding.
                Some(Bounds::new(b.origin + point(px(17.), px(17.)), size(px(24.), px(h))))
            }
            EmojiTarget::DocIcon => self.cover_ui.icon_bounds.get(),
        }
    }

    pub(super) fn emoji_picker(&self, target: EmojiTarget, selected: usize, window: &Window, cx: &mut Context<Self>) -> Option<AnyElement> {
        let a = self.emoji_anchor(target)?;
        let tabs = target == EmojiTarget::DocIcon;
        let h = if tabs { H + TABS_H - 10. } else { H };
        let q = self.emoji_query(cx);
        let searching = !q.is_empty();
        let secs = sections(&q);
        let content_h = secs.last().map_or(0., |s| s.top + s.height());
        let off = (-f32::from(self.emoji_scroll.offset().y)).clamp(0., (content_h - SCROLL_H).max(0.));

        // Rows in view, positioned by hand inside a box of the full height.
        let mut content = div().relative().w_full().h(px(content_h));
        let current = secs.iter().rposition(|s| s.top <= off + 0.5).unwrap_or(0);
        let hover_bg = Theme::fill_hover_strong();
        for (si, s) in secs.iter().enumerate() {
            if s.top > off + SCROLL_H || s.top + s.height() < off {
                continue;
            }
            // The current section's title is drawn stuck to the top instead.
            if searching || si != current {
                content = content.child(title_el(s.title, s.title_h).absolute().top(px(s.top)).left_0());
            }
            for (ri, row) in s.emojis.chunks(PER_LINE).enumerate() {
                let y = s.top + s.title_h + ri as f32 * ROW_H;
                if y > off + SCROLL_H || y + ROW_H < off {
                    continue;
                }
                let mut r = div().absolute().top(px(y)).left(px(INSET)).flex().gap(px(1.));
                for (ci, e) in row.iter().enumerate() {
                    let e: &'static Emoji = e;
                    let i = ri * PER_LINE + ci;
                    let lit = searching && i == selected;
                    r = r.child(
                        div()
                            .id(("emoji", si * 10_000 + i))
                            .size(px(CELL))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(ROW_RADIUS))
                            .cursor_pointer()
                            .text_size(px(24.))
                            .line_height(px(CELL))
                            .when(lit, |d| d.bg(hover_bg))
                            .hover(move |s| s.bg(hover_bg))
                            .tip(e.name, None)
                            .child(e.char)
                            .when(searching, |d| {
                                d.on_hover(cx.listener(move |ed, hovered: &bool, _, cx| {
                                    if *hovered && let Some(Menu::Emoji { selected, .. }) = &mut ed.menu {
                                        *selected = i;
                                        cx.notify();
                                    }
                                }))
                            })
                            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                            .on_click(cx.listener(move |ed, _, window, cx| ed.pick_emoji(target, e, true, window, cx))),
                    );
                }
                content = content.child(r);
            }
        }
        let sticky = (!searching).then(|| {
            let s = &secs[current];
            let next = secs.get(current + 1).map_or(f32::INFINITY, |n| n.top);
            let y = (next - off - TITLE_H).min(0.);
            title_el(s.title, TITLE_H).absolute().top(px(y)).left_0().bg(theme::ta(255, 255, 255, 0.8))
        });
        let thumb = (content_h > SCROLL_H).then(|| {
            let h = (SCROLL_H * SCROLL_H / content_h).max(THUMB_MIN);
            let top = (SCROLL_H - h) * off / (content_h - SCROLL_H);
            div().absolute().right_0().top(px(top)).w(px(7.)).h(px(h)).rounded(px(3.5)).group_hover("emoji-scroll", |s| s.bg(Theme::scrollbar_thumb()))
        });
        let scroll = div()
            .id("emoji-scroll-area")
            .group("emoji-scroll")
            .relative()
            .flex_none()
            .mt(px(6.))
            .h(px(SCROLL_H))
            .overflow_hidden()
            .child(div().id("emoji-scroll").size_full().overflow_y_scroll().track_scroll(&self.emoji_scroll).child(content))
            .children(sticky)
            .children(thumb);

        let field_on = self.emoji_search_active && self.emoji_field.read(cx).is_focused(window);
        let has_text = !self.emoji_field.read(cx).text.is_empty();
        // The search box: radius 9, white, hairline; the accent once active.
        let search = div()
            .id("emoji-search")
            .relative()
            .w(px(363.))
            .h(px(36.))
            .rounded(px(9.))
            .bg(crate::theme::raised(Theme::bg_body()))
            .border_1()
            .border_color(if field_on { Theme::primary() } else { Theme::line_border() })
            .pl(px(39.))
            .pr(px(43.))
            .flex()
            .items_center()
            .child(div().absolute().left(px(11.)).top(px(9.5)).child(icon("search", 15., Theme::text_caption())))
            .child(div().flex_1().min_w_0().child(self.emoji_field.clone()))
            .when(has_text, |d| {
                d.child(
                    div()
                        .id("emoji-search-clear")
                        .absolute()
                        .right(px(5.))
                        .top(px(5.))
                        .size(px(24.))
                        .rounded(px(ROW_RADIUS))
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .hover(|s| s.bg(Theme::fill_hover()))
                        .child(icon("x", 14., Theme::text_caption()))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|ed, _, window, cx| {
                                cx.stop_propagation();
                                ed.emoji_field.update(cx, |f, cx| f.set_text("", cx));
                                ed.emoji_scroll.set_offset(Point::default());
                                if let Some(Menu::Emoji { selected, .. }) = &mut ed.menu {
                                    *selected = 0;
                                }
                                let fh = ed.emoji_field.read(cx).focus_handle(cx);
                                window.focus(&fh);
                                cx.notify();
                            }),
                        ),
                )
            })
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|ed, _, _, cx| {
                    ed.set_emoji_search_active(true, cx);
                    cx.notify();
                }),
            );
        // 随机: a white button (hairline, 1px drop, radius 9).
        let random = div()
            .id("emoji-random")
            .size(px(36.))
            .rounded(px(9.))
            .bg(crate::theme::raised(Theme::bg_body()))
            .border_1()
            .border_color(Theme::button_border())
            .shadow(theme::shadow_button())
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .hover(|s| s.bg(Theme::fill_hover()))
            .active(|s| s.bg(Theme::fill_pressed()))
            .tip_above(crate::settings::text("随机"), None)
            .child(icon("shuffle", 18., Theme::text_caption()))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_click(cx.listener(move |ed, _, window, cx| ed.pick_emoji(target, emoji::random_pick(), false, window, cx)));

        let mut nav = div().relative().flex_1().px(px(INSET)).flex().items_center().justify_between().child(
            div()
                .absolute()
                .top_0()
                .left_0()
                .right_0()
                .h(px(2.))
                .bg(linear_gradient(180., linear_color_stop(theme::rgbaf(0, 0, 0, 0.05), 0.), linear_color_stop(theme::rgbaf(0, 0, 0, 0.), 1.))),
        );
        for (i, (label, ic)) in NAV.iter().enumerate() {
            let on = !searching && i == current;
            nav = nav.child(
                div()
                    .id(("emoji-nav", i))
                    .size(px(30.))
                    .rounded(px(ROW_RADIUS))
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .hover(|s| s.bg(Theme::fill_hover()))
                    .tip_above(label, None)
                    .child(icon(ic, 18., if on { Theme::primary() } else { Theme::text_caption() }))
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .on_click(cx.listener(move |ed, _, _, cx| {
                        // A category click leaves search.
                        ed.emoji_field.update(cx, |f, cx| f.set_text("", cx));
                        let top = sections("").get(i).map_or(0., |s| s.top);
                        ed.emoji_scroll.set_offset(point(px(0.), px(-top)));
                        cx.notify();
                    })),
            );
        }

        let el = div()
            .id("emoji-picker")
            .occlude()
            .relative()
            .w(px(W))
            .h(px(h))
            .flex()
            .flex_col()
            .bg(crate::theme::raised(Theme::bg_body()))
            .border_1()
            .border_color(Theme::line_border())
            .rounded(px(MENU_RADIUS))
            .shadow(theme::shadow_s4_down())
            .font_family(theme::UI_FONT)
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_scroll_wheel(cx.listener(|_, _, _, cx| cx.notify()))
            .when(tabs, |d| {
                d.child(
                    div().flex_none().h(px(TABS_H)).pl(px(24.)).flex().child(
                        div().pt(px(20.)).pb(px(12.)).text_size(px(13.)).line_height(px(22.)).font_weight(FontWeight::SEMIBOLD).text_color(Theme::primary()).child(crate::settings::text("表情符号")),
                    ),
                )
                .child(
                    div()
                        .id("emoji-remove")
                        .absolute()
                        .top(px(18.))
                        .right(px(21.))
                        .px(px(6.))
                        .rounded(px(ROW_RADIUS))
                        .text_size(px(13.))
                        .line_height(px(22.))
                        .text_color(Theme::text_caption())
                        .cursor_pointer()
                        .hover(|s| s.bg(Theme::fill_hover()).text_color(Theme::text()))
                        .child(crate::settings::text("移除"))
                        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                        .on_click(cx.listener(|ed, _, window, cx| {
                            ed.close_menu_focus(window, cx);
                            ed.pick_doc_icon(None, cx);
                        })),
                )
            })
            .child(div().flex_none().pt(px(if tabs { INSET - 10. } else { INSET })).pl(px(INSET)).flex().gap(px(7.)).child(search).child(random))
            .child(scroll)
            .child(nav);

        // bottomLeft, else right-start, whichever overflows the window less,
        // then shifted inside it (`boundaryPadding: 20`).
        let vp = window.viewport_size();
        let (vw, vh) = (f32::from(vp.width), f32::from(vp.height));
        let (l, t, r, b) = (f32::from(a.left()), f32::from(a.top()), f32::from(a.right()), f32::from(a.bottom()));
        let over_below = (b + OFFSET + h - (vh - BOUNDARY)).max(0.);
        let over_right = (t + h - (vh - BOUNDARY)).max(0.);
        let (x, y) = if over_below <= over_right { (l, b + OFFSET) } else { (r + OFFSET, t) };
        let x = x.min(vw - BOUNDARY - W).max(BOUNDARY);
        let y = y.min(vh - BOUNDARY - h).max(BOUNDARY);
        Some(at(self.rel(point(px(x), px(y))), Corner::TopLeft, el))
    }
}

/// Category title: a 12pt semibold group name in the secondary colour, 24px in.
fn title_el(t: &'static str, h: f32) -> Div {
    div()
        .w_full()
        .h(px(h))
        .pl(px(INSET))
        .flex()
        .items_center()
        .text_size(px(12.))
        .line_height(px(20.))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(Theme::text_caption())
        .child(crate::settings::text(t))
}

#[cfg(test)]
mod tests {
    use super::sections;

    #[test]
    fn section_geometry() {
        let s = sections("");
        assert_eq!(s.len(), 8);
        // 最近 69 tall (one row), 表情符号 1761 (48 rows).
        assert_eq!(s[0].height(), 68.5);
        assert_eq!(s[1].top, 68.5);
        assert_eq!(s[1].height(), 1760.5);
        let r = sections("猫");
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].height(), 16. + 72.);
    }
}
