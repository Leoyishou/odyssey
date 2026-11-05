//! Custom GPUI element that lays out and paints one block's rich text, its
//! selection, caret and IME marked range. Layout results are written into a
//! shared cache that the editor uses for hit-testing and caret navigation.

use crate::editor::core::Pos;
use crate::editor::math;
use crate::model::{Align, BlockId, Marks, RichText};
use crate::editor_theme::{self as theme, Theme, craft};
use gpui::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Range;
use std::rc::Rc;

/// One visual (wrapped) row of a block.
#[derive(Clone, Debug)]
pub struct Row {
    /// Byte range in the block text.
    pub range: Range<usize>,
    /// Index of the hard line (split on `\n`).
    pub hard: usize,
    /// Top of the row relative to the element origin.
    pub y: Pixels,
    /// Row height: the line height, or more when an equation sticks out.
    pub h: Pixels,
    /// Offset of the text line box (`line_height` tall) inside the row.
    pub base_dy: Pixels,
    /// x of the row start inside the unwrapped hard-line layout.
    pub x0: Pixels,
    /// Horizontal offset from alignment.
    pub align_dx: Pixels,
}

pub struct TextLayout {
    pub bounds: Bounds<Pixels>,
    pub lines: Vec<(usize, WrappedLine)>,
    pub rows: Vec<Row>,
    pub line_height: Pixels,
    /// Laid out as code (monospace).
    pub mono: bool,
    /// Equations: byte offset and window bounds of the view (the clickable,
    /// highlighted box).
    pub eqs: Vec<(usize, Bounds<Pixels>)>,
    /// 按钮: byte offset and window bounds of the button box.
    pub buttons: Vec<(usize, Bounds<Pixels>)>,
}

impl TextLayout {
    /// Wrapped rows per hard line.
    pub fn rows_per_line(&self) -> Vec<usize> {
        let mut v = vec![0; self.rows.last().map_or(0, |r| r.hard + 1)];
        for r in &self.rows {
            v[r.hard] += 1;
        }
        v
    }

    fn row_for(&self, index: usize) -> usize {
        // Prefer the row that starts at `index` (caret after a wrap goes down).
        for (i, r) in self.rows.iter().enumerate() {
            let last_of_hard = self.rows.get(i + 1).is_none_or(|n| n.hard != r.hard);
            if index >= r.range.start && (index < r.range.end || (last_of_hard && index == r.range.end)) {
                return i;
            }
        }
        self.rows.len().saturating_sub(1)
    }

    /// Caret position (top-left of the caret) relative to window.
    pub fn position_for(&self, index: usize) -> Point<Pixels> {
        let ri = self.row_for(index);
        let Some(r) = self.rows.get(ri) else { return self.bounds.origin };
        let (start, line) = &self.lines[r.hard];
        let x = line.unwrapped_layout.x_for_index(index.saturating_sub(*start)) - r.x0 + r.align_dx;
        point(self.bounds.left() + x, self.bounds.top() + r.y + r.base_dy)
    }

    /// The equation whose view contains a window-space point.
    pub fn equation_at(&self, p: Point<Pixels>) -> Option<usize> {
        self.eqs.iter().find(|(_, b)| b.contains(&p)).map(|(ix, _)| *ix)
    }

    pub fn equation_bounds(&self, ix: usize) -> Option<Bounds<Pixels>> {
        self.eqs.iter().find(|(i, _)| *i == ix).map(|(_, b)| *b)
    }

    pub fn button_at(&self, p: Point<Pixels>) -> Option<usize> {
        self.buttons.iter().find(|(_, b)| b.contains(&p)).map(|(ix, _)| *ix)
    }

    pub fn button_bounds(&self, ix: usize) -> Option<Bounds<Pixels>> {
        self.buttons.iter().find(|(i, _)| *i == ix).map(|(_, b)| *b)
    }

    pub fn row_index_for(&self, index: usize) -> usize {
        self.row_for(index)
    }

    /// Closest text index for a window-space point.
    pub fn index_for(&self, p: Point<Pixels>) -> usize {
        if self.rows.is_empty() {
            return 0;
        }
        let local_y = p.y - self.bounds.top();
        let ri = if local_y < px(0.) {
            0
        } else {
            self.rows
                .iter()
                .position(|r| local_y < r.y + r.h)
                .unwrap_or(self.rows.len() - 1)
        };
        self.index_in_row(ri, p.x)
    }

    pub fn index_in_row(&self, ri: usize, window_x: Pixels) -> usize {
        let r = &self.rows[ri];
        let (start, line) = &self.lines[r.hard];
        let x = window_x - self.bounds.left() - r.align_dx + r.x0;
        let ix = start + line.unwrapped_layout.closest_index_for_x(x.max(px(0.)));
        let last_of_hard = self.rows.get(ri + 1).is_none_or(|n| n.hard != r.hard);
        let mut ix = ix.clamp(r.range.start, r.range.end);
        if !last_of_hard && ix == r.range.end && r.range.end > r.range.start {
            // The wrap boundary belongs to the next row; stay on this one.
            ix = prev_char(&line.text, ix - start) + start;
        }
        ix
    }

    /// Rectangles covering `range` (window coords), one per row.
    pub fn rects_for(&self, range: Range<usize>) -> Vec<Bounds<Pixels>> {
        let mut out = vec![];
        for (ri, r) in self.rows.iter().enumerate() {
            let a = range.start.max(r.range.start);
            let b = range.end.min(r.range.end);
            let last_of_hard = self.rows.get(ri + 1).is_none_or(|n| n.hard != r.hard);
            // An empty row fully inside the range still shows a sliver.
            if a > b || (a == b && !(range.start <= r.range.start && range.end > r.range.end)) {
                continue;
            }
            let (start, line) = &self.lines[r.hard];
            let xa = line.unwrapped_layout.x_for_index(a - start) - r.x0 + r.align_dx;
            let mut xb = line.unwrapped_layout.x_for_index(b - start) - r.x0 + r.align_dx;
            if range.end > r.range.end && last_of_hard {
                xb += px(5.);
            }
            out.push(Bounds::from_corners(
                point(self.bounds.left() + xa, self.bounds.top() + r.y + r.base_dy),
                point(self.bounds.left() + xb, self.bounds.top() + r.y + r.base_dy + self.line_height),
            ));
        }
        out
    }
}

fn prev_char(s: &str, i: usize) -> usize {
    s[..i].char_indices().next_back().map(|(k, _)| k).unwrap_or(0)
}

pub type LayoutCache = Rc<RefCell<HashMap<BlockId, TextLayout>>>;

#[derive(Clone)]
pub struct TextSpec {
    pub size: f32,
    pub line_height: f32,
    pub weight: FontWeight,
    pub color: Hsla,
    pub mono: bool,
    pub strike_all: bool,
    pub wrap: bool,
    /// CSS letter-spacing in px (headings use -0.02em).
    pub letter_spacing: f32,
    /// Text colours and highlights at .7 opacity (inside a quote:
    /// `.docx-quote_container-block .textHighlight { opacity: .7 }`).
    pub dim_marks: bool,
    /// Links are underlined while the caret is in their block.
    pub link_underline: bool,
    /// Document text: drawn in the theme's body font (fields keep the UI font).
    pub doc_font: bool,
    /// A family of its own (headings, the title), else the body's.
    pub font: Option<&'static str>,
    /// Everything italic (a theme's quotes).
    pub italic: bool,
}

