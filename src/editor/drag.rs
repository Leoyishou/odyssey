//! Moving blocks by their handle: press the handle,
//! move past 2px and the blocks fade while a card of them follows the
//! pointer; an accent line shows where they land.

use super::core::{BlockSel, DropPlace, block_at, siblings_at};
use super::render::{Env, R, TOP, flattens, margins, visible_with_gaps};
use super::view::Editor;
use crate::model::*;
use crate::feishu_theme::{self as theme, Theme};
use gpui::{prelude::FluentBuilder, *};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::time::{Duration, Instant};

/// Nesting deeper than this is refused (`pE=16`).
const MAX_DEPTH: usize = 16;
/// Children sit this far right of their parent.
const INDENT: f32 = 25.;
const FLASH: Duration = Duration::from_millis(2000);

pub struct BlockDrag {
    /// The moved blocks, topmost only, in document order.
    pub ids: Vec<BlockId>,
    /// A multi-block selection the press started on (the handle menu narrows
    /// the selection to its block; dragging moves them all).
    sel: Option<BlockSel>,
    from: Point<Pixels>,
    at: Point<Pixels>,
    pub started: bool,
    drop: Option<BlockDrop>,
    scrolled: Option<Instant>,
    /// Size of the moved blocks on the page when the drag started.
    size: Size<Pixels>,
    /// The window was active at the press (losing focus cancels).
    active: Option<bool>,
}

#[derive(Clone, Debug)]
struct BlockDrop {
    target: BlockId,
    place: DropPlace,
    /// The 2px line (window coordinates).
    line: Bounds<Pixels>,
    /// Lighter 25px segments: left of the line (levels the drop can still
    /// move out to), or over its start when nesting.
    hints: usize,
    nest: bool,
    /// The container the blocks land in, tinted.
    area: Option<Bounds<Pixels>>,
}

/// A drop found by walking the page under the pointer.
struct Hit {
    target: BlockId,
    place: DropPlace,
    /// Right edge of the container (None: the target's own).
    right: Option<Pixels>,
}

struct Walk<'a> {
    bb: &'a HashMap<BlockId, Bounds<Pixels>>,
    ids: &'a HashSet<BlockId>,
    p: Point<Pixels>,
}

