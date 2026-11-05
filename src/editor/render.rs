//! Rendering of the document page: title, meta row and every block kind
//! (design-spec §2–§3). Floating panels are in `overlays.rs`.

use super::core::{self, Pos, TITLE, Unit};
use super::text::{LayoutCache, RichTextElement, TextSpec};
use super::view::{BoundsMap, Editor, Menu};
use crate::model::*;
use crate::editor_theme::{self as theme, Theme, craft};
use crate::editor_ui::{TipExt, icon};
use gpui::{prelude::FluentBuilder, *};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use rustc_hash::FxHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// Per-frame render context shared by all blocks.
pub(super) struct R<'a> {
    pub ed: &'a Editor,
    pub units: Rc<Vec<Unit>>,
    /// Equation numbers before each block that has equations.
    pub eqnos: Rc<HashMap<BlockId, u32>>,
    pub focused: bool,
    pub selected: Rc<HashSet<BlockId>>,
    pub targets: RefCell<Vec<BlockId>>,
    /// A block whose children are list rows of their own (not drawn inside it).
    pub flat: Option<BlockId>,
    pub cache: LayoutCache,
    pub bounds: BoundsMap,
    /// Drawing the drag ghost: a static copy (no handle, selection or caret)
    /// laid out apart from the page.
    pub ghost: bool,
}

/// State computed once per frame and shared by every list item.
#[derive(Default)]
pub(super) struct Frame {
    /// `core.generation` that `units` / `eqnos` were computed at.
    generation: Option<u64>,
    units: Rc<Vec<Unit>>,
    pub eqnos: Rc<HashMap<BlockId, u32>>,
    selected: Rc<HashSet<BlockId>>,
    focused: bool,
}

/// One row of the virtualized page.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum Item {
    /// Title and meta row.
    Header,
    /// A block (child indices from the top level) and the collapsed margin
    /// above it. The children of text, heading and list blocks get rows of
    /// their own, so a long nested list doesn't make one huge row.
    Block { path: Vec<usize>, gap: f32 },
    /// The click area below the last block.
    Below,
}

/// What to bring into view once the frame is laid out.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Reveal {
    /// The caret (or the block-selection head), kept 40px from the edges.
    Caret,
    /// A find hit, centred when it isn't comfortably visible.
    Find(Pos),
    /// A block, put 24px below the top.
    Block(BlockId),
}

/// `box-shadow: 0 0 0 1px` around a radius-8 bordered box. (A GPUI shadow
/// fills its whole rect, which shows through a box without a fill.)
fn outer_ring(color: Hsla) -> Div {
    div().absolute().top(px(-2.)).left(px(-2.)).right(px(-2.)).bottom(px(-2.)).rounded(px(9.)).border_1().border_color(color)
}

impl Editor {
    /// Fill and radius of selected blocks: `.block.hovered` while the handle
    /// menu is open, else `.selected-mask`.
    pub(super) fn block_tint(&self) -> (Hsla, f32) {
        if matches!(self.menu, Some(Menu::Block { .. } | Menu::Add { .. })) { (Theme::block_hover(), 8.) } else { (Theme::block_selected(), 6.) }
    }
}

/// Text, heading and list blocks whose children are drawn as separate rows.
pub(super) fn flattens(b: &Block) -> bool {
    !b.children.is_empty()
        && !b.folded
        && b.kind.nests()
}

/// Changes whenever a block's subtree may lay out differently (its own row
/// only when its children are rows of their own).
///
/// Text is keyed by length only: the list re-measures every visible row each
/// frame anyway, so a same-length edit (⌘B on a whole paragraph) can at worst
/// leave an off-screen row's cached height stale until it scrolls into view.
/// Hashing the text itself costs ~3ms per keystroke on a 1M-character page
/// (73k blocks), measured with harness/perf/b1m.txt.
fn hash_block(b: &Block, orders: &rustc_hash::FxHashMap<BlockId, String>, h: &mut FxHasher) {
    b.id.hash(h);
    std::mem::discriminant(&b.kind).hash(h);
    b.text.text.len().hash(h);
    b.text.runs.len().hash(h);
    b.folded.hash(h);
    match &b.kind {
        BlockKind::Heading { level, .. } => (level, orders.get(&b.id)).hash(h),
        BlockKind::Code { wrap, .. } => wrap.hash(h),
        BlockKind::Image { src, width, height, caption, crop, .. } => {
            (src, width.map(f32::to_bits), height.map(f32::to_bits), caption.as_ref().map(String::len), crop.map(|c| c.map(f32::to_bits))).hash(h)
        }
        BlockKind::Column { ratio } => ratio.to_bits().hash(h),
        BlockKind::File(f) => (&f.src, &f.name, f.view).hash(h),
        BlockKind::Table(t) => {
            t.col_widths.iter().for_each(|w| w.to_bits().hash(h));
            (t.header_rows, t.header_col).hash(h);
            for row in &t.rows {
                for cell in row {
                    (cell.colspan, cell.rowspan, &cell.bg, &cell.valign).hash(h);
                    cell.blocks.iter().for_each(|b| hash_block(b, orders, h));
                }
            }
        }
        _ => {}
    }
    b.children.len().hash(h);
    if !flattens(b) {
        b.children.iter().for_each(|c| hash_block(c, orders, h));
    }
}

/// Inherited per-subtree settings.
#[derive(Clone, Copy)]
pub(super) struct Env {
    /// List nesting depth (marker style cycles with it).
    pub depth: usize,
    /// Inside a table cell (14/22 text, no handles).
    pub cell: bool,
    /// Text color override from a quote or callout.
    pub color: Option<Hsla>,
    /// Inside a callout: its children show no handle of their own; hovering
    /// them targets the callout.
    pub in_callout: bool,
    /// A block of the page itself (not nested in another block).
    pub root: bool,
    /// Inside a grid column (no bottom margins).
    pub column: bool,
    /// Inside a quote (marks dimmed, see `TextSpec::dim_marks`).
    pub quote: bool,
    /// Width of the text column this subtree lays out in (0 in table cells).
    pub width: f32,
}

const TEXT_PLACEHOLDER: &str = "输入“/”快速插入内容";

pub(super) fn margins(b: &Block, env: Env) -> (f32, f32) {
    if env.cell {
        return (0., 0.);
    }
    let (mt, mb) = match &b.kind {
        BlockKind::Heading { level, .. } => {
            let m = theme::heading_metrics(*level);
            (m.mt, m.mb)
        }
        // Decorated blocks and code keep a little more air than text.
        BlockKind::Code { .. } | BlockKind::Callout { .. } => (16., 16.),
        // `.table`: 12 above and below.
        BlockKind::Table(_) => if crate::settings::feishu() { (29.5, 22.) } else { (16., 16.) },
        // `.paragraph` padding: one block every 35pt.
        _ => (theme::block_gap(), theme::block_gap()),
    };
    // Grid columns: no bottom margins inside a column.
    (mt, if env.column { 0. } else { mb })
}

/// Indices of the blocks shown in `list` (a folded heading hides what follows
/// it up to the next heading of the same or a higher level), each with the
/// collapsed margin above it.
pub(super) fn visible_with_gaps(list: &[Block], env: Env) -> Vec<(usize, f32)> {
    let mut out = vec![];
    let mut prev_mb: Option<f32> = None;
    let mut hide_below: Option<u8> = None;
    for (i, b) in list.iter().enumerate() {
        if let Some(level) = hide_below {
            match b.kind {
                BlockKind::Heading { level: l, .. } if l <= level => hide_below = None,
                _ => continue,
            }
        }
        if let BlockKind::Heading { level, .. } = b.kind
            && b.folded
        {
            hide_below = Some(level);
        }
        let (mt, mb) = margins(b, env);
        out.push((i, prev_mb.map(|p| p.max(mt)).unwrap_or(0.)));
        prev_mb = Some(mb);
    }
    out
}

