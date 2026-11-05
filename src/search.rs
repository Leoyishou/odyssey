//! 搜索 dialog: a 540-wide white dialog with the input row, 最近浏览 / hit
//! rows and a gray footer. Locally there is one owner and no sessions or
//! spaces, so each row's extra line is just 所有者 + the opened / modified
//! times. Radius 16, quiet row fills, monochrome page glyphs.

use crate::app::Workspace;
use crate::settings::display_title;
use crate::store::Entry;
use crate::theme::{self, Theme};
use crate::trash::scrim;
use crate::ui::{ROW_RADIUS, format_visit, icon, ui_runs, ui_text};
use gpui::{prelude::FluentBuilder, *};
use std::ops::Range;

/// How many documents 最近浏览 lists.
const RECENT: usize = 20;
/// Body text kept for a hit's two-line summary.
const SUMMARY_CHARS: usize = 160;
/// The summary joins its blocks with three no-break spaces.
const SUMMARY_SEP: &str = "\u{a0}\u{a0}\u{a0}";
/// Text column of a row: 540 − list padding 4 / 4 + 6 (scrollbar gutter)
/// − row padding 12 / 8 − icon 24 + 10.
const INFO_W: f32 = 472.;

/// Case-insensitive match position in `hay`, in `hay`'s own byte offsets.
pub(crate) fn find_ci(hay: &str, needle_lower: &str) -> Option<Range<usize>> {
    if needle_lower.is_empty() {
        return None;
    }
    let lower = hay.to_lowercase();
    if lower.len() == hay.len() {
        let i = lower.find(needle_lower)?;
        (hay.is_char_boundary(i) && hay.is_char_boundary(i + needle_lower.len())).then(|| i..i + needle_lower.len())
    } else {
        let i = hay.find(needle_lower)?;
        Some(i..i + needle_lower.len())
    }
}

/// Every match of `q` in `hay`.
fn find_all(hay: &str, q: &str) -> Vec<Range<usize>> {
    let mut out = vec![];
    let mut at = 0;
    while let Some(m) = find_ci(&hay[at..], q) {
        out.push(at + m.start..at + m.end);
        at += m.end;
    }
    out
}

/// Body text from the block holding the first match, with about 16
/// characters of context before it and the following blocks after it.
pub(crate) fn snippet(body: &str, q: &str) -> Option<(String, Vec<Range<usize>>)> {
    let m = find_ci(body, q)?;
    let line_start = body[..m.start].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let before: Vec<(usize, char)> = body[line_start..m.start].char_indices().collect();
    let cut = if before.len() > 16 { line_start + before[before.len() - 16].0 } else { line_start };
    let mut text = String::from(if cut > line_start { "…" } else { "" });
    let mut chars = 0;
    for line in body[cut..].lines().map(str::trim_end).filter(|l| !l.trim().is_empty()) {
        if !text.is_empty() && text != "…" {
            text.push_str(SUMMARY_SEP);
        }
        for ch in line.chars() {
            text.push(ch);
            chars += 1;
            if chars >= SUMMARY_CHARS {
                break;
            }
        }
        if chars >= SUMMARY_CHARS {
            break;
        }
    }
    let hits = find_all(&text, q);
    Some((text, hits))
}

/// The summary as two lines at most, ending in "…" when cut. GPUI's own
/// `line_clamp` truncates at twice the width, ignoring wrap leftovers.
pub(crate) fn summary_text(text: String, mut hits: Vec<Range<usize>>, width: f32, window: &Window) -> StyledText {
    let mut style = window.text_style();
    style.font_family = theme::UI_FONT.into();
    style.font_size = px(12.).into();
    style.color = Theme::text_caption();
    let lines = |s: &str| {
        window
            .text_system()
            .shape_text(s.to_string().into(), px(12.), &ui_runs(s, &style, &[], style.color), Some(px(width)), None)
            .map(|ls| ls.iter().map(|l| l.wrap_boundaries().len() + 1).sum::<usize>())
            .unwrap_or(1)
    };
    let text = if lines(&text) <= 2 {
        text
    } else {
        let ends: Vec<usize> = text.char_indices().map(|(i, _)| i).collect();
        let (mut lo, mut hi) = (0, ends.len());
        while lo + 1 < hi {
            let mid = (lo + hi) / 2;
            if lines(&format!("{}…", &text[..ends[mid]])) <= 2 { lo = mid } else { hi = mid }
        }
        let cut = ends[lo];
        hits.retain(|r| r.end <= cut);
        format!("{}…", &text[..cut])
    };
    let runs = ui_runs(&text, &style, &hits, Theme::link());
    StyledText::new(text).with_runs(runs)
}