impl Walk<'_> {
    /// The block's rows: its own and, when its children are rows of their
    /// own, theirs.
    fn span(&self, b: &Block) -> Option<Bounds<Pixels>> {
        let own = self.bb.get(&b.id).copied();
        if !flattens(b) {
            return own;
        }
        visible_with_gaps(&b.children, TOP)
            .into_iter()
            .filter_map(|(i, _)| self.span(&b.children[i]))
            .fold(own, |a, s| Some(a.map_or(s, |a| a.union(&s))))
    }

    fn nestable(b: &Block) -> bool {
        match b.kind {
            BlockKind::Bullet | BlockKind::Ordered { .. } | BlockKind::Todo { .. } => !b.folded,
            BlockKind::Text | BlockKind::Heading { .. } => !b.children.is_empty() && !b.folded,
            _ => false,
        }
    }

    fn level(&self, list: &[Block], right: Option<Pixels>) -> Option<Hit> {
        let (x, y) = (self.p.x, self.p.y);
        let vis: Vec<(&Block, Bounds<Pixels>)> =
            visible_with_gaps(list, TOP).into_iter().filter_map(|(i, _)| Some((&list[i], self.span(&list[i])?))).collect();
        let i = vis.iter().position(|(_, s)| s.bottom() >= y).unwrap_or(vis.len().checked_sub(1)?);
        let (b, span) = vis[i];
        let dragged = self.ids.contains(&b.id);
        if dragged && y >= span.top() {
            return None;
        }
        let own = *self.bb.get(&b.id)?;
        let hit = |target: BlockId, place| Some(Hit { target, place, right });
        let inside = y >= own.top() && y <= own.bottom();
        match &b.kind {
            BlockKind::Grid if inside => {
                let cols: Vec<(&Block, Bounds<Pixels>)> = b.children.iter().filter_map(|c| Some((c, *self.bb.get(&c.id)?))).collect();
                let (col, cb) = cols
                    .iter()
                    .min_by(|a, b| dist_x(x, &a.1).total_cmp(&dist_x(x, &b.1)))
                    .copied()?;
                return self.level(&col.children, Some(cb.right()));
            }
            BlockKind::Callout { .. } | BlockKind::Quote if inside => {
                let kids = visible_with_gaps(&b.children, TOP);
                let first = kids.first().and_then(|(i, _)| self.span(&b.children[*i]));
                let last = kids.last().and_then(|(i, _)| self.span(&b.children[*i]));
                if let (Some(f), Some(l)) = (first, last)
                    && y >= f.top()
                    && y <= l.bottom()
                {
                    let inset = if matches!(b.kind, BlockKind::Quote) { 0. } else { 16. };
                    return self.level(&b.children, Some(own.right() - px(inset)));
                }
                // The padding above and below the children: beside the box.
                return if y < own.center().y { self.before(&vis, i, right) } else { hit(b.id, DropPlace::After) };
            }
            _ => {}
        }
        if flattens(b) && y > own.bottom() {
            let r = self.level(&b.children, right)?;
            // Left of a last child: out to after its parent.
            let last = visible_with_gaps(&b.children, TOP).last().map(|(i, _)| b.children[*i].id);
            if r.place == DropPlace::After
                && Some(r.target) == last
                && self.bb.get(&r.target).is_some_and(|t| x < t.left() - px(5.))
            {
                return hit(b.id, DropPlace::After);
            }
            return Some(r);
        }
        if y < own.center().y {
            return self.before(&vis, i, right);
        }
        if dragged {
            return None;
        }
        if flattens(b) || (Self::nestable(b) && x - own.left() > px(36.)) {
            return hit(b.id, DropPlace::FirstChild);
        }
        hit(b.id, DropPlace::After)
    }

    /// Above block `i`: after the one before it when that has no rows below
    /// it (nesting under it when far enough right), else before `i`.
    fn before(&self, vis: &[(&Block, Bounds<Pixels>)], i: usize, right: Option<Pixels>) -> Option<Hit> {
        let b = vis[i].0;
        let hit = |target: BlockId, place| Some(Hit { target, place, right });
        match i.checked_sub(1).map(|j| vis[j].0) {
            Some(prev) if !flattens(prev) => {
                if self.ids.contains(&prev.id) {
                    return if self.ids.contains(&b.id) { None } else { hit(b.id, DropPlace::Before) };
                }
                let left = self.bb.get(&prev.id)?.left();
                if Self::nestable(prev) && self.p.x - left > px(36.) {
                    hit(prev.id, DropPlace::FirstChild)
                } else {
                    hit(prev.id, DropPlace::After)
                }
            }
            _ if self.ids.contains(&b.id) => None,
            _ => hit(b.id, DropPlace::Before),
        }
    }
}

fn dist_x(x: Pixels, b: &Bounds<Pixels>) -> f32 {
    if x < b.left() {
        f32::from(b.left() - x)
    } else if x > b.right() {
        f32::from(x - b.right())
    } else {
        0.
    }
}

impl Editor {
    pub(super) fn is_dragged(&self, id: BlockId) -> bool {
        self.block_drag.as_ref().is_some_and(|d| d.started && d.ids.contains(&id))
    }

    /// A press on a block handle: the drag starts once the pointer moves.
    pub(super) fn press_handle(&mut self, id: BlockId, at: Point<Pixels>) {
        if matches!(self.core.kind_of(id), None | Some(BlockKind::Grid | BlockKind::Column { .. })) {
            return;
        }
        let multi = self.core.block_sel_ids();
        let (ids, sel) = if multi.len() > 1 && multi.contains(&id) {
            (multi, self.core.block_sel)
        } else {
            // A folded heading carries its hidden section.
            let keep = self.core.block_sel;
            self.core.select_block(id);
            let ids = self.core.block_sel_ids();
            self.core.block_sel = keep;
            (ids, None)
        };
        let bb = self.block_bounds.borrow();
        let empty = HashSet::new();
        let w = Walk { bb: &bb, ids: &empty, p: at };
        let index = super::core::block_index(&self.core.doc);
        let size = ids_union(&w, &index, &ids).map_or(size(px(0.), px(0.)), |b| b.size);
        drop(bb);
        self.block_drag = Some(BlockDrag { ids, sel, from: at, at, started: false, drop: None, scrolled: None, size, active: None });
    }

