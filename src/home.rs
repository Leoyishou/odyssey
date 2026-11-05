//! 主页: the local library's home — the 新建 / 上传
//! cards (a new document, or 导入为文档 at once: no menus), the 最近访问 and 收藏 lists (list or grid view), 显示设置 for the
//! list columns and the sort dropdown. Dressed as 所有文档: a
//! hairline list with quiet hover fills, white cards and pills, native menus.

use crate::app::{RowMenu, Workspace};
#[path = "home_preview_style.rs"]
mod bs;
#[path = "home_text.rs"]
mod text_preview;
#[path = "home_scrollbar.rs"]
mod scrollbar;
use crate::settings::display_title;
use crate::model::BlockKind;
use crate::search::doc_icon;
use crate::store::Entry;
use crate::theme::{self, Theme};
use crate::trash::{empty_state, list_header_text, list_line, menu_label, page_header, row_hover};
use crate::ui::{ROW_RADIUS, TipExt, icon, menu_glyph, menu_row, popover, ui_text};
use gpui::{prelude::FluentBuilder, *};
use std::cell::RefCell;
use std::collections::{HashMap};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HomeTab {
    #[default]
    Recent,
    Starred,
}

/// The optional list columns (标题 is always shown).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Col {
    Modified,
    Created,
    Visited,
    Starred,
}

impl Col {
    fn label(self) -> &'static str {
        match self {
            Col::Modified => "修改时间",
            Col::Created => "创建时间",
            Col::Visited => "最近访问",
            Col::Starred => "收藏时间",
        }
    }
}

/// 最近访问's columns (no 所有者); 显示设置 toggles them.
const RECENT_COLS: [Col; 3] = [Col::Modified, Col::Created, Col::Visited];
const STARRED_COLS: [Col; 4] = [Col::Modified, Col::Created, Col::Visited, Col::Starred];

/// Bounded document trees retain formatting without retaining a whole long document.
type EquationNumbers = HashMap<crate::model::BlockId, u32>;
struct ThumbContent { blocks: Vec<crate::model::Block>, eqnos: EquationNumbers, cover: Option<crate::model::Cover> }
type Thumb = Rc<ThumbContent>;

fn thumb_blocks(blocks: &[crate::model::Block], budget: &mut usize, depth: usize, numbers: &EquationNumbers, eqnos: &mut EquationNumbers) -> Vec<crate::model::Block> {
    if depth > 8 { return vec![]; }
    let mut out = vec![];
    let mut sequence = 0;
    for b in blocks {
        if *budget == 0 { break; }
        *budget -= 1;
        let end = b.text.text.char_indices().nth(600).map_or(b.text.text.len(), |(i, _)| i);
        // A table owns its cell trees inside the kind, so bound those before cloning.
        let kind = match &b.kind {
            BlockKind::Table(t) => {
                let nr = t.rows.len().min(6); let nc = t.col_widths.len().min(6);
                let rows = t.rows.iter().take(nr).enumerate().map(|(r, row)| row.iter().take(nc).enumerate().map(|(c, cell)| {
                    crate::model::TableCell { blocks: thumb_blocks(&cell.blocks, budget, depth + 1, numbers, eqnos), bg: cell.bg.clone(), valign: cell.valign.clone(),
                        colspan: cell.colspan.min((nc - c) as u32), rowspan: cell.rowspan.min((nr - r) as u32) }
                }).collect()).collect();
                BlockKind::Table(crate::model::Table { col_widths: t.col_widths.iter().take(nc).copied().collect(), header_rows: t.header_rows.min(nr), header_col: t.header_col, rows })
            }
            BlockKind::Image { src, width, height, .. } => {
                let mut kind = b.kind.clone();
                // Resolve missing dimensions once per cached thumbnail, not each frame.
                let dimensions = crate::store::image_path(src).and_then(|p| image::ImageReader::open(p).ok())
                    .and_then(|r| r.with_guessed_format().ok()).and_then(|r| r.into_dimensions().ok());
                let (iw, ih) = dimensions.map(|(w, h)| (w.max(1) as f32, h.max(1) as f32)).unwrap_or((690., 460.));
                let w = width.unwrap_or_else(|| height.map(|h| h * iw / ih).unwrap_or(iw)).max(1.);
                let h = height.unwrap_or(w * ih / iw).max(1.);
                if let BlockKind::Image { width, height, .. } = &mut kind { *width = Some(w); *height = Some(h); }
                kind
            }
            BlockKind::Raw { tag, .. } => BlockKind::Raw { tag: tag.clone(), xml: String::new() },
            kind => kind.clone(),
        };
        let text = b.text.slice(0..end);
        let mut preview = crate::model::RichText::default();
        for (range, marks) in text.spans() {
            let mut marks = marks.clone();
            if marks.inline.as_ref().is_some_and(|i| i.equation().is_some()) {
                preview.push(&text.text[range], marks);
                continue;
            }
            let label = marks.inline.as_ref().map(|inline| match &inline.kind {
                crate::model::InlineKind::Equation(_) => unreachable!(),
                crate::model::InlineKind::File(file) => file.name.clone(),
                crate::model::InlineKind::Button(button) => button.text.clone(),
                crate::model::InlineKind::Reminder(reminder) => {
                    if crate::settings::english() {
                        let (y, m, d, h, mi) = crate::time::civil(reminder.expire.max(0) as u64 / 1000);
                        if reminder.whole_day { format!("{y}-{m:02}-{d:02}") } else { format!("{y}-{m:02}-{d:02} {h:02}:{mi:02}") }
                    } else { reminder.label(" ") }
                },
                crate::model::InlineKind::DocLink(doc) => display_title(&doc.title).to_string(),
                crate::model::InlineKind::Other { tag } => tag.clone(),
            }).map(|label| label.chars().take(160).collect::<String>());
            if let Some(inline) = &marks.inline { match &inline.kind {
                crate::model::InlineKind::Equation(_) => {},
                crate::model::InlineKind::Button(_) => marks.bold = true,
                crate::model::InlineKind::DocLink(_) | crate::model::InlineKind::File(_) => marks.link = Some(String::new()),
                _ => {},
            } }
            marks.inline = None;
            preview.push(&text.text[range].replace('\u{fffc}', label.as_deref().unwrap_or("")), marks);
        }
        let mut copy = crate::model::Block::new(kind, preview);
        if let Some(n) = numbers.get(&b.id) { eqnos.insert(copy.id, *n); }
        if let BlockKind::Ordered { seq } = &mut copy.kind { sequence = seq.unwrap_or(sequence + 1); *seq = Some(sequence); } else { sequence = 0; }
        copy.bg = b.bg.clone(); copy.align = b.align;
        copy.children = thumb_blocks(&b.children, budget, depth + 1, numbers, eqnos);
        out.push(copy);
    }
    out
}

