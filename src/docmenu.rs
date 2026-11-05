//! The header "…" menu: page width, find & replace, move, pin, star,
//! duplicate, 下载为 PDF / Markdown, 打印, document info and delete.

use crate::app::Workspace;
use crate::settings::display_title;
use crate::store::PageWidth;
use crate::theme::{self, Theme, craft};
use crate::ui::{icon, popover};
use gpui::{prelude::FluentBuilder, *};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DocSub {
    Width,
    Download,
    /// 更多: 显示引用列表.
    More,
}

const MENU_W: f32 = 200.;
/// Gap between the "…" button and the menu.
const MENU_GAP: f32 = 6.;
/// Rows: 32 tall; dividers 1px with 5px either side; 6px padding.
const ROW_H: f32 = crate::ui::MENU_ROW_H;
const DIV_H: f32 = 11.;
const PAD: f32 = crate::ui::MENU_PAD_Y;

/// Offset of item `k`'s row from the menu's top with `dividers` divider rows above it.
fn item_top(k: usize, dividers: usize) -> f32 {
    1. + PAD + ROW_H * k as f32 + DIV_H * dividers as f32
}

fn menu_item(id: &'static str, ic: &'static str, label: impl Into<SharedString>, sub: bool, open: bool) -> Stateful<Div> {
    let g = SharedString::from(id);
    crate::ui::menu_row(id, g.clone())
        .when(open, |d| d.bg(Theme::menu_highlight()).text_color(gpui::white()))
        .child(if open { icon(ic, 16., gpui::white()) } else { crate::ui::menu_glyph(ic, 16., g.clone()) })
        .child(div().flex_1().min_w_0().overflow_hidden().child(crate::ui::menu_label(label, g.clone(), gpui::white())))
        .when(sub, |d| d.child(if open { icon("chevron-right", 10., gpui::white()) } else { crate::ui::menu_glyph("chevron-right", 10., g) }))
}

/// 下载为's text-only items.
pub(crate) fn sub_item(id: &'static str, label: &'static str) -> Stateful<Div> {
    crate::ui::menu_row(id, SharedString::from(id)).mx(px(0.)).child(crate::ui::menu_label(label, SharedString::from(id), gpui::white()))
}

/// Native-menu separator: 1px, 5px either side, inset 13px.
fn divider() -> Div {
    div().flex_none().mx(px(13.)).my(px(5.)).h(px(1.)).bg(Theme::line_border())
}

impl Workspace {
    fn close_doc_menu(&mut self, cx: &mut Context<Self>) {
        self.doc_menu = false;
        self.doc_sub = None;
        cx.notify();
    }

    pub fn set_page_width(&mut self, width: PageWidth, cx: &mut Context<Self>) {
        let Some(id) = self.current.clone() else { return };
        if let Some(e) = self.entries.iter_mut().find(|e| e.id == id) {
            e.meta.width = width;
        }
        let _ = self.store.set_width(&id, width);
        if let Some(ed) = &self.editor {
            ed.update(cx, |e, cx| {
                e.page_width = width;
                cx.notify();
            });
        }
        cx.notify();
    }

    /// The document as it stands: the open editor's copy, or the file on disk.
    pub(crate) fn doc_of(&mut self, id: &str, cx: &mut Context<Self>) -> Option<crate::model::Document> {
        if self.current.as_deref() == Some(id) {
            self.flush(cx);
            return self.editor.as_ref().map(|ed| ed.read(cx).core.doc.clone());
        }
        self.store.load(id).ok()
    }

    /// 创建副本: "<标题> 副本", opened right away.
    pub fn duplicate_doc(&mut self, id: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.close_doc_menu(cx);
        self.row_menu = None;
        if let Some(new) = self.copy_doc(id, cx) {
            self.entries = self.store.list();
            self.sort();
            self.open(&new, window, cx);
        }
        cx.notify();
    }

