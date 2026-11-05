//! 表格 (table blocks) on screen: the row and column bars with their insert
//! pins, column resizing and moving lines. The operations themselves are
//! `table_ops.rs`.

pub use super::table_ops::{CellSel, CellSelKind, first_row_text};
use super::core::Selection;
use super::render::{Env, R};
use super::view::Editor;
use crate::model::*;
use crate::editor_theme::{self as theme, Theme};
use crate::editor_ui::TipExt;
use gpui::{prelude::FluentBuilder, *};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

/// Width of the row bar / height of the column bar.
const BAR: f32 = 10.;
/// Outer corner radius of the cell grid.
/// `top: 26px` for the pinned header row.
const STICKY_TOP: f32 = 26.;

/// Window bounds of every bar segment (last frame), keyed by (table,
/// column bar?, index).
pub type Geo = Rc<RefCell<HashMap<(BlockId, bool, usize), Bounds<Pixels>>>>;

#[derive(Default)]
pub struct TableUi {
    /// The hovered insert point: (table, between columns?, boundary).
    pub insert: Option<(BlockId, bool, usize)>,
    pub drag: Option<Drag>,
    /// The pointer is on the toolbar's delete button: what would go turns red.
    pub danger: bool,
    pub geo: Geo,
    /// Header rows pinned below the page top this frame, by table.
    pub stuck: Stuck,
    /// When a moving drag last scrolled the page (it runs while the pointer
    /// stays near the page's top or bottom edge).
    pub scrolled: Option<Instant>,
}

pub type Stuck = Rc<RefCell<HashMap<BlockId, Bounds<Pixels>>>>;

pub enum Drag {
    /// Dragging the right border of column `col`.
    Resize { table: BlockId, col: usize, x0: f32, w0: f32, live: f32 },
    /// Extending a row / column selection along its bar.
    Bars { table: BlockId, cols: bool },
    /// Moving the selected lines `a..=b`, pressed on line `line`. It starts
    /// once the pointer is more than 2px from `from`; a press that never
    /// moves selects just `line`.
    Move { table: BlockId, cols: bool, line: usize, a: usize, b: usize, from: Point<Pixels>, at: Point<Pixels>, started: bool, drop: Option<TableDrop> },
}

/// Where moving lines would go: before line boundary `to`, with the 2px
/// indicator `off` px from the grid's left (columns) or top (rows).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TableDrop {
    pub to: usize,
    pub off: Pixels,
}

impl Drag {
    pub fn table(&self) -> BlockId {
        match self {
            Drag::Resize { table, .. } | Drag::Bars { table, .. } | Drag::Move { table, .. } => *table,
        }
    }
}

impl Editor {
    /// Column widths a table is drawn with (live while one is dragged).
    pub(super) fn table_widths(&self, id: BlockId, t: &Table) -> Vec<f32> {
        let mut w = t.col_widths.clone();
        if let Some(Drag::Resize { table, col, live, .. }) = &self.tables.drag
            && *table == id
            && let Some(x) = w.get_mut(*col)
        {
            *x = *live;
        }
        w
    }

    /// A table drawn with a live width, re-measured every frame it changes.
    pub(super) fn table_override(&self) -> Option<(BlockId, u64)> {
        match &self.tables.drag {
            Some(Drag::Resize { table, live, .. }) => Some((*table, live.to_bits() as u64 | 1)),
            _ => None,
        }
    }

    /// The grid slot under `p`: `clamp` maps points outside the table to the
    /// nearest edge slot instead of None.
    fn slot_at(&self, table: BlockId, p: Point<Pixels>, clamp: bool) -> Option<(usize, usize)> {
        let geo = self.tables.geo.borrow();
        let find = |cols: bool, v: Pixels| -> Option<usize> {
            let span = |b: &Bounds<Pixels>| if cols { (b.left(), b.right()) } else { (b.top(), b.bottom()) };
            let segs: Vec<(Pixels, Pixels)> = (0..).map_while(|i| geo.get(&(table, cols, i)).map(span)).collect();
            if let Some(i) = segs.iter().position(|&(a, z)| v >= a && v < z) {
                return Some(i);
            }
            if !clamp || segs.is_empty() {
                return None;
            }
            Some(if v < segs[0].0 { 0 } else { segs.len() - 1 })
        };
        Some((find(false, p.y)?, find(true, p.x)?))
    }