fn thumb_block(b: &crate::model::Block, depth: usize, inherited: Hsla, quote: bool, eqnos: &EquationNumbers) -> AnyElement {
    thumb_block_in(b, depth, inherited, quote, false, 0, eqnos)
}

fn thumb_block_in(b: &crate::model::Block, depth: usize, inherited: Hsla, quote: bool, cell: bool, list_depth: usize, eqnos: &EquationNumbers) -> AnyElement {
    let s = bs::SCALE;
    let metrics = bs::typography(&b.kind);
    let (font_size, line_height) = if cell && !matches!(b.kind, BlockKind::Heading { .. } | BlockKind::Code { .. }) { (14., 22.) } else { (metrics.size, metrics.line_height) };
    let done = matches!(b.kind, BlockKind::Todo { done: true });
    let color = if done { Theme::todo_done() } else { inherited };
    let mut node = div().w_full().min_w_0().flex_none().flex().flex_col()
        .text_size(px(font_size * s)).line_height(px(line_height * s)).text_color(color)
        .when(b.kind.has_text(), |d| d.min_h(px(line_height * s)));
    if let Some(bg) = b.bg.as_ref().and_then(|c| theme::block_bg(&c.0)) { node = node.rounded(px(6. * s)).bg(bs::display_fill(bg)); }
    match &b.kind {
        BlockKind::Image { src, width, height, caption, .. } => {
            let path = crate::store::image_path(src);
            let ratio = width.unwrap_or(690.).max(1.) / height.unwrap_or(460.).max(1.);
            node = match b.align { crate::model::Align::Left => node.items_start(), crate::model::Align::Center => node.items_center(), crate::model::Align::Right => node.items_end() };
            // Reserve the proportional height at the available width before loading.
            let media = div().relative().flex_none().w_full().max_w_full().overflow_hidden().when_some(*width, |d, w| d.w(px(w * s)))
                .map(|mut d| { d.style().aspect_ratio = Some(ratio); d });
            node = node.child(if let Some(path) = path {
                media.child(img(path).absolute().inset_0().size_full().min_w_0().min_h_0().rounded(px(bs::RADIUS * s)).object_fit(ObjectFit::Contain)).into_any_element()
            } else {
                media.rounded(px(bs::RADIUS * s)).bg(bs::code_surface()).flex().items_center().justify_center().child(icon("image-broken", 24. * s, Theme::text_disabled())).into_any_element()
            });
            return node.when_some(caption.as_ref().filter(|c| !c.is_empty()), |d, caption| d.child(div().w_full().max_w_full().when_some(*width, |d, w| d.w(px(w * s))).mt(px(6. * s)).text_size(px(13. * s)).line_height(px(20. * s)).text_color(Theme::text_caption()).text_center().child(caption.clone()))).into_any_element();
        }
        BlockKind::Divider => return node.h(px(25. * s)).justify_center().child(div().h(px(s)).w_full().bg(Theme::line_divider())).into_any_element(),
        BlockKind::Grid => node = node.flex_row().gap(px(20. * s)),
        BlockKind::Column { .. } => node = node.flex_1(),
        BlockKind::Quote => node = node.relative().py(px(bs::QUOTE_PAD * s)).pl(px(bs::QUOTE_INSET * s))
            .child(div().absolute().left_0().top_0().bottom_0().w(px(bs::QUOTE_BAR * s)).rounded_full().bg(color)),
        BlockKind::Callout { emoji, bg, border, text_color } => {
            let tc = text_color.as_ref().and_then(|c| theme::text_color(&c.0)).map(bs::display_ink).unwrap_or(color);
            let mut body = div().flex_1().min_w_0().flex().flex_col();
            if depth < 8 { for (i, child) in b.children.iter().enumerate() {
                let gap = if i == 0 { 0. } else { bs::gap(&b.children[i - 1].kind).1.max(bs::gap(&child.kind).0) };
                body = body.child(div().mt(px(gap * s)).child(thumb_block_in(child, depth + 1, tc, quote, cell, list_depth, eqnos)));
            } }
            let first_line = b.children.first().map(|b| bs::typography(&b.kind).line_height).unwrap_or(theme::BODY_LINE);
            return node.relative().p(px(bs::INSET * s)).rounded(px(bs::CALLOUT_RADIUS * s)).border(px(s)).border_color(bs::border(border.as_ref().map(|c| c.0.as_str()))).bg(bs::fill(bg.as_ref().map(|c| c.0.as_str())))
                .child(div().flex().gap(px(bs::ICON_GAP * s)).child(div().size(px(bs::ICON * s)).mt(px((first_line - bs::ICON) / 2. * s)).text_size(px(20. * s)).line_height(px(22. * s)).flex_none().child(emoji.clone())).child(body)).into_any_element();
        }
        BlockKind::Code { lang, caption, wrap } => {
            node = node.rounded(px(bs::RADIUS * s)).border(px(s)).border_color(Theme::line_border()).bg(bs::code_surface()).font_family(theme::MONO_FONT)
                .child(div().h(px(bs::CODE_HEADER * s)).pl(px(28. * s)).pr(px(8. * s)).rounded_t(px((bs::RADIUS - 1.) * s)).bg(bs::code_header()).flex().items_center().font_family(theme::UI_FONT).text_size(px(13. * s)).text_color(Theme::text_caption()).child(caption.clone().unwrap_or_else(|| crate::settings::text("代码块").into())));
            let spans = crate::highlight::spans(lang, &b.text.text);
            let mut body = div().pt(px(14. * s)).pb(px(18. * s)).pr(px(20. * s)).flex().flex_col();
            let mut off = 0;
            for (i, line) in b.text.text.split('\n').enumerate() {
                let end = off + line.len();
                let mut cuts = vec![off, end];
                for (r, _, _) in &spans {
                    if r.start > off && r.start < end { cuts.push(r.start); }
                    if r.end > off && r.end < end { cuts.push(r.end); }
                }
                cuts.sort_unstable(); cuts.dedup();
                let runs = cuts.windows(2).map(|r| {
                    let h = spans.iter().find(|(h, _, _)| h.start <= r[0] && r[1] <= h.end);
                    let mut f = font(theme::MONO_FONT); if h.is_some_and(|(_, _, it)| *it) { f.style = FontStyle::Italic; }
                    TextRun { len: r[1] - r[0], font: f, color: h.map_or(Theme::text_code(), |(_, c, _)| *c), background_color: None, underline: None, strikethrough: None }
                }).collect::<Vec<_>>();
                body = body.child(div().flex().items_start()
                    .child(div().w(px(bs::CODE_GUTTER * s)).mr(px(bs::CODE_GAP * s)).flex_none().text_right().text_color(Theme::text_caption()).child((i + 1).to_string()))
                    .child(div().min_w_0().flex_1().overflow_hidden().when(!*wrap, |d| d.whitespace_nowrap()).child(if line.is_empty() { StyledText::new(" ") } else { StyledText::new(line.to_string()).with_runs(runs) })));
                off = end + 1;
            }
            return node.child(body).into_any_element();
        }
        BlockKind::Table(table) => {
            let widths: Vec<f32> = table.col_widths.iter().map(|w| w * s).collect();
            let mut table_el = div().grid().grid_col_widths(widths.clone()).w(px(widths.iter().sum())).max_w_full();
            for (r, c, value) in table.cells() {
                let (rs, cs) = (value.rowspan as usize, value.colspan as usize);
                let header = r < table.header_rows || (table.header_col && c == 0);
                let mut col = div().min_w_0().flex().flex_col().row_start(r as i16 + 1).row_end((r + rs) as i16 + 1).col_start(c as i16 + 1).col_end((c + cs) as i16 + 1)
                    .p(px(8. * s)).border_r(px(s)).border_b(px(s)).when(r == 0, |d| d.border_t(px(s))).when(c == 0, |d| d.border_l(px(s))).border_color(bs::table_line())
                    .when(header, |d| d.bg(Theme::table_header())).when_some(value.bg.as_ref().and_then(|c| theme::block_bg(&c.0)), |d, c| d.bg(bs::display_fill(c)))
                    .map(|d| match value.valign.as_deref() { Some("middle") => d.justify_center(), Some("bottom") => d.justify_end(), _ => d });
                for (i, child) in value.blocks.iter().enumerate() {
                    let gap = if i == 0 { 0. } else { bs::gap(&value.blocks[i - 1].kind).1.min(6.).max(bs::gap(&child.kind).0.min(8.)) };
                    col = col.child(div().mt(px(gap * s)).child(thumb_block_in(child, depth + 1, color, quote, true, 0, eqnos)));
                }
                table_el = table_el.child(col);
            }
            return node.child(table_el).into_any_element();
        }
        BlockKind::File(f) => return node.p(px(12. * s)).rounded(px(bs::RADIUS * s)).border(px(s)).border_color(Theme::line_border()).bg(bs::code_surface()).flex_row().items_center().gap(px(12. * s))
            .child(icon("file-text", 30. * s, Theme::text_caption())).child(div().flex_1().min_w_0().text_size(px(15. * s)).font_weight(FontWeight::MEDIUM).child(f.name.clone())).into_any_element(),
        BlockKind::Countdown(c) => {
            let left = c.remaining(crate::time::now_ms());
            let color = u32::from_str_radix(c.color.trim_start_matches('#'), 16).map(theme::c).unwrap_or(Theme::primary_500());
            let mut ink = bs::display_ink(color); ink.l = ink.l.min(0.38);
            let mut row = div().flex().justify_center().gap(px(16. * s));
            for (i, value) in [left / 86400, left / 3600 % 24, left / 60 % 60, left % 60].into_iter().enumerate() {
                row = row.child(div().flex().flex_col().items_center().child(div().size(px(52. * s)).rounded(px(8. * s)).bg(bs::highlight(color)).flex().items_center().justify_center().font_family(theme::MONO_FONT).font_weight(FontWeight::MEDIUM).text_size(px(26. * s)).text_color(ink).child(format!("{value:02}")))
                    .child(div().mt(px(4. * s)).text_size(px(14. * s)).line_height(px(20. * s)).text_color(Theme::text_caption()).child([crate::settings::text("天"), crate::settings::text("时"), crate::settings::text("分"), crate::settings::text("秒")][i])));
            }
            return node.py(px(18. * s)).rounded(px(bs::RADIUS * s)).border(px(s)).border_color(Theme::line_border()).bg(bs::code_surface()).child(row).into_any_element();
        }
        BlockKind::SubPageList => {
            node = node.p(px(8. * s)).rounded(px(bs::RADIUS * s)).border(px(s)).border_color(Theme::line_border()).bg(Theme::bg_overlay());
            if b.children.is_empty() { return node.h(px(96. * s)).justify_center().text_center().text_color(Theme::text_caption()).child(crate::settings::text("目录下暂无子页面")).into_any_element(); }
            for child in &b.children { node = node.child(div().py(px(9. * s)).px(px(6. * s)).flex().items_start().gap(px(6. * s)).child(icon("doc", 16. * s, Theme::text_caption())).child(div().flex_1().min_w_0().font_weight(FontWeight::MEDIUM).child(child.text.text.clone()))); }
            return node.into_any_element();
        }
        BlockKind::Raw { tag, .. } => return node.p(px(8. * s)).rounded(px(8. * s)).bg(Theme::bg_overlay()).text_color(Theme::text_caption()).child(crate::localized_format!("<{tag}> 已保留的内容", "<{tag}> Preserved content")).into_any_element(),
        _ => {}
    }
    if b.kind.has_text() {
        let runs = b.text.runs.iter().map(|r| {
            let mut f = font(if r.marks.code { theme::MONO_FONT } else { theme::UI_FONT });
            f.weight = if r.marks.bold { FontWeight::BOLD } else { bs::weight(&b.kind) };
            if r.marks.italic || quote { f.style = FontStyle::Italic; }
            let mut ink = if r.marks.link.is_some() { Theme::link() } else { r.marks.text_color.as_ref().and_then(|c| theme::text_color(&c.0)).map(bs::display_ink).unwrap_or(color) };
            if done { ink.a = 0.5; }
            let mark = r.marks.bg_color.as_ref().and_then(|c| theme::bg_color(&c.0)).map(bs::highlight);
            TextRun { len: r.len, font: f, color: if mark.is_some() { crate::theme::on_mark(ink) } else { ink },
                background_color: mark.or_else(|| r.marks.code.then(bs::code_surface)),
                underline: (r.marks.underline || r.marks.link.is_some()).then_some(UnderlineStyle { thickness: px(s), color: None, wavy: false }),
                strikethrough: (r.marks.strike || done).then_some(StrikethroughStyle { thickness: px(s), color: None }) }
        }).collect::<Vec<_>>();
        let marker = match b.kind {
            BlockKind::Bullet => Some(div().w(px(24. * s)).h(px(line_height * s)).pl(px(4. * s)).flex_none().flex().items_center().child(match list_depth % 3 {
                0 => div().size(px(5.5 * s)).rounded_full().bg(Theme::text_caption()),
                1 => div().size(px(5.5 * s)).rounded_full().border(px(s)).border_color(Theme::text_caption()),
                _ => div().size(px(5. * s)).bg(Theme::text_caption()),
            }).into_any_element()),
            BlockKind::Ordered { seq } => Some(div().min_w(px(18. * s)).mr(px(6. * s)).flex_none().text_color(Theme::text_caption()).child(crate::editor::core::ordered_marker(seq.unwrap_or(1), list_depth)).into_any_element()),
            BlockKind::Todo { done } => Some(div().w(px(24. * s)).h(px(line_height * s)).flex_none().flex().items_center().child(div().size(px(16. * s)).rounded(px(4. * s)).border(px(s)).border_color(if done { Theme::primary_500() } else { Theme::text_disabled() }).bg(if done { Theme::primary_500() } else { Theme::bg_body() }).flex().items_center().justify_center().when(done, |d| d.child(icon("todo-check", 12. * s, gpui::white())))).into_any_element()),
            _ => None,
        };
        let text = if b.text.runs.iter().any(|r| r.marks.inline.as_ref().is_some_and(|i| i.equation().is_some())) {
            text_preview::PreviewText { text: b.text.clone(), runs, size: font_size * s, line_height: line_height * s, align: b.align, eq_counter: eqnos.get(&b.id).copied().unwrap_or(0) }.into_any_element()
        } else { StyledText::new(b.text.text.clone()).with_runs(runs).into_any_element() };
        node = node.child(div().flex().items_start().children(marker).child(div().min_w_0().flex_1().map(|d| match b.align { crate::model::Align::Center => d.text_center(), crate::model::Align::Right => d.text_right(), _ => d }).child(text)));
    }
    if depth < 8 { for (i, child) in b.children.iter().enumerate() {
        let nested = b.kind.nests();
        let gap = if i == 0 { if nested { 8. } else { 0. } } else { bs::gap(&b.children[i - 1].kind).1.max(bs::gap(&child.kind).0) };
        node = node.child(div().min_w_0().when(!matches!(b.kind, BlockKind::Grid), |d| d.mt(px(gap * s)))
            .when(nested, |d| d.pl(px(25. * s)))
            .when(matches!(b.kind, BlockKind::Grid), |d| d.flex().w_0().map(|mut d| { d.style().flex_grow = Some(match child.kind { BlockKind::Column { ratio } => ratio, _ => 1. }); d }))
            .child(thumb_block_in(child, depth + 1, color, quote || matches!(b.kind, BlockKind::Quote), cell, list_depth + usize::from(nested), eqnos)));
    } }
    node.into_any_element()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum HomeSort { Visited, Modified, Created, Title }
impl HomeSort {
    fn key(self) -> &'static str { match self { Self::Visited => "visited", Self::Modified => "modified", Self::Created => "created", Self::Title => "title" } }
    fn of(key: &str) -> Self { match key { "modified" => Self::Modified, "created" => Self::Created, "title" => Self::Title, _ => Self::Visited } }
}
impl HomeSort {
    fn label(self) -> &'static str {
        match self {
            Self::Visited => "最近访问时间",
            Self::Modified => "最近修改时间",
            Self::Created => "创建时间（最新优先）",
            Self::Title => "标题（升序）",
        }
    }
}

