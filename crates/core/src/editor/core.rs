//! GUI-independent editing operations on a [`Document`].
//!
//! Positions are `(block id, utf-8 byte offset)`. The title is addressed with
//! the reserved id [`TITLE`]. Every mutation goes through [`Core`], which keeps
//! the selection valid and records undo snapshots.

use crate::model::*;
use std::cell::{Cell, RefCell};
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::ops::Range;
use std::time::{Duration, Instant};
use unicode_segmentation::UnicodeSegmentation;

pub const TITLE: BlockId = BlockId(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pos {
    pub block: BlockId,
    pub offset: usize,
}

impl Pos {
    pub fn new(block: BlockId, offset: usize) -> Self {
        Pos { block, offset }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Selection {
    pub anchor: Pos,
    pub head: Pos,
}

impl Selection {
    pub fn caret(p: Pos) -> Self {
        Selection { anchor: p, head: p }
    }
    pub fn is_collapsed(&self) -> bool {
        self.anchor == self.head
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Seg {
    Child(usize),
    Cell(usize, usize, usize),
}

pub type Path = Vec<Seg>;

/// A selectable stop in document order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Unit {
    pub id: BlockId,
    /// Text-bearing (caret can move inside) vs. atomic (divider, image, ...).
    pub text: bool,
    /// Table this unit lives in, if any (selection never crosses this boundary).
    pub scope: Option<BlockId>,
}

// ------------------------------------------------------------------ tree access

pub fn find_path(blocks: &[Block], id: BlockId) -> Option<Path> {
    for (i, b) in blocks.iter().enumerate() {
        if b.id == id {
            return Some(vec![Seg::Child(i)]);
        }
        if let Some(mut p) = find_path(&b.children, id) {
            p.insert(0, Seg::Child(i));
            // Child paths are relative to `b`; re-root below.
            return Some(p);
        }
        if let BlockKind::Table(t) = &b.kind {
            for (r, row) in t.rows.iter().enumerate() {
                for (c, cell) in row.iter().enumerate() {
                    if let Some(mut p) = find_path(&cell.blocks, id) {
                        let first = match p[0] {
                            Seg::Child(k) => Seg::Cell(r, c, k),
                            s => s,
                        };
                        p[0] = first;
                        p.insert(0, Seg::Child(i));
                        return Some(p);
                    }
                }
            }
        }
    }
    None
}

/// Paths of every block (table cells included), for bulk lookups that would
/// otherwise call `find_path` once per block.
pub fn path_index(blocks: &[Block]) -> HashMap<BlockId, Path> {
    fn walk(blocks: &[Block], base: &[Seg], seg: &dyn Fn(usize) -> Seg, out: &mut HashMap<BlockId, Path>) {
        for (i, b) in blocks.iter().enumerate() {
            let mut p = base.to_vec();
            p.push(seg(i));
            walk(&b.children, &p, &Seg::Child, out);
            if let BlockKind::Table(t) = &b.kind {
                for (r, row) in t.rows.iter().enumerate() {
                    for (c, cell) in row.iter().enumerate() {
                        walk(&cell.blocks, &p, &move |k| Seg::Cell(r, c, k), out);
                    }
                }
            }
            out.entry(b.id).or_insert(p);
        }
    }
    let mut out = HashMap::default();
    walk(blocks, &[], &Seg::Child, &mut out);
    out
}

/// Whether a block is one of `ids` or nested inside one of them.
fn inside_of(paths: HashMap<BlockId, Path>, ids: &[BlockId]) -> impl Fn(BlockId) -> bool + use<> {
    let set: HashSet<Path> = ids.iter().filter_map(|id| paths.get(id)).cloned().collect();
    move |id| paths.get(&id).is_some_and(|p| (1..=p.len()).any(|n| set.contains(&p[..n])))
}

/// Every block by id (table cells included).
pub fn block_index(doc: &Document) -> HashMap<BlockId, &Block> {
    fn walk<'a>(blocks: &'a [Block], out: &mut HashMap<BlockId, &'a Block>) {
        for b in blocks {
            out.entry(b.id).or_insert(b);
            walk(&b.children, out);
            if let BlockKind::Table(t) = &b.kind {
                t.rows.iter().flatten().for_each(|c| walk(&c.blocks, out));
            }
        }
    }
    let mut out = HashMap::default();
    walk(&doc.blocks, &mut out);
    out
}

/// Position of every unit in document order.
pub fn unit_positions(units: &[Unit]) -> HashMap<BlockId, usize> {
    units.iter().enumerate().map(|(i, u)| (u.id, i)).collect()
}

// A path is interpreted as: first seg indexes the root list; each following
// seg indexes into the previous block's children (Child) or table cell (Cell).

pub fn block_at<'a>(blocks: &'a [Block], path: &[Seg]) -> Option<&'a Block> {
    let Seg::Child(i) = path[0] else { return None };
    let mut b = blocks.get(i)?;
    for seg in &path[1..] {
        b = match *seg {
            Seg::Child(k) => b.children.get(k)?,
            Seg::Cell(r, c, k) => match &b.kind {
                BlockKind::Table(t) => t.rows.get(r)?.get(c)?.blocks.get(k)?,
                _ => return None,
            },
        };
    }
    Some(b)
}

pub fn block_at_mut<'a>(blocks: &'a mut [Block], path: &[Seg]) -> Option<&'a mut Block> {
    let Seg::Child(i) = path[0] else { return None };
    let mut b = blocks.get_mut(i)?;
    for seg in &path[1..] {
        b = match *seg {
            Seg::Child(k) => b.children.get_mut(k)?,
            Seg::Cell(r, c, k) => match &mut b.kind {
                BlockKind::Table(t) => t.rows.get_mut(r)?.get_mut(c)?.blocks.get_mut(k)?,
                _ => return None,
            },
        };
    }
    Some(b)
}

/// The list containing the block at `path`, and its index in that list.
pub fn siblings_at<'a>(blocks: &'a [Block], path: &[Seg]) -> Option<(&'a [Block], usize)> {
    let (last, outer) = path.split_last()?;
    if outer.is_empty() {
        let Seg::Child(i) = *last else { return None };
        return Some((blocks, i));
    }
    let parent = block_at(blocks, outer)?;
    match (*last, &parent.kind) {
        (Seg::Child(k), _) => Some((&parent.children, k)),
        (Seg::Cell(r, c, k), BlockKind::Table(t)) => Some((&t.rows.get(r)?.get(c)?.blocks, k)),
        _ => None,
    }
}

/// `siblings_at` for writing.
pub fn container_mut<'a>(blocks: &'a mut Vec<Block>, path: &[Seg]) -> Option<(&'a mut Vec<Block>, usize)> {
    if path.len() == 1 {
        let Seg::Child(i) = path[0] else { return None };
        return Some((blocks, i));
    }
    let parent = block_at_mut(blocks, &path[..path.len() - 1])?;
    match *path.last().unwrap() {
        Seg::Child(k) => Some((&mut parent.children, k)),
        Seg::Cell(r, c, k) => match &mut parent.kind {
            BlockKind::Table(t) => Some((&mut t.rows.get_mut(r)?.get_mut(c)?.blocks, k)),
            _ => None,
        },
    }
}

pub fn parent_path(path: &[Seg]) -> Option<&[Seg]> {
    if path.len() > 1 { Some(&path[..path.len() - 1]) } else { None }
}

/// Maximum nesting of text blocks below one container.
pub const MAX_NEST_DEPTH: usize = 16;

/// Nesting depth of the block at `path` counted from its nearest container
/// (document root or table cell).
fn nest_depth(path: &[Seg]) -> usize {
    path.iter().rev().take_while(|s| matches!(s, Seg::Child(_))).count()
}

fn seg_index(s: Seg) -> usize {
    match s {
        Seg::Child(i) | Seg::Cell(_, _, i) => i,
    }
}

fn with_index(s: Seg, i: usize) -> Seg {
    match s {
        Seg::Child(_) => Seg::Child(i),
        Seg::Cell(r, c, _) => Seg::Cell(r, c, i),
    }
}

pub fn units(doc: &Document) -> Vec<Unit> {
    let mut out = vec![Unit { id: TITLE, text: true, scope: None }];
    fn walk(blocks: &[Block], scope: Option<BlockId>, out: &mut Vec<Unit>) {
        let mut hide_below: Option<u8> = None;
        for b in blocks {
            // Folded headings hide following siblings until a heading of the
            // same or higher level.
            if let Some(level) = hide_below {
                match b.kind {
                    BlockKind::Heading { level: l, .. } if l <= level => hide_below = None,
                    _ => continue,
                }
            }
            if b.kind.has_text() {
                out.push(Unit { id: b.id, text: true, scope });
                if let BlockKind::Heading { level, .. } = b.kind
                    && b.folded
                {
                    hide_below = Some(level);
                }
                if !b.folded {
                    walk(&b.children, scope, out);
                }
            } else if b.kind.is_container() {
                walk(&b.children, scope, out);
            } else if let BlockKind::Table(t) = &b.kind {
                for row in &t.rows {
                    for cell in row {
                        walk(&cell.blocks, Some(b.id), out);
                    }
                }
            } else {
                out.push(Unit { id: b.id, text: false, scope });
            }
        }
    }
    walk(&doc.blocks, None, &mut out);
    out
}

pub fn prev_grapheme(s: &str, offset: usize) -> usize {
    s[..offset].grapheme_indices(true).next_back().map(|(i, _)| i).unwrap_or(0)
}

pub fn next_grapheme(s: &str, offset: usize) -> usize {
    s[offset..].graphemes(true).next().map(|g| offset + g.len()).unwrap_or(s.len())
}

pub fn prev_word(s: &str, offset: usize) -> usize {
    let mut last = 0;
    for (i, w) in s.split_word_bound_indices() {
        if i >= offset {
            break;
        }
        if !w.trim().is_empty() {
            last = i;
        }
    }
    last
}

