//! The named colors of DocxXML (`text-color` / `background-color` values)
//! and the swatch orders of the color panels. `theme` turns these into GPUI
//! colors; paste matches clipboard colors against them.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

pub const fn hex(v: u32) -> Rgba {
    Rgba { r: (v >> 16) as u8, g: (v >> 8) as u8, b: v as u8, a: 1. }
}

pub const fn rgba(r: u8, g: u8, b: u8, a: f32) -> Rgba {
    Rgba { r, g, b, a }
}

pub const HUES: [&str; 7] = ["gray", "red", "orange", "yellow", "green", "blue", "purple"];

/// Font color swatches in panel order (first = default).
pub const FONT_COLORS: [(Option<&str>, u32); 8] = [
    (None, 0x1f2329),
    (Some("gray"), 0x8f959e),
    (Some("red"), 0xd83931),
    (Some("orange"), 0xde7802),
    (Some("yellow"), 0xdc9b04),
    (Some("green"), 0x2ea121),
    (Some("blue"), 0x245bdb),
    (Some("purple"), 0x6425d0),
];

/// Text highlight swatches of the A panel in order: two rows of 8 (row 1
/// starts with "none").
pub const BG_COLORS: [Option<&str>; 16] = [
    None,
    Some("light-gray"),
    Some("light-red"),
    Some("light-orange"),
    Some("light-yellow"),
    Some("light-green"),
    Some("light-blue"),
    Some("light-purple"),
    Some("medium-gray"),
    Some("gray"),
    Some("red"),
    Some("orange"),
    Some("yellow"),
    Some("green"),
    Some("blue"),
    Some("purple"),
];

/// Callout fill swatches (§9.1), the block-background palette.
pub const CALLOUT_BG_COLORS: [Option<&str>; 16] = [
    None,
    Some("light-gray"),
    Some("light-red"),
    Some("light-orange"),
    Some("light-yellow"),
    Some("light-green"),
    Some("light-blue"),
    Some("light-purple"),
    Some("medium-gray"),
    Some("gray"),
    Some("medium-red"),
    Some("medium-orange"),
    Some("medium-yellow"),
    Some("medium-green"),
    Some("medium-blue"),
    Some("medium-purple"),
];

pub fn text_color(name: &str) -> Option<Rgba> {
    FONT_COLORS.iter().find(|(n, _)| *n == Some(name)).map(|(_, v)| hex(*v))
}

/// Text highlight for a DocxXML color name: the
/// `--ccmtoken-doc-highlightcolor-*` tokens; the panel swatch is the same
/// color. `medium-{hue}` (written by older builds) reads as the row-2 hue.
pub fn bg_color(name: &str) -> Option<Rgba> {
    Some(match name {
        "light-gray" => hex(0xf2f3f5),
        "light-red" => hex(0xfbbfbc),
        "light-orange" => rgba(254, 212, 164, 0.8),
        "light-yellow" => rgba(255, 246, 122, 0.8),
        "light-green" => rgba(183, 237, 177, 0.8),
        "light-blue" => rgba(186, 206, 253, 0.7),
        "light-purple" => rgba(205, 178, 250, 0.7),
        "medium-gray" => rgba(222, 224, 227, 0.8),
        "gray" => hex(0xbbbfc4),
        "medium-red" | "red" => hex(0xf76964),
        "medium-orange" | "orange" => hex(0xffa53d),
        "medium-yellow" | "yellow" => hex(0xffe928),
        "medium-green" | "green" => hex(0x62d256),
        "medium-blue" | "blue" => rgba(78, 131, 253, 0.55),
        "medium-purple" | "purple" => rgba(147, 90, 246, 0.55),
        _ => return None,
    })
}

/// The A panel's name for a text highlight (`medium-red` → `red`).
pub fn bg_canon(name: &str) -> &str {
    match name.strip_prefix("medium-") {
        Some(h) if h != "gray" => h,
        _ => name,
    }
}