pub struct Home {
    pub tab: HomeTab,
    pub grid: bool,
    grid_scroll: UniformListScrollHandle,
    list_scroll: ScrollHandle,
    grid_scrollbar: Rc<scrollbar::Scrollbar>,
    list_scrollbar: Rc<scrollbar::Scrollbar>,
    sort: HomeSort,
    pub sort_menu: bool,
    thumbs: RefCell<HashMap<String, (u64, Thumb)>>,
    /// Last-frame bounds of the controls the popovers hang from.
    anchors: Rc<RefCell<HashMap<&'static str, Bounds<Pixels>>>>,
}

impl Default for Home {
    fn default() -> Self {
        Home {
            tab: HomeTab::Recent,
            grid: true,
            grid_scroll: Default::default(),
            list_scroll: Default::default(),
            grid_scrollbar: Default::default(),
            list_scrollbar: Default::default(),
            sort: HomeSort::Visited,
            sort_menu: false,
            thumbs: Default::default(),
            anchors: Default::default(),
        }
    }
}

impl Home {
    fn anchor(&self, key: &str) -> Option<Bounds<Pixels>> {
        self.anchors.borrow().get(key).copied()
    }

    /// Records the element's bounds under `key` each frame.
    fn track(&self, key: &'static str) -> impl IntoElement {
        let rec = self.anchors.clone();
        canvas(move |b, _, _| {
            rec.borrow_mut().insert(key, b);
        }, |_, _, _, _| {})
        .absolute()
        .inset_0()
    }

