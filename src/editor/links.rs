//! 双向链接: the @文档 panel and the 反向引用 / 本文引用 list at the end of
//! the document. The library side (titles, who links here) comes from the
//! workspace (`Editor::set_links`).

use std::collections::HashSet;
use std::rc::Rc;

use super::core::{Pos, Selection};
use super::overlays::at;
use super::view::{Editor, EditorEvent};
use crate::settings::display_title;
use crate::model::{BlockId, DocLink};
use crate::editor_theme::Theme;
use crate::editor_ui::{ROW_RADIUS, TipExt, icon, popover, ui_text};
use gpui::{prelude::FluentBuilder, *};

#[derive(Clone, Debug, PartialEq)]
pub struct DocRef {
    pub id: String,
    pub title: String,
    pub preview: String,
    pub pending: bool,
    pub icon: Option<String>,
    pub modified: u64,
    pub opened: u64,
}

/// A document linking here, with the paragraphs that do.
#[derive(Clone, Debug, PartialEq)]
pub struct Backlink {
    pub id: String,
    pub lines: Vec<DocLink>,
}

#[derive(Default)]
pub struct Links {
    /// The library, the open document included.
    pub docs: Rc<Vec<DocRef>>,
    pub me: String,
    pub backlinks: Vec<Backlink>,
    /// 显示引用列表 off.
    pub hidden: bool,
    /// The chosen tab (true: 反向引用); unset follows the counts.
    pub tab: Option<bool>,
    pub collapsed: HashSet<String>,
    /// 查看更多 clicked, per tab (反向引用, 本文引用).
    pub more: [bool; 2],
    /// Bumped when a title changes (links re-shape).
    pub generation: u64,
    /// 本文引用, cached against `core.generation`.
    refs: (Option<u64>, Vec<String>),
}

/// The open @ panel: `trigger` is the byte offset of the "@", the query is
/// `text[start..caret]`.
#[derive(Clone, Debug)]
pub struct Mention {
    pub selection: Option<Selection>,
    pub block: BlockId,
    pub trigger: usize,
    pub start: usize,
    pub selected: usize,
}

/// Only a newly typed trigger opens a chooser. An older, cancelled trigger
/// stays ordinary text when the user continues writing.
fn new_wikilink_trigger(prefix: &str, inserted: &str) -> Option<(usize, usize)> {
    ["[[", "【【", "［［"].iter()
        .filter(|trigger| prefix.ends_with(**trigger) || inserted.contains(**trigger))
        .filter_map(|trigger| prefix.rfind(trigger).map(|start| (start, trigger.len())))
        .max_by_key(|(start, _)| *start)
}

const INIT_RENDER: usize = 10;
/// The reference list sits in the page with crisp corners: the box at 5,
/// hover backgrounds inside it at 4.
const CARD: f32 = 5.;
const ROW: f32 = 4.;

impl Links {
    fn doc(&self, id: &str) -> Option<&DocRef> {
        self.docs.iter().find(|d| d.id == id)
    }
}

impl Editor {
    pub fn set_links(&mut self, me: String, docs: Vec<DocRef>, backlinks: Vec<Backlink>, hidden: bool, cx: &mut Context<Self>) {
        let l = &mut self.links;
        if l.me != me {
            *l = Links { generation: l.generation, ..Links::default() };
            l.me = me;
        }
        if *l.docs != docs {
            let titles = docs.iter().map(|d| (d.id.clone(), (if d.pending { format!("未创建：{}", d.title) } else { d.title.clone() }, d.icon.clone()))).collect();
            super::text::DOC_TITLES.with(|t| *t.borrow_mut() = titles);
            l.docs = Rc::new(docs);
            l.generation += 1;
        }
        l.backlinks = backlinks;
        l.hidden = hidden;
        cx.notify();
    }

    /// Distinct link targets of the open document, in order.
    pub(super) fn doc_refs(&mut self) -> &[String] {
        if self.links.refs.0 != Some(self.core.generation) {
            let mut seen = HashSet::new();
            let refs = self.core.doc.doc_links().into_iter().map(|l| l.target).filter(|t| seen.insert(t.clone())).collect();
            self.links.refs = (Some(self.core.generation), refs);
        }
        &self.links.refs.1
    }

