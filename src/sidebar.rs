//! Left pane: document list with search. Rows, sections and footer, sunken:
//! no panel, border or shadow, straight on the window surface (only the note
//! floats). 30pt rows on a 31pt pitch, selection pill radius 9.

use crate::app::{Confirm, RowMenu, View, Workspace};
use crate::settings::display_title;
use crate::store::Entry;
use crate::theme::{self, Theme, craft};
use crate::ui::{TipExt, TitleBarExt, icon, popover, ui_text};
use gpui::{prelude::FluentBuilder, *};

/// Row "…" menu geometry: 32pt rows, 11pt dividers, 6pt padding inside
/// a 1pt border.
const ROW_MENU_W: f32 = 180.;
const ROW_ITEM: f32 = crate::ui::MENU_ROW_H;
const ROW_DIVIDER: f32 = 11.;

fn row_menu_height(items: usize, dividers: usize) -> f32 {
    2. + crate::ui::MENU_PAD_Y * 2. + items as f32 * ROW_ITEM + dividers as f32 * ROW_DIVIDER
}

/// Sidebar geometry (measured from the window edge), shared by the
/// document sidebar and 设置's: selection pills start 14pt in, every row is
/// 30pt with radius 9 on a 31pt pitch, glyphs (18pt) centre at 37pt and
/// labels start at 54pt; groups are 16pt apart, a section header is 34pt
/// (6 above its 28pt label).
pub(crate) const RAIL_W: f32 = 44.;

pub(crate) const SB_X: f32 = 14.;
pub(crate) const ROW_H: f32 = 30.;
const DOC_ROW_H: f32 = ROW_H;
pub(crate) const ROW_R: f32 = 9.;
pub(crate) const ROW_GAP: f32 = 1.;
pub(crate) const ROW_ICON: f32 = 18.;
pub(crate) const GROUP_GAP: f32 = 16.;
pub(crate) const SECTION_H: f32 = 34.;
pub(crate) const SECTION_TOP: f32 = 6.;

/// 批量操作 caps at 50 pages.
pub(crate) const BATCH_LIMIT: usize = 50;

/// The footer row: like 主页 (same pill, glyph and label columns) under
/// the footer line, so the bottom corner mirrors the top.
pub(crate) fn footer_row(id: &'static str, glyph: &'static str, label: &'static str) -> Stateful<Div> {
    div()
        .id(id)
        .h(px(ROW_H))
        .pl(px(6.))
        .pr(px(4.))
        .flex()
        .items_center()
        .rounded(px(ROW_R))
        .cursor_pointer()
        .text_size(px(13.))
        .line_height(px(20.))
        .text_color(Theme::sidebar_text())
        .hover(|s| s.bg(Theme::sidebar_hover()))
        .child(div().size(px(20.)).flex_none().flex().items_center().justify_center().mr(px(9.)).child(icon(glyph, ROW_ICON, Theme::icon())))
        .child(label)
}

/// The footer's divider and padding around `row`.
pub(crate) fn footer_slot(row: impl IntoElement) -> Div {
    div().pt(px(6.)).pb(px(10.)).px(px(SB_X)).child(row)
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Check {
    Off,
    Partial,
    On,
}

/// 14px checkbox; an unchecked box turns blue when `group` is hovered.
pub(crate) fn checkbox(state: Check, disabled: bool, group: &'static str) -> Div {
    let white = Theme::bg_body();
    div()
        .size(px(14.))
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(4.))
        .when(state != Check::Off, |d| d.bg(Theme::primary()))
        .when(state == Check::Off, |d| {
            d.border_1()
                .when(disabled, |d| d.border_color(Theme::line_border()).bg(theme::c(craft::N100)))
                .when(!disabled, |d| d.border_color(theme::c(craft::N500)).bg(white).group_hover(group, |s| s.border_color(Theme::primary())))
        })
        .when(state == Check::On, |d| d.child(icon("todo-check", 12., white)))
        .when(state == Check::Partial, |d| d.child(div().w(px(8.)).h(px(2.)).rounded(px(1.)).bg(white)))
}

impl Workspace {
    pub(crate) fn render_tool_rail(&self, cx: &mut Context<Self>) -> AnyElement {
        let tool = |id, name: &str, label| div().id(id).size(px(32.)).rounded(px(8.))
            .flex().items_center().justify_center().cursor_pointer().hover(|s| s.bg(Theme::sidebar_hover()))
            .tip(label, None).child(icon(name, 18., Theme::icon()));
        div().w(px(RAIL_W)).flex_none().h_full().pt(px(crate::app::HEADER_H + 8.)).pb(px(10.))
            .flex().flex_col().items_center().gap(px(8.))
            .child(tool("rail-search", "search", crate::settings::tr("搜索", "Search")).on_click(cx.listener(|w, _, window, cx| w.open_search(window, cx))))
            .child(tool("rail-sidebar", "panel-left-close", crate::settings::tr("显示/隐藏导航", "Toggle navigation")).on_click(cx.listener(|w, _, _, cx| { w.sidebar_open = !w.sidebar_open; cx.notify(); })))
            .child(div().flex_1())
            .child(tool("rail-vault", "folder-input", crate::settings::tr("文档库", "Vaults")).on_click(cx.listener(|w, _, _, cx| w.open_settings(crate::settings::Page::Library, cx))))
            .child(tool("rail-settings", "settings-gear", crate::settings::tr("设置", "Settings")).on_click(cx.listener(|w, _, _, cx| w.show_view(View::Settings, cx))))
            .into_any_element()
    }

