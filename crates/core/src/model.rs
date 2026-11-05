//! In-memory document model. Mirrors the DocxXML block structure closely so
//! that load → edit → save round-trips without loss.

use std::ops::Range;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BlockId(pub u64);

impl BlockId {
    /// A fresh id; deliberately not `Default`, which would read as a fixed value.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        BlockId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

/// What a document with an empty title is called.
pub const UNTITLED: &str = "未命名文档";

/// A document title as shown in lists and links.
pub fn display_title(t: &str) -> String {
    let t = t.trim();
    if t.is_empty() { UNTITLED.to_string() } else { t.to_string() }
}

/// Named palette color as used by DocxXML (`red`, `light-blue`, `medium-gray`, ...).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ColorName(pub String);

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Marks {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    pub code: bool,
    pub link: Option<String>,
    pub text_color: Option<ColorName>,
    pub bg_color: Option<ColorName>,
    /// Inline object (equation, button, mention, ...). Each object is one
    /// U+FFFC in the text.
    pub inline: Option<Inline>,
}

/// An inline object. `xml` is its element exactly as read or as
/// `docxml::to_inline` wrote it, and is what gets saved, so attributes we
/// don't model survive; `kind` is what it means, parsed once on load.
#[derive(Clone, Debug)]
pub struct Inline {
    pub kind: InlineKind,
    pub xml: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InlineKind {
    /// `<latex>`: its LaTeX source.
    Equation(String),
    /// `<source>` in a paragraph (文本视图); the card it becomes as a block.
    File(FileBlock),
    Button(InlineButton),
    Reminder(InlineReminder),
    DocLink(InlineDocLink),
    /// Kept verbatim and shown as its text (`<cite type="user">`, ...).
    Other { tag: String },
}

// `kind` follows from `xml`.
impl PartialEq for Inline {
    fn eq(&self, other: &Self) -> bool {
        self.xml == other.xml
    }
}

impl Eq for Inline {}

impl std::hash::Hash for Inline {
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
        self.xml.hash(h)
    }
}

impl Inline {
    pub fn equation(&self) -> Option<&str> {
        match &self.kind {
            InlineKind::Equation(src) => Some(src),
            _ => None,
        }
    }

    pub fn file(&self) -> Option<&FileBlock> {
        match &self.kind {
            InlineKind::File(f) => Some(f),
            _ => None,
        }
    }

    pub fn button(&self) -> Option<&InlineButton> {
        match &self.kind {
            InlineKind::Button(b) => Some(b),
            _ => None,
        }
    }

    pub fn reminder(&self) -> Option<&InlineReminder> {
        match &self.kind {
            InlineKind::Reminder(r) => Some(r),
            _ => None,
        }
    }

    pub fn doc_link(&self) -> Option<&InlineDocLink> {
        match &self.kind {
            InlineKind::DocLink(l) => Some(l),
            _ => None,
        }
    }
}

/// 按钮 (`<button action src background-color>文字</button>`).
#[derive(Clone, Debug, PartialEq)]
pub struct InlineButton {
    pub text: String,
    pub action: ButtonAction,
    /// OpenLink's URL.
    pub src: String,
    /// `light-blue`, `deep-red`, ... (orange also arrives as `origin`; both
    /// are accepted).
    pub color: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonAction {
    OpenLink,
    DuplicatePage,
    FollowPage,
}

impl ButtonAction {
    pub fn name(self) -> &'static str {
        match self {
            ButtonAction::OpenLink => "OpenLink",
            ButtonAction::DuplicatePage => "DuplicatePage",
            ButtonAction::FollowPage => "FollowPage",
        }
    }

    /// The type menu's label, also a new button's text.
    pub fn label(self) -> &'static str {
        match self {
            ButtonAction::OpenLink => "打开超链接",
            ButtonAction::DuplicatePage => "创建副本",
            ButtonAction::FollowPage => "关注文档更新",
        }
    }
}

pub const BUTTON_DEFAULT_COLOR: &str = "light-blue";

impl InlineButton {
    pub fn new(action: ButtonAction) -> Self {
        InlineButton { text: action.label().into(), action, src: String::new(), color: BUTTON_DEFAULT_COLOR.into() }
    }
}

/// 日期提醒 (`<time>`); times are unix ms. DocxXML 里没有全天标记
/// （模型里有 `isWholeDay`）：我们写 `is-whole-day`；没有这个属性、
/// 且到期时间是本地 23:59 的提醒，也按全天处理。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InlineReminder {
    pub expire: i64,
    pub notify: i64,
    pub should_notify: bool,
    pub whole_day: bool,
}

