//! 表格 operations on the document: cell / row / column selection and the
//! edits behind the table toolbar and the block menu.

use super::core::{Core, EditKind, Pos, Seg, Selection, TITLE, block_at, block_at_mut};
use crate::model::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellSelKind {
    /// A rectangle dragged across cells.
    Range,
    /// Whole rows (row bar).
    Rows,
    /// Whole columns (column bar).
    Cols,
}

/// Selected cells of one table, from `anchor` to `head` (grid slots).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellSel {
    pub table: BlockId,
    pub anchor: (usize, usize),
    pub head: (usize, usize),
    pub kind: CellSelKind,
}

impl CellSel {
    /// The selected slots before merged cells widen them.
    pub fn raw(&self, t: &Table) -> CellRange {
        let (nr, nc) = (t.rows.len().max(1), t.ncols().max(1));
        let clamp = |(r, c): (usize, usize)| (r.min(nr - 1), c.min(nc - 1));
        let mut r = CellRange::span(clamp(self.anchor), clamp(self.head));
        match self.kind {
            CellSelKind::Rows => (r.left, r.right) = (0, nc - 1),
            CellSelKind::Cols => (r.top, r.bottom) = (0, nr - 1),
            CellSelKind::Range => {}
        }
        r
    }
}

/// Where a caret entering these blocks lands.
fn first_leaf(blocks: &[Block]) -> Option<BlockId> {
    let b = blocks.first()?;
    match &b.kind {
        k if k.has_text() => Some(b.id),
        BlockKind::Table(t) => t.cells().next().and_then(|(_, _, c)| first_leaf(&c.blocks)),
        k if k.is_container() => first_leaf(&b.children).or(Some(b.id)),
        _ => Some(b.id),
    }
}

/// The text blocks in a table's first row.
pub fn first_row_text(t: &Table) -> Vec<BlockId> {
    let mut out = vec![];
    t.cells().filter(|(r, _, _)| *r == 0).for_each(|(_, _, c)| text_blocks(&c.blocks, &mut out));
    out
}

fn text_blocks(blocks: &[Block], out: &mut Vec<BlockId>) {
    for b in blocks {
        if b.kind.has_text() {
            out.push(b.id);
        }
        text_blocks(&b.children, out);
        if let BlockKind::Table(t) = &b.kind {
            for (_, _, c) in t.cells() {
                text_blocks(&c.blocks, out);
            }
        }
    }
}

impl Core {
    pub fn table(&self, id: BlockId) -> Option<&Table> {
        match &self.block(id)?.kind {
            BlockKind::Table(t) => Some(t),
            _ => None,
        }
    }

    fn table_mut(&mut self, id: BlockId) -> Option<&mut Table> {
        let p = self.path(id)?;
        match &mut block_at_mut(&mut self.doc.blocks, &p)?.kind {
            BlockKind::Table(t) => Some(t),
            _ => None,
        }
    }

    /// The innermost table cell holding `id`: (table, row, column).
    pub fn cell_of(&self, id: BlockId) -> Option<(BlockId, usize, usize)> {
        let p = self.path(id)?;
        let i = p.iter().rposition(|s| matches!(s, Seg::Cell(..)))?;
        let Seg::Cell(r, c, _) = p[i] else { return None };
        Some((block_at(&self.doc.blocks, &p[..i])?.id, r, c))
    }

    /// The selected cells' table and rectangle (merged cells included whole).
    pub fn cell_range(&self) -> Option<(BlockId, CellRange)> {
        let s = self.cell_sel?;
        let t = self.table(s.table)?;
        Some((s.table, t.expand(s.raw(t))))
    }

    /// Top-level blocks of the selected cells.
    pub fn cell_blocks(&self) -> Vec<BlockId> {
        let Some((id, r)) = self.cell_range() else { return vec![] };
        let t = self.table(id).unwrap();
        t.cells_in(r).into_iter().flat_map(|(row, col)| t.rows[row][col].blocks.iter().map(|b| b.id)).collect()
    }

    /// Every text block in the selected cells.
    pub fn cell_text_blocks(&self) -> Vec<BlockId> {
        let Some((id, r)) = self.cell_range() else { return vec![] };
        let t = self.table(id).unwrap();
        let mut out = vec![];
        for (row, col) in t.cells_in(r) {
            text_blocks(&t.rows[row][col].blocks, &mut out);
        }
        out
    }

    /// The selected cells as (row, column) origins.
    pub fn selected_cells(&self) -> Vec<(usize, usize)> {
        self.cell_range().and_then(|(id, r)| Some(self.table(id)?.cells_in(r))).unwrap_or_default()
    }