    /// A text drag that reaches another cell of its table selects cells
    /// instead; back in the first cell it is a text selection again. True
    /// while cells are selected.
    pub(super) fn cell_drag_to(&mut self, p: Point<Pixels>, cx: &mut Context<Self>) -> bool {
        let Some((table, ar, ac)) = self.core.cell_of(self.core.sel.anchor.block) else { return false };
        let ours = self.core.cell_sel.is_some_and(|s| s.table == table);
        let Some((r, c)) = self.slot_at(table, p, ours) else { return false };
        let Some(t) = self.core.table(table) else { return false };
        let head = t.origin(r, c);
        if head == (ar, ac) {
            if ours {
                self.core.cell_sel = None;
                cx.notify();
            }
            return false;
        }
        let s = CellSel { table, anchor: (ar, ac), head, kind: CellSelKind::Range };
        if self.core.cell_sel != Some(s) {
            self.core.sel = Selection::caret(self.core.sel.anchor);
            self.core.block_sel = None;
            self.core.cell_sel = Some(s);
            self.menu = None;
            cx.notify();
        }
        true
    }

    /// Mouse down on segment `i` of a bar: select that row / column (shift
    /// extends) and follow the pointer along the bar. On an already selected
    /// segment it picks the selected lines up to move them instead.
    fn bar_down(&mut self, table: BlockId, cols: bool, i: usize, p: Point<Pixels>, shift: bool, window: &mut Window, cx: &mut Context<Self>) {
        window.focus(&self.focus);
        self.menu = None;
        self.slash = None;
        self.toolbar_hidden = false;
        self.tables.danger = false;
        let kind = if cols { CellSelKind::Cols } else { CellSelKind::Rows };
        let head = if cols { (0, i) } else { (i, 0) };
        if !shift
            && self.core.cell_sel.is_some_and(|s| s.kind == kind)
            && let Some((t, r)) = self.core.cell_range()
            && t == table
        {
            let (a, b) = if cols { (r.left, r.right) } else { (r.top, r.bottom) };
            if (a..=b).contains(&i) {
                self.tables.drag = Some(Drag::Move { table, cols, line: i, a, b, from: p, at: p, started: false, drop: None });
                cx.notify();
                return;
            }
        }
        let anchor = match self.core.cell_sel {
            Some(s) if shift && s.table == table && s.kind == kind => s.anchor,
            _ => head,
        };
        self.core.select_cells(CellSel { table, anchor, head, kind });
        self.tables.drag = Some(Drag::Bars { table, cols });
        cx.notify();
    }

    pub(super) fn table_drag_move(&mut self, p: Point<Pixels>, pressed: bool, window: &mut Window, cx: &mut Context<Self>) {
        if !pressed {
            return self.end_table_drag(window, cx);
        }
        if let Some(Drag::Resize { x0, w0, live, .. }) = &mut self.tables.drag {
            *live = (*w0 + f32::from(p.x) - *x0).max(Table::MIN_WIDTH);
            cx.notify();
        } else if let Some(Drag::Bars { table, cols }) = self.tables.drag
            && let Some((r, c)) = self.slot_at(table, p, true)
            && let Some(s) = &mut self.core.cell_sel
            && s.table == table
        {
            let head = if cols { (0, c) } else { (r, 0) };
            if s.head != head {
                s.head = head;
                cx.notify();
            }
        } else if let Some(Drag::Move { from, at, started, .. }) = &mut self.tables.drag {
            *at = p;
            if !*started && ((p.x - from.x).abs() > px(2.) || (p.y - from.y).abs() > px(2.)) {
                *started = true;
                self.toolbar_hidden = true;
            }
            self.update_table_drop();
            cx.notify();
        }
    }

    fn update_table_drop(&mut self) {
        let Some(Drag::Move { table, cols, a, b, at, started: true, .. }) = self.tables.drag else { return };
        let d = self.table_drop_at(table, cols, a, b, at);
        if let Some(Drag::Move { drop, .. }) = &mut self.tables.drag {
            *drop = d;
        }
    }