    /// Write "<标题> 副本" of `id` to the library; the caller refreshes the list.
    pub(crate) fn copy_doc(&mut self, id: &str, cx: &mut Context<Self>) -> Option<String> {
        let mut doc = self.doc_of(id, cx)?;
        let title = crate::localized_format!("{} 副本", "{} copy", display_title(&doc.title.text));
        doc.title = crate::model::RichText::plain(title);
        let width = self.entries.iter().find(|e| e.id == id).map(|e| e.meta.width).unwrap_or_default();
        match self.store.create_from(&doc) {
            Ok(new) => {
                let _ = self.store.set_width(&new, width);
                Some(new)
            }
            Err(e) => {
                self.status = crate::app::SaveStatus::Failed(crate::localized_format!("创建副本失败：{}", "Could not duplicate document: {}", crate::settings::diagnostic(&e.to_string())));
                None
            }
        }
    }

    /// 查找和替换 (also ⌘F / ⌘⇧H).
    pub fn find_in_doc(&mut self, replace: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.close_doc_menu(cx);
        if let Some(ed) = &self.editor {
            ed.update(cx, |e, cx| e.open_find(replace, window, cx));
        }
    }

    pub(crate) fn render_doc_menu(&self, window: &Window, cx: &mut Context<Self>) -> Vec<AnyElement> {
        if !self.doc_menu {
            return vec![];
        }
        let Some(id) = self.current.clone() else { return vec![] };
        let entry = self.entries.iter().find(|e| e.id == id);
        let pinned = entry.is_some_and(|e| e.meta.pinned);
        let starred = entry.is_some_and(|e| e.meta.starred > 0);
        let width = entry.map(|e| e.meta.width).unwrap_or_default();
        let refs_shown = !entry.is_some_and(|e| e.meta.refs_hidden);
        let sub = self.doc_sub;
        let hover_sub = |s: Option<DocSub>| {
            cx.listener(move |w: &mut Workspace, hovered: &bool, _, cx| {
                if *hovered && w.doc_sub != s {
                    w.doc_sub = s;
                    cx.notify();
                }
            })
        };
        let sub_bounds = self.doc_sub_bounds.clone();
        let (pin_id, star_id, mv_id, del_id, dup_id, print_id) = (id.clone(), id.clone(), id.clone(), id.clone(), id.clone(), id.clone());
        let menu = popover()
            .id("doc-menu")
            .occlude()
            .w(px(MENU_W))
            .py(px(PAD))
            .flex()
            .flex_col()
            .shadow(theme::shadow_s4_down())
            .on_mouse_down_out(cx.listener(move |w, ev: &MouseDownEvent, _, cx| {
                if sub_bounds.get().is_none_or(|b| !b.contains(&ev.position)) {
                    w.close_doc_menu(cx);
                }
            }))
            .child(
                menu_item("dm-width", "page-width", crate::settings::text("页宽设置"), true, sub == Some(DocSub::Width))
                    .on_hover(hover_sub(Some(DocSub::Width))),
            )
            .child(divider())
            .child(
                menu_item("dm-find", "text-search", crate::settings::text("查找和替换"), false, false)
                    .on_hover(hover_sub(None))
                    .on_click(cx.listener(|w, _, window, cx| w.find_in_doc(false, window, cx))),
            )
            .child(divider())
            .child(
                menu_item("dm-move", "folder-input", crate::settings::text("移动到"), false, false)
                    .on_hover(hover_sub(None))
                    .on_click(cx.listener(move |w, _, window, cx| {
                        w.close_doc_menu(cx);
                        w.open_move(vec![mv_id.clone()], window, cx);
                    })),
            )
            .child(
                menu_item(
                    "dm-pin",
                    if pinned { "pin-off" } else { "pin" },
                    if pinned { crate::settings::text("从“置顶”移除") } else { crate::settings::text("添加到“置顶”") },
                    false,
                    false,
                )
                .on_hover(hover_sub(None))
                .on_click(cx.listener(move |w, _, _, cx| {
                    w.toggle_pin(&pin_id, cx);
                    w.close_doc_menu(cx);
                })),
            )
            .child(
                menu_item("dm-star", if starred { "star-off" } else { "star" }, if starred { crate::settings::text("取消收藏") } else { crate::settings::text("添加到收藏") }, false, false)
                    .on_hover(hover_sub(None))
                    .on_click(cx.listener(move |w, _, _, cx| w.toggle_star(&star_id, cx))),
            )
            .child(divider())
            .child(
                menu_item("dm-copy", "copy", crate::settings::text("创建副本"), false, false)
                    .on_hover(hover_sub(None))
                    .on_click(cx.listener(move |w, _, window, cx| w.duplicate_doc(&dup_id, window, cx))),
            )
            .child(
                menu_item("dm-download", "export", crate::settings::text("下载为"), true, sub == Some(DocSub::Download))
                    .on_hover(hover_sub(Some(DocSub::Download))),
            )
            .child(divider())
            .when(crate::printing::SUPPORTED, |d| d.child(
                menu_item("dm-print", "printer", crate::settings::text("打印"), false, false)
                    .on_hover(hover_sub(None))
                    .on_click(cx.listener(move |w, _, window, cx| w.print_doc(&print_id, window, cx))),
            ).child(divider()))
            .child(
                menu_item("dm-info", "info", crate::settings::text("文档信息"), false, false)
                    .on_hover(hover_sub(None))
                    .on_click(cx.listener(|w, _, window, cx| w.open_doc_info(window, cx))),
            )
            .child(divider())
            // 更多 and 删除 share the last group.
            .child(menu_item("dm-more", "ellipsis", crate::settings::text("更多"), true, sub == Some(DocSub::More)).on_hover(hover_sub(Some(DocSub::More))))
            .child(
                menu_item("dm-delete", "bin", crate::settings::text("删除"), false, false)
                    .on_hover(hover_sub(None))
                    .on_click(cx.listener(move |w, _, _, cx| w.ask_delete(&del_id, cx))),
            )
            .child(divider())
            .child(
                menu_item("dm-shortcuts", "keyboard", crate::settings::text("快捷键"), false, false)
                    .on_hover(hover_sub(None))
                    .on_click(cx.listener(|w, _, _, cx| {
                        w.close_doc_menu(cx);
                        w.shortcuts = true;
                        cx.notify();
                    })),
            );
        let vw = window.viewport_size().width;
        let btn = self.more_btn_bounds.get();
        let right = btn.map_or(vw - px(120.), |b| b.right());
        let menu_top = btn.map_or(px(60.), |b| b.bottom() + px(MENU_GAP));
        let mut out = vec![
            deferred(anchored().anchor(Corner::TopRight).position(point(right, menu_top)).child(menu))
                .with_priority(2)
                .into_any_element(),
        ];
        // Submenus open to the left, 4px from the menu, level with their item.
        let sub_at = point(right - px(MENU_W + 4.), px(0.));
        let item_top = |k: usize, d: usize| f32::from(menu_top) + item_top(k, d);
        let record = self.doc_sub_bounds.clone();
        record.set(None);
        let recorder = move || {
            let record = record.clone();
            canvas(move |b, _, _| record.set(Some(b)), |_, _, _, _| {}).absolute().inset_0()
        };
        let sub_el = match sub {
            Some(DocSub::Width) => Some((item_top(0, 0) - 1. - PAD, self.width_panel(width, cx).child(recorder()).into_any_element())),
            // Level the submenu's first item with 下载为.
            Some(DocSub::Download) => Some((
                item_top(6, 3) - 1. - PAD,
                {
                    let pdf = id.clone();
                    popover()
                        .id("dm-download-menu")
                        .occlude()
                        .w(px(130.))
                        .px(px(5.))
                        .py(px(PAD))
                        .child(recorder())
                        .when(crate::printing::SUPPORTED, |d| d.child(sub_item("dm-pdf", "PDF").on_click(cx.listener(move |w, _, window, cx| w.export_pdf(&pdf, window, cx)))))
                        .child(sub_item("dm-md", "Markdown").on_click(cx.listener(move |w, _, _, cx| w.open_export_md(&id, cx))))
                        .into_any_element()
                },
            )),
            Some(DocSub::More) => Some((
                if crate::printing::SUPPORTED { item_top(9, 6) } else { item_top(8, 5) } - 1. - PAD,
                popover()
                    .id("dm-more-menu")
                    .occlude()
                    .w(px(210.))
                    .px(px(5.))
                    .py(px(PAD))
                    .child(recorder())
                    .child(
                        sub_item("dm-refs", crate::settings::text("显示引用列表"))
                            .justify_between()
                            .child(
                                div()
                                    .flex_none()
                                    .w(px(28.))
                                    .h(px(16.))
                                    .rounded_full()
                                    .p(px(2.))
                                    .flex()
                                    .when(refs_shown, |d| d.justify_end())
                                    .bg(if refs_shown { Theme::primary_500() } else { Theme::block_placeholder() })
                                    .child(div().size(px(12.)).rounded_full().bg(crate::theme::raised(Theme::bg_body()))),
                            )
                            .on_click(cx.listener(move |w, _, _, cx| {
                                w.close_doc_menu(cx);
                                w.ask_refs(!refs_shown, cx);
                            })),
                    )
                    .into_any_element(),
            )),
            None => None,
        };
        if let Some((top, el)) = sub_el {
            out.push(
                deferred(anchored().anchor(Corner::TopRight).position(point(sub_at.x, px(top))).snap_to_window_with_margin(px(8.)).child(el))
                    .with_priority(2)
                    .into_any_element(),
            );
        }
        out
    }