    /// The selection when it is one merged cell (拆分单元格).
    pub fn merged_selected(&self) -> Option<(BlockId, usize, usize)> {
        let (id, r) = self.cell_range()?;
        let t = self.table(id)?;
        match t.cells_in(r).as_slice() {
            &[(row, col)] if t.rows[row][col].colspan > 1 || t.rows[row][col].rowspan > 1 => Some((id, row, col)),
            _ => None,
        }
    }

    /// The first selected cell, whose style the toolbar shows.
    pub fn first_selected_cell(&self) -> Option<&TableCell> {
        let (id, _) = self.cell_range()?;
        let &(r, c) = self.selected_cells().first()?;
        Some(&self.table(id)?.rows[r][c])
    }

    /// Caret at the start of the cell covering slot (r, c), clamped to the grid.
    pub fn caret_in_cell(&mut self, table: BlockId, r: usize, c: usize) {
        let Some(t) = self.table(table) else { return };
        if t.rows.is_empty() || t.ncols() == 0 {
            return;
        }
        let (r, c) = t.origin(r.min(t.rows.len() - 1), c.min(t.ncols() - 1));
        if let Some(id) = first_leaf(&t.rows[r][c].blocks) {
            self.sel = Selection::caret(Pos::new(id, 0));
        }
    }

    pub fn select_cells(&mut self, s: CellSel) {
        self.block_sel = None;
        self.cell_sel = Some(s);
        self.caret_in_cell(s.table, s.anchor.0, s.anchor.1);
    }

    /// One undo step changing table `id`; false when it isn't a table.
    fn edit_table(&mut self, id: BlockId, f: impl FnOnce(&mut Table)) -> bool {
        if self.table(id).is_none() {
            return false;
        }
        self.checkpoint(EditKind::Other);
        let t = self.table_mut(id).unwrap();
        f(t);
        t.repair();
        true
    }

    /// Pasting exactly one table (empty paragraphs aside) into a table
    /// writes its cells from the selection's top-left (or the caret's) cell,
    /// adding rows and columns as needed, then selects them.
    pub(super) fn paste_table_matrix(&mut self, blocks: &[Block]) -> bool {
        let mut rest = blocks.iter().filter(|b| !(matches!(b.kind, BlockKind::Text) && b.text.is_empty() && b.children.is_empty()));
        let (Some(Block { kind: BlockKind::Table(src), .. }), None) = (rest.next(), rest.next()) else { return false };
        let (table, r0, c0) = match self.cell_range() {
            Some((id, r)) => (id, r.top, r.left),
            None => match self.cell_of(self.sel.head.block) {
                Some(at) => at,
                None => return false,
            },
        };
        let (h, w) = (src.rows.len(), src.ncols());
        if h == 0 || w == 0 {
            return false;
        }
        let src = src.clone();
        self.edit_table(table, |t| {
            while t.rows.len() < r0 + h {
                t.insert_row(t.rows.len());
            }
            while t.ncols() < c0 + w {
                t.insert_col(t.ncols());
            }
            for (r, c, cell) in src.cells() {
                let slot = &mut t.rows[r0 + r][c0 + c];
                if slot.is_covered() {
                    continue;
                }
                slot.blocks = cell.blocks.iter().map(|b| b.fresh_ids()).collect();
                if slot.blocks.is_empty() {
                    slot.blocks.push(Block::text(""));
                }
                if cell.bg.is_some() {
                    slot.bg = cell.bg.clone();
                }
            }
        });
        self.select_cells(CellSel { table, anchor: (r0, c0), head: (r0 + h - 1, c0 + w - 1), kind: CellSelKind::Range });
        true
    }

    /// A new row (or column) before boundary `at`; the caret goes into it.
    pub fn table_insert(&mut self, table: BlockId, cols: bool, at: usize) {
        if !self.edit_table(table, |t| if cols { t.insert_col(at) } else { t.insert_row(at) }) {
            return;
        }
        self.cell_sel = None;
        if cols { self.caret_in_cell(table, 0, at) } else { self.caret_in_cell(table, at, 0) }
    }

