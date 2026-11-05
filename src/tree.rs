//! 我的文档 as a page tree: parent links kept in the
//! index, the sidebar's expand state, 新建子页面, 移动到 and its dialog, and
//! the ancestor chain used by the header breadcrumbs.

use crate::app::Workspace;
use crate::settings::display_title;
use crate::editor::view::SubPage;
use crate::input::{TextField, TextFieldEvent};
use crate::store::Entry;
use crate::theme::{self, Theme, craft};
use crate::trash::{close_button, dialog, primary_button, scrim, secondary_button};
use crate::ui::{ROW_RADIUS, icon, ui_text};
use gpui::{prelude::FluentBuilder, *};
use std::collections::{HashMap, HashSet};

/// Logical library paths for link completion, distinct from physical XML filenames.
pub(crate) fn document_paths(entries: &[Entry]) -> HashMap<String, String> {
    let by_id: HashMap<&str, &Entry> = entries.iter().map(|e| (e.id.as_str(), e)).collect();
    let mut out = HashMap::new();
    for entry in entries {
        let mut labels = vec![display_title(&entry.title).to_string()];
        let mut seen = HashSet::from([entry.id.as_str()]);
        let mut parent = entry.meta.parent.as_deref();
        while let Some(e) = parent.and_then(|id| by_id.get(id)) {
            if !seen.insert(e.id.as_str()) { break }
            labels.push(display_title(&e.title).to_string());
            parent = e.meta.parent.as_deref();
        }
        labels.reverse();
        out.insert(entry.id.clone(), format!("/{}", labels.join(" / ")));
    }
    let mut counts = HashMap::new();
    for path in out.values() { *counts.entry(path.clone()).or_insert(0) += 1; }
    // IDs appear only when even the full logical paths are identical.
    for (id, path) in &mut out {
        if counts.get(path).is_some_and(|n| *n > 1) {
            let suffix: String = id.chars().rev().take(8).collect::<String>().chars().rev().collect();
            path.push_str(&format!(" · {suffix}"));
        }
    }
    out
}

/// Children of every page, keyed by parent (`None`: the top level, in list
/// order); siblings oldest first. A page whose parent is gone, or that sits
/// in a parent cycle, is shown at the top level.
pub(crate) struct Tree<'a> {
    roots: Vec<&'a Entry>,
    kids: HashMap<&'a str, Vec<&'a Entry>>,
}

impl<'a> Tree<'a> {
    pub fn new(entries: &'a [Entry]) -> Self {
        let ids: HashSet<&str> = entries.iter().map(|e| e.id.as_str()).collect();
        let parent = |e: &'a Entry| e.meta.parent.as_deref().filter(|p| ids.contains(p) && *p != e.id);
        let mut roots: Vec<&Entry> = vec![];
        let mut kids: HashMap<&str, Vec<&Entry>> = HashMap::new();
        for e in entries {
            match parent(e) {
                Some(p) => kids.entry(p).or_default().push(e),
                None => roots.push(e),
            }
        }
        // Lift pages unreachable from the top level (cycles).
        let mut seen: HashSet<&str> = HashSet::new();
        let mut stack: Vec<&str> = roots.iter().map(|e| e.id.as_str()).collect();
        while let Some(id) = stack.pop() {
            if seen.insert(id) {
                stack.extend(kids.get(id).map_or(vec![], |v| v.iter().map(|e| e.id.as_str()).collect()));
            }
        }
        for e in entries {
            if !seen.contains(e.id.as_str()) {
                seen.insert(&e.id);
                if let Some(v) = parent(e).and_then(|p| kids.get_mut(p)) {
                    v.retain(|x| x.id != e.id);
                }
                roots.push(e);
            }
        }
        for v in kids.values_mut() {
            v.sort_by_key(|e| e.meta.sibling_key());
        }
        Tree { roots, kids }
    }

    pub fn children(&self, id: Option<&str>) -> &[&'a Entry] {
        match id {
            None => &self.roots,
            Some(id) => self.kids.get(id).map_or(&[], |v| v.as_slice()),
        }
    }