impl InlineReminder {
    /// `LL` and, unless whole-day, `sep` + `LT` ("2026年10月23日，14:00").
    pub fn label(&self, sep: &str) -> String {
        let (y, m, d, h, mi) = crate::time::civil(self.expire.max(0) as u64 / 1000);
        if self.whole_day { format!("{y}年{m}月{d}日") } else { format!("{y}年{m}月{d}日{sep}{h:02}:{mi:02}") }
    }
}

/// @文档 a mention (`<cite type="doc" doc-id file-type>`).
/// The title is kept as its text, for exports and for a document that is
/// gone; the live title comes from the library.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InlineDocLink {
    pub id: String,
    pub title: String,
    pub alias: Option<String>,
    pub pending: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Run {
    pub len: usize,
    pub marks: Marks,
}

/// Plain text plus a run-length list of marks. `runs` always covers `text`
/// exactly (sum of lens == text.len()), with no zero-length runs and adjacent
/// runs having different marks.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RichText {
    pub text: String,
    pub runs: Vec<Run>,
}

impl RichText {
    /// The text with equations replaced by their LaTeX source and other inline
    /// objects dropped.
    pub fn plain_text(&self) -> String {
        if !self.text.contains('\u{fffc}') {
            return self.text.clone();
        }
        let mut out = String::with_capacity(self.text.len());
        for (r, m) in self.spans() {
            match m.inline.as_ref().and_then(Inline::equation) {
                Some(src) => {
                    for _ in self.text[r].chars() {
                        out.push_str(src);
                    }
                }
                None => out.push_str(&self.text[r].replace('\u{fffc}', "")),
            }
        }
        out
    }