    /// The bottom list's state, for the row key.
    pub(super) fn links_key(&mut self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = rustc_hash::FxHasher::default();
        self.doc_refs().hash(&mut h);
        let l = &self.links;
        (l.generation, l.hidden, l.tab, l.more, l.backlinks.len()).hash(&mut h);
        for b in &l.backlinks {
            (&b.id, b.lines.len(), l.collapsed.contains(&b.id)).hash(&mut h);
            for d in &b.lines {
                (d.block, &d.line).hash(&mut h);
            }
        }
        h.finish() | 1
    }

    // ---- @ panel

    pub(super) fn maybe_open_mention(&mut self, trigger_len: usize) {
        let h = self.core.sel.head;
        if h.block == crate::editor::core::TITLE || self.core.is_code(h.block) || h.offset < trigger_len {
            return;
        }
        if !self.core.kind_of(h.block).is_some_and(|k| k.has_text()) {
            return;
        }
        self.mention = Some(Mention { selection: None, block: h.block, trigger: h.offset - trigger_len, start: h.offset, selected: 0 });
        self.mention_scroll.set_offset(Point::default());
    }

    pub(super) fn link_selection(&mut self, cx: &mut Context<Self>) {
        let (a, b) = self.core.ordered();
        if a.block != b.block || a.block == crate::editor::core::TITLE || self.core.is_code(a.block) { return }
        self.mention = Some(Mention { selection: Some(self.core.sel), block: a.block, trigger: a.offset, start: a.offset, selected: 0 });
        self.mention_scroll.set_offset(Point::default());
        cx.notify();
    }

    pub(super) fn sync_wikilink_input(&mut self, inserted: &str, window: &mut Window, cx: &mut Context<Self>) {
        let h = self.core.sel.head;
        if self.core.is_code(h.block) || h.block == crate::editor::core::TITLE || self.core.text_of(h.block).is_some_and(|t| t.marks_at(h.offset.saturating_sub(1)).code) { return }
        let prefix = self.core.text_of(h.block).and_then(|t| t.text.get(..h.offset)).unwrap_or("").to_string();
        if self.mention.is_none() && let Some((start, trigger_len)) = new_wikilink_trigger(&prefix, inserted) {
                let query = &prefix[start + trigger_len..];
                if !query.contains('\n') && !query.contains("\u{fffc}") && query.chars().count() <= 200 {
                    self.maybe_open_mention(h.offset - start);
                    if let Some(m) = &mut self.mention { m.start = start + trigger_len; }
                }
        }
        let Some(m) = &self.mention else { return };
        if !matches!(prefix.get(m.trigger..m.start), Some("[[" | "【【" | "［［")) || !["]]", "】】", "］］"].iter().any(|end| prefix.ends_with(end)) { return }
        let query = self.mention_query();
        let name = query.trim_end_matches(&[']', '】', '］'][..]).split('|').next().unwrap_or("").trim();
        let exact: Vec<_> = self.mention_results().into_iter().filter(|d| d.title.trim() == name).collect();
        // A typed title may be ambiguous: leave the chooser open in that case.
        if exact.len() == 1 { self.pick_mention(exact[0].clone(), window, cx); }
    }

    pub(super) fn sync_mention(&mut self) {
        let Some(m) = self.mention.clone() else { return };
        if m.selection.is_some() { return }
        let sel = self.core.sel;
        let text = self.core.text_of(m.block).map(|t| t.text.clone()).unwrap_or_default();
        let query = text.get(m.start..sel.head.offset.max(m.start)).unwrap_or("");
        let close = !sel.is_collapsed()
            || self.core.in_block_mode()
            || sel.head.block != m.block
            || sel.head.offset < m.start
            || sel.head.offset > text.len()
            || !matches!(text.get(m.trigger..m.start), Some("@") | Some("＠") | Some("[[") | Some("【【") | Some("［［"))
            || query.contains('\n')
            || query.chars().count() > 200
            || (query.ends_with(' ') && self.mention_results().is_empty());
        if close {
            self.mention = None;
            return;
        }
        let n = self.mention_results().len();
        if let Some(m) = &mut self.mention {
            m.selected = m.selected.min(n.saturating_sub(1));
        }
    }

    fn mention_query(&self) -> String {
        let Some(m) = &self.mention else { return String::new() };
        if m.selection.is_some() { return String::new() }
        let text = self.core.text_of(m.block).map(|t| t.text.as_str()).unwrap_or("");
        text.get(m.start..self.core.sel.head.offset).unwrap_or("").trim().to_string()
    }