    pub(super) fn block_drag_move(&mut self, at: Point<Pixels>, pressed: bool, window: &mut Window, cx: &mut Context<Self>) {
        if !pressed {
            return self.end_block_drag(None, window, cx);
        }
        let Some(d) = &mut self.block_drag else { return };
        d.at = at;
        if !d.started {
            let m = d.at - d.from;
            if f32::from(m.x).hypot(f32::from(m.y)) <= 2. {
                return;
            }
            d.started = true;
            if let Some(sel) = d.sel {
                self.core.block_sel = Some(sel);
            }
            self.menu = None;
            self.slash = None;
            self.toolbar_hidden = true;
        }
        self.update_block_drop();
        cx.notify();
    }

    pub(super) fn end_block_drag(&mut self, at: Option<Point<Pixels>>, window: &mut Window, cx: &mut Context<Self>) {
        let Some(d) = self.block_drag.take() else { return };
        if !d.started {
            return;
        }
        self.toolbar_hidden = false;
        let m = at.unwrap_or(d.at) - d.from;
        // A press that hardly moved is a click.
        if f32::from(m.x).hypot(f32::from(m.y)) > 10.
            && let Some(drop) = d.drop
            && self.core.move_blocks(&d.ids, drop.target, drop.place)
        {
            self.core.clear_block_sel();
            self.drop_flash = Some((d.ids, Instant::now()));
            self.changed(window, cx);
        }
        cx.notify();
    }

    /// Recomputes the drop under the pointer from last frame's layout.
    fn update_block_drop(&mut self) {
        let Some(d) = &self.block_drag else { return };
        if !d.started {
            return;
        }
        let drop = self.find_drop(d.at, &d.ids);
        if let Some(d) = &mut self.block_drag {
            d.drop = drop;
        }
    }

    /// Where dropped files land: the block-drag drop under `p` (target,
    /// place and the 2px line).
    pub(super) fn file_drop_at(&self, p: Point<Pixels>) -> Option<(BlockId, DropPlace, Bounds<Pixels>)> {
        self.find_drop(p, &[]).map(|d| (d.target, d.place, d.line))
    }

    fn find_drop(&self, p: Point<Pixels>, ids: &[BlockId]) -> Option<BlockDrop> {
        if !self.list.viewport_bounds().contains(&p) {
            return None;
        }
        let bb = self.block_bounds.borrow();
        let set: HashSet<BlockId> = ids.iter().copied().collect();
        let w = Walk { bb: &bb, ids: &set, p };
        let hit = w.level(&self.core.doc.blocks, None)?;
        let core = &self.core;
        // Where the blocks land: the parent they join.
        let parent = match hit.place {
            DropPlace::FirstChild => Some(hit.target),
            _ => core.parent_of(hit.target).map(|b| b.id),
        };
        let chain: Vec<&Block> = match parent {
            Some(par) => {
                let path = core.path(par)?;
                (1..=path.len()).filter_map(|n| block_at(&core.doc.blocks, &path[..n])).collect()
            }
            None => vec![],
        };
        if chain.len() + 1 > MAX_DEPTH {
            return None;
        }
        let in_quote = chain.iter().any(|b| matches!(b.kind, BlockKind::Quote));
        let in_callout = chain.iter().any(|b| matches!(b.kind, BlockKind::Callout { .. }));
        for id in ids {
            let ok = match core.kind_of(*id)? {
                BlockKind::Grid => chain.is_empty(),
                BlockKind::Quote => !in_quote,
                BlockKind::Callout { .. } | BlockKind::Table(_) => !in_quote && !in_callout,
                _ => true,
            };
            if !ok {
                return None;
            }
        }

        let index = super::core::block_index(&core.doc);
        let t = index.get(&hit.target)?;
        let own = *bb.get(&hit.target)?;
        let span = w.span(t)?;
        let right = hit.right.unwrap_or(own.right());
        let top = match hit.place {
            DropPlace::Before => own.top(),
            DropPlace::After => span.bottom(),
            DropPlace::FirstChild => own.bottom(),
        };
        let line = Bounds::from_corners(point(own.left(), top), point(right.max(own.left() + px(INDENT * 2.)), top + px(2.)));
        let hints = match hit.place {
            DropPlace::FirstChild => 1,
            DropPlace::Before => 0,
            // How many levels further out the line could still move: the
            // target ends each parent's rows up the chain.
            DropPlace::After => {
                let mut n = 0;
                let mut cur = hit.target;
                while let Some(par) = core.parent_of(cur)
                    && flattens(par)
                    && visible_with_gaps(&par.children, TOP).last().map(|(i, _)| par.children[*i].id) == Some(cur)
                {
                    n += 1;
                    cur = par.id;
                }
                n
            }
        };
        let area = chain
            .last()
            .filter(|b| !matches!(b.kind, BlockKind::Column { .. } | BlockKind::Callout { .. } | BlockKind::Grid))
            .and_then(|b| w.span(b));
        Some(BlockDrop { target: hit.target, place: hit.place, line, hints, nest: hit.place == DropPlace::FirstChild, area })
    }

