//! 子页面列表 block (`<sub-page-list/>`): the page's name as a small
//! header, then one row per child page in a crisp box (1pt hairline,
//! radius 5). The list itself comes from the library (`Editor::sub_pages`).

use super::render::R;
use super::view::{Editor, EditorEvent, Menu};
use crate::settings::display_title;
use crate::model::Block;
use crate::editor_theme::Theme;
use crate::editor_ui::{icon, ui_text};
use gpui::{prelude::FluentBuilder, *};

/// Containers inside the page keep crisp corners: the box at 5, the
/// hover under a row at 4.
const RADIUS: f32 = 5.;
const ROW: f32 = 4.;

/// 16px page glyph, or the page's emoji icon.
fn page_icon(emoji: &Option<String>) -> AnyElement {
    match emoji {
        Some(e) => div().text_size(px(16.)).line_height(px(18.)).child(e.clone()).into_any_element(),
        None => icon("doc", 16., Theme::text_caption()).into_any_element(),
    }
}

impl R<'_> {
    pub(super) fn sub_page_list(&self, b: &Block, cx: &mut Context<Editor>) -> AnyElement {
        let ed = self.ed;
        let id = b.id;
        // Handle menu open: the accent-tinted ring of the code block;
        // block-selected: an accent rim; otherwise the card hairline.
        let menu_on = matches!(&ed.menu, Some(Menu::Block { block, .. }) | Some(Menu::Add { block, .. }) if *block == id);
        let selected = !menu_on && self.selected.contains(&id);
        let header = div().min_h(px(24.)).mb(px(8.)).flex().items_center().child(
            div()
                .h(px(20.))
                .min_w_0()
                .pr(px(4.))
                .flex()
                .items_center()
                .rounded(px(ROW))
                .hover(|s| s.bg(Theme::fill_hover()))
                .child(div().size(px(20.)).p(px(2.)).flex_none().flex().items_center().justify_center().child(page_icon(&ed.core.doc.icon)))
                .child(
                    div()
                        .min_w_0()
                        .max_w(px(476.))
                        .text_size(px(12.))
                        .line_height(px(20.))
                        .text_color(Theme::text_caption())
                        .overflow_hidden()
                        .whitespace_nowrap()
                        .text_ellipsis()
                        .child(ui_text(display_title(&ed.core.doc.title.text))),
                ),
        );
        let mut content = div()
            .relative()
            .p(px(4.))
            .rounded(px(RADIUS))
            .border_1()
            .border_color(if menu_on {
                Theme::code_hover_ring()
            } else if selected {
                Theme::primary()
            } else {
                Theme::button_border()
            })
            .when(menu_on, |d| {
                d.child(div().absolute().top(px(-3.)).left(px(-3.)).right(px(-3.)).bottom(px(-3.)).rounded(px(RADIUS + 3.)).border_2().border_color(Theme::code_hover_ring()))
            });
        if ed.sub_pages.is_empty() {
            content = content.child(
                div().h(px(240.)).flex().items_center().justify_center().text_size(px(13.)).line_height(px(20.)).text_color(Theme::text_caption()).child(crate::settings::text("目录下暂无子页面")),
            );
        }
        for p in &ed.sub_pages {
            let doc = p.id.clone();
            content = content.child(
                div()
                    .id(SharedString::from(format!("subpage-{}-{}", id.0, p.id)))
                    .min_h(px(36.))
                    .px(px(6.))
                    .py(px(7.))
                    .flex()
                    .items_start()
                    .rounded(px(ROW))
                    .cursor_pointer()
                    .hover(|s| s.bg(Theme::fill_hover()))
                    .child(div().w(px(20.)).h(px(22.)).mr(px(6.)).p(px(2.)).flex_none().flex().items_center().justify_center().child(page_icon(&p.icon)))
                    // A page block's title: body size.
                    .child(div().flex_1().min_w_0().text_size(px(15.)).line_height(px(22.)).text_color(Theme::text()).line_clamp(2).child(display_title(&p.title)))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |_, _, _, cx| {
                            cx.stop_propagation();
                            cx.emit(EditorEvent::OpenDoc(doc.clone()));
                        }),
                    ),
            );
        }
        div().max_w(px(720.)).child(header).child(content).into_any_element()
    }
}
