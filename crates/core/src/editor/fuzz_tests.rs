//! Random-edit fuzzer: drives [`Core`] the way the view does (a selection,
//! then one editing command) from a rich seed document and checks the model
//! invariants after every step, plus a save round trip and full undo/redo.
//! Deterministic: a failure prints its seed and step for replay.

use super::core::*;
use super::table_ops::{CellSel, CellSelKind};
use crate::docxml::{parse, serialize};
use crate::model::*;
use std::collections::HashSet;

const SEED_DOC: &str = r#"<title>随机 <b>编辑</b></title>
<h1>标题一</h1>
<p>普通 <b>加粗</b> <em>斜体</em> <code>code</code> 😀👍🏽 中文<br/>第二行</p>
<h2 folded="true">可折叠</h2>
<p>折叠内容</p>
<ul><li>一<ul><li>一.一</li><li>一.二</li></ul></li><li>二</li></ul>
<ol><li seq="3">三</li><li>四</li></ol>
<checkbox done="true">完成</checkbox>
<blockquote><p>引用一</p><p>引用二</p></blockquote>
<pre lang="rust"><code>fn main() {
    let s = "é";
}</code></pre>
<hr/>
<callout emoji="💡"><p>提示</p><ul><li>提示列表</li></ul></callout>
<grid><column width-ratio="0.5"><p>左</p></column><column width-ratio="0.5"><p>右</p><p>右二</p></column></grid>
<table><colgroup><col width="120"/><col width="80"/><col width="80"/></colgroup><thead><tr><th><p>A</p></th><th><p>B</p></th><th><p>C</p></th></tr></thead><tbody><tr><td colspan="2" rowspan="2"><p>合并</p><ul><li>格内列表</li></ul></td><td><p>3</p></td></tr><tr><td><p>6</p></td></tr><tr><td><p>7</p></td><td><p>8</p></td><td><pre lang="text"><code>a
b</code></pre></td></tr></tbody></table>
<p>公式 <latex>E=mc^2</latex> 结束</p>
<p></p>
"#;

const TEXTS: &[&str] = &["a", "中文", "😀", " ", "ab cd", "é", "👍🏽", "x"];
const PASTES: &[&str] = &["一\n二", "a\n\nb\nc", "\n", "😀\n中", "line"];

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        // xorshift64*
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n.max(1) as u64) as usize
    }

    fn pick<'a, T>(&mut self, xs: &'a [T]) -> &'a T {
        &xs[self.below(xs.len())]
    }

    fn chance(&mut self, percent: u64) -> bool {
        self.next() % 100 < percent
    }
}

/// A char-boundary offset in the unit's text (0 for atomic units).
fn offset_in(c: &Core, id: BlockId, rng: &mut Rng) -> usize {
    let Some(t) = c.text_of(id) else { return 0 };
    let mut o = rng.below(t.len() + 1);
    while !t.text.is_char_boundary(o) {
        o -= 1;
    }
    o
}

/// A caret or a range inside one scope, as mouse and keyboard produce.
fn random_selection(c: &mut Core, rng: &mut Rng) {
    let us = c.units();
    let a = *rng.pick(&us);
    let same: Vec<Unit> = us.iter().copied().filter(|u| u.scope == a.scope).collect();
    let b = if rng.chance(60) { a } else { *rng.pick(&same) };
    c.block_sel = None;
    c.cell_sel = None;
    let anchor = Pos::new(a.id, offset_in(c, a.id, rng));
    let head = Pos::new(b.id, offset_in(c, b.id, rng));
    c.sel = Selection { anchor, head };
    c.fix_selection();
}

fn random_kind(rng: &mut Rng) -> BlockKind {
    match rng.below(7) {
        0 => BlockKind::Text,
        1 => BlockKind::Heading { level: 1 + rng.below(9) as u8, seq_auto: false },
        2 => BlockKind::Bullet,
        3 => BlockKind::Ordered { seq: None },
        4 => BlockKind::Todo { done: false },
        5 => BlockKind::Quote,
        _ => BlockKind::Code { lang: "plaintext".into(), caption: None, wrap: false },
    }
}

fn tables(doc: &Document) -> Vec<BlockId> {
    let mut out = vec![];
    doc.for_each_block(|b| {
        if matches!(b.kind, BlockKind::Table(_)) {
            out.push(b.id)
        }
    });
    out
}