    pub fn has_children(&self, id: &str) -> bool {
        !self.children(Some(id)).is_empty()
    }

    /// Pre-order rows with their depth, descending into pages `open` says yes to.
    pub fn rows(&self, open: impl Fn(&str) -> bool) -> Vec<(&'a Entry, usize)> {
        let mut out = vec![];
        let mut stack: Vec<(&Entry, usize)> = self.children(None).iter().rev().map(|e| (*e, 0)).collect();
        while let Some((e, d)) = stack.pop() {
            out.push((e, d));
            if open(&e.id) {
                stack.extend(self.children(Some(&e.id)).iter().rev().map(|c| (*c, d + 1)));
            }
        }
        out
    }

    /// `id` and every page under it.
    pub fn subtree(&self, id: &str) -> Vec<String> {
        let mut out = vec![id.to_string()];
        let mut i = 0;
        while i < out.len() {
            let kids: Vec<String> = self.children(Some(&out[i])).iter().map(|e| e.id.clone()).collect();
            out.extend(kids);
            i += 1;
        }
        out
    }
}

/// The 移动到 dialog.
pub struct MoveDialog {
    /// Pages being moved (each with its subtree).
    pub ids: Vec<String>,
    /// Chosen destination: `Some(None)` is the top level of 我的文档.
    pub target: Option<Option<String>>,
    pub expanded: HashSet<String>,
    pub search: Entity<TextField>,
    pub scroll: ScrollHandle,
    _subs: Vec<Subscription>,
}

impl Workspace {
    /// Ancestors of `id`, top level first.
    pub(crate) fn ancestors(&self, id: &str) -> Vec<&Entry> {
        let mut out: Vec<&Entry> = vec![];
        let mut cur = self.entries.iter().find(|e| e.id == id);
        while let Some(p) = cur.and_then(|e| e.meta.parent.as_deref()).and_then(|p| self.entries.iter().find(|e| e.id == p)) {
            if p.id == id || out.iter().any(|e| e.id == p.id) {
                break;
            }
            out.push(p);
            cur = Some(p);
        }
        out.reverse();
        out
    }

    /// Expand the sidebar down to `id`.
    pub(crate) fn reveal_in_tree(&mut self, id: &str) {
        let ids: Vec<String> = self.ancestors(id).iter().map(|e| e.id.clone()).collect();
        self.expanded.extend(ids);
    }

    /// Hand the open document its child pages (for 子页面列表 blocks).
    pub(crate) fn sync_sub_pages(&mut self, cx: &mut Context<Self>) {
        let (Some(id), Some(ed)) = (self.current.clone(), self.editor.clone()) else { return };
        let pages = Tree::new(&self.entries)
            .children(Some(&id))
            .iter()
            .map(|e| SubPage { id: e.id.clone(), title: e.title.clone(), icon: e.icon.clone() })
            .collect();
        ed.update(cx, |e, cx| e.set_sub_pages(pages, cx));
        self.sync_links(cx);
    }

    /// The row "+" → 文档: a new page under `parent`, opened.
    pub fn new_child(&mut self, parent: &str, window: &mut Window, cx: &mut Context<Self>) {
        let p = parent.to_string();
        if !self.save_before(cx, move |w, window, cx| w.new_child(&p, window, cx)) {
            return;
        }
        let made = self.store.create().and_then(|(id, _)| self.store.set_parent(&id, Some(parent)).map(|_| id));
        match made {
            Ok(id) => {
                self.entries = self.store.list();
                self.sort();
                self.expanded.insert(parent.to_string());
                self.open(&id, window, cx);
            }
            Err(e) => self.status = crate::app::SaveStatus::Failed(crate::localized_format!("新建失败：{}", "Could not create document: {}", crate::settings::diagnostic(&e.to_string()))),
        }
        cx.notify();
    }