pub(super) fn record_bounds(id: BlockId, map: BoundsMap) -> impl IntoElement {
    canvas(
        move |b, _, _| {
            map.borrow_mut().insert(id, b);
        },
        |_, _, _, _| {},
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full()
}

/// The document-wide `katexEqnNo` counter at each block with equations.
pub(crate) fn equation_numbers(doc: &crate::model::Document) -> HashMap<BlockId, u32> {
    let mut out = HashMap::new();
    let mut n = 0;
    doc.for_each_block(|b| {
        if !b.text.text.contains('\u{fffc}') {
            return;
        }
        let mut here = 0;
        for (r, m) in b.text.spans() {
            if let Some(src) = m.inline.as_ref().and_then(crate::model::Inline::equation) {
                here += crate::editor::math::numbers_in(src) * (r.len() / 3) as u32;
            }
        }
        if here > 0 || n > 0 {
            out.insert(b.id, n);
        }
        n += here;
    });
    out
}

impl R<'_> {
    fn text_el(&self, id: BlockId, text: &RichText, mut spec: TextSpec, align: Align, placeholder: Option<SharedString>) -> RichTextElement {
        let core = &self.ed.core;
        let bm = core.in_block_mode() || self.ghost;
        let sel = core.sel;
        spec.link_underline = !bm && self.focused && sel.head.block == id;
        let selection = if bm || sel.is_collapsed() { None } else { core.range_in(id, &self.units) };
        let caret_here = !bm && core.cell_sel.is_none() && sel.head.block == id && sel.is_collapsed();
        RichTextElement {
            id,
            text: text.clone(),
            spec,
            align,
            placeholder,
            placeholder_color: None,
            placeholder_dx: 0.,
            selection,
            caret: caret_here.then_some(sel.head.offset),
            marked: self.ed.marked.as_ref().filter(|(b, _)| *b == id && !self.ghost).map(|(_, r)| r.clone()),
            cache: self.cache.clone(),
            show_caret: caret_here && self.focused && self.ed.caret_visible && !self.ed.read_only,
            highlight: None,
            found: if self.ghost { vec![] } else { self.ed.find_hits(id) },
            eq_counter: self.eqnos.get(&id).copied().unwrap_or(0),
            button_edit: if self.ghost { None } else { self.ed.button_preview(id) },
        }
    }

    fn caret_in_empty(&self, b: &Block) -> bool {
        let s = self.ed.core.sel;
        self.focused && !self.ed.core.in_block_mode() && s.is_collapsed() && s.head.block == b.id && b.text.is_empty()
    }

    /// An empty body (a single empty paragraph) keeps its hint even while the caret is in the title.
    fn only_empty_block(&self, b: &Block) -> bool {
        matches!(self.ed.core.doc.blocks.as_slice(), [only] if only.id == b.id && b.text.is_empty() && b.children.is_empty())
    }

    fn spec_for(&self, b: &Block, env: Env) -> TextSpec {
        if crate::settings::feishu() { return self.spec_for_feishu(b, env); }

        let st = theme::style();
        let mut spec = TextSpec::body();
        if env.cell {
            spec.size = st.num("table.size");
            spec.line_height = st.num("table.line-height");
        }
        if let BlockKind::Heading { level, .. } = b.kind {
            let m = theme::heading_metrics(level);
            spec.size = m.size;
            spec.line_height = m.line_height;
            spec.weight = theme::heading_weight(level);
            if let Some(c) = theme::heading_color(level) { spec.color = c; }
            let f = theme::heading_font(level);
            if f != theme::body_font() { spec.font = Some(f); }
        }
        spec.dim_marks = env.quote;
        if env.quote {
            if let Some(c) = st.color("quote.color") { spec.color = c; }
            spec.italic = st.flag("quote.italic");
        }
        if let Some(c) = env.color {
            spec.color = c;
        }
        if let BlockKind::Todo { done: true } = b.kind {
            spec.color = Theme::todo_done();
            spec.strike_all = st.flag("done.strike");
        }
        spec
    }

    /// A column of sibling blocks with collapsed margins and heading folding.
    pub fn blocks(&self, list: &[Block], env: Env, cx: &mut Context<Editor>) -> Div {
        let mut col = div().flex().flex_col().w_full();
        for (i, gap) in visible_with_gaps(list, env) {
            col = col.child(div().mt(px(gap)).child(self.block(&list[i], list, i, Env { root: false, ..env }, cx)));
        }
        col
    }

    pub(super) fn block(&self, b: &Block, siblings: &[Block], idx: usize, env: Env, cx: &mut Context<Editor>) -> AnyElement {
        if crate::settings::feishu() { return self.block_feishu(b, siblings, idx, env, cx); }

        if !env.cell && !env.in_callout && !matches!(b.kind, BlockKind::Column { .. }) && !self.ghost {
            self.targets.borrow_mut().push(b.id);
        }
        let content: AnyElement = match &b.kind {
            BlockKind::Text | BlockKind::Heading { .. } => self.text_block(b, env, cx),
            BlockKind::Bullet | BlockKind::Ordered { .. } | BlockKind::Todo { .. } => self.list_item(b, siblings, idx, env, cx),
            BlockKind::Code { lang, wrap, .. } => self.code_block(b, lang, *wrap, cx),
            // The focus decoration: a 4pt bar at the column edge, the text
            // 12pt after it and unchanged.
            BlockKind::Quote => {
                let (indent, bar) = (theme::style().num("quote.indent"), theme::style().num("quote.bar"));
                div()
                    .relative()
                    .pl(px(indent))
                    .child(div().absolute().left_0().top_0().bottom_0().w(px(bar)).rounded(px((bar / 4.).min(1.))).bg(Theme::quote_bar()))
                    .child(self.blocks(&b.children, Env { quote: true, width: env.width - indent, ..env }, cx))
                    .into_any_element()
            }
            BlockKind::Callout { emoji, bg, border, text_color } => {
                let bgc = bg.as_ref().and_then(|c| theme::callout_bg(&c.0)).unwrap_or(theme::c(craft::O50));
                let bc = border.as_ref().and_then(|c| theme::callout_border(&c.0)).unwrap_or(theme::c(craft::O200));
                let tc = text_color.as_ref().and_then(|c| theme::text_color(&c.0)).or(env.color);
                let id = b.id;
                // `.docx-callout-block.hovered` (handle menu open): `--B200` ring;
                // `.selected`: `--text-link-hover` border. Both drop the fill
                // so the block tint shows.
                let menu_on = matches!(&self.ed.menu, Some(Menu::Block { block, .. }) | Some(Menu::Add { block, .. }) if *block == id);
                let selected = !menu_on && self.selected.contains(&id);
                // Block decoration: a tinted field, radius 3; the rim
                // is the chosen border colour, faint.
                let st = theme::style();
                div()
                    .relative()
                    .flex()
                    .px(px(st.num("callout.pad-x")))
                    .py(px(st.num("callout.pad-y")))
                    .rounded(px(st.num("callout.radius")))
                    .map(|d| match st.num("callout.border").round() as i32 { ..=0 => d, 1 => d.border_1(), _ => d.border_2() })
                    .border_color(if menu_on {
                        Theme::code_hover_ring()
                    } else if selected {
                        Theme::primary_500()
                    } else {
                        bc.opacity(0.45)
                    })
                    .when(!menu_on && !selected, |d| d.bg(bgc))
                    .when(menu_on, |d| {
                        d.child(outer_ring(Theme::code_hover_ring()))
                    })
                    .child(
                        div()
                            .id(("callout-emoji", id.0))
                            .flex_none()
                            .size(px(24.))
                            .mt(px((super::emoji_picker::callout_emoji_height(b.children.first().map(|c| &c.kind)) - 24.) / 2.))
                            .rounded(px(4.))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(20.))
                            .line_height(px(24.))
                            .cursor_pointer()
                            .hover(|s| s.bg(Theme::fill_hover()))
                            .tip_above(crate::settings::text("点击更换图标"), None)
                            .child(emoji.clone())
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |e, _, window, cx| {
                                    cx.stop_propagation();
                                    e.toggle_emoji_picker(super::emoji_picker::EmojiTarget::Callout(id), window, cx);
                                }),
                            ),
                    )
                    .child(div().flex_1().min_w_0().ml(px(10.)).child(self.blocks(&b.children, Env { color: tc, in_callout: true, width: env.width - 64., ..env }, cx)))
                    .into_any_element()
            }
            // The default separator ('light'): a dotted rule, 1.5pt dots
            // every 5pt at half the text colour.
            BlockKind::Divider => div()
                .h(px(20.))
                .w_full()
                .child(
                    canvas(
                        |_, _, _| {},
                        |b, _, window, _| {
                            let y = b.center().y;
                            let st = theme::style();
                            let w = st.num("hr.width");
                            match st.line("hr.style") {
                                crate::themes::LineStyle::Dotted => {
                                    let n = (f32::from(b.size.width) / 5.).floor() as usize;
                                    let left = b.left() + (b.size.width - px(5. * n as f32 - 3.5)) / 2.;
                                    for i in 0..n {
                                        let x = left + px(5. * i as f32);
                                        window.paint_quad(fill(Bounds::new(point(x, y - px(w / 2.)), size(px(w), px(w))), Theme::separator_dots()).corner_radii(px(w / 2.)));
                                    }
                                }
                                crate::themes::LineStyle::Solid => window.paint_quad(fill(Bounds::new(point(b.left(), y - px(w / 2.)), size(b.size.width, px(w))), Theme::separator_dots())),
                                crate::themes::LineStyle::Dashed => {
                                    let mut x = b.left();
                                    while x < b.right() {
                                        window.paint_quad(fill(Bounds::new(point(x, y - px(w / 2.)), size(px(6.).min(b.right() - x), px(w))), Theme::separator_dots()));
                                        x += px(10.);
                                    }
                                }
                            }
                        },
                    )
                    .size_full(),
                )
                .into_any_element(),
            BlockKind::Image { .. } => self.image(b, env, cx),
            BlockKind::SubPageList => self.sub_page_list(b, cx),
            BlockKind::Countdown(c) => self.countdown(b, c, cx),
            BlockKind::File(f) => self.file(b, f, env, cx),
            BlockKind::Table(t) if self.ghost => self.ghost_table(b, t, cx),
            BlockKind::Table(t) => self.table(b, t, cx),
            BlockKind::Grid => self.grid(b, env, cx),
            BlockKind::Column { .. } => self.blocks(&b.children, env, cx).into_any_element(),
            BlockKind::Raw { tag, .. } => div()
                .px(px(12.))
                .py(px(8.))
                .rounded(px(5.))
                .bg(Theme::bg_overlay())
                .text_size(px(13.))
                .line_height(px(20.))
                .text_color(Theme::text_caption())
                .child(crate::localized_format!("<{tag}> 本地版暂不支持编辑，已原样保留", "<{tag}> Editing is not supported; original content is preserved"))
                .into_any_element(),
        };
        let atomic_sel = !self.ghost
            && !b.kind.has_text()
            && !b.kind.is_container()
            && !self.ed.core.in_block_mode()
            && !self.ed.core.sel.is_collapsed()
            && self.ed.core.atomic_selected(b.id, &self.units);
        let atomic_caret = !self.ghost
            && !b.kind.has_text()
            && !b.kind.is_container()
            && !matches!(b.kind, BlockKind::Table(_) | BlockKind::Image { .. } | BlockKind::File(_))
            && self.ed.core.sel.is_collapsed()
            && self.ed.core.sel.head.block == b.id
            && !self.ed.core.in_block_mode();
        let selected = self.selected.contains(&b.id) || atomic_sel;
        let handle = (!env.cell && !env.in_callout && !self.ghost && self.ed.handle_block() == Some(b.id))
            .then(|| {
                let first_line = match &b.kind {
                    BlockKind::Text | BlockKind::Heading { .. } | BlockKind::Bullet | BlockKind::Ordered { .. } | BlockKind::Todo { .. } => {
                        self.spec_for(b, env).line_height
                    }
                    // Level with the toolbar row at the block top.
                    BlockKind::Code { .. } => 32.,
                    _ => 26.,
                };
                self.ed.handle(b.id, first_line, cx)
            })
            .flatten();
        // `.background-mask`: behind the block, inset like the selection
        // mask; a flattened subtree's rows continue it below.
        let bg = b.bg.as_ref().filter(|_| b.kind.takes_bg()).and_then(|c| theme::block_bg(&c.0)).map(|fill| {
            // Block decoration: 7pt past the column, 9pt above and
            // below (so neighbours with the same decoration merge), radius 3.
            let m = div().absolute().top(px(-9.)).left(px(-7.)).right(px(-7.)).bottom(px(-9.)).bg(fill);
            if self.flat == Some(b.id) { m.rounded_t(px(3.)) } else { m.rounded(px(3.)) }
        });
        div()
            .relative()
            .w_full()
            .children(bg)
            .child(content)
            .child(record_bounds(b.id, self.bounds.clone()))
            .children(handle)
            .children(self.fold_button(b, env, cx))
            .when(!self.ghost && self.ed.is_dragged(b.id), |d| d.opacity(0.5))
            .children(self.ed.drop_flash_el(b.id).filter(|_| !self.ghost))
            .when(atomic_caret && self.focused, |d| {
                d.child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .rounded(px(4.))
                        .border_2()
                        .border_color(Theme::primary_500()),
                )
            })
            // A table draws its own selection (tinted bars and cells).
            .when(selected && !matches!(b.kind, BlockKind::Table(_) | BlockKind::File(_)), |d| {
                // Rows of a flattened subtree continue the highlight below.
                let (tint, r) = self.ed.block_tint();
                let hl = div().absolute().top_0().left(px(-2.)).right(px(-2.)).bottom_0().bg(tint);
                d.child(if self.flat == Some(b.id) { hl.rounded_t(px(r)) } else { hl.rounded(px(r)) })
            })
            .into_any_element()
    }

    /// `.fold-handler`: an 18×16 box (2px left padding) 20px left of the
    /// block, shown while the block is hovered and always once folded. The
    /// triangle turns −90° over .2s ease-out.
    fn fold_button(&self, b: &Block, env: Env, cx: &mut Context<Editor>) -> Option<AnyElement> {
        if env.cell || env.in_callout || self.ghost || !self.ed.core.can_fold(b.id) {
            return None;
        }
        if !b.folded && self.ed.handle_block() != Some(b.id) {
            return None;
        }
        let top = (self.spec_for(b, env).line_height - 16.) / 2.;
        // Grid columns drop the padding; a quote's own children sit left of its bar.
        let (left, pad) = match self.ed.core.parent_of(b.id).map(|p| &p.kind) {
            Some(BlockKind::Column { .. }) => (-20., 0.),
            Some(BlockKind::Quote) => (-34., 2.),
            _ => (-20., 2.),
        };
        let k = match self.ed.fold_anim {
            Some((id, t0)) if id == b.id => (t0.elapsed().as_secs_f32() / 0.2).min(1.),
            _ => 1.,
        };
        let eased = 1. - (1. - k).powi(3);
        let turn = if b.folded { eased } else { 1. - eased };
        let id = b.id;
        Some(
            div()
                // A new id per state, so the open tooltip restarts with the new label.
                .id((if b.folded { "unfold" } else { "fold" }, id.0))
                .group("fold-handler")
                .absolute()
                .left(px(left))
                .top(px(top))
                .w(px(16. + pad))
                .h(px(16.))
                .pl(px(pad))
                .cursor_pointer()
                .child(
                    icon("triangle-down", 16., Theme::text_caption())
                        .group_hover("fold-handler", |s| s.text_color(Theme::text()))
                        .with_transformation(Transformation::rotate(radians(-std::f32::consts::FRAC_PI_2 * turn))),
                )
                .tip_above(if b.folded { crate::settings::text("展开") } else { crate::settings::text("收起") }, None)
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |e, _, window, cx| {
                        cx.stop_propagation();
                        e.core.toggle_fold(id);
                        e.core.fix_selection();
                        e.keep_read_only_fold();
                        e.fold_anim = Some((id, std::time::Instant::now()));
                        e.changed(window, cx);
                    }),
                )
                .into_any_element(),
        )
    }

    fn text_block(&self, b: &Block, env: Env, cx: &mut Context<Editor>) -> AnyElement {
        if crate::settings::feishu() { return self.text_block_feishu(b, env, cx); }

        let spec = self.spec_for(b, env);
        let placeholder = match b.kind {
            // Table cells never show the hint (only the caret does).
            BlockKind::Text if !env.cell && (self.caret_in_empty(b) || self.only_empty_block(b)) && self.ed.slash.is_none() => {
                Some(crate::settings::text(TEXT_PLACEHOLDER).into())
            }
            // Empty headings always show "H1"…"H9", focused or not.
            BlockKind::Heading { level, .. } if b.text.is_empty() => Some(format!("H{level}").into()),
            _ => None,
        };
        let (lh, size, weight) = (spec.line_height, spec.size, spec.weight);
        let mut el = self.text_el(b.id, &b.text, spec, b.align, placeholder);
        // A doc whose only block is empty shows the "first-line placeholder"
        // instead: #8f959e (measured) and 2px in from the content edge.
        if matches!(b.kind, BlockKind::Text) && self.only_empty_block(b) {
            el.placeholder_color = Some(Theme::text_placeholder());
            el.placeholder_dx = 2.;
        }
        let BlockKind::Heading { .. } = b.kind else {
            return div().child(el).child(self.children_of(b, env, cx)).into_any_element();
        };
        // `.heading-order`: a flex row of number + text, on one baseline.
        let line = match self.ed.heading_orders.get(&b.id) {
            Some(order) => div()
                .flex()
                .child(
                    div()
                        .flex_none()
                        .h(px(lh))
                        .px(px(2.))
                        .mr(px(6.))
                        .rounded(px(2.))
                        .flex()
                        .items_center()
                        .whitespace_nowrap()
                        .font_family(theme::UI_FONT)
                        .font_weight(weight)
                        .text_size(px(size))
                        .text_color(Theme::text_caption())
                        .child(order.clone()),
                )
                .child(div().flex_1().min_w_0().child(el))
                .into_any_element(),
            None => el.into_any_element(),
        };
        div().child(line).child(self.children_of(b, env, cx)).into_any_element()
    }

    fn children_of(&self, b: &Block, env: Env, cx: &mut Context<Editor>) -> Div {
        if crate::settings::feishu() { return self.children_of_feishu(b, env, cx); }

        if b.children.is_empty() || b.folded || self.flat == Some(b.id) {
            return div();
        }
        let indent = theme::style().num("list.indent");
        div().pl(px(indent)).mt(px(theme::block_gap())).child(self.blocks(&b.children, Env { depth: env.depth + 1, width: env.width - indent, ..env }, cx))
    }

    fn list_item(&self, b: &Block, siblings: &[Block], idx: usize, env: Env, cx: &mut Context<Editor>) -> AnyElement {
        if crate::settings::feishu() { return self.list_item_feishu(b, siblings, idx, env, cx); }

        let spec = self.spec_for(b, env);
        let lh = spec.line_height;
        let id = b.id;
        let marker: AnyElement = match b.kind {
            // Drawn rather than typeset so the dot size (5.5) and its centring
            // on the line don't depend on how the font's bullet glyph renders.
            // Bullets: text-coloured dots (hollow on the second level).
            BlockKind::Bullet => {
                let c = theme::style().color("bullet.color").unwrap_or_else(Theme::text);
                let dot = div().size(px(5.)).flex_none();
                let dot = match env.depth % 3 {
                    0 => dot.rounded_full().bg(c),
                    1 => dot.rounded_full().border_1().border_color(c),
                    _ => dot.size(px(4.5)).rounded(px(1.)).bg(c),
                };
                div().w(px(18.)).pl(px(5.)).mr(px(6.)).flex().child(dot).into_any_element()
            }
            BlockKind::Ordered { .. } => div()
                .id(("ordered-number", id.0))
                .rounded(px(3.))
                .cursor_pointer()
                .hover(|s| s.bg(Theme::fill_hover()))
                .on_mouse_down(MouseButton::Left, cx.listener(move |e, ev: &MouseDownEvent, window, cx| {
                    cx.stop_propagation();
                    if !e.read_only { e.open_ordered_number(id, env.depth, ev.position, window, cx); }
                }))
                .min_w(px(18.))
                .px(px(1.))
                .mr(px(6.))
                .whitespace_nowrap()
                .font_family(theme::UI_FONT)
                .text_size(px(spec.size))
                .text_color(theme::style().color("number.color").unwrap_or_else(Theme::text))
                .child(core::ordered_marker(core::ordered_number(siblings, idx), env.depth))
                .into_any_element(),
            _ => {
                // The task box: a rounded square, hairline grey open,
                // accent-filled with a white check when done.
                let done = matches!(b.kind, BlockKind::Todo { done: true });
                let st = theme::style();
                let (box_size, fill) = (st.num("task.size"), st.color("task.checked").unwrap_or_else(Theme::primary));
                let check = st.color("task.check").unwrap_or_else(gpui::white);
                div()
                    .id(("todo", id.0))
                    .size(px(box_size))
                    .mr(px(8.))
                    .rounded(px(st.num("task.radius")))
                    .border_1()
                    .when(!done, |d| d.border_color(Theme::todo_box()).bg(crate::theme::raised(Theme::bg_body())).hover(|s| s.border_color(Theme::primary())))
                    .when(done, |d| d.border_color(fill).bg(fill))
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .when(done, |d| d.child(icon("todo-check", (box_size * 13. / 17.).round(), check)))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |e, _, window, cx| {
                            cx.stop_propagation();
                            window.focus(&e.focus);
                            e.core.toggle_todo(id);
                            e.changed(window, cx);
                        }),
                    )
                    .into_any_element()
            }
        };
        let el = self.text_el(b.id, &b.text, spec, b.align, None);
        // To-do rows get a faint 4px-rounded fill while hovered or holding the caret.
        let todo = matches!(b.kind, BlockKind::Todo { .. });
        let core = &self.ed.core;
        let caret_here = self.focused && core.sel.is_collapsed() && core.sel.head.block == id && !core.in_block_mode();
        div()
            .child(
                div()
                    .id(("row", id.0))
                    .flex()
                    .when(todo, |d| {
                        d.mx(px(-3.))
                            .px(px(3.))
                            .rounded(px(6.))
                            .when(caret_here, |d| d.bg(Theme::block_bg_area()))
                            .hover(|s| s.bg(Theme::block_bg_area()))
                    })
                    .child(
                        div()
                            .flex_none()
                            .h(px(lh))
                            .flex()
                            .items_center()
                            .child(marker),
                    )
                    .child(div().flex_1().min_w_0().child(el)),
            )
            .child(self.children_of(b, env, cx))
            .into_any_element()
    }

    fn code_block(&self, b: &Block, lang: &str, wrap: bool, cx: &mut Context<Editor>) -> AnyElement {
        if crate::settings::feishu() { return self.code_block_feishu(b, lang, wrap, cx); }

        let id = b.id;
        let g = SharedString::from(format!("code-{}", id.0));
        let st = theme::style();
        let (code_size, code_line, radius) = (st.num("pre.size"), st.num("pre.line"), st.num("pre.radius"));
        let gutter_w = if st.flag("pre.numbers") { GUTTER_W } else { 0. };
        // SF Mono 13 on a 20pt line.
        let spec = TextSpec {
            size: code_size,
            line_height: code_line,
            weight: FontWeight::NORMAL,
            color: Theme::code_text(),
            mono: true,
            strike_all: false,
            wrap,
            letter_spacing: 0.,
            dim_marks: false,
            link_underline: false,
            doc_font: true,
            font: None,
            italic: false,
        };
        // Line numbers follow the wrapped rows of the last layout.
        let rows_per_line: Vec<usize> = {
            let hard = b.text.text.split('\n').count();
            let mut v = vec![1; hard];
            if wrap && let Some(rows) = self.ed.code_rows.get(&id) {
                for (n, r) in v.iter_mut().zip(rows) {
                    *n = (*r).max(1);
                }
            }
            v
        };
        let text = b.text.text.clone();
        let mut el = self.text_el(id, &b.text, spec, Align::Left, None);
        if lang != "Plain Text" {
            let lang = lang.to_string();
            el.highlight = Some(Box::new(move |t| crate::highlight::spans(&lang, t)));
        }
        // Header: fold triangle, caption at x 29, then the toolbar (22 tall,
        // right-aligned), revealed on hover / while editing.
        let editing = self.focused && self.ed.core.sel.head.block == id;
        let folded = self.ed.code_folded.contains(&id);
        // Handle hovered / its menu open (`.hovered`): light blue ring
        // over a clear body so the block tint shows; block-selected: blue
        // border on the gray body.
        let menu_on = matches!(&self.ed.menu, Some(Menu::Block { block, .. }) | Some(Menu::Add { block, .. }) if *block == id);
        let selected = !menu_on && self.selected.contains(&id);
        // Toolbar items sit in a floating white pill (top right).
        let header_btn = |id_s: &'static str| {
            div()
                .id((id_s, id.0))
                .relative()
                .h(px(22.))
                .pl(px(2.))
                .pr(px(4.))
                .flex()
                .items_center()
                .gap(px(4.))
                .rounded(px(4.))
                .text_size(px(14.))
                .line_height(px(22.))
                .text_color(Theme::text_caption())
                .cursor_pointer()
                .hover(|s| s.bg(Theme::fill_hover_strong()))
                .active(|s| s.bg(crate::theme::adapt(rgba(0x1f232933).into())))
        };
        // 1px divider centred in the 17px gap after a toolbar item.
        let sep = |right: f32| div().absolute().top(px(3.)).right(px(right)).w(px(1.)).h(px(16.)).bg(Theme::line_divider());
        let lang_btn = if folded {
            // Folded: the language is a plain label.
            div()
                .relative()
                .h(px(22.))
                .px(px(4.))
                .flex()
                .items_center()
                .text_size(px(14.))
                .text_color(Theme::text_caption())
                .child(SharedString::from(lang.to_string()))
                .child(sep(-8.))
                .into_any_element()
        } else {
            header_btn("code-lang")
                .px(px(0.))
                .gap(px(0.))
                .text_color(Theme::text())
                .child(div().px(px(4.)).child(SharedString::from(lang.to_string())))
                .child(icon("caret-down", 14., Theme::text_placeholder()))
                .child(sep(-8.))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |e, _, window, cx| {
                        cx.stop_propagation();
                        e.open_lang_menu(id, window, cx);
                    }),
                )
                .into_any_element()
        };
        let caption = self.ed.caption_field(id).map(|f| {
            div()
                .id(("code-caption", id.0))
                .flex_1()
                .min_w(px(50.))
                .ml(px(if gutter_w > 0. { gutter_w + 10. } else { 16. }))
                .mr(px(16.))
                .h(px(28.))
                .flex()
                .child(f)
                // The field takes the click; keep the page from moving the caret.
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|e, _, _, cx| {
                        cx.stop_propagation();
                        e.core.clear_block_sel();
                        e.menu = None;
                        cx.notify();
                    }),
                )
        });
        let fold_btn = div()
            .id(("code-fold", id.0))
            .absolute()
            .left(px(6.))
            .top(px(7.))
            .size(px(24.))
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .when(!folded, |d| d.opacity(0.).group_hover(g.clone(), |s| s.opacity(1.)))
            .child(
                div()
                    .size(px(16.))
                    .rounded(px(4.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .hover(|s| s.bg(Theme::fill_hover()))
                    .child(icon(if folded { "code-fold-right" } else { "code-fold-down" }, 12., Theme::text_caption())),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |e, _, _, cx| {
                    cx.stop_propagation();
                    if !e.code_folded.remove(&id) {
                        e.code_folded.insert(id);
                        // Leave the hidden code (it opens again if the caret comes back).
                        if e.core.sel.head.block == id || e.core.sel.anchor.block == id {
                            e.core.select_block(id);
                        }
                    }
                    cx.notify();
                }),
            );
        let border = if menu_on {
            Theme::code_hover_ring()
        } else if selected {
            Theme::primary_500()
        } else {
            Theme::code_border()
        };
        // The code block: #FBFBFB in a 1pt rim, radius 6, a #F8F8F8 line-number
        // gutter split off by a hairline.
        div()
            .relative()
            .group(g.clone())
            .rounded(px(radius))
            .border_1()
            .border_color(border)
            .when(!menu_on, |d| d.bg(Theme::code_bg()))
            .when(menu_on, |d| {
                d.child(outer_ring(Theme::code_hover_ring()))
            })
            .when(!folded && !menu_on && gutter_w > 0., |d| {
                d.child(
                    div()
                        .absolute()
                        .top_0()
                        .bottom_0()
                        .left_0()
                        .w(px(gutter_w))
                        .rounded_l(px(radius))
                        .bg(Theme::code_gutter())
                        .border_r_1()
                        .border_color(Theme::code_gutter_line()),
                )
            })
            // Pointer over the block: the rim darkens a step.
            .when(!folded && !menu_on && !selected, |d| d.hover(|s| s.border_color(theme::c(craft::N400))))
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .right_0()
                    .h(px(32.))
                    .pt(px(6.))
                    .pr(px(6.))
                    .flex()
                    .items_center()
                    .font_family(theme::UI_FONT)
                    .children(caption)
                    .child(
                        div()
                            .flex()
                            .flex_none()
                            .items_center()
                            .gap(px(17.))
                            .when(!editing, |d| d.opacity(0.).group_hover(g.clone(), |s| s.opacity(1.)))
                            .child(lang_btn)
                            .when(!folded, |d| {
                                d.child(
                                    header_btn("code-wrap")
                                        .child(icon("wrap-text", 14., Theme::text_caption()))
                                        .child(if wrap { crate::settings::text("取消换行") } else { crate::settings::text("自动换行") })
                                        .child(sep(-3.))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |e, _, window, cx| {
                                                cx.stop_propagation();
                                                e.core.checkpoint(core::EditKind::Other);
                                                if let Some(b) = e.core.block_mut(id)
                                                    && let BlockKind::Code { wrap, .. } = &mut b.kind
                                                {
                                                    *wrap = !*wrap;
                                                }
                                                e.changed(window, cx);
                                            }),
                                        ),
                                )
                            })
                            .child(
                                header_btn("code-copy")
                                    .child(icon("copy", 14., Theme::text_caption()))
                                    .child(crate::settings::tr("复制", "Copy"))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |_, _, _, cx| {
                                            cx.stop_propagation();
                                            cx.write_to_clipboard(ClipboardItem::new_string(text.clone()));
                                        }),
                                    ),
                            ),
                    ),
            )
            .child(fold_btn)
            .when(folded, |d| d.child(div().h(px(36.))))
            .when(!folded, |d| {
                d.child(
                    div()
                        .pt(px(36.))
                        .pb(px(12.))
                        .pr(px(16.))
                        .flex()
                        .when(gutter_w == 0., |d| d.pl(px(16.)))
                        .when(gutter_w > 0., |d| d.child(
                            // Numbers right-aligned in the gutter, code 10pt past its hairline.
                            div()
                                .flex_none()
                                .w(px(gutter_w))
                                .pr(px(9.))
                                .mr(px(10.))
                                .flex()
                                .flex_col()
                                .items_end()
                                .font_family(theme::MONO_FONT)
                                .text_size(px(12.))
                                .line_height(px(code_line))
                                .text_color(Theme::code_line_number())
                                .children(
                                    rows_per_line
                                        .iter()
                                        .enumerate()
                                        .map(|(i, n)| div().h(px(code_line * *n as f32)).child(format!("{}", i + 1))),
                                ),
                        ))
                        .child(div().flex_1().min_w_0().overflow_hidden().child(el)),
                )
            })
            .into_any_element()
    }
}

