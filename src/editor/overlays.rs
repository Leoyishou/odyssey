//! Floating UI over the page: block handle, slash / "+" menu, block menu,
//! text toolbar with its dropdowns, color panels, link editor, code
//! language picker and the callout emoji picker (design-spec §4).

use super::core::{Pos, TITLE};
use super::menus::{self, SLASH_ITEMS, TURN_ITEMS};
use super::table::CellSelKind;
use super::view::{BlockAct, BlockSub, Editor, Mark, Menu};
use crate::model::*;
use crate::feishu_theme::{self as theme, Theme};
use crate::editor_ui::{TipExt, icon, panel, popover};
use gpui::{prelude::FluentBuilder, *};

const SLASH_W: f32 = 247.;
const SLASH_MAX_H: f32 = 351.;
const SCROLLBAR_GUTTER: f32 = 15.;
const MENU_W: f32 = 247.;
/// Text toolbar: 1px border, padding 8, T▾ 44, sep, ≡▾ 44, sep, six 24px
/// marks, A▾ 44 (gap 8), a full-height divider 12px either side, 更多 24.
/// Table handle: 44.5 left of the grid, centred on the column-dot row 16.5 above it.
const TABLE_HANDLE_DX: f32 = 44.5;
const TABLE_HANDLE_DY: f32 = 16.5 + 13.;
const TOOLBAR_W: f32 = 1. + 8. + 44. + 17. + 44. + 17. + 6. * 32. + 44. + 25. + 24. + 8. + 1.;
/// Callout hover bar: one 24px palette button in 10px padding, 1px border;
/// it floats 9px above the callout, centred on it.
const CALLOUT_BAR_W: f32 = 10. + 24. + 10. + 2.;
const CALLOUT_BAR_H: f32 = 40. + 2.;
const CALLOUT_PANEL_W: f32 = 230.;

/// Size picker: 24px cells 6 apart in 12px insets, inside a 1px box with 12px
/// margins and a 1px border, capped at `max-width: 314px` (cells then shrink).
fn picker_width(cols: usize) -> f32 {
    (2. + 24. + 2. + 24. + cols as f32 * 24. + (cols - 1) as f32 * 6.).min(314.)
}

/// Header 12 + 20 + 12, grid box (1 + 12 + rows + 12 + 1), 12 below, 1px borders.
fn picker_height(kind: menus::SlashKind) -> f32 {
    let rows = if kind == menus::SlashKind::Table { 9. * 24. + 8. * 6. } else { 80. };
    1. + 44. + 1. + 12. + rows + 12. + 1. + 12. + 1.
}

/// Place `el` at `rel` (content-column coordinates), above everything else.
pub(super) fn at(rel: Point<Pixels>, corner: Corner, el: impl IntoElement) -> AnyElement {
    div()
        .absolute()
        .top_0()
        .left_0()
        .child(
            deferred(
                anchored()
                    .position_mode(AnchoredPositionMode::Local)
                    .position(rel)
                    .anchor(corner)
                    .snap_to_window_with_margin(px(8.))
                    .child(el),
            )
            .with_priority(1),
        )
        .into_any_element()
}

pub fn kind_icon(k: &BlockKind) -> &'static str {
    match k {
        BlockKind::Text => "type",
        BlockKind::Heading { level, .. } => match level {
            1 => "heading-1",
            2 => "heading-2",
            3 => "heading-3",
            4 => "heading-4",
            5 => "heading-5",
            6 => "heading-6",
            _ => "heading",
        },
        BlockKind::Bullet => "list",
        BlockKind::Ordered { .. } => "list-ordered",
        BlockKind::Todo { .. } => "todo",
        BlockKind::Code { .. } => "braces",
        BlockKind::Quote => "quote",
        BlockKind::Callout { .. } => "callout",
        BlockKind::Divider => "divider",
        BlockKind::Image { .. } => "image",
        BlockKind::Table(_) => "table",
        BlockKind::Grid | BlockKind::Column { .. } => "columns-2",
        BlockKind::Raw { .. } => "file-text",
        BlockKind::SubPageList => "list-tree",
        BlockKind::Countdown(_) => "timer",
        BlockKind::File(_) => "paperclip",
    }
}

/// Handle icon tint, same per-family colours as the slash menu.
pub fn kind_color(k: &BlockKind) -> Hsla {
    theme::c(match k {
        BlockKind::Bullet | BlockKind::Ordered { .. } | BlockKind::Todo { .. } | BlockKind::Grid | BlockKind::Column { .. } => menus::INDIGO,
        BlockKind::Code { .. } => menus::GREEN,
        BlockKind::Divider | BlockKind::Callout { .. } => menus::ORANGE,
        BlockKind::Image { .. } => menus::YELLOW,
        BlockKind::Table(_) => menus::TEAL,
        BlockKind::Raw { .. } => menus::NEUTRAL,
        BlockKind::Countdown(_) => menus::ORANGE,
        BlockKind::File(_) => menus::WATHET,
        _ => menus::BLUE,
    })
}

/// 32-tall menu row: icon, label, optional right hint.
fn row(id: impl Into<ElementId>, icon_name: &str, label: impl Into<SharedString>, right: Option<&str>, highlighted: bool) -> Stateful<Div> {
    row_tinted(id, icon_name, label, right, highlighted, Theme::text(), Theme::text_code())
}

/// `row` with explicit label / icon colours (active and disabled items).
fn row_tinted(
    id: impl Into<ElementId>,
    icon_name: &str,
    label: impl Into<SharedString>,
    right: Option<&str>,
    highlighted: bool,
    text: Hsla,
    icon_color: Hsla,
) -> Stateful<Div> {
    div()
        .id(id)
        .h(px(32.))
        .px(px(8.))
        .flex()
        .items_center()
        .rounded(px(4.))
        .text_size(px(14.))
        .line_height(px(22.))
        .text_color(text)
        .when(highlighted, |d| d.bg(Theme::fill_hover()))
        // Disabled items (drawn in text_disabled) neither hover nor point.
        .when(text != Theme::text_disabled(), |d| d.cursor_pointer().hover(|s| s.bg(Theme::fill_hover())))
        // Same shell as the slash menu: 18px icon at x16, label at x53.
        .child(icon(icon_name, 18., icon_color))
        .child(div().ml(px(19.)).flex_1().whitespace_nowrap().child(crate::settings::label(label.into())))
        .when_some(right, |d, r| {
            if r == "›" {
                d.child(icon("chevron-right", 14., Theme::text_caption()))
            } else {
                d.child(div().text_size(px(12.)).text_color(Theme::text_caption()).child(r.to_string()))
            }
        })
}

/// Color-panel swatch: 26 outer with a 2px ring (blue when selected), 22 inner.
/// Font swatches and the "none" swatch carry a hairline; filled ones don't.
fn swatch(id: impl Into<ElementId>, sel: bool, hairline: bool, inner: Div) -> Stateful<Div> {
    div()
        .id(id)
        .size(px(26.))
        .rounded(px(4.))
        .border_2()
        .border_color(if sel { Theme::primary_500() } else { transparent_black() })
        .cursor_pointer()
        .child(inner.size(px(22.)).rounded(px(2.)).when(hairline && !sel, |d| d.border_1().border_color(Theme::swatch_border())))
}

fn font_swatch(hex: u32) -> Div {
    div().flex().items_center().justify_center().text_size(px(14.)).text_color(theme::c(hex)).child("A")
}

/// White square crossed by a grey diagonal: "no color".
fn none_swatch() -> Div {
    div().flex().items_center().justify_center().bg(crate::theme::raised(Theme::bg_body())).child(icon("swatch-none", 20., Theme::block_placeholder()))
}

fn color_title(t: &'static str) -> Div {
    div().mt(px(7.)).mb(px(4.)).text_size(px(12.)).line_height(px(20.)).text_color(Theme::text()).child(crate::settings::text(t))
}

/// 在下方添加 ›: hovering opens the insert panel next to the handle menu.
fn add_below_row(open: bool, cx: &Context<Editor>) -> Stateful<Div> {
    row("bm-add", "square-plus", "在下方添加", Some("›"), open)
        .on_mouse_move(cx.listener(|e: &mut Editor, _: &MouseMoveEvent, _, cx| {
            if let Some(Menu::Block { sub, .. }) = &mut e.menu
                && !matches!(sub, Some(BlockSub::Add(_)))
            {
                *sub = Some(BlockSub::Add(usize::MAX));
                e.size_pick = (0, 0);
                e.slash_scroll.set_offset(Point::default());
                cx.notify();
            }
        }))
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
}

/// Split the basic grid into rows of 6, a plain 6-column grid, so items
/// just flow with no forced breaks.
fn grid_rows<T>(cells: Vec<(menus::SlashKind, T)>) -> Vec<Vec<T>> {
    let mut rows: Vec<Vec<T>> = vec![];
    for (_, c) in cells {
        match rows.last_mut() {
            Some(r) if r.len() < menus::GRID_COLS => r.push(c),
            _ => rows.push(vec![c]),
        }
    }
    rows
}

fn separator_h() -> Div {
    div().my(px(4.)).h(px(1.)).bg(Theme::line_border())
}

fn group_title(t: &str) -> Div {
    div()
        .flex_none()
        .h(px(32.))
        .px(px(4.))
        .flex()
        .items_center()
        .text_size(px(14.))
        .text_color(Theme::text_caption())
        .child(crate::settings::label(t))
}

/// A layout lookup an overlay depends on; re-checked after paint so an
/// overlay placed from a stale frame is redrawn at the fresh position.
#[derive(Clone, Copy, PartialEq)]
pub enum Probe {
    Point(Pos),
    Block(BlockId),
    /// A table bar segment: (table, column bar?, index).
    Bar(BlockId, bool, usize),
    /// An image's box.
    Image(BlockId),
    File(BlockId),
    /// A 按钮's box.
    Button(Pos),
}

impl Editor {
    pub(super) fn probe(&self, p: Probe) -> Option<Bounds<Pixels>> {
        match p {
            Probe::Point(pos) => {
                let cache = self.cache.borrow();
                let l = cache.get(&pos.block)?;
                Some(Bounds::new(l.position_for(pos.offset), size(px(0.), l.line_height)))
            }
            Probe::Block(id) => self.block_bounds.borrow().get(&id).copied(),
            Probe::Bar(t, cols, i) => self.tables.geo.borrow().get(&(t, cols, i)).copied(),
            Probe::Image(id) => self.img.boxes.borrow().get(&id).copied(),
            Probe::File(id) => self.file.boxes.borrow().get(&id).copied(),
            Probe::Button(p) => self.button_rect(p),
        }
    }

    pub(super) fn probed(&self, p: Probe) -> Option<Bounds<Pixels>> {
        let v = self.probe(p);
        self.probes.borrow_mut().push((p, v));
        v
    }

    pub(super) fn rel(&self, p: Point<Pixels>) -> Point<Pixels> {
        let o = self.content_origin.get();
        point(p.x - o.x, p.y - o.y)
    }