    pub fn plain(s: impl Into<String>) -> Self {
        let text: String = s.into();
        let runs = if text.is_empty() {
            vec![]
        } else {
            vec![Run { len: text.len(), marks: Marks::default() }]
        };
        RichText { text, runs }
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn push(&mut self, s: &str, marks: Marks) {
        if s.is_empty() {
            return;
        }
        self.text.push_str(s);
        match self.runs.last_mut() {
            Some(last) if last.marks == marks => last.len += s.len(),
            _ => self.runs.push(Run { len: s.len(), marks }),
        }
    }

    pub fn append(&mut self, other: &RichText) {
        let mut off = 0;
        for run in &other.runs {
            self.push(&other.text[off..off + run.len], run.marks.clone());
            off += run.len;
        }
    }

    /// Iterate `(range, marks)` for every run.
    pub fn spans(&self) -> impl Iterator<Item = (Range<usize>, &Marks)> {
        let mut off = 0;
        self.runs.iter().map(move |r| {
            let range = off..off + r.len;
            off += r.len;
            (range, &r.marks)
        })
    }

    pub fn marks_at(&self, offset: usize) -> Marks {
        // Marks that typing at `offset` should inherit: the run ending at or
        // containing offset (prefer the left side, like most editors).
        let mut last = Marks::default();
        for (range, marks) in self.spans() {
            if range.start < offset && offset < range.end {
                // Strictly inside a run: links and inline code extend too.
                return Marks { inline: None, ..marks.clone() };
            }
            if range.start < offset && offset == range.end {
                return typing_marks(marks);
            }
            if range.start >= offset {
                if offset == 0 {
                    return typing_marks(marks);
                }
                break;
            }
            last = marks.clone();
        }
        typing_marks(&last)
    }

    /// `range` clamped to the text and moved back onto character
    /// boundaries, so a stale offset can't panic in a string slice.
    fn clamp(&self, range: Range<usize>) -> Range<usize> {
        debug_assert!(self.text.is_char_boundary(range.start.min(self.len())) && self.text.is_char_boundary(range.end.min(self.len())), "{range:?} in {:?}", self.text);
        let e = self.text.floor_char_boundary(range.end);
        self.text.floor_char_boundary(range.start).min(e)..e
    }

    pub fn slice(&self, range: Range<usize>) -> RichText {
        let range = self.clamp(range);
        let mut out = RichText::default();
        for (r, marks) in self.spans() {
            let s = r.start.max(range.start);
            let e = r.end.min(range.end);
            if s < e {
                out.push(&self.text[s..e], marks.clone());
            }
        }
        out
    }

    pub fn replace(&mut self, range: Range<usize>, insert: &RichText) {
        let mut out = self.slice(0..range.start);
        out.append(insert);
        out.append(&self.slice(range.end..self.len()));
        *self = out;
    }

    pub fn insert_plain(&mut self, offset: usize, s: &str, marks: Marks) {
        let mut ins = RichText::default();
        ins.push(s, marks);
        self.replace(offset..offset, &ins);
    }

    pub fn split_off(&mut self, offset: usize) -> RichText {
        let tail = self.slice(offset..self.len());
        *self = self.slice(0..offset);
        tail
    }

    /// Apply `f` to marks of every run overlapping `range`.
    pub fn update_marks(&mut self, range: Range<usize>, f: impl Fn(&mut Marks)) {
        let range = self.clamp(range);
        if range.is_empty() {
            return;
        }
        let mut out = RichText::default();
        for (r, marks) in self.spans() {
            let a = r.start;
            let b = r.end;
            let s = a.max(range.start).min(b);
            let e = b.min(range.end).max(s);
            out.push(&self.text[a..s], marks.clone());
            let mut m = marks.clone();
            f(&mut m);
            out.push(&self.text[s..e], m);
            out.push(&self.text[e..b], marks.clone());
        }
        *self = out;
    }

    /// True if every char in range satisfies predicate.
    pub fn all_marks(&self, range: Range<usize>, f: impl Fn(&Marks) -> bool) -> bool {
        if range.is_empty() {
            return f(&self.marks_at(range.start));
        }
        self.spans()
            .filter(|(r, _)| r.start < range.end && r.end > range.start)
            .all(|(_, m)| f(m))
    }
}

/// Marks inherited by newly typed text: links, code and inline objects don't extend.
fn typing_marks(m: &Marks) -> Marks {
    Marks {
        link: None,
        code: false,
        inline: None,
        ..m.clone()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Align {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TableCell {
    pub blocks: Vec<Block>,
    pub bg: Option<ColorName>,
    pub valign: Option<String>,
    pub colspan: u32,
    pub rowspan: u32,
}

impl TableCell {
    pub fn empty() -> Self {
        TableCell {
            blocks: vec![Block::text("")],
            bg: None,
            valign: None,
            colspan: 1,
            rowspan: 1,
        }
    }

    /// A grid slot hidden under a merged cell (spans 0 × 0, no content).
    pub fn covered() -> Self {
        TableCell { blocks: vec![], bg: None, valign: None, colspan: 0, rowspan: 0 }
    }

    pub fn is_covered(&self) -> bool {
        self.colspan == 0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub col_widths: Vec<f32>,
    pub header_rows: usize,
    /// The first column is a header column (标题列).
    pub header_col: bool,
    /// A full rows × columns grid; slots hidden under a merged cell hold
    /// [`TableCell::covered`] (DocxXML simply omits them).
    pub rows: Vec<Vec<TableCell>>,
}

impl Table {
    pub const DEFAULT_WIDTH: f32 = 100.;
    pub const MIN_WIDTH: f32 = 50.;

    pub fn ncols(&self) -> usize {
        self.col_widths.len()
    }

    /// The visible cell whose span covers slot `(r, c)`.
    pub fn origin(&self, r: usize, c: usize) -> (usize, usize) {
        for or in (0..=r).rev() {
            for oc in (0..=c).rev() {
                let cell = &self.rows[or][oc];
                if !cell.is_covered() && or + cell.rowspan as usize > r && oc + cell.colspan as usize > c {
                    return (or, oc);
                }
            }
        }
        (r, c)
    }

    /// Visible cells in reading order.
    pub fn cells(&self) -> impl Iterator<Item = (usize, usize, &TableCell)> {
        self.rows
            .iter()
            .enumerate()
            .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, cell)| (r, c, cell)))
            .filter(|(_, _, cell)| !cell.is_covered())
    }

    /// Make the grid rectangular and every span fit: rows padded to the column
    /// count, spans clipped to the grid, and orphaned covered slots revived.
    pub fn repair(&mut self) {
        let n = self.rows.iter().map(|r| r.len()).max().unwrap_or(0).max(self.col_widths.len()).max(1);
        self.col_widths.resize(n, Self::DEFAULT_WIDTH);
        if self.rows.is_empty() {
            self.rows.push(vec![]);
        }
        for row in &mut self.rows {
            row.resize_with(n, TableCell::empty);
        }
        let nrows = self.rows.len();
        let mut owned = vec![vec![false; n]; nrows];
        for r in 0..nrows {
            for c in 0..n {
                if owned[r][c] {
                    if !self.rows[r][c].is_covered() {
                        self.rows[r][c] = TableCell::covered();
                    }
                    continue;
                }
                let cell = &mut self.rows[r][c];
                if cell.is_covered() {
                    *cell = TableCell::empty();
                }
                let mut cs = (cell.colspan.max(1) as usize).min(n - c);
                // A span may not run into a slot an earlier cell already owns.
                if let Some(k) = (1..cs).find(|k| owned[r][c + k]) {
                    cs = k;
                }
                let rs = (cell.rowspan.max(1) as usize).min(nrows - r);
                let rs = (1..rs).find(|k| (c..c + cs).any(|x| owned[r + k][x])).unwrap_or(rs);
                cell.colspan = cs as u32;
                cell.rowspan = rs as u32;
                for row in owned.iter_mut().skip(r).take(rs) {
                    row[c..c + cs].iter_mut().for_each(|o| *o = true);
                }
            }
        }
        self.header_rows = self.header_rows.min(nrows);
    }

    /// Visible cells as `(top, left, rowspan, colspan, cell)`.
    fn take_cells(&mut self) -> Vec<(usize, usize, usize, usize, TableCell)> {
        let mut out = vec![];
        for (r, row) in std::mem::take(&mut self.rows).into_iter().enumerate() {
            for (c, cell) in row.into_iter().enumerate() {
                if !cell.is_covered() {
                    out.push((r, c, cell.rowspan as usize, cell.colspan as usize, cell));
                }
            }
        }
        out
    }

    /// Lays cells out on an `nrows × ncols` grid; free slots get empty cells.
    fn put_cells(&mut self, nrows: usize, cells: Vec<(usize, usize, usize, usize, TableCell)>) {
        let n = self.col_widths.len();
        let mut rows: Vec<Vec<Option<TableCell>>> = (0..nrows).map(|_| (0..n).map(|_| None).collect()).collect();
        for (r, c, rs, cs, mut cell) in cells {
            if rs == 0 || cs == 0 || r >= nrows || c >= n {
                continue;
            }
            cell.rowspan = rs as u32;
            cell.colspan = cs as u32;
            for row in rows.iter_mut().skip(r).take(rs) {
                for slot in row.iter_mut().skip(c).take(cs) {
                    *slot = Some(TableCell::covered());
                }
            }
            rows[r][c] = Some(cell);
        }
        self.rows = rows.into_iter().map(|row| row.into_iter().map(|c| c.unwrap_or_else(TableCell::empty)).collect()).collect();
        self.repair();
    }

    /// A new empty row before row `at`; cells merged across that boundary grow.
    pub fn insert_row(&mut self, at: usize) {
        let nrows = self.rows.len() + 1;
        let cells = self.take_cells().into_iter().map(|(r, c, rs, cs, cell)| {
            let (top, end) = (r + (r >= at) as usize, r + rs + (r + rs > at) as usize);
            (top, c, end - top, cs, cell)
        });
        let cells = cells.collect();
        self.put_cells(nrows, cells);
    }

    /// A new empty column before column `at` (default width).
    pub fn insert_col(&mut self, at: usize) {
        let at = at.min(self.col_widths.len());
        let nrows = self.rows.len();
        let cells = self.take_cells().into_iter().map(|(r, c, rs, cs, cell)| {
            let (left, end) = (c + (c >= at) as usize, c + cs + (c + cs > at) as usize);
            (r, left, rs, end - left, cell)
        });
        let cells = cells.collect();
        self.col_widths.insert(at, Self::DEFAULT_WIDTH);
        self.put_cells(nrows, cells);
    }

    /// Removes rows `range`; merged cells shrink, and one that started in
    /// the range moves (with its content) to the first row after it.
    pub fn delete_rows(&mut self, range: std::ops::Range<usize>) {
        let (a, b) = (range.start, range.end.min(self.rows.len()));
        if a >= b {
            return;
        }
        let f = |x: usize| if x <= a { x } else if x >= b { x - (b - a) } else { a };
        let nrows = self.rows.len() - (b - a);
        let cells = self.take_cells().into_iter().map(|(r, c, rs, cs, cell)| {
            let (top, end) = (f(r), f(r + rs));
            (top, c, end.saturating_sub(top), cs, cell)
        });
        let cells = cells.collect();
        self.put_cells(nrows, cells);
    }

    pub fn delete_cols(&mut self, range: std::ops::Range<usize>) {
        let (a, b) = (range.start, range.end.min(self.col_widths.len()));
        if a >= b {
            return;
        }
        let f = |x: usize| if x <= a { x } else if x >= b { x - (b - a) } else { a };
        let nrows = self.rows.len();
        let cells = self.take_cells().into_iter().map(|(r, c, rs, cs, cell)| {
            let (left, end) = (f(c), f(c + cs));
            (r, left, rs, end.saturating_sub(left), cell)
        });
        let cells = cells.collect();
        self.col_widths.drain(a..b);
        self.put_cells(nrows, cells);
    }

    /// Moves rows `range` before row `to` (numbered as before the move, not
    /// inside the range). Callers keep merged cells whole: none may cross
    /// the range's edges or boundary `to`.
    pub fn move_rows(&mut self, range: std::ops::Range<usize>, to: usize) {
        let order = reordered(self.rows.len(), range, to);
        self.rows = permute(std::mem::take(&mut self.rows), &order);
        self.repair();
    }

    pub fn move_cols(&mut self, range: std::ops::Range<usize>, to: usize) {
        let order = reordered(self.col_widths.len(), range, to);
        self.col_widths = order.iter().map(|&i| self.col_widths[i]).collect();
        for row in &mut self.rows {
            *row = permute(std::mem::take(row), &order);
        }
        self.repair();
    }

    /// The smallest rectangle holding `r` that no merged cell crosses.
    pub fn expand(&self, mut r: CellRange) -> CellRange {
        loop {
            let mut grown = r;
            for row in r.top..=r.bottom {
                for col in r.left..=r.right {
                    let (or, oc) = self.origin(row, col);
                    let cell = &self.rows[or][oc];
                    grown.top = grown.top.min(or);
                    grown.left = grown.left.min(oc);
                    grown.bottom = grown.bottom.max(or + cell.rowspan.max(1) as usize - 1);
                    grown.right = grown.right.max(oc + cell.colspan.max(1) as usize - 1);
                }
            }
            if grown == r {
                return r;
            }
            r = grown;
        }
    }

    /// Visible cells inside `r` (reading order).
    pub fn cells_in(&self, r: CellRange) -> Vec<(usize, usize)> {
        self.cells().filter(|(row, col, _)| r.contains(*row, *col)).map(|(row, col, _)| (row, col)).collect()
    }

    /// Merges `r` into its top-left cell, which takes every non-empty
    /// cell's content in reading order.
    pub fn merge(&mut self, r: CellRange) {
        let r = self.expand(r);
        let cells = self.cells_in(r);
        if cells.len() < 2 {
            return;
        }
        let mut blocks = vec![];
        for &(row, col) in &cells {
            let cell = std::mem::replace(&mut self.rows[row][col], TableCell::covered());
            let empty = cell.blocks.iter().all(|b| b.text.is_empty() && b.children.is_empty() && b.kind.has_text());
            if !empty {
                blocks.extend(cell.blocks);
            }
            if (row, col) == (r.top, r.left) {
                self.rows[row][col] = TableCell { blocks: vec![], ..cell };
            }
        }
        let origin = &mut self.rows[r.top][r.left];
        if blocks.is_empty() {
            blocks.push(Block::text(""));
        }
        origin.blocks = blocks;
        origin.rowspan = (r.bottom - r.top + 1) as u32;
        origin.colspan = (r.right - r.left + 1) as u32;
    }

    /// Splits the merged cell at `(r, c)`; its content stays in the first slot.
    pub fn unmerge(&mut self, r: usize, c: usize) {
        let cell = &mut self.rows[r][c];
        let (rs, cs) = (cell.rowspan as usize, cell.colspan as usize);
        cell.rowspan = 1;
        cell.colspan = 1;
        for row in self.rows.iter_mut().skip(r).take(rs) {
            for slot in row.iter_mut().skip(c).take(cs) {
                if slot.is_covered() {
                    *slot = TableCell::empty();
                }
            }
        }
    }

}

/// The old index of each of `n` lines after `range` moves before `to`.
fn reordered(n: usize, range: std::ops::Range<usize>, to: usize) -> Vec<usize> {
    let mut order: Vec<usize> = (0..n).filter(|i| !range.contains(i)).collect();
    let at = order.iter().filter(|&&i| i < to).count();
    order.splice(at..at, range);
    order
}

fn permute<T>(v: Vec<T>, order: &[usize]) -> Vec<T> {
    let mut old: Vec<Option<T>> = v.into_iter().map(Some).collect();
    order.iter().filter_map(|&i| old.get_mut(i)?.take()).collect()
}

/// An inclusive rectangle of grid slots.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellRange {
    pub top: usize,
    pub left: usize,
    pub bottom: usize,
    pub right: usize,
}

impl CellRange {
    pub fn span(a: (usize, usize), b: (usize, usize)) -> Self {
        CellRange { top: a.0.min(b.0), left: a.1.min(b.1), bottom: a.0.max(b.0), right: a.1.max(b.1) }
    }