/// Code blocks: SF Mono 13 / 20, a 36pt line-number gutter.
const GUTTER_W: f32 = 36.;

pub(super) const TOP: Env = Env { depth: 0, cell: false, color: None, in_callout: false, root: true, column: false, quote: false, width: 0. };

impl Editor {
    /// Rebuilds the list items from the document and tells the list which of
    /// them changed (only those are re-measured; off-screen ones keep their
    /// heights).
    fn sync_items(&mut self, focused: bool, cover_h: f32) {
        let mut items = vec![Item::Header];
        let mut h = FxHasher::default();
        // Every row re-measures when the text column changes width.
        let seed = (self.layout.ml.to_bits(), self.layout.mr.to_bits(), self.column_max().map(f32::to_bits), self.img.generation, crate::themes::generation(), &self.sub_pages, self.links.generation);
        seed.hash(&mut h);
        let seed = h.finish();
        self.core.doc.title.text.len().hash(&mut h);
        self.core.doc.icon.is_some().hash(&mut h);
        self.core.doc.cover.as_ref().map(|_| cover_h.to_bits()).hash(&mut h);
        self.info.hidden.hash(&mut h);
        let mut keys = vec![h.finish()];
        let links = self.links_key();
        // Rows whose look depends on editor state, not the document.
        let mut nonces = self.grid_overrides();
        // A folded code block is a 40px row.
        nonces.extend(self.code_folded.iter().map(|&id| (id, id.0.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1)));
        nonces.extend(self.image_nonces(focused).into_iter().chain(self.button_nonce()));
        // Walking a long document costs ~1.5ms (1M chars): skip it when
        // nothing the rows depend on changed, as on every scrolling frame.
        let mut fh = FxHasher::default();
        (self.core.generation, seed, keys[0], links, &nonces).hash(&mut fh);
        let frame_key = fh.finish();
        if self.items_key == Some(frame_key) {
            return;
        }
        self.items_key = Some(frame_key);
        self.heading_orders = core::heading_orders(&self.core.doc.blocks);
        let orders = &self.heading_orders;
        #[allow(clippy::too_many_arguments)]
        fn rows(list: &[Block], orders: &rustc_hash::FxHashMap<BlockId, String>, path: &mut Vec<usize>, lead: f32, seed: u64, items: &mut Vec<Item>, keys: &mut Vec<u64>) {
            for (n, (ix, gap)) in visible_with_gaps(list, TOP).into_iter().enumerate() {
                // A nested column starts 8px below its parent (`children_of`).
                let gap = gap + if n == 0 { lead } else { 0. };
                path.push(ix);
                let mut h = FxHasher::default();
                seed.hash(&mut h);
                gap.to_bits().hash(&mut h);
                path.len().hash(&mut h);
                hash_block(&list[ix], orders, &mut h);
                items.push(Item::Block { path: path.clone(), gap });
                keys.push(h.finish());
                if flattens(&list[ix]) {
                    rows(&list[ix].children, orders, path, 8., seed, items, keys);
                }
                path.pop();
            }
        }
        rows(&self.core.doc.blocks, orders, &mut vec![], 0., seed, &mut items, &mut keys);
        items.push(Item::Below);
        keys.push(links);
        for (id, nonce) in nonces {
            if let Some(ix) = super::grid::item_containing(&items, &self.core.doc.blocks, id) {
                keys[ix] ^= nonce;
            }
        }

        let old = &self.item_keys;
        let pre = old.iter().zip(&keys).take_while(|(a, b)| a == b).count();
        if pre < old.len() || old.len() != keys.len() {
            let room = old.len().min(keys.len()) - pre;
            let suf = old.iter().rev().zip(keys.iter().rev()).take(room).take_while(|(a, b)| a == b).count();
            let range = pre..old.len() - suf;
            let count = keys.len() - suf - pre;
            // Keep the page still while the item under the top edge changes
            // (the list would otherwise snap to its first line).
            let top = self.list.logical_scroll_top();
            self.list.splice(range.clone(), count);
            if range.contains(&top.item_ix) {
                self.list.scroll_to(match count {
                    0 => ListOffset { item_ix: pre, offset_in_item: px(0.) },
                    _ => ListOffset { item_ix: top.item_ix.min(pre + count - 1), offset_in_item: top.offset_in_item },
                });
            }
        }
        self.items = items;
        self.item_keys = keys;
    }