    /// Caret top-left and line height for a position (last frame).
    pub(super) fn point_of(&self, p: Pos) -> Option<(Point<Pixels>, Pixels)> {
        self.probed(Probe::Point(p)).map(|b| (b.origin, b.size.height))
    }

    pub(super) fn block_rect(&self, id: BlockId) -> Option<Bounds<Pixels>> {
        self.probed(Probe::Block(id))
    }

    /// Handle bounds (window coords) for a block: centered on its first line.
    fn handle_rect(&self, id: BlockId, add: bool) -> Option<Bounds<Pixels>> {
        let b = self.block_rect(id)?;
        let (top, lh) = {
            let first = self.probed(Probe::Point(Pos::new(id, 0)));
            match (self.core.kind_of(id), first) {
                (Some(BlockKind::Code { .. }), _) => (b.top() + px(3.), px(26.)),
                (_, Some(l)) => (l.top(), l.size.height),
                _ => (b.top(), px(26.).min(b.size.height)),
            }
        };
        let w = if add { px(28.) } else { px(42.) };
        if matches!(self.core.kind_of(id), Some(BlockKind::Table(_))) {
            return Some(Bounds::new(point(b.left() - px(TABLE_HANDLE_DX), b.top() - px(TABLE_HANDLE_DY)), size(w, px(26.))));
        }
        Some(Bounds::new(point(b.left() + px(self.handle_dx(id, add)), top + (lh - px(26.)) / 2.), size(w, px(26.))))
    }

    /// The handle's left edge relative to the block (tables aside).
    fn handle_dx(&self, id: BlockId, add: bool) -> f32 {
        let mut dx = if add { -34. } else { -48. };
        // Foldable blocks leave room for the fold arrow between handle and text.
        if self.core.can_fold(id) {
            dx -= 14.;
        }
        // A quote's own children put theirs where the quote's would be.
        if matches!(self.core.parent_of(id).map(|p| &p.kind), Some(BlockKind::Quote)) {
            dx -= 14.;
        }
        dx
    }

    fn is_empty_line(&self, id: BlockId) -> bool {
        self.core.block(id).is_some_and(|b| matches!(b.kind, BlockKind::Text) && b.text.is_empty() && b.children.is_empty())
    }