pub fn next_word(s: &str, offset: usize) -> usize {
    for (i, w) in s.split_word_bound_indices() {
        let end = i + w.len();
        if end > offset && !w.trim().is_empty() {
            return end;
        }
    }
    s.len()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DropPlace {
    Before,
    After,
    FirstChild,
}

pub fn word_at(s: &str, offset: usize) -> Range<usize> {
    for (i, w) in s.split_word_bound_indices() {
        if offset >= i && offset < i + w.len() {
            return i..i + w.len();
        }
    }
    offset..offset
}

// ------------------------------------------------------------------ core

#[derive(Clone)]
struct Snapshot {
    doc: Document,
    sel: Selection,
    /// Rough heap size of `doc`, for the undo budget.
    bytes: usize,
}

/// The undo history keeps at most this many steps …
const UNDO_STEPS: usize = 500;
/// … and roughly this much memory (every step is a whole document), but
/// never fewer than `UNDO_MIN_STEPS` steps however long the document.
const UNDO_BYTES: usize = 256 << 20;
const UNDO_MIN_STEPS: usize = 20;

fn doc_bytes(doc: &Document) -> usize {
    let mut n = doc.title.text.len();
    doc.for_each_block(|b| {
        n += std::mem::size_of::<Block>() + b.text.text.len() + b.text.runs.len() * std::mem::size_of::<crate::model::Run>();
        if let BlockKind::Raw { xml, .. } = &b.kind {
            n += xml.len();
        }
    });
    n
}

impl Snapshot {
    fn new(doc: Document, sel: Selection) -> Self {
        let bytes = doc_bytes(&doc);
        Snapshot { doc, sel, bytes }
    }
}

/// The object replacement character standing for an equation.
pub const EQUATION: &str = "\u{fffc}";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditKind {
    Typing,
    Delete,
    /// Typing in an equation's editor.
    Equation,
    Other,
}

/// Block-selection mode: blocks from `anchor` to `head` in visible order.
/// The icon is file-level meta, and folding is not undoable:
/// undo and redo keep both as they are now instead of the snapshot's.
fn keep_outside_history(now: &Document, restored: &mut Document) {
    restored.icon = now.icon.clone();
    let mut folded = HashMap::default();
    now.for_each_block(|b| {
        folded.insert(b.id, b.folded);
    });
    // Blocks in table cells never fold.
    fn walk(blocks: &mut [Block], folded: &HashMap<BlockId, bool>) {
        for b in blocks {
            if let Some(f) = folded.get(&b.id) {
                b.folded = *f;
            }
            walk(&mut b.children, folded);
        }
    }
    walk(&mut restored.blocks, &folded);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockSel {
    pub anchor: BlockId,
    pub head: BlockId,
}

#[derive(Clone, Debug, PartialEq)]
enum Stop {
    Cell(BlockId, usize, usize),
    Block(BlockId),
    Root,
}

/// Whether the block at `outer` is the one at `inner` or one of its ancestors.
fn covers(outer: &[Seg], inner: &[Seg]) -> bool {
    outer.len() <= inner.len() && inner.starts_with(outer)
}

#[derive(Clone)]
pub struct Core {
    pub doc: Document,
    pub sel: Selection,
    /// Set while whole blocks are selected (Esc, handle click, ⌘A, ⇧↑ at a block edge).
    pub block_sel: Option<BlockSel>,
    /// Set while table cells are selected (a cell drag, a row / column bar
    /// click). The caret stays collapsed in the anchor cell meanwhile.
    pub cell_sel: Option<super::table_ops::CellSel>,
    /// Marks to apply to the next typed text (set by ⌘B etc. on a caret).
    pub pending_marks: Option<Marks>,
    undo: std::collections::VecDeque<std::sync::Arc<Snapshot>>,
    redo: Vec<std::sync::Arc<Snapshot>>,
    last_edit: Option<(EditKind, Instant, BlockId)>,
    pub dirty: bool,
    /// Bumped by every change to the document, including folding.
    pub generation: u64,
    /// `find_path` memo. Entries are checked against the tree on use, so
    /// edits never need to invalidate it; a stale entry triggers a rebuild.
    paths: RefCell<HashMap<BlockId, Path>>,
    /// Memo hits since the last rebuild.
    path_hits: Cell<u32>,
    /// Misses since the last rebuild, and how many it takes to rebuild.
    path_misses: Cell<(u32, u32)>,
    /// A message for the view to toast (e.g. 当前位置不支持粘贴表格).
    pub notice: Option<&'static str>,
}

impl Core {
    pub fn new(doc: Document) -> Self {
        let first = units(&doc).get(1).map(|u| u.id).unwrap_or(TITLE);
        let start = if doc.title.is_empty() { TITLE } else { first };
        Core {
            doc,
            sel: Selection::caret(Pos::new(start, 0)),
            block_sel: None,
            cell_sel: None,
            pending_marks: None,
            undo: Default::default(),
            redo: vec![],
            last_edit: None,
            dirty: false,
            generation: 0,
            paths: Default::default(),
            path_hits: Cell::new(u32::MAX),
            path_misses: Cell::new((0, 1)),
            notice: None,
        }
    }

    // ---- lookup

    pub fn text_of(&self, id: BlockId) -> Option<&RichText> {
        if id == TITLE {
            return Some(&self.doc.title);
        }
        let p = self.path(id)?;
        // Atomic blocks (divider, image, ...) have no caret text.
        block_at(&self.doc.blocks, &p).filter(|b| b.kind.has_text()).map(|b| &b.text)
    }

    /// Path of block `id` (cf. `find_path`), answered from the memo when
    /// its entry still points at the block.
    pub fn path(&self, id: BlockId) -> Option<Path> {
        if let Some(p) = self.paths.borrow().get(&id)
            && block_at(&self.doc.blocks, p).is_some_and(|b| b.id == id)
        {
            self.path_hits.set(self.path_hits.get().saturating_add(1));
            return Some(p.clone());
        }
        // A rebuild costs many searches. When the last one went stale before
        // it paid off (a loop restructuring the tree at every step), search
        // directly for a while, backing off further each time.
        let (misses, need) = self.path_misses.get();
        if misses + 1 < need {
            self.path_misses.set((misses + 1, need));
            return find_path(&self.doc.blocks, id);
        }
        let need = if self.path_hits.get() < 64 { (need * 2).min(1 << 16) } else { 1 };
        self.path_misses.set((0, need));
        *self.paths.borrow_mut() = path_index(&self.doc.blocks);
        self.path_hits.set(0);
        self.paths.borrow().get(&id).cloned()
    }

    pub fn text_mut(&mut self, id: BlockId) -> Option<&mut RichText> {
        if id == TITLE {
            return Some(&mut self.doc.title);
        }
        let p = self.path(id)?;
        block_at_mut(&mut self.doc.blocks, &p).filter(|b| b.kind.has_text()).map(|b| &mut b.text)
    }

    pub fn block(&self, id: BlockId) -> Option<&Block> {
        let p = self.path(id)?;
        block_at(&self.doc.blocks, &p)
    }

    pub fn block_mut(&mut self, id: BlockId) -> Option<&mut Block> {
        self.generation += 1;
        let p = self.path(id)?;
        block_at_mut(&mut self.doc.blocks, &p)
    }

    pub fn parent_of(&self, id: BlockId) -> Option<&Block> {
        let p = self.path(id)?;
        let pp = parent_path(&p)?;
        block_at(&self.doc.blocks, pp)
    }

    pub fn is_in_table(&self, id: BlockId) -> bool {
        self.path(id).is_some_and(|p| p.iter().any(|s| matches!(s, Seg::Cell(..))))
    }

    pub fn units(&self) -> Vec<Unit> {
        units(&self.doc)
    }

    fn unit_index(&self, units: &[Unit], id: BlockId) -> Option<usize> {
        units.iter().position(|u| u.id == id)
    }

    pub fn len_of(&self, id: BlockId) -> usize {
        self.text_of(id).map(|t| t.len()).unwrap_or(0)
    }

    /// (start, end) of the selection in document order.
    pub fn ordered(&self) -> (Pos, Pos) {
        let s = self.sel;
        if s.anchor.block == s.head.block {
            if s.anchor.offset <= s.head.offset { (s.anchor, s.head) } else { (s.head, s.anchor) }
        } else {
            self.ordered_in(&self.units())
        }
    }

    /// `ordered` with the units already at hand.
    pub fn ordered_in(&self, us: &[Unit]) -> (Pos, Pos) {
        let s = self.sel;
        if s.anchor.block == s.head.block {
            return self.ordered();
        }
        let a = self.unit_index(us, s.anchor.block).unwrap_or(0);
        let h = self.unit_index(us, s.head.block).unwrap_or(0);
        if a <= h { (s.anchor, s.head) } else { (s.head, s.anchor) }
    }

    /// Selected byte range inside block `id`, if the selection touches it.
    pub fn range_in(&self, id: BlockId, units: &[Unit]) -> Option<Range<usize>> {
        let (s, e) = self.ordered_in(units);
        if s.block == e.block {
            return (s.block == id).then_some(s.offset..e.offset);
        }
        let si = self.unit_index(units, s.block)?;
        let ei = self.unit_index(units, e.block)?;
        let i = self.unit_index(units, id)?;
        if i < si || i > ei {
            return None;
        }
        let len = self.len_of(id);
        let a = if i == si { s.offset } else { 0 };
        let b = if i == ei { e.offset } else { len };
        Some(a..b)
    }

    /// Atomic blocks fully covered by the selection (rendered as selected).
    pub fn atomic_selected(&self, id: BlockId, units: &[Unit]) -> bool {
        let (s, e) = self.ordered_in(units);
        if s.block == id && e.block == id {
            return true;
        }
        let (Some(si), Some(ei), Some(i)) =
            (self.unit_index(units, s.block), self.unit_index(units, e.block), self.unit_index(units, id))
        else {
            return false;
        };
        si != ei && i >= si && i <= ei
    }

    pub fn selected_text(&self) -> String {
        let us = self.units();
        let (s, e) = self.ordered_in(&us);
        let si = self.unit_index(&us, s.block).unwrap_or(0);
        let ei = self.unit_index(&us, e.block).unwrap_or(0);
        let index = block_index(&self.doc);
        let mut out = String::new();
        for u in us[si..=ei.max(si)].iter().filter(|u| u.text) {
            let Some(t) = (if u.id == TITLE { Some(&self.doc.title) } else { index.get(&u.id).map(|b| &b.text) }) else { continue };
            let a = if u.id == s.block { s.offset } else { 0 };
            let b = if u.id == e.block { e.offset } else { t.len() };
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str(&t.text[a.min(b)..b]);
        }
        out
    }

    // ---- undo

    /// Record a snapshot before mutating. Consecutive typing in the same block
    /// coalesces into one undo step while each keystroke follows the previous
    /// one within 700 ms (the window slides).
    pub fn checkpoint(&mut self, kind: EditKind) {
        self.generation += 1;
        let now = Instant::now();
        let block = self.sel.head.block;
        let coalesce = matches!(
            self.last_edit,
            Some((k, t, b)) if k == kind && kind != EditKind::Other && b == block
                && (kind == EditKind::Equation || now.duration_since(t) < Duration::from_millis(700))
        );
        if !coalesce {
            self.push_undo(Snapshot::new(self.doc.clone(), self.sel));
        }
        self.last_edit = Some((kind, now, block));
        self.redo.clear();
        self.dirty = true;
    }

    /// Put `doc` back outside the edit history (阅读模式 refusing an edit).
    pub fn restore(&mut self, doc: Document) {
        self.generation += 1;
        self.doc = doc;
        self.block_sel = None;
        self.cell_sel = None;
        self.last_edit = None;
        self.dirty = false;
        self.fix_selection();
    }

    pub fn break_coalescing(&mut self) {
        self.last_edit = None;
    }

    fn push_undo(&mut self, s: Snapshot) {
        self.undo.push_back(std::sync::Arc::new(s));
        let mut total: usize = self.undo.iter().map(|s| s.bytes).sum();
        while self.undo.len() > UNDO_STEPS || (total > UNDO_BYTES && self.undo.len() > UNDO_MIN_STEPS) {
            let Some(old) = self.undo.pop_front() else { break };
            total -= old.bytes;
        }
    }

    pub fn undo(&mut self) -> bool {
        let Some(s) = self.undo.pop_back() else { return false };
        self.generation += 1;
        let mut doc = s.doc.clone();
        keep_outside_history(&self.doc, &mut doc);
        self.redo.push(std::sync::Arc::new(Snapshot::new(std::mem::replace(&mut self.doc, doc), self.sel)));
        self.sel = s.sel;
        self.block_sel = None;
        self.cell_sel = None;
        self.last_edit = None;
        self.dirty = true;
        self.fix_selection();
        true
    }

    pub fn redo(&mut self) -> bool {
        let Some(s) = self.redo.pop() else { return false };
        self.generation += 1;
        let mut doc = s.doc.clone();
        keep_outside_history(&self.doc, &mut doc);
        let back = Snapshot::new(std::mem::replace(&mut self.doc, doc), self.sel);
        self.push_undo(back);
        self.sel = s.sel;
        self.block_sel = None;
        self.cell_sel = None;
        self.last_edit = None;
        self.dirty = true;
        self.fix_selection();
        true
    }

    /// Clamp the selection to existing blocks / valid char boundaries.
    pub fn fix_selection(&mut self) {
        let us = self.units();
        let fix = |core: &Core, p: Pos| -> Pos {
            match core.text_of(p.block) {
                Some(t) => {
                    let mut o = p.offset.min(t.len());
                    while !t.text.is_char_boundary(o) {
                        o -= 1;
                    }
                    Pos::new(p.block, o)
                }
                None if us.iter().any(|u| u.id == p.block) => Pos::new(p.block, 0),
                None => Pos::new(us.get(1).map(|u| u.id).unwrap_or(TITLE), 0),
            }
        };
        self.sel = Selection { anchor: fix(self, self.sel.anchor), head: fix(self, self.sel.head) };
    }

    // ---- primitive mutations (no checkpoint)

    fn ensure_non_empty(&mut self) {
        if self.doc.blocks.is_empty() {
            self.doc.blocks.push(Block::text(""));
        }
    }

    /// Remove a block; its children are lifted into its place.
    fn remove_lift(&mut self, id: BlockId) {
        let Some(p) = self.path(id) else { return };
        let Some((list, i)) = container_mut(&mut self.doc.blocks, &p) else { return };
        let b = list.remove(i);
        let kids = b.children;
        for (k, c) in kids.into_iter().enumerate() {
            list.insert(i + k, c);
        }
    }

    fn remove_subtree(&mut self, id: BlockId) -> Option<Block> {
        let p = self.path(id)?;
        let (list, i) = container_mut(&mut self.doc.blocks, &p)?;
        Some(list.remove(i))
    }

    pub fn insert_after(&mut self, id: BlockId, b: Block) {
        if id == TITLE {
            self.doc.blocks.insert(0, b);
            return;
        }
        let Some(p) = self.path(id) else { return };
        if let Some((list, i)) = container_mut(&mut self.doc.blocks, &p) {
            list.insert(i + 1, b);
        }
    }

    /// `blocks` in order right after `id` (at the top for the title).
    fn insert_all_after(&mut self, id: BlockId, blocks: Vec<Block>) {
        let mut anchor = id;
        for b in blocks {
            let next = b.id;
            self.insert_after(anchor, b);
            anchor = next;
        }
    }

    fn insert_before(&mut self, id: BlockId, b: Block) {
        let Some(p) = self.path(id) else { return };
        if let Some((list, i)) = container_mut(&mut self.doc.blocks, &p) {
            list.insert(i, b);
        }
    }

    /// Drop empty containers left behind after deletions/moves.
    fn normalize(&mut self) {
        fn walk(list: &mut Vec<Block>, in_cell: bool) {
            for b in list.iter_mut() {
                walk(&mut b.children, false);
                if let BlockKind::Table(t) = &mut b.kind {
                    for row in &mut t.rows {
                        for cell in row {
                            walk(&mut cell.blocks, true);
                            if cell.blocks.is_empty() && !cell.is_covered() {
                                cell.blocks.push(Block::text(""));
                            }
                        }
                    }
                }
                if matches!(b.kind, BlockKind::Grid) {
                    drop_empty_columns(&mut b.children);
                }
            }
            list.retain(|b| !(matches!(b.kind, BlockKind::Quote | BlockKind::Callout { .. }) && b.children.is_empty()));
            // A grid left with one column dissolves into its content.
            let mut i = 0;
            while i < list.len() {
                if matches!(list[i].kind, BlockKind::Grid) && list[i].children.len() <= 1 {
                    let g = list.remove(i);
                    let content: Vec<Block> = g.children.into_iter().flat_map(|c| c.children).collect();
                    let n = content.len();
                    list.splice(i..i, content);
                    i += n;
                } else {
                    i += 1;
                }
            }
            let _ = in_cell;
        }
        walk(&mut self.doc.blocks, false);
        self.ensure_non_empty();
    }

    /// Delete the current selection (if any). Returns the collapsed caret.
    fn delete_selection_inner(&mut self) -> Pos {
        let (s, e) = self.ordered();
        if s == e {
            return s;
        }
        if s.block == e.block {
            if let Some(t) = self.text_mut(s.block) {
                t.replace(s.offset..e.offset, &RichText::default());
            } else {
                // Atomic block selected on its own.
                return self.delete_atomic(s.block);
            }
            self.sel = Selection::caret(s);
            return s;
        }
        let us = self.units();
        let si = self.unit_index(&us, s.block).unwrap_or(0);
        let ei = self.unit_index(&us, e.block).unwrap_or(0);
        let start_unit = us[si];
        let end_unit = us[ei];
        // Tail of the end block survives and joins the start block.
        let tail = if end_unit.text {
            self.text_of(e.block).map(|t| t.slice(e.offset..t.len())).unwrap_or_default()
        } else {
            RichText::default()
        };
        let middle: Vec<BlockId> = us[si + 1..ei].iter().map(|u| u.id).collect();
        if start_unit.text {
            let tail = if self.is_code(s.block) { RichText::plain(tail.plain_text()) } else { tail };
            if let Some(t) = self.text_mut(s.block) {
                let len = t.len();
                t.replace(s.offset..len, &tail);
            }
            for id in middle.iter().rev() {
                self.remove_lift(*id);
            }
            if e.block != TITLE {
                // Children of the end block move under the start block when
                // it can hold them; otherwise they follow it.
                let kids = self.block_mut(e.block).map(|b| std::mem::take(&mut b.children)).unwrap_or_default();
                self.remove_lift(e.block);
                match self.block_mut(s.block) {
                    Some(b) if b.kind.nests() => {
                        let mut kids = kids;
                        kids.append(&mut b.children);
                        b.children = kids;
                    }
                    _ => self.insert_all_after(s.block, kids),
                }
            }
            self.normalize();
            self.sel = Selection::caret(s);
            s
        } else {
            // Selection starts on an atomic block: remove everything up to the end.
            for id in middle.iter().rev() {
                self.remove_lift(*id);
            }
            if end_unit.text {
                if let Some(t) = self.text_mut(e.block) {
                    t.replace(0..e.offset, &RichText::default());
                }
                self.remove_subtree(s.block);
                self.normalize();
                let p = Pos::new(e.block, 0);
                self.sel = Selection::caret(p);
                p
            } else {
                self.remove_subtree(e.block);
                self.delete_atomic(s.block)
            }
        }
    }

    /// Remove an atomic block, putting the caret on a neighbor (or a fresh paragraph).
    fn delete_atomic(&mut self, id: BlockId) -> Pos {
        let us = self.units();
        let i = self.unit_index(&us, id).unwrap_or(0);
        let p = self.path(id);
        let Some(p) = p else { return self.sel.head };
        let in_scope = us[i].scope;
        let next = us.get(i + 1).filter(|u| u.text && u.scope == in_scope).map(|u| u.id);
        let prev = us.get(i.wrapping_sub(1)).filter(|u| u.text && u.scope == in_scope && u.id != TITLE).map(|u| u.id);
        let (list, k) = container_mut(&mut self.doc.blocks, &p).unwrap();
        let pos = if let Some(n) = next.or(prev) {
            list.remove(k);
            if Some(n) == next { Pos::new(n, 0) } else { Pos::new(n, self.len_of(n)) }
        } else {
            let nb = Block::text("");
            let nid = nb.id;
            list[k] = nb;
            Pos::new(nid, 0)
        };
        self.normalize();
        self.sel = Selection::caret(pos);
        pos
    }

    // ---- public editing commands

    pub fn delete_selection(&mut self) {
        if !self.sel.is_collapsed() {
            self.checkpoint(EditKind::Other);
            self.delete_selection_inner();
        }
    }

    /// Insert text at the selection. `\n` in a text block is a soft break.
    pub fn insert_text(&mut self, s: &str) {
        self.checkpoint(EditKind::Typing);
        let mut p = self.delete_selection_inner();
        if self.text_of(p.block).is_none() {
            // Caret on an atomic block: typing goes into a new paragraph below.
            let b = Block::text("");
            let id = b.id;
            self.insert_after(p.block, b);
            p = Pos::new(id, 0);
        }
        let marks = match self.pending_marks.take() {
            Some(m) => m,
            None => self.text_of(p.block).map(|t| t.marks_at(p.offset)).unwrap_or_default(),
        };
        let marks = if self.is_code(p.block) || p.block == TITLE { Marks::default() } else { marks };
        if let Some(t) = self.text_mut(p.block) {
            t.insert_plain(p.offset, s, marks);
        }
        self.sel = Selection::caret(Pos::new(p.block, p.offset + s.len()));
    }

    /// Replace `range` in the caret block (IME / input handler path).
    pub fn replace_in_block(&mut self, id: BlockId, range: Range<usize>, s: &str) {
        self.checkpoint(EditKind::Typing);
        let marks = self.text_of(id).map(|t| t.marks_at(range.start)).unwrap_or_default();
        let marks = self.pending_marks.take().unwrap_or(marks);
        let marks = if self.is_code(id) || id == TITLE { Marks::default() } else { marks };
        if let Some(t) = self.text_mut(id) {
            let mut ins = RichText::default();
            ins.push(s, marks);
            t.replace(range.start..range.end.min(t.len()), &ins);
        }
        self.sel = Selection::caret(Pos::new(id, range.start + s.len()));
    }

    /// Insert an equation at the caret; returns its position.
    pub fn insert_equation(&mut self, src: &str) -> Option<Pos> {
        self.insert_object(InlineKind::Equation(src.to_string()))
    }

    /// Insert a 按钮 at the caret; returns its position.
    pub fn insert_button(&mut self, b: &InlineButton) -> Option<Pos> {
        self.insert_object(InlineKind::Button(b.clone()))
    }

    /// Insert a 日期提醒 at the caret; returns its position.
    pub fn insert_reminder(&mut self, r: &InlineReminder) -> Option<Pos> {
        self.insert_object(InlineKind::Reminder(*r))
    }

    /// @文档: an inline link to a local document.
    pub fn insert_doc_link(&mut self, l: &InlineDocLink) -> Option<Pos> {
        self.insert_object(InlineKind::DocLink(l.clone()))
    }

    fn insert_object(&mut self, kind: InlineKind) -> Option<Pos> {
        self.checkpoint(EditKind::Other);
        self.break_coalescing();
        let p = self.delete_selection_inner();
        if self.is_code(p.block) || p.block == TITLE {
            return None;
        }
        let mut marks = self.text_of(p.block)?.marks_at(p.offset);
        marks.inline = Some(crate::docxml::to_inline(kind));
        let t = self.text_mut(p.block)?;
        let mut ins = RichText::default();
        ins.push(EQUATION, marks);
        t.replace(p.offset..p.offset, &ins);
        self.sel = Selection::caret(Pos::new(p.block, p.offset + EQUATION.len()));
        Some(p)
    }

    /// The inline object at `p`.
    pub fn object_at(&self, p: Pos) -> Option<&Inline> {
        let t = self.text_of(p.block)?;
        let (_, m) = t.spans().find(|(r, _)| r.start <= p.offset && p.offset < r.end)?;
        m.inline.as_ref()
    }

    /// Replace the object at `p` with one of `kind` (one undo step, except
    /// that an equation's live edits coalesce).
    fn set_object(&mut self, p: Pos, kind: InlineKind) {
        let obj = crate::docxml::to_inline(kind);
        if self.object_at(p).is_none_or(|old| old.kind == obj.kind) {
            return;
        }
        let keep = self.sel;
        self.sel = Selection::caret(p);
        if matches!(obj.kind, InlineKind::Equation(_)) {
            self.checkpoint(EditKind::Equation);
        } else {
            self.checkpoint(EditKind::Other);
            self.break_coalescing();
        }
        self.sel = keep;
        if let Some(t) = self.text_mut(p.block) {
            t.update_marks(p.offset..p.offset + EQUATION.len(), |m| m.inline = Some(obj.clone()));
        }
    }

    pub fn doc_link_at(&self, p: Pos) -> Option<InlineDocLink> {
        self.object_at(p)?.doc_link().cloned()
    }

    /// The LaTeX source of the equation at `p`.
    pub fn equation_source(&self, p: Pos) -> Option<String> {
        self.object_at(p)?.equation().map(str::to_string)
    }

    /// Replace the source of the equation at `p` (its editor's live updates
    /// coalesce into one undo step).
    pub fn set_equation_source(&mut self, p: Pos, src: &str) {
        if self.equation_source(p).is_some() {
            self.set_object(p, InlineKind::Equation(src.to_string()));
        }
    }

    /// The 按钮 at `p`.
    pub fn button_at(&self, p: Pos) -> Option<InlineButton> {
        self.object_at(p)?.button().cloned()
    }

    /// Replace the 按钮 at `p` (one undo step).
    pub fn set_button(&mut self, p: Pos, b: &InlineButton) {
        if self.button_at(p).is_some() {
            self.set_object(p, InlineKind::Button(b.clone()));
        }
    }

    /// The 日期提醒 at `p`.
    pub fn reminder_at(&self, p: Pos) -> Option<InlineReminder> {
        self.object_at(p)?.reminder().copied()
    }

    /// Replace the 日期提醒 at `p` (one undo step).
    pub fn set_reminder(&mut self, p: Pos, r: &InlineReminder) {
        if self.reminder_at(p).is_some() {
            self.set_object(p, InlineKind::Reminder(*r));
        }
    }

    /// The inline attachment (文本视图) at `p`.
    pub fn file_at(&self, p: Pos) -> Option<FileBlock> {
        self.object_at(p)?.file().cloned()
    }

    /// 卡片视图 / 预览视图 of an attachment block.
    pub fn set_file_view(&mut self, id: BlockId, view: crate::model::FileView) {
        if !matches!(self.kind_of(id), Some(BlockKind::File(f)) if f.view != view) {
            return;
        }
        self.checkpoint(EditKind::Other);
        self.break_coalescing();
        if let Some(BlockKind::File(f)) = self.block_mut(id).map(|b| &mut b.kind) {
            f.view = view;
        }
    }

    /// 文本视图: the block becomes a paragraph holding the attachment
    /// inline (same alignment); the caret lands after it.
    pub fn file_to_inline(&mut self, id: BlockId) {
        let Some(BlockKind::File(f)) = self.kind_of(id).cloned() else { return };
        self.checkpoint(EditKind::Other);
        self.break_coalescing();
        let Some(b) = self.block_mut(id) else { return };
        let mut marks = crate::model::Marks::default();
        marks.inline = Some(crate::docxml::to_inline(InlineKind::File(f)));
        let mut t = RichText::default();
        t.push(EQUATION, marks);
        b.kind = BlockKind::Text;
        b.text = t;
        self.block_sel = None;
        self.sel = Selection::caret(Pos::new(id, EQUATION.len()));
    }

    /// 卡片视图 / 预览视图 of an inline attachment: it moves out of its
    /// paragraph into a block right after it (taking the paragraph's place
    /// when nothing else is left in it).
    pub fn inline_to_file(&mut self, p: Pos, view: crate::model::FileView) -> Option<BlockId> {
        let mut f = self.file_at(p)?;
        f.view = view;
        self.checkpoint(EditKind::Other);
        self.break_coalescing();
        let t = self.text_mut(p.block)?;
        t.replace(p.offset..p.offset + EQUATION.len(), &RichText::default());
        let b = self.block(p.block)?;
        let empty = matches!(b.kind, BlockKind::Text) && b.text.is_empty() && b.children.is_empty();
        let id = if empty {
            let b = self.block_mut(p.block)?;
            b.kind = BlockKind::File(f);
            b.id
        } else {
            let nb = Block::new(BlockKind::File(f), RichText::default());
            let id = nb.id;
            self.insert_after(p.block, nb);
            id
        };
        self.block_sel = None;
        self.sel = Selection::caret(Pos::new(id, 0));
        Some(id)
    }

    /// Insert `blocks` in a row like `insert_block` (one undo step); the
    /// last one's id.
    pub fn insert_blocks(&mut self, blocks: Vec<Block>) -> Option<BlockId> {
        let mut it = blocks.into_iter();
        let mut last = self.insert_block(it.next()?);
        for b in it {
            let id = b.id;
            self.insert_after(last, b);
            last = id;
        }
        Some(last)
    }

    pub fn is_code(&self, id: BlockId) -> bool {
        self.block(id).is_some_and(|b| matches!(b.kind, BlockKind::Code { .. }))
    }

    pub fn kind_of(&self, id: BlockId) -> Option<&BlockKind> {
        self.block(id).map(|b| &b.kind)
    }

    /// Enter.
    pub fn enter(&mut self) {
        self.checkpoint(EditKind::Other);
        self.break_coalescing();
        let p = self.delete_selection_inner();
        if p.block == TITLE {
            // Enter in the title moves into the body, splitting the title text.
            let tail = self.doc.title.split_off(p.offset);
            let first = self.doc.blocks.first().map(|b| b.id);
            let first_empty = self.doc.blocks.first().is_some_and(|b| matches!(b.kind, BlockKind::Text) && b.text.is_empty() && b.children.is_empty());
            if tail.is_empty() && first_empty {
                self.sel = Selection::caret(Pos::new(first.unwrap(), 0));
            } else {
                let b = Block::new(BlockKind::Text, tail);
                let id = b.id;
                self.doc.blocks.insert(0, b);
                self.sel = Selection::caret(Pos::new(id, 0));
            }
            return;
        }
        let Some(block) = self.block(p.block).cloned() else {
            // Atomic: add a paragraph after it.
            let b = Block::text("");
            let id = b.id;
            self.insert_after(p.block, b);
            self.sel = Selection::caret(Pos::new(id, 0));
            return;
        };
        if !block.kind.has_text() {
            let b = Block::text("");
            let id = b.id;
            self.insert_after(p.block, b);
            self.sel = Selection::caret(Pos::new(id, 0));
            return;
        }
        if let BlockKind::Code { .. } = block.kind {
            // The new line keeps the current line's indentation.
            let before = &block.text.text[..p.offset];
            let line = &before[before.rfind('\n').map_or(0, |i| i + 1)..];
            let indent: String = line.chars().take_while(|c| *c == ' ' || *c == '\t').collect();
            let ins = format!("\n{indent}");
            self.text_mut(p.block).unwrap().insert_plain(p.offset, &ins, Marks::default());
            self.sel = Selection::caret(Pos::new(p.block, p.offset + ins.len()));
            return;
        }
        let empty = block.text.is_empty();
        // Empty list item / todo: outdent if nested, otherwise turn into text.
        if empty && block.kind.is_list() {
            if self.parent_is_text_block(p.block) {
                self.outdent_one(p.block);
            } else {
                self.block_mut(p.block).unwrap().kind = BlockKind::Text;
            }
            self.sel = Selection::caret(Pos::new(p.block, 0));
            return;
        }
        // Empty last paragraph inside a quote/callout exits the container.
        if empty && matches!(block.kind, BlockKind::Text) && block.children.is_empty() {
            if let Some(parent) = self.parent_of(p.block).cloned()
                && matches!(parent.kind, BlockKind::Quote | BlockKind::Callout { .. })
                && parent.children.last().map(|c| c.id) == Some(p.block)
                && parent.children.len() > 1
            {
                let b = self.remove_subtree(p.block).unwrap();
                self.insert_after(parent.id, b);
                self.sel = Selection::caret(Pos::new(p.block, 0));
                return;
            }
            if self.parent_is_text_block(p.block) {
                self.outdent_one(p.block);
                self.sel = Selection::caret(Pos::new(p.block, 0));
                return;
            }
        }
        let new_kind = match &block.kind {
            // Below a folded heading's section a paragraph would still be in
            // it, hidden; a heading of the same level ends the section.
            BlockKind::Heading { level, .. } if block.folded => BlockKind::Heading { level: *level, seq_auto: false },
            BlockKind::Heading { .. } => BlockKind::Text,
            BlockKind::Ordered { .. } => BlockKind::Ordered { seq: None },
            BlockKind::Todo { .. } => BlockKind::Todo { done: false },
            k => k.clone(),
        };
        if p.offset == 0 && !empty {
            // Caret at start: push an empty block of the same kind above.
            let kind = match &block.kind {
                BlockKind::Heading { .. } => BlockKind::Text,
                _ => new_kind,
            };
            let mut nb = Block::new(kind, RichText::default());
            nb.align = block.align;
            self.insert_before(p.block, nb);
            self.sel = Selection::caret(Pos::new(p.block, 0));
            return;
        }
        let tail = self.text_mut(p.block).unwrap().split_off(p.offset);
        let mut nb = Block::new(
            if tail.is_empty() { new_kind } else { block.kind.clone().reset_for_split() },
            tail,
        );
        nb.align = block.align;
        if !nb.text.is_empty() {
            nb.bg = block.bg.clone();
        }
        let nid = nb.id;
        let has_kids = !block.children.is_empty() && !block.folded;
        if has_kids {
            // New item becomes the first child, like an outliner.
            self.block_mut(p.block).unwrap().children.insert(0, nb);
        } else {
            // A folded heading keeps its section: the new block goes below it.
            let after = self.fold_end(p.block);
            self.insert_after(after, nb);
        }
        self.sel = Selection::caret(Pos::new(nid, 0));
    }

    fn parent_is_text_block(&self, id: BlockId) -> bool {
        self.parent_of(id).is_some_and(|p| p.kind.has_text())
    }

    /// Backspace.
    pub fn backspace(&mut self) {
        if !self.sel.is_collapsed() {
            self.checkpoint(EditKind::Delete);
            self.delete_selection_inner();
            return;
        }
        let p = self.sel.head;
        if p.offset > 0 {
            self.checkpoint(EditKind::Delete);
            let t = self.text_mut(p.block).unwrap();
            let start = prev_grapheme(&t.text, p.offset);
            t.replace(start..p.offset, &RichText::default());
            self.sel = Selection::caret(Pos::new(p.block, start));
            return;
        }
        if p.block == TITLE {
            return;
        }
        let Some(block) = self.block(p.block).cloned() else {
            self.checkpoint(EditKind::Other);
            self.delete_atomic(p.block);
            return;
        };
        if !block.kind.has_text() {
            self.checkpoint(EditKind::Other);
            self.delete_atomic(p.block);
            return;
        }
        self.checkpoint(EditKind::Other);
        self.break_coalescing();
        match block.kind {
            BlockKind::Text => {}
            BlockKind::Code { .. } => {
                if block.text.is_empty() {
                    self.block_mut(p.block).unwrap().kind = BlockKind::Text;
                }
                return;
            }
            _ => {
                // Non-text block at offset 0 first degrades to a paragraph.
                let b = self.block_mut(p.block).unwrap();
                b.kind = BlockKind::Text;
                return;
            }
        }
        // Indented paragraph: outdent.
        if self.parent_is_text_block(p.block) {
            self.outdent_one(p.block);
            return;
        }
        // First paragraph in a quote/callout: lift it out.
        if let Some(parent) = self.parent_of(p.block).cloned()
            && matches!(parent.kind, BlockKind::Quote | BlockKind::Callout { .. })
            && parent.children.first().map(|c| c.id) == Some(p.block)
        {
            let b = self.remove_subtree(p.block).unwrap();
            self.insert_before(parent.id, b);
            self.normalize();
            return;
        }
        self.merge_into_previous(p.block);
    }

    fn merge_into_previous(&mut self, id: BlockId) {
        let us = self.units();
        let Some(i) = self.unit_index(&us, id) else { return };
        if i == 0 {
            return;
        }
        let cur = us[i];
        let prev = us[i - 1];
        // Backspace never leaves a cell. An empty first line goes
        // when a paragraph follows it in the cell, which takes the caret.
        // Once the cell is empty the table gets selected, so holding
        // Backspace inside it ends up removing the table.
        if let Some(mut p) = self.path(id)
            && let Some(&Seg::Cell(r, c, 0)) = p.last()
        {
            if self.len_of(id) == 0 && self.block(id).is_some_and(|b| b.children.is_empty() && matches!(b.kind, BlockKind::Text)) {
                let table = self.table_scope_at(&p);
                *p.last_mut().unwrap() = Seg::Cell(r, c, 1);
                match block_at(&self.doc.blocks, &p).map(|b| (b.id, b.kind.has_text())) {
                    Some((next, true)) => {
                        self.remove_subtree(id);
                        self.normalize();
                        self.sel = Selection::caret(Pos::new(next, 0));
                    }
                    Some(_) => {}
                    None => {
                        if let Some(t) = table {
                            self.select_block(t);
                        }
                    }
                }
            }
            return;
        }
        if prev.scope != cur.scope {
            // Previous unit is a cell of a table right before this block:
            // an empty paragraph disappears, then the table gets selected so
            // the next Backspace removes it.
            if let Some(t) = self.enclosing_table_in(prev.id, cur.scope) {
                if self.len_of(id) == 0 && self.block(id).is_some_and(|b| b.children.is_empty() && matches!(b.kind, BlockKind::Text)) {
                    self.remove_subtree(id);
                    self.normalize();
                    self.sel = Selection::caret(Pos::new(prev.id, self.len_of(prev.id)));
                } else {
                    self.sel = Selection::caret(Pos::new(id, 0));
                }
                self.select_block(t);
            }
            return;
        }
        if !prev.text {
            // Previous is atomic (divider, image...): an empty paragraph just
            // disappears, otherwise the atomic block is removed.
            if self.len_of(id) == 0 && self.block(id).is_some_and(|b| b.children.is_empty()) {
                self.remove_subtree(id);
                self.normalize();
                self.sel = Selection::caret(Pos::new(prev.id, 0));
            } else {
                self.remove_subtree(prev.id);
                self.normalize();
                self.sel = Selection::caret(Pos::new(id, 0));
            }
            return;
        }
        if self.is_code(prev.id) {
            // Don't merge text into code; just move the caret.
            self.sel = Selection::caret(Pos::new(prev.id, self.len_of(prev.id)));
            return;
        }
        let text = self.text_of(id).cloned().unwrap_or_default();
        let kids = self.block_mut(id).map(|b| std::mem::take(&mut b.children)).unwrap_or_default();
        let at = self.len_of(prev.id);
        self.text_mut(prev.id).unwrap().append(&text);
        self.remove_subtree(id);
        match self.block_mut(prev.id) {
            Some(pb) if pb.kind.nests() => pb.children.extend(kids),
            _ => self.insert_all_after(prev.id, kids),
        }
        self.normalize();
        self.sel = Selection::caret(Pos::new(prev.id, at));
    }

    /// Forward delete.
    pub fn delete_forward(&mut self) {
        if !self.sel.is_collapsed() {
            self.checkpoint(EditKind::Delete);
            self.delete_selection_inner();
            return;
        }
        let p = self.sel.head;
        let Some(len) = self.text_of(p.block).map(|t| t.len()) else {
            self.checkpoint(EditKind::Other);
            self.delete_atomic(p.block);
            return;
        };
        if p.offset < len {
            self.checkpoint(EditKind::Delete);
            let t = self.text_mut(p.block).unwrap();
            let end = next_grapheme(&t.text, p.offset);
            t.replace(p.offset..end, &RichText::default());
            return;
        }
        let us = self.units();
        let Some(i) = self.unit_index(&us, p.block) else { return };
        let Some(next) = us.get(i + 1).copied() else { return };
        if next.scope != us[i].scope {
            // A table follows: select it, so the next Delete removes it.
            if let Some(t) = self.enclosing_table_in(next.id, us[i].scope) {
                self.checkpoint(EditKind::Other);
                if p.block != TITLE
                    && len == 0
                    && self.block(p.block).is_some_and(|b| b.children.is_empty() && matches!(b.kind, BlockKind::Text))
                {
                    self.remove_subtree(p.block);
                    self.normalize();
                    self.sel = Selection::caret(Pos::new(next.id, 0));
                }
                self.select_block(t);
            }
            return;
        }
        // Nor does Delete pull the next cell in.
        if self.path(next.id).is_some_and(|q| matches!(q.last(), Some(Seg::Cell(_, _, 0)))) {
            return;
        }
        self.checkpoint(EditKind::Other);
        if !next.text {
            self.remove_subtree(next.id);
            self.normalize();
            self.sel = Selection::caret(p);
            return;
        }
        if self.is_code(next.id) || self.is_code(p.block) {
            return;
        }
        self.merge_into_previous(next.id);
        self.sel = Selection::caret(p);
    }

    /// Delete to start of the visual line is GUI-dependent; this deletes to block start.
    pub fn delete_to_start(&mut self) {
        let p = self.sel.head;
        if self.sel.is_collapsed() && p.offset > 0 {
            self.sel.anchor = Pos::new(p.block, 0);
        }
        self.backspace();
    }

    pub fn delete_word_back(&mut self) {
        let p = self.sel.head;
        if self.sel.is_collapsed() && p.offset > 0 {
            let t = &self.text_of(p.block).unwrap().text;
            self.sel.anchor = Pos::new(p.block, prev_word(t, p.offset));
        }
        self.backspace();
    }

    // ---- indentation

    /// Blocks touched by the selection that are "top-most" (no ancestor also selected).
    pub fn selected_blocks(&self) -> Vec<BlockId> {
        if self.block_sel.is_some() {
            return self.block_sel_ids();
        }
        if self.cell_sel.is_some() {
            return self.cell_blocks();
        }
        let us = self.units();
        let (s, e) = self.ordered_in(&us);
        let si = self.unit_index(&us, s.block).unwrap_or(0);
        let ei = self.unit_index(&us, e.block).unwrap_or(0);
        let ids: Vec<BlockId> = us[si..=ei].iter().map(|u| u.id).filter(|id| *id != TITLE).collect();
        self.topmost_in(&ids, &path_index(&self.doc.blocks))
    }

    pub fn indent(&mut self) {
        let ids = self.selected_blocks();
        if ids.is_empty() {
            return;
        }
        self.checkpoint(EditKind::Other);
        for id in ids {
            self.indent_one(id);
        }
    }

    /// Whether Tab would nest `id` under its previous sibling.
    pub fn can_indent(&self, id: BlockId) -> bool {
        let Some(p) = self.path(id) else { return false };
        let i = seg_index(*p.last().unwrap());
        if i == 0 {
            return false;
        }
        let mut prev_path = p.clone();
        *prev_path.last_mut().unwrap() = with_index(*p.last().unwrap(), i - 1);
        let Some(prev) = block_at(&self.doc.blocks, &prev_path) else { return false };
        let can_parent = prev.kind.nests();
        let me = block_at(&self.doc.blocks, &p).unwrap();
        can_parent && !matches!(me.kind, BlockKind::Column { .. }) && nest_depth(&p) < MAX_NEST_DEPTH
    }

    /// Whether Shift+Tab would lift `id` out of its parent.
    pub fn can_outdent(&self, id: BlockId) -> bool {
        let Some(p) = self.path(id) else { return false };
        let Some(pp) = parent_path(&p) else { return false };
        let Some(parent) = block_at(&self.doc.blocks, pp) else { return false };
        parent.kind.has_text() && !matches!(p.last(), Some(Seg::Cell(..)))
    }

    fn indent_one(&mut self, id: BlockId) -> bool {
        let Some(p) = self.path(id) else { return false };
        let i = seg_index(*p.last().unwrap());
        if i == 0 {
            return false;
        }
        let mut prev_path = p.clone();
        *prev_path.last_mut().unwrap() = with_index(*p.last().unwrap(), i - 1);
        let Some(prev) = block_at(&self.doc.blocks, &prev_path) else { return false };
        // Nestable parents: text, headings and the list kinds.
        let can_parent = matches!(
            prev.kind,
            BlockKind::Text
                | BlockKind::Heading { .. }
                | BlockKind::Bullet
                | BlockKind::Ordered { .. }
                | BlockKind::Todo { .. }
        );
        let me = block_at(&self.doc.blocks, &p).unwrap();
        if !can_parent || matches!(me.kind, BlockKind::Column { .. }) || nest_depth(&p) >= MAX_NEST_DEPTH {
            return false;
        }
        let b = self.remove_subtree(id).unwrap();
        let prev = block_at_mut(&mut self.doc.blocks, &prev_path).unwrap();
        prev.children.push(b);
        true
    }

    pub fn outdent(&mut self) {
        let ids = self.selected_blocks();
        if ids.is_empty() {
            return;
        }
        self.checkpoint(EditKind::Other);
        for id in ids.into_iter().rev() {
            self.outdent_one(id);
        }
    }

    /// Move a nested block after its parent; following siblings become its children.
    fn outdent_one(&mut self, id: BlockId) -> bool {
        let Some(p) = self.path(id) else { return false };
        let Some(pp) = parent_path(&p) else { return false };
        let Some(parent) = block_at(&self.doc.blocks, pp) else { return false };
        if !parent.kind.has_text() || matches!(p.last(), Some(Seg::Cell(..))) {
            return false;
        }
        let parent_id = parent.id;
        let i = seg_index(*p.last().unwrap());
        let pb = block_at_mut(&mut self.doc.blocks, pp).unwrap();
        let mut b = pb.children.remove(i);
        let following: Vec<Block> = pb.children.drain(i..).collect();
        // They stay below it: as its children, or after it when it can't hold any (code).
        let id = b.id;
        if b.kind.nests() {
            b.children.extend(following);
            self.insert_after(parent_id, b);
        } else {
            self.insert_after(parent_id, b);
            self.insert_all_after(id, following);
        }
        true
    }

    /// Numbering is scoped to siblings, including lists inside cells and containers.
    pub fn ordered_number_options(&self, id: BlockId) -> Option<(u32, Option<u32>)> {
        let path = self.path(id)?;
        let (list, i) = siblings_at(&self.doc.blocks, &path)?;
        if !matches!(list[i].kind, BlockKind::Ordered { .. }) { return None }
        let previous = (0..i).rev().find(|j| matches!(list[*j].kind, BlockKind::Ordered { .. }));
        Some((ordered_number(list, i), previous.map(|j| ordered_number(list, j).saturating_add(1))))
    }

    pub fn set_ordered_number(&mut self, id: BlockId, value: u32, continue_previous: bool) {
        if value == 0 || self.ordered_number_options(id).is_none() { return }
        let seq = if continue_previous {
            let Some(path) = self.path(id) else { return };
            let Some((list, i)) = siblings_at(&self.doc.blocks, &path) else { return };
            if i > 0 && matches!(list[i - 1].kind, BlockKind::Ordered { .. }) { None } else { Some(value) }
        } else { Some(value) };
        self.checkpoint(EditKind::Other);
        if let Some(b) = self.block_mut(id) { b.kind = BlockKind::Ordered { seq }; }
        self.break_coalescing();
    }

    // ---- block kind changes

    pub fn turn_into(&mut self, ids: &[BlockId], kind: BlockKind) {
        let ids: Vec<BlockId> = ids.iter().copied().filter(|id| *id != TITLE).collect();
        if ids.is_empty() {
            return;
        }
        self.checkpoint(EditKind::Other);
        self.break_coalescing();
        if matches!(kind, BlockKind::Quote | BlockKind::Callout { .. }) {
            self.wrap_blocks(&ids, kind);
        } else {
            for id in &ids {
                self.turn_one(*id, kind.clone());
            }
        }
        self.normalize();
        self.fix_selection();
    }

    /// Keyboard turn-into toggles: when every target already has the kind,
    /// it goes back to plain text (or leaves the quote).
    pub fn toggle_turn_into(&mut self, ids: &[BlockId], kind: BlockKind) {
        let same = |k: &BlockKind| match (k, &kind) {
            (BlockKind::Heading { level: a, .. }, BlockKind::Heading { level: b, .. }) => a == b,
            (a, b) => std::mem::discriminant(a) == std::mem::discriminant(b),
        };
        let already = !ids.is_empty()
            && ids.iter().all(|id| match kind {
                BlockKind::Quote => self.parent_of(*id).is_some_and(|p| matches!(p.kind, BlockKind::Quote)),
                _ => self.kind_of(*id).is_some_and(same),
            });
        if !already {
            self.turn_into(ids, kind);
        } else if matches!(kind, BlockKind::Quote) {
            self.checkpoint(EditKind::Other);
            for id in ids {
                self.lift_out_of_container(*id);
            }
            self.normalize();
            self.fix_selection();
        } else {
            self.turn_into(ids, BlockKind::Text);
        }
    }

    /// Move a container's children out in place of it.
    pub fn unwrap_container(&mut self, children: &[BlockId]) {
        self.checkpoint(EditKind::Other);
        for c in children {
            self.lift_out_of_container(*c);
        }
        self.normalize();
        self.fix_selection();
    }

    /// Wrap runs of adjacent sibling blocks into one quote / callout each.
    fn wrap_blocks(&mut self, ids: &[BlockId], kind: BlockKind) {
        let disc = std::mem::discriminant(&kind);
        // Blocks already inside the same kind of container stay put.
        let ids: Vec<BlockId> = ids
            .iter()
            .copied()
            .filter(|id| !self.parent_of(*id).is_some_and(|p| std::mem::discriminant(&p.kind) == disc))
            .filter(|id| !(disc == std::mem::discriminant(&BlockKind::Quote) && self.kind_of(*id).is_some_and(|k| matches!(k, BlockKind::Quote | BlockKind::Table(_)))))
            .collect();
        let mut groups: Vec<Vec<BlockId>> = vec![];
        let mut last: Option<Path> = None;
        for id in ids {
            let Some(p) = self.path(id) else { continue };
            let adjacent = last.as_ref().is_some_and(|l| {
                l.len() == p.len()
                    && l[..l.len() - 1] == p[..p.len() - 1]
                    && seg_index(*l.last().unwrap()) + 1 == seg_index(*p.last().unwrap())
            });
            if adjacent {
                groups.last_mut().unwrap().push(id);
            } else {
                groups.push(vec![id]);
            }
            last = Some(p);
        }
        for g in groups {
            let p = self.path(g[0]).unwrap();
            let (list, i) = container_mut(&mut self.doc.blocks, &p).unwrap();
            let inner: Vec<Block> = list.drain(i..i + g.len()).collect();
            list.insert(i, Block::new(kind.clone(), RichText::default()).with_children(inner));
        }
    }

    /// Move a block out of its quote / callout, splitting the container when
    /// the block sits in the middle.
    fn lift_out_of_container(&mut self, id: BlockId) {
        let Some(parent) = self.parent_of(id).cloned() else { return };
        if !matches!(parent.kind, BlockKind::Quote | BlockKind::Callout { .. }) {
            return;
        }
        let i = parent.children.iter().position(|c| c.id == id).unwrap();
        let pp = self.path(parent.id).unwrap();
        let (list, k) = container_mut(&mut self.doc.blocks, &pp).unwrap();
        let mut before = list.remove(k);
        let kind = before.kind.clone();
        let mut after_kids = before.children.split_off(i);
        let me = after_kids.remove(0);
        let mut at = k;
        if !before.children.is_empty() {
            list.insert(at, before);
            at += 1;
        }
        list.insert(at, me);
        if !after_kids.is_empty() {
            let mut rest = Block::new(kind, RichText::default());
            rest.children = after_kids;
            list.insert(at + 1, rest);
        }
    }

    fn turn_one(&mut self, id: BlockId, kind: BlockKind) {
        if id == TITLE {
            return;
        }
        match kind {
            BlockKind::Quote | BlockKind::Callout { .. } => {
                // Already inside such a container? Then nothing to do.
                if self.parent_of(id).is_some_and(|p| std::mem::discriminant(&p.kind) == std::mem::discriminant(&kind)) {
                    return;
                }
                let Some(p) = self.path(id) else { return };
                let (list, i) = container_mut(&mut self.doc.blocks, &p).unwrap();
                let inner = list.remove(i);
                let wrapper = Block::new(kind, RichText::default()).with_children(vec![inner]);
                list.insert(i, wrapper);
            }
            k => {
                // Turning a block inside a quote/callout into plain text keeps it there.
                let Some(b) = self.block_mut(id) else { return };
                if !b.kind.has_text() {
                    return;
                }
                if let BlockKind::Code { .. } = k {
                    b.text = RichText::plain(b.text.plain_text());
                }
                // A folded heading hides its section, any other fold just the children: drop the fold
                // rather than hide or reveal the blocks below.
                if matches!(b.kind, BlockKind::Heading { .. }) != matches!(k, BlockKind::Heading { .. }) {
                    b.folded = false;
                }
                // Headings, lists and text can all hold children, so the
                // subtree stays attached; a code block can't, its children follow it instead.
                b.kind = k;
                if !b.kind.nests() {
                    let kids = std::mem::take(&mut b.children);
                    self.insert_all_after(id, kids);
                }
            }
        }
    }

    pub fn set_align(&mut self, ids: &[BlockId], align: Align) {
        self.checkpoint(EditKind::Other);
        for id in ids {
            if let Some(b) = self.block_mut(*id) {
                b.align = align;
            }
        }
    }

    pub fn set_block_bg(&mut self, ids: &[BlockId], bg: Option<ColorName>) {
        self.checkpoint(EditKind::Other);
        for id in ids {
            if let Some(b) = self.block_mut(*id)
                && b.kind.takes_bg()
            {
                b.bg = bg.clone();
            }
        }
    }

    pub fn toggle_todo(&mut self, id: BlockId) {
        self.checkpoint(EditKind::Other);
        if let Some(b) = self.block_mut(id)
            && let BlockKind::Todo { done } = &mut b.kind
        {
            *done = !*done;
        }
    }

    /// What folds: headings always; paragraphs with children; list
    /// and todo items with children (or still marked folded). Never inside a
    /// table or a callout.
    pub fn can_fold(&self, id: BlockId) -> bool {
        let Some(p) = self.path(id) else { return false };
        if p.iter().any(|s| matches!(s, Seg::Cell(..)))
            || (1..p.len()).any(|k| block_at(&self.doc.blocks, &p[..k]).is_some_and(|a| matches!(a.kind, BlockKind::Callout { .. })))
        {
            return false;
        }
        let Some(b) = block_at(&self.doc.blocks, &p) else { return false };
        match b.kind {
            BlockKind::Heading { .. } => true,
            BlockKind::Text => !b.children.is_empty(),
            BlockKind::Bullet | BlockKind::Ordered { .. } | BlockKind::Todo { .. } => !b.children.is_empty() || b.folded,
            _ => false,
        }
    }

    /// `folded` is part of the document (a single transaction).
    pub fn toggle_fold(&mut self, id: BlockId) {
        self.generation += 1;
        self.dirty = true;
        if let Some(b) = self.block_mut(id) {
            b.folded = !b.folded;
        }
        self.fix_selection_after_fold(id);
    }

    /// The last block a folded heading hides (itself when nothing is hidden),
    /// so content added "after" it lands below the folded section.
    pub fn fold_end(&self, id: BlockId) -> BlockId {
        let Some(b) = self.block(id) else { return id };
        let BlockKind::Heading { level, .. } = b.kind else { return id };
        if !b.folded {
            return id;
        }
        let Some(p) = self.path(id) else { return id };
        let Some((list, i)) = siblings_at(&self.doc.blocks, &p) else { return id };
        list[i + 1..]
            .iter()
            .take_while(|s| !matches!(s.kind, BlockKind::Heading { level: l, .. } if l <= level))
            .last()
            .map(|s| s.id)
            .unwrap_or(id)
    }

    /// Unfolds whatever hides `id`: folded ancestors, and folded headings
    /// above it in the same list.
    pub fn unfold_to(&mut self, id: BlockId) {
        self.generation += 1;
        // Fold state is saved with the document, so opening one is an edit.
        fn go(list: &mut [Block], id: BlockId, changed: &mut bool) {
            let Some(i) = list.iter().position(|b| find_path(std::slice::from_ref(b), id).is_some()) else { return };
            loop {
                let mut hider: Option<(u8, usize)> = None;
                for (j, b) in list[..i].iter().enumerate() {
                    if let BlockKind::Heading { level, .. } = b.kind {
                        if hider.is_some_and(|(l, _)| level <= l) {
                            hider = None;
                        }
                        if hider.is_none() && b.folded {
                            hider = Some((level, j));
                        }
                    }
                }
                if let (Some((l, _)), BlockKind::Heading { level, .. }) = (hider, &list[i].kind)
                    && *level <= l
                {
                    hider = None;
                }
                match hider {
                    Some((_, j)) => {
                        list[j].folded = false;
                        *changed = true;
                    }
                    None => break,
                }
            }
            if list[i].id != id {
                let b = &mut list[i];
                *changed |= b.folded;
                b.folded = false;
                go(&mut b.children, id, changed);
                if let BlockKind::Table(t) = &mut b.kind {
                    t.rows.iter_mut().flatten().for_each(|cell| go(&mut cell.blocks, id, changed));
                }
            }
        }
        let mut changed = false;
        go(&mut self.doc.blocks, id, &mut changed);
        self.dirty |= changed;
    }

    /// Whether folding hides `id` (block_order's rule, without walking the
    /// whole document): a folded ancestor, or a folded heading above it or
    /// above an ancestor with no heading at its level or higher in between.
    pub fn is_hidden(&self, id: BlockId) -> bool {
        let Some(p) = self.path(id) else { return false };
        for n in 1..=p.len() {
            let Some((list, i)) = siblings_at(&self.doc.blocks, &p[..n]) else { return false };
            if n < p.len() && list[i].folded {
                return true;
            }
            let mut stop = match list[i].kind {
                BlockKind::Heading { level, .. } => level,
                _ => u8::MAX,
            };
            for b in list[..i].iter().rev() {
                if let BlockKind::Heading { level, .. } = b.kind
                    && level < stop
                {
                    if b.folded {
                        return true;
                    }
                    stop = level;
                }
            }
        }
        false
    }

    /// Unfolds whatever hides either end of the (block) selection, so no
    /// command leaves it out of sight (indenting under a folded section,
    /// lifting a folded heading out of its parent, ...).
    pub fn reveal_caret(&mut self) {
        let ends = match self.block_sel {
            Some(bs) => [bs.anchor, bs.head],
            None => [self.sel.anchor.block, self.sel.head.block],
        };
        for id in ends {
            if id != TITLE && self.is_hidden(id) {
                self.unfold_to(id);
            }
        }
    }

    fn fix_selection_after_fold(&mut self, id: BlockId) {
        let us = self.units();
        if let Some(bs) = self.block_sel {
            // A block selection reaching into what just got hidden is dropped.
            let shown = self.block_order(true);
            if !shown.contains(&bs.anchor) || !shown.contains(&bs.head) {
                self.block_sel = None;
                self.sel = Selection::caret(Pos::new(id, self.len_of(id)));
            }
        }
        if !us.iter().any(|u| u.id == self.sel.head.block) || !us.iter().any(|u| u.id == self.sel.anchor.block) {
            // The caret got hidden: move it to the end of the folded block.
            self.sel = Selection::caret(Pos::new(id, self.len_of(id)));
            self.fix_selection();
        }
    }

    /// Insert a new block after the caret block (or replace it when it's an
    /// empty paragraph), returning its id.
    pub fn insert_block(&mut self, b: Block) -> BlockId {
        self.checkpoint(EditKind::Other);
        let id = b.id;
        let at = self.sel.head.block;
        let replace = at != TITLE
            && self.block(at).is_some_and(|x| matches!(x.kind, BlockKind::Text) && x.text.is_empty() && x.children.is_empty());
        if replace {
            let p = self.path(at).unwrap();
            let (list, i) = container_mut(&mut self.doc.blocks, &p).unwrap();
            list[i] = b;
        } else {
            self.insert_after(at, b);
        }
        id
    }

    /// Make sure something editable follows `id` (used after inserting atomic blocks).
    pub fn ensure_paragraph_after(&mut self, id: BlockId) -> BlockId {
        let us = self.units();
        let i = self.unit_index(&us, id);
        if let Some(i) = i
            && let Some(n) = us.get(i + 1)
            && n.text
            && n.scope == us[i].scope
        {
            return n.id;
        }
        // For tables the next unit is a cell; check the block after the table instead.
        let b = Block::text("");
        let nid = b.id;
        self.insert_after(id, b);
        nid
    }

    /// A new empty paragraph right after `id` (undoable), returning its id.
    pub fn insert_paragraph_after(&mut self, id: BlockId) -> BlockId {
        self.checkpoint(EditKind::Other);
        let b = Block::text("");
        let nid = b.id;
        self.insert_after(id, b);
        nid
    }

    pub fn delete_block(&mut self, id: BlockId) {
        self.checkpoint(EditKind::Other);
        let us = self.units();
        let i = self.unit_index(&us, id).unwrap_or(0);
        self.remove_subtree(id);
        self.normalize();
        let us2 = self.units();
        let target = us2.get(i.min(us2.len() - 1).max(1)).or(us2.last()).map(|u| u.id).unwrap_or(TITLE);
        self.sel = Selection::caret(Pos::new(target, 0));
    }

    /// Move block `id` before/after `target` (drag and drop).
    pub fn move_block(&mut self, id: BlockId, target: BlockId, after: bool) {
        if id == target {
            return;
        }
        // Can't drop a block inside itself.
        if let (Some(p), Some(t)) = (self.path(id), self.path(target))
            && t.starts_with(&p)
        {
            return;
        }
        self.checkpoint(EditKind::Other);
        let Some(b) = self.remove_subtree(id) else { return };
        if after { self.insert_after(target, b) } else { self.insert_before(target, b) }
        self.normalize();
        self.fix_selection();
    }

    /// Drag and drop: move `ids` (top-most blocks, document order) next to
    /// `target` or in as its first children. False when nothing would change.
    pub fn move_blocks(&mut self, ids: &[BlockId], target: BlockId, place: DropPlace) -> bool {
        let Some(tp) = self.path(target) else { return false };
        if ids.iter().any(|id| self.path(*id).is_none_or(|p| tp.starts_with(&p))) {
            return false;
        }
        // Only text blocks and the containers a drop can open hold children.
        if place == DropPlace::FirstChild
            && !self.kind_of(target).is_some_and(|k| k.nests() || matches!(k, BlockKind::Quote | BlockKind::Callout { .. } | BlockKind::Column { .. }))
        {
            return false;
        }
        let before = self.doc.blocks.clone();
        let blocks: Vec<Block> = ids.iter().filter_map(|id| self.remove_subtree(*id)).collect();
        let first = blocks.first().map(|b| b.id);
        match place {
            DropPlace::Before => blocks.into_iter().for_each(|b| self.insert_before(target, b)),
            DropPlace::After => self.insert_all_after(target, blocks),
            DropPlace::FirstChild => {
                if let Some(t) = self.block_mut(target) {
                    t.children.splice(0..0, blocks);
                }
            }
        }
        if self.doc.blocks == before || first.is_none() {
            self.doc.blocks = before;
            return false;
        }
        let after = std::mem::replace(&mut self.doc.blocks, before);
        self.checkpoint(EditKind::Other);
        self.doc.blocks = after;
        self.normalize();
        self.fix_selection();
        // A selection reaching into the moved blocks may now cross into or out of a table.
        let us = self.units();
        let scope = |id| us.iter().find(|u| u.id == id).map(|u| u.scope);
        if scope(self.sel.anchor.block) != scope(self.sel.head.block) {
            self.sel = Selection::caret(self.sel.head);
        }
        true
    }

    /// New blocks (dropped files) placed relative to `target`, as one edit.
    pub fn insert_blocks_at(&mut self, blocks: Vec<Block>, target: BlockId, place: DropPlace) {
        if blocks.is_empty() || self.path(target).is_none() {
            return;
        }
        self.checkpoint(EditKind::Other);
        match place {
            DropPlace::Before => blocks.into_iter().for_each(|b| self.insert_before(target, b)),
            DropPlace::After => self.insert_all_after(target, blocks),
            DropPlace::FirstChild => {
                if let Some(t) = self.block_mut(target) {
                    t.children.splice(0..0, blocks);
                }
            }
        }
        self.normalize();
    }

    pub fn column_ratios(&self, grid: BlockId) -> Vec<f32> {
        self.block(grid)
            .map(|g| g.children.iter().map(|c| if let BlockKind::Column { ratio } = c.kind { ratio } else { 0. }).collect())
            .unwrap_or_default()
    }

    /// The new column takes 1/(n+1), paid for by the
    /// columns that stay wider than 1/24 afterwards.
    pub fn add_column(&mut self, grid: BlockId, at: usize) -> Option<BlockId> {
        let n = self.block(grid)?.children.len();
        if n >= MAX_COLUMNS {
            return None;
        }
        self.checkpoint(EditKind::Other);
        let g = self.block_mut(grid)?;
        for c in g.children.iter_mut().filter(|c| c.children.is_empty()) {
            c.children.push(Block::text(""));
        }
        let m = 1. / (n as f32 + 1.);
        let ratio = |b: &Block| if let BlockKind::Column { ratio } = b.kind { ratio } else { 0. };
        let eligible = |r: f32| r * (1. - m) > MIN_COLUMN_RATIO;
        let o: f32 = g.children.iter().map(ratio).filter(|&r| eligible(r)).sum();
        let i = if m < o { m / o } else { 0. };
        for c in g.children.iter_mut() {
            if let BlockKind::Column { ratio } = &mut c.kind
                && eligible(*ratio)
            {
                *ratio *= 1. - i;
            }
        }
        let text = Block::text("");
        let tid = text.id;
        let col = Block::new(BlockKind::Column { ratio: m }, RichText::default()).with_children(vec![text]);
        g.children.insert(at.min(n), col);
        self.sel = Selection::caret(Pos::new(tid, 0));
        Some(tid)
    }

    /// Commit a column resize as one undo step.
    pub fn set_column_ratios(&mut self, grid: BlockId, ratios: &[f32]) {
        if self.column_ratios(grid) == ratios {
            return;
        }
        self.checkpoint(EditKind::Other);
        if let Some(g) = self.block_mut(grid) {
            for (c, &r) in g.children.iter_mut().zip(ratios) {
                if let BlockKind::Column { ratio } = &mut c.kind {
                    *ratio = r;
                }
            }
        }
    }

    /// Clicking below a column's content lands in its trailing empty
    /// paragraph, creating one if needed.
    pub fn focus_column_end(&mut self, col: BlockId) {
        let Some(c) = self.block(col) else { return };
        let reuse = c.children.last().filter(|b| matches!(b.kind, BlockKind::Text) && b.text.is_empty()).map(|b| b.id);
        let id = match reuse {
            Some(id) => id,
            None => {
                self.checkpoint(EditKind::Other);
                let b = Block::text("");
                let id = b.id;
                if let Some(c) = self.block_mut(col) {
                    c.children.push(b);
                }
                id
            }
        };
        self.sel = Selection::caret(Pos::new(id, 0));
    }

    // ---- marks

    pub fn toggle_mark(&mut self, get: fn(&Marks) -> bool, set: fn(&mut Marks, bool)) {
        if self.sel.is_collapsed() {
            let p = self.sel.head;
            let mut m = self.pending_marks.clone().unwrap_or_else(|| self.text_of(p.block).map(|t| t.marks_at(p.offset)).unwrap_or_default());
            let v = get(&m);
            set(&mut m, !v);
            self.pending_marks = Some(m);
            return;
        }
        let us = self.units();
        let targets: Vec<(BlockId, Range<usize>)> = us
            .iter()
            .filter(|u| u.text && u.id != TITLE && !self.is_code(u.id))
            .filter_map(|u| self.range_in(u.id, &us).map(|r| (u.id, r)))
            .filter(|(_, r)| !r.is_empty())
            .collect();
        if targets.is_empty() {
            return;
        }
        let all = targets.iter().all(|(id, r)| self.text_of(*id).unwrap().all_marks(r.clone(), get));
        self.checkpoint(EditKind::Other);
        for (id, r) in targets {
            self.text_mut(id).unwrap().update_marks(r, |m| set(m, !all));
        }
    }

    /// Apply a mark mutation to the selection (colors, links).
    pub fn apply_marks(&mut self, f: impl Fn(&mut Marks) + Copy) {
        let us = self.units();
        let targets: Vec<(BlockId, Range<usize>)> = us
            .iter()
            .filter(|u| u.text && u.id != TITLE && !self.is_code(u.id))
            .filter_map(|u| self.range_in(u.id, &us).map(|r| (u.id, r)))
            .filter(|(_, r)| !r.is_empty())
            .collect();
        if targets.is_empty() {
            return;
        }
        self.checkpoint(EditKind::Other);
        for (id, r) in targets {
            self.text_mut(id).unwrap().update_marks(r, f);
        }
    }

    /// Apply to whole blocks (block menu → 颜色).
    pub fn apply_marks_to_blocks(&mut self, ids: &[BlockId], f: impl Fn(&mut Marks) + Copy) {
        self.checkpoint(EditKind::Other);
        for id in ids {
            if let Some(t) = self.text_mut(*id) {
                let len = t.len();
                t.update_marks(0..len, f);
            }
        }
    }

    // ---- block selection (behavior-spec §4.4, §4.6, §5.4)

    pub fn in_block_mode(&self) -> bool {
        self.block_sel.is_some()
    }

    pub fn clear_block_sel(&mut self) {
        self.block_sel = None;
    }

    /// Enter block-selection mode on a single block (Esc, handle click).
    pub fn select_block(&mut self, id: BlockId) {
        if id == TITLE || self.block(id).is_none() {
            return;
        }
        self.block_sel = Some(BlockSel { anchor: id, head: id });
        self.cell_sel = None;
    }

    fn select_range(&mut self, anchor: BlockId, head: BlockId) {
        self.block_sel = Some(BlockSel { anchor, head });
        self.cell_sel = None;
    }

    /// Blocks in document order. With `visible`, content hidden by folded
    /// headings is skipped. Grid columns are layout only and never listed.
    pub fn block_order(&self, visible: bool) -> Vec<BlockId> {
        fn walk(blocks: &[Block], visible: bool, out: &mut Vec<BlockId>) {
            let mut hide_below: Option<u8> = None;
            for b in blocks {
                if visible && let Some(level) = hide_below {
                    match b.kind {
                        BlockKind::Heading { level: l, .. } if l <= level => hide_below = None,
                        _ => continue,
                    }
                }
                if !matches!(b.kind, BlockKind::Column { .. }) {
                    out.push(b.id);
                }
                if let BlockKind::Heading { level, .. } = b.kind
                    && b.folded
                {
                    hide_below = Some(level);
                }
                if !(visible && b.folded) {
                    walk(&b.children, visible, out);
                }
                if let BlockKind::Table(t) = &b.kind {
                    for row in &t.rows {
                        for cell in row {
                            walk(&cell.blocks, visible, out);
                        }
                    }
                }
            }
        }
        let mut out = vec![];
        walk(&self.doc.blocks, visible, &mut out);
        out
    }

    /// Nearest enclosing table, if any (block navigation stays inside it).
    /// Tab / Shift+Tab inside a table: caret to the end of the next / previous
    /// cell; Tab in the last cell appends a row. False outside tables.
    pub fn cell_step(&mut self, forward: bool) -> bool {
        let h = self.sel.head;
        let Some(p) = self.path(h.block) else { return false };
        let Some(i) = p.iter().rposition(|s| matches!(s, Seg::Cell(..))) else { return false };
        let Seg::Cell(r, c, _) = p[i] else { return false };
        let tpath = p[..i].to_vec();
        let Some(BlockKind::Table(t)) = block_at(&self.doc.blocks, &tpath).map(|b| &b.kind) else { return false };
        let cells: Vec<(usize, usize)> = t.cells().map(|(ri, ci, _)| (ri, ci)).collect();
        let Some(k) = cells.iter().position(|x| *x == (r, c)) else { return false };
        let target = if forward { cells.get(k + 1).copied() } else { k.checked_sub(1).map(|j| cells[j]) };
        let (tr, tc) = match target {
            Some(x) => x,
            None if forward => {
                self.checkpoint(EditKind::Other);
                let Some(Block { kind: BlockKind::Table(t), .. }) = block_at_mut(&mut self.doc.blocks, &tpath) else { return false };
                let n = t.rows.len();
                t.insert_row(n);
                (n, 0)
            }
            None => return true,
        };
        let Some(BlockKind::Table(t)) = block_at(&self.doc.blocks, &tpath).map(|b| &b.kind) else { return false };
        // The cell's last stop: its last block may be a container or hidden under a fold.
        let ids: Vec<BlockId> = t.rows[tr][tc].blocks.iter().map(|b| b.id).collect();
        let inside = inside_of(path_index(&self.doc.blocks), &ids);
        let Some(last) = units(&self.doc).into_iter().rev().find(|u| inside(u.id)).map(|u| u.id) else { return true };
        self.break_coalescing();
        self.sel = Selection::caret(Pos::new(last, self.len_of(last)));
        true
    }

    fn table_scope(&self, id: BlockId) -> Option<BlockId> {
        self.table_scope_at(&self.path(id)?)
    }

    fn table_scope_at(&self, p: &[Seg]) -> Option<BlockId> {
        let i = p.iter().rposition(|s| matches!(s, Seg::Cell(..)))?;
        block_at(&self.doc.blocks, &p[..i]).map(|b| b.id)
    }

    /// The table that holds `id` in its cells and itself sits in `scope`
    /// (the editing scope of a neighbouring block).
    fn enclosing_table_in(&self, id: BlockId, scope: Option<BlockId>) -> Option<BlockId> {
        let mut t = self.table_scope(id);
        while let Some(tid) = t {
            let outer = self.table_scope(tid);
            if outer == scope {
                return Some(tid);
            }
            t = outer;
        }
        None
    }

    /// `ids` without the blocks nested inside another of them.
    fn topmost_in(&self, ids: &[BlockId], paths: &HashMap<BlockId, Path>) -> Vec<BlockId> {
        let set: HashSet<&[Seg]> = ids.iter().filter_map(|id| paths.get(id)).map(|p| p.as_slice()).collect();
        ids.iter()
            .filter(|id| paths.get(id).is_some_and(|p| !(1..p.len()).any(|n| set.contains(&p[..n]))))
            .copied()
            .collect()
    }

    /// The selected blocks in document order, top-most only. Passing over a
    /// folded heading also takes the blocks it hides.
    pub fn block_sel_ids(&self) -> Vec<BlockId> {
        let Some(bs) = self.block_sel else { return vec![] };
        let vis = self.block_order(true);
        let (Some(a), Some(h)) = (vis.iter().position(|x| *x == bs.anchor), vis.iter().position(|x| *x == bs.head))
        else {
            return vec![];
        };
        let (s, e) = if a <= h { (a, h) } else { (h, a) };
        let all = self.block_order(false);
        let first = all.iter().position(|x| *x == vis[s]).unwrap();
        // Extend the end over anything hidden behind the last visible block.
        let next_vis = vis.get(e + 1).and_then(|n| all.iter().position(|x| x == n)).unwrap_or(all.len());
        let paths = path_index(&self.doc.blocks);
        let last_path = &paths[&vis[e]];
        let mut end = all.iter().position(|x| *x == vis[e]).unwrap();
        while end + 1 < next_vis {
            // Only absorb hidden blocks; stop at anything outside the last
            // block's container (e.g. leaving a table cell).
            if !covers(&last_path[..last_path.len() - 1], &paths[&all[end + 1]]) {
                break;
            }
            end += 1;
        }
        self.topmost_in(&all[first..=end], &paths)
    }

    /// ↑ / ↓ (and ⌃P / ⌃N) in block mode. `extend` is Shift.
    /// ↓ past the last block focuses the trailing empty line, adding one if
    /// needed (the only way out of a final code block or table).
    pub fn trailing_line(&mut self) -> BlockId {
        if let Some(last) = self.doc.blocks.last()
            && matches!(last.kind, BlockKind::Text)
            && last.text.is_empty()
            && last.children.is_empty()
        {
            return last.id;
        }
        self.checkpoint(EditKind::Other);
        let b = Block::text("");
        let id = b.id;
        self.doc.blocks.push(b);
        id
    }

    pub fn block_sel_move(&mut self, down: bool, extend: bool) {
        let Some(bs) = self.block_sel else { return };
        let scope = self.table_scope(bs.head);
        let paths = path_index(&self.doc.blocks);
        let vis: Vec<BlockId> = self
            .block_order(true)
            .into_iter()
            .filter(|id| paths.get(id).and_then(|p| self.table_scope_at(p)) == scope)
            .collect();
        let Some(i) = vis.iter().position(|x| *x == bs.head) else { return };
        // Moving collapses onto the edge of the current range first.
        let ids = self.block_sel_ids();
        let next = if down {
            // Skip descendants of the selection when stepping down.
            let last = if extend { bs.head } else { *ids.last().unwrap_or(&bs.head) };
            let lp = paths.get(&last).cloned().unwrap_or_default();
            let li = vis.iter().position(|x| *x == last).unwrap_or(i);
            vis[li + 1..].iter().copied().find(|id| paths.get(id).is_some_and(|p| extend || !covers(&lp, p)))
        } else {
            let first = if extend { bs.head } else { *ids.first().unwrap_or(&bs.head) };
            let fi = vis.iter().position(|x| *x == first).unwrap_or(i);
            fi.checked_sub(1).map(|k| vis[k])
        };
        match next {
            Some(n) if extend => self.select_range(bs.anchor, n),
            Some(n) => self.select_block(n),
            None if down && !extend && scope.is_none() => {
                let id = self.trailing_line();
                self.block_sel = None;
                self.sel = Selection::caret(Pos::new(id, 0));
            }
            None => {}
        }
    }

    /// ← selects the parent (or folds); → selects the first child (or unfolds).
    pub fn block_sel_horizontal(&mut self, right: bool) {
        let Some(bs) = self.block_sel else { return };
        let id = bs.head;
        let Some(b) = self.block(id) else { return };
        let foldable = self.can_fold(id);
        if right {
            if b.folded {
                self.toggle_fold(id);
            } else if let Some(c) = b.children.iter().find(|c| !matches!(c.kind, BlockKind::Column { .. })).or_else(|| {
                b.children.first().and_then(|col| col.children.first())
            }) {
                let c = c.id;
                self.select_block(c);
            }
        } else if foldable && !b.folded && b.kind.has_text() {
            self.toggle_fold(id);
            self.select_block(id);
        } else {
            let mut p = self.parent_of(id).map(|p| (p.id, matches!(p.kind, BlockKind::Column { .. })));
            // Columns aren't selectable; go to the grid.
            if let Some((pid, true)) = p {
                p = self.parent_of(pid).map(|g| (g.id, false));
            }
            if let Some((pid, _)) = p {
                self.select_block(pid);
            }
        }
    }

    /// ⌘⇧↑ / ⌘⇧↓: select from the current top-level block (within the
    /// enclosing cell, callout, column or quote) to the start / end.
    pub fn block_sel_to_edge(&mut self, down: bool) {
        let cur = self.block_sel.map(|b| b.head).unwrap_or(self.sel.head.block);
        if cur == TITLE {
            let first = self.doc.blocks.first().map(|b| b.id);
            let last = self.doc.blocks.last().map(|b| b.id);
            if let (Some(f), Some(l)) = (first, last) {
                self.select_range(f, l);
            }
            return;
        }
        let Some(p) = self.path(cur) else { return };
        // Container boundary: the deepest cell / callout / column / quote ancestor.
        let mut base = 0;
        for n in (1..p.len()).rev() {
            if matches!(p[n], Seg::Cell(..)) {
                base = n;
                break;
            }
            let anc = block_at(&self.doc.blocks, &p[..n]).unwrap();
            if matches!(anc.kind, BlockKind::Callout { .. } | BlockKind::Column { .. } | BlockKind::Quote) {
                base = n;
                break;
            }
        }
        let top = block_at(&self.doc.blocks, &p[..=base]).unwrap().id;
        let siblings: Vec<BlockId> = match parent_path(&p[..=base]) {
            None => self.doc.blocks.iter().map(|b| b.id).collect(),
            Some(pp) => {
                let parent = block_at(&self.doc.blocks, pp).unwrap();
                match (p[base], &parent.kind) {
                    (Seg::Cell(r, c, _), BlockKind::Table(t)) => t.rows[r][c].blocks.iter().map(|b| b.id).collect(),
                    _ => parent.children.iter().map(|b| b.id).collect(),
                }
            }
        };
        let edge = if down { *siblings.last().unwrap() } else { siblings[0] };
        self.select_range(top, edge);
    }

    /// Shift+↑ at the start of a block / Shift+↓ at its end: switch from a
    /// text selection to selecting blocks.
    pub fn extend_into_blocks(&mut self, down: bool) {
        let id = self.sel.head.block;
        if id == TITLE {
            return;
        }
        self.select_block(id);
        self.block_sel_move(down, true);
    }

    /// Backspace / Delete in block mode.
    pub fn delete_selected_blocks(&mut self) {
        let ids = self.block_sel_ids();
        if ids.is_empty() {
            return;
        }
        self.checkpoint(EditKind::Other);
        self.break_coalescing();
        let us = self.units();
        let inside = inside_of(path_index(&self.doc.blocks), &ids);
        let first_unit = us.iter().position(|u| inside(u.id));
        let prev = first_unit.and_then(|i| us[..i].iter().rev().find(|u| u.id == TITLE || !inside(u.id)));
        for id in ids.iter().rev() {
            self.remove_subtree(*id);
        }
        self.normalize();
        self.block_sel = None;
        match prev.copied() {
            Some(u) if u.id == TITLE => {
                let first = self.doc.blocks[0].id;
                self.caret_to_start(first);
            }
            Some(u) if u.text => self.sel = Selection::caret(Pos::new(u.id, self.len_of(u.id))),
            Some(u) => {
                self.sel = Selection::caret(Pos::new(u.id, 0));
                self.select_block(u.id);
            }
            None => {
                let first = self.doc.blocks[0].id;
                self.caret_to_start(first);
            }
        }
        self.fix_selection();
    }

    fn caret_to_start(&mut self, id: BlockId) {
        let us = self.units();
        let inside = inside_of(path_index(&self.doc.blocks), &[id]);
        let target = us.iter().find(|u| inside(u.id)).map(|u| u.id).unwrap_or(id);
        self.sel = Selection::caret(Pos::new(target, 0));
    }

    /// Enter in block mode: caret to the end of the last selected text block,
    /// or a new paragraph after the selection.
    pub fn enter_from_block_sel(&mut self) {
        let ids = self.block_sel_ids();
        let Some(last) = ids.last().copied() else { return };
        self.block_sel = None;
        let us = self.units();
        let inside = inside_of(path_index(&self.doc.blocks), &ids);
        let target = us.iter().rev().find(|u| u.text && inside(u.id));
        if let Some(u) = target {
            self.sel = Selection::caret(Pos::new(u.id, self.len_of(u.id)));
        } else {
            self.checkpoint(EditKind::Other);
            let b = Block::text("");
            let id = b.id;
            self.insert_after(last, b);
            self.sel = Selection::caret(Pos::new(id, 0));
        }
    }

    /// ⌘A: block text → enclosing cell / table / callout → whole document.
    pub fn select_all(&mut self) {
        if self.block_sel.is_none() {
            let (s, e) = self.ordered();
            if s.block == e.block && self.text_of(s.block).is_some() {
                let len = self.len_of(s.block);
                if len > 0 && !(s.offset == 0 && e.offset == len) {
                    self.sel = Selection { anchor: Pos::new(s.block, 0), head: Pos::new(s.block, len) };
                    return;
                }
            }
        }
        let current: Vec<BlockId> = if self.block_sel.is_some() {
            self.block_sel_ids()
        } else {
            let (s, e) = self.ordered();
            if s.block == TITLE {
                vec![]
            } else if s.block == e.block {
                vec![s.block]
            } else {
                self.selected_blocks()
            }
        };
        let paths = path_index(&self.doc.blocks);
        let chains: Vec<Vec<Stop>> = current.iter().filter_map(|id| paths.get(id)).map(|p| self.stop_chain(p)).collect();
        let candidates: Vec<Stop> = match chains.first() {
            Some(c) => c.iter().filter(|s| chains.iter().all(|o| o.contains(s))).cloned().collect(),
            None => vec![Stop::Root],
        };
        for stop in candidates {
            let content = self.stop_content(&stop);
            if content != current && !content.is_empty() {
                // Ending on the last visible block still takes in what a
                // folded heading hides after it (block_sel_ids).
                let vis: HashSet<BlockId> = self.block_order(true).into_iter().collect();
                let mut shown = content.iter().filter(|id| vis.contains(id));
                if let Some(&first) = shown.next() {
                    self.select_range(first, shown.next_back().copied().unwrap_or(first));
                    return;
                }
            }
        }
    }

    /// Select-all stops from the block at `path` outward: tables, callouts,
    /// table cells, then the document root. Grids, columns and quotes are
    /// passed through.
    fn stop_chain(&self, path: &[Seg]) -> Vec<Stop> {
        let mut out = vec![];
        for n in (1..=path.len()).rev() {
            let b = block_at(&self.doc.blocks, &path[..n]).unwrap();
            if matches!(b.kind, BlockKind::Table(_) | BlockKind::Callout { .. }) {
                out.push(Stop::Block(b.id));
            }
            if n >= 2
                && let Seg::Cell(r, c, _) = path[n - 1]
            {
                let t = block_at(&self.doc.blocks, &path[..n - 1]).unwrap().id;
                out.push(Stop::Cell(t, r, c));
            }
        }
        out.push(Stop::Root);
        out
    }

    fn stop_content(&self, s: &Stop) -> Vec<BlockId> {
        match *s {
            Stop::Root => self.doc.blocks.iter().map(|b| b.id).collect(),
            Stop::Block(id) => vec![id],
            Stop::Cell(t, r, c) => match self.block(t).map(|b| &b.kind) {
                Some(BlockKind::Table(tb)) => tb.rows[r][c].blocks.iter().map(|b| b.id).collect(),
                _ => vec![],
            },
        }
    }

    // ---- Markdown-style shortcuts (behavior-spec §2)

    /// Blocks in which live Markdown conversion runs: text, headings, lists
    /// and todos (paragraphs inside quotes are plain text blocks).
    fn markdown_enabled(&self, id: BlockId) -> bool {
        id != TITLE
            && self.kind_of(id).is_some_and(|k| {
                matches!(
                    k,
                    BlockKind::Text | BlockKind::Heading { .. } | BlockKind::Bullet | BlockKind::Ordered { .. } | BlockKind::Todo { .. }
                )
            })
    }

    fn parent_kind_is(&self, id: BlockId, f: fn(&BlockKind) -> bool) -> bool {
        self.parent_of(id).is_some_and(|p| f(&p.kind))
    }

    /// Line-start trigger for `prefix` (text before the caret, trigger
    /// character included). `enter` is true when the trigger is Enter.
    fn block_trigger(&self, id: BlockId, prefix: &str, enter: bool) -> Option<BlockKind> {
        let cur = self.kind_of(id)?;
        let in_cell = self.is_in_table(id);
        let in_quote = self.parent_kind_is(id, |k| matches!(k, BlockKind::Quote));
        let in_callout = self.parent_kind_is(id, |k| matches!(k, BlockKind::Callout { .. }));
        let code = |lang: &str| -> Option<BlockKind> {
            (!lang.contains(char::is_whitespace) && !in_callout)
                .then(|| BlockKind::Code { lang: crate::lang::resolve(lang).into(), caption: None, wrap: false })
        };
        if enter {
            return prefix.strip_prefix("```").and_then(code);
        }
        let body = prefix.strip_suffix(' ')?;
        match body {
            "-" | "*" | "+" => return Some(BlockKind::Bullet),
            "[]" | "[ ]" | "【】" => return Some(BlockKind::Todo { done: false }),
            ">" | "|" | "》" if !in_cell && !in_quote => return Some(BlockKind::Quote),
            _ => {}
        }
        if !body.is_empty() && body.len() <= 9 && body.bytes().all(|c| c == b'#') {
            let level = body.len() as u8;
            if matches!(cur, BlockKind::Heading { level: l, .. } if *l == level) {
                return None;
            }
            return Some(BlockKind::Heading { level, seq_auto: false });
        }
        if let Some(num) = body.strip_suffix('.').or_else(|| body.strip_suffix('。'))
            && !num.is_empty()
            && num.bytes().all(|c| c.is_ascii_digit())
        {
            let n: u32 = num.parse().ok()?;
            // On a heading, "1. " switches on heading auto-numbering instead.
            if let BlockKind::Heading { level, .. } = cur {
                return Some(BlockKind::Heading { level: *level, seq_auto: true });
            }
            return Some(BlockKind::Ordered { seq: (n != 1).then_some(n) });
        }
        if let Some(lang) = body.strip_prefix("```") {
            return code(lang);
        }
        None
    }

    /// Called right after a space was typed.
    pub fn markdown_block_shortcut(&mut self) -> bool {
        self.markdown_block_trigger(false)
    }

    /// Called on Enter before splitting ("```lang" + Enter makes a code block).
    pub fn markdown_enter_shortcut(&mut self) -> bool {
        self.markdown_divider() || self.markdown_block_trigger(true)
    }

    fn markdown_block_trigger(&mut self, enter: bool) -> bool {
        let p = self.sel.head;
        if !self.sel.is_collapsed() || !self.markdown_enabled(p.block) {
            return false;
        }
        let prefix = self.text_of(p.block).unwrap().text[..p.offset].to_string();
        let Some(kind) = self.block_trigger(p.block, &prefix, enter) else { return false };
        // The trigger character was its own undo step; the conversion is
        // another, so the first ⌘Z restores the literal prefix.
        self.checkpoint(EditKind::Other);
        self.break_coalescing();
        let t = self.text_mut(p.block).unwrap();
        t.replace(0..p.offset, &RichText::default());
        if matches!(kind, BlockKind::Quote) {
            self.wrap_blocks(&[p.block], kind);
        } else {
            if matches!(kind, BlockKind::Code { .. })
                && let Some(b) = self.block_mut(p.block)
            {
                b.text = RichText::plain(b.text.text.clone());
            }
            self.turn_one(p.block, kind);
        }
        self.normalize();
        self.sel = Selection::caret(Pos::new(p.block, 0));
        true
    }

    /// `---` (on the third dash) and `***` (on space or Enter) at the start of
    /// a line become a divider; text after the caret moves below it.
    pub fn markdown_divider(&mut self) -> bool {
        let p = self.sel.head;
        if !self.sel.is_collapsed() || !self.markdown_enabled(p.block) || self.is_in_table(p.block) {
            return false;
        }
        let in_callout = self.parent_kind_is(p.block, |k| matches!(k, BlockKind::Callout { .. }));
        let text = &self.text_of(p.block).unwrap().text;
        let head = text[..p.offset].trim_end_matches(' ');
        if in_callout || !matches!(head, "---" | "***") {
            return false;
        }
        let tail = self.text_mut(p.block).unwrap().split_off(p.offset);
        self.checkpoint(EditKind::Other);
        self.break_coalescing();
        let id = p.block;
        let path = self.path(id).unwrap();
        let (list, i) = container_mut(&mut self.doc.blocks, &path).unwrap();
        let old = std::mem::replace(&mut list[i], Block::new(BlockKind::Divider, RichText::default()));
        let mut nb = Block::new(BlockKind::Text, tail);
        nb.children = old.children;
        let nid = nb.id;
        list.insert(i + 1, nb);
        self.sel = Selection::caret(Pos::new(nid, 0));
        true
    }

    /// A URL followed by a typed space becomes a link (`http(s)://…`, `www.…`).
    pub fn autolink(&mut self) -> bool {
        let p = self.sel.head;
        if !self.sel.is_collapsed() || !self.markdown_enabled(p.block) {
            return false;
        }
        let Some(t) = self.text_of(p.block) else { return false };
        let Some(before) = t.text[..p.offset].strip_suffix(' ') else { return false };
        let start = before.rfind(char::is_whitespace).map_or(0, |i| i + 1);
        let word = before[start..].trim_end_matches(|c: char| ".,;:!?)]}'\"。，；：！？）】、".contains(c));
        let lower = word.to_ascii_lowercase();
        let rest = ["https://", "http://", "www."].iter().find_map(|s| lower.strip_prefix(s));
        if !rest.is_some_and(|r| r.len() > 1 && r.contains(|c: char| c.is_alphanumeric())) {
            return false;
        }
        let range = start..start + word.len();
        if t.slice(range.clone()).runs.iter().any(|r| r.marks.link.is_some() || r.marks.code || r.marks.inline.is_some()) {
            return false;
        }
        let href = if lower.starts_with("www.") { format!("https://{word}") } else { word.to_string() };
        self.checkpoint(EditKind::Other);
        self.break_coalescing();
        self.text_mut(p.block).unwrap().update_marks(range, |m| m.link = Some(href.clone()));
        true
    }

    /// Inline Markdown, run after each typed character: the closing delimiter
    /// turns `**x**`, `__x__`, `*x*`, `_x_`, `***x***`, `~~x~~`, `~x~`
    /// (underline) and `` `x` `` into marks, in a fixed priority order.
    pub fn markdown_inline_shortcut(&mut self) -> bool {
        let p = self.sel.head;
        if !self.sel.is_collapsed() || !self.markdown_enabled(p.block) {
            return false;
        }
        let Some(t) = self.text_of(p.block) else { return false };
        let before = &t.text[..p.offset];
        let after = &t.text[p.offset..];
        type Setter = fn(&mut Marks);
        // (delimiter, mark, needs whitespace/line boundary on both sides)
        let rules: [(&str, Setter, bool); 9] = [
            ("***", |m| { m.bold = true; m.italic = true }, false),
            ("___", |m| { m.bold = true; m.italic = true }, true),
            ("**", |m| m.bold = true, false),
            ("__", |m| m.bold = true, true),
            ("*", |m| m.italic = true, false),
            ("_", |m| m.italic = true, true),
            ("~~", |m| m.strike = true, false),
            ("~", |m| m.underline = true, true),
            ("`", |m| m.code = true, false),
        ];
        let boundary = |c: Option<char>| c.is_none_or(char::is_whitespace);
        // `$x$` and `$$x$$` become an inline equation.
        for delim in ["$$", "$"] {
            if !before.ends_with(delim) || t.slice(0..p.offset).runs.last().is_some_and(|r| r.marks.code) {
                continue;
            }
            let inner_end = before.len() - delim.len();
            let Some(open) = before[..inner_end].rfind(delim) else { continue };
            let inner = &before[open + delim.len()..inner_end];
            if inner.trim().is_empty()
                || inner.starts_with('$')
                || inner.ends_with('$')
                || inner.contains(EQUATION)
                || before[..open].ends_with('$')
                || !boundary(before[..open].chars().next_back())
                || !boundary(after.chars().next())
                || t.slice(open..p.offset).runs.iter().any(|r| r.marks.code || r.marks.inline.is_some())
            {
                continue;
            }
            let mut marks = t.marks_at(open + delim.len());
            marks.inline = Some(crate::docxml::to_inline(InlineKind::Equation(inner.to_string())));
            self.checkpoint(EditKind::Other);
            self.break_coalescing();
            let mut ins = RichText::default();
            ins.push(EQUATION, marks);
            let t = self.text_mut(p.block).unwrap();
            t.replace(open..p.offset, &ins);
            self.sel = Selection::caret(Pos::new(p.block, open + EQUATION.len()));
            return true;
        }
        for (delim, set, bounded) in rules {
            if !before.ends_with(delim) {
                continue;
            }
            let inner_end = before.len() - delim.len();
            let Some(open) = before[..inner_end].rfind(delim) else { continue };
            let inner = &before[open + delim.len()..inner_end];
            let dc = delim.chars().next().unwrap();
            if inner.is_empty() || inner.starts_with(char::is_whitespace) || inner.starts_with(dc) || inner.ends_with(dc) {
                continue;
            }
            // The delimiter run must be exactly this long on both ends.
            if before[..open].ends_with(dc) {
                continue;
            }
            if bounded && !(boundary(before[..open].chars().next_back()) && boundary(after.chars().next())) {
                continue;
            }
            if dc != '`' && t.slice(open..inner_end).runs.iter().any(|r| r.marks.code) {
                continue;
            }
            let mut core_inner = t.slice(open + delim.len()..inner_end);
            if core_inner.runs.iter().all(|r| {
                let mut m = r.marks.clone();
                set(&mut m);
                m == r.marks
            }) {
                // Already formatted this way: leave the text alone.
                continue;
            }
            let len = core_inner.len();
            core_inner.update_marks(0..len, set);
            self.checkpoint(EditKind::Other);
            self.break_coalescing();
            let t = self.text_mut(p.block).unwrap();
            t.replace(open..p.offset, &core_inner);
            let end = open + len;
            self.sel = Selection::caret(Pos::new(p.block, end));
            // Typing after the converted span continues unformatted.
            let mut m = self.text_of(p.block).unwrap().marks_at(end);
            m.bold = false;
            m.italic = false;
            m.underline = false;
            m.strike = false;
            m.code = false;
            self.pending_marks = Some(m);
            return true;
        }
        false
    }

    // ---- paste

    /// Paste blocks at the caret. The first pasted paragraph joins the text
    /// before the caret and the text after the caret joins the last one.
    pub fn paste_blocks(&mut self, blocks: Vec<Block>) {
        if blocks.is_empty() {
            return;
        }
        // One table pasted into a table fills its cells.
        if self.paste_table_matrix(&blocks) {
            return;
        }
        self.checkpoint(EditKind::Other);
        self.break_coalescing();
        let p = self.delete_selection_inner();
        let mut blocks: Vec<Block> = blocks.iter().map(|b| b.fresh_ids()).collect();
        let is_plain = |b: &Block| matches!(b.kind, BlockKind::Text) && b.children.is_empty();
        if p.block == TITLE {
            // Only the first block's text goes into the title; the rest
            // starts the body.
            let first = blocks.remove(0);
            let text = first.text.plain_text().replace('\n', " ");
            self.text_mut(TITLE).unwrap().insert_plain(p.offset, &text, Marks::default());
            self.sel = Selection::caret(Pos::new(TITLE, p.offset + text.len()));
            let mut prev = TITLE;
            for b in blocks {
                let id = b.id;
                self.insert_after(prev, b);
                prev = id;
            }
            if prev != TITLE {
                self.normalize();
                self.caret_to_end_of(prev);
            }
            return;
        }
        if self.is_code(p.block) {
            // Flattened to plain lines, two spaces per nesting level.
            fn flat(blocks: &[Block], depth: usize, out: &mut Vec<String>) {
                for b in blocks {
                    if b.kind.has_text() {
                        for line in b.text.plain_text().split('\n') {
                            out.push(format!("{}{line}", "  ".repeat(depth)));
                        }
                    }
                    if let BlockKind::Table(t) = &b.kind {
                        for (_, _, c) in t.cells() {
                            flat(&c.blocks, depth, out);
                        }
                    }
                    let nested = b.kind.has_text() && !b.kind.is_container();
                    flat(&b.children, depth + nested as usize, out);
                }
            }
            let mut lines = vec![];
            flat(&blocks, 0, &mut lines);
            let text = lines.join("\n");
            let t = self.text_mut(p.block).unwrap();
            t.insert_plain(p.offset, &text, Marks::default());
            self.sel = Selection::caret(Pos::new(p.block, p.offset + text.len()));
            return;
        }
        if self.is_in_table(p.block) {
            let n = blocks.len();
            blocks.retain(|b| !matches!(b.kind, BlockKind::Table(_)));
            if blocks.len() < n {
                self.notice = Some("当前位置不支持粘贴表格");
            }
            if blocks.is_empty() {
                return;
            }
        }
        let Some(cur) = self.block(p.block).cloned().filter(|b| b.kind.has_text()) else {
            // Caret on an atomic block: insert after it.
            let mut prev = p.block;
            let last = blocks.last().unwrap().id;
            for b in blocks {
                let id = b.id;
                self.insert_after(prev, b);
                prev = id;
            }
            self.normalize();
            self.caret_to_end_of(last);
            return;
        };
        let tail = self.text_mut(p.block).unwrap().split_off(p.offset);
        // First block merges into the head.
        let mut caret = Pos::new(p.block, p.offset);
        if is_plain(&blocks[0]) {
            let first = blocks.remove(0);
            let t = self.text_mut(p.block).unwrap();
            t.append(&first.text);
            caret = Pos::new(p.block, t.len());
        }
        let mut prev = p.block;
        let mut last_inserted = None;
        let head_empty = self.len_of(p.block) == 0;
        for b in blocks {
            let id = b.id;
            self.insert_after(prev, b);
            prev = id;
            last_inserted = Some(id);
        }
        if let Some(last) = last_inserted {
            let last_is_text = self.block(last).is_some_and(|b| b.kind.has_text() && !matches!(b.kind, BlockKind::Code { .. }));
            if last_is_text {
                let len = self.len_of(last);
                self.text_mut(last).unwrap().append(&tail);
                caret = Pos::new(last, len);
                // Pasting a non-text first block into an empty paragraph replaces it.
                if head_empty && matches!(cur.kind, BlockKind::Text) && cur.children.is_empty() {
                    self.remove_subtree(p.block);
                }
            } else {
                if !tail.is_empty() {
                    let nb = Block::new(cur.kind.clone().reset_for_split(), tail);
                    self.insert_after(last, nb);
                }
                if head_empty && matches!(cur.kind, BlockKind::Text) && cur.children.is_empty() {
                    self.remove_subtree(p.block);
                }
                self.normalize();
                self.caret_to_end_of(last);
                return;
            }
        } else {
            self.text_mut(p.block).unwrap().append(&tail);
        }
        self.normalize();
        self.sel = Selection::caret(caret);
    }

    /// Put the caret at the end of the last text unit inside block `id`
    /// (or on it when atomic).
    pub fn caret_to_end_of(&mut self, id: BlockId) {
        let us = units(&self.doc);
        let inside = inside_of(path_index(&self.doc.blocks), &[id]);
        let target = us.iter().rev().find(|u| u.id == id || inside(u.id)).map(|u| u.id).unwrap_or(id);
        let len = self.len_of(target);
        self.sel = Selection::caret(Pos::new(target, len));
    }

    /// Paste plain text: newlines create paragraphs (or stay literal in code).
    pub fn paste_text(&mut self, s: &str) {
        let s = s.replace("\r\n", "\n").replace('\r', "\n");
        if !s.contains('\n') || self.is_code(self.sel.head.block) {
            self.break_coalescing();
            self.insert_text(&s);
            self.break_coalescing();
            return;
        }
        let blocks: Vec<Block> = s.split('\n').map(Block::text).collect();
        self.paste_blocks(blocks);
    }

    /// Paste plain text as Markdown (behavior-spec §2.5): nothing is parsed
    /// in code or the title, a non-empty target only gets the inline rules
    /// (one paragraph per line), and the target disables the rules for its
    /// own kind.
    pub fn paste_markdown(&mut self, s: &str) {
        let s = s.replace("\r\n", "\n").replace('\r', "\n");
        let id = self.sel.head.block;
        if id == TITLE || self.is_code(id) {
            return self.paste_text(&s);
        }
        let empty_target = self.sel.is_collapsed() && self.block_sel.is_none() && self.len_of(id) == 0;
        if !empty_target {
            return self.paste_blocks(crate::paste::markdown_inline_lines(&s));
        }
        let kind = self.kind_of(id).cloned();
        let off = crate::paste::Off {
            bullet: matches!(kind, Some(BlockKind::Bullet)),
            ordered: matches!(kind, Some(BlockKind::Ordered { .. })),
            todo: matches!(kind, Some(BlockKind::Todo { .. })),
            quote: self.parent_kind_is(id, |k| matches!(k, BlockKind::Quote)),
            code_divider: self.parent_kind_is(id, |k| matches!(k, BlockKind::Callout { .. })),
            file: false,
        };
        let blocks = crate::paste::markdown_to_blocks(&s, off);
        if blocks.is_empty() {
            return self.paste_text(&s);
        }
        self.paste_blocks(blocks);
    }

    /// Selected content as blocks (for internal copy).
    pub fn selected_blocks_content(&self) -> Vec<Block> {
        let index = block_index(&self.doc);
        if self.block_sel.is_some() {
            return self.block_sel_ids().iter().filter_map(|id| index.get(id).map(|b| (*b).clone())).collect();
        }
        let us = self.units();
        let pos = unit_positions(&us);
        let (s, e) = self.ordered_in(&us);
        let mut out = vec![];
        if s.block == e.block {
            if let Some(t) = self.text_of(s.block) {
                out.push(Block::new(BlockKind::Text, t.slice(s.offset..e.offset)));
            } else if let Some(b) = self.block(s.block) {
                out.push(b.clone());
            }
            return out;
        }
        let (si, ei) = (pos.get(&s.block).copied().unwrap_or(0), pos.get(&e.block).copied().unwrap_or(0));
        // Selected part of a block's own text (cf. `range_in`).
        let range = |id: BlockId, len: usize| {
            let i = *pos.get(&id)?;
            (si..=ei).contains(&i).then_some((if i == si { s.offset } else { 0 })..(if i == ei { e.offset } else { len }))
        };
        for id in self.selected_blocks() {
            let Some(b) = index.get(&id) else { continue };
            let mut b = (*b).clone();
            if let Some(r) = range(id, b.text.len()) {
                b.text = b.text.slice(r);
            }
            // Trim children that fall after the selection end.
            prune_after(&mut b, e, &pos);
            out.push(b);
        }
        out
    }
}

/// A column goes once its last block does, and the
/// others share its width in proportion.
pub const MAX_COLUMNS: usize = 10;
pub const MIN_COLUMN_RATIO: f32 = 1. / 24.;

fn drop_empty_columns(cols: &mut Vec<Block>) {
    let ratio = |b: &Block| if let BlockKind::Column { ratio } = b.kind { ratio } else { 0. };
    let total: f32 = cols.iter().map(ratio).sum();
    let gone: f32 = cols.iter().filter(|c| c.children.is_empty()).map(ratio).sum();
    if cols.iter().all(|c| !c.children.is_empty()) {
        return;
    }
    cols.retain(|c| !c.children.is_empty());
    let left = total - gone;
    for c in cols.iter_mut() {
        if let BlockKind::Column { ratio } = &mut c.kind
            && left > 0.
        {
            *ratio /= left;
        }
    }
}

fn prune_after(b: &mut Block, end: Pos, pos: &HashMap<BlockId, usize>) {
    let ei = pos.get(&end.block).copied().unwrap_or(usize::MAX);
    fn walk(list: &mut Vec<Block>, ei: usize, end: Pos, pos: &HashMap<BlockId, usize>) {
        list.retain(|c| pos.get(&c.id).is_none_or(|i| *i <= ei));
        for c in list.iter_mut() {
            if c.id == end.block {
                c.text = c.text.slice(0..end.offset.min(c.text.len()));
            }
            walk(&mut c.children, ei, end, pos);
        }
    }
    walk(&mut b.children, ei, end, pos);
}

impl BlockKind {
    /// Kind for the second half when a block is split in the middle.
    pub fn reset_for_split(self) -> BlockKind {
        match self {
            BlockKind::Ordered { .. } => BlockKind::Ordered { seq: None },
            BlockKind::Todo { .. } => BlockKind::Todo { done: false },
            k => k,
        }
    }
}

/// Ordered list numbering: number for each Ordered block given its siblings.
pub fn ordered_number(siblings: &[Block], index: usize) -> u32 {
    let mut n = 0u32;
    let mut i = index as isize;
    // Walk back to the start of the run of ordered items.
    while i >= 0 {
        match siblings[i as usize].kind {
            BlockKind::Ordered { seq } => {
                if let Some(s) = seq {
                    return s.saturating_add(n);
                }
                n += 1;
            }
            _ => break,
        }
        i -= 1;
    }
    n
}

/// Numbers of the auto-numbered headings (`seq="auto"`): "1." at the outer
/// depth, "1.2.3" below. Each container (page, callout, grid column, table
/// cell) counts on its own, and its depths are the heading levels it uses,
/// in order, so h2 + h4 number 1. / 1.1.
pub fn heading_orders(blocks: &[Block]) -> HashMap<BlockId, String> {
    fn gather<'a>(list: &'a [Block], run: &mut Vec<(BlockId, u8)>, containers: &mut Vec<&'a [Block]>) {
        for b in list {
            match &b.kind {
                BlockKind::Heading { level, seq_auto: true } => run.push((b.id, *level)),
                BlockKind::Callout { .. } | BlockKind::Column { .. } => {
                    containers.push(&b.children);
                    continue;
                }
                BlockKind::Table(t) => t.rows.iter().flatten().for_each(|c| containers.push(&c.blocks)),
                _ => {}
            }
            gather(&b.children, run, containers);
        }
    }
    let mut out = HashMap::default();
    let mut containers = vec![blocks];
    while let Some(list) = containers.pop() {
        let mut run = vec![];
        gather(list, &mut run, &mut containers);
        let mut levels: Vec<u8> = run.iter().map(|r| r.1).collect();
        levels.sort_unstable();
        levels.dedup();
        let mut o: Vec<u32> = vec![];
        for (id, level) in run {
            let d = levels.iter().position(|l| *l == level).unwrap_or(0) + 1;
            if o.len() < d {
                o.resize(d, 1);
            } else {
                o.truncate(d);
                o[d - 1] += 1;
            }
            let s = if d == 1 { format!("{}.", o[0]) } else { o.iter().map(u32::to_string).collect::<Vec<_>>().join(".") };
            out.insert(id, s);
        }
    }
    out
}

/// Marker text for an ordered item at nesting `depth` (1. → a. → i.).
pub fn ordered_marker(n: u32, depth: usize) -> String {
    match depth % 3 {
        0 => format!("{n}."),
        1 => format!("{}.", alpha(n)),
        _ => format!("{}.", roman(n)),
    }
}

fn alpha(mut n: u32) -> String {
    let mut s = vec![];
    while n > 0 {
        n -= 1;
        s.push((b'a' + (n % 26) as u8) as char);
        n /= 26;
    }
    s.iter().rev().collect()
}

fn roman(mut n: u32) -> String {
    const T: [(u32, &str); 13] = [
        (1000, "m"), (900, "cm"), (500, "d"), (400, "cd"), (100, "c"), (90, "xc"),
        (50, "l"), (40, "xl"), (10, "x"), (9, "ix"), (5, "v"), (4, "iv"), (1, "i"),
    ];
    let mut s = String::new();
    for (v, r) in T {
        while n >= v {
            s.push_str(r);
            n -= v;
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc(xml: &str) -> Core {
        Core::new(crate::docxml::parse(xml).unwrap())
    }

    fn xml(c: &Core) -> String {
        crate::docxml::serialize(&c.doc)
    }

    fn id(c: &Core, n: usize) -> BlockId {
        c.units()[n].id
    }

    #[test]
    fn select_all_twice_deletes_a_folded_tail() {
        let mut c = doc("<title>T</title><h1>a</h1><h2 folded=\"true\">b</h2><p>hidden</p>");
        c.sel = Selection::caret(Pos::new(id(&c, 1), 0));
        c.select_all();
        c.select_all();
        assert_eq!(c.block_sel_ids().len(), 3);
        c.delete_selected_blocks();
        assert!(!xml(&c).contains("hidden"), "{}", xml(&c));
    }

    #[test]
    fn enter_on_a_folded_heading_never_hides_the_new_line() {
        let mut c = doc("<title>T</title><h1>a</h1><p>x</p><h1 folded=\"true\">b</h1><p>hidden</p>");
        let b = id(&c, 3);
        c.sel = Selection::caret(Pos::new(b, 1));
        c.enter();
        assert!(c.units().iter().any(|u| u.id == c.sel.head.block));
        assert!(c.block(b).unwrap().folded);
        assert!(xml(&c).ends_with("<p>hidden</p>\n<h1></h1>\n"), "{}", xml(&c));
    }

    #[test]
    fn code_keeps_equations_as_source_and_never_holds_children() {
        let mut c = doc("<title>T</title><p>e <latex>x^2</latex><ul><li>kid</li></ul></p><p>b <b>bold</b></p>");
        let p = id(&c, 1);
        c.turn_into(&[p], BlockKind::Code { lang: "plaintext".into(), caption: None, wrap: false });
        assert!(xml(&c).starts_with("<title>T</title>\n<pre lang=\"plaintext\"><code>e x^2</code></pre>\n<ul>"), "{}", xml(&c));
        // Deleting into code merges the tail as plain text, its children after the code block.
        let mut c = doc("<title>T</title><pre lang=\"plaintext\"><code>c</code></pre><p>b <b>bold</b><ul><li>kid</li></ul></p>");
        c.sel = Selection { anchor: Pos::new(id(&c, 1), 1), head: Pos::new(id(&c, 2), 2) };
        c.delete_selection();
        assert!(xml(&c).starts_with("<title>T</title>\n<pre lang=\"plaintext\"><code>cbold</code></pre>\n<ul>"), "{}", xml(&c));
        // Nor does a drop open blocks that can't hold children.
        let mut c = doc("<title>T</title><hr/><p>a</p>");
        let (hr, a) = (id(&c, 1), id(&c, 2));
        assert!(!c.move_blocks(&[a], hr, DropPlace::FirstChild));
    }

    #[test]
    fn undo_keeps_folds_and_icon() {
        let mut c = doc("<title>T</title><h1>h</h1><p>a</p>");
        let h = id(&c, 1);
        c.sel = Selection::caret(Pos::new(id(&c, 2), 1));
        c.insert_text("b");
        c.toggle_fold(h);
        c.doc.icon = Some("🐱".into());
        c.undo();
        assert_eq!(c.text_of(id(&c, 1)).map(|t| t.text.as_str()), Some("h"));
        assert!(c.block(h).unwrap().folded);
        assert_eq!(c.doc.icon.as_deref(), Some("🐱"));
        assert!(xml(&c).contains("<p>a</p>"), "{}", xml(&c));
    }

    #[test]
    fn move_blocks_reorders_nests_and_undoes() {
        let mut c = doc("<title>T</title><p>a</p><p>b</p><ul><li>c</li></ul>");
        let (a, b, li) = (id(&c, 1), id(&c, 2), id(&c, 3));
        assert!(c.move_blocks(&[a], b, DropPlace::After));
        assert!(xml(&c).starts_with("<title>T</title>\n<p>b</p>\n<p>a</p>"), "{}", xml(&c));
        assert!(c.move_blocks(&[a, b], li, DropPlace::FirstChild));
        let li_el = c.block(li).unwrap();
        assert_eq!(li_el.children.iter().map(|x| x.id).collect::<Vec<_>>(), vec![a, b]);
        // Into its own subtree, or where it already is: nothing happens.
        assert!(!c.move_blocks(&[li], a, DropPlace::After));
        assert!(!c.move_blocks(&[b], a, DropPlace::After));
        c.undo();
        c.undo();
        assert!(xml(&c).starts_with("<title>T</title>\n<p>a</p>\n<p>b</p>"), "{}", xml(&c));
    }

    #[test]
    fn can_fold_follows_feishu() {
        let c = doc(concat!(
            "<title>T</title><h2>h</h2><p>plain</p><p>parent<p>child</p></p><ul><li>leaf</li><li folded=\"true\">gone</li></ul>",
            "<callout emoji=\"💡\"><ul><li>x<ul><li>y</li></ul></li></ul></callout>",
            "<table><tbody><tr><td><h2>in cell</h2></td></tr></tbody></table>"
        ));
        fn walk(list: &[Block], t: &str) -> Option<BlockId> {
            list.iter().find_map(|b| {
                let rows = match &b.kind {
                    BlockKind::Table(tb) => tb.rows.iter().flat_map(|r| r.iter()).find_map(|cell| walk(&cell.blocks, t)),
                    _ => None,
                };
                (b.text.text == t).then_some(b.id).or(rows).or_else(|| walk(&b.children, t))
            })
        }
        let find = |t: &str| walk(&c.doc.blocks, t).unwrap();
        assert!(c.can_fold(find("h")));
        assert!(!c.can_fold(find("plain")));
        assert!(c.can_fold(find("parent")));
        assert!(!c.can_fold(find("leaf")));
        // A list item stays foldable while marked folded, even with no children.
        assert!(c.can_fold(find("gone")));
        assert!(!c.can_fold(find("x")));
        assert!(!c.can_fold(find("in cell")));
    }

    #[test]
    fn undo_history_is_bounded() {
        let mut c = doc("<title>T</title><p>a</p>");
        for _ in 0..UNDO_STEPS + 10 {
            c.checkpoint(EditKind::Other);
        }
        assert_eq!(c.undo.len(), UNDO_STEPS);
        // Huge steps: memory wins, down to the minimum.
        for _ in 0..UNDO_MIN_STEPS + 5 {
            c.push_undo(Snapshot { doc: Document::default(), sel: c.sel, bytes: UNDO_BYTES / 4 });
        }
        assert_eq!(c.undo.len(), UNDO_MIN_STEPS);
        assert!(c.undo());
    }

    #[test]
    fn unfolding_to_a_heading_is_saved() {
        let mut c = doc("<title>T</title><h1 folded=\"true\">A</h1><h2>B</h2><p>b1</p>");
        let b = c.doc.blocks[1].id;
        c.dirty = false;
        c.unfold_to(b);
        assert!(c.dirty);
        assert_eq!(xml(&c), "<title>T</title>\n<h1>A</h1>\n<h2>B</h2>\n<p>b1</p>\n");
        // Nothing hidden: nothing to save.
        c.dirty = false;
        c.unfold_to(b);
        assert!(!c.dirty);
    }

    #[test]
    fn folding_moves_hidden_caret_and_enter_skips_section() {
        let mut c = doc("<title>T</title><h1>A</h1><p>a1</p><h2>B</h2><p>b1</p><h1>C</h1>");
        let (a, b1) = (id(&c, 1), id(&c, 4));
        c.sel = Selection::caret(Pos::new(b1, 1));
        c.toggle_fold(a);
        assert!(c.dirty);
        assert_eq!(c.sel.head, Pos::new(a, 1));
        assert_eq!(c.units().len(), 3);
        c.enter();
        assert_eq!(xml(&c), "<title>T</title>\n<h1 folded=\"true\">A</h1>\n<p>a1</p>\n<h2>B</h2>\n<p>b1</p>\n<h1></h1>\n<h1>C</h1>\n");
    }

    #[test]
    fn enter_splits_and_backspace_merges() {
        let mut c = doc("<title>T</title><p>hello world</p>");
        let b = id(&c, 1);
        c.sel = Selection::caret(Pos::new(b, 5));
        c.enter();
        assert_eq!(c.units().len(), 3);
        assert_eq!(c.text_of(c.sel.head.block).unwrap().text, " world");
        c.backspace();
        assert_eq!(xml(&c), "<title>T</title>\n<p>hello world</p>\n");
        assert_eq!(c.sel.head, Pos::new(b, 5));
    }

    #[test]
    fn code_enter_keeps_indent_and_down_leaves_last_block() {
        let mut c = doc("<title>T</title><pre lang=\"py\"><code>def f():\n    x</code></pre>");
        let b = id(&c, 1);
        c.sel = Selection::caret(Pos::new(b, c.len_of(b)));
        c.enter();
        c.insert_text("y");
        assert!(xml(&c).contains("<pre lang=\"python\"><code>def f():\n    x\n    y</code></pre>"), "{}", xml(&c));
        let t = c.trailing_line();
        assert_ne!(t, b);
        assert_eq!(c.trailing_line(), t);
        assert!(xml(&c).ends_with("</pre>\n<p></p>\n"), "{}", xml(&c));
    }

    #[test]
    fn backspace_stays_inside_table_cells() {
        let src = "<title>T</title><p>ab</p><table><tr><td><p>x</p></td><td><p></p><p>y</p></td></tr></table>";
        let mut c = doc(src);
        let us = c.units();
        // Start of the first cell: nothing happens, however often.
        c.sel = Selection::caret(Pos::new(us[2].id, 0));
        c.backspace();
        c.backspace();
        assert_eq!(xml(&c), xml(&doc(src)));
        assert!(!c.in_block_mode());
        // An empty first line of a cell goes; the next one takes the caret.
        c.sel = Selection::caret(Pos::new(us[3].id, 0));
        c.backspace();
        assert_eq!(c.sel.head, Pos::new(us[4].id, 0));
        assert!(xml(&c).contains("<td><p>y</p></td>"), "{}", xml(&c));
        // Then it stays put too.
        c.backspace();
        assert!(xml(&c).contains("<td><p>x</p></td><td><p>y</p></td>"), "{}", xml(&c));
        assert_eq!(c.sel.head, Pos::new(us[4].id, 0));
        assert!(!c.in_block_mode());
        // Delete at the end of a cell doesn't pull the next one in.
        c.sel = Selection::caret(Pos::new(us[2].id, 1));
        c.delete_forward();
        assert!(xml(&c).contains("<td><p>x</p></td><td><p>y</p></td>"), "{}", xml(&c));
    }

    #[test]
    fn backspace_in_empty_cell_selects_then_deletes_table() {
        let mut c = doc("<title>T</title><p>ab</p><table><tr><td><p>x</p></td><td><p></p></td></tr></table><p>cd</p>");
        let us = c.units();
        c.sel = Selection::caret(Pos::new(us[3].id, 0));
        c.backspace();
        assert!(c.in_block_mode());
        assert!(xml(&c).contains("<table>"), "{}", xml(&c));
        c.delete_selected_blocks();
        assert!(!xml(&c).contains("<table>"), "{}", xml(&c));
        // A nested table goes first, leaving the outer one.
        let mut c = doc("<title>T</title><table><tr><td><table><tr><td><p></p></td></tr></table></td><td><p>y</p></td></tr></table>");
        let us = c.units();
        c.sel = Selection::caret(Pos::new(us[1].id, 0));
        c.backspace();
        c.delete_selected_blocks();
        assert_eq!(xml(&c).matches("<table>").count(), 1, "{}", xml(&c));
    }

    #[test]
    fn backspace_after_table_selects_then_deletes_it() {
        let mut c = doc("<title>T</title><p>ab</p><table><tr><td><p>x</p></td><td><p></p></td></tr></table><p></p>");
        let last = c.units().last().unwrap().id;
        c.sel = Selection::caret(Pos::new(last, 0));
        // Empty line goes, table is selected.
        c.backspace();
        assert!(c.in_block_mode());
        assert!(xml(&c).contains("<table>"), "{}", xml(&c));
        assert!(!xml(&c).ends_with("<p></p>\n"), "{}", xml(&c));
        // Next Backspace (block mode) removes the table, caret lands on "ab".
        c.delete_selected_blocks();
        assert!(!xml(&c).contains("<table>"), "{}", xml(&c));
        assert_eq!(c.text_of(c.sel.head.block).unwrap().text, "ab");
        assert_eq!(c.sel.head.offset, 2);
        // And deletion keeps going through the text.
        c.backspace();
        assert_eq!(c.text_of(c.sel.head.block).unwrap().text, "a");

        // A non-empty line after the table stays; only the table is selected.
        let mut c = doc("<title>T</title><table><tr><td><p>x</p></td></tr></table><p>keep</p>");
        let last = c.units().last().unwrap().id;
        c.sel = Selection::caret(Pos::new(last, 0));
        c.backspace();
        assert!(c.in_block_mode());
        c.delete_selected_blocks();
        assert_eq!(xml(&c), "<title>T</title>\n<p>keep</p>\n");
    }

    #[test]
    fn delete_before_table_selects_it() {
        let mut c = doc("<title>T</title><p>ab</p><table><tr><td><p>x</p></td></tr></table>");
        let a = id(&c, 1);
        c.sel = Selection::caret(Pos::new(a, 2));
        c.delete_forward();
        assert!(c.in_block_mode());
        c.delete_selected_blocks();
        assert!(!xml(&c).contains("<table>"), "{}", xml(&c));
    }

    #[test]
    fn tab_steps_through_table_cells() {
        let mut c = doc("<title>T</title><table><tr><td><p></p></td><td><p></p></td></tr></table><p></p>");
        let a = id(&c, 1);
        c.sel = Selection::caret(Pos::new(a, 0));
        c.insert_text("a");
        assert!(c.cell_step(true));
        c.insert_text("b");
        assert!(c.cell_step(true));
        c.insert_text("c");
        assert!(c.cell_step(false));
        assert!(c.cell_step(false));
        c.insert_text("X");
        let out = xml(&c);
        assert!(out.contains("<td><p>aX</p></td>"), "{out}");
        assert!(out.contains("<td><p>b</p></td>"), "{out}");
        assert!(out.contains("<td><p>c</p></td>"), "{out}");
        assert_eq!(out.matches("<tr>").count(), 2, "{out}");
        let p = c.units().last().unwrap().id;
        c.sel = Selection::caret(Pos::new(p, 0));
        assert!(!c.cell_step(true));
    }

    #[test]
    fn typed_url_becomes_link() {
        let mut c = doc("<title>T</title><p></p>");
        let b = id(&c, 1);
        c.sel = Selection::caret(Pos::new(b, 0));
        c.insert_text("见 www.a.com/x).");
        c.insert_text(" ");
        assert!(c.autolink());
        c.insert_text("https:// ");
        assert!(!c.autolink());
        assert!(xml(&c).contains("<p>见 <a href=\"https://www.a.com/x\">www.a.com/x</a>). https:// </p>"), "{}", xml(&c));
    }

    #[test]
    fn list_enter_on_empty_exits() {
        let mut c = doc("<title>T</title><ul><li>a</li></ul>");
        let b = id(&c, 1);
        c.sel = Selection::caret(Pos::new(b, 1));
        c.enter();
        c.enter();
        assert!(xml(&c).contains("<ul>\n  <li>a</li>\n</ul>\n<p></p>"), "{}", xml(&c));
    }

    #[test]
    fn markdown_shortcuts() {
        let mut c = doc("<title>T</title><p></p>");
        let b = id(&c, 1);
        c.sel = Selection::caret(Pos::new(b, 0));
        for ch in ["#", "#", " "] {
            c.insert_text(ch);
        }
        assert!(c.markdown_block_shortcut());
        assert!(matches!(c.kind_of(b), Some(BlockKind::Heading { level: 2, .. })));
        c.insert_text("a**b*");
        assert!(!c.markdown_inline_shortcut());
        c.insert_text("*");
        assert!(c.markdown_inline_shortcut());
        assert_eq!(xml(&c), "<title>T</title>\n<h2>a<b>b</b></h2>\n");
        c.undo();
        assert_eq!(c.text_of(b).unwrap().text, "a**b**");
    }

    #[test]
    fn markdown_equations() {
        for (typed_text, src) in [("x $a^2$", Some("a^2")), ("x $$b$$", Some("b")), ("x$a$", None), ("$ $", None)] {
            let mut c = doc("<title>T</title><p></p>");
            let b = id(&c, 1);
            c.sel = Selection::caret(Pos::new(b, 0));
            typed(&mut c, typed_text);
            let t = c.text_of(b).unwrap().text.clone();
            match src {
                Some(src) => {
                    assert_eq!(t, format!("x {EQUATION}"), "{typed_text}");
                    assert_eq!(c.equation_source(Pos::new(b, 2)).as_deref(), Some(src));
                    assert_eq!(c.sel.head.offset, t.len());
                }
                None => assert!(!t.contains(EQUATION), "{typed_text}"),
            }
        }
    }

    #[test]
    fn equation_edits_coalesce() {
        let mut c = doc("<title>T</title><p>ab</p>");
        let b = id(&c, 1);
        c.sel = Selection::caret(Pos::new(b, 1));
        let p = c.insert_equation("").unwrap();
        c.break_coalescing();
        for s in ["x", "x^", "x^2"] {
            c.set_equation_source(p, s);
        }
        assert_eq!(c.equation_source(p).as_deref(), Some("x^2"));
        c.undo();
        assert_eq!(c.equation_source(p).as_deref(), Some(""));
        c.undo();
        assert_eq!(c.text_of(b).unwrap().text, "ab");
    }

    #[test]
    fn tab_indents_under_previous() {
        let mut c = doc("<title>T</title><ul><li>a</li><li>b</li><li>c</li></ul>");
        let b = id(&c, 2);
        c.sel = Selection::caret(Pos::new(b, 0));
        c.indent();
        assert_eq!(c.doc.blocks.len(), 2);
        assert_eq!(c.doc.blocks[0].children.len(), 1);
        c.outdent();
        assert_eq!(c.doc.blocks.len(), 3);
    }

    #[test]
    fn cross_block_delete() {
        let mut c = doc("<title>T</title><p>abc</p><hr/><h1>def</h1>");
        let a = id(&c, 1);
        let d = id(&c, 3);
        c.sel = Selection { anchor: Pos::new(a, 1), head: Pos::new(d, 2) };
        c.backspace();
        assert_eq!(xml(&c), "<title>T</title>\n<p>af</p>\n");
    }

    #[test]
    fn ordered_numbering() {
        let c = doc("<title>T</title><ol><li>a</li><li>b</li></ol><p>x</p><ol><li seq=\"5\">c</li><li>d</li></ol>");
        let bl = &c.doc.blocks;
        assert_eq!(ordered_number(bl, 1), 2);
        assert_eq!(ordered_number(bl, 4), 6);
        assert_eq!(ordered_marker(3, 1), "c.");
        assert_eq!(ordered_marker(4, 2), "iv.");
    }

    #[test]
    fn paste_multiline() {
        let mut c = doc("<title>T</title><p>xy</p>");
        let a = id(&c, 1);
        c.sel = Selection::caret(Pos::new(a, 1));
        c.paste_text("1\n2\n3");
        assert_eq!(xml(&c), "<title>T</title>\n<p>x1</p>\n<p>2</p>\n<p>3y</p>\n");
        assert_eq!(c.sel.head.offset, 1);
    }

    #[test]
    fn paste_markdown_targets() {
        // An empty paragraph gets block rules.
        let mut c = doc("<title>T</title><p></p>");
        c.sel = Selection::caret(Pos::new(id(&c, 1), 0));
        c.paste_markdown("# H\n- a\n**b**");
        assert_eq!(xml(&c), "<title>T</title>\n<h1>H</h1>\n<ul>\n  <li>a</li>\n</ul>\n<p><b>b</b></p>\n");
        // A non-empty one only inline rules, one paragraph per line.
        let mut c = doc("<title>T</title><p>xy</p>");
        c.sel = Selection::caret(Pos::new(id(&c, 1), 1));
        c.paste_markdown("# H\n**b**");
        assert_eq!(xml(&c), "<title>T</title>\n<p>x# H</p>\n<p><b>b</b>y</p>\n");
        // The title takes the first line; the rest starts the body.
        let mut c = doc("<title>T</title><p>body</p>");
        c.sel = Selection::caret(Pos::new(TITLE, 1));
        c.paste_markdown("A\nB");
        assert_eq!(xml(&c), "<title>TA</title>\n<p>B</p>\n<p>body</p>\n");
        assert_eq!(c.sel.head, Pos::new(id(&c, 1), 1));
    }

    #[test]
    fn paste_blocks_into_code_flattens() {
        let mut c = doc("<title>T</title><pre lang=\"python\"><code></code></pre>");
        c.sel = Selection::caret(Pos::new(id(&c, 1), 0));
        let src = crate::docxml::parse("<title>x</title><ul><li><b>a</b><ul><li>b</li></ul></li></ul><p>c</p>").unwrap();
        c.paste_blocks(src.blocks);
        assert_eq!(c.text_of(id(&c, 1)).unwrap().text, "a\n  b\nc");
    }

    #[test]
    fn paste_table_into_table() {
        let mut c = doc("<title>T</title><table><tbody><tr><td><p>1</p></td><td><p>2</p></td></tr></tbody></table>");
        let cell = id(&c, 2);
        c.sel = Selection::caret(Pos::new(cell, 0));
        let src = crate::docxml::parse("<title>x</title><table><tbody><tr><td><p>a</p></td><td><p>b</p></td></tr><tr><td><p>c</p></td><td><p>d</p></td></tr></tbody></table>").unwrap();
        c.paste_blocks(src.blocks);
        let t = c.doc.blocks.iter().find_map(|b| match &b.kind {
            BlockKind::Table(t) => Some(t.clone()),
            _ => None,
        });
        let t = t.unwrap();
        let cells: Vec<String> = t.cells().map(|(_, _, c)| c.blocks[0].text.text.clone()).collect();
        assert_eq!((t.rows.len(), t.ncols()), (2, 3));
        assert_eq!(cells, ["1", "a", "b", "", "c", "d"]);
        assert!(c.cell_sel.is_some());
        // Anything else with a table drops it, with a notice.
        let mut c = doc("<title>T</title><table><tbody><tr><td><p>1</p></td></tr></tbody></table>");
        c.sel = Selection::caret(Pos::new(id(&c, 1), 1));
        let src = crate::docxml::parse("<title>x</title><p>p</p><table><tbody><tr><td><p>a</p></td></tr></tbody></table>").unwrap();
        c.paste_blocks(src.blocks);
        assert_eq!(c.notice, Some("当前位置不支持粘贴表格"));
        assert_eq!(c.text_of(id(&c, 1)).unwrap().text, "1p");
    }

    fn typed(c: &mut Core, s: &str) {
        for ch in s.chars() {
            c.insert_text(&ch.to_string());
            if ch == ' ' {
                c.markdown_block_shortcut();
            }
            if ch == '-' || ch == ' ' {
                c.markdown_divider();
            }
            c.markdown_inline_shortcut();
        }
    }

    #[test]
    fn markdown_block_triggers() {
        let cases = [
            ("- ", "<ul>\n  <li></li>\n</ul>"),
            ("+ ", "<ul>\n  <li></li>\n</ul>"),
            ("3. ", "<ol>\n  <li seq=\"3\"></li>\n</ol>"),
            ("1。 ", "<ol>\n  <li></li>\n</ol>"),
            ("【】 ", "<checkbox"),
            ("> ", "<blockquote>"),
            ("```rust ", "<pre lang=\"rust\""),
            ("--- ", "<hr/>"),
            ("*** ", "<hr/>"),
        ];
        for (input, want) in cases {
            let mut c = doc("<title>T</title><p></p>");
            let b = id(&c, 1);
            c.sel = Selection::caret(Pos::new(b, 0));
            typed(&mut c, input);
            assert!(xml(&c).contains(want), "{input:?} -> {}", xml(&c));
        }
        // "[x] " stays literal.
        let mut c = doc("<title>T</title><p></p>");
        c.sel = Selection::caret(Pos::new(id(&c, 1), 0));
        typed(&mut c, "[x] ");
        assert!(xml(&c).contains("<p>[x] </p>"), "{}", xml(&c));
    }

    #[test]
    fn markdown_in_list_and_heading() {
        // Triggers also convert non-text blocks.
        let mut c = doc("<title>T</title><ul><li></li></ul>");
        let b = id(&c, 1);
        c.sel = Selection::caret(Pos::new(b, 0));
        typed(&mut c, "## ");
        assert!(matches!(c.kind_of(b), Some(BlockKind::Heading { level: 2, seq_auto: false })));
        typed(&mut c, "1. ");
        assert!(matches!(c.kind_of(b), Some(BlockKind::Heading { level: 2, seq_auto: true })));
    }

    #[test]
    fn markdown_inline_rules() {
        let cases = [
            ("a __b__ ", "<b>b</b>"),
            ("a_b_c", "a_b_c"),
            ("_i_", "<em>i</em>"),
            ("***x***", "<b><em>x</em></b>"),
            ("~~s~~", "<del>s</del>"),
            ("~u~", "<u>u</u>"),
            ("`c`", "<code>c</code>"),
            ("a * x*", "a * x*"),
        ];
        for (input, want) in cases {
            let mut c = doc("<title>T</title><p></p>");
            c.sel = Selection::caret(Pos::new(id(&c, 1), 0));
            typed(&mut c, input);
            assert!(xml(&c).contains(want), "{input:?} -> {}", xml(&c));
        }
    }

    #[test]
    fn toggle_turn_into_quote() {
        let mut c = doc("<title>T</title><p>a</p><p>b</p>");
        let (a, b) = (id(&c, 1), id(&c, 2));
        c.toggle_turn_into(&[a, b], BlockKind::Quote);
        assert_eq!(c.doc.blocks.len(), 1);
        assert_eq!(c.doc.blocks[0].children.len(), 2);
        c.toggle_turn_into(&[a, b], BlockKind::Quote);
        assert_eq!(xml(&c), "<title>T</title>\n<p>a</p>\n<p>b</p>\n");
    }

    #[test]
    fn heading_numbers() {
        let c = doc(concat!(
            "<title>T</title><h2 seq=\"auto\">a</h2><h4 seq=\"auto\">b</h4><h4 seq=\"auto\">c</h4>",
            "<h2 seq=\"auto\">d</h2><h2>e</h2><callout><h3 seq=\"auto\">f</h3></callout><h4 seq=\"auto\">g</h4>"
        ));
        let o = heading_orders(&c.doc.blocks);
        let n = |i| o.get(&id(&c, i)).map(String::as_str);
        assert_eq!([n(1), n(2), n(3), n(4), n(5), n(6), n(7)], [Some("1."), Some("1.1"), Some("1.2"), Some("2."), None, Some("1."), Some("2.1")]);
    }

    #[test]
    fn block_selection() {
        let mut c = doc("<title>T</title><p>a</p><ul><li>b<ul><li>c</li></ul></li></ul><p>d</p>");
        let (a, b, cc, d) = (id(&c, 1), id(&c, 2), id(&c, 3), id(&c, 4));
        c.select_block(a);
        c.block_sel_move(true, false);
        assert_eq!(c.block_sel_ids(), vec![b]);
        // Plain ↓ from a parent skips over its children.
        c.block_sel_move(true, false);
        assert_eq!(c.block_sel_ids(), vec![d]);
        c.block_sel_move(false, true);
        assert_eq!(c.block_sel_ids(), vec![cc, d]);
        c.block_sel_horizontal(false);
        assert_eq!(c.block_sel_ids(), vec![b]);
        c.select_range(a, cc);
        assert_eq!(c.block_sel_ids(), vec![a, b]);
        c.delete_selected_blocks();
        assert_eq!(xml(&c), "<title>T</title>\n<p>d</p>\n");
        assert_eq!(c.sel.head, Pos::new(d, 0));
    }

    #[test]
    fn select_all_escalates() {
        let mut c = doc("<title>T</title><p>x</p><callout><p>ab</p></callout>");
        let (x, ab) = (id(&c, 1), id(&c, 2));
        let callout = c.doc.blocks[1].id;
        c.sel = Selection::caret(Pos::new(ab, 1));
        c.select_all();
        assert_eq!(c.selected_text(), "ab");
        c.select_all();
        assert_eq!(c.block_sel_ids(), vec![callout]);
        c.select_all();
        assert_eq!(c.block_sel_ids(), vec![x, callout]);
    }

    #[test]
    fn block_mode_enter() {
        let mut c = doc("<title>T</title><p>ab</p><hr/>");
        let (ab, hr) = (id(&c, 1), id(&c, 2));
        c.select_range(ab, hr);
        c.enter_from_block_sel();
        assert_eq!(c.sel.head, Pos::new(ab, 2));
        c.select_block(hr);
        c.enter_from_block_sel();
        assert_eq!(c.units().len(), 4);
    }
}