    pub fn contains(&self, r: usize, c: usize) -> bool {
        (self.top..=self.bottom).contains(&r) && (self.left..=self.right).contains(&c)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BlockKind {
    Text,
    Heading { level: u8, seq_auto: bool },
    Bullet,
    Ordered { seq: Option<u32> },
    Todo { done: bool },
    Code { lang: String, caption: Option<String>, wrap: bool },
    Quote,
    Callout {
        emoji: String,
        bg: Option<ColorName>,
        border: Option<ColorName>,
        text_color: Option<ColorName>,
    },
    Divider,
    Image {
        /// Which DocxXML attribute carries the source: `path`, `href` or `src`.
        src_kind: String,
        src: String,
        /// Displayed size in px of the visible (cropped) part; None: the
        /// natural size.
        width: Option<f32>,
        height: Option<f32>,
        caption: Option<String>,
        /// Visible part as [left, top, right, bottom] fractions of the full
        /// image (the `image.crop` attribute); None: uncropped.
        crop: Option<[f32; 4]>,
    },
    Table(Table),
    Grid,
    /// One column of a grid; children hold content.
    Column { ratio: f32 },
    /// 子页面列表 (`<sub-page-list/>`): lists the document's child pages.
    SubPageList,
    /// 倒计时 (an add-on block; `<countdown …/>` is ours).
    Countdown(Countdown),
    /// 视频或文件 (`<source>`, or `<figure view-type><source/></figure>`).
    File(FileBlock),
    /// Any element we don't model; kept verbatim for round-tripping.
    Raw { tag: String, xml: String },
}

impl BlockKind {
    /// Blocks that own a rich-text line that the caret can enter.
    pub fn has_text(&self) -> bool {
        matches!(
            self,
            BlockKind::Text
                | BlockKind::Heading { .. }
                | BlockKind::Bullet
                | BlockKind::Ordered { .. }
                | BlockKind::Todo { .. }
                | BlockKind::Code { .. }
        )
    }

    /// Container blocks whose children are rendered inside them.
    pub fn is_container(&self) -> bool {
        matches!(
            self,
            BlockKind::Quote | BlockKind::Callout { .. } | BlockKind::Grid | BlockKind::Column { .. }
        )
    }

    /// Text blocks that take indented children (Tab, drop as first child).
    pub fn nests(&self) -> bool {
        matches!(self, BlockKind::Text | BlockKind::Heading { .. } | BlockKind::Bullet | BlockKind::Ordered { .. } | BlockKind::Todo { .. })
    }

    pub fn is_list(&self) -> bool {
        matches!(self, BlockKind::Bullet | BlockKind::Ordered { .. } | BlockKind::Todo { .. })
    }

    /// Blocks the handle menu's 颜色 can give a background (the
    /// `background_color` block style).
    pub fn takes_bg(&self) -> bool {
        matches!(
            self,
            BlockKind::Text | BlockKind::Heading { .. } | BlockKind::Bullet | BlockKind::Ordered { .. } | BlockKind::Todo { .. } | BlockKind::Quote
        )
    }
}

/// The countdown add-on's record: `{duration, startTime, isNotify, color,
/// timingType, settingData}`.
#[derive(Clone, Debug, PartialEq)]
pub struct Countdown {
    /// Seconds; the set length plus one, as the add-on stores it.
    pub duration: f64,
    /// Unix ms the countdown started; 0: never set (shows 00:00:00:00).
    pub start: u64,
    /// 倒计时结束时显示气泡提醒.
    pub notify: bool,
    /// One of `COUNTDOWN_COLORS`.
    pub color: String,
    /// 选择具体日期 (else 输入倒计时时长).
    pub by_date: bool,
    /// The settings dialog's last values: days, hours, minutes, seconds.
    pub dhms: [u32; 4],
    /// …and the end as "YYYY-MM-DD" / "HH:mm".
    pub date: String,
    pub time: String,
}

/// An attachment: a local file (`path`, under the library's assets) or a
/// cloud token we can't fetch.
#[derive(Clone, Debug, PartialEq)]
pub struct FileBlock {
    /// `path` or `token`.
    pub src_kind: String,
    pub src: String,
    pub name: String,
    pub view: FileView,
}

/// 卡片视图 / 预览视图 (文本视图 is an inline `<source>` in a paragraph).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FileView {
    Card,
    Preview,
}

/// 红 / 橙 / 黄 / 绿 / 蓝 / 紫 / 灰; orange is the default.
pub const COUNTDOWN_COLORS: [&str; 7] = ["#F76964", "#FF8800", "#FFC60A", "#62D256", "#4E83FD", "#935AF6", "#BBBFC4"];

impl Default for Countdown {
    fn default() -> Self {
        Countdown {
            duration: 0.,
            start: 0,
            notify: true,
            color: COUNTDOWN_COLORS[1].into(),
            by_date: false,
            dhms: [0; 4],
            date: String::new(),
            time: String::new(),
        }
    }
}

impl Countdown {
    /// Whole seconds left at `now_ms` (`round(duration) - ceil(elapsed)`),
    /// never below 0.
    pub fn remaining(&self, now_ms: u64) -> u64 {
        if self.start == 0 {
            return 0;
        }
        let elapsed = (now_ms.saturating_sub(self.start) as f64 / 1000.).ceil();
        (self.duration.round() - elapsed).max(0.) as u64
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub id: BlockId,
    pub kind: BlockKind,
    pub text: RichText,
    pub align: Align,
    /// Whole-block background (`background-color` on the block's tag), drawn
    /// behind the block and its children.
    pub bg: Option<ColorName>,
    pub children: Vec<Block>,
    /// Collapsed (`folded="true"`): hides children, and for a heading its
    /// section too.
    pub folded: bool,
}

impl Block {
    pub fn new(kind: BlockKind, text: RichText) -> Self {
        Block {
            id: BlockId::new(),
            kind,
            text,
            align: Align::Left,
            bg: None,
            children: vec![],
            folded: false,
        }
    }