impl TextSpec {
    /// The page body: 15pt, 1.25 × the natural line height.
    pub fn body() -> Self {
        TextSpec {
            size: theme::body_size(),
            line_height: theme::body_line(),
            weight: FontWeight::NORMAL,
            color: Theme::text(),
            mono: false,
            strike_all: false,
            wrap: true,
            letter_spacing: 0.,
            dim_marks: false,
            link_underline: false,
            doc_font: true,
            font: None,
            italic: false,
        }
    }
}

pub struct RichTextElement {
    pub id: BlockId,
    pub text: RichText,
    pub spec: TextSpec,
    pub align: Align,
    pub placeholder: Option<SharedString>,
    pub placeholder_color: Option<Hsla>,
    /// Horizontal nudge for the placeholder glyphs only (caret stays put).
    pub placeholder_dx: f32,
    pub selection: Option<Range<usize>>,
    pub caret: Option<usize>,
    pub marked: Option<Range<usize>>,
    pub cache: LayoutCache,
    pub show_caret: bool,
    pub highlight: Option<Box<dyn Fn(&str) -> Vec<crate::highlight::Span>>>,
    /// 查找和替换 hits as (range, is_current), tinted over the glyphs.
    pub found: Vec<(Range<usize>, bool)>,
    /// Equation numbers shown before this block (`katexEqnNo`).
    pub eq_counter: u32,
    /// The button whose 按钮设置 panel is open (`.editing`), shown with
    /// the panel's unsaved values.
    pub button_edit: Option<(usize, crate::model::InlineButton)>,
}

pub struct Prepared {
    lines: Vec<(usize, WrappedLine)>,
    wrap_width: Option<Pixels>,
    line_height: Pixels,
    /// (height, base_dy) of every row, in order.
    rows: Vec<(Pixels, Pixels)>,
    height: Pixels,
    eqs: Vec<Rc<EqGeom>>,
}

/// The equation box is `vertical-align: middle`: its centre sits this
/// far (in em of the paragraph font) above the baseline.
const EQ_MIDDLE: f32 = 0.375;

/// An equation's box inside its line. Coordinates are relative to the
/// box's left edge and vertical centre.
pub struct EqGeom {
    /// Byte offset of its U+FFFC.
    ix: usize,
    display: bool,
    /// Advance in the line (`.editor-kit-equation-block`, padding 0 1px).
    adv: f32,
    /// Layout height (a `transform: scale` shrink keeps it).
    box_h: f32,
    /// The view: x, y, w, h (radius 4, hover / selected backgrounds).
    view: [f32; 4],
    /// Empty or invalid: `.placeholder` background.
    placeholder: bool,
    /// Buttons sit relative to the top of the line box instead of the
    /// equation middle: the row extents they need there (top, bottom).
    ext: Option<[f32; 2]>,
    body: EqBody,
}

enum EqBody {
    /// Baseline origins of the formula and its `\tag`.
    Math { math: Rc<math::Math>, at: [f32; 2], tag_at: Option<[f32; 2]> },
    /// 添加 LaTeX 公式 / 无效公式 (with the warning icon's top-left).
    Text { line: ShapedLine, at: [f32; 2], icon: Option<[f32; 2]> },
    /// 按钮: `view` is the button box; the label is painted in a `bh` tall
    /// line box `pad` from its left edge. `wrap_h`: the wrapper's height
    /// (selected background).
    Button { line: ShapedLine, font: Font, color: String, disabled: bool, editing: bool, bh: f32, wrap_h: f32, pad: f32 },
    /// 文本视图 attachment: the type icon box (x, y, size) and the name
    /// painted from `text_x` in the row's line box.
    File { line: ShapedLine, font: Font, kind: crate::editor::file::Kind, icon: [f32; 3], text_x: f32 },
    /// A plain inline document link, with the surrounding text metrics.
    Doc { line: ShapedLine, font: Font, deleted: bool, wrap: Option<Pixels>, line_height: f32 },
    /// 日期提醒: the clock icon box (x, y, size), filled when it notifies,
    /// and the date text from `text_x`.
    Reminder { line: ShapedLine, icon: [f32; 3], icon_color: Hsla, filled: bool, text_x: f32 },
}

enum ObjSrc {
    Eq(String),
    Btn(crate::model::InlineButton, bool),
    File(crate::model::FileBlock),
    Rem(crate::model::InlineReminder),
    Doc(crate::model::InlineDocLink, Marks),
}

thread_local! {
    /// Live titles and icons of the library's documents, for @文档 links
    /// (set by the editor from the workspace).
    pub static DOC_TITLES: std::cell::RefCell<std::collections::HashMap<String, (String, Option<String>)>> = Default::default();
}

/// Document references use the same font and line box as surrounding text.
fn doc_geom(ix: usize, l: &crate::model::InlineDocLink, marks: &Marks, spec: &TextSpec, row_w: f32, window: &mut Window) -> EqGeom {
    let (fs, lh) = (spec.size, spec.line_height);
    let live = DOC_TITLES.with(|t| t.borrow().get(&l.id).cloned());
    let pending = live.as_ref().is_some_and(|(t, _)| t.starts_with("未创建："));
    let deleted = live.is_none();
    let title = live.map(|(t, _)| crate::settings::display_title(&t).to_string()).unwrap_or_else(|| crate::settings::text("此文档已被删除").to_string());
    let font = font_for(spec, marks);
    let mut name = l.alias.as_deref().unwrap_or(&title).to_string();
    if pending && l.alias.is_some() { name.push_str("（未创建）"); }
    let color = if deleted { Theme::text_disabled() } else { Theme::link() };
    let run = TextRun { len: name.len(), font: font.clone(), color, background_color: None, underline: None, strikethrough: None };
    let line = window.text_system().shape_line(name.into(), px(fs), std::slice::from_ref(&run), None);
    let w = f32::from(line.width).min(row_w.max(1.));
    let wrap = (f32::from(line.width) > w).then(|| px(w));
    let rows = window.text_system().shape_text(line.text.clone(), px(fs), &[run], wrap, None).unwrap_or_default();
    let height = rows.iter().map(|l| l.wrap_boundaries().len() + 1).sum::<usize>().max(1) as f32 * lh;
    EqGeom { ix, display: false, adv: w, box_h: height, view: [0., 0., w, height], placeholder: false, ext: Some([0., height]),
        body: EqBody::Doc { line, font, deleted, wrap, line_height: lh } }
}

/// `.docx-reminder-inline-component`: margin-left .25em, padding
/// .125em .25em .125em 1.125em with the .875em clock as its background
/// (at 0 56%), in the status colour.
fn reminder_geom(ix: usize, r: &crate::model::InlineReminder, spec: &TextSpec, window: &mut Window) -> EqGeom {
    let (fs, lh) = (spec.size, spec.line_height);
    let font = Font {
        family: theme::UI_FONT.into(),
        features: FontFeatures::default(),
        fallbacks: Some(FontFallbacks::from_fonts(vec!["PingFang SC".into(), "Apple Color Emoji".into()])),
        weight: spec.weight,
        style: FontStyle::Normal,
    };
    let now = crate::time::now_ms() as i64;
    let text = crate::editor::reminder::chip_text(r, now);
    let (color, icon_color) = crate::editor::reminder::colors(r.expire, now);
    let run = TextRun { len: text.len(), font, color, background_color: None, underline: None, strikethrough: None };
    let line = window.text_system().shape_line(text.into(), px(fs), &[run], None);
    let m = 0.25 * fs;
    let text_x = m + 1.125 * fs;
    let w = text_x + f32::from(line.width) + 0.25 * fs;
    // The padding box: the font's content area (≈1.4em) and .25em of padding.
    let bh = (1.65 * fs).min(lh);
    let top = (lh - bh) / 2.;
    let s = 0.875 * fs;
    EqGeom {
        ix,
        display: false,
        adv: w,
        box_h: lh,
        view: [m, top, w - m, bh],
        placeholder: false,
        ext: Some([0., lh]),
        body: EqBody::Reminder { line, icon: [m, top + (bh - s) * 0.56, s], icon_color, filled: r.should_notify, text_x },
    }
}