    pub(crate) fn open_move(&mut self, ids: Vec<String>, window: &mut Window, cx: &mut Context<Self>) {
        self.row_menu = None;
        let search = cx.new(|cx| {
            let mut f = TextField::new("搜索目标位置", cx);
            f.placeholder_color = Theme::text_placeholder();
            f
        });
        let subs = vec![cx.subscribe_in(&search, window, |w, _, ev: &TextFieldEvent, window, cx| {
            if let TextFieldEvent::Cancel = ev {
                w.close_move(window, cx);
            }
            cx.notify();
        })];
        window.focus(&search.focus_handle(cx));
        self.move_dlg = Some(MoveDialog { ids, target: None, expanded: HashSet::new(), search, scroll: ScrollHandle::new(), _subs: subs });
        cx.notify();
    }

    fn close_move(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.move_dlg = None;
        if let Some(ed) = &self.editor {
            ed.read(cx).focus(window);
        }
        cx.notify();
    }

    /// 移动到: hang each page under `target` (`None`: top level), skipping
    /// moves into its own subtree.
    pub(crate) fn move_docs(&mut self, ids: &[String], target: Option<String>, cx: &mut Context<Self>) {
        for id in ids {
            let inside = target.as_ref().is_some_and(|t| Tree::new(&self.entries).subtree(id).contains(t));
            if inside {
                continue;
            }
            match self.store.set_parent(id, target.as_deref()) {
                Ok(meta) => {
                    if let Some(e) = self.entries.iter_mut().find(|e| &e.id == id) {
                        e.meta = meta;
                    }
                }
                Err(e) => self.status = crate::app::SaveStatus::Failed(crate::localized_format!("移动失败：{}", "Could not move document: {}", crate::settings::diagnostic(&e.to_string()))),
            }
        }
        if let Some(t) = &target {
            self.expanded.insert(t.clone());
            self.reveal_in_tree(&t.clone());
        }
        self.sync_sub_pages(cx);
        self.show_toast(crate::settings::text("移动成功"), cx);
        cx.notify();
    }