    /// Matching documents: the most recently opened first, or title matches.
    fn mention_results(&self) -> Vec<DocRef> {
        let query = self.mention_query();
        let q = query.split('|').next().unwrap_or("").trim_end_matches(&[']', '】', '］'][..]).trim().to_lowercase();
        let mut out: Vec<DocRef> = self.links.docs.iter().cloned().collect();
        if q.is_empty() {
            out.sort_by(|a, b| b.opened.cmp(&a.opened).then(b.modified.cmp(&a.modified)));
        } else {
            out.retain(|d| display_title(&d.title).to_lowercase().contains(&q));
            out.sort_by_key(|e| std::cmp::Reverse(e.modified));
        }
        if !q.is_empty() && !out.iter().any(|d| d.title.to_lowercase() == q) {
            let title = query.trim_end_matches(&[']', '】', '］'][..]).split('|').next().unwrap_or("").trim().to_string();
            out.push(DocRef { id: uuid::Uuid::new_v4().to_string(), title, preview: "尚未创建，点击链接后创建文档".into(), pending: true, icon: None, modified: 0, opened: 0 });
        }
        out.truncate(20);
        out
    }

    /// Keys while the @ panel is open. Returns true when consumed.
    pub(super) fn mention_key(&mut self, up: Option<bool>, enter: bool, close: bool, window: &mut Window, cx: &mut Context<Self>) -> bool {
        let Some(m) = self.mention.clone() else { return false };
        if close {
            self.mention = None;
            cx.notify();
            return true;
        }
        let results = self.mention_results();
        if let Some(up) = up {
            let n = results.len().max(1);
            let v = if up { (m.selected + n - 1) % n } else { (m.selected + 1) % n };
            if let Some(m) = &mut self.mention {
                m.selected = v;
            }
            self.mention_scroll.scroll_to_item(v);
            cx.notify();
            return true;
        }
        if enter {
            match results.get(m.selected) {
                Some(d) => self.pick_mention(d.clone(), window, cx),
                None => self.mention = None,
            }
            cx.notify();
            return true;
        }
        false
    }

    fn pick_mention(&mut self, d: DocRef, window: &mut Window, cx: &mut Context<Self>) {
        let Some(m) = self.mention.take() else { return };
        let head = self.core.sel.head;
        self.core.sel = m.selection.unwrap_or(Selection { anchor: Pos::new(m.block, m.trigger), head });
        let text = self.core.text_of(m.block).map(|t| t.text.as_str()).unwrap_or("");
        let selected_alias = m.selection.and_then(|sel| text.get(sel.anchor.offset.min(sel.head.offset)..sel.anchor.offset.max(sel.head.offset))).map(str::to_string).filter(|s| !s.is_empty());
        let alias = selected_alias.or_else(|| text.get(m.start..head.offset).and_then(|q| q.split_once('|')).map(|(_, a)| a.trim_end_matches(&[']', '】', '］'][..]).trim().to_string()).filter(|a| !a.is_empty()));
        let link = crate::model::InlineDocLink { id: d.id, title: display_title(&d.title).to_string(), alias, pending: d.pending };
        if self.core.insert_doc_link(&link).is_none() {
            self.core.sel = Selection::caret(head);
        }
        self.changed(window, cx);
    }

