//! The 飞书 theme's base: tokens for how Docx renders a document in its light
//! theme — heading metrics, mono font family, and the named colours DocxXML
//! carries (`text-color`, `background-color`) resolved to hex.

use gpui::{BoxShadow, Hsla, Rgba, point, px, rgb};

pub fn c(hex: u32) -> Hsla {
    crate::theme::adapt(rgb(hex).into())
}

/// A literal colour (shadows); `ta` is the adapted one, as in theme.rs.
pub fn rgbaf(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    Rgba { r: r as f32 / 255., g: g as f32 / 255., b: b as f32 / 255., a }.into()
}

/// The UD palette (`--N*`, `--B*`, `--R*`, `--O*`), light theme.
/// Colours elsewhere name these instead of hex, so they can be swapped
/// for a dark theme.
#[allow(dead_code)]
pub mod ud {
    pub const N00: u32 = 0xffffff;
    pub const N50: u32 = 0xf5f6f7;
    pub const N100: u32 = 0xf2f3f5;
    pub const N200: u32 = 0xeff0f1;
    pub const N300: u32 = 0xdee0e3;
    pub const N350: u32 = 0xd0d3d6;
    pub const N400: u32 = 0xbbbfc4;
    pub const N500: u32 = 0x8f959e;
    pub const N600: u32 = 0x646a73;
    pub const N650: u32 = 0x51565d;
    pub const N700: u32 = 0x373c43;
    pub const N800: u32 = 0x2b2f36;
    pub const N900: u32 = 0x1f2329;
    pub const N950: u32 = 0x0f1114;
    pub const N1000: u32 = 0x000000;
    pub const B50: u32 = 0xf0f4ff;
    pub const B100: u32 = 0xe0e9ff;
    pub const B200: u32 = 0xc2d4ff;
    pub const B300: u32 = 0x94b4ff;
    pub const B350: u32 = 0x7aa2ff;
    pub const B400: u32 = 0x5083fb;
    pub const B500: u32 = 0x336df4;
    pub const B600: u32 = 0x1456f0;
    pub const B700: u32 = 0x0442d2;
    pub const B800: u32 = 0x002f9e;
    pub const B900: u32 = 0x002270;
    pub const R50: u32 = 0xfef0f0;
    pub const R100: u32 = 0xfee3e2;
    pub const R200: u32 = 0xfdc6c4;
    pub const R300: u32 = 0xf89e9b;
    pub const R350: u32 = 0xff7570;
    pub const R400: u32 = 0xf54a45;
    pub const R500: u32 = 0xe22e28;
    pub const R600: u32 = 0xc02a26;
    pub const R700: u32 = 0xa11c17;
    pub const R800: u32 = 0x741915;
    pub const R900: u32 = 0x590603;
    pub const O50: u32 = 0xfff3e5;
    pub const O100: u32 = 0xfee7cd;
    pub const O200: u32 = 0xfec48b;
    pub const O300: u32 = 0xff9d4c;
    pub const O350: u32 = 0xff811a;
    pub const O400: u32 = 0xed6d0c;
    pub const O500: u32 = 0xc25705;
    pub const O600: u32 = 0xa44904;
    pub const O700: u32 = 0x853a05;
    pub const O800: u32 = 0x642b02;
    pub const O900: u32 = 0x3b1a02;

    /// The older UD scale, still used by some tokens (dialog buttons,
    /// the countdown, inputs' focus border, a few highlights).
    pub mod old {
        pub const B50: u32 = 0xf0f4ff;
        pub const B100: u32 = 0xe1eaff;
        pub const B200: u32 = 0xbacefd;
        pub const B300: u32 = 0x82a7fc;
        pub const B400: u32 = 0x4e83fd;
        pub const B500: u32 = 0x3370ff;
        pub const B600: u32 = 0x245bdb;
        pub const B700: u32 = 0x1c4cba;
        pub const G500: u32 = 0x34c724;
        pub const O500: u32 = 0xff8800;
    }
}

fn shadow(a: f32, y: f32, blur: f32, spread: f32) -> BoxShadow {
    let a = if crate::theme::dark() { (a * 3.).min(0.6) } else { a };
    BoxShadow { color: rgbaf(31, 35, 41, a), offset: point(px(0.), px(y)), blur_radius: px(blur), spread_radius: px(spread) }
}

/// Dropdowns and the slash / block menus (`shadow.popover`: 0 4 8).
pub fn shadow_menu() -> Vec<BoxShadow> {
    vec![shadow(0.1, 4., 8., 0.)]
}

/// Second-level panels, search popups, toasts (0 6 24).
pub fn shadow_panel() -> Vec<BoxShadow> {
    vec![shadow(0.1, 6., 24., 0.)]
}

/// Modal dialogs (0 8 24 4 at .12).
pub fn shadow_dialog() -> Vec<BoxShadow> {
    vec![shadow(0.12, 8., 24., 4.)]
}