    /// Where lines `a..=b` would go with the pointer at `p`, from last
    /// frame's bar segments.
    fn table_drop_at(&self, table: BlockId, cols: bool, a: usize, b: usize, p: Point<Pixels>) -> Option<TableDrop> {
        let hot = px(20.);
        let t = self.core.table(table)?;
        let geo = self.tables.geo.borrow();
        let segs = |c: bool| (0..).map_while(|i| geo.get(&(table, c, i)).copied()).collect::<Vec<_>>();
        let (cs, rs) = (segs(true), segs(false));
        let (c0, r0) = (*cs.first()?, *rs.first()?);
        let (left, right, top, bottom) = (c0.left(), cs.last()?.right(), r0.top(), rs.last()?.bottom());
        let view = self.list.viewport_bounds();
        if p.x < left - hot || p.x > right.min(view.right()) || p.y < top - hot || p.y > bottom + hot {
            return None;
        }
        let lines = if cols { &cs } else { &rs };
        let n = lines.len();
        let edge = |m: usize, after: bool| {
            let s = lines[m];
            let off = match (cols, after) {
                (true, false) => s.left() - c0.left() + px(BAR - 1.),
                (true, true) => s.right() - c0.left() + px(BAR),
                (false, false) => s.top() - r0.top() + px(BAR - 1.),
                (false, true) => s.bottom() - r0.top() + px(BAR),
            };
            TableDrop { to: m + after as usize, off }
        };
        // Next to the moving lines themselves: no move.
        let stays = |d: &TableDrop| (a..=b + 1).contains(&d.to);
        // Within 20px of the table's start / end edge: the first / last line
        // (the right edge counts from 20px inside the table).
        let (v, lo, hi) = if cols { (p.x, left, right - hot) } else { (p.y, top, bottom) };
        let zone = if (v - lo).abs() <= hot {
            Some(edge(0, false))
        } else if (hi - v).abs() <= hot {
            Some(edge(n - 1, true))
        } else {
            None
        };
        if let Some(d) = zone.filter(|d| !stays(d)) {
            return Some(d);
        }
        // Otherwise within 20px of a side of the cell under the pointer; up
        // to 26px above / left of the table counts as its first row / column.
        let fix = |v: Pixels, o: Pixels| if v < o && v >= o - px(26.) { o } else { v };
        let (x, y) = (fix(p.x, left), fix(p.y, top));
        let r = rs.iter().position(|s| y >= s.top() && y < s.bottom())?;
        let c = cs.iter().position(|s| x >= s.left() && x < s.right())?;
        let (or, oc) = t.origin(r, c);
        let cell = &t.rows[or][oc];
        let (i, span) = if cols { (oc, cell.colspan) } else { (or, cell.rowspan) };
        let last = (i + span.max(1) as usize - 1).min(n - 1);
        let (lo, hi) = if cols { (lines[i].left(), lines[last].right()) } else { (lines[i].top(), lines[last].bottom()) };
        let d = if (v - lo).abs() <= hot {
            edge(i, false)
        } else if (hi - v).abs() <= hot {
            edge(last, true)
        } else {
            return None;
        };
        // `checkCanDrop`: lines can't go into the middle of a merged cell.
        let e = d.to;
        let across = if cols { t.rows.len() } else { t.ncols() };
        let splits = e > 0 && e < n && (0..across).any(|k| if cols { t.origin(k, e).1 != e } else { t.origin(e, k).0 != e });
        (!splits && !stays(&d)).then_some(d)
    }

    /// While lines are moved with the pointer within 20px of the page's top
    /// or bottom edge, the page scrolls 6px every 10ms.
    pub(super) fn table_autoscroll(&mut self, window: &mut Window) {
        self.update_table_drop();
        let dir = match &self.tables.drag {
            Some(Drag::Move { started: true, at, .. }) => {
                let v = self.list.viewport_bounds();
                if at.y < v.top() + px(20.) {
                    -1.
                } else if at.y > v.bottom() - px(20.) {
                    1.
                } else {
                    0.
                }
            }
            _ => 0.,
        };
        if dir == 0. {
            self.tables.scrolled = None;
            return;
        }
        let now = Instant::now();
        let dt = self.tables.scrolled.map_or(0., |t| (now - t).as_secs_f32().min(0.05));
        self.tables.scrolled = Some(now);
        self.list.scroll_by(px(dir * 600. * dt));
        window.request_animation_frame();
    }