    /// The list item showing `id` (the deepest row on its path).
    pub(super) fn item_of(&self, id: BlockId) -> Option<usize> {
        if id == TITLE {
            return Some(0);
        }
        super::grid::item_containing(&self.items, &self.core.doc.blocks, id)
    }

    fn render_item(&mut self, ix: usize, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let Some(item) = self.items.get(ix).cloned() else { return div().into_any_element() };
        if let Item::Block { path, .. } = &item {
            let mut list = self.core.doc.blocks.as_slice();
            let mut b = None;
            for &i in path {
                b = list.get(i);
                list = b.map_or(&[][..], |b| b.children.as_slice());
            }
            let found = b.map(super::caption::captions_in).unwrap_or_default();
            if !found.is_empty() {
                self.ensure_captions(found, window, cx);
            }
        }
        let mut r = R {
            ed: self,
            units: self.frame.units.clone(),
            eqnos: self.frame.eqnos.clone(),
            focused: self.frame.focused,
            selected: self.frame.selected.clone(),
            targets: RefCell::new(vec![]),
            flat: None,
            cache: self.cache.clone(),
            bounds: self.block_bounds.clone(),
            ghost: false,
        };
        let mut cover = None;
        let content = match item {
            Item::Header => {
                // The page title above the document body.
                let mut spec = TextSpec::body();
                spec.size = theme::title().size;
                spec.line_height = theme::title().line_height;
                spec.weight = FontWeight(theme::style().num("title.weight"));
                if let Some(c) = theme::style().color("title.color") { spec.color = c; }
                let tf = theme::style().font("title.font");
                if tf != theme::body_font() { spec.font = Some(tf); }
                cover = self.cover_el(window, cx);
                div()
                    .relative()
                    .group("doc-header")
                    .pt(px(if crate::settings::feishu() { 48. } else { 52. }))
                    .pb(px(if crate::settings::feishu() { 22. } else { 24. }))
                    .children(self.header_entries(cx))
                    .child(
                        div()
                            .flex()
                            .items_start()
                            .children(self.doc_icon_el(cx))
                            .child(div().flex_1().min_w_0().child(r.text_el(TITLE, &self.core.doc.title, spec, Align::Left, Some(crate::settings::text("请输入标题").into())))),
                    )
                    .into_any_element()
            }
            Item::Block { path, gap } => {
                let Some((&bi, outer)) = path.split_last() else { return div().into_any_element() };
                let mut list = self.core.doc.blocks.as_slice();
                // Depth of the outermost selected ancestor, whose highlight
                // spans its children's rows too.
                let mut sel_depth = None;
                // Rows of a dragged block's subtree fade with it.
                let mut dimmed = false;
                // Ancestors whose background spans this row.
                let mut bgs = vec![];
                for (d, &i) in outer.iter().enumerate() {
                    let Some(b) = list.get(i) else { return div().into_any_element() };
                    if sel_depth.is_none() && r.selected.contains(&b.id) {
                        sel_depth = Some(d);
                    }
                    if let Some(fill) = b.bg.as_ref().filter(|_| b.kind.takes_bg()).and_then(|c| theme::block_bg(&c.0)) {
                        bgs.push((d, fill));
                    }
                    dimmed |= self.is_dragged(b.id);
                    list = &b.children;
                }
                let Some(b) = list.get(bi) else { return div().into_any_element() };
                let depth = outer.len();
                r.flat = flattens(b).then_some(b.id);
                let ends = |d: usize| !matches!(self.items.get(ix + 1), Some(Item::Block { path: next, .. }) if next.starts_with(&path[..=d]));
                let last = sel_depth.is_some_and(ends);
                div()
                    .relative()
                    .pt(px(gap))
                    .pl(px(25. * depth as f32))
                    .when(dimmed, |d| d.opacity(0.5))
                    .children(bgs.into_iter().map(|(d, fill)| {
                        div()
                            .absolute()
                            .top_0()
                            .bottom_0()
                            .left(px(25. * d as f32 - 2.))
                            .right(px(-2.))
                            .when(ends(d), |e| e.rounded_b(px(3.)))
                            .bg(fill)
                    }))
                    .child(r.block(b, list, bi, Env { depth, root: depth == 0, width: self.layout.content - 25. * depth as f32, ..TOP }, cx))
                    .when_some(sel_depth, |d, sd| {
                        d.child(
                            div()
                                .absolute()
                                .top_0()
                                .bottom_0()
                                .left(px(25. * sd as f32 - 2.))
                                .right(px(-2.))
                                .when(last, |d| d.rounded_b(px(self.block_tint().1)))
                                .bg(self.block_tint().0),
                        )
                    })
                    .into_any_element()
            }
            Item::Below => div()
                .id("below")
                .w_full()
                .child(div().h(px(200.)).w_full())
                .on_mouse_down(MouseButton::Left, cx.listener(Self::on_click_below))
                .into_any_element(),
        };
        let targets = r.targets.into_inner();
        self.handle_targets.extend(targets);
        let padded = div().w_full().pl(px(self.layout.ml)).pr(px(self.layout.mr)).child(content);
        match cover {
            // The cover spans the whole pane, above the padded header.
            Some(c) => div().w_full().child(c).child(padded).into_any_element(),
            None => padded.into_any_element(),
        }
    }

