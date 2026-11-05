pub use crate::ui::*;
use gpui::*;
use crate::feishu_theme::{self as theme, Theme};
pub fn icon(name: &str, size: f32, color: Hsla) -> Svg { svg().path(SharedString::from(format!("editor-icons/{name}.svg"))).size(px(size)).flex_none().text_color(color) }
pub fn popover() -> Div {
    div()
        .bg(crate::theme::raised(Theme::bg_body()))
        .border_1()
        .border_color(Theme::line_border())
        .rounded(px(6.))
        .shadow(theme::shadow_menu())
        .font_family(theme::UI_FONT)
        .text_color(Theme::text())
}
pub fn panel() -> Div {
    div()
        .bg(crate::theme::raised(Theme::bg_body()))
        .border_1()
        .border_color(Theme::line_border())
        .rounded(px(6.))
        .shadow(theme::shadow_panel())
        .font_family(theme::UI_FONT)
        .text_color(Theme::text())
}
pub const MENU_RADIUS: f32 = 6.;
pub const ROW_RADIUS: f32 = 4.;
pub const MENU_ROW_H: f32 = 32.;
pub const MENU_PAD_Y: f32 = 8.;

pub fn menu_row(id: impl Into<ElementId>, group: SharedString) -> Stateful<Div> {
    div().id(id).group(group).h(px(32.)).mx(px(4.)).px(px(8.)).flex().items_center().gap(px(12.)).rounded(px(4.))
        .text_size(px(14.)).line_height(px(22.)).text_color(Theme::text()).cursor_pointer().hover(|s| s.bg(Theme::fill_hover()))
}
pub fn menu_label(label: impl Into<SharedString>, _group: SharedString, _hover: Hsla) -> Div {
    div().whitespace_nowrap().child(crate::settings::label(label.into()))
}
pub fn menu_glyph(name: &str, size: f32, _group: SharedString) -> Svg { icon(name, size, Theme::text_code()) }