/// `.embed-inline-link.file-block`: the type icon (1.15em, raised 2px)
/// and the name in the link colour, padding 0 2px.
fn file_geom(ix: usize, f: &crate::model::FileBlock, spec: &TextSpec, row_w: f32, window: &mut Window) -> EqGeom {
    let (fs, lh) = (spec.size, spec.line_height);
    let font = Font {
        family: theme::UI_FONT.into(),
        features: FontFeatures::default(),
        fallbacks: Some(FontFallbacks::from_fonts(vec!["PingFang SC".into(), "Apple Color Emoji".into()])),
        weight: spec.weight,
        style: FontStyle::Normal,
    };
    let name: String = f.name.chars().take(200).collect();
    let run = TextRun { len: name.len(), font: font.clone(), color: Theme::link(), background_color: None, underline: None, strikethrough: None };
    let line = window.text_system().shape_line(name.into(), px(fs), &[run], None);
    let s = (fs * 1.15).round();
    let text_x = 2. + s + 4.;
    let w = (text_x + f32::from(line.width) + 2.).min(row_w.max(text_x + 2.));
    EqGeom {
        ix,
        display: false,
        adv: w,
        box_h: lh,
        view: [0., 0., w, lh],
        placeholder: false,
        ext: Some([0., lh]),
        body: EqBody::File { line, font, kind: crate::editor::file::kind_of(&f.name), icon: [2., (lh - s) / 2. - 1., s], text_x },
    }
}

/// `.docx-button-component__wrapper { margin: 2px 4px }` around
/// `.docx-button-component { padding: 0 8px }`.
fn button_geom(ix: usize, b: &crate::model::InlineButton, editing: bool, spec: &TextSpec, row_w: f32, window: &mut Window) -> EqGeom {
    let (fs, bh, weight, heading) = crate::editor::button::metrics(spec.size, spec.weight);
    let lh = spec.line_height;
    let wrap_lh = if heading { bh } else { lh };
    let font = Font {
        family: theme::UI_FONT.into(),
        features: FontFeatures::default(),
        fallbacks: Some(FontFallbacks::from_fonts(vec!["PingFang SC".into(), "Apple Color Emoji".into()])),
        weight,
        style: FontStyle::Normal,
    };
    let label: String = b.text.chars().take(200).collect();
    let pal = crate::editor::button::palette(&b.color);
    let run = TextRun { len: label.len(), font: font.clone(), color: pal.text, background_color: None, underline: None, strikethrough: None };
    let line = window.text_system().shape_line(label.into(), px(fs), &[run], None);
    let w = (f32::from(line.width) + 16.).min((row_w - 8.).max(16.));
    let box_top = (lh - wrap_lh) / 2.;
    let wrap_h = bh.max(wrap_lh);
    EqGeom {
        ix,
        display: false,
        adv: w + 8.,
        box_h: wrap_h + 4.,
        view: [4., box_top, w, bh],
        placeholder: false,
        ext: Some([(box_top - 2.).min(0.), (box_top + wrap_h + 2.).max(lh)]),
        body: EqBody::Button {
            line,
            font,
            color: b.color.clone(),
            disabled: b.action == crate::model::ButtonAction::FollowPage,
            editing,
            bh,
            wrap_h,
            pad: 8.,
        },
    }
}

fn eq_geom(ix: usize, src: &str, fs: f32, row_w: f32, color: Hsla, eqno: u32, window: &mut Window) -> EqGeom {
    let display = math::is_block(src);
    let parsed = if src.trim().is_empty() { None } else { Some(math::layout(src, display, math::FONT_PX, color, eqno, window)) };
    let Some(Some(m)) = parsed else {
        // `.equation-block-placeholder` / `-error-placeholder`: 14px/20px,
        // padding 3px 8px; the error text has margin-left 4 after the icon.
        let error = parsed.is_some();
        let (label, tc) = if error { (crate::settings::text("无效公式"), Theme::text()) } else { (crate::settings::text("添加 LaTeX 公式"), Theme::text_caption()) };
        let font = Font {
            family: theme::UI_FONT.into(),
            features: FontFeatures::default(),
            fallbacks: Some(FontFallbacks::from_fonts(vec!["PingFang SC".into()])),
            weight: FontWeight::NORMAL,
            style: FontStyle::Normal,
        };
        let run = TextRun { len: label.len(), font, color: tc, background_color: None, underline: None, strikethrough: None };
        let line = window.text_system().shape_line(label.into(), px(14.), &[run], None);
        let (asc, desc) = (f32::from(line.ascent), f32::from(line.descent));
        let icon_w = if error { 14. + 4. } else { 0. };
        let view_w = 2. + 8. + icon_w + f32::from(line.width) + 8. + 2.;
        let top = -13.;
        let baseline = top + 3. + (20. - asc - desc) / 2. + asc;
        return EqGeom {
            ix,
            display,
            adv: (view_w + 2.).min(row_w),
            box_h: 26.,
            view: [1., top, view_w, 26.],
            placeholder: true,
            ext: None,
            body: EqBody::Text { line, at: [1. + 2. + 8. + icon_w, baseline], icon: error.then_some([1. + 2. + 8., top + 3. + 3.]) },
        };
    };
    if display {
        // `.katex-display { margin: 1em 0 }` at the view's 15px, content padding-bottom 3.
        let tag_w = m.tag.as_ref().map_or(0., |t| t.width);
        let r = row_w - 4.;
        let a = m.width + tag_w;
        let squeeze = m.tag.is_some() && r < a + tag_w;
        let d = (r / (a.max(m.width) + 1.)).min(1.);
        let view_h = 15. + m.ascent + m.depth + 15. + 3.;
        let top = -view_h / 2.;
        if d >= 1. && !squeeze {
            let base = top + 15. + m.ascent;
            return EqGeom {
                ix,
                display,
                adv: row_w,
                box_h: view_h,
                view: [1., top, row_w - 2., view_h],
                placeholder: false,
                ext: None,
                body: EqBody::Math {
                    at: [3. + (row_w - 6. - m.width) / 2., base],
                    tag_at: m.tag.as_ref().map(|t| [row_w - 3. - t.width, base]),
                    math: m,
                },
            };
        }
        // `.overflow-numbering`: the tag follows the formula and the view
        // shrinks to fit, scaled about its left centre.
        let sm = if d < 1. { math::layout(src, true, math::FONT_PX * d, color, eqno, window).unwrap_or(m.clone()) } else { m.clone() };
        let view_w = a + 4.;
        let base = (top + 15. + m.ascent) * d;
        return EqGeom {
            ix,
            display,
            adv: row_w,
            box_h: view_h,
            view: [1., top * d, view_w * d, view_h * d],
            placeholder: false,
            ext: None,
            body: EqBody::Math { at: [1. + 2. * d, base], tag_at: sm.tag.as_ref().map(|_| [1. + 2. * d + sm.width, base]), math: sm },
        };
    }
    // Inline: `.editor-kit-equation-view { padding: 0 2px; min-width: 16px }`;
    // the katex strut is at least 16.48 / 6.59 around the baseline, plus
    // the content's 3px bottom padding.
    let asc = m.ascent.max(16.48);
    let desc = m.depth.max(6.59);
    let view_w = (m.width + 4.).max(16.);
    let view_h = asc + desc + 3.;
    let top = -view_h / 2.;
    let d = ((row_w - 4.) / (m.width + 1.)).min(1.);
    let _ = fs;
    let (math, d) = if d < 1. {
        (math::layout(src, false, math::FONT_PX * d, color, eqno, window).unwrap_or(m), d)
    } else {
        (m, 1.)
    };
    EqGeom {
        ix,
        display,
        adv: (view_w + 2.).min(row_w),
        box_h: view_h,
        view: [1., top * d, view_w * d, view_h * d],
        placeholder: false,
        ext: None,
        body: EqBody::Math { at: [1. + 2. * d, (top + asc) * d], tag_at: None, math },
    }
}