    /// Applies the pending scroll request against this frame's layout. A
    /// target outside the rendered items is first brought on screen by its
    /// item and refined on the next frame.
    fn apply_reveal(&mut self, window: &mut Window) {
        let Some((req, tries)) = self.reveal.take() else { return };
        let view = self.list.viewport_bounds();
        let caret_rect = |p: Pos| {
            self.cache.borrow().get(&p.block).map(|l| Bounds::new(l.position_for(p.offset), size(px(2.), l.line_height)))
        };
        let (block, target) = match req {
            Reveal::Caret => match self.core.block_sel {
                Some(b) => (b.head, self.block_bounds.borrow().get(&b.head).copied()),
                // A selected image has no text layout: keep its box in view.
                None => {
                    let h = self.core.sel.head;
                    (h.block, caret_rect(h).or_else(|| self.img.boxes.borrow().get(&h.block).copied()))
                }
            },
            Reveal::Find(p) => (p.block, caret_rect(p)),
            Reveal::Block(id) => (id, self.block_bounds.borrow().get(&id).copied()),
        };
        let Some(t) = target else {
            if tries < 3
                && let Some(ix) = self.item_of(block)
            {
                self.list.scroll_to(ListOffset { item_ix: ix, offset_in_item: px(0.) });
                self.reveal = Some((req, tries + 1));
                // `refresh` is a no-op while drawing.
                window.request_animation_frame();
            }
            return;
        };
        let margin = px(40.);
        let d = match req {
            Reveal::Caret if t.top() < view.top() + margin => t.top() - (view.top() + margin),
            Reveal::Caret if t.bottom() > view.bottom() - margin => {
                (t.bottom() - (view.bottom() - margin)).min(t.top() - view.top() - margin)
            }
            Reveal::Caret => px(0.),
            // The find box covers the top right of the page.
            Reveal::Find(_) if t.top() >= view.top() + px(250.) && t.bottom() <= view.bottom() - margin => px(0.),
            Reveal::Find(_) => t.top() - (view.top() + view.size.height / 2.),
            Reveal::Block(_) => t.top() - view.top() - px(24.),
        };
        if d != px(0.) {
            self.list.scroll_by(d);
            window.request_animation_frame();
        }
    }