    pub(super) fn end_table_drag(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(d) = self.tables.drag.take() else { return };
        self.tables.scrolled = None;
        match d {
            Drag::Resize { table, col, live, .. } if self.core.table_set_width(table, col, live) => self.changed(window, cx),
            Drag::Move { started: true, drop, .. } => {
                self.toolbar_hidden = false;
                if let Some(d) = drop
                    && self.core.table_move_selected(d.to)
                {
                    self.changed(window, cx);
                }
            }
            // A click on a selected segment selects just its line.
            Drag::Move { table, cols, line, .. } => {
                let head = if cols { (0, line) } else { (line, 0) };
                let kind = if cols { CellSelKind::Cols } else { CellSelKind::Rows };
                self.core.select_cells(CellSel { table, anchor: head, head, kind });
            }
            _ => {}
        }
        cx.notify();
    }

    /// While a column is resized, a layer over the page keeps the cursor;
    /// moving lines, it also carries the 「共 N 行」 snapshot at the pointer.
    pub(super) fn table_drag_overlay(&self) -> Option<AnyElement> {
        let layer = |c: CursorStyle| div().id("table-drag-overlay").absolute().inset_0().occlude().cursor(c);
        let el = match &self.tables.drag {
            Some(Drag::Resize { .. }) => layer(CursorStyle::ResizeLeftRight),
            &Some(Drag::Move { cols, a, b, at, started: true, .. }) => {
                let o = self.content_origin.get();
                layer(CursorStyle::ClosedHand).child(move_snapshot(cols, b - a + 1).left(at.x - o.x).top(at.y - o.y))
            }
            _ => return None,
        };
        Some(deferred(el).with_priority(4).into_any_element())
    }

    fn table_insert_at(&mut self, table: BlockId, cols: bool, at: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.core.block_sel = None;
        self.core.table_insert(table, cols, at);
        self.tables.insert = None;
        window.focus(&self.focus);
        self.changed(window, cx);
    }
}

/// The snapshot under the pointer while lines move: a primary bar with a
/// 「共 N 行 / 列」 label beside (rows) or below (columns) it.
fn move_snapshot(cols: bool, n: usize) -> Div {
    let label = div()
        .flex()
        .justify_center()
        .px(px(10.))
        .text_size(px(12.))
        .text_color(Theme::text())
        .opacity(0.8)
        .bg(Theme::primary_fill_1())
        .border_color(Theme::primary())
        .border_r_1()
        .border_b_1()
        .rounded_br(px(2.))
        .child(crate::localized_format!("共 {n} {}", "{n} {}", if cols { crate::settings::text("列") } else { crate::settings::text("行") }));
    let bar = div().bg(Theme::primary()).rounded_tl(px(2.));
    if cols {
        div()
            .absolute()
            .flex()
            .flex_col()
            .child(bar.absolute().top(px(-BAR)).left_0().w_full().min_w(px(50.)).h(px(BAR)).rounded_tr(px(2.)))
            .child(label.min_w(px(50.)).h(px(80.)).line_height(px(70.)).border_l_1().rounded_bl(px(2.)))
    } else {
        div()
            .absolute()
            .flex()
            .child(bar.w(px(BAR)).h(px(40.)).rounded_bl(px(2.)))
            .child(label.min_w(px(100.)).h(px(40.)).line_height(px(40.)).border_t_1().rounded_tr(px(2.)))
    }
}

/// Places a child of a row-bar segment `top` px below its top edge, or, for
/// the boundary under the last row, the same distance from its bottom edge.
fn place(d: Stateful<Div>, top: f32, h: f32, at_bottom: bool) -> Stateful<Div> {
    if at_bottom { d.bottom(px(-(top + h))).h(px(h)) } else { d.top(px(top)).h(px(h)) }
}

fn dot() -> Div {
    div().absolute().size(px(3.)).rounded_full().bg(Theme::block_placeholder())
}