/// One editing command; returns its name for the failure report.
fn step(c: &mut Core, rng: &mut Rng) -> String {
    if rng.chance(70) {
        random_selection(c, rng);
    }
    if std::env::var("FDL_FUZZ_TRACE").is_ok() {
        eprintln!("  before: sel {:?} block_sel {:?}", c.sel, c.block_sel);
    }
    if c.block_sel.is_some() {
        return block_mode_step(c, rng);
    }
    let us = c.units();
    let any = |rng: &mut Rng| rng.pick(&us).id;
    match rng.below(23) {
        0..=2 => {
            let s = *rng.pick(TEXTS);
            c.insert_text(s);
            format!("insert_text({s:?})")
        }
        3 => {
            c.enter();
            "enter".into()
        }
        4 | 5 => {
            c.backspace();
            "backspace".into()
        }
        6 => {
            c.delete_forward();
            "delete_forward".into()
        }
        7 => {
            c.delete_word_back();
            "delete_word_back".into()
        }
        8 => {
            c.indent();
            "indent".into()
        }
        9 => {
            c.outdent();
            "outdent".into()
        }
        10 => {
            let ids = c.selected_blocks();
            let kind = random_kind(rng);
            let name = format!("turn_into({ids:?}, {kind:?})");
            c.turn_into(&ids, kind);
            name
        }
        11 => {
            match rng.below(3) {
                0 => c.toggle_mark(|m| m.bold, |m, v| m.bold = v),
                1 => c.toggle_mark(|m| m.italic, |m, v| m.italic = v),
                _ => c.toggle_mark(|m| m.code, |m, v| m.code = v),
            }
            "toggle_mark".into()
        }
        12 => {
            c.select_all();
            if rng.chance(50) {
                c.select_all();
            }
            if rng.chance(50) {
                c.delete_selected_blocks();
                return "select_all+delete_selected_blocks".into();
            }
            "select_all".into()
        }
        13 => {
            let id = any(rng);
            c.select_block(id);
            c.delete_selected_blocks();
            format!("delete_selected_blocks({id:?})")
        }
        14 => {
            let fwd = rng.chance(50);
            c.cell_step(fwd);
            format!("cell_step({fwd})")
        }
        15 => {
            let s = *rng.pick(PASTES);
            c.paste_text(s);
            format!("paste_text({s:?})")
        }
        16 => {
            c.undo();
            "undo".into()
        }
        17 => {
            c.redo();
            "redo".into()
        }
        18 => {
            let id = any(rng);
            if c.can_fold(id) {
                c.toggle_fold(id);
            }
            format!("toggle_fold({id:?})")
        }
        19 => {
            let (a, b) = (any(rng), any(rng));
            let place = *rng.pick(&[DropPlace::Before, DropPlace::After, DropPlace::FirstChild]);
            c.move_blocks(&[a], b, place);
            format!("move_blocks({a:?}, {b:?}, {place:?})")
        }
        20 => table_step(c, rng),
        21 => object_step(c, rng),
        _ => {
            c.trailing_line();
            "trailing_line".into()
        }
    }
}

/// Block mode takes only block commands (the view ignores text ones).
fn block_mode_step(c: &mut Core, rng: &mut Rng) -> String {
    let ids = c.block_sel_ids();
    match rng.below(8) {
        0 | 1 => {
            c.delete_selected_blocks();
            format!("block mode: delete {ids:?}")
        }
        2 => {
            let down = rng.chance(50);
            c.block_sel_move(down, rng.chance(50));
            format!("block mode: move selection (down {down})")
        }
        3 => {
            let kind = random_kind(rng);
            let name = format!("block mode: turn_into({ids:?}, {kind:?})");
            c.turn_into(&ids, kind);
            name
        }
        4 => {
            c.select_all();
            "block mode: select_all".into()
        }
        5 => {
            c.undo();
            "block mode: undo".into()
        }
        _ => {
            // Paste and Enter leave block mode at the end of the selection.
            if let Some(&last) = ids.last() {
                c.clear_block_sel();
                c.caret_to_end_of(last);
            }
            let s = *rng.pick(PASTES);
            c.paste_text(s);
            format!("block mode: paste_text({s:?}) after {ids:?}")
        }
    }
}

