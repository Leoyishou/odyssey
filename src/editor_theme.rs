pub use crate::theme::{craft, c, ca, rgbaf, ta, TITLE, UI_FONT, MONO_FONT, BlockMetrics};
/// The active theme's resolved style (themes.rs).
pub fn style() -> &'static crate::themes::Style { crate::themes::active() }
pub fn body_size() -> f32 { style().num("page.size") }
pub fn body_line() -> f32 { style().num("page.line") }
pub fn block_gap() -> f32 { style().num("p.gap") }
fn h(level: u8) -> u8 { level.clamp(1, 9) }
pub fn heading_metrics(level: u8) -> BlockMetrics {
    let (s, l) = (style(), h(level));
    BlockMetrics { size: s.num(&format!("h{l}.size")), line_height: s.num(&format!("h{l}.line")), mt: s.num(&format!("h{l}.mt")), mb: s.num(&format!("h{l}.mb")) }
}
pub fn heading_weight(level: u8) -> gpui::FontWeight { gpui::FontWeight(style().num(&format!("h{}.weight", h(level)))) }
pub fn heading_font(level: u8) -> &'static str { style().font(&format!("h{}.font", h(level))) }
pub fn heading_color(level: u8) -> Option<gpui::Hsla> { style().color(&format!("h{}.color", h(level))) }
/// The page title.
pub fn title() -> BlockMetrics { BlockMetrics { size: style().num("title.size"), line_height: style().num("title.line"), mt: 0., mb: 0. } }
/// Document text's family (UI fonts stay the system's).
pub fn body_font() -> &'static str { style().font("page.font") }
/// Document colour tokens: a theme file may override any of them by name
/// (docs/THEMES.md); otherwise they come from the base style.
macro_rules! tokens {
    ($($n:ident),* $(,)?) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy)]
        enum Tok { $($n),* }
        /// Names a theme file uses for the colour tokens, in index order.
        pub const TOKENS: &[&str] = &[$(stringify!($n)),*];
        /// The base style's value of token `i`.
        pub fn base_color(i: usize, feishu: bool) -> gpui::Hsla {
            const ALL: &[(fn() -> gpui::Hsla, fn() -> gpui::Hsla)] = &[$((crate::feishu_theme::Theme::$n, crate::theme::Theme::$n)),*];
            if feishu { (ALL[i].0)() } else { (ALL[i].1)() }
        }
        pub struct Theme;
        impl Theme {
            $(pub fn $n() -> gpui::Hsla { crate::themes::color(Tok::$n as usize).unwrap_or_else(|| base_color(Tok::$n as usize, crate::settings::feishu())) })*
        }
    };
}
tokens!(
    text, text_caption, text_placeholder, block_placeholder, text_disabled, text_code,
    link, link_hover, primary, primary_500, primary_fill_1, bg_body,
    bg_overlay, line_divider, line_border, fill_hover, fill_hover_strong, fill_pressed,
    fill_turninto_active, selection, block_hover, block_selected, handle_active, size_cell,
    size_cell_border, size_cell_active, size_cell_disabled, swatch_border, block_bg_area, todo_done,
    scrollbar_thumb, shadow, code_hover_ring, quote_bar, caret, pinned,
    table_header, table_line, table_bar_hover, table_bar_selected, danger_bar, danger_cell,
    danger, window, sidebar_text, sidebar_hover, sidebar_selected, sidebar_section,
    button_border, code_chip, code_chip_border, code_bg, code_gutter, code_gutter_line,
    code_border, code_line_number, caption, menu_highlight, popover_bg, separator_dots,
    todo_box, code_text,
);
fn hue(name: &str) -> &str { name.strip_prefix("light-").or_else(|| name.strip_prefix("medium-")).unwrap_or(name) }
pub fn text_color(name: &str) -> Option<gpui::Hsla> { style().palette("color", name).or_else(|| if crate::settings::feishu() { crate::feishu_theme::text_color(name) } else { crate::theme::text_color(name) }) }
pub fn bg_color(name: &str) -> Option<gpui::Hsla> { style().palette("highlight", name).or_else(|| if crate::settings::feishu() { crate::feishu_theme::bg_color(name) } else { crate::theme::bg_color(name) }) }
pub fn block_bg(name: &str) -> Option<gpui::Hsla> { style().palette("background", name).or_else(|| if crate::settings::feishu() { crate::feishu_theme::block_bg(name) } else { crate::theme::block_bg(name) }) }
pub fn bg_swatch(name: &str) -> Option<gpui::Hsla> { if crate::settings::feishu() { crate::feishu_theme::bg_swatch(name) } else { crate::theme::bg_swatch(name) } }
pub fn callout_bg(name: &str) -> Option<gpui::Hsla> { style().palette("callout", name).or_else(|| if crate::settings::feishu() { crate::feishu_theme::callout_bg(name) } else { crate::theme::callout_bg(name) }) }
pub fn callout_border(name: &str) -> Option<gpui::Hsla> { style().palette("border", hue(name)).or_else(|| if crate::settings::feishu() { crate::feishu_theme::callout_border(name) } else { crate::theme::callout_border(name) }) }
pub fn callout_border_swatch(name: &str) -> Option<gpui::Hsla> { if crate::settings::feishu() { crate::feishu_theme::callout_border_swatch(name) } else { crate::theme::callout_border_swatch(name) } }
pub use crate::theme::{shadow_button, shadow_menu, shadow_panel, shadow_dialog};

pub fn mono_font() -> &'static str { style().font("pre.font") }