    /// Deletes the selected rows / columns; all of them delete the table.
    pub fn table_delete_selected(&mut self) {
        let Some(s) = self.cell_sel else { return };
        let Some(t) = self.table(s.table) else { return };
        let r = s.raw(t);
        let all = match s.kind {
            CellSelKind::Rows => r.top == 0 && r.bottom + 1 >= t.rows.len(),
            CellSelKind::Cols => r.left == 0 && r.right + 1 >= t.ncols(),
            CellSelKind::Range => return,
        };
        let first = t.cells().next().and_then(|(_, _, c)| first_leaf(&c.blocks));
        self.cell_sel = None;
        if all {
            // The unit after the table takes the first cell's place.
            let us = self.units();
            let i = first.and_then(|id| us.iter().position(|u| u.id == id));
            self.delete_block(s.table);
            let us = self.units();
            let target = i.and_then(|i| us.get(i)).or(us.last()).map(|u| u.id).unwrap_or(TITLE);
            self.sel = Selection::caret(Pos::new(target, 0));
            return;
        }
        self.edit_table(s.table, |t| match s.kind {
            CellSelKind::Rows => t.delete_rows(r.top..r.bottom + 1),
            _ => t.delete_cols(r.left..r.right + 1),
        });
        match s.kind {
            CellSelKind::Rows => self.caret_in_cell(s.table, r.top, 0),
            _ => self.caret_in_cell(s.table, 0, r.left),
        }
    }

    pub fn table_merge(&mut self) {
        let Some((id, r)) = self.cell_range() else { return };
        if self.selected_cells().len() < 2 {
            return;
        }
        self.edit_table(id, |t| t.merge(r));
        self.select_cells(CellSel { table: id, anchor: (r.top, r.left), head: (r.top, r.left), kind: CellSelKind::Range });
    }

    /// Splits the selected merged cell; the former span stays selected.
    pub fn table_unmerge(&mut self) {
        let Some((id, r, c)) = self.merged_selected() else { return };
        let cell = &self.table(id).unwrap().rows[r][c];
        let head = (r + cell.rowspan as usize - 1, c + cell.colspan as usize - 1);
        self.edit_table(id, |t| t.unmerge(r, c));
        self.select_cells(CellSel { table: id, anchor: (r, c), head, kind: CellSelKind::Range });
    }

    fn edit_cells(&mut self, f: impl Fn(&mut TableCell)) {
        let Some((id, _)) = self.cell_range() else { return };
        let cells = self.selected_cells();
        self.edit_table(id, |t| cells.iter().for_each(|&(r, c)| f(&mut t.rows[r][c])));
    }

    pub fn table_set_bg(&mut self, bg: Option<ColorName>) {
        self.edit_cells(|c| c.bg = bg.clone());
    }

    /// "middle" / "bottom"; top is the default (None).
    pub fn table_set_valign(&mut self, v: Option<&str>) {
        self.edit_cells(|c| c.valign = v.map(str::to_string));
    }

    /// Empties the selected cells (Backspace, ⌘X); the selection stays.
    pub fn table_clear_cells(&mut self) {
        let Some(s) = self.cell_sel else { return };
        self.edit_cells(|c| c.blocks = vec![Block::text("")]);
        self.caret_in_cell(s.table, s.anchor.0, s.anchor.1);
    }

    /// Commits a column resize; false when nothing changed.
    pub fn table_set_width(&mut self, table: BlockId, col: usize, w: f32) -> bool {
        let w = w.max(Table::MIN_WIDTH).round();
        if self.table(table).and_then(|t| t.col_widths.get(col)).is_none_or(|x| *x == w) {
            return false;
        }
        self.edit_table(table, |t| t.col_widths[col] = w)
    }

    /// 均分列宽: every column gets the average width.
    pub fn table_distribute(&mut self, table: BlockId) {
        self.edit_table(table, |t| {
            let avg = t.col_widths.iter().sum::<f32>() / t.col_widths.len().max(1) as f32;
            t.col_widths.iter_mut().for_each(|w| *w = avg.round());
        });
    }

    pub fn table_toggle_header_row(&mut self, table: BlockId) {
        self.edit_table(table, |t| t.header_rows = if t.header_rows > 0 { 0 } else { 1 });
    }

    pub fn table_toggle_header_col(&mut self, table: BlockId) {
        self.edit_table(table, |t| t.header_col = !t.header_col);
    }