/// A result's icon: the page's emoji, or the page glyph.
pub(crate) fn entry_icon(e: &Entry) -> Div {
    match &e.icon {
        Some(em) => div().flex_none().size(px(24.)).mr(px(10.)).flex().items_center().justify_center().text_size(px(20.)).child(em.clone()),
        None => doc_icon(),
    }
}

/// A page without an emoji: the monochrome page glyph (the sidebar's), in
/// a 24 slot.
pub(crate) fn doc_icon() -> Div {
    div()
        .flex_none()
        .size(px(24.))
        .mr(px(10.))
        .flex()
        .items_center()
        .justify_center()
        .child(icon("doc", 18., Theme::text_caption()))
}

/// The 24px square buttons at the right of the input row.
fn input_button(id: &'static str, glyph: &'static str, size: f32, color: Hsla) -> Stateful<Div> {
    div()
        .id(id)
        .flex_none()
        .size(px(24.))
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(ROW_RADIUS))
        .cursor_pointer()
        .hover(|s| s.bg(Theme::fill_hover()))
        .child(icon(glyph, size, color))
}

/// One `·`-separated part of a row's extra line.
fn extra_segment(glyph: Option<&'static str>, text: String) -> Div {
    div()
        .flex_none()
        .flex()
        .items_center()
        .children(glyph.map(|g| icon(g, 12., Theme::text_caption()).mr(px(4.))))
        .child(ui_text(text))
}

/// A result's extra line: 所有者 and the opened / modified times.
pub(crate) fn extra_line(e: &Entry) -> Div {
    let mut extra = vec![extra_segment(None, crate::settings::tr("所有者：我", "Owner: me").into())];
    if e.meta.opened > 0 {
        extra.push(extra_segment(Some("eye"), if crate::settings::english() { format!("Opened {}", format_visit(e.meta.opened)) } else { format!("你在 {}打开过", format_visit(e.meta.opened)) }));
    }
    extra.push(extra_segment(Some("pen-line"), if crate::settings::english() { format!("Modified {}", format_visit(e.meta.modified)) } else { format!("我在 {}修改过", format_visit(e.meta.modified)) }));
    let last = extra.len() - 1;
    let extra = extra.into_iter().enumerate().flat_map(|(i, seg)| {
        let seg = if i == last { seg.flex_shrink().min_w_0().overflow_hidden() } else { seg };
        [(i > 0).then(|| div().flex_none().child(" · ")), Some(seg)].into_iter().flatten()
    });
    div()
        .mt(px(2.))
        .h(px(20.))
        .flex()
        .items_center()
        .overflow_hidden()
        .whitespace_nowrap()
        .text_ellipsis()
        .text_size(px(12.))
        .line_height(px(20.))
        .text_color(Theme::text_caption())
        .children(extra)
}

/// A result's title (13pt semibold, hits in the link colour), summary and extra line.
pub(crate) fn result_info(e: &Entry, q: &str, summary: Option<(String, Vec<Range<usize>>)>, width: f32, window: &Window) -> Div {
    let title = display_title(&e.title);
    let title_hit = find_ci(&title, q);
    div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .overflow_hidden()
        .child(
            div()
                .text_size(px(13.))
                .line_height(px(20.))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(Theme::text())
                .overflow_hidden()
                .whitespace_nowrap()
                .text_ellipsis()
                .child(ui_text(title).hit(title_hit, Theme::link())),
        )
        .children(summary.map(|(text, hits)| div().text_size(px(12.)).line_height(px(20.)).child(summary_text(text, hits, width, window))))
        .child(extra_line(e))
}