    fn render_editor(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.sync_crop(window, cx);
        self.tick_image_ui(window, cx);
        // Overlays are positioned from last frame's layout, so build them
        // before the caches are reset for this frame.
        let overlays = self.render_overlays(window, cx);
        let lightbox = self.lightbox_el(window, cx);
        let toast = self.toast_el(window);
        let probes = std::mem::take(&mut *self.probes.borrow_mut());
        let origin0 = self.content_origin.get();
        let counter = self.word_counter();
        let find_box = self.find_box(window, cx);
        if std::mem::take(&mut self.autoscroll) && self.reveal.is_none() {
            self.reveal = Some((Reveal::Caret, 0));
        }
        // The handle floats above everything (deferred), so the page viewport
        // doesn't clip it: hide it once its block's top scrolls out of view.
        let viewport = self.list.viewport_bounds();
        self.hover_clipped = self
            .hover
            .and_then(|id| self.block_bounds.borrow().get(&id).copied())
            .is_some_and(|b| b.top() < viewport.top() || b.top() + px(26.) > viewport.bottom());
        self.code_rows = code_rows(&self.cache.borrow());
        // Sets the margins `sync_items` keys on.
        self.update_catalogue();

        // Reads last frame's bar segments.
        self.table_autoscroll(window);
        self.block_autoscroll(window);
        self.tick_drop_flash(window);
        if let Some((_, t0)) = self.fold_anim {
            if t0.elapsed().as_secs_f32() >= 0.2 {
                self.fold_anim = None;
            }
            window.request_animation_frame();
        }
        self.cache.borrow_mut().clear();
        self.block_bounds.borrow_mut().clear();
        self.tables.geo.borrow_mut().clear();
        self.tables.stuck.borrow_mut().clear();
        self.img.boxes.borrow_mut().clear();

        if self.tick_grid_animations() {
            window.request_animation_frame();
        }
        let focused = self.focus.is_focused(window);
        self.sync_items(focused, super::cover::cover_height(window));
        let generation = self.core.generation;
        let (units, eqnos) = match self.frame.generation == Some(generation) {
            true => (self.frame.units.clone(), self.frame.eqnos.clone()),
            false => (Rc::new(self.core.units()), Rc::new(equation_numbers(&self.core.doc))),
        };
        self.frame = Frame {
            generation: Some(generation),
            units,
            eqnos,
            selected: Rc::new(self.core.block_sel_ids().into_iter().collect()),
            focused,
        };
        // Filled in as the list renders its visible items.
        self.handle_targets.clear();

        let entity = cx.entity();
        let focus = self.focus.clone();
        let dragging = self.dragging;
        let col_dragging = self.col_drag.is_some();
        let col_overlay = self.col_drag_overlay();
        let table_dragging = self.tables.drag.is_some();
        let table_overlay = self.table_drag_overlay();
        let block_dragging = self.block_drag.is_some();
        let block_overlay = self.block_drag_overlay(cx);
        let image_dragging = self.resizing_image() || self.crop_dragging();
        let cover_grab = self.cover_grabbed();
        let image_layer = self.image_drag_layer();
        let file_line = self.file_drop_line();
        let pointer_hidden = self.pointer_hidden;
        let read_only = self.read_only;
        let origin = self.content_origin.clone();
        let pane0 = self.pane_size();
        let link_rects = self.pointer_rects();
        let root_canvas = canvas(
            move |b, window, _| {
                origin.set(b.origin);
                link_rects.into_iter().map(|r| window.insert_hitbox(r, HitboxBehavior::Normal)).collect::<Vec<_>>()
            },
            move |bounds, link_hitboxes, window, cx| {
                if !read_only {
                    window.handle_input(&focus, ElementInputHandler::new(bounds, entity.clone()), cx);
                }
                // Links point (`cursor: pointer`).
                for h in &link_hitboxes {
                    window.set_cursor_style(CursorStyle::PointingHand, h);
                }
                {
                    let ed = entity.read(cx);
                    let stale = ed.content_origin.get() != origin0
                        || bounds.size != pane0
                        || probes.iter().any(|(p, v)| ed.probe(*p) != *v)
                        || code_rows(&ed.cache.borrow()) != ed.code_rows;
                    if stale {
                        // Painting: `refresh` would be dropped.
                        let e = entity.clone();
                        window.on_next_frame(move |_, cx| e.update(cx, |_, cx| cx.notify()));
                    }
                }
                if pointer_hidden {
                    window.set_window_cursor_style(CursorStyle::None);
                    let e = entity.clone();
                    window.on_mouse_event(move |_: &MouseMoveEvent, phase, _, cx| {
                        if phase == DispatchPhase::Bubble {
                            e.update(cx, |ed, cx| {
                                ed.pointer_hidden = false;
                                cx.notify();
                            });
                        }
                    });
                }
                if dragging {
                    let e = entity.clone();
                    window.on_mouse_event(move |ev: &MouseMoveEvent, phase, _, cx| {
                        if phase == DispatchPhase::Bubble {
                            e.update(cx, |ed, cx| match ev.pressed_button {
                                Some(MouseButton::Left) => ed.drag_to(ev.position, cx),
                                // The button went up somewhere we didn't see.
                                _ => ed.end_drag(None, cx),
                            });
                        }
                    });
                    let e = entity.clone();
                    window.on_mouse_event(move |ev: &MouseUpEvent, phase, _, cx| {
                        if phase == DispatchPhase::Bubble {
                            e.update(cx, |ed, cx| ed.end_drag(Some(ev.position), cx));
                        }
                    });
                }
                if col_dragging {
                    let e = entity.clone();
                    window.on_mouse_event(move |ev: &MouseMoveEvent, phase, window, cx| {
                        if phase == DispatchPhase::Bubble {
                            e.update(cx, |ed, cx| match ev.pressed_button {
                                Some(MouseButton::Left) => ed.col_drag_to(ev.position.x, cx),
                                _ => ed.end_col_drag(window, cx),
                            });
                        }
                    });
                    let e = entity.clone();
                    window.on_mouse_event(move |_: &MouseUpEvent, phase, window, cx| {
                        if phase == DispatchPhase::Bubble {
                            e.update(cx, |ed, cx| ed.end_col_drag(window, cx));
                        }
                    });
                }
                if table_dragging {
                    let e = entity.clone();
                    window.on_mouse_event(move |ev: &MouseMoveEvent, phase, window, cx| {
                        if phase == DispatchPhase::Bubble {
                            e.update(cx, |ed, cx| ed.table_drag_move(ev.position, ev.pressed_button == Some(MouseButton::Left), window, cx));
                        }
                    });
                    let e = entity.clone();
                    window.on_mouse_event(move |_: &MouseUpEvent, phase, window, cx| {
                        if phase == DispatchPhase::Bubble {
                            e.update(cx, |ed, cx| ed.end_table_drag(window, cx));
                        }
                    });
                }
                if block_dragging {
                    let e = entity.clone();
                    window.on_mouse_event(move |ev: &MouseMoveEvent, phase, window, cx| {
                        if phase == DispatchPhase::Bubble {
                            e.update(cx, |ed, cx| ed.block_drag_move(ev.position, ev.pressed_button == Some(MouseButton::Left), window, cx));
                        }
                    });
                    let e = entity.clone();
                    window.on_mouse_event(move |ev: &MouseUpEvent, phase, window, cx| {
                        if phase == DispatchPhase::Bubble {
                            e.update(cx, |ed, cx| ed.end_block_drag(Some(ev.position), window, cx));
                        }
                    });
                }
                if cover_grab {
                    // 调整图片位置 follows the pointer outside the cover.
                    let e = entity.clone();
                    window.on_mouse_event(move |ev: &MouseMoveEvent, phase, _, cx| {
                        if phase == DispatchPhase::Bubble {
                            e.update(cx, |ed, cx| match ev.pressed_button {
                                Some(MouseButton::Left) => ed.cover_drag_to(ev.position, cx),
                                _ => ed.end_cover_grab(cx),
                            });
                        }
                    });
                    let e = entity.clone();
                    window.on_mouse_event(move |_: &MouseUpEvent, phase, _, cx| {
                        if phase == DispatchPhase::Bubble {
                            e.update(cx, |ed, cx| ed.end_cover_grab(cx));
                        }
                    });
                }
                if image_dragging {
                    // The handle or crop box keeps following the pointer
                    // outside the page.
                    let e = entity.clone();
                    window.on_mouse_event(move |ev: &MouseMoveEvent, phase, _, cx| {
                        if phase == DispatchPhase::Bubble {
                            e.update(cx, |ed, cx| match ev.pressed_button {
                                Some(MouseButton::Left) if ed.resizing_image() => ed.resize_to(ev.position.x, cx),
                                Some(MouseButton::Left) => ed.crop_drag_to(ev.position, ev.modifiers.shift, cx),
                                _ => {
                                    ed.end_resize(cx);
                                    ed.end_crop_drag(cx);
                                }
                            });
                        }
                    });
                    let e = entity.clone();
                    window.on_mouse_event(move |_: &MouseUpEvent, phase, _, cx| {
                        if phase == DispatchPhase::Bubble {
                            e.update(cx, |ed, cx| {
                                ed.end_resize(cx);
                                ed.end_crop_drag(cx);
                            });
                        }
                    });
                }
                entity.update(cx, |ed, cx| {
                    ed.start_downloads(cx);
                    ed.start_thumbs(cx);
                    // A notify during the draw is dropped (even a deferred
                    // one); send it from a task, after the frame.
                    if ed.update_active() {
                        cx.spawn(async |this, cx| this.update(cx, |_, cx| cx.notify())).detach();
                    }
                    ed.apply_reveal(window);
                });
            },
        )
        .absolute()
        .top_0()
        .left_0()
        .size_full();

        let root = div()
            .key_context(if read_only { "EditorRO" } else { "Editor" })
            .when(read_only, |d| {
                d.on_key_down(cx.listener(|ed, ev: &KeyDownEvent, window, cx| {
                    if ed.focus.is_focused(window) {
                        ed.read_only_key(ev, cx);
                    }
                }))
            })
            .track_focus(&self.focus)
            .size_full()
            .relative()
            // No fill of its own: the note panel behind it is white and rounded.
            .font_family(theme::UI_FONT)
            .text_color(Theme::text())
            .on_mouse_move(cx.listener(Self::on_hover_move))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|ed, ev: &MouseUpEvent, window, cx| {
                    ed.end_resize(cx);
                    ed.end_crop_drag(cx);
                    ed.end_col_drag(window, cx);
                    ed.end_table_drag(window, cx);
                    ed.end_block_drag(Some(ev.position), window, cx);
                    ed.button_release(ev.position, cx);
                    ed.gap_release(ev.position, window, cx);
                    ed.end_drag(Some(ev.position), cx)
                }),
            );
        self.register_actions(root, cx)
            .child(
                div()
                    .id("editor-page")
                    .size_full()
                    .cursor(CursorStyle::IBeam)
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
                    .on_drag_move(cx.listener(Self::file_drag_move))
                    .on_drop(cx.listener(Self::drop_files))
                    .child(list(self.list.clone(), cx.processor(Self::render_item)).size_full()),
            )
            .child(self.page_scrollbar())
            .children(file_line)
            .children(overlays)
            .children(col_overlay)
            .children(table_overlay)
            .children(block_overlay)
            .children(image_layer)
            .children(counter)
            .children(find_box)
            .children(lightbox)
            .children(toast)
            .child(root_canvas)
    }
}

/// Wrapped rows per hard line of every laid-out code block.
fn code_rows(cache: &HashMap<BlockId, super::text::TextLayout>) -> HashMap<BlockId, Vec<usize>> {
    cache.iter().filter(|(_, l)| l.mono).map(|(id, l)| (*id, l.rows_per_line())).collect()
}

impl Render for Editor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t0 = std::time::Instant::now();
        let el = self.render_editor(window, cx).into_any_element();
        let outline = self.render_catalogue(window, cx);
        let refs = self.refs_list(window, cx);
        let open = self.auxiliary_open;
        // On small windows collapse the auxiliary area without changing the saved preference.
        let available = f32::from(self.auxiliary_bounds.get().size.width);
        let fits = available >= self.auxiliary_width + 400.;
        let visible = open && (fits || self.auxiliary_forced);
        let floating = visible && !fits;
        let panel = div().id("auxiliary").relative().h_full().flex_none().flex().flex_col()
            .w(px(if visible { self.auxiliary_width.min(available.max(160.)) } else { 0. })).bg(Theme::bg_body())
            .when(floating, |d| d.absolute().right_0().top_0().bottom_0().border_l_1().border_color(Theme::line_divider()).occlude())
            .when(visible, |d| d.px(px(12.))
                .child(div().id("auxiliary-scroll").flex_1().min_h_0().overflow_y_scroll().children(outline).children(refs))
                .child(div().id("auxiliary-resize").absolute().left_0().top_0().bottom_0().w(px(6.)).cursor(CursorStyle::ResizeLeftRight)
                    .on_mouse_down(MouseButton::Left, cx.listener(|ed, ev: &MouseDownEvent, _, cx| {
                        ed.auxiliary_drag = Some((f32::from(ev.position.x), ed.auxiliary_width)); cx.stop_propagation(); cx.notify();
                    }))));
        let drag = self.auxiliary_drag.map(|(x, width)| deferred(div().id("auxiliary-drag").absolute().inset_0().occlude().cursor(CursorStyle::ResizeLeftRight)
            .on_mouse_move(cx.listener(move |ed, ev: &MouseMoveEvent, _, cx| {
                if ev.pressed_button == Some(MouseButton::Left) {
                    ed.auxiliary_width = (width + x - f32::from(ev.position.x)).clamp(240., 420.);
                    cx.emit(super::view::EditorEvent::AuxiliaryChanged);
                }
                else { ed.auxiliary_drag = None; crate::settings::save_auxiliary(ed.auxiliary_open, ed.auxiliary_width); }
                cx.notify();
            }))
            .on_mouse_up(MouseButton::Left, cx.listener(|ed, _, _, cx| { ed.auxiliary_drag = None; crate::settings::save_auxiliary(ed.auxiliary_open, ed.auxiliary_width); cx.notify(); }))).with_priority(4));
        crate::devtools::RENDER_NS.fetch_add(t0.elapsed().as_nanos() as u64, std::sync::atomic::Ordering::Relaxed);
        let bounds_cell = self.auxiliary_bounds.clone();
        let entity = cx.entity();
        let measure = canvas(move |bounds, _, _| bounds, move |bounds, _, window, _| {
            if bounds_cell.replace(bounds) != bounds { let e = entity.clone(); window.on_next_frame(move |_, cx| e.update(cx, |_, cx| { cx.emit(super::view::EditorEvent::AuxiliaryChanged); cx.notify(); })); }
        }).absolute().inset_0();
        div().size_full().relative().flex().child(div().flex_1().min_w_0().h_full().child(el)).child(panel).children(drag).child(measure)
    }
}

impl Editor {
    /// "N 字" chip at the bottom right while text is selected (§2).
    fn word_counter(&self) -> Option<AnyElement> {
        let n = if self.core.in_block_mode() {
            let index = core::block_index(&self.core.doc);
            self.core
                .block_sel_ids()
                .iter()
                .filter_map(|id| index.get(id))
                .map(|b| crate::docinfo::blocks_word_count(std::slice::from_ref(*b)))
                .sum()
        } else if self.core.sel.is_collapsed() {
            return None;
        } else {
            crate::docinfo::word_count(&self.core.selected_text())
        };
        // Selection word count: a small white pill at the note's bottom right.
        Some(
            div()
                .absolute()
                .bottom(px(10.))
                .right(px(34.))
                .h(px(24.))
                .px(px(9.))
                .rounded(px(8.))
                .bg(crate::theme::raised(Theme::bg_body()))
                .border_1()
                .border_color(Theme::button_border())
                .shadow(theme::shadow_button())
                .text_size(px(12.))
                .line_height(px(22.))
                .text_color(Theme::text_caption())
                .child(if crate::settings::english() { format!("{n} {}", if n == 1 { "word" } else { "words" }) } else { format!("{n} 字") })
                .into_any_element(),
        )
    }
}