    fn cols(&self) -> Vec<Col> {
        match self.tab {
            HomeTab::Recent => RECENT_COLS.to_vec(),
            HomeTab::Starred => STARRED_COLS.to_vec(),
        }
    }

    fn sort_col(&self) -> Col {
        match self.sort {
            HomeSort::Visited => Col::Visited,
            HomeSort::Modified => Col::Modified,
            HomeSort::Created => Col::Created,
            HomeSort::Title => Col::Visited,
        }
    }
}

fn visited(e: &Entry) -> u64 {
    if e.meta.opened > 0 { e.meta.opened } else { e.meta.modified }
}

fn cell_text(e: &Entry, c: Col) -> String {
    match c {
        Col::Modified => crate::ui::format_visit(e.meta.modified),
        Col::Created => crate::ui::format_visit(e.meta.created),
        Col::Visited => crate::ui::format_visit(visited(e)),
        Col::Starred => crate::ui::format_visit(e.meta.starred),
    }
}

/// The title column grows three times as fast as the others.
fn weight3(mut d: Div) -> Div {
    d.style().flex_grow = Some(3.);
    d
}

/// A document's icon: its emoji, or the monochrome page glyph.
fn entry_icon(e: &Entry) -> AnyElement {
    match &e.icon {
        Some(em) => div().size(px(24.)).mr(px(12.)).flex_none().flex().items_center().justify_center().text_size(px(18.)).child(em.clone()).into_any_element(),
        None => doc_icon().mr(px(12.)).into_any_element(),
    }
}