fn font_for(spec: &TextSpec, m: &Marks) -> Font {
    let family: SharedString = if spec.mono || m.code { theme::mono_font().into() } else if let Some(f) = spec.font { f.into() } else if spec.doc_font { theme::body_font().into() } else { theme::UI_FONT.into() };
    Font {
        family,
        features: FontFeatures::default(),
        fallbacks: Some(FontFallbacks::from_fonts(vec!["PingFang SC".into(), "Apple Color Emoji".into()])),
        weight: if m.bold { bold_weight() } else { spec.weight },
        style: if m.italic || spec.italic { FontStyle::Italic } else { FontStyle::Normal },
    }
}

/// `strong { font-weight }` (700).
fn bold_weight() -> FontWeight {
    if crate::settings::feishu() { FontWeight::BOLD } else { FontWeight(theme::style().num("strong.weight")) }
}

fn span_color(spec: &TextSpec, m: &Marks) -> Hsla {
    if m.link.is_some() {
        return Theme::link();
    }
    let st = theme::style();
    let own = if crate::settings::feishu() || spec.mono { None } else if m.code { st.color("code.color") } else if m.bold { st.color("strong.color") } else { None };
    let c = match m.text_color.as_ref().and_then(|c| theme::text_color(&c.0)) {
        Some(c) if spec.dim_marks => c.opacity(0.7),
        Some(c) => c,
        None => own.unwrap_or(spec.color),
    };
    // Strikethrough: colour blended 35% toward the background.
    if m.strike && !crate::settings::feishu() { c.opacity(c.a * 0.65) } else { c }
}

use crate::editor_ui::{CJK_PUNCT_FONT, is_cjk_punct};

const QUOTE_FONT: &str = "Helvetica Neue";

/// Shaped in place of an equation's U+FFFC: same UTF-8 length, a glyph of
/// the text font (so the line's ascent is unchanged).
const EQ_STAND_IN: &str = "\u{2013}";

pub struct PlacedEq {
    /// Box left edge, vertical centre.
    origin: Point<Pixels>,
    view: Bounds<Pixels>,
    hitbox: Hitbox,
    geom: Rc<EqGeom>,
}

thread_local! {
    /// The equation under a pressed mouse button (`:active`).
    static EQ_PRESSED: std::cell::Cell<Option<(BlockId, usize)>> = const { std::cell::Cell::new(None) };
}

fn is_quote(ch: char) -> bool {
    matches!(ch, '\u{2018}' | '\u{2019}' | '\u{201C}' | '\u{201D}')
}

pub fn build_runs(
    text: &RichText,
    spec: &TextSpec,
    marked: Option<&Range<usize>>,
    highlight: Option<&Vec<crate::highlight::Span>>,
) -> Vec<TextRun> {
    let mut runs = vec![];
    for (range, m) in text.spans() {
        // Split by the marked (IME) range and syntax highlight boundaries.
        let mut cuts = vec![range.start, range.end];
        if let Some(mr) = marked {
            cuts.push(mr.start.clamp(range.start, range.end));
            cuts.push(mr.end.clamp(range.start, range.end));
        }
        if let Some(h) = highlight {
            for (r, _, _) in h {
                cuts.push(r.start.clamp(range.start, range.end));
                cuts.push(r.end.clamp(range.start, range.end));
            }
        }
        // Curly quotes render in a Latin-width face (QUOTE_FONT) rather
        // than the body face's full-width ones.
        for (i, ch) in text.text[range.clone()].char_indices() {
            if is_quote(ch) || is_cjk_punct(ch) {
                cuts.push(range.start + i);
                cuts.push(range.start + i + ch.len_utf8());
            }
        }
        cuts.sort();
        cuts.dedup();
        for w in cuts.windows(2) {
            let (a, b) = (w[0], w[1]);
            if a >= b {
                continue;
            }
            let mut color = span_color(spec, m);
            let mut hl_italic = false;
            if let Some(h) = highlight
                && let Some((_, hc, it)) = h.iter().find(|(r, _, _)| r.start <= a && b <= r.end)
            {
                color = *hc;
                hl_italic = *it;
            }
            let in_marked = marked.is_some_and(|mr| mr.start <= a && b <= mr.end && mr.start < mr.end);
            // `a { text-decoration: underline }`: links stay underlined, not
            // only while being edited.
            let link_line = m.link.is_some() && !m.underline && !in_marked && !crate::settings::feishu() && theme::style().flag("a.underline");
            let underline = if link_line {
                Some(UnderlineStyle { thickness: px(1.), color: Some(theme::style().color("a.underline-color").unwrap_or(color)), wavy: false })
            } else if in_marked || m.underline || (spec.link_underline && m.link.is_some()) {
                Some(UnderlineStyle { thickness: px(1.), color: Some(color), wavy: false })
            } else {
                None
            };
            let mut font = font_for(spec, m);
            if !spec.mono && !m.code {
                if text.text[a..b].chars().all(is_quote) {
                    font.family = QUOTE_FONT.into();
                } else if text.text[a..b].chars().all(is_cjk_punct) {
                    font.family = CJK_PUNCT_FONT.into();
                }
            }
            if hl_italic {
                font.style = FontStyle::Italic;
            }
            if m.code && !spec.mono {
                font.weight = if m.bold { bold_weight() } else { FontWeight::NORMAL };
            }
            let background_color = m.bg_color.as_ref().and_then(|c| theme::bg_color(&c.0)).map(|c| if spec.dim_marks { c.opacity(0.7) } else { c });
            runs.push(TextRun {
                len: b - a,
                font,
                color: if background_color.is_some() { crate::theme::on_mark(color) } else { color },
                background_color,
                underline,
                // Painted by `RichTextElement::paint`, see `paint_strikes`.
                strikethrough: None,
            });
        }
    }
    runs
}

impl IntoElement for RichTextElement {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}