/// `--shadow-s4-down`: the 「…」 menu, home menus, the icon picker.
pub fn shadow_s4_down() -> Vec<BoxShadow> {
    vec![shadow(0.04, 8., 24., 8.), shadow(0.04, 6., 12., 0.), shadow(0.06, 4., 8., -8.)]
}

pub const UI_FONT: &str = ".SystemUIFont";
pub const MONO_FONT: &str = "Menlo";

pub struct Theme;

impl Theme {
    pub fn text() -> Hsla { c(ud::N900) }
    pub fn text_caption() -> Hsla { c(ud::N600) }
    pub fn text_placeholder() -> Hsla { c(ud::N500) }
    pub fn block_placeholder() -> Hsla { c(ud::N400) }
    pub fn text_disabled() -> Hsla { c(ud::N400) }
    pub fn text_code() -> Hsla { c(ud::N800) }
    pub fn link() -> Hsla { c(ud::B500) }
    pub fn primary() -> Hsla { c(ud::B600) }
    pub fn primary_500() -> Hsla { c(ud::B500) }
    /// `--primary-fill-solid-01`.
    pub fn primary_fill_1() -> Hsla { c(ud::B50) }
    pub fn bg_body() -> Hsla { c(ud::N00) }
    pub fn bg_overlay() -> Hsla { c(ud::N50) }
    /// `--bg-content-base`: the gray behind search result cards.
    pub fn line_divider() -> Hsla { ta(31, 35, 41, 0.15) }
    pub fn line_border() -> Hsla { c(ud::N300) }
    pub fn fill_hover() -> Hsla { ta(31, 35, 41, 0.08) }
    pub fn fill_hover_strong() -> Hsla { ta(31, 35, 41, 0.10) }
    pub fn fill_pressed() -> Hsla { ta(31, 35, 41, 0.12) }
    pub fn fill_active() -> Hsla { ta(20, 86, 240, 0.15) }
    pub fn fill_turninto_active() -> Hsla { c(ud::B100) }
    pub fn selection() -> Hsla { ta(20, 86, 240, 0.18) }
    /// `--ccmtoken-doc-block-bg-hover`: a block whose handle menu is open
    /// (`.block.hovered`, radius 8), a selected table's cells.
    pub fn block_hover() -> Hsla { ta(20, 86, 240, 0.10) }
    /// `--ccmtoken-doc-block-bg-selected`: selected blocks' `.selected-mask` (radius 6).
    pub fn block_selected() -> Hsla { ta(20, 86, 240, 0.18) }
    /// Handle while hovered / its menu is open (fill-hover over white, opaque).
    pub fn handle_active() -> Hsla { c(ud::N200) }
    /// Insert-size picker cells: `--bg-base` in a 2px `--N200` frame; picked
    /// cells `--primary-content-loading`, default counts `--primary-fill-solid-02`.
    pub fn size_cell() -> Hsla { c(ud::N100) }
    pub fn size_cell_border() -> Hsla { c(ud::N200) }
    pub fn size_cell_active() -> Hsla { c(ud::B300) }
    pub fn size_cell_disabled() -> Hsla { c(ud::B100) }
    /// Hairline drawn inside font-color swatches (and the "none" swatch).
    pub fn swatch_border() -> Hsla { ta(31, 35, 41, 0.08) }
    /// `--ccmtoken-doc-block-bg-area`: hovered / focused to-do row.
    pub fn block_bg_area() -> Hsla { ta(100, 106, 115, 0.05) }
    /// Done to-do text: text-caption at the 0.7 opacity used on done spans.
    pub fn todo_done() -> Hsla { ta(100, 106, 115, 0.7) }
    pub fn scrollbar_thumb() -> Hsla { ta(31, 35, 41, 0.3) }
    pub fn shadow() -> Hsla { ta(31, 35, 41, 0.10) }
    /// `--B200`: ring of a code block / callout whose handle menu is open.
    pub fn code_hover_ring() -> Hsla { c(ud::B200) }
    pub fn quote_bar() -> Hsla { c(ud::N400) }
    pub fn caret() -> Hsla { c(ud::N900) }
    pub fn pinned() -> Hsla { c(0x10a893) }
    /// Table header row / column fill (N100).
    pub fn table_header() -> Hsla { c(ud::N100) }
    /// Table chrome bar hover (#dee0e3) and the will-delete reds.
    pub fn table_bar_hover() -> Hsla { c(ud::N300) }
    /// Both bars while the whole table is selected.
    pub fn table_bar_selected() -> Hsla { c(0xc5d3fb) }
    pub fn danger_bar() -> Hsla { c(ud::R350) }
    pub fn danger_cell() -> Hsla { c(ud::R50) }
    pub fn danger() -> Hsla { c(ud::R400) }
}