impl Workspace {
    /// In 批量管理 a click checks or unchecks; otherwise it opens.
    fn home_click(&mut self, id: &str, window: &mut Window, cx: &mut Context<Self>) {
        if self.batch.is_none() { return self.open(id, window, cx) }
        if !self.toggle_check(id) {
            self.show_toast_kind(crate::ui::ToastKind::Error, &crate::localized_format!("一次最多选择 {} 项", "Select up to {} items at a time", crate::sidebar::BATCH_LIMIT), cx);
        }
        cx.notify();
    }

    /// The row's / card's checkbox while 批量管理 is on.
    fn home_check(&self, e: &Entry) -> Option<Div> {
        use crate::sidebar::{BATCH_LIMIT, Check, checkbox};
        let sel = self.batch.as_ref()?;
        let on = sel.contains(&e.id);
        Some(checkbox(if on { Check::On } else { Check::Off }, !on && sel.len() >= BATCH_LIMIT, "home-card"))
    }

    fn home_entries(&self) -> Vec<Entry> {
        let mut v: Vec<Entry> = match self.home.tab {
            HomeTab::Recent => self.entries.clone(),
            HomeTab::Starred => self.entries.iter().filter(|e| e.meta.starred > 0).cloned().collect(),
        };
        v.sort_by(|a, b| {
            let order = match self.home.sort {
                HomeSort::Visited => visited(b).cmp(&visited(a)),
                HomeSort::Modified => b.meta.modified.cmp(&a.meta.modified),
                HomeSort::Created => b.meta.created.cmp(&a.meta.created),
                HomeSort::Title => a.title.cmp(&b.title),
            };
            order.then_with(|| a.id.cmp(&b.id))
        });
        v
    }

    fn thumb(&self, e: &Entry) -> Thumb {
        use std::hash::{Hash, Hasher};
        let children: Vec<&Entry> = self.entries.iter().filter(|p| p.meta.parent.as_deref() == Some(e.id.as_str())).collect();
        let mut stamp = rustc_hash::FxHasher::default();
        e.meta.modified.hash(&mut stamp);
        crate::settings::feishu().hash(&mut stamp);
        for child in &children { (&child.id, &child.title, &child.icon).hash(&mut stamp); }
        let stamp = stamp.finish();
        if let Some((m, t)) = self.home.thumbs.borrow().get(&e.id)
            && *m == stamp { return t.clone(); }
        // The derived rows exist only in this preview tree and never enter the editor/store.
        fn subpages(blocks: &mut [crate::model::Block], entries: &[&Entry], budget: &mut usize) {
            for b in blocks {
                if matches!(b.kind, BlockKind::SubPageList) {
                    b.children = entries.iter().take(*budget).map(|e| crate::model::Block::text(&format!("{}{}", e.icon.as_ref().map(|i| format!("{i} ")).unwrap_or_default(), display_title(&e.title)))).collect();
                    *budget -= b.children.len();
                } else {
                    subpages(&mut b.children, entries, budget);
                    if let BlockKind::Table(t) = &mut b.kind { for row in &mut t.rows { for cell in row { subpages(&mut cell.blocks, entries, budget); } } }
                }
            }
        }
        let mut budget = 64;
        let mut eqnos = HashMap::new();
        let mut cover = None;
        let mut blocks = self.store.load(&e.id).map(|doc| {
            cover = doc.cover.clone();
            let numbers = crate::editor::equation_numbers(&doc);
            thumb_blocks(&doc.blocks, &mut budget, 0, &numbers, &mut eqnos)
        }).unwrap_or_default();
        subpages(&mut blocks, &children, &mut budget);
        let t: Thumb = Rc::new(ThumbContent { blocks, eqnos, cover });
        self.home.thumbs.borrow_mut().insert(e.id.clone(), (stamp, t.clone()));
        t
    }

    fn open_row_menu(&mut self, id: &str, key: &str, cx: &mut Context<Self>) {
        let b = self.more_bounds.borrow().get(key).copied();
        let Some(b) = b else { return };
        self.row_menu = Some(RowMenu { id: id.to_string(), pos: point(b.right(), b.bottom() + px(2.)), corner: Corner::TopRight, sub: false, batch: false });
        cx.notify();
    }