    pub fn text(s: &str) -> Self {
        Block::new(BlockKind::Text, RichText::plain(s))
    }

    pub fn with_children(mut self, children: Vec<Block>) -> Self {
        self.children = children;
        self
    }

    /// Deep clone with fresh ids (used for paste / duplicate).
    pub fn fresh_ids(&self) -> Block {
        let mut b = self.clone();
        fn walk(b: &mut Block) {
            b.id = BlockId::new();
            for c in &mut b.children {
                walk(c);
            }
            if let BlockKind::Table(t) = &mut b.kind {
                for row in &mut t.rows {
                    for cell in row {
                        for c in &mut cell.blocks {
                            walk(c);
                        }
                    }
                }
            }
        }
        walk(&mut b);
        b
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Document {
    pub title: RichText,
    /// 文档图标: an emoji shown before the title (file-level, not undone).
    pub icon: Option<String>,
    pub cover: Option<Cover>,
    pub blocks: Vec<Block>,
}

/// 封面: a local image plus `offset_ratio_x/y` (0 = centred).
#[derive(Clone, Debug, PartialEq)]
pub struct Cover {
    pub src: String,
    pub x: f32,
    pub y: f32,
}

impl Default for Document {
    fn default() -> Self {
        Document {
            title: RichText::default(),
            icon: None,
            cover: None,
            blocks: vec![Block::text("")],
        }
    }
}

/// An @文档 mention: the document it points at, the block holding it, and
/// that block's text as plain runs (`Some(id)`: a mention, shown with the
/// target's live title).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DocLink {
    pub pending_title: Option<String>,
    pub target: String,
    pub block: usize,
    pub line: Vec<(String, Option<String>)>,
}

fn link_line(t: &RichText) -> Vec<(String, Option<String>)> {
    let mut out: Vec<(String, Option<String>)> = vec![];
    for (r, m) in t.spans() {
        let seg = &t.text[r];
        match m.inline.as_ref().and_then(Inline::doc_link) {
            Some(l) => out.push((l.title.clone(), Some(l.id.clone()))),
            None => {
                let s = match m.inline.as_ref().and_then(Inline::equation) {
                    Some(src) => src.to_string(),
                    None => seg.replace(['\u{fffc}', '\n'], " "),
                };
                match out.last_mut() {
                    Some((p, None)) => p.push_str(&s),
                    _ => out.push((s, None)),
                }
            }
        }
    }
    out
}

impl Document {
    /// A fresh, untouched document: no title and a single empty paragraph.
    pub fn is_blank(&self) -> bool {
        self.title.is_empty()
            && self.icon.is_none()
            && self.cover.is_none()
            && matches!(self.blocks.as_slice(), [b] if matches!(b.kind, BlockKind::Text) && b.text.is_empty() && b.children.is_empty())
    }