fn table_step(c: &mut Core, rng: &mut Rng) -> String {
    // Only tables on screen (not hidden under a folded heading).
    let vis: HashSet<BlockId> = c.block_order(true).into_iter().collect();
    let ts: Vec<BlockId> = tables(&c.doc).into_iter().filter(|t| vis.contains(t)).collect();
    if ts.is_empty() {
        return "table: none".into();
    }
    let table = *rng.pick(&ts);
    let t = c.table(table).unwrap();
    let (nr, nc) = (t.rows.len(), t.ncols());
    let slot = |rng: &mut Rng| (rng.below(nr), rng.below(nc));
    let kind = *rng.pick(&[CellSelKind::Range, CellSelKind::Rows, CellSelKind::Cols]);
    let s = CellSel { table, anchor: slot(rng), head: slot(rng), kind };
    c.select_cells(s);
    let what = match rng.below(6) {
        0 => {
            c.table_merge();
            "merge"
        }
        1 => {
            c.table_unmerge();
            "unmerge"
        }
        2 => {
            c.table_delete_selected();
            "delete_selected"
        }
        3 => {
            let cols = rng.chance(50);
            c.table_insert(table, cols, rng.below(if cols { nc } else { nr } + 1));
            "insert"
        }
        4 => {
            c.table_move_selected(rng.below(nr.max(nc) + 1));
            "move_selected"
        }
        _ => {
            c.table_clear_cells();
            "clear_cells"
        }
    };
    format!("table_{what}({s:?})")
}

/// Inline objects: insert one at the caret, or edit the one under it.
fn object_step(c: &mut Core, rng: &mut Rng) -> String {
    let p = c.sel.head;
    match rng.below(5) {
        0 => {
            c.insert_equation(rng.pick(&["x^2", "a<b", ""]));
            "insert_equation".into()
        }
        1 => {
            c.insert_button(&InlineButton::new(*rng.pick(&[ButtonAction::DuplicatePage, ButtonAction::FollowPage])));
            "insert_button".into()
        }
        2 => {
            c.insert_reminder(&InlineReminder { expire: 1_700_000_000_000, notify: 1_700_000_000_000, should_notify: rng.chance(50), whole_day: rng.chance(50) });
            "insert_reminder".into()
        }
        3 => {
            c.insert_doc_link(&InlineDocLink { id: "doc1".into(), title: "文档 & <一>".into(), alias: Some("别名".into()), pending: false });
            "insert_doc_link".into()
        }
        _ => {
            c.set_equation_source(p, rng.pick(&["y", "\\frac{1}{2}", "x^2"]));
            format!("set_equation_source({p:?})")
        }
    }
}

// ------------------------------------------------------------------ invariants

fn check_text(t: &RichText, at: &str) -> Result<(), String> {
    let mut o = 0;
    for (i, r) in t.runs.iter().enumerate() {
        if r.len == 0 {
            return Err(format!("{at}: zero-length run {i} in {t:?}"));
        }
        o += r.len;
        if o > t.text.len() || !t.text.is_char_boundary(o) {
            return Err(format!("{at}: run {i} ends off a char boundary in {t:?}"));
        }
        if r.marks.inline.is_some() && t.text[o - r.len..o].chars().any(|ch| ch != '\u{FFFC}') {
            return Err(format!("{at}: object run {i} holds text in {t:?}"));
        }
        if i > 0 && t.runs[i - 1].marks == r.marks {
            return Err(format!("{at}: runs {} and {i} have the same marks in {t:?}", i - 1));
        }
    }
    if o != t.text.len() {
        return Err(format!("{at}: runs cover {o} of {} bytes in {t:?}", t.text.len()));
    }
    Ok(())
}

fn check_table(t: &Table, at: &str) -> Result<(), String> {
    let (nr, nc) = (t.rows.len(), t.ncols());
    if nr == 0 || nc == 0 {
        return Err(format!("{at}: empty table {nr}×{nc}"));
    }
    let mut count = vec![vec![0; nc]; nr];
    for (r, row) in t.rows.iter().enumerate() {
        if row.len() != nc {
            return Err(format!("{at}: row {r} has {} of {nc} cells", row.len()));
        }
        for (c, cell) in row.iter().enumerate() {
            if cell.is_covered() {
                if cell.rowspan != 0 || !cell.blocks.is_empty() {
                    return Err(format!("{at}: covered slot ({r},{c}) has content or a span"));
                }
                continue;
            }
            let (rs, cs) = (cell.rowspan as usize, cell.colspan as usize);
            if rs == 0 || r + rs > nr || c + cs > nc {
                return Err(format!("{at}: cell ({r},{c}) spans {rs}×{cs} outside {nr}×{nc}"));
            }
            if cell.blocks.is_empty() {
                return Err(format!("{at}: cell ({r},{c}) has no blocks"));
            }
            for row in count.iter_mut().skip(r).take(rs) {
                for n in row.iter_mut().skip(c).take(cs) {
                    *n += 1;
                }
            }
        }
    }
    for (r, row) in count.iter().enumerate() {
        for (c, n) in row.iter().enumerate() {
            if *n != 1 {
                return Err(format!("{at}: slot ({r},{c}) covered by {n} cells"));
            }
        }
    }
    Ok(())
}

