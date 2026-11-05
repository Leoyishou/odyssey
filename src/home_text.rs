//! Inert library text previews. Formulas reuse the editor's KaTeX layout and
//! line-breaking rules; no selection, caret, hitboxes, or editing listeners.
use crate::editor::{math, text::wrap_spaced};
use crate::model::{Align, RichText};
use crate::theme::{self, Theme};
use gpui::*;
use std::{cell::RefCell, rc::Rc, sync::Arc};

pub struct PreviewText {
    pub text: RichText,
    pub runs: Vec<TextRun>,
    pub size: f32,
    pub line_height: f32,
    pub align: Align,
    pub eq_counter: u32,
}

struct Equation {
    ix: usize,
    display: bool,
    advance: f32,
    ascent: f32,
    depth: f32,
    left: f32,
    body: Body,
}

enum Body { Math(Rc<math::Math>), Invalid(ShapedLine) }
struct Placed { equation: Rc<Equation>, x: Pixels, baseline: Pixels }
pub struct Prepared { lines: Vec<(WrappedLine, Vec<Pixels>)>, equations: Vec<Placed>, height: Pixels, width: Pixels }

fn equation(ix: usize, source: &str, width: f32, color: Hsla, eqno: u32, window: &Window) -> Rc<Equation> {
    let scale = super::bs::SCALE;
    let pad = 2. * scale;
    let display = math::is_block(source);
    if let Some(mut m) = math::layout(source, display, math::FONT_PX * scale, color, eqno, window) {
        let tag = m.tag.as_ref().map_or(0., |t| t.width + 8. * scale);
        let fit = ((width - 2. * pad).max(scale) / (m.width + tag).max(scale)).min(1.);
        if fit < 1. { m = math::layout(source, display, math::FONT_PX * scale * fit, color, eqno, window).unwrap_or(m); }
        let w = m.width + m.tag.as_ref().map_or(0., |t| t.width + 8. * scale);
        let margin = if display { 15. * scale } else { 0. };
        Rc::new(Equation { ix, display, advance: if display { width } else { w + 2. * pad }, ascent: m.ascent + margin, depth: m.depth + margin,
            left: if display { ((width - w) / 2.).max(pad) } else { pad }, body: Body::Math(m) })
    } else {
        let label: SharedString = if source.trim().is_empty() { crate::settings::text("公式") } else { crate::settings::text("无效公式") }.into();
        let run = TextRun { len: label.len(), font: font(theme::UI_FONT), color: Theme::text_caption(), background_color: None, underline: None, strikethrough: None };
        let line = window.text_system().shape_line(label, px(13. * scale), &[run], None);
        Rc::new(Equation { ix, display: false, advance: f32::from(line.width) + pad * 2., ascent: line.ascent.into(), depth: line.descent.into(), left: pad, body: Body::Invalid(line) })
    }
}

fn widen(line: &mut WrappedLine, equations: &[Rc<Equation>], start: usize, width: Pixels) {
    let src = line.unwrapped_layout.clone();
    let x_at = |ix: usize| src.runs.iter().flat_map(|r| &r.glyphs).find(|g| g.index >= ix).map_or(src.width, |g| g.position.x);
    let mut shifts = vec![px(0.); line.len() + 1];
    let mut extra = px(0.);
    for (ix, ch) in line.text.char_indices() {
        shifts[ix] = extra;
        if let Some(eq) = equations.iter().find(|eq| eq.ix == start + ix) {
            extra += px(eq.advance) - (x_at(ix + ch.len_utf8()) - x_at(ix));
        }
    }
    shifts[line.len()] = extra;
    let runs = src.runs.iter().map(|run| ShapedRun { font_id: run.font_id, glyphs: run.glyphs.iter().map(|g| {
        let mut g = g.clone(); g.position.x += shifts[g.index.min(line.len())]; g
    }).collect() }).collect();
    let layout = LineLayout { font_size: src.font_size, width: src.width + extra, ascent: src.ascent, descent: src.descent, runs, len: src.len };
    let marks: Vec<_> = equations.iter().filter(|e| e.ix >= start && e.ix < start + line.len()).map(|e| (e.ix - start, e.display)).collect();
    let boundaries = wrap_spaced(&line.text, &layout, width, &marks);
    let target: &mut Arc<WrappedLineLayout> = line;
    *target = Arc::new(WrappedLineLayout { unwrapped_layout: Arc::new(layout), wrap_boundaries: boundaries.into(), wrap_width: Some(width) });
}