    /// Sidebar width for the current window: the dragged width, kept between
    /// 268 and min(800, window − 540).
    pub(crate) fn sidebar_w(&self, window: &Window) -> f32 {
        let vw = f32::from(window.viewport_size().width);
        let max = theme::SIDEBAR_MAX.min(vw - RAIL_W - theme::SIDEBAR_MIN_CONTENT).max(theme::SIDEBAR_MIN);
        self.sidebar_width.clamp(theme::SIDEBAR_MIN, max)
    }

    pub(crate) fn render_sidebar(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let entries: Vec<Entry> = self.entries.clone();
        self.more_bounds.borrow_mut().clear();

        let header = div()
            .h(px(crate::app::HEADER_H))
            .flex_none()
            .flex()
            .items_center()
            .pl(px((theme::TRAFFIC_LIGHTS_W - RAIL_W).max(0.)))
            .pr(px(12.))
            .title_bar()
            .when(cfg!(target_os = "windows"), |d| d.child(self.header_navigation(cx)))
            .when(!cfg!(target_os = "windows"), |d| d.child(
                div()
                    .id("collapse-sidebar")
                    .no_drag()
                    .flex_none()
                    .ml(px(4.))
                    .size(px(28.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(8.))
                    .cursor_pointer()
                    .hover(|s| s.bg(Theme::sidebar_hover()))
                    .tip(crate::settings::tr("收起侧栏", "Hide sidebar"), Some(crate::shortcuts::hint("workspace::ToggleSidebar", "⌘ + ]")))
                    .child(icon("panel-left-close", 17., Theme::icon()))
                    .on_click(cx.listener(|w, _, _, cx| {
                        w.sidebar_open = false;
                        cx.notify();
                    })),
            ))
            .child(div().flex_1())
            .children(crate::settings::update_pill(cx));

        // The nav is just 主页: 36px items, a blue tint when selected. Its
        // icon column (x 17) is shared with 搜索 and the doc rows.
        let home_on = self.view == View::Home;
        let nav = div().flex_none().pt(px(8.)).pb(px(4.)).px(px(SB_X)).child(
            div()
                .id("nav-home")
                .h(px(ROW_H))
                .pl(px(6.))
                .pr(px(4.))
                .flex()
                .items_center()
                .rounded(px(ROW_R))
                .cursor_pointer()
                .text_size(px(13.))
                .line_height(px(20.))
                .when(home_on, |d| d.bg(Theme::sidebar_selected()).text_color(Theme::text()))
                .when(!home_on, |d| d.text_color(Theme::sidebar_text()).hover(|s| s.bg(Theme::sidebar_hover())))
                .child(
                    div().size(px(20.)).flex_none().flex().items_center().justify_center().mr(px(9.)).child(icon(
                        if home_on { "home-filled" } else { "home" },
                        ROW_ICON,
                        if home_on { Theme::text_code() } else { Theme::icon() },
                    )),
                )
                .child(crate::settings::tr("主页", "Home"))
                .on_click(cx.listener(|w, _, _, cx| w.show_view(View::Home, cx))),
        );

        let mut list = div().flex().flex_col().px(px(SB_X)).pb(px(8.));
        // Pinned docs are shortcuts: they stay in 我的文档 too.
        let pinned: Vec<&Entry> = entries.iter().filter(|e| e.meta.pinned).collect();
        if !pinned.is_empty() {
            list = list.child(self.section(0, crate::settings::text("置顶"), cx));
            if !self.folded[0] {
                for e in pinned {
                    list = list.child(self.doc_row(e, "pinned", 0, false, cx));
                }
            }
            list = list.child(div().h(px(GROUP_GAP - SECTION_TOP)));
        }
        list = list.child(self.section(1, crate::settings::text("我的文档"), cx));
        if !self.folded[1] {
            let tree = crate::tree::Tree::new(&entries);
            for (e, depth) in tree.rows(|id| self.expanded.contains(id)) {
                list = list.child(self.doc_row(e, "doc", depth, tree.has_children(&e.id), cx));
            }
        }

        // The footer holds one row: 设置 (存储管理 and 回收站 live in 设置).
        // The 批量操作 toolbar keeps the footer glyphs: bare on the sidebar, a rounded fill on hover.
        let footer_btn = |id: &'static str, name: &'static str, tip: &'static str, enabled: bool, on: bool| {
            let group = SharedString::from(format!("{id}-slot"));
            div()
                .id(id)
                .flex_none()
                .w(px(36.))
                .h(px(32.))
                .group(group.clone())
                .when(enabled, |d| d.cursor_pointer())
                .tip(tip, None)
                .child(
                    div()
                        .size_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded(px(8.))
                        .when(on, |d| d.bg(Theme::sidebar_selected()))
                        .when(enabled && !on, |d| d.group_hover(group, |s| s.bg(Theme::sidebar_hover())))
                        .child(match (on, enabled) {
                            (true, _) => icon(&format!("{name}-filled"), 17., Theme::text()),
                            (false, true) => icon(name, 17., Theme::text_code()),
                            (false, false) => icon(name, 17., Theme::text_disabled()),
                        }),
                )
        };
        let some = self.batch.as_ref().is_some_and(|b| !b.is_empty());
        // 设置; in 设置 the same row reads 返回 (`settings.rs`), so one spot toggles.
        let footer = div()
            .flex_none()
            .flex()
            .flex_col()
            .when(self.batch.is_none(), |d| {
                d.child(footer_slot(
                    footer_row("ft-vault", "folder-input", crate::settings::tr("文档库", "Vaults"))
                        .on_click(cx.listener(|w, _, _, cx| w.open_settings(crate::settings::Page::Library, cx))),
                ))
            })
            // 批量操作 toolbar: 创建副本 | 移动到 | 删除, minus the cloud ones.
            .when(self.batch.is_some(), |d| {
                d.child(
                    div().h(px(46.)).mx(px(SB_X + 6. + 10. - 18.)).flex().items_center().gap(px(4.))
                        .child(footer_btn("bt-copy", "copy", crate::settings::text("创建副本"), some, false).on_click(cx.listener(|w, _, _, cx| w.batch_copy(cx))))
                        .child(footer_btn("bt-move", "folder-input", crate::settings::text("移动到"), some, false).on_click(cx.listener(|w, _, window, cx| w.batch_move(window, cx))))
                        .child(footer_btn("bt-delete", "bin", crate::settings::text("删除"), some, false).on_click(cx.listener(|w, _, _, cx| w.ask_batch_delete(cx)))),
                )
            });

        div()
            .w(px(self.sidebar_w(window)))
            .flex_none()
            .h_full()
            .flex()
            .flex_col()
            .child(header)
            .child(nav)
            .child(div().px(px(SB_X + 6.)).pb(px(8.)).text_size(px(12.)).text_color(Theme::text_caption()).truncate().child(crate::store::Store::root_dir().file_name().unwrap_or_default().to_string_lossy().into_owned()))
            .child(div().id("doc-list").flex_1().min_h_0().overflow_y_scroll().track_scroll(&self.list_scroll).child(list))
            .children(crate::changelog::card(cx.listener(|_, _, _, cx| cx.notify())))
            .child(footer)
            .into_any_element()
    }