    /// Within 20px of the page's top or bottom edge the page scrolls 6px
    /// every 10ms; the drop follows the page.
    pub(super) fn block_autoscroll(&mut self, window: &mut Window) {
        let active = window.is_window_active();
        if let Some(d) = &mut self.block_drag
            && *d.active.get_or_insert(active)
            && !active
        {
            // Leaving the window drops nothing.
            self.block_drag = None;
            self.toolbar_hidden = false;
            return;
        }
        self.update_block_drop();
        let v = self.list.viewport_bounds();
        let Some(d) = &mut self.block_drag else { return };
        let dir = if !d.started {
            0.
        } else if d.at.y < v.top() + px(20.) {
            -1.
        } else if d.at.y > v.bottom() - px(20.) {
            1.
        } else {
            0.
        };
        if dir == 0. {
            d.scrolled = None;
            return;
        }
        let now = Instant::now();
        let dt = d.scrolled.map_or(0., |t| (now - t).as_secs_f32().min(0.05));
        d.scrolled = Some(now);
        self.list.scroll_by(px(dir * 600. * dt));
        window.request_animation_frame();
    }

    /// Moved blocks flash in the accent for 2s, as light as the
    /// selected-block tint.
    pub(super) fn drop_flash_el(&self, id: BlockId) -> Option<Div> {
        let (ids, t0) = self.drop_flash.as_ref()?;
        if !ids.contains(&id) {
            return None;
        }
        let k = t0.elapsed().as_secs_f32() / FLASH.as_secs_f32();
        let a = if k < 0.2 { k / 0.2 } else if k < 0.8 { 1. } else { ((1. - k) / 0.2).max(0.) };
        Some(div().absolute().inset_0().rounded(px(3.)).bg(Theme::block_selected().opacity(a)))
    }

    /// Ends the flash once it has run (and keeps frames coming until then).
    pub(super) fn tick_drop_flash(&mut self, window: &mut Window) {
        if let Some((_, t0)) = &self.drop_flash {
            if t0.elapsed() >= FLASH {
                self.drop_flash = None;
            }
            window.request_animation_frame();
        }
    }

    /// The layer over the page while blocks move: the grabbing cursor, the
    /// drop line and the card following the pointer.
    pub(super) fn block_drag_overlay(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let d = self.block_drag.as_ref().filter(|d| d.started)?;
        let o = self.content_origin.get();
        let mut layer = div().id("block-drag-overlay").absolute().inset_0().occlude().cursor(CursorStyle::ClosedHand);
        if let Some(drop) = &d.drop {
            if let Some(a) = drop.area {
                layer = layer.child(
                    div()
                        .absolute()
                        .left(a.left() - o.x)
                        .top(a.top() - o.y)
                        .w(a.size.width)
                        .h(a.size.height)
                        .rounded(px(3.))
                        .bg(Theme::primary().opacity(25. / 255.)),
                );
            }
            let l = drop.line;
            // The drop indicator: a 2pt accent line, its ends rounded.
            layer = layer.child(div().absolute().left(l.left() - o.x).top(l.top() - o.y).w(l.size.width).h(px(2.)).rounded(px(1.)).bg(Theme::primary()));
            // The hint: bg-body segments covering the line, each with a
            // lighter inner bar and a 2px gap at its right end.
            let n = drop.hints;
            let alpha = (256. * (1. - 1. / (n as f32 + 2.))).floor() / 255.;
            for i in 0..n {
                let dx = if drop.nest { INDENT * i as f32 } else { -INDENT * (i as f32 + 1.) };
                layer = layer.child(
                    div()
                        .absolute()
                        .left(l.left() - o.x + px(dx))
                        .top(l.top() - o.y)
                        .w(px(INDENT))
                        .h(px(2.))
                        .bg(crate::theme::raised(Theme::bg_body()))
                        .child(div().w(px(INDENT - 2.)).h_full().bg(Theme::primary().opacity(alpha))),
                );
            }
        }
        Some(deferred(layer.child(self.drag_ghost(d, cx)?.left(d.at.x - o.x).top(d.at.y - o.y + px(28.)))).with_priority(4).into_any_element())
    }