fn check_blocks(blocks: &[Block], parent: Option<&BlockKind>, ids: &mut HashSet<BlockId>) -> Result<(), String> {
    for b in blocks {
        let at = format!("block {:?} ({:?})", b.id, std::mem::discriminant(&b.kind));
        if !ids.insert(b.id) {
            return Err(format!("{at}: duplicate id"));
        }
        check_text(&b.text, &at)?;
        let in_grid = matches!(parent, Some(BlockKind::Grid));
        if matches!(b.kind, BlockKind::Column { .. }) != in_grid {
            return Err(format!("{at}: column outside a grid, or a grid holding a non-column"));
        }
        if matches!(b.kind, BlockKind::Grid | BlockKind::Column { .. }) && b.children.is_empty() {
            return Err(format!("{at}: empty grid or column"));
        }
        if matches!(b.kind, BlockKind::Code { .. }) && b.text.runs.iter().any(|r| r.marks != Marks::default()) {
            return Err(format!("{at}: marks or inline objects in code"));
        }
        if !b.children.is_empty() && !b.kind.nests() && !b.kind.is_container() {
            return Err(format!("{at}: children under a block that can't hold (or save) them"));
        }
        if let BlockKind::Table(t) = &b.kind {
            check_table(t, &at)?;
            for (_, _, cell) in t.cells() {
                check_blocks(&cell.blocks, None, ids)?;
            }
        }
        check_blocks(&b.children, Some(&b.kind), ids)?;
    }
    Ok(())
}

fn check(c: &Core) -> Result<(), String> {
    if c.doc.blocks.is_empty() {
        return Err("document has no blocks".into());
    }
    check_text(&c.doc.title, "title")?;
    check_blocks(&c.doc.blocks, None, &mut HashSet::new())?;
    let us = c.units();
    if let Some(bs) = c.block_sel {
        // Block mode: the text selection is not shown; its ends are.
        for id in [bs.anchor, bs.head] {
            if c.block(id).is_none() || c.is_hidden(id) {
                return Err(format!("block selection end {id:?} is gone or hidden"));
            }
        }
        return Ok(());
    }
    let (a, h) = (c.sel.anchor, c.sel.head);
    for p in [a, h] {
        let Some(u) = us.iter().find(|u| u.id == p.block) else {
            return Err(format!("selection end {p:?} is not a unit (selection {:?})", c.sel));
        };
        match c.text_of(p.block) {
            Some(t) if p.offset > t.len() || !t.text.is_char_boundary(p.offset) => {
                return Err(format!("selection end {p:?} is off the text {:?}", t.text));
            }
            None if u.text => return Err(format!("text unit {p:?} has no text")),
            _ => {}
        }
    }
    let vis: HashSet<BlockId> = c.block_order(true).into_iter().collect();
    let mut hidden_mismatch = None;
    c.doc.for_each_block(|b| {
        if !matches!(b.kind, BlockKind::Column { .. }) && c.is_hidden(b.id) == vis.contains(&b.id) {
            hidden_mismatch.get_or_insert(b.id);
        }
    });
    if let Some(id) = hidden_mismatch {
        return Err(format!("is_hidden({id:?}) disagrees with block_order"));
    }
    let scope = |id| us.iter().find(|u| u.id == id).map(|u| u.scope);
    if c.cell_sel.is_none() && scope(a.block) != scope(h.block) {
        return Err(format!("selection {a:?}..{h:?} crosses a table boundary"));
    }
    Ok(())
}