/// Shared by the pinned parts of one table during a frame.
#[derive(Default)]
struct StickyState {
    grid: Cell<Bounds<Pixels>>,
    dy: Cell<Pixels>,
}

/// Moves its child down by however far the header row has to go to stay
/// `STICKY_TOP` below the visible top, up to the table's bottom. The lead
/// (first header cell) works the offset out, records the pinned row and
/// paints the page-coloured band above it (from 10.5px above the table);
/// the rest follow it.
struct Sticky {
    child: AnyElement,
    st: Rc<StickyState>,
    lead: Option<(BlockId, Stuck)>,
}

impl IntoElement for Sticky {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}

impl Element for Sticky {
    type RequestLayoutState = ();
    type PrepaintState = Option<Bounds<Pixels>>;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(&mut self, _: Option<&GlobalElementId>, _: Option<&InspectorElementId>, window: &mut Window, cx: &mut App) -> (LayoutId, ()) {
        (self.child.request_layout(window, cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Bounds<Pixels>> {
        let mut mask = None;
        if let Some((table, stuck)) = &self.lead {
            let g = self.st.grid.get();
            let max = (g.bottom() - bounds.bottom()).max(px(0.));
            let dy = (window.content_mask().bounds.top() + px(STICKY_TOP) - bounds.top()).clamp(px(0.), max);
            self.st.dy.set(dy);
            if dy > px(0.) {
                let top = bounds.top() + dy;
                stuck.borrow_mut().insert(*table, Bounds::from_corners(point(g.left() + px(BAR), top), point(g.right(), top + bounds.size.height)));
                mask = Some(Bounds::from_corners(point(g.left() - px(16.), (top - px(STICKY_TOP + 0.5)).max(bounds.top() - px(10.5))), point(g.right(), top)));
            }
        }
        window.with_element_offset(point(px(0.), self.st.dy.get()), |window| self.child.prepaint(window, cx));
        mask
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        mask: &mut Option<Bounds<Pixels>>,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(m) = *mask {
            window.paint_quad(fill(m, Theme::bg_body()));
        }
        self.child.paint(window, cx);
    }
}

impl R<'_> {
    pub(super) fn table(&self, b: &Block, t: &Table, cx: &mut Context<Editor>) -> AnyElement {
        let ed = self.ed;
        let id = b.id;
        let env = Env { depth: 0, cell: true, color: None, in_callout: false, root: false, column: false, quote: false, width: 0. };
        let widths = ed.table_widths(id, t);
        let n = widths.len();
        let nrows = t.rows.len();
        let total: f32 = widths.iter().sum();
        let sel = ed.core.cell_sel.filter(|s| s.table == id);
        let range = sel.map(|s| t.expand(s.raw(t)));
        let bar_sel = sel.map(|s| s.kind).filter(|k| *k != CellSelKind::Range);
        let danger = ed.tables.danger && bar_sel.is_some();
        let insert = ed.tables.insert.filter(|i| i.0 == id);
        let busy = ed.tables.drag.as_ref().is_some_and(|d| d.table() == id);
        let moving = match &ed.tables.drag {
            Some(Drag::Move { table, cols, started: true, drop, .. }) if *table == id => Some((*cols, *drop)),
            _ => None,
        };
        let caret_in = !ed.core.in_block_mode() && ed.core.cell_of(ed.core.sel.head.block).is_some_and(|c| c.0 == id);
        // The whole table selected as a block (or inside a text range).
        let whole = self.selected.contains(&id)
            || (!ed.core.in_block_mode() && !ed.core.sel.is_collapsed() && ed.core.atomic_selected(id, &self.units));
        let active = !ed.read_only && (whole || (self.focused && caret_in) || sel.is_some() || busy || insert.is_some() || (ed.hover == Some(id) && !ed.dragging));
        let key = |kind: u64, i: usize| id.0.wrapping_mul(8192).wrapping_add(kind * 2048 + i as u64);
        let x_of = |c: usize| widths[..c].iter().sum::<f32>();

        let mut cols_tpl = vec![BAR];
        cols_tpl.extend(&widths);
        let mut grid = div().relative().grid().grid_col_widths(cols_tpl).w(px(BAR + total));

        // A header row whose cells all end in it stays in view while the
        // table scrolls under it; painted after the body so it covers it.
        let sticky = t.header_rows > 0 && t.cells().all(|(r, _, c)| r > 0 || c.rowspan <= 1);
        let st = Rc::new(StickyState::default());
        let pin_el = |el: AnyElement, lead: bool| Sticky { child: el, st: st.clone(), lead: lead.then(|| (id, ed.tables.stuck.clone())) }.into_any_element();
        let mut head: Vec<AnyElement> = vec![];
        if sticky {
            let st = st.clone();
            grid = grid.child(canvas(move |b, _, _| st.grid.set(b), |_, _, _, _| {}).absolute().top_0().left_0().size_full());
        }

        for (r, c, cell) in t.cells() {
            let (rs, cs) = (cell.rowspan.max(1) as usize, cell.colspan.max(1) as usize);
            let w: f32 = widths[c..(c + cs).min(n)].iter().sum();
            let header = r < t.header_rows || (t.header_col && c == 0);
            let selected = range.is_some_and(|g| g.contains(r, c));
            let bg = if selected && danger {
                Some(Theme::danger_cell())
            } else {
                cell.bg.as_ref().and_then(|x| theme::block_bg(&x.0)).or(header.then(Theme::table_header))
            };
            let table_r = if crate::settings::feishu() { 0. } else { theme::style().num("table.radius") };
            let el = div()
                    .relative()
                    // (`row_span` / `col_span` would reset the start line.)
                    .row_start(r as i16 + 2)
                    .row_end((r + rs) as i16 + 2)
                    .col_start(c as i16 + 2)
                    .col_end((c + cs) as i16 + 2)
                    .w(px(w))
                    .flex()
                    .flex_col()
                    .map(|d| match cell.valign.as_deref() {
                        Some("middle") => d.justify_center(),
                        Some("bottom") => d.justify_end(),
                        _ => d,
                    })
                    // The table: 14 / 20 text, 9 / 10 padding, hairline grid,
                    // the outer corners rounded.
                    .px(px(if crate::settings::feishu() { 8. } else { 10. }))
                    .pt(px(if crate::settings::feishu() { 8. } else { 9. })).pb(px(9.))
                    .when(r == 0, |d| d.border_t_1())
                    .when(c == 0, |d| d.border_l_1())
                    .border_r_1()
                    .border_b_1()
                    .border_color(Theme::table_line())
                    .when(r == 0 && c == 0, |d| d.rounded_tl(px(table_r)))
                    .when(r == 0 && c + cs >= n, |d| d.rounded_tr(px(table_r)))
                    .when(r + rs >= t.rows.len() && c == 0, |d| d.rounded_bl(px(table_r)))
                    .when(r + rs >= t.rows.len() && c + cs >= n, |d| d.rounded_br(px(table_r)))
                    .when_some(bg, |d, c| d.bg(c))
                    .when(selected && moving.is_some(), |d| d.opacity(0.8))
                    .child(self.blocks(&cell.blocks, Env { width: w - if crate::settings::feishu() { 17. } else { 21. }, ..env }, cx))
                    .when(selected && !danger, |d| d.child(div().absolute().inset_0().bg(Theme::selection())));
            if sticky && r == 0 {
                head.push(pin_el(el.into_any_element(), head.is_empty()));
            } else {
                grid = grid.child(el);
            }
        }

        let geo = |cols: bool, i: usize| {
            let g = ed.tables.geo.clone();
            canvas(
                move |bounds, _, _| {
                    g.borrow_mut().insert((id, cols, i), bounds);
                },
                |_, _, _, _| {},
            )
            .absolute()
            .top_0()
            .left_0()
            .size_full()
        };
        let set_insert = move |cols: bool, k: usize| {
            move |e: &mut Editor, hovered: &bool, _: &mut Window, cx: &mut Context<Editor>| {
                let me = (id, cols, k);
                if *hovered {
                    e.tables.insert = Some(me);
                } else if e.tables.insert == Some(me) {
                    e.tables.insert = None;
                } else {
                    return;
                }
                cx.notify();
            }
        };
        let do_insert = move |cols: bool, k: usize| {
            move |e: &mut Editor, _: &MouseDownEvent, window: &mut Window, cx: &mut Context<Editor>| {
                cx.stop_propagation();
                e.table_insert_at(id, cols, k, window, cx);
            }
        };
        // The pin grows from its centre (scale 0 → 1, 0.2s).
        let pin = |src: &'static str, w: f32, h: f32, k: u64| {
            img(src).absolute().with_animation(("table-pin", k), Animation::new(Duration::from_millis(200)), move |im, t| {
                im.left(px(w / 2. * (1. - t))).top(px(h / 2. * (1. - t))).w(px(w * t)).h(px(h * t))
            })
        };
        let segment_color = |on: bool| {
            if whole {
                Theme::table_bar_selected()
            } else if on && danger {
                Theme::danger_bar()
            } else if on {
                Theme::primary()
            } else {
                Theme::handle_active()
            }
        };

        // Column bar: one segment per column; insert points on its boundaries.
        let mut col_segs: Vec<AnyElement> = vec![];
        for c in 0..n {
            let on = bar_sel == Some(CellSelKind::Cols) && range.is_some_and(|g| (g.left..=g.right).contains(&c));
            let mut seg = div()
                .id(("tcol", key(0, c)))
                .relative()
                .row_start(1)
                .col_start(c as i16 + 2)
                .h(px(BAR))
                .child(geo(true, c));
            if active {
                seg = seg
                    .when(c == 0, |d| d.rounded_tl(px(4.)))
                    .when(c + 1 == n, |d| d.rounded_tr(px(4.)))
                    .bg(segment_color(on))
                    .when(!on, |d| d.hover(|s| s.bg(Theme::table_bar_hover())))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |e, ev: &MouseDownEvent, window, cx| {
                            cx.stop_propagation();
                            e.bar_down(id, true, c, ev.position, ev.modifiers.shift, window, cx);
                        }),
                    );
                let ks: &[usize] = if c + 1 == n { &[c, c + 1] } else { &[c] };
                for &k in ks {
                    let bx = if k == c { 0. } else { widths[c] };
                    let hovered = insert == Some((id, true, k));
                    let dl = if k == 0 { 0.5 } else { bx - 2.5 };
                    let zone = div().id(("tcol-ins", key(1, k))).absolute().cursor_pointer();
                    // Dot zone 15 × 16 over T−24..T−8 (the segment top is T−10);
                    // hovered, it grows to the 22 × 24 pin at T−25.4.
                    let zone = if hovered {
                        zone.left(px(bx - 11.)).top(px(-15.4)).w(px(22.)).h(px(24.)).child(pin("editor-icons/column-add.svg", 22., 24., key(1, k)))
                    } else {
                        zone.left(px(dl - 6.)).top(px(-14.)).w(px(15.)).h(px(16.)).child(dot().left(px(6.)).top(px(6.6)))
                    };
                    seg = seg.child(
                        zone.tip_above(crate::settings::text("插入列"), None)
                            .on_hover(cx.listener(set_insert(true, k)))
                            .on_mouse_down(MouseButton::Left, cx.listener(do_insert(true, k))),
                    );
                }
            }
            col_segs.push(if sticky { pin_el(seg.into_any_element(), false) } else { seg.into_any_element() });
        }