    pub(super) fn render_overlays(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Vec<AnyElement> {
        self.probes.borrow_mut().clear();
        self.callout_zone.borrow_mut().clear();
        let mut out = vec![];
        if self.read_only {
            return out;
        }
        let menu_el = match self.menu.clone() {
            Some(Menu::OrderedNumber { block, pos, edit, depth }) => self.ordered_number_menu(block, pos, edit, depth, cx),
            Some(Menu::Block { block, sub }) => self.block_menu(block, sub, cx),
            Some(Menu::Link { block, range, only_link }) => self.link_panel(block, range, only_link, window, cx),
            Some(Menu::Equation { block, offset }) => self.equation_editor(block, offset, window, cx),
            Some(Menu::Lang { block, selected }) => self.lang_menu(block, selected, window, cx),
            Some(Menu::Emoji { target, selected }) => self.emoji_picker(target, selected, window, cx),
            Some(Menu::CoverEdit) => self.cover_menu(cx),
            Some(Menu::CoverPick { upload }) => self.cover_panel(upload, window, cx),
            Some(Menu::Button { block, offset }) => self.button_panel(block, offset, window, cx),
            Some(Menu::Reminder { block, offset }) => self.reminder_popover(block, offset, window, cx),
            _ => None,
        };
        out.extend(menu_el);
        // After the handle menu: 在下方添加's panel sits on top of it.
        out.extend(self.slash_menu(window, cx));
        out.extend(self.mention_panel(window, cx));
        out.extend(self.callout_bar(cx));
        if let Some(t) = self.toolbar(window, cx) {
            out.push(t);
        }
        out.extend(self.table_toolbar(window, cx));
        out.extend(self.image_toolbar(window, cx));
        out.extend(self.file_toolbar(window, cx));
        out.extend(self.link_card(window, cx));
        out.extend(self.button_layers(window, cx));
        out.extend(self.reminder_layers(window, cx));
        out.extend(self.countdown_overlays(window, cx));
        out
    }

    // ---- handle

    /// The block that shows the handle this frame.
    pub(super) fn handle_block(&self) -> Option<BlockId> {
        if self.read_only || self.dragging || self.slash.is_some() || self.block_drag.as_ref().is_some_and(|d| d.started) {
            return None;
        }
        match &self.menu {
            Some(Menu::Block { block, .. }) | Some(Menu::Add { block, .. }) => Some(*block),
            _ => self.hover.filter(|_| !self.hover_clipped && !self.typing),
        }
    }

    /// The handle, laid out together with its block (not from last frame's
    /// bounds) so it never lags behind when the block changes height.
    /// `first_line` is the height of the block's first line box.
    pub(super) fn handle(&self, id: BlockId, first_line: f32, cx: &mut Context<Self>) -> Option<AnyElement> {
        let kind = self.core.kind_of(id)?.clone();
        let add = self.is_empty_line(id);
        let (w, left) = match (add, &kind) {
            (false, BlockKind::Table(_)) => (42., -TABLE_HANDLE_DX),
            _ => (if add { 28. } else { 42. }, self.handle_dx(id, add)),
        };
        // Tables put the handle on the row of column dots above the grid.
        let top = if matches!(kind, BlockKind::Table(_)) { -TABLE_HANDLE_DY } else { (first_line - 26.) / 2. };
        let active = matches!(&self.menu, Some(Menu::Block { block, .. }) | Some(Menu::Add { block, .. }) if *block == id);
        let el = div()
            .id(("handle", id.0))
            .occlude()
            .w(px(w))
            .h(px(26.))
            .flex()
            .items_center()
            .when(add, |d| d.justify_center())
            .rounded(px(6.))
            .border_1()
            .border_color(Theme::line_border())
            .bg(if active { Theme::handle_active() } else { Theme::bg_body() })
            .hover(|s| s.bg(Theme::handle_active()))
            .cursor(if add { CursorStyle::PointingHand } else { CursorStyle::OpenHand })
            .when(add, |d| d.child(icon("plus", 14., Theme::text())))
            .when(!add, |d| {
                // 1px padding, 24px type-icon box (14px glyph, family tint), 14px grip (#8f959e).
                d.pl(px(1.))
                    .child(div().size(px(24.)).flex().items_center().justify_center().child(icon(kind_icon(&kind), 14., kind_color(&kind))))
                    .child(icon("grip-vertical", 14., Theme::text_placeholder()))
            })
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |e, ev: &MouseDownEvent, window, cx| {
                    cx.stop_propagation();
                    if !add {
                        e.press_handle(id, ev.position);
                    }
                    let open = match &e.menu {
                        Some(Menu::Block { block, .. }) | Some(Menu::Add { block, .. }) => *block == id,
                        _ => false,
                    };
                    if open {
                        e.menu = None;
                        e.core.clear_block_sel();
                        cx.notify();
                    } else {
                        e.open_block_menu(id, add, window, cx);
                    }
                }),
            );
        Some(at(point(px(left), px(top)), Corner::TopLeft, el))
    }

    // ---- slash / add menu

    /// The menu, plus the size picker when one is open.
    fn slash_menu(&self, window: &Window, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let mut out = vec![];
        self.slash_menu_into(window, cx, &mut out);
        out
    }

    fn slash_menu_into(&self, window: &Window, cx: &mut Context<Self>, out: &mut Vec<AnyElement>) -> Option<()> {
        // `beside`: the handle menu's 在下方添加 row (its top-left in window
        // coordinates); the panel opens right next to the menu.
        let (anchor_top, anchor_bottom, selected, beside) = match (&self.slash, &self.menu) {
            (Some(s), _) => {
                let (p, lh) = self.point_of(Pos::new(s.block, s.trigger))?;
                (p, point(p.x, p.y + lh), s.selected, None)
            }
            (None, Some(Menu::Add { block, selected })) => {
                let r = self.handle_rect(*block, true)?;
                (point(r.right() + px(4.), r.top()), point(r.right() + px(4.), r.top()), *selected, None)
            }
            (None, Some(Menu::Block { block, sub: Some(BlockSub::Add(selected)) })) => {
                let r = self.handle_rect(*block, false)?;
                let p = point(r.left() + px(MENU_W), r.bottom() + px(4. + self.block_menu_add_y(*block)));
                (p, p, *selected, Some(p))
            }
            _ => return None,
        };
        let results = self.slash_results();
        let grid_basic = self.slash_grid();
        let mut list = div()
            .id("slash-list")
            .w(px(SLASH_W - SCROLLBAR_GUTTER - 2.))
            .max_h(px(SLASH_MAX_H - 2.))
            .overflow_y_scroll()
            .track_scroll(&self.slash_scroll)
            .p(px(8.))
            .flex()
            .flex_col();
        let hover_sel = |n: usize| {
            cx.listener(move |e: &mut Editor, _: &MouseMoveEvent, _, cx| {
                let cur = match (&mut e.slash, &mut e.menu) {
                    (Some(s), _) => &mut s.selected,
                    (None, Some(Menu::Add { selected, .. })) => selected,
                    (None, Some(Menu::Block { sub: Some(BlockSub::Add(selected)), .. })) => selected,
                    _ => return,
                };
                if *cur != n {
                    *cur = n;
                    e.size_pick = (0, 0);
                    cx.notify();
                }
            })
        };
        // In the "+" panel 表格 and 分栏 open a size picker instead of inserting.
        let add_panel = self.slash.is_none();
        // 按钮 opens its type menu in both.
        let has_picker = |k: menus::SlashKind| {
            k == menus::SlashKind::Button || add_panel && matches!(k, menus::SlashKind::Table | menus::SlashKind::Grid)
        };
        let mut picker_at: Option<(menus::SlashKind, f32)> = None;
        let run = |kind: menus::SlashKind| {
            cx.listener(move |e: &mut Editor, _: &MouseDownEvent, window, cx| {
                cx.stop_propagation();
                e.run_slash(kind, window, cx);
            })
        };
        let mut last_group = usize::MAX;
        let mut content_h: f32 = 16.;
        // Basic group as one 6-per-row grid of 24px icon buttons
        // (spread across the 214px row, 12px between rows, 8px under the grid).
        let mut grid: Option<Vec<(menus::SlashKind, AnyElement)>> = None;
        // Rows hold 6 and 分隔线 always starts a new one; short rows are padded
        // so every row keeps the same columns.
        let grid_el = |cells: Vec<(menus::SlashKind, AnyElement)>| {
            let mut rows = div().flex_none().mb(px(8.)).px(px(4.)).flex().flex_col().gap(px(12.));
            for r in grid_rows(cells) {
                let pad = 6 - r.len();
                rows = rows.child(div().flex().justify_between().children(r).children((0..pad).map(|_| div().size(px(24.)))));
            }
            rows
        };
        let grid_h = |kinds: &[menus::SlashKind]| {
            let n = grid_rows(kinds.iter().map(|k| (*k, ())).collect()).len() as f32;
            24. * n + 12. * (n - 1.).max(0.) + 8.
        };
        for (n, i) in results.iter().enumerate() {
            let it = &SLASH_ITEMS[*i];
            if it.group != last_group {
                last_group = it.group;
                if let Some(g) = grid.take() {
                    list = list.child(grid_el(g));
                }
                list = list.child(group_title(menus::GROUPS[it.group].title));
                content_h += 32.;
                if grid_basic && it.group == 0 {
                    grid = Some(vec![]);
                    let kinds: Vec<menus::SlashKind> =
                        results.iter().map(|i| &SLASH_ITEMS[*i]).filter(|it| it.group == 0).map(|it| it.kind).collect();
                    content_h += grid_h(&kinds);
                }
            }
            let kind = it.kind;
            if let Some(g) = &mut grid {
                let (keys, md) = menus::basic_tip(kind);
                g.push((
                    kind,
                        div()
                            .id(("slash", n))
                            .size(px(24.))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(4.))
                            .cursor_pointer()
                            .when(n == selected, |d| d.bg(Theme::fill_hover()))
                            .tip_md(it.label, keys, md)
                            .child(icon(it.icon, 18., Theme::text_code()))
                            .on_mouse_move(hover_sel(n))
                            .on_mouse_down(MouseButton::Left, run(kind))
                            .into_any_element(),
                ));
                continue;
            }
            let sub = has_picker(kind);
            if sub && n == selected {
                picker_at = Some((kind, content_h - 8.));
            }
            content_h += 32.;
            list = list.child(
                div()
                    .id(("slash", n))
                    // The list is height-capped; rows must not shrink to fit it.
                    .flex_none()
                    .h(px(32.))
                    .px(px(4.))
                    .flex()
                    .items_center()
                    .rounded(px(4.))
                    .cursor_pointer()
                    .when(n == selected, |d| d.bg(Theme::fill_hover()))
                    .child(div().size(px(24.)).flex().items_center().justify_center().child(icon(it.icon, 18., crate::feishu_theme::c(it.color))))
                    .child(div().ml(px(16.)).flex_1().text_size(px(14.)).line_height(px(22.)).child(crate::settings::label(it.label)))
                    .when(sub, |d| d.child(icon("chevron-right", 16., Theme::text_caption())))
                    .on_mouse_move(hover_sel(n))
                    .when(!sub, |d| d.on_mouse_down(MouseButton::Left, run(kind)))
                    .when(sub, |d| d.on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())),
            );
        }
        if let Some(g) = grid.take() {
            list = list.child(grid_el(g));
        }
        if results.is_empty() {
            content_h += 32.;
            list = list.child(
                div().flex_none().h(px(32.)).px(px(8.)).flex().items_center().text_size(px(14.)).text_color(Theme::text_caption()).child(crate::settings::text("未能找到相关结果")),
            );
        }
        // An always-visible 8px thumb, rgba(31,35,41,.3), 3px in
        // from the right border and 2px from the top/bottom.
        let view_h = content_h.min(SLASH_MAX_H - 2.);
        let vh = f32::from(window.viewport_size().height);
        let (pos, corner) = if let Some(p) = beside {
            // The panel lines up 8px above the row, inside the window.
            let top = (f32::from(p.y) - 8.).min(vh - 8. - view_h - 2.).max(8.);
            (point(p.x, px(top)), Corner::TopLeft)
        } else if vh - f32::from(anchor_bottom.y) < 240. {
            (anchor_top, Corner::BottomLeft)
        } else {
            // The menu hangs right off the line box, no gap.
            (anchor_bottom, Corner::TopLeft)
        };
        let thumb = (content_h > view_h).then(|| {
            let track = view_h - 4.;
            let h = (track * view_h / content_h).max(20.);
            let off = -f32::from(self.slash_scroll.offset().y);
            let top = 2. + (track - h) * (off / (content_h - view_h)).clamp(0., 1.);
            div().absolute().right(px(3.)).top(px(top)).w(px(8.)).h(px(h)).rounded_full().bg(Theme::scrollbar_thumb())
        });
        let el = popover()
            .id("slash-menu")
            .occlude()
            .relative()
            .w(px(SLASH_W))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_scroll_wheel(cx.listener(|_, _, _, cx| cx.notify()))
            .child(list)
            .children(thumb);
        out.push(at(self.rel(pos), corner, el));
        let (kind, row_top) = picker_at?;
        // The picker hangs off the list's right edge (over the scrollbar
        // gutter), centred on its row and kept inside the window.
        let (picker, ph) = if kind == menus::SlashKind::Button {
            (self.button_type_menu(cx), super::button::TYPE_MENU_H)
        } else {
            (self.size_picker(kind, cx), picker_height(kind))
        };
        let menu_top = if corner == Corner::TopLeft { pos.y } else { pos.y - px(view_h + 2.) };
        let off = -f32::from(self.slash_scroll.offset().y);
        let top = (f32::from(menu_top) + 1. + row_top - off + 16. - ph / 2.).min(vh - 8. - ph).max(8.);
        let left = pos.x + px(1. + SLASH_W - SCROLLBAR_GUTTER - 2.);
        out.push(at(self.rel(point(left, px(top))), Corner::TopLeft, picker));
        Some(())
    }

    /// The insert-size picker: 9×9 for 表格, 1×5 for 分栏.
    /// Hovering a cell picks rows × cols; leaving the grid resets to 0 × 0;
    /// a click inserts unless the pick is within the default counts.
    fn size_picker(&self, kind: menus::SlashKind, cx: &mut Context<Self>) -> Stateful<Div> {
        let table = kind == menus::SlashKind::Table;
        let (title, rows, cols, def) = if table { (crate::settings::text("插入支持富文本的表格"), 9, 9, (0, 0)) } else { (crate::settings::text("选择栏数"), 1, 5, (1, 1)) };
        let (r, c) = self.size_pick;
        let disabled = r <= def.0 && c <= def.1;
        let cell_h = if rows == 1 { 80. } else { 24. };
        let mut grid = div()
            .id("size-grid")
            .mx(px(12.))
            .mb(px(12.))
            .border_1()
            .border_color(Theme::line_border())
            .rounded(px(2.))
            .overflow_hidden()
            .cursor_pointer()
            .on_hover(cx.listener(|e, hovered: &bool, _, cx| {
                if !*hovered && e.size_pick != (0, 0) {
                    e.size_pick = (0, 0);
                    cx.notify();
                }
            }))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_click(cx.listener(move |e, _, window, cx| {
                let (r, c) = e.size_pick;
                if r > def.0 || c > def.1 {
                    e.run_slash_sized(kind, Some((r, c)), window, cx);
                }
            }));
        for i in 0..rows {
            let mut line = div().flex().h(px(cell_h)).mt(px(if i == 0 { 12. } else { 6. })).when(i + 1 == rows, |d| d.mb(px(12.)));
            for j in 0..cols {
                let (ri, cj) = (i + 1, j + 1);
                let (bg, border) = if ri <= def.0 && cj <= def.1 {
                    (Theme::size_cell_disabled(), Theme::size_cell_disabled())
                } else if !disabled && ri <= r && cj <= c {
                    (Theme::size_cell_active(), Theme::size_cell_active())
                } else {
                    (Theme::size_cell(), Theme::size_cell_border())
                };
                line = line.child(
                    div()
                        .id(("size-cell", i * cols + j))
                        .flex_1()
                        .h(px(cell_h))
                        .when(j == 0, |d| d.ml(px(12.)))
                        .mr(px(if j + 1 == cols { 12. } else { 6. }))
                        .bg(bg)
                        .border_2()
                        .border_color(border)
                        .when(ri <= def.0 && cj <= def.1, |d| d.cursor(CursorStyle::OperationNotAllowed))
                        .on_mouse_move(cx.listener(move |e, _: &MouseMoveEvent, _, cx| {
                            if e.size_pick != (ri, cj) {
                                e.size_pick = (ri, cj);
                                cx.notify();
                            }
                        })),
                );
            }
            grid = grid.child(line);
        }
        let count = if table { format!("{c} x {r}") } else { c.to_string() };
        div()
            .id("size-picker")
            .occlude()
            .w(px(picker_width(cols)))
            .bg(crate::theme::raised(Theme::bg_body()))
            .border_1()
            .border_color(Theme::line_border())
            .rounded(px(6.))
            .shadow(theme::shadow_panel())
            .font_family(theme::UI_FONT)
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(
                div()
                    .my(px(12.))
                    .px(px(12.))
                    .h(px(20.))
                    .flex()
                    .justify_between()
                    .text_size(px(12.))
                    .line_height(px(20.))
                    .text_color(Theme::text_caption())
                    .child(title)
                    .child(div().flex_none().w(px(36.)).ml(px(8.)).text_color(Theme::text()).when(disabled, |d| d.opacity(0.)).child(count)),
            )
            .child(grid)
    }

    // ---- block menu

    fn block_menu(&self, block: BlockId, sub: Option<BlockSub>, cx: &mut Context<Self>) -> Option<AnyElement> {
        let r = self.handle_rect(block, false)?;
        let b = self.core.block(block)?;
        let kind = b.kind.clone();
        if let BlockKind::Table(t) = &kind {
            return self.table_block_menu(block, (t.header_rows > 0, t.header_col), sub, r, cx);
        }
        let shape = menus::MenuShape::of(&kind);
        let act = |a: BlockAct| {
            cx.listener(move |e: &mut Editor, _: &MouseDownEvent, window, cx| {
                cx.stop_propagation();
                e.block_menu_action(block, a.clone(), window, cx);
            })
        };
        let set_sub = |s: Option<BlockSub>| {
            cx.listener(move |e: &mut Editor, _: &MouseMoveEvent, _, cx| {
                if let Some(Menu::Block { sub, .. }) = &mut e.menu
                    && *sub != s
                {
                    *sub = s;
                    cx.notify();
                }
            })
        };
        let mut m = popover().id("block-menu").occlude().w(px(MENU_W)).p(px(8.)).flex().flex_col();
        let turn = shape.turn_items();
        if !turn.is_empty() {
            // A callout's grid shows what its content is, next to 高亮块 itself.
            let current = |k: &BlockKind| match &kind {
                BlockKind::Callout { .. } if !matches!(k, BlockKind::Callout { .. }) => {
                    !b.children.is_empty() && b.children.iter().all(|c| menus::same_kind(&c.kind, k))
                }
                _ => menus::same_kind(&kind, k),
            };
            let mut grid = div().flex().flex_wrap().gap_x(px(12.5)).gap_y(px(12.)).px(px(4.)).py(px(4.));
            for (i, it) in turn.iter().enumerate() {
                let k = (it.kind)();
                let active = current(&k);
                let (tip, keys, md) = (it.tip, it.keys.map(|k| crate::shortcuts::hint(it.action, k)), it.md);
                grid = grid.child(
                    div()
                        .id(("turn", i))
                        .size(px(24.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded(px(4.))
                        .cursor_pointer()
                        .when(active, |d| d.bg(Theme::fill_turninto_active()))
                        .hover(|s| s.bg(Theme::fill_hover()))
                        .tip_md(tip, keys, md)
                        .child(icon(it.icon, 18., if active { Theme::primary() } else { Theme::text_code() }))
                        .on_mouse_down(MouseButton::Left, act(BlockAct::Turn(k))),
                );
            }
            m = m.child(grid).child(separator_h().on_mouse_move(set_sub(None)));
        }
        // Grid: 2 rows of 24 + 12 gap + 4+4 padding, then a 9px separator.
        let indent_y = 8. + if turn.is_empty() { 0. } else { 4. + 24. * 2. + 12. + 4. + 9. };
        // Text blocks get 缩进和对齐 and 颜色; the rest only indent.
        let (indent_icon, indent_label) = if shape == menus::MenuShape::Text { ("indent-align", crate::settings::text("缩进和对齐")) } else { ("indent-increase", crate::settings::text("缩进")) };
        m = m.child(
            row("bm-indent", indent_icon, indent_label, Some("›"), sub == Some(BlockSub::Indent))
                .on_mouse_move(set_sub(Some(BlockSub::Indent))),
        );
        let colorable = shape == menus::MenuShape::Text;
        if colorable {
            m = m.child(row("bm-color", "palette", crate::settings::text("颜色"), Some("›"), sub == Some(BlockSub::Color)).on_mouse_move(set_sub(Some(BlockSub::Color))));
        }
        let adding = matches!(sub, Some(BlockSub::Add(_)));
        m = m
            .child(separator_h().on_mouse_move(set_sub(None)))
            .child(row("bm-cut", "scissors", crate::settings::text("剪切"), None, false).on_mouse_move(set_sub(None)).on_mouse_down(MouseButton::Left, act(BlockAct::Cut)))
            .child(row("bm-copy", "copy", crate::settings::text("复制"), None, false).on_mouse_move(set_sub(None)).on_mouse_down(MouseButton::Left, act(BlockAct::Copy)))
            .child(row("bm-del", "trash-2", crate::settings::text("删除"), None, false).on_mouse_move(set_sub(None)).on_mouse_down(MouseButton::Left, act(BlockAct::Delete)))
            .child(separator_h().on_mouse_move(set_sub(None)))
            .child(add_below_row(adding, cx));
        let color_y = indent_y + 32.;
        let sub_el: Option<AnyElement> = match sub {
            Some(BlockSub::Indent) => {
                // Order: the three alignments, a divider, then indent /
                // outdent. Shortcuts show as tooltips on the right; indent items
                // are greyed out when they wouldn't apply.
                let align = b.align;
                let can_in = self.core.can_indent(block);
                let can_out = self.core.can_outdent(block);
                let align_row = |id: &'static str, ic: &'static str, label: &'static str, keys: &'static str, a: Align| {
                    let (t, i) = if align == a { (Theme::primary(), Theme::primary()) } else { (Theme::text(), Theme::text_code()) };
                    row_tinted(id, ic, label, None, false, t, i)
                        .tip_right(label, Some(keys))
                        .on_mouse_down(MouseButton::Left, act(BlockAct::Align(a)))
                };
                let indent_row = |id: &'static str, ic: &'static str, label: &'static str, keys: Option<&'static str>, enabled: bool, a: BlockAct| {
                    if enabled {
                        let r = row(id, ic, label, None, false).on_mouse_down(MouseButton::Left, act(a));
                        match keys {
                            Some(k) => r.tip_right(label, Some(k)),
                            None => r,
                        }
                    } else {
                        let d = Theme::text_disabled();
                        row_tinted(id, ic, label, None, false, d, d)
                            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    }
                };
                Some(
                    popover()
                        .id("bm-indent-sub")
                        .occlude()
                        .absolute()
                        .left(px(MENU_W + 4.))
                        .top(px(indent_y - 8.))
                        .w(px(MENU_W))
                        .p(px(8.))
                        .when(shape == menus::MenuShape::Text, |d| {
                            d.child(align_row("al-l", "align-left", crate::settings::text("左对齐"), crate::shortcuts::hint("editor::AlignLeft", "⌘ + Shift + L"), Align::Left))
                                .child(align_row("al-c", "align-center", crate::settings::text("居中对齐"), crate::shortcuts::hint("editor::AlignCenter", "⌘ + Shift + E"), Align::Center))
                                .child(align_row("al-r", "align-right", crate::settings::text("右对齐"), crate::shortcuts::hint("editor::AlignRight", "⌘ + Shift + R"), Align::Right))
                                .child(separator_h())
                        })
                        .child(indent_row("in-inc", "indent-increase", crate::settings::text("增加缩进"), None, can_in, BlockAct::Indent))
                        .child(indent_row("in-dec", "indent-decrease", crate::settings::text("减少缩进"), Some("Shift + Tab"), can_out, BlockAct::Outdent))
                        .into_any_element(),
                )
            }
            Some(BlockSub::Color) if colorable => {
                let p = if matches!(kind, BlockKind::Callout { .. }) {
                    self.callout_panel(block, cx).into_any_element()
                } else {
                    self.color_panel(Some(block), cx).into_any_element()
                };
                Some(div().absolute().left(px(MENU_W + 4.)).top(px(color_y - 8.)).child(p).into_any_element())
            }
            _ => None,
        };
        let el = div()
            .relative()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(m)
            .children(sub_el);
        Some(at(self.rel(point(r.left(), r.bottom() + px(4.))), Corner::TopLeft, el))
    }

    /// Top of the 在下方添加 row inside the handle menu (mirrors its layout).
    fn block_menu_add_y(&self, block: BlockId) -> f32 {
        let Some(b) = self.core.block(block) else { return 0. };
        // separators are 9 tall, rows 32; 剪切 / 复制 / 删除 sit between two.
        let tail = 9. + 32. * 3. + 9.;
        if matches!(b.kind, BlockKind::Table(_)) {
            return 8. + 32. + tail + 32. * 3. + 9.;
        }
        let shape = menus::MenuShape::of(&b.kind);
        let grid = if shape.turn_items().is_empty() { 0. } else { 4. + 24. * 2. + 12. + 4. + 9. };
        let color = if shape == menus::MenuShape::Text { 32. } else { 0. };
        8. + grid + 32. + color + tail
    }

    // ---- color panels

    /// Text / background color panel (230×202) for the selection or selected
    /// blocks. From the handle menu (`block`) the background row is the
    /// block palette instead: pale swatches that fill the whole block.
    fn color_panel(&self, block: Option<BlockId>, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let (cur_t, mut cur_b) = self.current_colors();
        if let Some(id) = block {
            cur_b = self.core.block(id).and_then(|b| b.bg.clone());
        }
        let mut fonts = div().flex().justify_between();
        for (i, (name, hex)) in theme::FONT_COLORS.iter().enumerate() {
            let sel = cur_t.as_ref().map(|c| c.0.as_str()) == *name;
            let value = name.map(|n| ColorName(n.to_string()));
            let toggled = if sel && value.is_some() { None } else { value };
            fonts = fonts.child(swatch(("fc", i), sel, true, font_swatch(*hex)).on_mouse_down(
                MouseButton::Left,
                cx.listener(move |e, _, window, cx| {
                    cx.stop_propagation();
                    e.apply_color(Some(toggled.clone()), None, window, cx);
                }),
            ));
        }
        let mut bgs = div().flex().flex_wrap().justify_between().gap_y(px(4.));
        let (names, fill): (&[Option<&str>], fn(&str) -> Option<Hsla>) =
            if block.is_some() { (&theme::CELL_BG_COLORS, theme::bg_swatch) } else { (&theme::BG_COLORS, theme::bg_color) };
        for (i, name) in names.iter().enumerate() {
            let sel = cur_b.as_ref().map(|c| theme::bg_canon(&c.0)) == *name;
            let value = name.map(|n| ColorName(n.to_string()));
            let toggled = if sel && value.is_some() { None } else { value };
            let inner = match name.and_then(fill) {
                Some(fill) => div().bg(fill),
                None => none_swatch(),
            };
            bgs = bgs.child(swatch(("bc", i), sel, name.is_none(), inner).on_mouse_down(
                MouseButton::Left,
                cx.listener(move |e, _, window, cx| {
                    cx.stop_propagation();
                    match block {
                        Some(id) => e.set_block_bg(id, toggled.clone(), window, cx),
                        None => e.apply_color(None, Some(toggled.clone()), window, cx),
                    }
                }),
            ));
        }
        let title = color_title;
        panel()
            .id("color-panel")
            .occlude()
            .w(px(CALLOUT_PANEL_W))
            .pt(px(4.))
            .px(px(10.))
            .pb(px(12.))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(title(crate::settings::text("字体颜色")))
            .child(fonts)
            .child(title(crate::settings::text("背景颜色")))
            .child(bgs)
            .child(
                div()
                    .id("color-reset")
                    .mt(px(12.))
                    .h(px(24.))
                    .rounded(px(6.))
                    .border_1()
                    .border_color(Theme::line_border())
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(12.))
                    .cursor_pointer()
                    .hover(|s| s.bg(Theme::fill_hover()))
                    .child(crate::settings::text("恢复默认"))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |e, _, window, cx| {
                            cx.stop_propagation();
                            match block {
                                Some(id) => {
                                    e.apply_color(Some(None), None, window, cx);
                                    e.set_block_bg(id, None, window, cx);
                                }
                                None => e.apply_color(Some(None), Some(None), window, cx),
                            }
                        }),
                    ),
            )
    }

    /// Records an overlay's painted bounds into the callout hover zone.
    fn zone_probe(&self) -> impl IntoElement {
        let zone = self.callout_zone.clone();
        canvas(move |b, _, _| zone.borrow_mut().push(b), |_, _, _, _| {}).absolute().top_0().left_0().size_full()
    }

    /// The callout hover bar: the palette, whose colour panel opens on
    /// hover, centred below it.
    fn callout_bar(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let (block, open) = match &self.menu {
            Some(Menu::CalloutColor { block, .. }) => (*block, true),
            None => (self.handle_block()?, false),
            _ => return None,
        };
        if !matches!(self.core.kind_of(block), Some(BlockKind::Callout { .. })) {
            return None;
        }
        let b = self.block_rect(block)?;
        let pos = point(b.center().x - px(CALLOUT_BAR_W / 2.), b.top() - px(9. + CALLOUT_BAR_H));
        let palette = div()
            .id("callout-palette")
            .size(px(24.))
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(4.))
            .cursor_pointer()
            .when(open, |d| d.bg(Theme::fill_hover_strong()))
            .hover(|s| s.bg(Theme::fill_hover_strong()))
            .child(icon("palette", 18., Theme::text_code()))
            .on_hover(cx.listener(move |e, hovered: &bool, _, cx| {
                if *hovered && e.menu.is_none() {
                    e.menu = Some(Menu::CalloutColor { block, section: 0 });
                    cx.notify();
                }
            }))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |e, _, _, cx| {
                    cx.stop_propagation();
                    e.menu = if open { None } else { Some(Menu::CalloutColor { block, section: 0 }) };
                    cx.notify();
                }),
            );
        let drop = open.then(|| {
            div()
                .absolute()
                .top(px(CALLOUT_BAR_H + 4.))
                .left(px((CALLOUT_BAR_W - CALLOUT_PANEL_W) / 2. - 1.))
                .child(self.callout_panel(block, cx).relative().child(self.zone_probe()))
        });
        let el = popover()
            .id("callout-bar")
            .occlude()
            .relative()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(div().h(px(40.)).px(px(10.)).flex().items_center().child(palette))
            .child(self.zone_probe())
            .children(drop);
        Some(at(self.rel(pos), Corner::TopLeft, el))
    }

    /// Callout panel: 字体颜色 8 / 边框颜色 8 / 填充颜色 16, 随机, 恢复默认.
    fn callout_panel(&self, block: BlockId, cx: &mut Context<Self>) -> Stateful<Div> {
        let (cur_bg, cur_border, cur_text) = match self.core.kind_of(block) {
            Some(BlockKind::Callout { bg, border, text_color, .. }) => (bg.clone(), border.clone(), text_color.clone()),
            _ => (None, None, None),
        };
        let name_eq = |c: &Option<ColorName>, n: Option<&str>| c.as_ref().map(|c| c.0.as_str()) == n;
        let mut fonts = div().flex().justify_between();
        for (i, (name, hex)) in theme::FONT_COLORS.iter().enumerate() {
            let sel = name_eq(&cur_text, *name);
            let v = if sel { None } else { name.map(|n| ColorName(n.to_string())) };
            fonts = fonts.child(
                swatch(("cf", i), sel, true, font_swatch(*hex))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |e, _, window, cx| {
                        cx.stop_propagation();
                        let v = v.clone();
                        e.set_callout(block, |_, _, t, _| *t = v, window, cx);
                    }),
                ),
            );
        }
        let mut borders = div().flex().justify_between();
        let border_names: [Option<&str>; 8] = [None, Some("gray"), Some("red"), Some("orange"), Some("yellow"), Some("green"), Some("blue"), Some("purple")];
        for (i, name) in border_names.iter().enumerate() {
            let sel = name_eq(&cur_border, *name);
            let v = if sel { None } else { name.map(|n| ColorName(n.to_string())) };
            let inner = match name.and_then(theme::callout_border_swatch) {
                Some(col) => div().rounded(px(2.)).border_2().border_color(col),
                None => none_swatch(),
            };
            borders = borders.child(
                swatch(("cb", i), sel, name.is_none(), inner).on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |e, _, window, cx| {
                        cx.stop_propagation();
                        let v = v.clone();
                        e.set_callout(block, |_, b, _, _| *b = v, window, cx);
                    }),
                ),
            );
        }
        let mut fills = div().flex().flex_wrap().justify_between().gap_y(px(4.));
        for (i, name) in theme::CALLOUT_BG_COLORS.iter().enumerate() {
            let sel = name_eq(&cur_bg, *name);
            let v = if sel { None } else { name.map(|n| ColorName(n.to_string())) };
            let inner = match name.and_then(theme::callout_bg) {
                Some(col) => div().bg(col),
                None => none_swatch(),
            };
            fills = fills.child(swatch(("cg", i), sel, name.is_none(), inner).on_mouse_down(
                MouseButton::Left,
                cx.listener(move |e, _, window, cx| {
                    cx.stop_propagation();
                    let v = v.clone();
                    e.set_callout(block, |bg, _, _, _| *bg = v, window, cx);
                }),
            ));
        }
        let title = color_title;
        let button = |id: &'static str, label: &'static str| {
            div()
                .id(id)
                .flex_1()
                .h(px(24.))
                .rounded(px(6.))
                .border_1()
                .border_color(Theme::line_border())
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(12.))
                .cursor_pointer()
                .hover(|s| s.bg(Theme::fill_hover()))
                .child(crate::settings::label(label))
        };
        panel()
            .id("callout-panel")
            .occlude()
            .w(px(CALLOUT_PANEL_W))
            .pt(px(4.))
            .px(px(10.))
            .pb(px(12.))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(title("字体颜色"))
            .child(fonts)
            .child(title("边框颜色"))
            .child(borders)
            .child(title("填充颜色"))
            .child(fills)
            .child(
                div()
                    .mt(px(12.))
                    .flex()
                    .gap(px(8.))
                    .child(button("callout-random", "随机").on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |e, _, window, cx| {
                            cx.stop_propagation();
                            let hue = theme::HUES[menus::random_index(theme::HUES.len())];
                            e.set_callout(
                                block,
                                |bg, b, _, _| {
                                    *bg = Some(ColorName(format!("light-{hue}")));
                                    *b = Some(ColorName(hue.to_string()));
                                },
                                window,
                                cx,
                            );
                        }),
                    ))
                    .child(button("callout-reset", crate::settings::text("恢复默认")).on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |e, _, window, cx| {
                            cx.stop_propagation();
                            e.set_callout(
                                block,
                                |bg, b, t, _| {
                                    *bg = Some(ColorName("light-orange".into()));
                                    *b = Some(ColorName("orange".into()));
                                    *t = None;
                                },
                                window,
                                cx,
                            );
                        }),
                    )),
            )
    }

    /// The LaTeX editor under an equation.
    fn equation_editor(&self, block: BlockId, offset: usize, window: &mut Window, cx: &App) -> Option<AnyElement> {
        let Some(b) = self.cache.borrow().get(&block).and_then(|l| l.equation_bounds(offset)) else {
            // Not laid out yet (just inserted): place it next frame.
            window.request_animation_frame();
            return None;
        };
        let focused = self.eq_field.read(cx).is_focused(window);
        let primary = Theme::primary_500();
        let el = panel()
            .id("equation-editor")
            .occlude()
            .w(px(400.))
            .pt(px(16.))
            .px(px(16.))
            .pb(px(8.))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(
                div()
                    .id("equation-input")
                    .rounded(px(6.))
                    .border_1()
                    .border_color(if focused { primary } else { theme::ta(0xd0, 0xd3, 0xd6, 1.) })
                    .hover(move |s| s.border_color(primary))
                    .py(px(8.))
                    .px(px(12.))
                    .max_h(px(500.))
                    .overflow_hidden()
                    .flex()
                    .child(self.eq_field.clone()),
            )
            .child(
                div()
                    .mt(px(8.))
                    .text_size(px(12.))
                    .line_height(px(20.))
                    .text_color(Theme::text_placeholder())
                    .child(crate::settings::text("按 ESC 完成输入")),
            );
        Some(at(self.rel(point(b.left(), b.bottom() + px(4.))), Corner::TopLeft, el))
    }

    // ---- code language picker

    fn lang_menu(&self, block: BlockId, selected: usize, window: &Window, cx: &mut Context<Self>) -> Option<AnyElement> {
        let b = self.block_rect(block)?;
        let current = match self.core.kind_of(block) {
            Some(BlockKind::Code { lang, .. }) => lang.clone(),
            _ => String::new(),
        };
        let q = self.lang_field.read(cx).text.clone();
        let langs = menus::filter_languages(&q);
        let mut list = div().id("lang-list").max_h(px(288.)).overflow_y_scroll().flex().flex_col();
        for (i, l) in langs.iter().enumerate() {
            let l: &'static str = l;
            let is_cur = current.eq_ignore_ascii_case(l);
            list = list.child(
                div()
                    .id(("lang", i))
                    .flex_none()
                    .h(px(36.))
                    .px(px(12.))
                    .flex()
                    .items_center()
                    .justify_between()
                    .rounded(px(4.))
                    .cursor_pointer()
                    .text_size(px(14.))
                    .when(i == selected, |d| d.bg(Theme::fill_hover()))
                    .hover(|s| s.bg(Theme::fill_hover()))
                    .child(l)
                    .when(is_cur, |d| d.child(icon("check", 14., Theme::primary())))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |e, _, window, cx| {
                            cx.stop_propagation();
                            e.set_lang(block, l, window, cx);
                        }),
                    ),
            );
        }
        if langs.is_empty() {
            list = list.child(div().h(px(36.)).px(px(12.)).flex().items_center().text_size(px(14.)).text_color(Theme::text_caption()).child(crate::settings::text("无匹配结果")));
        }
        let el = popover()
            .id("lang-menu")
            .occlude()
            .w(px(240.))
            .p(px(4.))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(
                div()
                    .m(px(4.))
                    .mb(px(8.))
                    .h(px(32.))
                    .px(px(8.))
                    .flex()
                    .items_center()
                    .gap(px(6.))
                    .rounded(px(6.))
                    .border_1()
                    .border_color(Theme::line_border())
                    .child(icon("search", 14., Theme::text_placeholder()))
                    .child(self.lang_field.clone()),
            )
            .child(list);
        // Under the language button, which sits ~267px left of the block's right
        // edge; above it when the list wouldn't fit below.
        let h = px(8. + 44. + (langs.len().max(1) as f32 * 36.).min(288.) + 2.);
        let x = b.right() - px(267.);
        let below = b.top() + px(34.);
        let (pos, corner) = if below + h > window.viewport_size().height - px(8.) && b.top() + px(4.) - h >= px(8.) {
            (point(x, b.top() + px(4.)), Corner::BottomLeft)
        } else {
            (point(x, below), Corner::TopLeft)
        };
        Some(at(self.rel(pos), corner, el))
    }

    // ---- floating text toolbar

    fn toolbar(&self, window: &Window, cx: &mut Context<Self>) -> Option<AnyElement> {
        let dropdown = match &self.menu {
            None => None,
            Some(m @ (Menu::TurnInto | Menu::Align | Menu::Color | Menu::TbMore)) => Some(m.clone()),
            _ => return None,
        };
        let sel = self.core.sel;
        if self.core.cell_sel.is_some() || self.toolbar_hidden || matches!(self.menu, Some(Menu::Equation { .. })) || self.dragging || sel.is_collapsed() || self.core.in_block_mode() || self.slash.is_some() {
            return None;
        }
        if !self.focus.is_focused(window) {
            return None;
        }
        let (s, e) = self.core.ordered();
        if s.block == TITLE {
            return None;
        }
        let (sp, _) = self.point_of(s)?;
        let (ep, elh) = self.point_of(e)?;
        let (hp, _) = self.point_of(sel.head)?;
        // The bar is centred on the selection focus and floats 5px
        // above the first line box (below the last one when there is no room).
        let viewport = self.list.viewport_bounds();
        let view_top = viewport.top();
        let pane_left = self.rel(point(viewport.left() + px(8.), px(0.))).x;
        let x = (hp.x - px(TOOLBAR_W / 2.)).max(pane_left);
        let above = sp.y - px(45.) >= view_top;
        let pos = if above { point(x, sp.y - px(45.)) } else { point(x, ep.y + elh + px(5.)) };

        let head_kind = self.core.kind_of(s.block).cloned().unwrap_or(BlockKind::Text);
        let align = self.current_align();
        let btn = |id: &'static str| {
            div()
                .id(id)
                .h(px(24.))
                .min_w(px(24.))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(4.))
                .cursor_pointer()
                .hover(|s| s.bg(Theme::fill_hover_strong()))
        };
        let sep = || div().w(px(1.)).h(px(20.)).bg(Theme::line_border());
        let mark_btn = |id: &'static str, ic: &'static str, tip: &'static str, keys: &'static str, md: Option<&'static str>, mark: Mark, get: fn(&Marks) -> bool| {
            let active = self.mark_active(get);
            btn(id)
                .w(px(24.))
                .when(active, |d| d.bg(Theme::fill_turninto_active()))
                .tip_md(tip, Some(keys), md)
                .child(icon(ic, 18., if active { Theme::primary() } else { Theme::text_code() }))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |e, _, window, cx| {
                        cx.stop_propagation();
                        e.menu = None;
                        e.toggle_mark(mark, window, cx);
                    }),
                )
        };
        // Hover opens the dropdowns; a press keeps them open.
        let open = |m: Menu| {
            cx.listener(move |e: &mut Editor, _: &MouseDownEvent, _, cx| {
                cx.stop_propagation();
                e.menu = Some(m.clone());
                cx.notify();
            })
        };
        let hover = |m: Menu| cx.listener(move |e: &mut Editor, h: &bool, _, cx| e.tb_drop_over(m.clone(), 0, *h, cx));
        let turn_below = (window.viewport_size().height - pos.y - px(52.)).max(px(0.));
        let turn_above = (pos.y - px(8.)).max(px(0.));
        let turn_up = turn_below < px(338.) && turn_above > turn_below;
        let turn_height = if turn_up { turn_above } else { turn_below }.min(px(338.));
        let more_height = px(2. + 2. * crate::editor_ui::MENU_PAD_Y + 2. * crate::editor_ui::MENU_ROW_H);
        let more_above = (pos.y - px(10.)).max(px(0.));
        let more_below = (window.viewport_size().height - pos.y - px(52.)).max(px(0.));
        // Down unless it doesn't fit there, like the other dropdowns.
        let more_up = more_below < more_height && more_above > more_below;
        let drop = |m: Menu, x: f32| {
            div()
                .id("tb-drop")
                .absolute()
                .when(matches!(m, Menu::TurnInto) && turn_up, |d| d.bottom(px(44.)))
                .when(!matches!(m, Menu::TurnInto) || !turn_up, |d| d.top(px(44.)))
                .left(px(x))
                .on_hover(cx.listener(move |e: &mut Editor, h: &bool, _, cx| e.tb_drop_over(m.clone(), 1, *h, cx)))
        };
        // The chevron flips while its dropdown is open.
        let chevron = |m: Menu| icon(if dropdown.as_ref() == Some(&m) { "caret-up" } else { "caret-down" }, 12., Theme::text_placeholder());
        let align_icon = match align {
            Align::Left => "align-left",
            Align::Center => "align-center",
            Align::Right => "align-right",
        };
        let color_btn = self.color_split("tb-color", matches!(dropdown, Some(Menu::Color)), cx);

        // x offsets of the dropdown buttons (padding 8, gap 8, separators 1).
        let x_turn = 8.;
        let x_align = x_turn + 44. + 8. + 1. + 8.;
        let x_color = x_align + 44. + 8. + 1. + 8. + 6. * (24. + 8.);
        let bar = div()
            .h(px(40.))
            .px(px(8.))
            .flex()
            .items_center()
            .gap(px(8.))
            .child(
                btn("tb-turn")
                    .w(px(44.))
                    .gap(px(6.))
                    .when(matches!(dropdown, Some(Menu::TurnInto)), |d| d.bg(Theme::fill_hover_strong()))
                    .child(icon(kind_icon(&head_kind), 18., Theme::text_code()))
                    .child(chevron(Menu::TurnInto))
                    .on_hover(hover(Menu::TurnInto))
                    .on_mouse_down(MouseButton::Left, open(Menu::TurnInto)),
            )
            .child(sep())
            .child(
                btn("tb-align")
                    .w(px(44.))
                    .gap(px(6.))
                    .when(matches!(dropdown, Some(Menu::Align)), |d| d.bg(Theme::fill_hover_strong()))
                    .child(icon(align_icon, 18., Theme::text_code()))
                    .child(chevron(Menu::Align))
                    .on_hover(hover(Menu::Align))
                    .on_mouse_down(MouseButton::Left, open(Menu::Align)),
            )
            .child(sep())
            .child(mark_btn("tb-b", "bold", crate::settings::text("粗体"), crate::shortcuts::hint("editor::Bold", "⌘ + B"), Some(crate::settings::text("**文本** 空格")), Mark::Bold, |m| m.bold))
            .child(mark_btn("tb-s", "strikethrough", crate::settings::text("删除线"), crate::shortcuts::hint("editor::Strike", "⌘ + Shift + X"), Some(crate::settings::text("~~文本~~ 空格")), Mark::Strike, |m| m.strike))
            .child(mark_btn("tb-i", "italic", crate::settings::text("斜体"), crate::shortcuts::hint("editor::Italic", "⌘ + I"), Some(crate::settings::text("*文本* 空格")), Mark::Italic, |m| m.italic))
            .child(mark_btn("tb-u", "underline", crate::settings::text("下划线"), crate::shortcuts::hint("editor::Underline", "⌘ + U"), Some(crate::settings::text("~文本~ 空格")), Mark::Underline, |m| m.underline))
            .child(
                btn("tb-link")
                    .w(px(24.))
                    .tip_above(crate::settings::text("链接"), Some(crate::shortcuts::hint("editor::Link", "⌘ + K")))
                    .child(icon("link", 18., Theme::text_code()))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|e, _, window, cx| {
                            cx.stop_propagation();
                            e.link_from_selection(window, cx);
                        }),
                    ),
            )
            .child(mark_btn("tb-code", "code", crate::settings::text("代码"), crate::shortcuts::hint("editor::InlineCode", "⌘ + Control + C"), Some(crate::settings::text("`代码`")), Mark::Code, |m| m.code))
            .child(color_btn)
            .child(div().mx(px(4.)).w(px(1.)).h(px(40.)).bg(Theme::line_border()))
            .child(
                btn("tb-more")
                    .w(px(24.))
                    .when(matches!(dropdown, Some(Menu::TbMore)), |d| d.bg(Theme::fill_hover_strong()))
                    .child(icon("layout-grid", 18., Theme::text_code()))
                    .on_hover(hover(Menu::TbMore))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|e, _, _, cx| {
                            cx.stop_propagation();
                            e.menu = Some(Menu::TbMore);
                            cx.notify();
                        }),
                    ),
            );

        let drop_el: Option<AnyElement> = match dropdown {
            Some(Menu::TurnInto) => {
                let mut list = popover().id("tb-turn-menu").occlude().w(px(200.)).p(px(8.)).max_h(turn_height).overflow_y_scroll().on_scroll_wheel(|_, _, cx| cx.stop_propagation());
                for (i, it) in TURN_ITEMS.iter().enumerate() {
                    let k = (it.kind)();
                    let active = menus::same_kind(&head_kind, &k);
                    list = list.child(
                        row(("tt", i), it.icon, it.label, None, false)
                            .when(active, |d| d.child(icon("check", 14., Theme::primary())))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |e, _, window, cx| {
                                    cx.stop_propagation();
                                    e.menu = None;
                                    e.turn(k.clone(), false, window, cx);
                                }),
                            ),
                    );
                }
                Some(drop(Menu::TurnInto, x_turn).child(list).into_any_element())
            }
            Some(Menu::Align) => {
                let item = |id: &'static str, ic: &'static str, label: &'static str, keys: &'static str, a: Align| {
                    let (t, i) = if align == a { (Theme::primary(), Theme::primary()) } else { (Theme::text(), Theme::text_code()) };
                    row_tinted(id, ic, label, None, false, t, i).tip_above(label, Some(keys)).on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |e, _, window, cx| {
                            cx.stop_propagation();
                            e.menu = None;
                            e.align(a, window, cx);
                        }),
                    )
                };
                let list = popover()
                    .id("tb-align-menu")
                    .occlude()
                    .w(px(180.))
                    .p(px(8.))
                    .child(item("ta-l", "align-left", crate::settings::text("左对齐"), crate::shortcuts::hint("editor::AlignLeft", "⌘ + Shift + L"), Align::Left))
                    .child(item("ta-c", "align-center", crate::settings::text("居中对齐"), crate::shortcuts::hint("editor::AlignCenter", "⌘ + Shift + E"), Align::Center))
                    .child(item("ta-r", "align-right", crate::settings::text("右对齐"), crate::shortcuts::hint("editor::AlignRight", "⌘ + Shift + R"), Align::Right));
                Some(drop(Menu::Align, x_align).child(list).into_any_element())
            }
            Some(Menu::Color) => Some(drop(Menu::Color, x_color).child(self.color_panel(None, cx)).into_any_element()),
            Some(Menu::TbMore) => Some(div().absolute().when(more_up, |d| d.bottom(px(44.))).when(!more_up, |d| d.top(px(44.))).right_0().child(self.tb_more_menu(cx)).into_any_element()),
            _ => None,
        };
        let el = popover()
            .id("toolbar")
            .occlude()
            .relative()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(bar)
            .children(drop_el);
        Some(at(self.rel(pos), Corner::TopLeft, el))
    }
}