impl R<'_> {
fn spec_for_feishu(&self, b: &Block, env: Env) -> TextSpec {
        use crate::feishu_theme::{self as theme, Theme};

        let mut spec = TextSpec::body();
        if env.cell {
            spec.size = 14.;
            spec.line_height = 22.;
        }
        if let BlockKind::Heading { level, .. } = b.kind {
            let m = theme::heading_metrics(level);
            spec.size = m.size;
            spec.line_height = m.line_height;
            spec.weight = FontWeight::MEDIUM;
            spec.letter_spacing = m.size * -0.02;
        }
        spec.dim_marks = env.quote;
        if let Some(c) = env.color {
            spec.color = c;
        }
        if let BlockKind::Todo { done: true } = b.kind {
            spec.color = Theme::todo_done();
            spec.strike_all = true;
        }
        spec
    }
}

impl R<'_> {
pub(super) fn block_feishu(&self, b: &Block, siblings: &[Block], idx: usize, env: Env, cx: &mut Context<Editor>) -> AnyElement {
        use crate::feishu_theme::{self as theme, Theme};

        if !env.cell && !env.in_callout && !matches!(b.kind, BlockKind::Column { .. }) && !self.ghost {
            self.targets.borrow_mut().push(b.id);
        }
        let content: AnyElement = match &b.kind {
            BlockKind::Text | BlockKind::Heading { .. } => self.text_block(b, env, cx),
            BlockKind::Bullet | BlockKind::Ordered { .. } | BlockKind::Todo { .. } => self.list_item(b, siblings, idx, env, cx),
            BlockKind::Code { lang, wrap, .. } => self.code_block(b, lang, *wrap, cx),
            BlockKind::Quote => div()
                .relative()
                .pl(px(14.))
                .child(div().absolute().left_0().top_0().bottom_0().w(px(2.)).rounded(px(1.)).bg(Theme::quote_bar()))
                .child(self.blocks(&b.children, Env { color: Some(Theme::text_caption()), quote: true, width: env.width - 14., ..env }, cx))
                .into_any_element(),
            BlockKind::Callout { emoji, bg, border, text_color } => {
                let bgc = bg.as_ref().and_then(|c| theme::callout_bg(&c.0)).unwrap_or(theme::c(0xfff5eb));
                let bc = border.as_ref().and_then(|c| theme::callout_border(&c.0)).unwrap_or(theme::c(0xfed4a4));
                let tc = text_color.as_ref().and_then(|c| theme::text_color(&c.0)).or(env.color);
                let id = b.id;
                // `.docx-callout-block.hovered` (handle menu open): `--B200` ring;
                // `.selected`: `--text-link-hover` border. Both drop the fill
                // so the block tint shows.
                let menu_on = matches!(&self.ed.menu, Some(Menu::Block { block, .. }) | Some(Menu::Add { block, .. }) if *block == id);
                let selected = !menu_on && self.selected.contains(&id);
                div()
                    .relative()
                    .flex()
                    .p(px(16.))
                    .rounded(px(8.))
                    .border_1()
                    .border_color(if menu_on {
                        Theme::code_hover_ring()
                    } else if selected {
                        Theme::primary_500()
                    } else {
                        bc
                    })
                    .when(!menu_on && !selected, |d| d.bg(bgc))
                    .when(menu_on, |d| {
                        d.child(outer_ring(Theme::code_hover_ring()))
                    })
                    .child(
                        div()
                            .id(("callout-emoji", id.0))
                            .flex_none()
                            .size(px(24.))
                            .mt(px((super::emoji_picker::callout_emoji_height(b.children.first().map(|c| &c.kind)) - 24.) / 2.))
                            .rounded(px(4.))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(20.))
                            .line_height(px(24.))
                            .cursor_pointer()
                            .hover(|s| s.bg(Theme::fill_hover()))
                            .tip_above(crate::settings::text("点击更换图标"), None)
                            .child(SharedString::from(emoji.clone()))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |e, _, window, cx| {
                                    cx.stop_propagation();
                                    e.toggle_emoji_picker(super::emoji_picker::EmojiTarget::Callout(id), window, cx);
                                }),
                            ),
                    )
                    .child(div().flex_1().min_w_0().ml(px(8.)).child(self.blocks(&b.children, Env { color: tc, in_callout: true, width: env.width - 66., ..env }, cx)))
                    .into_any_element()
            }
            BlockKind::Divider => div()
                .h(px(25.))
                .flex()
                .items_center()
                .child(div().h(px(1.)).w_full().bg(Theme::line_divider()))
                .into_any_element(),
            BlockKind::Image { .. } => self.image(b, env, cx),
            BlockKind::SubPageList => self.sub_page_list(b, cx),
            BlockKind::Countdown(c) => self.countdown(b, c, cx),
            BlockKind::File(f) => self.file(b, f, env, cx),
            BlockKind::Table(t) if self.ghost => self.ghost_table(b, t, cx),
            BlockKind::Table(t) => self.table(b, t, cx),
            BlockKind::Grid => self.grid(b, env, cx),
            BlockKind::Column { .. } => self.blocks(&b.children, env, cx).into_any_element(),
            BlockKind::Raw { tag, .. } => div()
                .px(px(12.))
                .py(px(8.))
                .rounded(px(8.))
                .bg(Theme::bg_overlay())
                .text_size(px(14.))
                .line_height(px(22.))
                .text_color(Theme::text_caption())
                .child(crate::localized_format!("<{tag}> 本地版暂不支持编辑，已原样保留", "<{tag}> Editing is not supported; original content is preserved"))
                .into_any_element(),
        };
        let atomic_sel = !self.ghost
            && !b.kind.has_text()
            && !b.kind.is_container()
            && !self.ed.core.in_block_mode()
            && !self.ed.core.sel.is_collapsed()
            && self.ed.core.atomic_selected(b.id, &self.units);
        let atomic_caret = !self.ghost
            && !b.kind.has_text()
            && !b.kind.is_container()
            && !matches!(b.kind, BlockKind::Table(_) | BlockKind::Image { .. } | BlockKind::File(_))
            && self.ed.core.sel.is_collapsed()
            && self.ed.core.sel.head.block == b.id
            && !self.ed.core.in_block_mode();
        let selected = self.selected.contains(&b.id) || atomic_sel;
        let handle = (!env.cell && !env.in_callout && !self.ghost && self.ed.handle_block() == Some(b.id))
            .then(|| {
                let first_line = match &b.kind {
                    BlockKind::Text | BlockKind::Heading { .. } | BlockKind::Bullet | BlockKind::Ordered { .. } | BlockKind::Todo { .. } => {
                        self.spec_for(b, env).line_height
                    }
                    // Centered on the first code line (3px below the block top).
                    BlockKind::Code { .. } => 32.,
                    _ => 26.,
                };
                self.ed.handle(b.id, first_line, cx)
            })
            .flatten();
        // `.background-mask`: behind the block, inset like the selection
        // mask; a flattened subtree's rows continue it below.
        let bg = b.bg.as_ref().filter(|_| b.kind.takes_bg()).and_then(|c| theme::block_bg(&c.0)).map(|fill| {
            let m = div().absolute().top_0().left(px(-2.)).right(px(-2.)).bottom_0().bg(fill);
            if self.flat == Some(b.id) { m.rounded_t(px(6.)) } else { m.rounded(px(6.)) }
        });
        div()
            .relative()
            .w_full()
            .children(bg)
            .child(content)
            .child(record_bounds(b.id, self.bounds.clone()))
            .children(handle)
            .children(self.fold_button(b, env, cx))
            .when(!self.ghost && self.ed.is_dragged(b.id), |d| d.opacity(0.5))
            .children(self.ed.drop_flash_el(b.id).filter(|_| !self.ghost))
            .when(atomic_caret && self.focused, |d| {
                d.child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .rounded(px(4.))
                        .border_2()
                        .border_color(Theme::primary_500()),
                )
            })
            // A table draws its own selection (tinted bars and cells).
            .when(selected && !matches!(b.kind, BlockKind::Table(_) | BlockKind::File(_)), |d| {
                // Rows of a flattened subtree continue the highlight below.
                let (tint, r) = self.ed.block_tint();
                let hl = div().absolute().top_0().left(px(-2.)).right(px(-2.)).bottom_0().bg(tint);
                d.child(if self.flat == Some(b.id) { hl.rounded_t(px(r)) } else { hl.rounded(px(r)) })
            })
            .into_any_element()
    }
}

impl R<'_> {
fn text_block_feishu(&self, b: &Block, env: Env, cx: &mut Context<Editor>) -> AnyElement {
        use crate::feishu_theme::Theme;

        let spec = self.spec_for(b, env);
        let placeholder = match b.kind {
            // Table cells never show the hint (only the caret does).
            BlockKind::Text if !env.cell && (self.caret_in_empty(b) || self.only_empty_block(b)) && self.ed.slash.is_none() => {
                Some(crate::settings::text(TEXT_PLACEHOLDER).into())
            }
            // Empty headings always show "H1"…"H9", focused or not.
            BlockKind::Heading { level, .. } if b.text.is_empty() => Some(format!("H{level}").into()),
            _ => None,
        };
        let (lh, size, weight) = (spec.line_height, spec.size, spec.weight);
        let mut el = self.text_el(b.id, &b.text, spec, b.align, placeholder);
        // A doc whose only block is empty shows the "first-line placeholder"
        // instead: #8f959e (measured) and 2px in from the content edge.
        if matches!(b.kind, BlockKind::Text) && self.only_empty_block(b) {
            el.placeholder_color = Some(Theme::text_placeholder());
            el.placeholder_dx = 2.;
        }
        let BlockKind::Heading { .. } = b.kind else {
            return div().child(el).child(self.children_of(b, env, cx)).into_any_element();
        };
        // `.heading-order`: a flex row of number + text, on one baseline.
        let line = match self.ed.heading_orders.get(&b.id) {
            Some(order) => div()
                .flex()
                .child(
                    div()
                        .flex_none()
                        .h(px(lh))
                        .px(px(2.))
                        .mr(px(6.))
                        .rounded(px(2.))
                        .flex()
                        .items_center()
                        .whitespace_nowrap()
                        .font_family("Helvetica Neue")
                        .font_weight(weight)
                        .text_size(px(size))
                        .text_color(Theme::primary())
                        .child(order.clone()),
                )
                .child(div().flex_1().min_w_0().child(el))
                .into_any_element(),
            None => el.into_any_element(),
        };
        div().child(line).child(self.children_of(b, env, cx)).into_any_element()
    }
}