    /// The moved blocks, drawn again on a card. Large ones are shown at a
    /// smaller size: the card is `s` times as wide and is cut at `s` times
    /// the height (GPUI can't scale, so the text rewraps instead).
    fn drag_ghost(&self, d: &BlockDrag, cx: &mut Context<Self>) -> Option<Div> {
        let (w, h) = (f32::from(d.size.width), f32::from(d.size.height));
        if w <= 0. {
            return None;
        }
        let s = if h <= 78. || w <= 78. { 1. } else { (365. / w.max(h)).max(78. / w.min(h)).min(1.) };
        let r = R {
            ed: self,
            units: Rc::new(vec![]),
            eqnos: self.frame.eqnos.clone(),
            focused: false,
            selected: Rc::new(HashSet::new()),
            targets: RefCell::new(vec![]),
            flat: None,
            cache: Rc::new(RefCell::new(HashMap::new())),
            bounds: Rc::new(RefCell::new(HashMap::new())),
            ghost: true,
        };
        let core = &self.core;
        let mut col = div().flex().flex_col();
        let mut prev_mb: Option<f32> = None;
        for id in &d.ids {
            let Some(path) = core.path(*id) else { continue };
            let Some((list, ix)) = siblings_at(&core.doc.blocks, &path) else { continue };
            let b = &list[ix];
            let depth = path.len() - 1;
            let env = Env { depth, root: depth == 0, width: w * s - 4., ..TOP };
            let (mt, mb) = margins(b, env);
            col = col.child(div().pl(px(2.)).mt(px(prev_mb.map_or(0., |p| p.max(mt)))).child(r.block(b, list, ix, env, cx)));
            prev_mb = Some(mb);
        }
        // A lifted copy of the blocks: white, hairline, crisp radius 5, the panel drop.
        Some(
            div()
                .absolute()
                .w(px(w * s))
                .max_h(px(h * s))
                .overflow_hidden()
                .rounded(px(5.))
                .border_1()
                .border_color(Theme::line_border())
                .bg(crate::theme::raised(Theme::bg_body()))
                .shadow(theme::shadow_panel())
                .child(col),
        )
    }
}

/// The list a block sits in, and its index there.
fn ids_union(w: &Walk, index: &rustc_hash::FxHashMap<BlockId, &Block>, ids: &[BlockId]) -> Option<Bounds<Pixels>> {
    ids.iter().filter_map(|id| w.span(index.get(id)?)).reduce(|a, b| a.union(&b))
}

impl R<'_> {
    /// A table on the drag card: cells and borders only (the page's table
    /// keeps its own bar and scroll state).
    pub(super) fn ghost_table(&self, b: &Block, t: &Table, cx: &mut Context<Editor>) -> AnyElement {
        let env = Env { cell: true, root: false, ..TOP };
        let widths = self.ed.table_widths(b.id, t);
        let n = widths.len();
        let mut grid = div().grid().grid_col_widths(widths.clone()).w(px(widths.iter().sum()));
        for (r, c, cell) in t.cells() {
            let (rs, cs) = (cell.rowspan.max(1) as usize, cell.colspan.max(1) as usize);
            let header = r < t.header_rows || (t.header_col && c == 0);
            let bg = cell.bg.as_ref().and_then(|x| theme::block_bg(&x.0)).or(header.then(Theme::table_header));
            grid = grid.child(
                div()
                    .row_start(r as i16 + 1)
                    .row_end((r + rs) as i16 + 1)
                    .col_start(c as i16 + 1)
                    .col_end((c + cs) as i16 + 1)
                    .w(px(widths[c..(c + cs).min(n)].iter().sum()))
                    .p(px(8.))
                    .pb(px(9.))
                    .when(r == 0, |d| d.border_t_1())
                    .when(c == 0, |d| d.border_l_1())
                    .border_r_1()
                    .border_b_1()
                    .border_color(Theme::line_border())
                    .when_some(bg, |d, c| d.bg(c))
                    .child(self.blocks(&cell.blocks, env, cx)),
            );
        }
        div().child(grid).into_any_element()
    }
}