pub use crate::palette::{HUES, FONT_COLORS, BG_COLORS, CALLOUT_BG_COLORS, CELL_BG_COLORS, bg_canon};

fn hsla(c: crate::palette::Rgba) -> Hsla {
    ta(c.r, c.g, c.b, c.a)
}

pub fn text_color(name: &str) -> Option<Hsla> {
    crate::palette::text_color(name).map(hsla)
}

pub fn bg_color(name: &str) -> Option<Hsla> {
    crate::palette::bg_color(name).map(|c| crate::theme::mark(crate::theme::rgbaf(c.r, c.g, c.b, c.a)))
}

pub fn block_bg(name: &str) -> Option<Hsla> {
    crate::palette::block_bg(name).map(hsla)
}

pub fn bg_swatch(name: &str) -> Option<Hsla> {
    crate::palette::bg_swatch(name).map(hsla)
}

pub fn callout_bg(name: &str) -> Option<Hsla> {
    crate::palette::callout_bg(name).map(hsla)
}

pub fn callout_border(name: &str) -> Option<Hsla> {
    crate::palette::callout_border(name).map(hsla)
}

pub fn callout_border_swatch(name: &str) -> Option<Hsla> {
    crate::palette::callout_border_swatch(name).map(hsla)
}

// ---- layout metrics (px)

pub const SIDEBAR_WIDTH: f32 = 300.0;
pub const SIDEBAR_MIN: f32 = 268.0;
pub const SIDEBAR_MAX: f32 = 800.0;
/// The editor keeps at least this much room when the sidebar is dragged wider.
pub const SIDEBAR_MIN_CONTENT: f32 = 540.0;
/// Left inset of the header controls that sit beside the macOS traffic lights.
pub const TRAFFIC_LIGHTS_W: f32 = 92.0;

/// (font size, line height, margin top, margin bottom) per block.
pub struct BlockMetrics {
    pub size: f32,
    pub line_height: f32,
    pub mt: f32,
    pub mb: f32,
}

pub fn heading_metrics(level: u8) -> BlockMetrics {
    let (size, lh, mt, mb) = match level {
        1 => (26.0, 36.4, 26.0, 10.0),
        2 => (22.0, 30.8, 22.0, 8.0),
        3 => (20.0, 28.0, 20.0, 8.0),
        4 => (18.0, 25.2, 18.0, 8.0),
        5 => (16.0, 26.0, 18.0, 8.0),
        _ => (16.0, 26.0, 16.0, 8.0),
    };
    BlockMetrics { size, line_height: lh, mt, mb }
}

pub const TITLE: BlockMetrics = BlockMetrics { size: 34.0, line_height: 55.25, mt: 0.0, mb: 0.0 };

pub use crate::theme::{craft, ca, mix, ta, shadow_button};

impl Theme { pub fn link_hover() -> Hsla { c(ud::old::B600) } }

impl Theme { pub fn table_line() -> Hsla { ca(craft::N1000, 0.09) } }

impl Theme { pub fn window() -> Hsla { crate::theme::Theme::window() } }

impl Theme { pub fn sidebar_text() -> Hsla { ca(craft::N900, 0.85) } }

impl Theme { pub fn sidebar_hover() -> Hsla { ca(craft::N1000, 0.045) } }

impl Theme { pub fn sidebar_selected() -> Hsla { ca(craft::N1000, 0.072) } }

impl Theme { pub fn sidebar_section() -> Hsla { c(0x333739) } }

impl Theme { pub fn button_border() -> Hsla { ca(craft::N1000, 0.075) } }

impl Theme { pub fn code_chip() -> Hsla { c(0xf8f8f8) } }

impl Theme { pub fn code_chip_border() -> Hsla { c(0xe2e3e3) } }

impl Theme { pub fn code_bg() -> Hsla { c(0xfbfbfb) } }

impl Theme { pub fn code_gutter() -> Hsla { c(0xf8f8f8) } }

impl Theme { pub fn code_gutter_line() -> Hsla { c(0xe8e8e8) } }

impl Theme { pub fn code_border() -> Hsla { c(0xeaebeb) } }

impl Theme { pub fn code_line_number() -> Hsla { c(0xc2c2c3) } }

impl Theme { pub fn caption() -> Hsla { c(0x7e8287) } }

impl Theme { pub fn menu_highlight() -> Hsla { c(craft::B500) } }

impl Theme { pub fn popover_bg() -> Hsla { c(0xfafafa) } }

impl Theme { pub fn separator_dots() -> Hsla { ca(craft::N900, 0.5) } }

impl Theme { pub fn todo_box() -> Hsla { ca(craft::N1000, 0.22) } }

impl Theme { pub fn code_text() -> Hsla { c(0x4c4d50) } }

pub fn font_swatch(name: Option<&str>) -> Hsla { name.and_then(text_color).unwrap_or(Theme::text()) }