/// Block background for a DocxXML color name (behavior-spec §7.4
/// "实际背景" column, core.js `Li`): block backgrounds, table cells and
/// callout fills.
pub fn block_bg(name: &str) -> Option<Rgba> {
    Some(match name {
        "light-gray" => rgba(245, 246, 247, 0.9),
        "light-red" => hex(0xfef1f1),
        "light-orange" => hex(0xfff5eb),
        "light-yellow" => hex(0xfefff0),
        "light-green" => hex(0xf0fbef),
        "light-blue" => hex(0xf0f4ff),
        "light-purple" => hex(0xf6f1fe),
        "medium-gray" => hex(0xf2f3f5),
        "gray" => hex(0xeff0f1),
        "medium-red" | "red" => hex(0xfde2e2),
        "medium-orange" | "orange" => hex(0xfeead2),
        "medium-yellow" | "yellow" => hex(0xffffcc),
        "medium-green" | "green" => hex(0xd9f5d6),
        "medium-blue" | "blue" => hex(0xe1eaff),
        "medium-purple" | "purple" => hex(0xece2fe),
        _ => return None,
    })
}

/// Block background palette (单元格背景颜色, and 背景颜色 in the handle
/// menu's 颜色): two rows of eight, the first slot of row one restoring the
/// default.
pub const CELL_BG_COLORS: [Option<&str>; 16] = [
    None,
    Some("light-gray"),
    Some("light-red"),
    Some("light-orange"),
    Some("light-yellow"),
    Some("light-green"),
    Some("light-blue"),
    Some("light-purple"),
    Some("medium-gray"),
    Some("gray"),
    Some("red"),
    Some("orange"),
    Some("yellow"),
    Some("green"),
    Some("blue"),
    Some("purple"),
];

/// A block palette swatch (core.js `Gk`, the `panelBackground` map). The
/// colour applied to the cell or block is the paler `block_bg`.
pub fn bg_swatch(name: &str) -> Option<Rgba> {
    Some(match name {
        "light-gray" => rgba(239, 240, 241, 0.6),
        "light-red" => hex(0xfde2e2),
        "light-orange" => hex(0xfeead2),
        "light-yellow" => hex(0xffffcc),
        "light-green" => hex(0xd9f5d6),
        "light-blue" => hex(0xe1eaff),
        "light-purple" => hex(0xece2fe),
        "medium-gray" => rgba(222, 224, 227, 0.7),
        "gray" => rgba(187, 191, 196, 0.5),
        "red" => hex(0xfbbfbc),
        "orange" => hex(0xfed4a4),
        "yellow" => hex(0xfffca3),
        "green" => hex(0xb7edb1),
        "blue" => hex(0xbacefd),
        "purple" => hex(0xcdb2fa),
        _ => return block_bg(name),
    })
}

/// Callout fill (§9.1): the block background except light gray.
pub fn callout_bg(name: &str) -> Option<Rgba> {
    match name {
        "light-gray" => Some(hex(0xf5f6f7)),
        _ => block_bg(name),
    }
}

/// Callout border, "light" set used for new callouts (§9.1). The darker
/// legacy set is accepted on read through the same hue names.
pub fn callout_border(name: &str) -> Option<Rgba> {
    Some(match name {
        "gray" | "light-gray" | "medium-gray" | "neutral" => hex(0xeff0f1),
        "red" | "light-red" | "medium-red" => hex(0xfbbfbc),
        "orange" | "light-orange" | "medium-orange" => hex(0xfed4a4),
        "yellow" | "light-yellow" | "medium-yellow" => hex(0xfff67a),
        "green" | "light-green" | "medium-green" => hex(0xb7edb1),
        "blue" | "light-blue" | "medium-blue" => hex(0xbacefd),
        "purple" | "light-purple" | "medium-purple" => hex(0xcdb2fa),
        _ => return None,
    })
}

/// Border picker swatches (§9.1 "面板 swatch").
pub fn callout_border_swatch(name: &str) -> Option<Rgba> {
    Some(match name {
        "gray" => hex(0xdee0e3),
        "red" => hex(0xf98e8b),
        "orange" => hex(0xffba6b),
        "yellow" => hex(0xfff67a),
        "green" => hex(0x8ee085),
        "blue" => hex(0x82a7fc),
        "purple" => hex(0xad82f7),
        _ => return None,
    })
}