impl Element for RichTextElement {
    type RequestLayoutState = Rc<RefCell<Option<Prepared>>>;
    type PrepaintState = Vec<PlacedEq>;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        _cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let state: Rc<RefCell<Option<Prepared>>> = Rc::new(RefCell::new(None));
        let font_size = px(self.spec.size);
        let line_height = px(self.spec.line_height);
        let mut eq_src: Vec<(usize, ObjSrc)> = vec![];
        let (text, runs) = if self.text.is_empty() {
            match &self.placeholder {
                Some(ph) => {
                    let mut spec = self.spec.clone();
                    spec.color = self.placeholder_color.unwrap_or(Theme::block_placeholder());
                    let rt = RichText::plain(ph.to_string());
                    let runs = build_runs(&rt, &spec, None, None);
                    (ph.clone(), runs)
                }
                None => (SharedString::default(), vec![]),
            }
        } else {
            let hl = self.highlight.as_ref().map(|f| f(&self.text.text));
            let mut runs = build_runs(&self.text, &self.spec, self.marked.as_ref(), hl.as_ref());
            let mut text = self.text.text.clone();
            for (r, m) in self.text.spans() {
                let Some(obj) = m.inline.as_ref() else { continue };
                if let Some(src) = obj.equation() {
                    for (i, _) in self.text.text[r.clone()].char_indices() {
                        eq_src.push((r.start + i, ObjSrc::Eq(src.to_string())));
                    }
                } else if let Some(f) = obj.file() {
                    for (i, _) in self.text.text[r.clone()].char_indices() {
                        eq_src.push((r.start + i, ObjSrc::File(f.clone())));
                    }
                } else if let Some(l) = obj.doc_link() {
                    for (i, _) in self.text.text[r.clone()].char_indices() {
                        eq_src.push((r.start + i, ObjSrc::Doc(l.clone(), m.clone())));
                    }
                } else if let Some(&rm) = obj.reminder() {
                    for (i, _) in self.text.text[r.clone()].char_indices() {
                        eq_src.push((r.start + i, ObjSrc::Rem(rm)));
                    }
                } else if let Some(b) = obj.button() {
                    for (i, _) in self.text.text[r.clone()].char_indices() {
                        let ix = r.start + i;
                        match &self.button_edit {
                            Some((e, pb)) if *e == ix => eq_src.push((ix, ObjSrc::Btn(pb.clone(), true))),
                            _ => eq_src.push((ix, ObjSrc::Btn(b.clone(), false))),
                        }
                    }
                }
            }
            if !eq_src.is_empty() {
                // Shape each equation as an invisible stand-in of the same
                // UTF-8 length, widened to the equation's box afterwards.
                for (ix, _) in &eq_src {
                    text.replace_range(*ix..*ix + 3, EQ_STAND_IN);
                }
                let mut off = 0;
                for run in &mut runs {
                    if eq_src.iter().any(|(ix, _)| *ix >= off && *ix < off + run.len) {
                        run.color = transparent_black();
                        run.background_color = None;
                        run.underline = None;
                    }
                    off += run.len;
                }
            }
            (SharedString::from(text), runs)
        };
        let wrap = self.spec.wrap;
        let spacing = px(self.spec.letter_spacing);
        let fs = self.spec.size;
        let color = self.spec.color;
        let eq_counter = self.eq_counter;
        let spec = self.spec.clone();
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        let st = state.clone();
        let layout_id = window.request_measured_layout(style, move |known, available, window, _cx| {
            let wrap_width = if wrap {
                known.width.or(match available.width {
                    AvailableSpace::Definite(x) => Some(x),
                    _ => None,
                })
            } else {
                None
            };
            if let Some(p) = st.borrow().as_ref()
                && p.wrap_width == wrap_width
            {
                return size(known.width.unwrap_or(px(0.)), p.height);
            }
            let custom = spacing != px(0.) || !eq_src.is_empty();
            let mut lines = window
                .text_system()
                .shape_text(text.clone(), font_size, &runs, if custom { None } else { wrap_width }, None)
                .unwrap_or_default();
            let row_w = wrap_width.or(known.width).map_or(f32::INFINITY, f32::from);
            let mut eqno = eq_counter;
            let eqs: Vec<Rc<EqGeom>> = eq_src
                .iter()
                .map(|(ix, src)| match src {
                    ObjSrc::Eq(src) => {
                        let g = Rc::new(eq_geom(*ix, src, fs, row_w, color, eqno, window));
                        eqno += math::numbers_in(src);
                        g
                    }
                    ObjSrc::Btn(b, editing) => Rc::new(button_geom(*ix, b, *editing, &spec, row_w, window)),
                    ObjSrc::File(f) => Rc::new(file_geom(*ix, f, &spec, row_w, window)),
                    ObjSrc::Rem(r) => Rc::new(reminder_geom(*ix, r, &spec, window)),
                    ObjSrc::Doc(l, marks) => Rc::new(doc_geom(*ix, l, marks, &spec, row_w, window)),
                })
                .collect();
            let mut start = 0;
            let mut out = vec![];
            let mut rows = vec![];
            let mut h = px(0.);
            let mut w = px(0.);
            for mut l in lines.drain(..) {
                let len = l.len();
                let line_eqs: Vec<&EqGeom> = eqs.iter().map(|e| &**e).filter(|e| e.ix >= start && e.ix < start + len).collect();
                if custom {
                    adjust_line(&mut l, spacing, &line_eqs, start, wrap_width);
                }
                // Row heights: the line box grows to contain equation boxes.
                let layout = &l.unwrapped_layout;
                let base = (line_height - layout.ascent - layout.descent) / 2. + layout.ascent;
                let mut cuts: Vec<usize> = vec![0];
                for b in l.wrap_boundaries() {
                    cuts.push(layout.runs[b.run_ix].glyphs[b.glyph_ix].index);
                }
                cuts.push(len + 1);
                for c in cuts.windows(2) {
                    let (mut top, mut bottom) = (px(0.), line_height);
                    for e in line_eqs.iter().filter(|e| e.ix - start >= c[0] && e.ix - start < c[1]) {
                        if let Some([t, b]) = e.ext {
                            top = top.min(px(t));
                            bottom = bottom.max(px(b));
                            continue;
                        }
                        let mid = base - px(EQ_MIDDLE * fs);
                        top = top.min(mid - px(e.box_h / 2.));
                        bottom = bottom.max(mid + px(e.box_h / 2.));
                    }
                    rows.push((bottom - top, -top));
                    h += bottom - top;
                }
                w = w.max(l.width());
                out.push((start, l));
                start += len + 1;
            }
            if out.is_empty() {
                h = line_height;
            }
            *st.borrow_mut() = Some(Prepared { lines: out, wrap_width, line_height, rows, height: h, eqs });
            size(known.width.unwrap_or(w), h)
        });
        (layout_id, state)
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        state: &mut Self::RequestLayoutState,
        window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        let Some(p) = state.borrow_mut().take() else { return vec![] };
        let lh = p.line_height;
        let mut rows = vec![];
        let mut y = px(0.);
        let showing_placeholder = self.text.is_empty();
        let mut heights = p.rows.iter();
        for (hi, (start, line)) in p.lines.iter().enumerate() {
            let layout = &line.unwrapped_layout;
            let mut bounds_ix: Vec<(usize, Pixels)> = vec![(0, px(0.))];
            for b in line.wrap_boundaries() {
                let g = &layout.runs[b.run_ix].glyphs[b.glyph_ix];
                bounds_ix.push((g.index, g.position.x));
            }
            bounds_ix.push((line.len(), layout.width));
            for w in bounds_ix.windows(2) {
                let (a, xa) = w[0];
                let (b, xb) = w[1];
                let width = xb - xa;
                let align_dx = match self.align {
                    Align::Left => px(0.),
                    Align::Center => ((bounds.size.width - width) / 2.).max(px(0.)),
                    Align::Right => (bounds.size.width - width).max(px(0.)),
                };
                let (ra, rb) = if showing_placeholder { (0, 0) } else { (start + a, start + b) };
                let (h, base_dy) = heights.next().copied().unwrap_or((lh, px(0.)));
                rows.push(Row { range: ra..rb, hard: hi, y, h, base_dy, x0: xa, align_dx });
                y += h;
            }
        }
        let mut layout = TextLayout { bounds, lines: p.lines, rows, line_height: lh, mono: self.spec.mono, eqs: vec![], buttons: vec![] };
        let mut placed = vec![];
        for e in p.eqs {
            let ri = layout.row_for(e.ix);
            let Some(r) = layout.rows.get(ri) else { continue };
            let (start, line) = &layout.lines[r.hard];
            let l = &line.unwrapped_layout;
            let x = l.x_for_index(e.ix - start) - r.x0 + r.align_dx;
            let base = (lh - l.ascent - l.descent) / 2. + l.ascent;
            let line_top = bounds.top() + r.y + r.base_dy;
            let origin = point(bounds.left() + x, if e.ext.is_some() { line_top } else { line_top + base - px(EQ_MIDDLE * self.spec.size) });
            let [vx, vy, vw, vh] = e.view;
            let view = Bounds::new(point(origin.x + px(vx), origin.y + px(vy)), size(px(vw), px(vh)));
            if e.ext.is_some() {
                layout.buttons.push((e.ix, view));
            } else {
                layout.eqs.push((e.ix, view));
            }
            let hitbox = window.insert_hitbox(view, HitboxBehavior::Normal);
            placed.push(PlacedEq { origin, view, hitbox, geom: e });
        }
        self.cache.borrow_mut().insert(self.id, layout);
        placed
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _state: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let cache = self.cache.borrow();
        let Some(layout) = cache.get(&self.id) else { return };
        let lh = layout.line_height;
        let gpui_align = match self.align {
            Align::Left => TextAlign::Left,
            Align::Center => TextAlign::Center,
            Align::Right => TextAlign::Right,
        };
        // Inline code chips behind the text: a grey chip with a 1pt rim,
        // radius 3, 4.5pt beyond the glyphs.
        if !self.text.is_empty() && !self.spec.mono {
            for (range, m) in self.text.spans() {
                if !m.code {
                    continue;
                }
                for r in layout.rects_for(range) {
                    let inset = if crate::settings::feishu() { px(3.) } else { ((r.size.height - px(self.spec.size * 1.32)) / 2.).max(px(0.)) };
                    let (pad, radius) = if crate::settings::feishu() { (2., 4.) } else { (theme::style().num("code.pad"), theme::style().num("code.radius")) };
                    let r = Bounds::from_corners(point(r.left() - px(pad), r.top() + inset), point(r.right() + px(pad), r.bottom() - inset));
                    window.paint_quad(
                        quad(r, px(radius), if crate::settings::feishu() { Theme::bg_overlay() } else { Theme::code_chip() }, px(1.), if crate::settings::feishu() { Theme::line_border() } else { Theme::code_chip_border() }, BorderStyle::Solid),
                    );
                }
            }
        }
        if let Some(sel) = &self.selection {
            let rects = if self.text.is_empty() {
                vec![Bounds::new(bounds.origin, size(px(5.), lh))]
            } else {
                // Equations show their own selected background instead
                // (`.equation-leaf ::selection` is transparent).
                let mut parts = vec![];
                let mut a = sel.start;
                let mut objs: Vec<usize> = layout.eqs.iter().chain(&layout.buttons).map(|(ix, _)| *ix).collect();
                objs.sort();
                for ix in objs.iter().filter(|ix| sel.start <= **ix && **ix < sel.end) {
                    if a < *ix {
                        parts.push(a..*ix);
                    }
                    a = ix + 3;
                }
                if a < sel.end || parts.is_empty() && a == sel.start {
                    parts.push(a..sel.end);
                }
                parts.into_iter().flat_map(|r| layout.rects_for(r)).collect()
            };
            for r in rects {
                window.paint_quad(fill(r, Theme::selection()));
            }
        }
        for eq in prepaint.iter().filter(|e| e.geom.ext.is_none()) {
            let ix = eq.geom.ix;
            let selected = self.selection.as_ref().is_some_and(|s| s.start <= ix && ix + 3 <= s.end);
            let pressed = EQ_PRESSED.get() == Some((self.id, ix));
            let hovered = eq.hitbox.is_hovered(window);
            let ph = eq.geom.placeholder;
            let bg = if selected {
                Some(Theme::block_selected())
            } else if pressed {
                Some(if ph { theme::c(craft::N400) } else { Theme::fill_pressed() })
            } else if hovered {
                Some(if ph { theme::c(craft::N350) } else { Theme::fill_hover_strong() })
            } else if ph {
                Some(theme::c(craft::N100))
            } else {
                None
            };
            if let Some(bg) = bg {
                window.paint_quad(fill(eq.view, bg).corner_radii(px(4.)));
            }
        }
        let dx = if self.text.is_empty() { px(self.placeholder_dx) } else { px(0.) };
        for (hi, (_, line)) in layout.lines.iter().enumerate() {
            let tops: Vec<Pixels> = layout.rows.iter().filter(|r| r.hard == hi).map(|r| r.y + r.base_dy).collect();
            let o = point(bounds.left() + dx, bounds.top());
            let _ = line.paint_background_rows(o, lh, &tops, gpui_align, Some(bounds), window, cx);
            let _ = line.paint_rows(o, lh, &tops, gpui_align, Some(bounds), window, cx);
        }
        for eq in prepaint.iter() {
            let o = eq.origin;
            match &eq.geom.body {
                EqBody::Math { math, at, tag_at } => {
                    math.paint(point(o.x + px(at[0]), o.y + px(at[1])), window, cx);
                    if let (Some(tag), Some(t)) = (&math.tag, tag_at) {
                        tag.paint(point(o.x + px(t[0]), o.y + px(t[1])), window, cx);
                    }
                }
                EqBody::Button { .. } => self.paint_button(eq, window, cx),
                EqBody::File { .. } => self.paint_file(eq, window, cx),
                EqBody::Doc { .. } => self.paint_doc(eq, window, cx),
                EqBody::Reminder { .. } => self.paint_reminder(eq, window, cx),
                EqBody::Text { line, at, icon } => {
                    let top = o.y + px(at[1]) - line.ascent;
                    let _ = line.paint(point(o.x + px(at[0]), top), line.ascent + line.descent, window, cx);
                    if let Some(i) = icon {
                        let b = Bounds::new(point(o.x + px(i[0]), o.y + px(i[1])), size(px(14.), px(14.)));
                        let _ = window.paint_svg(
                            b,
                            "editor-icons/equation-error.svg".into(),
                            TransformationMatrix::unit(),
                            theme::c(0xff9200),
                            cx,
                        );
                    }
                }
            }
        }
        self.listen_equations(prepaint, window);
        if !self.text.is_empty() {
            self.paint_strikes(layout, window);
        }
        // Find marks sit over the text box of each hit.
        let box_h = (px(self.spec.size * 1.4)).min(lh);
        for (range, current) in &self.found {
            // Find hits in marker yellow; the current one in orange.
            let color = if *current { theme::ca(0xff9200, 0.45) } else { theme::ca(0xfff100, 0.35) };
            for r in layout.rects_for(range.clone()) {
                let top = r.top() + (lh - box_h) / 2.;
                window.paint_quad(fill(Bounds::new(point(r.left(), top), size(r.size.width, box_h)), color));
            }
        }
        if self.show_caret
            && let Some(c) = self.caret
        {
            let p = if self.text.is_empty() {
                let dx = match self.align {
                    Align::Left => px(0.),
                    Align::Center => bounds.size.width / 2.,
                    Align::Right => bounds.size.width - px(2.),
                };
                point(bounds.left() + dx, bounds.top())
            } else {
                layout.position_for(c)
            };
            // The caret: the accent, 2pt wide, the font's ascent + descent tall.
            let font_h = px(self.spec.size * 1.18);
            let top = p.y + (lh - font_h) / 2.;
            window.paint_quad(fill(Bounds::new(point(p.x, top), size(px(2.), font_h)), Theme::caret()).corner_radii(px(1.)));
        }
    }
}