    /// 为当前窗口选择合适页宽: 默认 / 较宽 / 全宽 cards.
    fn width_panel(&self, current: PageWidth, cx: &mut Context<Self>) -> Stateful<Div> {
        let card = |id: &'static str, label: &'static str, w: PageWidth, line: f32| {
            let on = w == current;
            let line_color = if on { Theme::primary() } else { theme::c(craft::N400) };
            div()
                .id(id)
                .w(px(56.))
                .h(px(66.))
                .p(px(4.))
                .flex()
                .flex_col()
                .items_center()
                .rounded(px(9.))
                .border_1()
                .border_color(if on { Theme::primary() } else { Theme::line_border() })
                .bg(crate::theme::raised(Theme::bg_body()))
                .cursor_pointer()
                .when(!on, |d| d.hover(|s| s.border_color(Theme::primary())))
                .child(
                    div()
                        .w(px(46.))
                        .h(px(28.))
                        .pt(px(7.))
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(2.))
                        .rounded(px(2.))
                        .bg(if on { theme::c(craft::B100) } else { theme::c(craft::N100) })
                        .children((0..4).map(move |_| div().w(px(line)).h(px(2.)).flex_none().bg(line_color))),
                )
                .child(
                    div()
                        .mt(px(6.))
                        .text_size(px(12.))
                        .line_height(px(18.))
                        .text_color(if on { Theme::primary() } else { Theme::text() })
                        .child(crate::settings::label(label)),
                )
                .on_click(cx.listener(move |ws, _, _, cx| ws.set_page_width(w, cx)))
        };
        popover()
            .id("dm-width-panel")
            .occlude()
            .relative()
            .w(px(238.))
            .h(px(136.))
            .child(
                div()
                    .absolute()
                    .left(px(19.))
                    .top(px(14.))
                    .text_size(px(12.))
                    .line_height(px(18.))
                    .text_color(Theme::text_caption())
                    .child(crate::settings::text("为当前窗口选择合适页宽")),
            )
            .child(
                div()
                    .absolute()
                    .left(px(19.))
                    .top(px(47.))
                    .flex()
                    .gap(px(16.))
                    .child(card("pw-standard", crate::settings::text("默认"), PageWidth::Standard, 20.))
                    .child(card("pw-wide", crate::settings::text("较宽"), PageWidth::Wide, 28.))
                    .child(card("pw-full", crate::settings::text("全宽"), PageWidth::Full, 38.)),
            )
    }
}