    /// The 移动到 dialog: 我的文档 on the left, its page tree on the right.
    /// Quiet selection fills, a 30pt search field, the modal buttons.
    pub(crate) fn render_move_dialog(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let dlg = self.move_dlg.as_ref()?;
        let tree = Tree::new(&self.entries);
        let hidden: HashSet<String> = dlg.ids.iter().flat_map(|id| tree.subtree(id)).collect();
        let q = dlg.search.read(cx).text.trim().to_lowercase();
        let border = Theme::line_border();

        let search = div()
            .mt(px(12.))
            .mx(px(8.))
            .mb(px(10.))
            .h(px(30.))
            .flex_none()
            .pl(px(10.))
            .pr(px(8.))
            .flex()
            .items_center()
            .rounded(px(9.))
            .border_1()
            .border_color(Theme::line_border())
            .bg(crate::theme::raised(Theme::bg_body()))
            .child(icon("search", 15., Theme::text_caption()))
            .child(div().flex_1().min_w_0().ml(px(6.)).flex().child(dlg.search.clone()));
        // The location column reads like a sidebar: a grey surface and
        // a quiet pill for the selected location.
        let left = div()
            .w(px(240.))
            .flex_none()
            .flex()
            .flex_col()
            .bg(Theme::bg_overlay())
            .rounded_l(px(9.))
            .child(search)
            .child(
                div()
                    .mx(px(8.))
                    .h(px(30.))
                    .pl(px(10.))
                    .pr(px(10.))
                    .flex()
                    .items_center()
                    .rounded(px(9.))
                    .bg(Theme::fill_hover_strong())
                    .child(icon("library", 15., Theme::text_code()))
                    .child(div().ml(px(8.)).text_size(px(13.)).line_height(px(20.)).child(crate::settings::tr("我的文档", "My documents"))),
            );

        let row = |key: String, selected: bool| {
            div()
                .id(SharedString::from(key))
                .h(px(30.))
                .mb(px(2.))
                .pr(px(5.))
                .flex()
                .items_center()
                .rounded(px(ROW_RADIUS))
                .cursor_pointer()
                .when(selected, |d| d.bg(Theme::fill_hover_strong()))
                .when(!selected, |d| d.hover(|s| s.bg(Theme::fill_hover())))
        };
        let title = |t: String| div().ml(px(9.)).flex_1().min_w_0().text_size(px(13.)).line_height(px(20.)).overflow_hidden().whitespace_nowrap().text_ellipsis().child(ui_text(t));
        let page_icon = |e: &Entry| match &e.icon {
            Some(em) => div().size(px(16.)).flex_none().flex().items_center().justify_center().text_size(px(14.)).child(em.clone()).into_any_element(),
            None => icon("doc", 16., Theme::text_caption()).into_any_element(),
        };
        let mut list = div().id("move-tree").flex_1().min_w_0().px(px(8.)).py(px(12.)).overflow_y_scroll().track_scroll(&dlg.scroll);
        if q.is_empty() {
            list = list.child(
                row("move-root".into(), dlg.target == Some(None))
                    .pl(px(16.))
                    .child(icon("library", 16., Theme::text_caption()))
                    .child(title(crate::settings::text("我的文档").into()))
                    .on_click(cx.listener(|w, _, _, cx| {
                        if let Some(d) = w.move_dlg.as_mut() {
                            d.target = Some(None);
                        }
                        cx.notify();
                    })),
            );
            for (e, depth) in tree.rows(|id| dlg.expanded.contains(id)) {
                if hidden.contains(&e.id) {
                    continue;
                }
                let kids = tree.children(Some(&e.id)).iter().any(|c| !hidden.contains(&c.id));
                let open = dlg.expanded.contains(&e.id);
                let (pick, toggle) = (e.id.clone(), e.id.clone());
                list = list.child(
                    row(format!("move-{}", e.id), dlg.target.as_ref().is_some_and(|t| t.as_deref() == Some(e.id.as_str())))
                        .pl(px(16. + 20. * depth as f32))
                        .child(
                            div()
                                .id(SharedString::from(format!("move-arrow-{}", e.id)))
                                .size(px(20.))
                                .p(px(5.))
                                .mr(px(8.))
                                .flex_none()
                                .rounded(px(5.))
                                .when(kids, |d| {
                                    d.hover(|s| s.bg(Theme::fill_hover_strong()))
                                        .child(icon("tree-triangle", 10., theme::c(craft::N500)).when(open, |s| s.with_transformation(Transformation::rotate(radians(std::f32::consts::FRAC_PI_2)))))
                                        .on_click(cx.listener(move |w, _, _, cx| {
                                            cx.stop_propagation();
                                            if let Some(d) = w.move_dlg.as_mut()
                                                && !d.expanded.remove(&toggle) {
                                                    d.expanded.insert(toggle.clone());
                                                }
                                            cx.notify();
                                        }))
                                }),
                        )
                        .child(page_icon(e))
                        .child(title(display_title(&e.title)))
                        .on_click(cx.listener(move |w, _, _, cx| {
                            if let Some(d) = w.move_dlg.as_mut() {
                                d.target = Some(Some(pick.clone()));
                            }
                            cx.notify();
                        })),
                );
            }
        } else {
            let hits: Vec<&Entry> = self.entries.iter().filter(|e| !hidden.contains(&e.id) && display_title(&e.title).to_lowercase().contains(&q)).collect();
            if hits.is_empty() {
                list = list.child(div().pt(px(80.)).flex().justify_center().text_size(px(13.)).text_color(Theme::text_caption()).child(crate::settings::text("无搜索结果")));
            }
            for e in hits {
                let pick = e.id.clone();
                list = list.child(
                    row(format!("move-hit-{}", e.id), dlg.target.as_ref().is_some_and(|t| t.as_deref() == Some(e.id.as_str())))
                        .pl(px(16.))
                        .child(page_icon(e))
                        .child(title(display_title(&e.title)))
                        .on_click(cx.listener(move |w, _, _, cx| {
                            if let Some(d) = w.move_dlg.as_mut() {
                                d.target = Some(Some(pick.clone()));
                            }
                            cx.notify();
                        })),
                );
            }
        }

        let ready = dlg.target.is_some();
        // 确定 waits for a target: the accent, faded, until one is picked.
        let ok = primary_button("move-ok", crate::settings::text("确定"));
        let ok = if ready {
            ok
        } else {
            let faded = theme::ca(craft::B500, 0.4);
            ok.bg(faded).cursor_default().hover(move |s| s.bg(faded)).active(move |s| s.bg(faded))
        };
        let dialog = dialog("move-dialog")
            .relative()
            .w(px(828.))
            .flex()
            .flex_col()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(div().pt(px(24.)).pr(px(56.)).pl(px(24.)).text_size(px(15.)).line_height(px(24.)).font_weight(FontWeight::SEMIBOLD).child(crate::settings::text("从 我的文档 移动到")))
            .child(close_button("move-close").absolute().top(px(27.)).right(px(25.)).on_click(cx.listener(|w, _, window, cx| w.close_move(window, cx))))
            .child(
                div().px(px(24.)).mt(px(16.)).mb(px(24.)).child(
                    div()
                        .h(px(512.))
                        .flex()
                        .border_1()
                        .border_color(border)
                        .rounded(px(10.))
                        .child(left)
                        .child(div().w(px(1.)).h_full().flex_none().bg(border))
                        .child(list),
                ),
            )
            .child(
                div()
                    .px(px(24.))
                    .pb(px(24.))
                    .flex()
                    .justify_end()
                    .gap(px(12.))
                    .child(secondary_button("move-cancel", crate::settings::text("取消")).on_click(cx.listener(|w, _, window, cx| w.close_move(window, cx))))
                    .child(ok.on_click(cx.listener(|w, _, window, cx| {
                        let Some(d) = w.move_dlg.as_ref() else { return };
                        let Some(target) = d.target.clone() else { return };
                        let ids = d.ids.clone();
                        w.batch = None;
                        w.move_docs(&ids, target, cx);
                        w.close_move(window, cx);
                    }))),
            );
        Some(
            deferred(
                div()
                    .id("move-mask")
                    .absolute()
                    .inset_0()
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(scrim())
                    .on_mouse_down(MouseButton::Left, cx.listener(|w, _, window, cx| w.close_move(window, cx)))
                    .child(dialog),
            )
            .with_priority(3)
            .into_any_element(),
        )
    }
}