impl RichTextElement {
    fn paint_button(&self, eq: &PlacedEq, window: &mut Window, cx: &mut App) {
        let EqBody::Button { line, font, color, disabled, editing, bh, wrap_h, pad } = &eq.geom.body else { return };
        let ix = eq.geom.ix;
        let v = eq.view;
        if self.selection.as_ref().is_some_and(|s| s.start <= ix && ix + 3 <= s.end) {
            // `.docx-button-component__wrapper--selected`, behind the button.
            window.paint_quad(fill(Bounds::new(v.origin, size(v.size.width, px(*wrap_h))), Theme::selection()));
        }
        let pal = crate::editor::button::palette(color);
        let pressed = EQ_PRESSED.get() == Some((self.id, ix));
        let hovered = eq.hitbox.is_hovered(window);
        let (bg, tc) = if *disabled {
            (if color.starts_with("deep") { pal.disabled_bg } else { pal.bg }, Theme::text_disabled())
        } else if pressed {
            (pal.active_bg, pal.active_text)
        } else if hovered {
            (pal.hover_bg, pal.hover_text)
        } else {
            (pal.bg, pal.text)
        };
        let lifted = !*disabled && (pressed || hovered || *editing);
        let shadows = if lifted { crate::editor::button::shadow_s4() } else { crate::editor::button::shadow_s1() };
        // Content stays crisp: buttons in the text round at 5.
        let radius = px(5.);
        window.paint_shadows(v, Corners::all(radius), &shadows);
        window.paint_quad(fill(v, bg).corner_radii(radius));
        // A done to-do: `.todo-line-through .docx-button-component` draws
        // a 1.6px `--icon-n2` line at 55% behind the label; `opacity: .7`.
        let dim = if self.spec.strike_all { 0.7 } else { 1. };
        if self.spec.strike_all {
            let y = v.top() + (v.size.height - px(1.6)) * 0.55;
            window.paint_quad(fill(Bounds::new(point(v.left(), y), size(v.size.width, px(1.6))), theme::c(craft::N600)));
        }
        let run = TextRun { len: line.text.len(), font: font.clone(), color: tc.opacity(dim), background_color: None, underline: None, strikethrough: None };
        let line = window.text_system().shape_line(line.text.clone(), line.font_size, &[run], None);
        let clip = Bounds::new(point(v.left() + px(*pad), v.top()), size((v.size.width - px(pad * 2.)).max(px(0.)), v.size.height));
        window.with_content_mask(Some(ContentMask { bounds: clip }), |window| {
            let _ = line.paint(point(v.left() + px(*pad), v.top()), px(*bh), window, cx);
        });
    }