/// Table toolbar: padding 8, merge 24, sep, palette 24, sep, T▾ 44, sep,
/// ≡▾ 44, sep, five 24px marks, A▾ 44 (gap 8); + sep, delete 24 for whole
/// rows / columns.
const TABLE_TB_W: f32 = 8. + 24. + 17. + 24. + 17. + 44. + 17. + 44. + 17. + 5. * 32. + 44. + 8.;
const TABLE_TB_H: f32 = 40. + 2.;

/// 28 × 16 switch (menu rows).
fn switch(on: bool) -> Div {
    div()
        .flex_none()
        .w(px(28.))
        .h(px(16.))
        .rounded_full()
        .p(px(2.))
        .flex()
        .when(on, |d| d.justify_end())
        .bg(if on { Theme::primary_500() } else { Theme::block_placeholder() })
        .child(div().size(px(12.)).rounded_full().bg(crate::theme::raised(Theme::bg_body())))
}

impl Editor {
    /// The toolbars' A▾. The A applies the last used colors at once (by
    /// default the yellow highlight); the chevron opens the panel on
    /// hover. Each half darkens over the button's own hover fill.
    fn color_split(&self, id: &'static str, open: bool, cx: &Context<Self>) -> Stateful<Div> {
        let (lt, lb) = self.last_color.clone();
        let fill = || crate::theme::adapt(rgba(0x1f23290d).into());
        div()
            .id(id)
            .h(px(24.))
            .flex()
            .rounded(px(4.))
            .overflow_hidden()
            .cursor_pointer()
            .hover(|s| s.bg(fill()))
            .child(
                div()
                    .id((id, 0usize))
                    .size(px(24.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .hover(|s| s.bg(fill()))
                    .child(
                        div()
                            .size(px(20.))
                            .rounded(px(2.))
                            .when_some(lb.as_ref().and_then(|c| theme::bg_color(&c.0)), |d, c| d.bg(c))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(16.))
                            .text_color({
                                let c = lt.as_ref().and_then(|c| theme::text_color(&c.0)).unwrap_or(Theme::text());
                                if lb.is_some() { crate::theme::on_mark(c) } else { c }
                            })
                            .child("A"),
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|e, _, window, cx| {
                            cx.stop_propagation();
                            e.menu = None;
                            let (t, b) = e.last_color.clone();
                            e.apply_color(Some(t), Some(b), window, cx);
                        }),
                    ),
            )
            .child(
                div()
                    .id((id, 1usize))
                    .w(px(20.))
                    .h(px(24.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .hover(|s| s.bg(fill()))
                    .when(open, |d| d.bg(fill()))
                    .child(icon(if open { "caret-up" } else { "caret-down" }, 12., Theme::text_placeholder()))
                    .on_hover(cx.listener(|e, h: &bool, _, cx| e.tb_drop_over(Menu::Color, 0, *h, cx)))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|e, _, _, cx| {
                            cx.stop_propagation();
                            e.menu = Some(Menu::Color);
                            cx.notify();
                        }),
                    ),
            )
    }