    /// Moves the selected rows / columns before boundary `to`; they stay
    /// selected. False when that leaves them where they are.
    pub fn table_move_selected(&mut self, to: usize) -> bool {
        let Some(s) = self.cell_sel else { return false };
        let Some((id, r)) = self.cell_range() else { return false };
        let cols = match s.kind {
            CellSelKind::Cols => true,
            CellSelKind::Rows => false,
            CellSelKind::Range => return false,
        };
        let (a, b) = if cols { (r.left, r.right) } else { (r.top, r.bottom) };
        if (a..=b + 1).contains(&to) {
            return false;
        }
        self.edit_table(id, |t| if cols { t.move_cols(a..b + 1, to) } else { t.move_rows(a..b + 1, to) });
        let (a, b) = if to < a { (to, to + b - a) } else { (to - (b + 1 - a), to - 1) };
        let (anchor, head) = if cols { ((0, a), (0, b)) } else { ((a, 0), (b, 0)) };
        self.select_cells(CellSel { table: id, anchor, head, kind: s.kind });
        true
    }

    /// The selected cells as a table of their own (copy).
    pub fn selected_cells_content(&self) -> Vec<Block> {
        let Some((id, r)) = self.cell_range() else { return vec![] };
        let Some(t) = self.table(id) else { return vec![] };
        let mut sub = Table {
            col_widths: t.col_widths[r.left..=r.right].to_vec(),
            header_rows: if r.top == 0 { t.header_rows } else { 0 },
            header_col: t.header_col && r.left == 0,
            rows: t.rows[r.top..=r.bottom].iter().map(|row| row[r.left..=r.right].to_vec()).collect(),
        };
        sub.repair();
        vec![Block::new(BlockKind::Table(sub), RichText::default()).fresh_ids()]
    }
}

#[cfg(test)]
mod tests {
    use super::{CellSel, CellSelKind};
    use crate::editor::core::Core;
    use crate::model::*;

    fn core_with(nr: usize, nc: usize) -> (Core, BlockId) {
        let t = Table {
            col_widths: vec![Table::DEFAULT_WIDTH; nc],
            header_rows: 0,
            header_col: false,
            rows: (0..nr)
                .map(|r| (0..nc).map(|c| TableCell { blocks: vec![Block::text(&format!("{r}{c}"))], ..TableCell::empty() }).collect())
                .collect(),
        };
        let b = Block::new(BlockKind::Table(t), RichText::default());
        let id = b.id;
        let core = Core::new(Document { title: RichText::plain("t"), blocks: vec![b, Block::text("after")], ..Default::default() });
        (core, id)
    }

    fn text(c: &Core, id: BlockId, r: usize, col: usize) -> String {
        c.table(id).unwrap().rows[r][col].blocks.iter().map(|b| b.text.text.clone()).collect::<Vec<_>>().join("|")
    }

    #[test]
    fn bar_selection_covers_whole_rows_and_columns() {
        let (mut c, id) = core_with(3, 3);
        c.select_cells(CellSel { table: id, anchor: (1, 0), head: (2, 0), kind: CellSelKind::Rows });
        assert_eq!(c.selected_cells().len(), 6);
        assert_eq!(c.text_of(c.sel.head.block).unwrap().text, "10");
        c.select_cells(CellSel { table: id, anchor: (0, 2), head: (0, 2), kind: CellSelKind::Cols });
        assert_eq!(c.selected_cells(), vec![(0, 2), (1, 2), (2, 2)]);
    }

    #[test]
    fn delete_rows_and_whole_table() {
        let (mut c, id) = core_with(3, 2);
        c.select_cells(CellSel { table: id, anchor: (0, 0), head: (1, 0), kind: CellSelKind::Rows });
        c.table_delete_selected();
        assert_eq!(c.table(id).unwrap().rows.len(), 1);
        assert_eq!(text(&c, id, 0, 0), "20");
        c.select_cells(CellSel { table: id, anchor: (0, 0), head: (0, 1), kind: CellSelKind::Cols });
        c.table_delete_selected();
        assert!(c.table(id).is_none());
        assert_eq!(c.text_of(c.sel.head.block).unwrap().text, "after");
    }

    #[test]
    fn merge_and_split() {
        let (mut c, id) = core_with(2, 3);
        c.select_cells(CellSel { table: id, anchor: (0, 0), head: (1, 1), kind: CellSelKind::Range });
        c.table_merge();
        let t = c.table(id).unwrap();
        assert_eq!((t.rows[0][0].rowspan, t.rows[0][0].colspan), (2, 2));
        assert_eq!(text(&c, id, 0, 0), "00|01|10|11");
        assert_eq!(c.merged_selected(), Some((id, 0, 0)));
        c.table_unmerge();
        assert_eq!(c.selected_cells().len(), 4);
        assert_eq!(text(&c, id, 1, 1), "");
        assert!(c.undo() && c.undo());
        assert_eq!(text(&c, id, 1, 1), "11");
    }