    /// Plain text of the whole document (for search and snippets).
    pub fn plain_text(&self) -> String {
        let mut out = String::new();
        fn walk(blocks: &[Block], out: &mut String) {
            for b in blocks {
                if !b.text.is_empty() {
                    if !out.is_empty() {
                        out.push('\n');
                    }
                    out.push_str(&b.text.plain_text());
                }
                walk(&b.children, out);
                if let BlockKind::Table(t) = &b.kind {
                    for row in &t.rows {
                        for cell in row {
                            walk(&cell.blocks, out);
                        }
                    }
                }
            }
        }
        walk(&self.blocks, &mut out);
        out
    }

    /// Visit every block in document order, including table cells.
    pub fn for_each_block(&self, mut f: impl FnMut(&Block)) {
        fn walk(blocks: &[Block], f: &mut dyn FnMut(&Block)) {
            for b in blocks {
                f(b);
                walk(&b.children, f);
                if let BlockKind::Table(t) = &b.kind {
                    for row in &t.rows {
                        for cell in row {
                            walk(&cell.blocks, f);
                        }
                    }
                }
            }
        }
        walk(&self.blocks, &mut f);
    }

    /// The 日期提醒 in the document.
    pub fn reminders(&self) -> Vec<InlineReminder> {
        let mut out = vec![];
        self.for_each_block(|b| out.extend(b.text.runs.iter().filter_map(|r| r.marks.inline.as_ref().and_then(Inline::reminder)).copied()));
        out
    }