impl IntoElement for PreviewText { type Element = Self; fn into_element(self) -> Self { self } }
impl Element for PreviewText {
    type RequestLayoutState = Rc<RefCell<Option<Prepared>>>;
    type PrepaintState = ();
    fn id(&self) -> Option<ElementId> { None }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> { None }
    fn request_layout(&mut self, _: Option<&GlobalElementId>, _: Option<&InspectorElementId>, window: &mut Window, _: &mut App) -> (LayoutId, Self::RequestLayoutState) {
        let state = Rc::new(RefCell::new(None));
        let stored = state.clone();
        let mut text = self.text.text.clone();
        let mut sources = vec![];
        for (range, marks) in self.text.spans() {
            if let Some(source) = marks.inline.as_ref().and_then(|i| i.equation()) {
                for (offset, ch) in self.text.text[range.clone()].char_indices() { if ch == '\u{fffc}' { sources.push((range.start + offset, source.to_string())); } }
            }
        }
        for (ix, _) in &sources { text.replace_range(*ix..*ix + 3, "\u{2013}"); }
        let mut runs = self.runs.clone();
        let mut off = 0;
        let mut colors = vec![];
        for run in &mut runs {
            for (ix, _) in sources.iter().filter(|(ix, _)| *ix >= off && *ix < off + run.len) { colors.push((*ix, run.color)); }
            if sources.iter().any(|(ix, _)| *ix >= off && *ix < off + run.len) { run.color = transparent_black(); run.background_color = None; run.underline = None; run.strikethrough = None; }
            off += run.len;
        }
        let text: SharedString = text.into();
        let (fs, lh, align) = (px(self.size), px(self.line_height), self.align);
        let eq_counter = self.eq_counter;
        let mut style = Style::default(); style.size.width = relative(1.).into();
        let id = window.request_measured_layout(style, move |known, available, window, _| {
            let width = known.width.or(match available.width { AvailableSpace::Definite(w) => Some(w), _ => None }).unwrap_or(px(690. * super::bs::SCALE)).max(px(1.));
            let mut eqno = eq_counter;
            let eqs: Vec<_> = sources.iter().map(|(ix, src)| {
                let eq = equation(*ix, src, width.into(), colors.iter().find(|c| c.0 == *ix).map_or(Theme::text(), |c| c.1), eqno, window);
                eqno += math::numbers_in(src);
                eq
            }).collect();
            let mut lines = vec![]; let mut placed = vec![];
            let mut y = px(0.); let mut start = 0;
            for mut line in window.text_system().shape_text(text.clone(), fs, &runs, None, None).unwrap_or_default() {
                widen(&mut line, &eqs, start, width);
                let layout = &line.unwrapped_layout;
                let base = (lh - layout.ascent - layout.descent) / 2. + layout.ascent;
                let mut cuts = vec![(0, px(0.))];
                for boundary in line.wrap_boundaries() { let g = &layout.runs[boundary.run_ix].glyphs[boundary.glyph_ix]; cuts.push((g.index, g.position.x)); }
                cuts.push((line.len() + 1, layout.width));
                let mut tops = vec![];
                for row in cuts.windows(2) {
                    let row_eqs: Vec<_> = eqs.iter().filter(|e| e.ix >= start + row[0].0 && e.ix < start + row[1].0).collect();
                    let mut top = px(0.); let mut bottom = lh;
                    for eq in &row_eqs { top = top.min(base - px(eq.ascent)); bottom = bottom.max(base + px(eq.depth)); }
                    let dx = match align { Align::Left => px(0.), Align::Center => ((width - row[1].1 + row[0].1) / 2.).max(px(0.)), Align::Right => (width - row[1].1 + row[0].1).max(px(0.)) };
                    for eq in row_eqs { placed.push(Placed { equation: (*eq).clone(), x: layout.x_for_index(eq.ix - start) - row[0].1 + dx, baseline: y - top + base }); }
                    tops.push(y - top); y += bottom - top;
                }
                start += line.len() + 1;
                lines.push((line, tops));
            }
            let height = y.max(lh);
            *stored.borrow_mut() = Some(Prepared { lines, equations: placed, height, width });
            size(width, height)
        });
        (id, state)
    }
    fn prepaint(&mut self, _: Option<&GlobalElementId>, _: Option<&InspectorElementId>, _: Bounds<Pixels>, _: &mut Self::RequestLayoutState, _: &mut Window, _: &mut App) {}
    fn paint(&mut self, _: Option<&GlobalElementId>, _: Option<&InspectorElementId>, bounds: Bounds<Pixels>, state: &mut Self::RequestLayoutState, _: &mut (), window: &mut Window, cx: &mut App) {
        let ready = state.borrow(); let Some(ready) = ready.as_ref() else { return };
        let align = match self.align { Align::Left => TextAlign::Left, Align::Center => TextAlign::Center, Align::Right => TextAlign::Right };
        let area = Bounds::new(bounds.origin, size(ready.width, ready.height));
        window.with_content_mask(Some(ContentMask { bounds: area }), |window| {
            for (line, tops) in &ready.lines {
                let _ = line.paint_background_rows(bounds.origin, px(self.line_height), tops, align, Some(area), window, cx);
                let _ = line.paint_rows(bounds.origin, px(self.line_height), tops, align, Some(area), window, cx);
            }
            for placed in &ready.equations {
                let origin = point(bounds.left() + placed.x + px(placed.equation.left), bounds.top() + placed.baseline);
                match &placed.equation.body {
                    Body::Math(math) => { math.paint(origin, window, cx); if let Some(tag) = &math.tag { tag.paint(point(origin.x + px(math.width + 8. * super::bs::SCALE), origin.y), window, cx); } }
                    Body::Invalid(line) => { let _ = line.paint(point(origin.x, origin.y - line.ascent), line.ascent + line.descent, window, cx); }
                }
            }
        });
    }
}