    /// 280 wide, the documents as 48px options, in the editor's list
    /// popover (grey row highlight).
    pub(super) fn mention_panel(&self, window: &Window, cx: &mut Context<Self>) -> Option<AnyElement> {
        let m = self.mention.as_ref()?;
        let (p, lh) = self.point_of(Pos::new(m.block, m.trigger))?;
        let q = self.mention_query();
        let results = self.mention_results();
        let mut list = div().id("mention-list").max_h(px(366.)).overflow_y_scroll().track_scroll(&self.mention_scroll).py(px(5.)).flex().flex_col();
        if results.is_empty() {
            list = list.child(div().p(px(16.)).text_size(px(13.)).line_height(px(20.)).text_color(Theme::text_caption()).child(ui_text(crate::settings::text("没有匹配结果，换个关键词试试吧"))));
        }
        for (k, d) in results.into_iter().enumerate() {
            let title = if d.pending { format!("未创建：{}", d.title) } else { display_title(&d.title).to_string() };
            let mut runs = vec![];
            let lower = title.to_lowercase();
            if let Some(at) = (!q.is_empty()).then(|| lower.find(&q.to_lowercase())).flatten().filter(|&a| title.is_char_boundary(a) && title.is_char_boundary(a + q.len())) {
                runs.push((0..at, Theme::text()));
                runs.push((at..at + q.len(), Theme::primary_500()));
                runs.push((at + q.len()..title.len(), Theme::text()));
            } else {
                runs.push((0..title.len(), Theme::text()));
            }
            let font = window.text_style().font();
            let hl: Vec<TextRun> = runs
                .into_iter()
                .filter(|(r, _)| !r.is_empty())
                .map(|(r, c)| TextRun { len: r.len(), font: font.clone(), color: c, background_color: None, underline: None, strikethrough: None })
                .collect();
            let avatar = match &d.icon {
                Some(e) => div().size(px(32.)).flex_none().flex().items_center().justify_center().text_size(px(24.)).child(e.clone()).into_any_element(),
                None => div()
                    .relative()
                    .flex_none()
                    .size(px(32.))
                    .child(icon("doc-blue", 32., Theme::primary_500()).absolute())
                    .child(icon("doc-blue-lines", 32., Theme::bg_body()).absolute())
                    .into_any_element(),
            };
            let pick = d.clone();
            list = list.child(
                div()
                    .id(("mention", k))
                    .flex_none()
                    .h(px(48.))
                    .mx(px(5.))
                    .px(px(8.))
                    .py(px(4.))
                    .rounded(px(ROW_RADIUS))
                    .flex()
                    .items_center()
                    .cursor_pointer()
                    .when(k == m.selected, |d| d.bg(Theme::fill_hover()))
                    .on_mouse_move(cx.listener(move |e, _: &MouseMoveEvent, _, cx| {
                        if let Some(m) = &mut e.mention
                            && m.selected != k
                        {
                            m.selected = k;
                            cx.notify();
                        }
                    }))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |e, _: &MouseDownEvent, window, cx| {
                            cx.stop_propagation();
                            e.pick_mention(pick.clone(), window, cx);
                        }),
                    )
                    .child(div().mr(px(8.)).child(avatar))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_size(px(13.))
                                    .line_height(px(20.))
                                    .text_color(Theme::text())
                                    .truncate()
                                    .child(StyledText::new(title).with_runs(hl)),
                            )
                            .child(
                                div()
                                    .text_size(px(11.))
                                    .line_height(px(16.))
                                    .text_color(Theme::text_placeholder())
                                    .truncate()
                                    .child(ui_text(crate::localized_format!("最近修改：{}", "Last modified: {}", format!("{} · {}", crate::editor_ui::format_ago(d.modified), &d.id[..d.id.len().min(8)])))),
                            ),
                    ),
            );
        }
        let el = popover()
            .id("mention-box")
            .occlude()
            .w(px(280.))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(list);
        let vh = f32::from(window.viewport_size().height);
        let below = point(p.x, p.y + lh);
        Some(if vh - f32::from(below.y) < 380. { at(self.rel(p), Corner::BottomLeft, el) } else { at(self.rel(below), Corner::TopLeft, el) })
    }

    // ---- 反向引用 / 本文引用

    /// The list, or nothing when there are no links either way (or the list
    /// is hidden).
    pub(super) fn refs_list(&self, window: &Window, cx: &mut Context<Self>) -> Option<AnyElement> {
        // Refreshed by `links_key` in `sync_items`.
        let refs = self.links.refs.1.clone();
        let l = &self.links;
        let (nb, nr) = (l.backlinks.len(), refs.len());
        if l.hidden || nb + nr == 0 {
            return None;
        }
        let back = l.tab.unwrap_or(!(nb == 0 && nr != 0));
        // The tabs: the chosen one in the accent and semibold over a
        // 2pt accent bar, the other secondary.
        let tab = |id: &'static str, name: &str, n: usize, active: bool, b: bool| {
            div()
                .id(id)
                .mr(px(32.))
                .relative()
                .flex()
                .items_center()
                .py(px(10.))
                .px(px(2.))
                .text_size(px(13.))
                .line_height(px(20.))
                .cursor_pointer()
                .text_color(if active { Theme::primary() } else { Theme::text_caption() })
                .when(active, |d| d.font_weight(FontWeight::SEMIBOLD))
                .when(!active, |d| d.hover(|s| s.text_color(Theme::text())))
                .on_click(cx.listener(move |e, _, _, cx| {
                    e.links.tab = Some(b);
                    cx.notify();
                }))
                .child(ui_text(format!("{name} ({n})")))
                .when(active, |d| d.child(div().absolute().left_0().right_0().bottom(px(-1.)).h(px(2.)).rounded(px(1.)).bg(Theme::primary())))
        };
        let help = div()
            .id("refs-help")
            .ml(px(4.))
            .child(icon("circle-help", 14., Theme::text_placeholder()))
            .tip_above(crate::settings::text("本文被以下文档所引用"), None);
        let close = div()
            .id("refs-hide")
            .size(px(24.))
            .rounded(px(ROW))
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .hover(|s| s.bg(Theme::fill_hover_strong()))
            .on_click(cx.listener(|_, _, _, cx| cx.emit(EditorEvent::HideRefs)))
            .child(icon("x", 16., Theme::text_caption()))
            .tip_above(crate::settings::text("隐藏引用列表"), None);
        let tabs = div()
            .flex()
            .items_center()
            .justify_between()
            .mr(px(20.))
            .border_b_1()
            .border_color(Theme::line_divider())
            .child(div().flex().items_center().child(tab("refs-tab-back", crate::settings::text("反向引用"), nb, back, true).child(help)).child(tab("refs-tab-refs", crate::settings::text("本文引用"), nr, !back, false)))
            .child(close);
        let mut list = div().py(px(12.)).pr(px(20.)).flex().flex_col();
        let docs = self.links.docs.clone();
        let font = window.text_style().font();
        let title_row = |id: String, row: usize, chevron: Option<bool>, cx: &mut Context<Self>| {
            let d = docs.iter().find(|d| d.id == id);
            let name = d.map(|d| display_title(&d.title).to_string()).unwrap_or_else(|| crate::settings::text("此文档已被删除").into());
            let glyph = match d.and_then(|d| d.icon.clone()) {
                Some(e) => div().mr(px(4.)).text_size(px(16.)).child(e).into_any_element(),
                None => div()
                    .relative()
                    .mr(px(4.))
                    .flex_none()
                    .size(px(16.))
                    .child(icon("doc-blue", 16., Theme::primary_500()).absolute())
                    .child(icon("doc-blue-lines", 16., Theme::bg_body()).absolute())
                    .into_any_element(),
            };
            let open = id.clone();
            div()
                .flex()
                .items_center()
                .py(px(3.))
                .when_some(chevron, |d, expanded| {
                    let key = id.clone();
                    d.child(
                        div()
                            .id(("refs-fold", row))
                            .size(px(20.))
                            .flex_none()
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(ROW))
                            .cursor_pointer()
                            .hover(|s| s.bg(Theme::fill_hover_strong()))
                            .on_click(cx.listener(move |e, _, _, cx| {
                                if !e.links.collapsed.remove(&key) {
                                    e.links.collapsed.insert(key.clone());
                                }
                                cx.notify();
                            }))
                            .child(icon(if expanded { "triangle-down" } else { "triangle-right" }, 12., Theme::text_caption()))
                            .tip_above(if expanded { crate::settings::text("收起") } else { crate::settings::text("展开") }, None),
                    )
                })
                .child(
                    // The document is a link: the link colour, a grey hover.
                    div()
                        .id(("refs-doc", row))
                        .min_w_0()
                        .h(px(26.))
                        .px(px(4.))
                        .rounded(px(ROW))
                        .flex()
                        .items_center()
                        .cursor_pointer()
                        .hover(|s| s.bg(Theme::fill_hover_strong()))
                        .active(|s| s.bg(Theme::fill_pressed()))
                        .on_click(cx.listener(move |_, _, _, cx| cx.emit(EditorEvent::OpenDoc(open.clone()))))
                        .child(glyph)
                        .child(div().min_w_0().truncate().text_size(px(15.)).text_color(Theme::link()).child(ui_text(name))),
                )
        };
        let (count, more_ix) = if back { (nb, 0) } else { (nr, 1) };
        let shown = if count > INIT_RENDER + 2 && !self.links.more[more_ix] { INIT_RENDER } else { count };
        if count == 0 {
            list = list.child(
                div()
                    .min_h(px(172.))
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .text_size(px(13.))
                    .line_height(px(20.))
                    .text_color(Theme::text_caption())
                    .child(ui_text(if back { crate::settings::text("本文暂未被其它文档引用") } else { crate::settings::text("本文暂未引用其它文档") })),
            );
        } else if back {
            let backlinks = self.links.backlinks.clone();
            for (row, b) in backlinks.into_iter().take(shown).enumerate() {
                let expanded = !self.links.collapsed.contains(&b.id);
                list = list.child(title_row(b.id.clone(), row, Some(expanded), cx));
                if !expanded {
                    continue;
                }
                let mut lines = div().ml(px(18.)).my(px(2.)).flex().flex_col();
                for (k, d) in b.lines.iter().enumerate() {
                    let (target, ordinal) = (b.id.clone(), d.block);
                    let mut text = String::new();
                    let mut hl = vec![];
                    for (seg, link) in &d.line {
                        let s = match link {
                            Some(id) => self.links.doc(id).map(|d| display_title(&d.title).to_string()).unwrap_or_else(|| seg.clone()),
                            None => seg.clone(),
                        };
                        let color = if link.is_some() { Theme::link() } else { Theme::text() };
                        hl.push((text.len()..text.len() + s.len(), color));
                        text.push_str(&s);
                    }
                    let runs: Vec<TextRun> = hl
                        .into_iter()
                        .filter(|(r, _)| !r.is_empty())
                        .map(|(r, color)| TextRun { len: r.len(), font: font.clone(), color, background_color: None, underline: None, strikethrough: None })
                        .collect();
                    lines = lines.child(
                        div()
                            .id(SharedString::from(format!("refs-line-{row}-{k}")))
                            .relative()
                            .text_size(px(13.))
                            .line_height(px(22.))
                            .pt(px(3.))
                            .pb(px(3.))
                            .pl(px(24.))
                            .pr(px(6.))
                            .rounded(px(ROW))
                            .cursor_pointer()
                            .hover(|s| s.bg(Theme::fill_hover_strong()))
                            .active(|s| s.bg(Theme::fill_pressed()))
                            .on_click(cx.listener(move |_, _, _, cx| cx.emit(EditorEvent::OpenDocAt(target.clone(), ordinal))))
                            .child(div().absolute().left(px(8.)).top(px(3.)).text_color(Theme::text_placeholder()).child("•"))
                            .child(StyledText::new(text).with_runs(runs)),
                    );
                }
                list = list.child(lines);
            }
        } else {
            for (row, id) in refs.into_iter().take(shown).enumerate() {
                list = list.child(div().h(px(32.)).flex().items_center().child(title_row(id, 1000 + row, None, cx)));
            }
        }
        if shown < count {
            list = list.child(
                div()
                    .id("refs-more")
                    .mt(px(4.))
                    .text_size(px(13.))
                    .line_height(px(20.))
                    .text_color(Theme::link())
                    .cursor_pointer()
                    .on_click(cx.listener(move |e, _, _, cx| {
                        e.links.more[more_ix] = true;
                        cx.notify();
                    }))
                    .child(ui_text(crate::settings::text("查看更多"))),
            );
        }
        // A quiet box: the light fill inside the note, crisp corners.
        Some(
            div()
                .id("refs")
                .mt(px(40.))
                .mb(px(40.))
                .pl(px(20.))
                .pt(px(8.))
                .rounded(px(CARD))
                .bg(Theme::bg_overlay())
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .child(tabs)
                .child(list)
                .into_any_element(),
        )
    }
}

#[cfg(test)]
mod wikilink_input_tests {
    use super::new_wikilink_trigger;

    #[test]
    fn cancelled_trigger_stays_text_when_typing_continues() {
        for trigger in ["[[", "【【", "［［"] {
            assert_eq!(new_wikilink_trigger(&format!("before {trigger}普通文字"), "普通文字"), None);
            assert_eq!(new_wikilink_trigger(&format!("{trigger}closed]]"), "]"), None);
        }
    }

    #[test]
    fn new_trigger_opens_at_unicode_byte_offset() {
        for trigger in ["[[", "【【", "［［"] {
            let prefix = format!("中文{trigger}");
            let last = trigger.chars().last().unwrap().to_string();
            assert_eq!(new_wikilink_trigger(&prefix, &last), Some(("中文".len(), trigger.len())));
            let composed = format!("{trigger}目标");
            assert_eq!(new_wikilink_trigger(&format!("前文{composed}"), &composed), Some(("前文".len(), trigger.len())));
            assert_eq!(new_wikilink_trigger(&format!("{trigger}cancelled {trigger}"), &last), Some((trigger.len() + "cancelled ".len(), trigger.len())));
        }
    }
}