        // Row bar.
        for r in 0..nrows {
            let on = bar_sel == Some(CellSelKind::Rows) && range.is_some_and(|g| (g.top..=g.bottom).contains(&r));
            let mut seg = div().id(("trow", key(2, r))).relative().col_start(1).row_start(r as i16 + 2).child(geo(false, r));
            if active {
                seg = seg
                    .when(r == 0, |d| d.rounded_tl(px(4.)))
                    .when(r + 1 == nrows, |d| d.rounded_bl(px(4.)))
                    .bg(segment_color(on))
                    .when(!on, |d| d.hover(|s| s.bg(Theme::table_bar_hover())))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |e, ev: &MouseDownEvent, window, cx| {
                            cx.stop_propagation();
                            e.bar_down(id, false, r, ev.position, ev.modifiers.shift, window, cx);
                        }),
                    );
                let ks: &[usize] = if r + 1 == nrows { &[r, r + 1] } else { &[r] };
                for &k in ks {
                    let below = k > r;
                    let hovered = insert == Some((id, false, k));
                    let zone = div().id(("trow-ins", key(3, k))).absolute().cursor_pointer();
                    // Dot zone 19 × 15 centred on the dot at L−17.5; hovered, the
                    // 24 × 22 pin at L−25.5.
                    let zone = if hovered {
                        place(zone.left(px(-15.5)).w(px(24.)), -11.5, 22., below).child(pin("editor-icons/row-add.svg", 24., 22., key(3, k)))
                    } else {
                        place(zone.left(px(-15.5)).w(px(19.)), -8., 15., below).child(dot().left(px(8.)).top(px(6.)))
                    };
                    seg = seg.child(
                        zone.tip_above(crate::settings::text("插入行"), None)
                            .on_hover(cx.listener(set_insert(false, k)))
                            .on_mouse_down(MouseButton::Left, cx.listener(do_insert(false, k))),
                    );
                    if hovered {
                        seg = seg.child(place(div().id(("trow-line", key(4, k))).absolute().left(px(BAR + 1.)).w(px(total - 1.)), -1., 2., below).bg(Theme::primary_500()));
                    }
                }
            }
            grid = grid.child(seg);
        }
        grid = grid.children(head).children(col_segs);

        // The blue line of a hovered column insert point.
        if let Some((_, true, k)) = insert {
            grid = grid.child(div().absolute().left(px(BAR + x_of(k.min(n)) - 1.)).top(px(BAR + 1.)).bottom_0().w(px(2.)).bg(Theme::primary_500()));
        }

        if whole {
            grid = grid.child(div().absolute().left(px(BAR)).top(px(BAR)).right_0().bottom_0().bg(Theme::block_hover()));
        }

        // Column resizers: 10px hot zones whose right edge is 3px past each border.
        let resizing = match &ed.tables.drag {
            Some(Drag::Resize { table, col, .. }) if *table == id => Some(*col),
            _ => None,
        };
        for c in 0..n {
            let bx = BAR + x_of(c + 1);
            let group = SharedString::from(format!("tres-{}", key(5, c)));
            let w0 = widths[c];
            grid = grid.child(
                div()
                    .id(("tres", key(5, c)))
                    .absolute()
                    .left(px(bx - 7.))
                    .top(px(BAR))
                    .bottom_0()
                    .w(px(10.))
                    .group(group.clone())
                    .cursor(CursorStyle::ResizeLeftRight)
                    .child(
                        div()
                            .ml(px(5.))
                            .w(px(2.))
                            .h_full()
                            .when(resizing == Some(c), |d| d.bg(Theme::primary_500()))
                            .when(resizing.is_none() && !ed.dragging, |d| d.group_hover(group, |s| s.bg(Theme::primary_500()))),
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |e, ev: &MouseDownEvent, _, cx| {
                            cx.stop_propagation();
                            e.tables.drag = Some(Drag::Resize { table: id, col: c, x0: f32::from(ev.position.x), w0, live: w0 });
                            cx.notify();
                        }),
                    ),
            );
        }

        // Where moving lines would go: a 2px line from the bar's outer edge.
        if let Some((cols, Some(d))) = moving {
            let line = div().absolute().bg(Theme::primary_500());
            grid = grid.child(if cols { line.left(d.off).top_0().bottom_0().w(px(2.)) } else { line.top(d.off).left_0().right_0().h(px(2.)) });
        }

        // The scroller's padding leaves room for the bars and pins (its clip
        // would cut them off); negative margins keep the block's bounds on
        // the table itself.
        div()
            .flex()
            .child(
                div()
                    .id(("table", id.0))
                    .flex_1()
                    .min_w_0()
                    .ml(px(-26.))
                    .mt(px(-26.))
                    .pl(px(16.))
                    .pt(px(16.))
                    .mr(px(-12.))
                    .pr(px(12.))
                    .mb(px(-12.))
                    .pb(px(12.))
                    .overflow_x_scroll()
                    .child(grid),
            )
            .into_any_element()
    }
}
