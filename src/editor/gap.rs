//! Clicking between blocks: a
//! click (moved at most 10px) within 16px of a block edge, where neither
//! neighbour has text to put the caret in, inserts an empty paragraph there.

use super::core::{DropPlace, Pos};
use super::render::{TOP, flattens, visible_with_gaps};
use super::view::Editor;
use crate::model::*;
use gpui::*;
use std::collections::HashMap;

/// How far from a block's edge a click still counts as next to it.
const REACH: f32 = 16.;
/// A press that moved further is a drag.
const SLOP: f32 = 10.;

/// One-line blocks: no text of their own to click into.
fn one_line(b: &Block) -> bool {
    matches!(
        b.kind,
        BlockKind::Code { .. }
            | BlockKind::Quote
            | BlockKind::Callout { .. }
            | BlockKind::Divider
            | BlockKind::Image { .. }
            | BlockKind::Table(_)
            | BlockKind::Grid
            | BlockKind::SubPageList
            | BlockKind::Countdown(_)
            | BlockKind::File(_)
            | BlockKind::Raw { .. }
    )
}

/// A text block whose last descendant is a one-line block.
fn text_child_one_line(b: &Block) -> bool {
    let text = matches!(b.kind, BlockKind::Text | BlockKind::Heading { .. } | BlockKind::Bullet | BlockKind::Ordered { .. } | BlockKind::Todo { .. });
    text && b.children.last().is_some_and(|l| one_line(l) || text_child_one_line(l))
}

struct Walk<'a> {
    bb: &'a HashMap<BlockId, Bounds<Pixels>>,
    p: Point<Pixels>,
}

impl Walk<'_> {
    /// The block's rows: its own and, when its children are rows of their own, theirs.
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

    /// Where `list` (the children of the innermost block under the point)
    /// takes the new paragraph.
    fn find(&self, list: &[Block], root: bool) -> Option<(BlockId, DropPlace)> {
        let y = self.p.y;
        let vis: Vec<(&Block, Bounds<Pixels>)> =
            visible_with_gaps(list, TOP).into_iter().filter_map(|(i, _)| Some((&list[i], self.span(&list[i])?))).collect();
        if let Some(&(b, s)) = vis.iter().find(|(_, s)| y >= s.top() && y <= s.bottom()) {
            let own = *self.bb.get(&b.id)?;
            return match &b.kind {
                BlockKind::Callout { .. } | BlockKind::Quote if own.contains(&self.p) => self.find(&b.children, false),
                BlockKind::Grid => {
                    let col = b.children.iter().find(|c| self.bb.get(&c.id).is_some_and(|cb| cb.contains(&self.p)))?;
                    self.find(&col.children, false)
                }
                _ if flattens(b) && y > own.bottom() && y <= s.bottom() => self.find(&b.children, false),
                _ => None,
            };
        }
        let next = vis.iter().position(|(_, s)| s.top() > y);
        let (above, below) = match next {
            Some(i) => (i.checked_sub(1).map(|j| vis[j]), Some(vis[i])),
            None => (vis.last().copied(), None),
        };
        let near = below.is_some_and(|(_, s)| f32::from(s.top() - y) < REACH) || above.is_some_and(|(_, s)| f32::from(y - s.bottom()) < REACH);
        if !near {
            return None;
        }
        match (above, below) {
            (None, Some((h, _))) => (!root && one_line(h)).then_some((h.id, DropPlace::Before)),
            (Some((d, _)), None) => one_line(d).then_some((d.id, DropPlace::After)),
            (Some((d, _)), Some((h, _))) => ((one_line(d) || text_child_one_line(d)) && one_line(h)).then_some((d.id, DropPlace::After)),
            (None, None) => None,
        }
    }
}

impl Editor {
    /// Where a click at `p` would insert a paragraph between blocks.
    pub(super) fn gap_at(&self, p: Point<Pixels>) -> Option<(BlockId, DropPlace)> {
        if self.read_only {
            return None;
        }
        let bb = self.block_bounds.borrow();
        Walk { bb: &bb, p }.find(&self.core.doc.blocks, true)
    }

    /// The release of a press that `on_mouse_down` left to [`Self::gap_at`].
    pub(super) fn gap_release(&mut self, p: Point<Pixels>, window: &mut Window, cx: &mut Context<Self>) {
        let Some(from) = self.gap_press.take() else { return };
        let d = p - from;
        if f32::from(d.x).abs() > SLOP || f32::from(d.y).abs() > SLOP {
            return;
        }
        let Some((target, place)) = self.gap_at(p) else { return };
        let b = Block::text("");
        let id = b.id;
        self.core.insert_blocks_at(vec![b], target, place);
        self.core.clear_block_sel();
        self.set_caret(Pos::new(id, 0), false);
        self.restart_blink(window, cx);
        self.changed(window, cx);
    }
}