fn round_trip(c: &Core) -> Result<(), String> {
    // Known loss: in a table cell, spaces between a block's text and its nested blocks read back as the
    // indentation older versions wrote there (docs/FORMAT.md).
    fn trim_before_nested(blocks: &mut [Block], in_cell: bool) {
        for b in blocks {
            let keep = b.text.text.trim_end_matches([' ', '\t']).len();
            if in_cell && !b.children.is_empty() && keep < b.text.len() {
                b.text.replace(keep..b.text.len(), &RichText::default());
            }
            trim_before_nested(&mut b.children, in_cell);
            if let BlockKind::Table(t) = &mut b.kind {
                t.rows.iter_mut().flatten().for_each(|cell| trim_before_nested(&mut cell.blocks, true));
            }
        }
    }
    let mut doc = c.doc.clone();
    trim_before_nested(&mut doc.blocks, false);
    let x = serialize(&doc);
    let back = parse(&x).map_err(|e| format!("saved document no longer parses: {e}\n{x}"))?;
    let y = serialize(&back);
    if x != y {
        return Err(format!("save → load → save differs:\n--- first\n{x}\n--- second\n{y}"));
    }
    Ok(())
}

/// The saved form with folds cleared: folding is outside the edit history.
fn unfolded(doc: &Document) -> String {
    fn walk(blocks: &mut [Block]) {
        for b in blocks {
            b.folded = false;
            walk(&mut b.children);
        }
    }
    let mut doc = doc.clone();
    walk(&mut doc.blocks);
    serialize(&doc)
}

/// The first differing line of two documents, with a little context.
fn diff(a: &str, b: &str) -> String {
    let (a, b): (Vec<&str>, Vec<&str>) = (a.lines().collect(), b.lines().collect());
    let i = a.iter().zip(&b).position(|(x, y)| x != y).unwrap_or(a.len().min(b.len()));
    let near = |v: &[&str]| v[i.saturating_sub(2)..(i + 3).min(v.len())].join("\n");
    format!("from line {}:\n--- expected\n{}\n--- actual\n{}", i + 1, near(&a), near(&b))
}

fn run(seed: u64, steps: usize) {
    let mut rng = Rng(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1);
    let mut c = Core::new(parse(SEED_DOC).unwrap());
    let start = unfolded(&c.doc);
    let mut log: Vec<String> = vec![];
    let fail = |log: &[String], c: &Core, e: String| -> ! {
        let tail = log.iter().rev().take(12).rev().cloned().collect::<Vec<_>>().join("\n  ");
        panic!("seed {seed}, step {}: {e}\nlast steps:\n  {tail}\ndocument:\n{}", log.len(), serialize(&c.doc));
    };
    for i in 0..steps {
        let (before, generation) = (c.doc.clone(), c.generation);
        log.push(step(&mut c, &mut rng));
        // The editor caches per-frame work against `generation`.
        if c.generation == generation && c.doc != before {
            fail(&log, &c, "the document changed without bumping `generation`".into());
        }
        // Editor::changed, after every command.
        c.reveal_caret();
        if std::env::var("FDL_FUZZ_TRACE").is_ok() {
            eprintln!("step {i}: {}\n  after: sel {:?}\n{}", log[i], c.sel, serialize(&c.doc));
        }
        if let Err(e) = check(&c) {
            fail(&log, &c, e);
        }
        if i % 25 == 0
            && let Err(e) = round_trip(&c)
        {
            fail(&log, &c, e);
        }
    }
    if let Err(e) = round_trip(&c) {
        fail(&log, &c, e);
    }
    // Every change is undoable (steps < UNDO_STEPS), and redo replays it.
    while c.redo() {}
    let end = unfolded(&c.doc);
    while c.undo() {}
    if unfolded(&c.doc) != start {
        fail(&log, &c, format!("undoing everything does not restore the seed document, {}", diff(&start, &unfolded(&c.doc))));
    }
    while c.redo() {}
    if unfolded(&c.doc) != end {
        fail(&log, &c, format!("redoing everything does not restore the final document, {}", diff(&end, &unfolded(&c.doc))));
    }
}

#[test]
fn random_edits_keep_the_model_valid() {
    // FDL_FUZZ_SEEDS=n runs seeds 0..n, FDL_FUZZ_STEPS=m steps each (under UNDO_STEPS for the undo check);
    // FDL_FUZZ_SEED=k replays one seed (with FDL_FUZZ_TRACE set, step by step).
    let env = |k: &str| std::env::var(k).ok().and_then(|s| s.parse::<u64>().ok());
    let seeds = match env("FDL_FUZZ_SEED") { Some(k) => k..k + 1, None => 0..env("FDL_FUZZ_SEEDS").unwrap_or(40) };
    let steps = env("FDL_FUZZ_STEPS").unwrap_or(300) as usize;
    for seed in seeds {
        run(seed, steps);
    }
}