    #[test]
    fn insert_puts_caret_in_new_cells() {
        let (mut c, id) = core_with(2, 2);
        c.table_insert(id, true, 1);
        assert_eq!(c.table(id).unwrap().ncols(), 3);
        assert_eq!(c.cell_of(c.sel.head.block), Some((id, 0, 1)));
        c.table_insert(id, false, 2);
        assert_eq!(c.table(id).unwrap().rows.len(), 3);
        assert_eq!(c.cell_of(c.sel.head.block), Some((id, 2, 0)));
    }

    #[test]
    fn cell_styles_widths_and_headers() {
        let (mut c, id) = core_with(2, 2);
        c.select_cells(CellSel { table: id, anchor: (0, 0), head: (0, 1), kind: CellSelKind::Range });
        c.table_set_bg(Some(ColorName("light-blue".into())));
        c.table_set_valign(Some("middle"));
        let t = c.table(id).unwrap();
        assert!(t.rows[0][1].bg.is_some() && t.rows[1][0].bg.is_none());
        assert_eq!(t.rows[0][0].valign.as_deref(), Some("middle"));
        assert!(c.table_set_width(id, 0, 20.));
        assert_eq!(c.table(id).unwrap().col_widths[0], Table::MIN_WIDTH);
        c.table_distribute(id);
        assert_eq!(c.table(id).unwrap().col_widths, vec![75., 75.]);
        c.table_toggle_header_row(id);
        c.table_toggle_header_col(id);
        let t = c.table(id).unwrap();
        assert!(t.header_rows == 1 && t.header_col);
    }

    #[test]
    fn clear_and_copy_cells() {
        let (mut c, id) = core_with(2, 2);
        c.select_cells(CellSel { table: id, anchor: (1, 0), head: (1, 1), kind: CellSelKind::Range });
        let copied = c.selected_cells_content();
        let BlockKind::Table(t) = &copied[0].kind else { panic!() };
        assert_eq!((t.rows.len(), t.ncols()), (1, 2));
        assert_eq!(t.rows[0][1].blocks[0].text.text, "11");
        c.table_clear_cells();
        assert_eq!(text(&c, id, 1, 1), "");
        assert_eq!(text(&c, id, 0, 1), "01");
        assert!(c.cell_sel.is_some());
        assert_eq!(c.cell_text_blocks().len(), 2);
    }

    #[test]
    fn move_rows_and_columns() {
        let (mut c, id) = core_with(4, 3);
        c.select_cells(CellSel { table: id, anchor: (0, 0), head: (1, 0), kind: CellSelKind::Rows });
        // Next to themselves: nothing moves.
        assert!(!c.table_move_selected(2));
        assert!(c.table_move_selected(4));
        assert_eq!((text(&c, id, 0, 0), text(&c, id, 2, 1), text(&c, id, 3, 2)), ("20".into(), "01".into(), "12".into()));
        assert_eq!(c.cell_range().map(|(_, r)| (r.top, r.bottom)), Some((2, 3)));
        c.select_cells(CellSel { table: id, anchor: (0, 2), head: (0, 2), kind: CellSelKind::Cols });
        c.table_set_width(id, 2, 180.);
        assert!(c.table_move_selected(0));
        let t = c.table(id).unwrap();
        assert_eq!(t.col_widths, vec![180., 100., 100.]);
        assert_eq!((text(&c, id, 0, 0), text(&c, id, 0, 1)), ("22".into(), "20".into()));
        assert_eq!(c.cell_range().map(|(_, r)| (r.left, r.right)), Some((0, 0)));
        assert!(c.undo());
        assert_eq!(text(&c, id, 0, 0), "20");
    }

    #[test]
    fn moved_rows_keep_merged_cells() {
        let (mut c, id) = core_with(4, 2);
        c.select_cells(CellSel { table: id, anchor: (2, 0), head: (3, 0), kind: CellSelKind::Range });
        c.table_merge();
        // Selecting row 3 takes in row 2, which the merged cell spans.
        c.select_cells(CellSel { table: id, anchor: (3, 0), head: (3, 0), kind: CellSelKind::Rows });
        assert!(c.table_move_selected(0));
        let t = c.table(id).unwrap();
        assert_eq!((t.rows[0][0].rowspan, t.rows[1][0].is_covered()), (2, true));
        assert_eq!((text(&c, id, 0, 0), text(&c, id, 0, 1), text(&c, id, 2, 0)), ("20|30".into(), "21".into(), "00".into()));
    }
}