    /// Window bounds of the selected cells and of the table's top edge.
    fn cell_sel_rect(&self) -> Option<(Bounds<Pixels>, Pixels)> {
        let (id, r) = self.core.cell_range()?;
        let l = self.probed(Probe::Bar(id, true, r.left))?;
        let rr = self.probed(Probe::Bar(id, true, r.right))?;
        let t = self.probed(Probe::Bar(id, false, r.top))?;
        let b = self.probed(Probe::Bar(id, false, r.bottom))?;
        let table_top = self.probed(Probe::Bar(id, true, 0))?.bottom();
        Some((Bounds::from_corners(point(l.left(), t.top()), point(rr.right(), b.bottom())), table_top))
    }

    /// The toolbar over selected table cells: no text toolbar and no
    /// context menu, so this bar carries the cell operations.
    fn table_toolbar(&self, window: &Window, cx: &mut Context<Self>) -> Option<AnyElement> {
        let s = self.core.cell_sel?;
        let dropdown = match &self.menu {
            None => None,
            Some(m @ (Menu::TurnInto | Menu::Align | Menu::Color | Menu::CellBg)) => Some(m.clone()),
            _ => return None,
        };
        if self.toolbar_hidden || self.dragging || self.tables.drag.is_some() || !self.focus.is_focused(window) {
            return None;
        }
        let (range_top, whole) = {
            let (_, r) = self.core.cell_range()?;
            (r.top, s.kind != CellSelKind::Range)
        };
        let (rect, table_top) = self.cell_sel_rect()?;
        let w = TABLE_TB_W + if whole { 17. + 24. } else { 0. };
        let viewport = self.list.viewport_bounds();
        let pane_left = viewport.left() + px(12.);
        let x = (rect.center().x - px(w / 2.)).max(pane_left);
        let bottom = if range_top == 0 { table_top - px(22.) } else { rect.top() - px(15.) };
        let y = if bottom - px(TABLE_TB_H) >= viewport.top() { bottom - px(TABLE_TB_H) } else { rect.bottom() + px(15.) };

        let cell = self.core.first_selected_cell();
        let valign = cell.and_then(|c| c.valign.clone());
        let cur_bg = cell.and_then(|c| c.bg.clone());
        let n_cells = self.core.selected_cells().len();
        let merged = self.core.merged_selected().is_some();
        let head_kind = self
            .core
            .cell_text_blocks()
            .first()
            .and_then(|id| self.core.kind_of(*id).cloned())
            .unwrap_or(BlockKind::Text);
        let align = self.current_align();

        let btn = |id: &'static str| {
            div()
                .id(id)
                .h(px(24.))
                .min_w(px(24.))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(4.))
                .cursor_pointer()
                .hover(|s| s.bg(Theme::fill_hover_strong()))
        };
        let sep = || div().w(px(1.)).h(px(20.)).bg(Theme::line_border());
        let toggle = |m: Menu| {
            cx.listener(move |e: &mut Editor, _: &MouseDownEvent, _, cx| {
                cx.stop_propagation();
                e.menu = if e.menu.as_ref() == Some(&m) { None } else { Some(m.clone()) };
                cx.notify();
            })
        };
        // Hover opens the dropdowns; a press keeps them open.
        let open = |m: Menu| {
            cx.listener(move |e: &mut Editor, _: &MouseDownEvent, _, cx| {
                cx.stop_propagation();
                e.menu = Some(m.clone());
                cx.notify();
            })
        };
        let hover = |m: Menu| cx.listener(move |e: &mut Editor, h: &bool, _, cx| e.tb_drop_over(m.clone(), 0, *h, cx));
        let turn_below = (window.viewport_size().height - y - px(52.)).max(px(0.));
        let turn_above = (y - px(8.)).max(px(0.));
        let turn_up = turn_below < px(338.) && turn_above > turn_below;
        let turn_height = if turn_up { turn_above } else { turn_below }.min(px(338.));
        let drop = |m: Menu, x: f32| {
            div()
                .id("ttb-drop")
                .absolute()
                .when(matches!(m, Menu::TurnInto) && turn_up, |d| d.bottom(px(44.)))
                .when(!matches!(m, Menu::TurnInto) || !turn_up, |d| d.top(px(44.)))
                .left(px(x))
                .on_hover(cx.listener(move |e: &mut Editor, h: &bool, _, cx| e.tb_drop_over(m.clone(), 1, *h, cx)))
        };
        // The chevron flips while its dropdown is open.
        let chevron = |m: Menu| icon(if dropdown.as_ref() == Some(&m) { "caret-up" } else { "caret-down" }, 12., Theme::text_placeholder());
        let mark_btn = |id: &'static str, ic: &'static str, tip: &'static str, keys: &'static str, md: Option<&'static str>, mark: Mark, get: fn(&Marks) -> bool| {
            let active = self.mark_active(get);
            btn(id)
                .w(px(24.))
                .when(active, |d| d.bg(Theme::fill_turninto_active()))
                .tip_md(tip, Some(keys), md)
                .child(icon(ic, 18., if active { Theme::primary() } else { Theme::text_code() }))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |e, _, window, cx| {
                        cx.stop_propagation();
                        e.menu = None;
                        e.toggle_mark(mark, window, cx);
                    }),
                )
        };

        let merge_btn = if merged {
            btn("ttb-merge")
                .w(px(24.))
                .tip_above(crate::settings::text("拆分单元格"), None)
                .child(icon("split-cells", 18., Theme::text_code()))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|e, _, window, cx| {
                        cx.stop_propagation();
                        e.menu = None;
                        e.core.table_unmerge();
                        e.changed(window, cx);
                    }),
                )
        } else if n_cells >= 2 {
            btn("ttb-merge")
                .w(px(24.))
                .tip_above(crate::settings::text("合并单元格"), None)
                .child(icon("merge-cells", 18., Theme::text_code()))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|e, _, window, cx| {
                        cx.stop_propagation();
                        e.menu = None;
                        e.core.table_merge();
                        e.changed(window, cx);
                    }),
                )
        } else {
            div()
                .id("ttb-merge")
                .size(px(24.))
                .flex()
                .items_center()
                .justify_center()
                .child(icon("merge-cells", 18., Theme::text_disabled()))
        };
        let align_icon = match align {
            Align::Left => "align-left",
            Align::Center => "align-center",
            Align::Right => "align-right",
        };
        let mut bar = div()
            .h(px(40.))
            .px(px(8.))
            .flex()
            .items_center()
            .gap(px(8.))
            .child(merge_btn)
            .child(sep())
            .child(
                btn("ttb-bg")
                    .w(px(24.))
                    .when(matches!(dropdown, Some(Menu::CellBg)), |d| d.bg(Theme::fill_hover_strong()))
                    .when(!matches!(dropdown, Some(Menu::CellBg)), |d| d.tip_above(crate::settings::text("单元格背景颜色"), None))
                    .child(icon("palette", 18., Theme::text_code()))
                    .on_mouse_down(MouseButton::Left, toggle(Menu::CellBg)),
            )
            .child(sep())
            .child(
                btn("ttb-turn")
                    .w(px(44.))
                    .gap(px(6.))
                    .when(matches!(dropdown, Some(Menu::TurnInto)), |d| d.bg(Theme::fill_hover_strong()))
                    .child(icon(kind_icon(&head_kind), 18., Theme::text_code()))
                    .child(chevron(Menu::TurnInto))
                    .on_hover(hover(Menu::TurnInto))
                    .on_mouse_down(MouseButton::Left, open(Menu::TurnInto)),
            )
            .child(sep())
            .child(
                btn("ttb-align")
                    .w(px(44.))
                    .gap(px(6.))
                    .when(matches!(dropdown, Some(Menu::Align)), |d| d.bg(Theme::fill_hover_strong()))
                    .child(icon(align_icon, 18., Theme::text_code()))
                    .child(chevron(Menu::Align))
                    .on_hover(hover(Menu::Align))
                    .on_mouse_down(MouseButton::Left, open(Menu::Align)),
            )
            .child(sep())
            .child(mark_btn("ttb-b", "bold", crate::settings::text("粗体"), crate::shortcuts::hint("editor::Bold", "⌘ + B"), Some(crate::settings::text("**文本** 空格")), Mark::Bold, |m| m.bold))
            .child(mark_btn("ttb-s", "strikethrough", crate::settings::text("删除线"), crate::shortcuts::hint("editor::Strike", "⌘ + Shift + X"), Some(crate::settings::text("~~文本~~ 空格")), Mark::Strike, |m| m.strike))
            .child(mark_btn("ttb-i", "italic", crate::settings::text("斜体"), crate::shortcuts::hint("editor::Italic", "⌘ + I"), Some(crate::settings::text("*文本* 空格")), Mark::Italic, |m| m.italic))
            .child(mark_btn("ttb-u", "underline", crate::settings::text("下划线"), crate::shortcuts::hint("editor::Underline", "⌘ + U"), Some(crate::settings::text("~文本~ 空格")), Mark::Underline, |m| m.underline))
            .child(mark_btn("ttb-code", "code", crate::settings::text("代码"), crate::shortcuts::hint("editor::InlineCode", "⌘ + Control + C"), Some(crate::settings::text("`代码`")), Mark::Code, |m| m.code))
            .child(self.color_split("ttb-color", matches!(dropdown, Some(Menu::Color)), cx));
        if whole {
            let label = if s.kind == CellSelKind::Cols { crate::settings::text("删除列") } else { crate::settings::text("删除行") };
            bar = bar.child(sep()).child(
                btn("ttb-del")
                    .w(px(24.))
                    .tip_above(label, None)
                    .child(icon("bin", 18., if self.tables.danger { Theme::danger() } else { Theme::text_code() }))
                    .on_hover(cx.listener(|e, hovered: &bool, _, cx| {
                        if e.tables.danger != *hovered {
                            e.tables.danger = *hovered;
                            cx.notify();
                        }
                    }))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|e, _, window, cx| {
                            cx.stop_propagation();
                            e.menu = None;
                            e.tables.danger = false;
                            e.core.table_delete_selected();
                            e.changed(window, cx);
                        }),
                    ),
            );
        }

        let x_bg = 8. + 24. + 17.;
        let x_turn = x_bg + 24. + 17.;
        let x_align = x_turn + 44. + 17.;
        let x_color = x_align + 44. + 17. + 5. * 32.;
        let drop_el: Option<AnyElement> = match dropdown {
            Some(Menu::TurnInto) => {
                let mut list = popover().id("ttb-turn-menu").occlude().w(px(200.)).p(px(8.)).max_h(turn_height).overflow_y_scroll().on_scroll_wheel(|_, _, cx| cx.stop_propagation());
                for (i, it) in TURN_ITEMS.iter().enumerate() {
                    let k = (it.kind)();
                    if matches!(k, BlockKind::Callout { .. }) {
                        continue;
                    }
                    let active = menus::same_kind(&head_kind, &k);
                    list = list.child(
                        row(("ttt", i), it.icon, it.label, None, false)
                            .when(active, |d| d.child(icon("check", 14., Theme::primary())))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |e, _, window, cx| {
                                    cx.stop_propagation();
                                    e.menu = None;
                                    e.turn(k.clone(), false, window, cx);
                                }),
                            ),
                    );
                }
                Some(drop(Menu::TurnInto, x_turn).child(list).into_any_element())
            }
            Some(Menu::Align) => {
                let item = |id: &'static str, ic: &'static str, label: &'static str, active: bool, keys: Option<&'static str>| {
                    let (t, i) = if active { (Theme::primary(), Theme::primary()) } else { (Theme::text(), Theme::text_code()) };
                    let r = row_tinted(id, ic, label, None, false, t, i).when(active, |d| d.child(icon("check", 14., Theme::primary())));
                    match keys {
                        Some(k) => r.tip_right(label, Some(k)),
                        None => r,
                    }
                };
                let h = |id, ic, label, keys, a: Align| {
                    item(id, ic, label, align == a, Some(keys)).on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |e, _, window, cx| {
                            cx.stop_propagation();
                            e.menu = None;
                            e.align(a, window, cx);
                        }),
                    )
                };
                let v = |id, ic, label, value: Option<&'static str>| {
                    item(id, ic, label, valign.as_deref() == value, None).on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |e, _, window, cx| {
                            cx.stop_propagation();
                            e.menu = None;
                            e.core.table_set_valign(value);
                            e.changed(window, cx);
                        }),
                    )
                };
                let list = popover()
                    .id("ttb-align-menu")
                    .occlude()
                    .w(px(232.))
                    .p(px(8.))
                    .child(h("tta-l", "align-left", crate::settings::text("左对齐"), crate::shortcuts::hint("editor::AlignLeft", "⌘ + Shift + L"), Align::Left))
                    .child(h("tta-c", "align-center", crate::settings::text("居中对齐"), crate::shortcuts::hint("editor::AlignCenter", "⌘ + Shift + E"), Align::Center))
                    .child(h("tta-r", "align-right", crate::settings::text("右对齐"), crate::shortcuts::hint("editor::AlignRight", "⌘ + Shift + R"), Align::Right))
                    .child(separator_h())
                    .child(v("tta-t", "valign-top", crate::settings::text("顶部对齐"), None))
                    .child(v("tta-m", "valign-middle", crate::settings::text("垂直居中"), Some("middle")))
                    .child(v("tta-b", "valign-bottom", crate::settings::text("底部对齐"), Some("bottom")));
                Some(drop(Menu::Align, x_align).child(list).into_any_element())
            }
            Some(Menu::Color) => Some(drop(Menu::Color, x_color).child(self.color_panel(None, cx)).into_any_element()),
            Some(Menu::CellBg) => Some(div().absolute().top(px(44.)).left(px(x_bg)).child(self.cell_bg_panel(cur_bg, cx)).into_any_element()),
            _ => None,
        };
        let el = popover()
            .id("table-toolbar")
            .occlude()
            .relative()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(bar)
            .children(drop_el);
        Some(at(self.rel(point(x, y)), Corner::TopLeft, el))
    }

    /// 单元格背景颜色 (230 × 142): 2 × 8 swatches and 恢复默认.
    fn cell_bg_panel(&self, cur: Option<ColorName>, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let set = |v: Option<ColorName>| {
            cx.listener(move |e: &mut Editor, _: &MouseDownEvent, window, cx| {
                cx.stop_propagation();
                e.menu = None;
                e.core.table_set_bg(v.clone());
                e.changed(window, cx);
            })
        };
        let mut grid = div().flex().flex_wrap().justify_between().gap_y(px(4.));
        for (i, name) in theme::CELL_BG_COLORS.iter().enumerate() {
            let sel = cur.as_ref().map(|c| c.0.as_str()) == *name;
            let inner = match name.and_then(theme::bg_swatch) {
                Some(fill) => div().bg(fill),
                None => none_swatch(),
            };
            grid = grid.child(
                swatch(("cbg", i), sel, name.is_none(), inner).on_mouse_down(MouseButton::Left, set(name.map(|n| ColorName(n.to_string())))),
            );
        }
        panel()
            .id("cell-bg-panel")
            .occlude()
            .w(px(CALLOUT_PANEL_W))
            .pt(px(4.))
            .px(px(10.))
            .pb(px(12.))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(color_title(crate::settings::text("单元格背景颜色")))
            .child(grid)
            .child(
                div()
                    .id("cell-bg-reset")
                    .mt(px(12.))
                    .h(px(24.))
                    .rounded(px(6.))
                    .border_1()
                    .border_color(Theme::line_border())
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(12.))
                    .cursor_pointer()
                    .hover(|s| s.bg(Theme::fill_hover()))
                    .child(crate::settings::text("恢复默认"))
                    .on_mouse_down(MouseButton::Left, set(None)),
            )
    }

    /// The handle menu of a table: 缩进›, 剪切 / 复制 / 删除, the 标题行 /
    /// 标题列 switches, 均分列宽 and 在下方添加.
    fn table_block_menu(&self, block: BlockId, (hrow, hcol): (bool, bool), sub: Option<BlockSub>, r: Bounds<Pixels>, cx: &mut Context<Self>) -> Option<AnyElement> {
        let act = |a: BlockAct| {
            cx.listener(move |e: &mut Editor, _: &MouseDownEvent, window, cx| {
                cx.stop_propagation();
                e.block_menu_action(block, a.clone(), window, cx);
            })
        };
        let set_sub = |s: Option<BlockSub>| {
            cx.listener(move |e: &mut Editor, _: &MouseMoveEvent, _, cx| {
                if let Some(Menu::Block { sub, .. }) = &mut e.menu
                    && *sub != s
                {
                    *sub = s;
                    cx.notify();
                }
            })
        };
        let plain = |id: &'static str, ic: &'static str, label: &'static str, a: BlockAct| {
            row(id, ic, label, None, false).on_mouse_move(set_sub(None)).on_mouse_down(MouseButton::Left, act(a))
        };
        let toggle = |id: &'static str, ic: &'static str, label: &'static str, on: bool, a: BlockAct| plain(id, ic, label, a).child(switch(on));
        let m = popover()
            .id("block-menu")
            .occlude()
            .w(px(MENU_W))
            .p(px(8.))
            .flex()
            .flex_col()
            .child(row("bm-indent", "indent-align", crate::settings::text("缩进"), Some("›"), sub == Some(BlockSub::Indent)).on_mouse_move(set_sub(Some(BlockSub::Indent))))
            .child(separator_h().on_mouse_move(set_sub(None)))
            .child(plain("bm-cut", "scissors", crate::settings::text("剪切"), BlockAct::Cut))
            .child(plain("bm-copy", "copy", crate::settings::text("复制"), BlockAct::Copy))
            .child(plain("bm-del", "trash-2", crate::settings::text("删除"), BlockAct::Delete))
            .child(separator_h().on_mouse_move(set_sub(None)))
            .child(toggle("bm-hrow", "header-row", crate::settings::text("标题行"), hrow, BlockAct::HeaderRow))
            .child(toggle("bm-hcol", "header-col", crate::settings::text("标题列"), hcol, BlockAct::HeaderCol))
            .child(plain("bm-dist", "distribute-cols", crate::settings::text("均分列宽"), BlockAct::DistributeCols))
            .child(separator_h().on_mouse_move(set_sub(None)))
            .child(add_below_row(matches!(sub, Some(BlockSub::Add(_))), cx));
        let sub_el = (sub == Some(BlockSub::Indent)).then(|| {
            let indent_row = |id: &'static str, ic: &'static str, label: &'static str, keys: Option<&'static str>, enabled: bool, a: BlockAct| {
                if enabled {
                    let r = row(id, ic, label, None, false).on_mouse_down(MouseButton::Left, act(a));
                    match keys {
                        Some(k) => r.tip_right(label, Some(k)),
                        None => r,
                    }
                } else {
                    let d = Theme::text_disabled();
                    row_tinted(id, ic, label, None, false, d, d).on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                }
            };
            popover()
                .id("bm-indent-sub")
                .occlude()
                .absolute()
                .left(px(MENU_W + 4.))
                .top_0()
                .w(px(MENU_W))
                .p(px(8.))
                .child(indent_row("in-inc", "indent-increase", crate::settings::text("增加缩进"), None, self.core.can_indent(block), BlockAct::Indent))
                .child(indent_row("in-dec", "indent-decrease", crate::settings::text("减少缩进"), Some("Shift + Tab"), self.core.can_outdent(block), BlockAct::Outdent))
                .into_any_element()
        });
        let el = div()
            .relative()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(m)
            .children(sub_el);
        Some(at(self.rel(point(r.left(), r.bottom() + px(4.))), Corner::TopLeft, el))
    }
}