    fn home_more(&self, e: &Entry, key: String, cx: &mut Context<Self>) -> Stateful<Div> {
        let id = e.id.clone();
        let (rec, k2) = (self.more_bounds.clone(), key.clone());
        let open = self.row_menu.as_ref().is_some_and(|m| m.id == e.id && m.pos.x > px(0.) && m.corner == Corner::TopRight);
        div()
            .id(SharedString::from(key.clone()))
            .relative()
            .size(px(24.))
            .flex_none()
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(6.))
            .cursor_pointer()
            .hover(|s| s.bg(Theme::fill_hover_strong()))
            .when(open, |d| d.bg(Theme::fill_hover_strong()))
            .child(icon("ellipsis", 16., Theme::text_caption()))
            .child(canvas(move |b, _, _| {
                rec.borrow_mut().insert(key.clone(), b);
            }, |_, _, _, _| {}).absolute().inset_0())
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_click(cx.listener(move |w, _, _, cx| {
                cx.stop_propagation();
                w.open_row_menu(&id, &k2, cx);
            }))
    }

    /// 主页's view and sort order are remembered (`settings.json`).
    pub(crate) fn load_home(&mut self) {
        let s = self.store.settings();
        self.home.grid = !s.home_list;
        self.home.sort = s.home_sort.as_deref().map(HomeSort::of).unwrap_or(HomeSort::Visited);
    }

    fn save_home(&mut self) {
        let mut s = self.store.settings();
        (s.home_list, s.home_sort) = (!self.home.grid, Some(self.home.sort.key().to_string()));
        if let Err(e) = self.store.save_settings(&s) { eprintln!("无法保存主页视图：{e}"); }
    }

    pub(crate) fn render_home(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        // Too narrow for the search box: a search icon beside +, in the same style.
        let search = self.header_search(window, cx);
        let narrow = search.is_none();
        let header = page_header(self, crate::settings::text("主页"), 26., cx).relative().children(search)
            .when(narrow, |d| d.child(crate::ui::header_icon("home-search", "search").mr(px(4.))
                .tip(crate::settings::tr("搜索文档", "Search documents"), Some(crate::shortcuts::hint("workspace::FocusSearch", "⌘ + J")))
                .on_click(cx.listener(|w, _, window, cx| w.open_search(window, cx)))))
            .child(
            crate::ui::header_icon("home-new", "plus")
                .tip(crate::settings::tr("新建文档", "New document"), Some(crate::shortcuts::hint("workspace::NewDoc", "⌘ + N")))
                .on_click(cx.listener(|w, _, window, cx| w.new_doc(window, cx))).into_any_element()
        );
        // Tabs as segmented items: the current one on a quiet fill, no accent.
        let tab = |id: &'static str, label: &'static str, t: HomeTab| {
            let on = self.home.tab == t;
            div()
                .id(id)
                .mr(px(8.))
                .h(px(28.))
                .px(px(10.))
                .flex()
                .items_center()
                .rounded(px(9.))
                .cursor_pointer()
                .text_size(px(13.))
                .line_height(px(20.))
                .when(on, |d| d.bg(Theme::fill_hover_strong()).text_color(Theme::text()))
                .when(!on, |d| d.text_color(Theme::text_caption()).hover(|s| s.bg(Theme::fill_hover())))
                .child(crate::settings::label(label))
                .on_click(cx.listener(move |w, _, _, cx| {
                    w.home.tab = t;
                    w.home.sort_menu = false;
                    cx.notify();
                }))
        };
        let toggle = |id: &'static str, ic: &'static str, on: bool, tip: &'static str, grid: bool| {
            div()
                .id(id)
                .w(px(30.))
                .h(px(24.))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(7.))
                .cursor_pointer()
                .when(on, |d| d.bg(Theme::fill_hover_strong()))
                .when(!on, |d| d.tip(tip, None).hover(|s| s.bg(Theme::fill_hover())))
                .child(icon(ic, 14., if on { Theme::text_code() } else { Theme::text_caption() }))
                .on_click(cx.listener(move |w, _, _, cx| {
                    w.home.grid = grid;
                    w.save_home();
                    w.home.sort_menu = false;
                    cx.notify();
                }))
        };
        // 批量管理: the sidebar's 批量操作 selection, entered and shown here
        // too, with a toolbar of its own.
        let text_btn = |id: &'static str, ic: &'static str, label: &'static str, enabled: bool| {
            div().id(id).flex_none().h(px(28.)).px(px(10.)).flex().items_center().gap(px(6.)).rounded(px(9.))
                .text_size(px(13.)).line_height(px(20.))
                .text_color(if enabled { Theme::text() } else { Theme::text_disabled() })
                .when(enabled, |d| d.cursor_pointer().hover(|s| s.bg(Theme::fill_hover())))
                .child(icon(ic, 14., if enabled { Theme::text_code() } else { Theme::text_disabled() }))
                .child(label)
        };
        let toolbar = div()
            .mt(px(12.))
            .h(px(28.))
            .flex_none()
            .pl(px(24.))
            .pr(px(24.))
            .flex()
            .items_center();
        let toolbar = if let Some(sel) = &self.batch {
            let ids: Vec<String> = self.home_entries().into_iter().take(crate::sidebar::BATCH_LIMIT).map(|e| e.id).collect();
            let n = sel.len();
            let all = n > 0 && ids.iter().all(|id| sel.contains(id));
            use crate::sidebar::{Check, checkbox};
            toolbar
                .child(div().id("home-batch-all").flex_none().ml(px(10.)).size(px(14.)).cursor_pointer().group("home-batch-all")
                    .child(checkbox(if all { Check::On } else if n > 0 { Check::Partial } else { Check::Off }, false, "home-batch-all"))
                    .tip(crate::settings::text("全选"), None)
                    .on_click(cx.listener(move |w, _, _, cx| { w.batch = Some(if all { vec![] } else { ids.clone() }); cx.notify(); })))
                .child(div().ml(px(10.)).text_size(px(13.)).line_height(px(20.)).text_color(Theme::text()).child(crate::localized_format!("已选 {n} 项", "Selected: {n}")))
                .child(div().flex_1())
                // The sidebar's 批量操作 actions: 创建副本 | 移动到 | 删除.
                .child(text_btn("home-batch-copy", "copy", crate::settings::text("创建副本"), n > 0)
                    .when(n > 0, |d| d.on_click(cx.listener(|w, _, _, cx| w.batch_copy(cx)))))
                .child(text_btn("home-batch-move", "folder-input", crate::settings::text("移动到"), n > 0).ml(px(4.))
                    .when(n > 0, |d| d.on_click(cx.listener(|w, _, window, cx| w.batch_move(window, cx)))))
                .child(text_btn("home-batch-delete", "bin", crate::settings::text("删除"), n > 0).ml(px(4.))
                    .when(n > 0, |d| d.on_click(cx.listener(|w, _, _, cx| w.ask_batch_delete(cx)))))
                .child(text_btn("home-batch-exit", "x", crate::settings::text("退出"), true).ml(px(8.))
                    .tip(crate::settings::text("退出批量操作"), None)
                    .on_click(cx.listener(|w, _, _, cx| { w.batch = None; w.row_menu = None; cx.notify(); })))
        } else { toolbar
            .child(tab("home-tab-recent", crate::settings::text("我的文档"), HomeTab::Recent))
            .child(tab("home-tab-star", crate::settings::text("收藏"), HomeTab::Starred))
            .child(div().flex_1())
            .child(text_btn("home-batch", "batch", crate::settings::text("批量管理"), true).mr(px(8.))
                .on_click(cx.listener(|w, _, _, cx| { w.batch = Some(vec![]); w.row_menu = None; w.home.sort_menu = false; cx.notify(); })))
            .child(
                div().id("home-sort").relative().h(px(28.)).px(px(10.)).flex().items_center().gap(px(6.))
                    .rounded(px(9.)).text_size(px(13.)).text_color(Theme::text_caption()).cursor_pointer()
                    .hover(|s| s.bg(Theme::fill_hover())).when(self.home.sort_menu, |d| d.bg(Theme::fill_hover_strong()))
                    .child(icon("sort-down", 14., Theme::text_caption())).child(crate::settings::label(self.home.sort.label()))
                    .child(self.home.track("home-sort"))
                    .on_click(cx.listener(|w, _, _, cx| { w.home.sort_menu = !w.home.sort_menu; cx.notify(); })),
            )
            .child(
                // The view switcher: one white pill, the current view on a quiet fill.
                crate::ui::pill()
                    .ml(px(16.))
                    .p(px(1.))
                    .flex()
                    .gap(px(2.))
                    .rounded(px(9.))
                    .child(toggle("home-grid", "view-grid", self.home.grid, crate::settings::text("切换为宫格视图"), true))
                    .child(toggle("home-list", "view-list", !self.home.grid, crate::settings::text("切换为列表视图"), false)),
            ) };

        let entries = self.home_entries();
        let body = if entries.is_empty() && (self.home.grid || self.home.tab == HomeTab::Recent) {
            empty_state("illus/empty-list.svg", crate::settings::text("当前列表暂无内容")).into_any_element()
        } else if self.home.grid {
            self.render_home_grid(&entries, window, cx)
        } else {
            self.render_home_list(&entries, cx)
        };
        div()
            .flex_1()
            .min_w_0()
            .h_full()
            .flex()
            .flex_col()
            .child(header)
            .child(toolbar)
            .child(body)
            .into_any_element()
    }

    fn render_home_list(&self, entries: &[Entry], cx: &mut Context<Self>) -> AnyElement {
        let cols = self.home.cols();
        let sort = self.home.sort_col();
        // The title is sized at about three times a column.
        let title_cell = || div().flex_grow().flex_shrink().flex_basis(relative(0.)).min_w(px(200.));
        let col_cell = || div().flex_grow().flex_shrink().flex_basis(relative(0.)).max_w(px(240.)).min_w(px(120.));
        // The list: 13pt tertiary headers over a hairline; the
        // sorted column in the text colour, semibold, with its arrow.
        let mut header = div()
            .h(px(33.))
            .mt(px(10.))
            .flex_none()
            .flex()
            .items_center()
            .border_b_1()
            .border_color(list_line())
            .child(
                weight3(title_cell()).pl(px(20.)).child(
                    div().id("home-col-title").px(px(4.)).text_size(px(13.)).line_height(px(24.)).text_color(list_header_text()).child(crate::settings::text("标题")),
                ),
            );
        for c in &cols {
            let sorted = self.home.sort != HomeSort::Title && *c == sort;
            header = header.child(
                col_cell().pl(px(20.)).child(
                    div()
                        .id(SharedString::from(format!("home-col-{}", c.label())))
                        .relative()
                        .px(px(4.))
                        .flex()
                        .items_center()
                        .gap(px(4.))
                        .rounded(px(ROW_RADIUS))
                        .text_size(px(13.))
                        .line_height(px(24.))
                        .text_color(if sorted { Theme::text() } else { list_header_text() })
                        .when(sorted, |d| d.font_weight(FontWeight::SEMIBOLD))
                        .hover(|s| s.bg(Theme::fill_hover()))
                        .when(sorted && self.home.sort_menu, |d| d.bg(Theme::fill_hover_strong()))
                        .child(crate::settings::label(c.label()))
                        .when(sorted, |d| {
                            d.child(icon("sort-down", 12., Theme::text()))
                                .cursor_pointer()
                                .on_click(cx.listener(|w, _, _, cx| {
                                    w.home.sort_menu = !w.home.sort_menu;
                                    cx.notify();
                                }))
                        }),
                ),
            );
        }
        header = header.child(div().w(px(60.)).flex_none());
        let mut rows = div().id("home-rows").flex_1().min_h_0().overflow_y_scroll().track_scroll(&self.home.list_scroll).px(px(24.)).pb(px(24.));
        if entries.is_empty() {
            rows = rows.child(div().pt(px(72.)).child(empty_state("illus/empty-list.svg", crate::settings::text("当前列表暂无内容"))));
        }
        for (i, e) in entries.iter().enumerate() {
            let (id, id2) = (e.id.clone(), e.id.clone());
            let menu_on = self.row_menu.as_ref().is_some_and(|m| m.id == e.id);
            let check = self.home_check(e);
            let mut row = div()
                .id(SharedString::from(format!("home-row-{}", e.id)))
                .h(px(56.))
                .flex()
                .items_center()
                .when(i > 0, |d| d.border_t_1().border_color(list_line()))
                .cursor_pointer()
                .hover(row_hover)
                .when(menu_on, |d| d.bg(Theme::fill_hover()).border_color(transparent_black()).rounded(px(ROW_RADIUS)))
                .on_click(cx.listener(move |w, _, window, cx| w.home_click(&id, window, cx)))
                .when(check.is_none(), |d| d.on_mouse_down(
                    MouseButton::Right,
                    cx.listener(move |w, ev: &MouseDownEvent, _, cx| {
                        w.row_menu = Some(RowMenu { id: id2.clone(), pos: ev.position, corner: Corner::TopLeft, sub: false, batch: false });
                        cx.notify();
                    }),
                ))
                .child(
                    weight3(title_cell())
                        .pl(px(24.))
                        .pr(px(16.))
                        .flex()
                        .items_center()
                        .when_some(check, |d, c| d.child(c.mr(px(12.))))
                        .child(entry_icon(e))
                        .child(div().min_w_0().text_size(px(15.)).line_height(px(22.)).overflow_hidden().whitespace_nowrap().text_ellipsis().child(ui_text(display_title(&e.title))))
                        .when(e.meta.pinned, |d| d.child(icon("pin-filled", 14., Theme::pinned()).ml(px(6.)))),
                );
            for c in &cols {
                row = row.child(col_cell().pl(px(24.)).text_size(px(13.)).text_color(Theme::text_caption()).whitespace_nowrap().overflow_hidden().child(cell_text(e, *c)));
            }
            row = row.child(div().w(px(60.)).flex_none().flex().justify_end().pr(px(12.)).when(self.batch.is_none(), |d| d.child(self.home_more(e, format!("home-more-{}", e.id), cx))));
            rows = rows.child(row);
        }
        div().flex_1().min_h_0().flex().flex_col().child(header.mx(px(24.)))
            .child(div().relative().flex_1().min_h_0().flex().flex_col().child(rows).child(self.home.list_scrollbar.render(&self.home.list_scroll))).into_any_element()
    }

    fn render_home_grid(&self, entries: &[Entry], window: &Window, cx: &mut Context<Self>) -> AnyElement {
        let main_w = f32::from(window.viewport_size().width) - if self.sidebar_open { self.sidebar_w(window) } else { 0. };
        let avail = (main_w - 48.).max(1.);
        let wide_cols = (((avail + 16.) / (200. + 16.)).floor() as usize).max(1);
        let n = if wide_cols >= 4 { wide_cols } else { (((avail + 16.) / (160. + 16.)).floor() as usize).clamp(1, 3) };
        let card_h = ((avail - 16. * (n - 1) as f32) / n as f32) * (264. / 236.);
        let entries = entries.to_vec();
        let rows = entries.len().div_ceil(n);
        let grid = uniform_list("home-grid-area", rows, cx.processor(move |workspace, range: std::ops::Range<usize>, _, cx| {
            range.map(|row| {
                let start = row * n;
                let end = (start + n).min(entries.len());
                div().w_full().h(px(card_h + 16.)).px(px(24.)).pb(px(16.))
                    .grid().grid_cols(n as u16).gap(px(16.))
                    .children(entries[start..end].iter().map(|e| workspace.render_home_card(e, card_h, cx)))
            }).collect()
        }))
        // The last row's own 16 plus 16 more: 32 below the cards, like the
        // list view's 24 and clear of the window's rounded corner.
        .flex_1().min_h_0().pb(px(16.)).track_scroll(self.home.grid_scroll.clone());
        let scroll = self.home.grid_scroll.0.borrow().base_handle.clone();
        div().relative().flex_1().min_h_0().mt(px(10.)).flex().flex_col()
            .child(grid).child(self.home.grid_scrollbar.render(&scroll)).into_any_element()
    }

    fn render_home_card(&self, e: &Entry, card_h: f32, cx: &mut Context<Self>) -> AnyElement {
            let id = e.id.clone();
            let id2 = e.id.clone();
            let thumb = self.thumb(e);
            let page = if thumb.blocks.is_empty() && e.title.trim().is_empty() {
                div()
                    .size_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .child(img("illus/blank-doc.svg").size(px(64.)))
                    .child(div().mt(px(4.)).text_size(px(14.)).line_height(px(22.)).text_color(Theme::text_placeholder()).child(crate::settings::text("空白文档")))
            } else {
                let mut p = div().size_full().p(px(18.)).overflow_hidden().flex().flex_col();
                p = p.child(div().flex_none().flex().items_start().gap(px(6.)).mb(px(8.))
                    .when_some(e.icon.as_ref(), |d, emoji| d.child(div().flex_none().text_size(px(14.)).line_height(px(20.)).child(emoji.clone())))
                    .child(div().min_w_0().text_size(px(14.)).line_height(px(20.)).font_weight(FontWeight::BOLD).child(e.title.clone())));
                for (i, block) in thumb.blocks.iter().enumerate() {
                    let (mt, _) = bs::gap(&block.kind);
                    let prev = if i == 0 { 0. } else { bs::gap(&thumb.blocks[i - 1].kind).1 };
                    p = p.child(div().flex_none().mt(px(mt.max(prev) * bs::SCALE)).child(thumb_block(block, 0, Theme::text(), false, &thumb.eqnos)));
                }
                p
            };
            let cover = thumb.cover.as_ref().and_then(|c| crate::store::image_path(&c.src));
            let frame = 4.;
            let menu_on = self.row_menu.as_ref().is_some_and(|m| m.id == e.id);
            let check = self.home_check(e);
                div()
                    .id(SharedString::from(format!("home-card-{}", e.id)))
                    .w_full().min_w_0().h(px(card_h)).relative().group("home-card")
                    .flex_none()
                    .bg(Theme::bg_overlay()).p(px(frame))
                    .overflow_hidden()
                    .cursor_pointer()
                    .hover(|s| s.bg(Theme::fill_hover_strong()))
                    .when(menu_on, |d| d.bg(Theme::fill_hover_strong()))
                    .on_click(cx.listener(move |w, _, window, cx| w.home_click(&id, window, cx)))
                    .when(check.is_none(), |d| d.on_mouse_down(
                        MouseButton::Right,
                        cx.listener(move |w, ev: &MouseDownEvent, _, cx| {
                            w.row_menu = Some(RowMenu { id: id2.clone(), pos: ev.position, corner: Corner::TopLeft, sub: false, batch: false });
                            cx.notify();
                        }),
                    ))
                    .when_some(cover, |d, path| d.child(img(path).absolute().inset_0().size_full().object_fit(ObjectFit::Fill)))
                    .child(crate::ui::continuous_page(20. - frame).size_full().overflow_hidden().child(page.relative()))
                    .child(crate::ui::continuous_corner_mask(20., Theme::bg_body()))
                    .when_some(check, |d, c| d.child(div().absolute().top(px(12.)).left(px(12.)).child(c)))
                    .when(self.batch.is_none(), |d| d.child(
                        div().absolute().top(px(8.)).right(px(8.))
                            .when(!menu_on, |d| d.opacity(0.).group_hover("home-card", |s| s.opacity(1.)))
                            .child(self.home_more(e, format!("home-gmore-{}", e.id), cx).rounded_full().bg(Theme::bg_overlay())),
                    ))
            .into_any_element()
    }

    /// 显示设置 and the sort dropdown: the menu panel; the sort item is a
    /// native menu row.
    pub(crate) fn render_home_menus(&self, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let mut out = vec![];
        if self.view != crate::app::View::Home {
            return out;
        }
        let place = |pos: Point<Pixels>, corner: Corner, menu: Stateful<Div>| {
            deferred(anchored().anchor(corner).position(pos).snap_to_window_with_margin(px(8.)).child(menu)).with_priority(2).into_any_element()
        };
        if self.home.sort_menu
            && let Some(b) = self.home.anchor("home-sort") {
                let mut menu = popover().id("sort-menu").occlude().w(px(200.)).py(px(crate::ui::MENU_PAD_Y))
                    .on_mouse_down_out(cx.listener(|w, _, _, cx| { w.home.sort_menu = false; cx.notify(); }));
                for (index, order) in [HomeSort::Visited, HomeSort::Modified, HomeSort::Created, HomeSort::Title].into_iter().enumerate() {
                    let g = SharedString::from(format!("home-order-{index}"));
                    menu = menu.child(menu_row(g.clone(), g.clone())
                        .child(menu_label(order.label(), g.clone(), gpui::white()).flex_1())
                        .when(self.home.sort == order, |d| d.child(menu_glyph("check", 14., g)))
                        .on_click(cx.listener(move |w, _, _, cx| {
                            w.home.sort = order;
                            w.save_home();
                            w.home.sort_menu = false;
                            w.home.grid_scroll.scroll_to_item(0, ScrollStrategy::Top);
                            w.home.list_scroll.set_offset(point(px(0.), px(0.)));
                            cx.notify();
                        })));
                }
                out.push(place(point(b.right(), b.bottom() + px(4.)), Corner::TopRight, menu));
            }
        out
    }
}