impl Workspace {
    pub(crate) fn open_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.search_open = true;
        self.search_sel = Some(0);
        self.row_menu = None;
        self.doc_menu = false;
        self.doc_sub = None;
        let search = self.search.clone();
        search.update(cx, |s, cx| {
            // The dialog's query reads at body size, above the 13pt rows.
            s.size = 15.;
            s.line_height = 22.;
            s.set_text("", cx)
        });
        window.focus(&search.focus_handle(cx));
        self.search_scroll.set_offset(point(px(0.), px(0.)));
        cx.notify();
    }

    pub(crate) fn close_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.search_open {
            return;
        }
        self.search_open = false;
        if let Some(ed) = &self.editor {
            ed.read(cx).focus(window);
        }
        cx.notify();
    }

    /// First result is selected; an empty result list has no selection.
    pub(crate) fn default_search_sel(&self, cx: &App) -> Option<usize> {
        (!self.search_results(cx).is_empty()).then_some(0)
    }

    pub(crate) fn pick_search(&mut self, row: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(id) = self.search_results(cx).get(row).map(|e| e.id.clone()) else { return };
        self.close_search(window, cx);
        self.open(&id, window, cx);
        if let Some(ed) = &self.editor {
            ed.read(cx).focus(window);
        }
    }

    pub(crate) fn move_search_sel(&mut self, delta: isize, cx: &mut Context<Self>) {
        let n = self.search_results(cx).len();
        if n == 0 { self.search_sel = None; return; }
        let next = match self.search_sel {
            Some(i) => (i as isize + delta).rem_euclid(n as isize) as usize,
            None if delta > 0 => 0,
            None => n - 1,
        };
        self.search_sel = Some(next);
        let title = usize::from(self.query(cx).is_empty());
        self.search_scroll.scroll_to_item(next + title);
        cx.notify();
    }

    /// Title hits first, then body hits; with no query, 最近浏览.
    pub fn search_results(&self, cx: &App) -> Vec<&Entry> {
        let q = self.query(cx);
        if q.is_empty() {
            let mut recent: Vec<&Entry> = self.entries.iter().collect();
            recent.sort_by_key(|e| std::cmp::Reverse(e.meta.opened.max(e.meta.modified)));
            recent.truncate(RECENT);
            return recent;
        }
        let mut hits: Vec<(u8, &Entry)> = self
            .entries
            .iter()
            .filter_map(|e| {
                if e.title.to_lowercase().contains(&q) {
                    Some((0, e))
                } else if e.body.to_lowercase().contains(&q) {
                    Some((1, e))
                } else {
                    None
                }
            })
            .collect();
        hits.sort_by_key(|(rank, _)| *rank);
        hits.into_iter().map(|(_, e)| e).collect()
    }

    fn search_row(&self, e: &Entry, ix: usize, q: &str, window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        let summary = snippet(&e.body, q);
        // Result indices match keyboard selection.
        let row = ix;
        div()
            .id(("search-hit", ix))
            .flex_none()
            .pt(px(8.))
            .pr(px(8.))
            .pb(px(8.))
            .pl(px(12.))
            .flex()
            .items_center()
            .rounded(px(ROW_RADIUS))
            .cursor_pointer()
            .when(self.search_sel == Some(row), |d| d.bg(Theme::fill_hover()))
            .on_hover(cx.listener(move |w, hovered: &bool, _, cx| {
                if *hovered && w.search_sel != Some(row) {
                    w.search_sel = Some(row);
                    cx.notify();
                }
            }))
            .on_click(cx.listener(move |w, _, window, cx| w.pick_search(row, window, cx)))
            .child(entry_icon(e))
            .child(result_info(e, q, summary, INFO_W, window))
    }

    pub(crate) fn render_search(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Option<AnyElement> {
        if !self.search_open {
            return None;
        }
        let q = self.query(cx);
        let has_text = !self.search.read(cx).text.is_empty();
        let results: Vec<Entry> = self.search_results(cx).into_iter().cloned().collect();
        let vh = f32::from(window.viewport_size().height);

        let input = div()
            .flex_none()
            .h(px(46.))
            .pl(px(12.))
            .pr(px(8.))
            .flex()
            .items_center()
            .child(
                // The glyph's ink fills 14.6 of the 16px box; Lucide's fills 20/24.
                div()
                    .relative()
                    .flex_none()
                    .size(px(16.))
                    .ml(px(4.))
                    .mr(px(10.))
                    .child(icon("search", 17.5, Theme::text_placeholder()).absolute().top(px(-0.75)).left(px(-0.75))),
            )
            .child(div().flex_1().min_w_0().pl(px(2.)).flex().child(self.search.clone()))
            .when(has_text, |d| {
                d.child(input_button("search-clear", "search-clear", 18., Theme::text_disabled()).ml(px(8.)).on_click(cx.listener(
                    |w, _, window, cx| {
                        w.search.update(cx, |s, cx| s.set_text("", cx));
                        window.focus(&w.search.focus_handle(cx));
                        w.search_sel = Some(0);
                        w.search_scroll.set_offset(point(px(0.), px(0.)));
                        cx.notify();
                    },
                )))
            })
            ;

        let mut list = div()
            .id("search-results")
            .flex_1()
            .min_h_0()
            .pt(px(4.))
            .pl(px(4.))
            .pr(px(10.))
            .flex()
            .flex_col()
            .overflow_y_scroll()
            .track_scroll(&self.search_scroll);
        if q.is_empty() && !results.is_empty() {
            list = list.child(
                div()
                    .flex_none()
                    .h(px(36.))
                    .pt(px(2.))
                    .px(px(12.))
                    .flex()
                    .items_center()
                    .text_size(px(12.))
                    .line_height(px(18.))
                    .text_color(Theme::text_caption())
                    .child(crate::settings::tr("最近浏览", "Recently viewed")),
            );
        }
        if results.is_empty() && !q.is_empty() {
            list = list.child(
                div()
                    .flex_none()
                    .m(px(8.))
                    .px(px(10.))
                    .text_size(px(12.))
                    .line_height(px(18.))
                    .text_color(Theme::text_caption())
                    .child(crate::settings::text("没有找到匹配的结果，尝试其他关键词进行搜索")),
            );
        }
        for (i, e) in results.iter().enumerate() {
            list = list.child(self.search_row(e, i, &q, window, cx));
        }

        let footer = div()
            .flex_none()
            .h(px(36.))
            .pl(px(14.))
            .pr(px(8.))
            .flex()
            .items_center()
            // GPUI clips children to the dialog's rect, not its rounded corners.
            .rounded_b(px(16.))
            .border_t_1()
            .border_color(Theme::line_divider())
            .bg(Theme::bg_overlay())
            .text_size(px(12.))
            .line_height(px(18.))
            .text_color(Theme::text_caption())
            .whitespace_nowrap()
            .overflow_hidden()
            .child(ui_text(crate::settings::tr("支持 ↑↓ 键选择、Enter 键打开", "Use ↑↓ to select, Enter to open")));

        let modal = div()
            .id("search-dialog")
            .occlude()
            .w(px(540.))
            .h(px(630f32.min(vh - 120.).max(200.)))
            .flex()
            .flex_col()
            .rounded(px(16.))
            .bg(crate::theme::raised(Theme::bg_body()))
            .shadow(theme::shadow_dialog())
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(input)
            .child(div().flex_none().h(px(1.)).bg(Theme::line_divider()))
            .child(list)
            .child(footer);

        Some(
            deferred(
                div()
                    .id("search-mask")
                    .absolute()
                    .inset_0()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .bg(scrim())
                    .on_mouse_down(MouseButton::Left, cx.listener(|w, _, window, cx| w.close_search(window, cx)))
                    .child(modal),
            )
            .with_priority(3)
            .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::snippet;

    #[test]
    fn snippet_context() {
        let (s, r) = snippet("first line\nsome long prefix text before the Needle here\nnext", "needle").unwrap();
        assert_eq!(&s[r[0].clone()], "Needle");
        assert!(s.starts_with('…'));
        assert!(s.ends_with("\u{a0}\u{a0}\u{a0}next"));
        let (s, r) = snippet("中文内容测试内容", "内容").unwrap();
        assert_eq!(r.len(), 2);
        assert_eq!(&s[r[1].clone()], "内容");
        assert!(snippet("abc", "x").is_none());
    }
}