#[cfg(test)]
mod link_path_tests {
    use super::document_paths;
    use crate::store::Entry;
    fn entry(id: &str, title: &str, parent: Option<&str>) -> Entry {
        Entry { id: id.into(), title: title.into(), meta: crate::store::Meta { parent: parent.map(str::to_string), ..Default::default() }, icon: None, body: String::new(), reminders: vec![], links: vec![] }
    }
    #[test]
    fn same_title_in_different_parents_has_useful_paths() {
        let entries = vec![entry("a", "工作", None), entry("b", "学习", None), entry("x", "LLM", Some("a")), entry("y", "LLM", Some("b"))];
        let paths = document_paths(&entries);
        assert_eq!(paths["x"], "/工作 / LLM");
        assert_eq!(paths["y"], "/学习 / LLM");
    }
    #[test]
    fn malformed_cycles_terminate_and_duplicate_paths_are_distinct() {
        let entries = vec![entry("循环甲", "甲", Some("循环乙")), entry("循环乙", "乙", Some("循环甲")), entry("中文标识一", "同名", None), entry("中文标识二", "同名", None)];
        let paths = document_paths(&entries);
        assert_eq!(paths.len(), 4);
        assert_ne!(paths["中文标识一"], paths["中文标识二"]);
        assert!(paths["循环甲"].contains("甲"));
    }
}