    /// Its @文档 mentions, one per (target, block): the block's position in
    /// `for_each_block` order and its text, for 反向引用.
    pub fn doc_links(&self) -> Vec<DocLink> {
        let mut out: Vec<DocLink> = vec![];
        let mut ord = 0;
        self.for_each_block(|b| {
            let here = ord;
            ord += 1;
            if !b.text.text.contains('\u{fffc}') {
                return;
            }
            let mut line: Option<Vec<(String, Option<String>)>> = None;
            for (_, m) in b.text.spans() {
                let Some(l) = m.inline.as_ref().and_then(Inline::doc_link) else { continue };
                if out.iter().any(|d| d.block == here && d.target == l.id) {
                    continue;
                }
                let line = line.get_or_insert_with(|| link_line(&b.text)).clone();
                out.push(DocLink { pending_title: l.pending.then(|| l.title.clone()), target: l.id.clone(), block: here, line });
            }
        });
        out
    }

    /// Sources of the images and attachments (blocks and inline files).
    pub fn asset_sources(&self) -> Vec<String> {
        let mut out = vec![];
        self.for_each_block(|b| {
            match &b.kind {
                BlockKind::Image { src, .. } => out.push(src.clone()),
                BlockKind::File(f) if f.src_kind == "path" => out.push(f.src.clone()),
                _ => {}
            }
            for r in &b.text.runs {
                if let Some(f) = r.marks.inline.as_ref().and_then(Inline::file)
                    && f.src_kind == "path"
                {
                    out.push(f.src.clone());
                }
            }
        });
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rich_text_replace_and_marks() {
        let mut t = RichText::plain("hello world");
        t.update_marks(0..5, |m| m.bold = true);
        assert_eq!(t.runs.len(), 2);
        assert!(t.all_marks(0..5, |m| m.bold));
        t.replace(5..6, &RichText::plain(", "));
        assert_eq!(t.text, "hello, world");
        assert_eq!(t.runs.len(), 2);
        let tail = t.split_off(7);
        assert_eq!(tail.text, "world");
        assert_eq!(t.text, "hello, ");
        assert!(t.marks_at(3).bold);
        assert!(!t.marks_at(7).bold);
        assert!(t.marks_at(5).bold);
    }

    fn grid(nr: usize, nc: usize) -> Table {
        let rows = (0..nr)
            .map(|r| (0..nc).map(|c| TableCell { blocks: vec![Block::text(&format!("{r}{c}"))], ..TableCell::empty() }).collect())
            .collect();
        Table { col_widths: vec![100.; nc], header_rows: 0, header_col: false, rows }
    }

    fn text(t: &Table, r: usize, c: usize) -> String {
        t.rows[r][c].blocks.iter().map(|b| b.text.text.clone()).collect::<Vec<_>>().join("+")
    }

    #[test]
    fn table_merge_insert_delete() {
        let mut t = grid(3, 3);
        t.merge(CellRange::span((0, 0), (1, 1)));
        assert_eq!((t.rows[0][0].rowspan, t.rows[0][0].colspan), (2, 2));
        assert_eq!(text(&t, 0, 0), "00+01+10+11");
        assert!(t.rows[1][1].is_covered());
        assert_eq!(t.origin(1, 1), (0, 0));
        // A range touching a merged cell grows to hold all of it.
        assert_eq!(t.expand(CellRange::span((1, 1), (1, 2))), CellRange::span((0, 0), (1, 2)));
        // Inserting inside the merge stretches it; at its edge it doesn't.
        t.insert_row(1);
        assert_eq!(t.rows.len(), 4);
        assert_eq!(t.rows[0][0].rowspan, 3);
        assert!(!t.rows[1][2].is_covered());
        t.insert_col(0);
        assert_eq!(t.ncols(), 4);
        assert_eq!(t.rows[0][1].colspan, 2);
        // Deleting the merge's first row moves it down a row.
        t.delete_rows(0..1);
        assert_eq!(t.rows[0][1].rowspan, 2);
        assert_eq!(text(&t, 0, 1), "00+01+10+11");
        t.delete_cols(1..2);
        assert_eq!(t.rows[0][1].colspan, 1);
        t.unmerge(0, 1);
        assert_eq!(t.cells().count(), 9);
    }
}