    fn paint_file(&self, eq: &PlacedEq, window: &mut Window, cx: &mut App) {
        let EqBody::File { line, font, kind, icon, text_x } = &eq.geom.body else { return };
        let ix = eq.geom.ix;
        let v = eq.view;
        if self.selection.as_ref().is_some_and(|s| s.start <= ix && ix + 3 <= s.end) {
            window.paint_quad(fill(v, Theme::selection()));
        }
        let o = eq.origin;
        let ib = Bounds::new(point(o.x + px(icon[0]), o.y + px(icon[1])), size(px(icon[2]), px(icon[2])));
        crate::editor::file::paint_type_icon(*kind, ib, window, cx);
        let color = if eq.hitbox.is_hovered(window) { Theme::link_hover() } else { Theme::link() };
        let run = TextRun { len: line.text.len(), font: font.clone(), color, background_color: None, underline: None, strikethrough: None };
        let line = window.text_system().shape_line(line.text.clone(), line.font_size, &[run], None);
        let clip = Bounds::new(point(o.x + px(*text_x), v.top()), size((v.size.width - px(text_x + 2.)).max(px(0.)), v.size.height));
        window.with_content_mask(Some(ContentMask { bounds: clip }), |window| {
            let _ = line.paint(point(o.x + px(*text_x), v.top()), v.size.height, window, cx);
        });
    }

    fn paint_doc(&self, eq: &PlacedEq, window: &mut Window, cx: &mut App) {
        let EqBody::Doc { line, font, deleted, wrap, line_height } = &eq.geom.body else { return };
        let ix = eq.geom.ix;
        let v = eq.view;
        let hovered = !deleted && eq.hitbox.is_hovered(window);
        if self.selection.as_ref().is_some_and(|s| s.start <= ix && ix + 3 <= s.end) {
            window.paint_quad(fill(v, Theme::selection()));
        }
        let color = if *deleted { Theme::text_disabled() } else if hovered { Theme::link_hover() } else { Theme::link() };
        let run = TextRun { len: line.text.len(), font: font.clone(), color, background_color: None, underline: None, strikethrough: None };
        let lines = window.text_system().shape_text(line.text.clone(), line.font_size, &[run], *wrap, None).unwrap_or_default();
        let mut top = v.top();
        for line in lines {
            let _ = line.paint(point(v.left(), top), px(*line_height), TextAlign::Left, None, window, cx);
            top += px(*line_height * (line.wrap_boundaries().len() + 1) as f32);
        }
    }

    fn paint_reminder(&self, eq: &PlacedEq, window: &mut Window, cx: &mut App) {
        let EqBody::Reminder { line, icon, icon_color, filled, text_x } = &eq.geom.body else { return };
        let ix = eq.geom.ix;
        let v = eq.view;
        if self.selection.as_ref().is_some_and(|s| s.start <= ix && ix + 3 <= s.end) {
            window.paint_quad(fill(v, Theme::selection()));
        }
        let o = eq.origin;
        let ib = Bounds::new(point(o.x + px(icon[0]), o.y + px(icon[1])), size(px(icon[2]), px(icon[2])));
        let path = if *filled { "editor-icons/reminder-filled.svg" } else { "editor-icons/reminder.svg" };
        let _ = window.paint_svg(ib, path.into(), TransformationMatrix::unit(), *icon_color, cx);
        let lh = px(self.spec.line_height);
        let _ = line.paint(point(o.x + px(*text_x), o.y), lh, window, cx);
    }

    /// Pointer cursor, hover and `:active` repaints for equation views.
    fn listen_equations(&self, placed: &[PlacedEq], window: &mut Window) {
        if placed.is_empty() {
            return;
        }
        let id = self.id;
        let hits: Vec<(Hitbox, usize, bool)> =
            placed.iter().map(|e| (e.hitbox.clone(), e.geom.ix, e.hitbox.is_hovered(window))).collect();
        for (h, _, _) in &hits {
            window.set_cursor_style(CursorStyle::PointingHand, h);
        }
        let hs = hits.clone();
        window.on_mouse_event(move |_: &MouseMoveEvent, phase, window, _| {
            if phase == DispatchPhase::Bubble && hs.iter().any(|(h, _, was)| h.is_hovered(window) != *was) {
                window.refresh();
            }
        });
        let hs = hits.clone();
        window.on_mouse_event(move |ev: &MouseDownEvent, phase, window, _| {
            if phase == DispatchPhase::Capture
                && ev.button == MouseButton::Left
                && let Some((_, ix, _)) = hs.iter().find(|(h, _, _)| h.is_hovered(window))
            {
                EQ_PRESSED.set(Some((id, *ix)));
                window.refresh();
            }
        });
        window.on_mouse_event(move |_: &MouseUpEvent, phase, window, _| {
            if phase == DispatchPhase::Capture && EQ_PRESSED.get().is_some_and(|(b, _)| b == id) {
                EQ_PRESSED.set(None);
                window.refresh();
            }
        });
    }