    /// Width handle: a 15px hit strip straddling the sidebar edge (6px inside,
    /// 9px over the note's margin). Borderless like the sidebar: only the
    /// cursor changes, and a faint line shows while dragging.
    pub(crate) fn render_resize_handle(&self, window: &Window, cx: &mut Context<Self>) -> AnyElement {
        let w = self.sidebar_w(window) + RAIL_W;
        let dragging = self.sidebar_drag;
        div()
            .id("sidebar-resize")
            .absolute()
            .top_0()
            .bottom_0()
            .left(px(w - 6.))
            .w(px(15.))
            .group("sidebar-resize")
            .cursor(CursorStyle::ResizeLeftRight)
            .child(
                div()
                    .absolute()
                    .top_0()
                    .bottom_0()
                    .left(px(5.))
                    .w(px(2.))
                    .rounded(px(1.))
                    .when(dragging, |d| d.bg(Theme::primary()))
                    .group_hover("sidebar-resize", |s| s.bg(Theme::primary())),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |ws, _, window, cx| {
                    cx.stop_propagation();
                    ws.sidebar_width = ws.sidebar_w(window);
                    ws.sidebar_drag = true;
                    cx.notify();
                }),
            )
            .into_any_element()
    }

    /// While dragging, a transparent full-window layer keeps the resize cursor
    /// and follows the mouse.
    pub(crate) fn render_resize_overlay(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        if !self.sidebar_drag {
            return None;
        }
        let end = |ws: &mut Workspace, cx: &mut Context<Workspace>| {
            ws.sidebar_drag = false;
            let mut s = ws.store.settings();
            s.sidebar_width = Some(ws.sidebar_width);
            let _ = ws.store.save_settings(&s);
            cx.notify();
        };
        Some(
            deferred(
                div()
                    .id("sidebar-resize-overlay")
                    .absolute()
                    .inset_0()
                    .occlude()
                    .cursor(CursorStyle::ResizeLeftRight)
                    .on_mouse_move(cx.listener(move |ws, ev: &MouseMoveEvent, window, cx| {
                        // The release can land before this layer exists.
                        if ev.pressed_button != Some(MouseButton::Left) {
                            return end(ws, cx);
                        }
                        ws.sidebar_width = f32::from(ev.position.x) - RAIL_W;
                        ws.sidebar_width = ws.sidebar_w(window);
                        cx.notify();
                    }))
                    .on_mouse_up(MouseButton::Left, cx.listener(move |ws, _, _, cx| end(ws, cx)))
                    .on_mouse_up_out(MouseButton::Left, cx.listener(move |ws, _, _, cx| end(ws, cx))),
            )
            .with_priority(4)
            .into_any_element(),
        )
    }

    /// Section header: the title folds the list (an
    /// arrow appears on hover); 我的文档 has "+" and 批量操作 on the right,
    /// laid out so the last one lines up with the rows' "…".
    fn section(&self, idx: usize, title: &'static str, cx: &mut Context<Self>) -> Div {
        let group = SharedString::from(format!("section-{idx}"));
        let folded = self.folded[idx];
        let trigger = |id: &'static str, name: &'static str| {
            div()
                .id(id)
                .flex_none()
                .size(px(22.))
                .p(px(3.))
                .rounded(px(6.))
                .cursor_pointer()
                .hover(|s| s.bg(Theme::sidebar_hover()))
                .child(icon(name, 16., Theme::text_caption()))
        };
        let row = div().h(px(SECTION_H)).pl(px(4.)).pt(px(SECTION_TOP)).flex().items_center().group(group.clone());
        if let (1, Some(sel)) = (idx, self.batch.as_ref()) {
            let n = sel.len();
            let all = n > 0 && n >= self.entries.len().min(BATCH_LIMIT);
            return row
                .child(
                    div()
                        .id("batch-all")
                        .ml(px(6.))
                        .size(px(14.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .group("batch-all")
                        .child(checkbox(if all { Check::On } else if n > 0 { Check::Partial } else { Check::Off }, false, "batch-all"))
                        .on_click(cx.listener(move |w, _, _, cx| {
                            let ids = if all { vec![] } else { w.entries.iter().take(BATCH_LIMIT).map(|e| e.id.clone()).collect() };
                            w.batch = Some(ids);
                            cx.notify();
                        })),
                )
                .child(div().flex_1().min_w_0().ml(px(8.)).text_size(px(13.)).line_height(px(20.)).text_color(Theme::text()).child(crate::localized_format!("已选 {n} 项", "Selected: {n}")))
                .child(
                    div()
                        .id("batch-exit")
                        .flex_none()
                        .ml(px(8.))
                        .h(px(22.))
                        .px(px(10.))
                        .flex()
                        .items_center()
                        .rounded(px(7.))
                        .border_1()
                        .border_color(Theme::button_border())
                        .bg(crate::theme::raised(Theme::bg_body()))
                        .shadow(theme::shadow_button())
                        .cursor_pointer()
                        .hover(|s| s.bg(theme::c(craft::N100)))
                        .text_size(px(12.))
                        .line_height(px(18.))
                        .child(crate::settings::text("退出"))
                        .tip(crate::settings::text("退出批量操作"), None)
                        .on_click(cx.listener(|w, _, _, cx| {
                            w.batch = None;
                            w.row_menu = None;
                            cx.notify();
                        })),
                )
                .child(trigger("section-add", "plus").ml(px(2.)).mr(px(4.)).tip(crate::settings::tr("新建文档", "New document"), None).on_click(cx.listener(|w, _, window, cx| w.new_doc(window, cx))));
        }
        row.child(
            div()
                // Keyed by state so the tooltip is rebuilt after a toggle.
                .id(SharedString::from(format!("section-title-{idx}-{folded}")))
                .px(px(4.))
                .h(px(22.))
                .flex()
                .items_center()
                .rounded(px(6.))
                .cursor_pointer()
                .hover(|s| s.bg(Theme::sidebar_hover()))
                .tip(if folded { crate::settings::text("点击展开") } else { crate::settings::text("点击折叠") }, None)
                .child(div().pr(px(4.)).text_size(px(13.)).line_height(px(20.)).font_weight(FontWeight::SEMIBOLD).text_color(Theme::sidebar_section()).child(crate::settings::label(title)))
                .child(
                    div()
                        .when(!folded, |d| d.opacity(0.).group_hover(group, |s| s.opacity(1.)))
                        .child(icon(if folded { "chevron-right" } else { "chevron-down" }, 10., Theme::text_caption())),
                )
                .on_click(cx.listener(move |w, _, _, cx| {
                    w.folded[idx] = !w.folded[idx];
                    cx.notify();
                })),
        )
        .child(div().flex_1())
        .when(idx == 1, |d| {
            d.child(trigger("section-batch", "batch").tip(crate::settings::text("批量操作"), None).on_click(cx.listener(|w, _, _, cx| {
                w.batch = Some(vec![]);
                w.folded[1] = false;
                w.row_menu = None;
                cx.notify();
            })))
            .child(trigger("section-add", "plus").ml(px(2.)).mr(px(4.)).tip(crate::settings::tr("新建文档", "New document"), None).on_click(cx.listener(|w, _, window, cx| w.new_doc(window, cx))))
        })
    }

    /// Check or uncheck a 我的文档 row (entering 批量操作 if needed); false
    /// when the 50-page limit blocks it.
    pub(crate) fn toggle_check(&mut self, id: &str) -> bool {
        self.folded[1] = false;
        let sel = self.batch.get_or_insert_with(Vec::new);
        if let Some(i) = sel.iter().position(|x| x == id) {
            sel.remove(i);
        } else if sel.len() < BATCH_LIMIT {
            sel.push(id.to_string());
        } else {
            return false;
        }
        true
    }

    /// The checked docs, in list order.
    fn checked(&self) -> Vec<String> {
        let Some(sel) = &self.batch else { return vec![] };
        self.entries.iter().filter(|e| sel.contains(&e.id)).map(|e| e.id.clone()).collect()
    }

    /// 批量创建副本: a "… 副本" of each checked doc; leaves 批量操作.
    pub(crate) fn batch_copy(&mut self, cx: &mut Context<Self>) {
        let ids = self.checked();
        if ids.is_empty() {
            return;
        }
        for id in &ids {
            self.copy_doc(id, cx);
        }
        self.entries = self.store.list();
        self.sort();
        self.batch = None;
        self.row_menu = None;
        cx.notify();
    }

    pub(crate) fn batch_move(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let ids = self.checked();
        if !ids.is_empty() {
            self.open_move(ids, window, cx);
        }
    }

    pub(crate) fn ask_batch_delete(&mut self, cx: &mut Context<Self>) {
        let ids = self.checked();
        if !ids.is_empty() {
            self.confirm = Some(Confirm::Delete { ids, batch: true });
            self.row_menu = None;
            cx.notify();
        }
    }

    fn row_base(&self, e: &Entry, key: &str, cx: &mut Context<Self>) -> Stateful<Div> {
        let id = e.id.clone();
        let id2 = e.id.clone();
        let in_lib = key == "doc";
        div()
            .id(SharedString::from(format!("{key}-{}", e.id)))
            .group("doc-row")
            .mb(px(ROW_GAP))
            .rounded(px(ROW_R))
            .cursor_pointer()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |w, ev: &MouseDownEvent, window, cx| {
                    if w.rename.as_ref().is_some_and(|r| r.id == id) {
                        return;
                    }
                    w.row_menu = None;
                    // In 批量操作 a click checks the row; Shift-click enters it.
                    if in_lib && (w.batch.is_some() || ev.modifiers.shift) {
                        w.toggle_check(&id);
                        cx.notify();
                        return;
                    }
                    w.open(&id, window, cx);
                }),
            )
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(move |w, ev: &MouseDownEvent, _, cx| {
                    let batch = in_lib && w.batch.is_some();
                    if batch && !w.batch.as_ref().is_some_and(|b| b.contains(&id2)) && !w.toggle_check(&id2) {
                        return;
                    }
                    w.row_menu = Some(RowMenu { id: id2.clone(), pos: ev.position, corner: Corner::TopLeft, sub: false, batch });
                    cx.notify();
                }),
            )
    }

    fn more_button(&self, e: &Entry, key: &str, cx: &mut Context<Self>) -> Stateful<Div> {
        let id = e.id.clone();
        let slot = format!("{key}-{}", e.id);
        let slot2 = slot.clone();
        let record = self.more_bounds.clone();
        let lookup = self.more_bounds.clone();
        let open = self.row_menu.as_ref().is_some_and(|m| m.id == e.id && m.pos.x < px(0.));
        div()
            .id(SharedString::from(format!("more-{slot}")))
            .relative()
            .size(px(22.))
            .mr(px(4.))
            .flex_none()
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(6.))
            .when(!open, |d| d.opacity(0.).group_hover("doc-row", |s| s.opacity(1.)))
            .when(open, |d| d.bg(Theme::sidebar_selected()))
            .hover(|s| s.bg(Theme::sidebar_selected()))
            .child(icon("ellipsis", 15., Theme::text_code()))
            .child(canvas(move |b, _, _| record.borrow_mut().insert(slot.clone(), b), |_, _, _, _| {}).absolute().inset_0())
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |w, ev: &MouseDownEvent, _, cx| {
                    cx.stop_propagation();
                    // Hang the menu from the button; a negative x marks "from the button".
                    let b = lookup.borrow().get(&slot2).copied();
                    let pos = b.map_or(ev.position, |b| point(-b.left() - px(1.), b.top()));
                    w.row_menu = Some(RowMenu { id: id.clone(), pos, corner: Corner::TopLeft, sub: false, batch: false });
                    cx.notify();
                }),
            )
    }

    /// Hover "+" (新建子页面): a new page under this one, straight away.
    fn add_button(&self, e: &Entry, key: &str, cx: &mut Context<Self>) -> impl IntoElement {
        let id = e.id.clone();
        div()
            .id(SharedString::from(format!("add-{key}-{}", e.id)))
            .size(px(22.))
            .p(px(3.))
            .flex_none()
            .rounded(px(6.))
            .opacity(0.)
            .group_hover("doc-row", |s| s.opacity(1.))
            .hover(|s| s.bg(Theme::sidebar_selected()))
            .child(icon("plus", 16., Theme::text_code()))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_click(cx.listener(move |w, _, window, cx| {
                cx.stop_propagation();
                w.row_menu = None;
                w.new_child(&id, window, cx);
            }))
    }

    fn doc_row(&self, e: &Entry, key: &str, depth: usize, kids: bool, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.view == View::Doc && self.current.as_deref() == Some(e.id.as_str());
        let renaming = self.rename.as_ref().filter(|r| r.id == e.id).map(|r| r.field.clone());
        let title = match renaming {
            Some(field) => div()
                .ml(px(6.))
                .mr(px(6.))
                .flex_1()
                .min_w_0()
                .h(px(24.))
                .px(px(6.))
                .flex()
                .items_center()
                .rounded(px(6.))
                .bg(crate::theme::raised(Theme::bg_body()))
                .border_1()
                .border_color(Theme::primary_500())
                .text_size(px(13.))
                .child(field)
                .into_any_element(),
            None => div()
                .pl(px(9.))
                .flex_1()
                .min_w_0()
                .text_size(px(13.))
                .line_height(px(DOC_ROW_H))
                .overflow_hidden()
                .whitespace_nowrap()
                .text_ellipsis()
                .text_color(if selected { Theme::text() } else { Theme::sidebar_text() })
                .child(ui_text(display_title(&e.title)))
                .into_any_element(),
        };
        let renaming = self.rename.as_ref().is_some_and(|r| r.id == e.id);
        let folder = kids || self.entries.iter().any(|child| child.meta.parent.as_deref() == Some(e.id.as_str()));
        // 批量操作: 我的文档 rows get a checkbox and lose "…"; checked rows are tinted.
        let batch = self.batch.as_ref().filter(|_| key == "doc");
        let checked = batch.is_some_and(|b| b.contains(&e.id));
        let blocked = batch.is_some_and(|b| !checked && b.len() >= BATCH_LIMIT);
        self.row_base(e, key, cx)
            .h(px(DOC_ROW_H))
            .flex()
            .items_center()
            .when(selected || checked, |d| d.bg(Theme::sidebar_selected()))
            .when(!selected && !checked, |d| d.hover(|s| s.bg(Theme::sidebar_hover())))
            .pl(px(6. + 18. * depth as f32))
            .when(batch.is_some(), |d| {
                d.child(
                    div()
                        .id(SharedString::from(format!("check-{}", e.id)))
                        .size(px(20.))
                        .mr(px(4.))
                        .flex_none()
                        .flex()
                        .items_center()
                        .justify_center()
                        .when(blocked, |d| d.tip(crate::settings::text("无法选中，选择的页面数已到达上限"), None))
                        .child(checkbox(if checked { Check::On } else { Check::Off }, blocked, "doc-row")),
                )
            })
            .child({
                // The icon shares 搜索/主页's column, so there is no arrow column:
                // on pages with children, hovering the row swaps the icon for
                // a 10px expand triangle.
                let glyph = div().size(px(20.)).flex().items_center().justify_center().map(|d| match &e.icon {
                    _ if folder => d.child(icon("folder", ROW_ICON, if selected { Theme::text() } else { Theme::text_caption() })),
                    // 文档图标 replaces the page glyph.
                    Some(em) => d.text_size(px(13.)).line_height(px(20.)).child(em.clone()),
                    None => d.child(icon("doc", ROW_ICON, if selected { Theme::text() } else { Theme::text_caption() })),
                });
                let slot = div().id(SharedString::from(format!("arrow-{key}-{}", e.id))).relative().size(px(20.)).flex_none().rounded(px(5.));
                if kids {
                    let open = self.expanded.contains(&e.id);
                    let toggle = e.id.clone();
                    slot.hover(|s| s.bg(Theme::sidebar_selected()))
                        .active(|s| s.bg(Theme::fill_pressed()))
                        .child(glyph.group_hover("doc-row", |s| s.opacity(0.)))
                        .child(
                            div()
                                .absolute()
                                .inset_0()
                                .flex()
                                .items_center()
                                .justify_center()
                                .opacity(0.)
                                .group_hover("doc-row", |s| s.opacity(1.))
                                .child(icon("tree-triangle", 10., theme::c(craft::N500)).when(open, |s| s.with_transformation(Transformation::rotate(radians(std::f32::consts::FRAC_PI_2))))),
                        )
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |w, _, _, cx| {
                                cx.stop_propagation();
                                if !w.expanded.remove(&toggle) {
                                    w.expanded.insert(toggle.clone());
                                }
                                cx.notify();
                            }),
                        )
                } else {
                    slot.child(glyph)
                }
            })
            .child(title)
            .when(!renaming && batch.is_none() && key == "doc", |d| d.child(self.add_button(e, key, cx)))
            .when(!renaming && batch.is_none(), |d| d.child(self.more_button(e, key, cx).when(key == "doc", |d| d.ml(px(2.)))))
    }

    pub(crate) fn render_row_menu(&self, window: &Window, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let Some(m) = self.row_menu.clone() else { return vec![] };
        let Some((pinned, starred)) = self.entries.iter().find(|e| e.id == m.id).map(|e| (e.meta.pinned, e.meta.starred > 0)) else { return vec![] };
        // The context menu: accent highlight with a white label.
        let item_c = |id: &'static str, ic: &'static str, label: &'static str, color: Hsla, icon_color: Hsla| {
            let g = SharedString::from(id);
            crate::ui::menu_row(id, g.clone())
                .mx(px(0.))
                .text_color(color)
                .child(icon(ic, 16., icon_color).group_hover(g.clone(), |s| s.text_color(gpui::white())))
                .child(div().flex_1().min_w_0().child(crate::ui::menu_label(label, g, gpui::white())))
        };
        let item = |id, ic, label| item_c(id, ic, label, Theme::text(), Theme::text_code());
        let divider = || div().mx(px(8.)).my(px(5.)).h(px(1.)).bg(Theme::line_border());
        if m.batch {
            // Right-click in 批量操作: the batch menu minus 添加快捷方式到, 转移所有权.
            let n = self.batch.as_ref().map_or(0, |b| b.len());
            let menu = popover()
                .id("batch-menu")
                .occlude()
                .w(px(ROW_MENU_W))
                .px(px(5.))
                .py(px(crate::ui::MENU_PAD_Y))
                .on_mouse_down_out(cx.listener(|w, _, _, cx| {
                    w.row_menu = None;
                    cx.notify();
                }))
                .child(div().h(px(ROW_ITEM)).px(px(8.)).flex().items_center().text_size(px(12.)).text_color(Theme::text_placeholder()).child(crate::localized_format!("已选 {n} 项", "Selected: {n}")))
                .child(item("bm-copy", "copy", crate::settings::text("创建副本")).on_click(cx.listener(|w, _, _, cx| w.batch_copy(cx))))
                .child(item("bm-move", "folder-input", crate::settings::text("移动到")).on_click(cx.listener(|w, _, window, cx| w.batch_move(window, cx))))
                .child(item("bm-delete", "bin", crate::settings::text("删除")).on_click(cx.listener(|w, _, _, cx| w.ask_batch_delete(cx))));
            return vec![
                deferred(anchored().anchor(m.corner).position(m.pos).snap_to_window_with_margin(px(8.)).child(menu))
                    .with_priority(2)
                    .into_any_element(),
            ];
        }
        // Hovering 下载 opens its submenu; hovering any other item closes it.
        let set_sub = |open: bool| {
            cx.listener(move |w: &mut Workspace, hovered: &bool, _, cx| {
                if let Some(m) = w.row_menu.as_mut().filter(|m| *hovered && m.sub != open) {
                    m.sub = open;
                    cx.notify();
                }
            })
        };
        let (dup, mv, pin, star, dl, ren, del) = (m.id.clone(), m.id.clone(), m.id.clone(), m.id.clone(), m.id.clone(), m.id.clone(), m.id.clone());
        let pdf = m.id.clone();
        let new_window = m.id.clone();
        let sub_bounds = self.row_sub_bounds.clone();
        // The row menu leaves out the cloud entries 在新标签页打开, 分享,
        // 复制链接, 添加快捷方式到, 转移所有权, 设置密级.
        let menu = popover()
            .id("row-menu")
            .occlude()
            .w(px(ROW_MENU_W))
            .px(px(5.))
            .py(px(crate::ui::MENU_PAD_Y))
            .on_mouse_down_out(cx.listener(move |w, ev: &MouseDownEvent, _, cx| {
                if sub_bounds.get().is_none_or(|b| !b.contains(&ev.position)) {
                    w.row_menu = None;
                    cx.notify();
                }
            }))
            .child(
                item("rm-window", "copy", crate::settings::text("在新窗口中打开"))
                    .on_hover(set_sub(false))
                    .on_click(cx.listener(move |w, _, _, cx| {
                        w.row_menu = None;
                        let id = new_window.clone();
                        cx.defer(move |cx| crate::quick_note::open_document(Some(id), cx));
                        cx.notify();
                    })),
            )
            .child(
                item("rm-copy", "copy", crate::settings::text("创建副本"))
                    .on_hover(set_sub(false))
                    .on_click(cx.listener(move |w, _, window, cx| w.duplicate_doc(&dup, window, cx))),
            )
            .child(
                item("rm-move", "folder-input", crate::settings::text("移动到"))
                    .on_hover(set_sub(false))
                    .on_click(cx.listener(move |w, _, window, cx| w.open_move(vec![mv.clone()], window, cx))),
            )
            .child(
                item("rm-pin", if pinned { "pin-off" } else { "pin" }, if pinned { crate::settings::text("从“置顶”移除") } else { crate::settings::text("添加到“置顶”") })
                    .on_hover(set_sub(false))
                    .on_click(cx.listener(move |w, _, _, cx| w.toggle_pin(&pin, cx))),
            )
            .child(
                item("rm-star", if starred { "star-off" } else { "star" }, if starred { crate::settings::text("取消收藏") } else { crate::settings::tr("收藏", "Add to favorites") })
                    .on_hover(set_sub(false))
                    .on_click(cx.listener(move |w, _, _, cx| w.toggle_star(&star, cx))),
            )
            .child(divider())
            .child(
                item("rm-download", "export", crate::settings::text("下载"))
                    .when(m.sub, |d| d.bg(Theme::menu_highlight()).text_color(gpui::white()))
                    .child(icon("chevron-right", 10., if m.sub { gpui::white() } else { Theme::text_caption() }).group_hover("rm-download", |s| s.text_color(gpui::white())))
                    .on_hover(set_sub(true)),
            )
            .child(
                item("rm-rename", "square-pen", crate::settings::text("重命名"))
                    .on_hover(set_sub(false))
                    .on_click(cx.listener(move |w, _, window, cx| w.start_rename(&ren, window, cx))),
            )
            .child(divider())
            .child(
                item_c("rm-del", "bin", crate::settings::text("删除"), Theme::danger(), Theme::danger())
                    .on_hover(set_sub(false))
                    .on_click(cx.listener(move |w, _, _, cx| w.ask_delete(&del, cx))),
            );
        let vh = f32::from(window.viewport_size().height);
        let menu_h = row_menu_height(8, 2);
        // From the "…" button: hang 2px below it, or sit 2px above it when
        // there's no room below. From a right-click: at the pointer.
        let (origin, corner) = if m.pos.x < px(0.) {
            let (left, top) = (-m.pos.x, f32::from(m.pos.y));
            if top + 20. + 2. + menu_h > vh - 8. {
                (point(left, px(top - 2.)), Corner::BottomLeft)
            } else {
                (point(left, px(top + 22.)), Corner::TopLeft)
            }
        } else {
            (m.pos, m.corner)
        };
        let menu_top = match corner {
            Corner::BottomLeft | Corner::BottomRight => f32::from(origin.y) - menu_h,
            _ => f32::from(origin.y),
        };
        let mut out = vec![
            deferred(anchored().anchor(corner).position(origin).snap_to_window_with_margin(px(8.)).child(menu))
                .with_priority(2)
                .into_any_element(),
        ];
        if m.sub {
            let record = self.row_sub_bounds.clone();
            record.set(None);
            // Level the submenu's first item with 下载 (the 5th item, after one divider).
            let top = menu_top + 4. * ROW_ITEM + ROW_DIVIDER;
            let left = match corner {
                Corner::TopRight | Corner::BottomRight => origin.x - px(ROW_MENU_W),
                _ => origin.x,
            };
            let x = left + px(ROW_MENU_W + 4.);
            out.push(
                deferred(
                    anchored().anchor(Corner::TopLeft).position(point(x, px(top))).snap_to_window_with_margin(px(8.)).child(
                        popover()
                            .id("rm-download-menu")
                            .occlude()
                            .w(px(130.))
                            .px(px(5.))
                            .py(px(crate::ui::MENU_PAD_Y))
                            .child(canvas(move |b, _, _| record.set(Some(b)), |_, _, _, _| {}).absolute().inset_0())
                            .when(crate::printing::SUPPORTED, |d| d.child(crate::docmenu::sub_item("rm-pdf", "PDF").on_click(cx.listener(move |w, _, window, cx| w.export_pdf(&pdf, window, cx)))))
                            .child(crate::docmenu::sub_item("rm-md", "Markdown").on_click(cx.listener(move |w, _, _, cx| w.open_export_md(&dl, cx)))),
                    ),
                )
                .with_priority(2)
                .into_any_element(),
            );
        } else {
            self.row_sub_bounds.set(None);
        }
        out
    }
}