impl R<'_> {
fn list_item_feishu(&self, b: &Block, siblings: &[Block], idx: usize, env: Env, cx: &mut Context<Editor>) -> AnyElement {
        use crate::feishu_theme::Theme;

        let spec = self.spec_for(b, env);
        let lh = spec.line_height;
        let id = b.id;
        let marker: AnyElement = match b.kind {
            // Drawn rather than typeset so the dot size (5.5) and its centring
            // on the line don't depend on how the font's bullet glyph renders.
            BlockKind::Bullet => {
                let dot = div().size(px(5.5)).flex_none();
                let dot = match env.depth % 3 {
                    0 => dot.rounded_full().bg(Theme::primary()),
                    1 => dot.rounded_full().border_1().border_color(Theme::primary()),
                    _ => dot.size(px(5.)).bg(Theme::primary()),
                };
                div().w(px(18.)).pl(px(4.)).mr(px(6.)).flex().child(dot).into_any_element()
            }
            BlockKind::Ordered { .. } => div()
                .id(("ordered-number", id.0))
                .rounded(px(3.))
                .cursor_pointer()
                .hover(|s| s.bg(Theme::fill_hover()))
                .on_mouse_down(MouseButton::Left, cx.listener(move |e, ev: &MouseDownEvent, window, cx| {
                    cx.stop_propagation();
                    if !e.read_only { e.open_ordered_number(id, env.depth, ev.position, window, cx); }
                }))
                .min_w(px(18.))
                .px(px(2.))
                .mr(px(6.))
                .whitespace_nowrap()
                .font_family("Helvetica Neue")
                .text_size(px(spec.size))
                .text_color(Theme::primary())
                .child(core::ordered_marker(core::ordered_number(siblings, idx), env.depth))
                .into_any_element(),
            _ => {
                let done = matches!(b.kind, BlockKind::Todo { done: true });
                div()
                    .id(("todo", id.0))
                    .size(px(16.))
                    .mr(px(8.))
                    .rounded(px(4.))
                    .border_1()
                    .border_color(Theme::text())
                    .hover(|s| s.border_color(Theme::primary()))
                    .bg(crate::theme::raised(Theme::bg_body()))
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .when(done, |d| d.child(icon("todo-check", 14., Theme::primary_500())))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |e, _, window, cx| {
                            cx.stop_propagation();
                            window.focus(&e.focus);
                            e.core.toggle_todo(id);
                            e.changed(window, cx);
                        }),
                    )
                    .into_any_element()
            }
        };
        let el = self.text_el(b.id, &b.text, spec, b.align, None);
        // To-do rows get a faint 4px-rounded fill while hovered or holding the caret.
        let todo = matches!(b.kind, BlockKind::Todo { .. });
        let core = &self.ed.core;
        let caret_here = self.focused && core.sel.is_collapsed() && core.sel.head.block == id && !core.in_block_mode();
        div()
            .child(
                div()
                    .id(("row", id.0))
                    .flex()
                    .when(todo, |d| {
                        d.mx(px(-2.))
                            .px(px(2.))
                            .rounded(px(4.))
                            .when(caret_here, |d| d.bg(Theme::block_bg_area()))
                            .hover(|s| s.bg(Theme::block_bg_area()))
                    })
                    .child(
                        div()
                            .flex_none()
                            .h(px(lh))
                            .flex()
                            .items_center()
                            .child(marker),
                    )
                    .child(div().flex_1().min_w_0().child(el)),
            )
            .child(self.children_of(b, env, cx))
            .into_any_element()
    }
}

impl R<'_> {
fn code_block_feishu(&self, b: &Block, lang: &str, wrap: bool, cx: &mut Context<Editor>) -> AnyElement {
        use crate::feishu_theme::{self as theme, Theme};

        let id = b.id;
        let g = SharedString::from(format!("code-{}", id.0));
        let spec = TextSpec {
            size: 14.,
            line_height: 22.,
            weight: FontWeight::NORMAL,
            color: Theme::text_code(),
            mono: true,
            strike_all: false,
            wrap,
            letter_spacing: 0.,
            dim_marks: false,
            link_underline: false,
            doc_font: true,
            font: None,
            italic: false,
        };
        // Line numbers follow the wrapped rows of the last layout.
        let rows_per_line: Vec<usize> = {
            let hard = b.text.text.split('\n').count();
            let mut v = vec![1; hard];
            if wrap && let Some(rows) = self.ed.code_rows.get(&id) {
                for (n, r) in v.iter_mut().zip(rows) {
                    *n = (*r).max(1);
                }
            }
            v
        };
        let text = b.text.text.clone();
        let mut el = self.text_el(id, &b.text, spec, Align::Left, None);
        if lang != "Plain Text" {
            let lang = lang.to_string();
            el.highlight = Some(Box::new(move |t| crate::highlight::spans(&lang, t)));
        }
        // Header: fold triangle, caption at x 29, then the toolbar (22 tall,
        // right-aligned), revealed on hover / while editing.
        let editing = self.focused && self.ed.core.sel.head.block == id;
        let folded = self.ed.code_folded.contains(&id);
        // Handle hovered / its menu open (`.hovered`): light blue ring
        // over a clear body so the block tint shows; block-selected: blue
        // border on the gray body.
        let menu_on = matches!(&self.ed.menu, Some(Menu::Block { block, .. }) | Some(Menu::Add { block, .. }) if *block == id);
        let selected = !menu_on && self.selected.contains(&id);
        let header_btn = |id_s: &'static str| {
            div()
                .id((id_s, id.0))
                .relative()
                .h(px(22.))
                .pl(px(2.))
                .pr(px(4.))
                .flex()
                .items_center()
                .gap(px(4.))
                .rounded(px(4.))
                .text_size(px(14.))
                .line_height(px(22.))
                .text_color(Theme::text_caption())
                .cursor_pointer()
                .hover(|s| s.bg(Theme::fill_hover_strong()))
                .active(|s| s.bg(crate::theme::adapt(rgba(0x1f232933).into())))
        };
        // 1px divider centred in the 17px gap after a toolbar item.
        let sep = |right: f32| div().absolute().top(px(3.)).right(px(right)).w(px(1.)).h(px(16.)).bg(Theme::line_divider());
        let lang_btn = if folded {
            // Folded: the language is a plain label.
            div()
                .relative()
                .h(px(22.))
                .px(px(4.))
                .flex()
                .items_center()
                .text_size(px(14.))
                .text_color(Theme::text_caption())
                .child(SharedString::from(lang.to_string()))
                .child(sep(-8.))
                .into_any_element()
        } else {
            header_btn("code-lang")
                .px(px(0.))
                .gap(px(0.))
                .text_color(Theme::text())
                .child(div().px(px(4.)).child(SharedString::from(lang.to_string())))
                .child(icon("caret-down", 14., Theme::text_placeholder()))
                .child(sep(-8.))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |e, _, window, cx| {
                        cx.stop_propagation();
                        e.open_lang_menu(id, window, cx);
                    }),
                )
                .into_any_element()
        };
        let caption = self.ed.caption_field(id).map(|f| {
            div()
                .id(("code-caption", id.0))
                .flex_1()
                .min_w(px(50.))
                .ml(px(28.))
                .mr(px(16.))
                .h(px(30.))
                .flex()
                .child(f)
                // The field takes the click; keep the page from moving the caret.
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|e, _, _, cx| {
                        cx.stop_propagation();
                        e.core.clear_block_sel();
                        e.menu = None;
                        cx.notify();
                    }),
                )
        });
        let fold_btn = div()
            .id(("code-fold", id.0))
            .absolute()
            .left(px(3.))
            .top(px(7.))
            .size(px(24.))
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .when(!folded, |d| d.opacity(0.).group_hover(g.clone(), |s| s.opacity(1.)))
            .child(
                div()
                    .size(px(16.))
                    .rounded(px(4.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .hover(|s| s.bg(Theme::fill_hover()))
                    .child(icon(if folded { "code-fold-right" } else { "code-fold-down" }, 12., Theme::text_caption())),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |e, _, _, cx| {
                    cx.stop_propagation();
                    if !e.code_folded.remove(&id) {
                        e.code_folded.insert(id);
                        // Leave the hidden code (it opens again if the caret comes back).
                        if e.core.sel.head.block == id || e.core.sel.anchor.block == id {
                            e.core.select_block(id);
                        }
                    }
                    cx.notify();
                }),
            );
        let border = if menu_on {
            Theme::code_hover_ring()
        } else if selected {
            Theme::primary_500()
        } else {
            Theme::line_border()
        };
        div()
            .relative()
            .group(g.clone())
            .rounded(px(8.))
            .border_1()
            .border_color(border)
            .when(!menu_on, |d| d.bg(Theme::bg_overlay()))
            .when(menu_on, |d| {
                d.child(outer_ring(Theme::code_hover_ring()))
            })
            // Pointer over the block: the 1px primary outline of the resizable box.
            // (A group can't group-hover itself; its own hover is the same thing.)
            .when(!folded && !menu_on && !selected, |d| d.hover(|s| s.border_color(Theme::primary_500())))
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .right_0()
                    .h(px(32.))
                    .pt(px(8.))
                    .pr(px(8.))
                    .flex()
                    .items_center()
                    .font_family(theme::UI_FONT)
                    .children(caption)
                    .child(
                        div()
                            .flex()
                            .flex_none()
                            .items_center()
                            .gap(px(17.))
                            .when(!editing, |d| d.opacity(0.).group_hover(g.clone(), |s| s.opacity(1.)))
                            .child(lang_btn)
                            .when(!folded, |d| {
                                d.child(
                                    header_btn("code-wrap")
                                        .child(icon("wrap-text", 16., Theme::text_caption()))
                                        .child(if wrap { crate::settings::text("取消换行") } else { crate::settings::text("自动换行") })
                                        .child(sep(-9.))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |e, _, window, cx| {
                                                cx.stop_propagation();
                                                e.core.checkpoint(core::EditKind::Other);
                                                if let Some(b) = e.core.block_mut(id)
                                                    && let BlockKind::Code { wrap, .. } = &mut b.kind
                                                {
                                                    *wrap = !*wrap;
                                                }
                                                e.changed(window, cx);
                                            }),
                                        ),
                                )
                            })
                            .child(
                                header_btn("code-copy")
                                    .child(icon("copy", 16., Theme::text_caption()))
                                    .child(crate::settings::tr("复制", "Copy"))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |_, _, _, cx| {
                                            cx.stop_propagation();
                                            cx.write_to_clipboard(ClipboardItem::new_string(text.clone()));
                                        }),
                                    ),
                            ),
                    ),
            )
            .child(fold_btn)
            .when(folded, |d| d.child(div().h(px(38.))))
            .when(!folded, |d| {
                d.child(
                    div()
                        .pt(px(40.))
                        .pb(px(26.))
                        .pr(px(20.))
                        .flex()
                        .child(
                            // Number box spans x 0..36 of the outer block, text starts at 60.
                            div()
                                .flex_none()
                                .w(px(35.))
                                .mr(px(24.))
                                .flex()
                                .flex_col()
                                .items_end()
                                .font_family(theme::MONO_FONT)
                                .text_size(px(14.))
                                .line_height(px(22.))
                                .text_color(Theme::text_caption())
                                .children(
                                    rows_per_line
                                        .iter()
                                        .enumerate()
                                        .map(|(i, n)| div().h(px(22. * *n as f32)).child(format!("{}", i + 1))),
                                ),
                        )
                        .child(div().flex_1().min_w_0().overflow_hidden().child(el)),
                )
            })
            .into_any_element()
    }
}

impl R<'_> {
fn children_of_feishu(&self, b: &Block, env: Env, cx: &mut Context<Editor>) -> Div {

        if b.children.is_empty() || b.folded || self.flat == Some(b.id) {
            return div();
        }
        div().pl(px(25.)).mt(px(8.)).child(self.blocks(&b.children, Env { depth: env.depth + 1, width: env.width - 25., ..env }, cx))
    }
}