    /// GPUI's own strikethrough sits at ~45% of the ascent, right on the
    /// middle stroke of most CJK glyphs, and drops glyphs after a font
    /// fallback switch. Draw it like the browser does instead: through the
    /// middle of the glyph box (done to-dos: 1.6px at 56%, per `.todo-line-through`).
    fn paint_strikes(&self, layout: &TextLayout, window: &mut Window) {
        let font_h = self.spec.size * 1.3;
        let (thickness, at) = if self.spec.strike_all { (1.6, 0.56) } else { (1., 0.5) };
        for (range, m) in self.text.spans() {
            if !(m.strike || self.spec.strike_all) || m.inline.is_some() {
                continue;
            }
            let color = span_color(&self.spec, m);
            for r in layout.rects_for(range) {
                let top = r.top() + (r.size.height - px(font_h)) / 2. + px(font_h * at - thickness / 2.);
                window.paint_quad(fill(Bounds::new(point(r.left(), top), size(r.size.width, px(thickness))), color));
            }
        }
    }
}

/// GPUI has no letter-spacing: shift the glyphs of an unwrapped line the way
/// CSS does (the spacing goes after every character), widen each equation's
/// stand-in glyph to the equation's advance, and redo the wrapping on the
/// adjusted positions.
fn adjust_line(line: &mut WrappedLine, ls: Pixels, eqs: &[&EqGeom], start: usize, wrap_width: Option<Pixels>) {
    let text = line.text.clone();
    let src = line.unwrapped_layout.clone();
    let x_at = |ix: usize| {
        src.runs.iter().flat_map(|r| r.glyphs.iter()).find(|g| g.index >= ix).map_or(src.width, |g| g.position.x)
    };
    // Extra width added after each byte offset.
    let mut shift = vec![px(0.); text.len() + 1];
    let mut n = 0;
    let mut acc = px(0.);
    for (i, ch) in text.char_indices() {
        shift[i] = acc;
        n += 1;
        acc += ls;
        if let Some(e) = eqs.iter().find(|e| e.ix - start == i) {
            acc += px(e.adv) - (x_at(i + ch.len_utf8()) - x_at(i));
        }
    }
    shift[text.len()] = acc;
    let _ = n;
    let runs = src
        .runs
        .iter()
        .map(|r| ShapedRun {
            font_id: r.font_id,
            glyphs: r
                .glyphs
                .iter()
                .map(|g| {
                    let mut g = g.clone();
                    g.position.x += shift[g.index.min(text.len())];
                    g
                })
                .collect(),
        })
        .collect();
    let layout = LineLayout {
        font_size: src.font_size,
        width: src.width + acc,
        ascent: src.ascent,
        descent: src.descent,
        runs,
        len: src.len,
    };
    let marks: Vec<(usize, bool)> = eqs.iter().map(|e| (e.ix - start, e.display)).collect();
    let wraps = wrap_width.map(|w| wrap_spaced(&text, &layout, w, &marks)).unwrap_or_default();
    let wrapped: &mut std::sync::Arc<WrappedLineLayout> = line;
    *wrapped = std::sync::Arc::new(WrappedLineLayout {
        unwrapped_layout: std::sync::Arc::new(layout),
        wrap_boundaries: wraps.into(),
        wrap_width,
    });
}

fn is_cjk(ch: char) -> bool {
    ('\u{2E80}'..='\u{9FFF}').contains(&ch) || ('\u{AC00}'..='\u{D7AF}').contains(&ch) || ('\u{F900}'..='\u{FFEF}').contains(&ch)
}

fn no_break_before(ch: char) -> bool {
    "，。、；：！？）」』】》〉’”…·,.;:!?)]}%".contains(ch)
}

fn no_break_after(ch: char) -> bool {
    "（「『【《〈‘“([{".contains(ch)
}

/// Greedy line breaking close to the browser's: after spaces and around CJK
/// characters, keeping closing punctuation with what precedes it. Equations
/// (`eqs`: offset, display) can break on both sides; display ones always do.
pub(crate) fn wrap_spaced(text: &str, layout: &LineLayout, width: Pixels, eqs: &[(usize, bool)]) -> Vec<WrapBoundary> {
    let glyphs: Vec<(usize, usize, usize, Pixels)> = layout
        .runs
        .iter()
        .enumerate()
        .flat_map(|(ri, r)| r.glyphs.iter().enumerate().map(move |(gi, g)| (ri, gi, g.index, g.position.x)))
        .collect();
    let mut out = vec![];
    let mut row_start = 0;
    let mut last_break: Option<usize> = None;
    for k in 0..glyphs.len() {
        let ix = glyphs[k].2;
        let ch = text[ix..].chars().next().unwrap_or(' ');
        if k > row_start && glyphs[k - 1].2 != ix {
            let prev = text[..ix].chars().next_back().unwrap_or(' ');
            let prev_ix = glyphs[k - 1].2;
            let eq_here = eqs.iter().find(|e| e.0 == ix);
            let eq_prev = eqs.iter().find(|e| e.0 == prev_ix);
            if eq_here.is_some_and(|e| e.1) || eq_prev.is_some_and(|e| e.1) {
                out.push(WrapBoundary { run_ix: glyphs[k].0, glyph_ix: glyphs[k].1 });
                row_start = k;
                last_break = None;
                continue;
            }
            let breakable = if eq_here.is_some() || eq_prev.is_some() {
                !ch.is_whitespace() && !no_break_before(ch)
            } else if ch.is_whitespace() {
                false
            } else if prev.is_whitespace() {
                true
            } else {
                (is_cjk(ch) || is_cjk(prev)) && !no_break_before(ch) && !no_break_after(prev)
            };
            if breakable {
                last_break = Some(k);
            }
        }
        let end = glyphs.get(k + 1).map(|g| g.3).unwrap_or(layout.width);
        if k > row_start && !ch.is_whitespace() && end - glyphs[row_start].3 > width {
            let b = last_break.filter(|&b| b > row_start).unwrap_or(k);
            out.push(WrapBoundary { run_ix: glyphs[b].0, glyph_ix: glyphs[b].1 });
            row_start = b;
            last_break = None;
        }
    }
    out
}

/// Pos helper for callers that hit-test across blocks.
pub fn hit_test(cache: &LayoutCache, p: Point<Pixels>, candidates: &[BlockId]) -> Option<Pos> {
    let cache = cache.borrow();
    let mut best: Option<(f32, f32, BlockId)> = None;
    for id in candidates {
        let Some(l) = cache.get(id) else { continue };
        let b = l.bounds;
        let dy = if p.y < b.top() {
            f32::from(b.top() - p.y)
        } else if p.y > b.bottom() {
            f32::from(p.y - b.bottom())
        } else {
            0.
        };
        let dx = if p.x < b.left() {
            f32::from(b.left() - p.x)
        } else if p.x > b.right() {
            f32::from(p.x - b.right())
        } else {
            0.
        };
        let better = match best {
            None => true,
            Some((by, bx, _)) => dy < by || (dy == by && dx < bx),
        };
        if better {
            best = Some((dy, dx, *id));
        }
    }
    let (_, _, id) = best?;
    let l = cache.get(&id)?;
    Some(Pos::new(id, l.index_for(p)))
}

#[cfg(all(test, target_os = "macos"))]
mod tests {
    /// Editor text shapes CJK through CoreText's PingFang fallback, while CJK
    /// punctuation loads the PingFang SC family (`CJK_PUNCT_FONT`): both must
    /// keep drawing the right glyphs.
    #[test]
    fn cjk_fallback_glyphs() {
        gpui::check_fallback_glyphs("周末规划一背景本季度目标").unwrap();
    }
}
