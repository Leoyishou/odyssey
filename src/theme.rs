//! Design tokens: the app's palette, type scale, shapes and shadows (light).
//! The layout departs in one place: a sunken sidebar on the window surface
//! and a full-bleed white note plane instead of floating panels.

use gpui::{BoxShadow, Hsla, Rgba, WindowBackgroundAppearance, point, px, rgb};
use std::sync::atomic::{AtomicBool, Ordering};

// ---- dark mode
//
// Every colour here is designed for the light look. In dark mode the
// palette helpers (`c`, `ca`, `mix`, `ta`) and the theme files' colours
// pass through `adapt`, which mirrors lightness: the white page turns dark
// gray, near-black text near-white, tints dark tints, while vivid colours
// (the accent, coloured text) keep their hue and stay readable. Text
// highlights keep their light colours instead (`mark`, `on_mark`).
// `rgbaf` stays literal for shadows, scrims and controls drawn over images.

// ---- window glass

/// Whether windows sit on blurred glass. Windows 10 and 11 21H2 drag acrylic
/// windows with a lag (GitHub #32): there the window is opaque.
pub fn glass() -> bool {
    #[cfg(target_os = "windows")] {
        static GLASS: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *GLASS.get_or_init(|| windows_build().is_some_and(|b| b >= 22621))
    }
    #[cfg(not(target_os = "windows"))] { true }
}

#[cfg(target_os = "windows")]
fn windows_build() -> Option<u32> {
    let key = windows_registry::LOCAL_MACHINE.open(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion").ok()?;
    key.get_string("CurrentBuildNumber").ok()?.trim().parse().ok()
}

pub fn window_background() -> WindowBackgroundAppearance {
    if glass() { WindowBackgroundAppearance::Blurred } else { WindowBackgroundAppearance::Opaque }
}

static DARK: AtomicBool = AtomicBool::new(false);
thread_local! {
    /// Exports and printing draw light whatever the window shows.
    static FORCE_LIGHT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static ADAPTED: std::cell::RefCell<std::collections::HashMap<[u32; 4], Hsla>> = std::cell::RefCell::new(std::collections::HashMap::new());
}

pub fn dark() -> bool { DARK.load(Ordering::Relaxed) && !FORCE_LIGHT.with(|f| f.get()) }
/// Switch the look; true when it changed.
pub fn set_dark(on: bool) -> bool { DARK.swap(on, Ordering::Relaxed) != on }
/// Run `f` with the light colours (HTML export, printing, copy as rich text).
pub fn light<R>(f: impl FnOnce() -> R) -> R {
    let was = FORCE_LIGHT.with(|l| l.replace(true));
    let r = f();
    FORCE_LIGHT.with(|l| l.set(was));
    r
}

/// The colour to draw for light-look colour `c`: itself, or in dark mode its dark counterpart.
pub fn adapt(c: Hsla) -> Hsla {
    if !dark() { return c }
    let key = [c.h.to_bits(), c.s.to_bits(), c.l.to_bits(), c.a.to_bits()];
    if let Some(d) = ADAPTED.with(|m| m.borrow().get(&key).copied()) { return d }
    let d = to_dark(c);
    ADAPTED.with(|m| m.borrow_mut().insert(key, d));
    d
}

/// Lightness mirrored in OKLab: white → L 0.23 (the page, #1c1c1c), the
/// text (L 0.25) → L 0.91, black → white. Chroma and hue are kept; vivid
/// colours (chroma ≥ 0.14) keep their own lightness within 0.62–0.86, pale
/// ones follow the mirror, with a smooth blend between.
fn to_dark(c: Hsla) -> Hsla {
    let Rgba { r, g, b, a } = Rgba::from(c);
    let [ll, oa, ob] = oklab(r, g, b);
    let chroma = (oa * oa + ob * ob).sqrt();
    let mirror = (1.137 - 0.907 * ll).clamp(0., 1.);
    let t = ((chroma - 0.04) / 0.10).clamp(0., 1.);
    let t = t * t * (3. - 2. * t);
    let l2 = mirror + (ll.clamp(0.62, 0.86) - mirror) * t;
    srgb(l2, oa, ob, a)
}

/// The text highlight to draw for light-look highlight `c`: in dark mode
/// the same colour the light page shows (`c` over white, made opaque). Kept
/// translucent it lets the dark page through and turns muddy. Text on it
/// takes the light page's ink, see `on_mark`.
pub fn mark(c: Hsla) -> Hsla {
    if !dark() { return c }
    let Rgba { r, g, b, a } = Rgba::from(c);
    let w = |v: f32| v * a + 1. - a;
    Rgba { r: w(r), g: w(g), b: w(b), a: 1. }.into()
}

/// Text drawn on a highlight (`color`, already adapted). In dark mode it
/// gets back its light-page ink: the mirror undone (the near-white text →
/// L 0.25), at most L 0.55 so coloured text (red on light red) reads as it
/// does on the light page.
pub fn on_mark(c: Hsla) -> Hsla {
    if !dark() { return c }
    let Rgba { r, g, b, a } = Rgba::from(c);
    let [l, oa, ob] = oklab(r, g, b);
    let l2 = ((1.137 - l) / 0.907).clamp(0., 0.55);
    let chroma = (oa * oa + ob * ob).sqrt();
    let (sin, cos) = if chroma > 1e-4 { (ob / chroma, oa / chroma) } else { (0., 1.) };
    let k = in_gamut(l2, chroma, cos, sin);
    srgb(l2, k * cos, k * sin, a)
}

/// The largest chroma ≤ `max` at lightness `l` and hue (`cos`, `sin`) inside sRGB.
fn in_gamut(l: f32, max: f32, cos: f32, sin: f32) -> f32 {
    let fits = |k: f32| lrgb(l, k * cos, k * sin).iter().all(|v| (-1e-4..=1. + 1e-4).contains(v));
    if fits(max) { return max }
    let (mut lo, mut hi) = (0., max);
    for _ in 0..16 { let m = (lo + hi) / 2.; if fits(m) { lo = m } else { hi = m } }
    lo
}

// sRGB ⇄ OKLab with Björn Ottosson's matrices, kept digit for digit.
#[allow(clippy::excessive_precision)]
fn oklab(r: f32, g: f32, b: f32) -> [f32; 3] {
    let lin = |v: f32| if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) };
    let (r, g, b) = (lin(r), lin(g), lin(b));
    let l = (0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b).cbrt();
    let m = (0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b).cbrt();
    let s = (0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b).cbrt();
    [0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s, 1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s, 0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s]
}
/// OKLab → linear sRGB (unclamped: out of [0, 1] means out of gamut).
#[allow(clippy::excessive_precision)]
fn lrgb(l: f32, oa: f32, ob: f32) -> [f32; 3] {
    let l_ = l + 0.3963377774 * oa + 0.2158037573 * ob;
    let m_ = l - 0.1055613458 * oa - 0.0638541728 * ob;
    let s_ = l - 0.0894841775 * oa - 1.2914855480 * ob;
    let (l_, m_, s_) = (l_ * l_ * l_, m_ * m_ * m_, s_ * s_ * s_);
    [4.0767416621 * l_ - 3.3077115913 * m_ + 0.2309699292 * s_, -1.2684380046 * l_ + 2.6097574011 * m_ - 0.3413193965 * s_, -0.0041960863 * l_ - 0.7034186147 * m_ + 1.7076147010 * s_]
}
fn srgb(l: f32, oa: f32, ob: f32, a: f32) -> Hsla {
    let [r, g, b] = lrgb(l, oa, ob);
    let enc = |v: f32| { let v = v.clamp(0., 1.); if v <= 0.0031308 { v * 12.92 } else { 1.055 * v.powf(1. / 2.4) - 0.055 } };
    Rgba { r: enc(r), g: enc(g), b: enc(b), a }.into()
}

pub fn c(hex: u32) -> Hsla {
    adapt(rgb(hex).into())
}

/// A surface above the page (menus, dialogs, toasts, buttons, the picked
/// segment): `light` in the light look; in dark mode a step lighter than the
/// page, as raised surfaces are.
pub fn raised(light: Hsla) -> Hsla { if dark() { rgbaf(0x2c, 0x2c, 0x2e, 1.) } else { light } }

/// A literal colour, never adapted (shadows, scrims, glyphs over images).
pub fn rgbaf(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    Rgba { r: r as f32 / 255., g: g as f32 / 255., b: b as f32 / 255., a }.into()
}

/// `rgbaf` for the page and chrome: adapted in dark mode.
pub fn ta(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    adapt(rgbaf(r, g, b, a))
}

/// `hex` at opacity `a`.
pub fn ca(hex: u32, a: f32) -> Hsla {
    ta((hex >> 16) as u8, (hex >> 8) as u8, hex as u8, a)
}

/// Colour blend: `a` moved `t` of the way to `b` (opaque result).
pub fn mix(a: u32, b: u32, t: f32) -> Hsla {
    let ch = |s: u32| -> (f32, f32) { (((a >> s) & 0xff) as f32, ((b >> s) & 0xff) as f32) };
    let m = |(x, y): (f32, f32)| (x + (y - x) * t).round() as u8;
    ta(m(ch(16)), m(ch(8)), m(ch(0)), 1.)
}

/// The palette on the ramp names the code was written against
/// (`N*` neutrals, `B*` accent, `R*` / `O*` reds and oranges), so every
/// `craft::N300` still means "border", `N600` "secondary text" and so on.
/// Neutrals are measured greys; `B*` is built around the accent
/// (#3A85F7); `R*` / `O*` are Tailwind v4 ramps.
#[allow(dead_code)]
pub mod craft {
    pub const N00: u32 = 0xffffff;
    /// Code block fill.
    pub const N50: u32 = 0xfbfbfb;
    /// Popover / quick-entry body.
    pub const N100: u32 = 0xf7f7f7;
    /// Inactive chrome.
    pub const N200: u32 = 0xf1f2f3;
    /// Code block border, card hairlines (opaque on white).
    pub const N300: u32 = 0xeaebeb;
    /// Selected sidebar row, opaque.
    pub const N350: u32 = 0xdedfe0;
    pub const N400: u32 = 0xc9cacb;
    /// Quaternary text, inactive glyphs.
    pub const N500: u32 = 0x9c9fa0;
    /// Secondary text (dates, meta).
    pub const N600: u32 = 0x737477;
    pub const N650: u32 = 0x5d6064;
    /// Sidebar rows (text at 85% on the sidebar).
    pub const N700: u32 = 0x3f4143;
    /// Toolbar glyphs.
    pub const N800: u32 = 0x303335;
    /// Text (`.page` color).
    pub const N900: u32 = 0x1f2225;
    pub const N950: u32 = 0x141517;
    pub const N1000: u32 = 0x000000;
    pub const B50: u32 = 0xf0f5fe;
    pub const B100: u32 = 0xe3edfd;
    pub const B200: u32 = 0xc6dafc;
    pub const B300: u32 = 0x9cc0fa;
    pub const B350: u32 = 0x80aef9;
    pub const B400: u32 = 0x5d99f8;
    /// The accent: caret, primary button, menu highlight.
    pub const B500: u32 = 0x3a85f7;
    pub const B600: u32 = 0x2f74e0;
    pub const B700: u32 = 0x2660c0;
    pub const B800: u32 = 0x1d4b96;
    pub const B900: u32 = 0x14356a;
    pub const R50: u32 = 0xfef2f2;
    pub const R100: u32 = 0xffe2e2;
    pub const R200: u32 = 0xffc9c9;
    pub const R300: u32 = 0xffa2a2;
    pub const R350: u32 = 0xff6467;
    pub const R400: u32 = 0xfb2c36;
    pub const R500: u32 = 0xe7000b;
    pub const R600: u32 = 0xc10007;
    pub const R700: u32 = 0x9f0712;
    pub const R800: u32 = 0x82181a;
    pub const R900: u32 = 0x460809;
    pub const O50: u32 = 0xfff7ed;
    pub const O100: u32 = 0xffedd4;
    pub const O200: u32 = 0xffd6a8;
    pub const O300: u32 = 0xffb86a;
    pub const O350: u32 = 0xff8904;
    pub const O400: u32 = 0xff6900;
    pub const O500: u32 = 0xf54900;
    pub const O600: u32 = 0xca3500;
    pub const O700: u32 = 0x9f2d00;
    pub const O800: u32 = 0x7e2a0c;
    pub const O900: u32 = 0x441306;

    /// The secondary accents some controls use (dialog buttons, countdown,
    /// focus rings, success): one accent family and a green.
    pub mod old {
        pub const B50: u32 = 0xf0f5fe;
        pub const B100: u32 = 0xe3edfd;
        pub const B200: u32 = 0xc6dafc;
        pub const B300: u32 = 0x9cc0fa;
        pub const B400: u32 = 0x5d99f8;
        pub const B500: u32 = 0x3a85f7;
        pub const B600: u32 = 0x2f74e0;
        pub const B700: u32 = 0x2660c0;
        pub const G500: u32 = 0x00c950;
        pub const O500: u32 = 0xfe9a00;
    }

    /// Window surfaces (measured on the key window).
    /// The sunken surface: the sidebar material, here the whole window
    /// behind the note.
    pub const WINDOW: u32 = 0xeff0f1;
    /// The chrome between floating panels.
    pub const CHROME: u32 = 0xe6e7e8;
}

/// Shadows stay black; on the dark page they need more weight to show.
fn shadow(a: f32, y: f32, blur: f32, spread: f32) -> BoxShadow {
    let a = if dark() { (a * 3.).min(0.6) } else { a };
    BoxShadow { color: rgbaf(0, 0, 0, a), offset: point(px(0.), px(y)), blur_radius: px(blur), spread_radius: px(spread) }
}

/// Menus and dropdowns: macOS 26 menu panel, soft and wide.
pub fn shadow_menu() -> Vec<BoxShadow> {
    vec![shadow(0.05, 1., 3., 0.), shadow(0.10, 8., 24., 0.)]
}

/// Second-level panels, search popups, toasts.
pub fn shadow_panel() -> Vec<BoxShadow> {
    vec![shadow(0.05, 1., 4., 0.), shadow(0.12, 12., 32., 0.)]
}

/// Modal dialogs (the modal container shadow is a deep, soft 9-slice).
pub fn shadow_dialog() -> Vec<BoxShadow> {
    vec![shadow(0.06, 2., 8., 0.), shadow(0.16, 24., 64., 0.)]
}

/// Larger menus (「…」, home menus, icon picker): same family as menus.
pub fn shadow_s4_down() -> Vec<BoxShadow> {
    vec![shadow(0.05, 1., 3., 0.), shadow(0.11, 10., 28., 0.)]
}

/// Cards resting on the sidebar (更新日志): a faint lift, no border.
pub fn shadow_card() -> Vec<BoxShadow> {
    vec![shadow(0.04, 1., 2., 0.), shadow(0.06, 4., 14., 0.)]
}

/// Header buttons and other white pills on a white panel (back / share).
pub fn shadow_button() -> Vec<BoxShadow> {
    vec![shadow(0.05, 1., 1.5, 0.)]
}

pub const UI_FONT: &str = ".SystemUIFont";
/// SF Mono.
#[cfg(target_os = "macos")]
pub const MONO_FONT: &str = ".AppleSystemUIFontMonospaced";
#[cfg(not(target_os = "macos"))]
pub const MONO_FONT: &str = "Consolas";

pub struct Theme;

impl Theme {
    pub fn text() -> Hsla { c(craft::N900) }
    /// Secondary text: dates, meta, captions of controls.
    pub fn text_caption() -> Hsla { c(craft::N600) }
    pub fn text_placeholder() -> Hsla { c(craft::N500) }
    /// Empty-block hints: placeholders are text at ~25%.
    pub fn block_placeholder() -> Hsla { ca(craft::N900, 0.25) }
    pub fn text_disabled() -> Hsla { ca(craft::N900, 0.3) }
    /// Toolbar glyphs.
    pub fn text_code() -> Hsla { c(craft::N800) }
    /// The link colour: the content accent (#0087FF) blended half-way to the
    /// text colour, #0F5492 measured.
    pub fn link() -> Hsla { mix(0x0087ff, craft::N900, 0.5) }
    /// A hovered link or document chip: a step closer to the text colour.
    pub fn link_hover() -> Hsla { mix(0x0087ff, craft::N900, 0.65) }
    pub fn primary() -> Hsla { c(craft::B500) }
    pub fn primary_500() -> Hsla { c(craft::B500) }
    pub fn primary_fill_1() -> Hsla { c(craft::B50) }
    /// The note: white page.
    pub fn bg_body() -> Hsla { c(craft::N00) }
    /// Light fills inside the note (code blocks, quiet cards).
    pub fn bg_overlay() -> Hsla { c(craft::N100) }
    pub fn line_divider() -> Hsla { ca(craft::N1000, 0.06) }
    pub fn line_border() -> Hsla { ca(craft::N1000, 0.08) }
    pub fn fill_hover() -> Hsla { ca(craft::N1000, 0.045) }
    pub fn fill_hover_strong() -> Hsla { ca(craft::N1000, 0.06) }
    pub fn fill_pressed() -> Hsla { ca(craft::N1000, 0.09) }
    pub fn fill_turninto_active() -> Hsla { c(craft::B100) }
    /// Text selection: the accent, light.
    pub fn selection() -> Hsla { ca(craft::B500, 0.24) }
    /// A block whose handle menu is open.
    pub fn block_hover() -> Hsla { ca(craft::B500, 0.08) }
    /// Selected blocks.
    pub fn block_selected() -> Hsla { ca(craft::B500, 0.14) }
    pub fn handle_active() -> Hsla { ca(craft::N1000, 0.06) }
    pub fn size_cell() -> Hsla { c(craft::N100) }
    pub fn size_cell_border() -> Hsla { c(craft::N300) }
    pub fn size_cell_active() -> Hsla { c(craft::B300) }
    pub fn size_cell_disabled() -> Hsla { c(craft::B100) }
    /// Hairline drawn inside colour swatches (and the "none" swatch).
    pub fn swatch_border() -> Hsla { ca(craft::N1000, 0.08) }
    /// Hovered / focused to-do row.
    pub fn block_bg_area() -> Hsla { ca(craft::N1000, 0.03) }
    /// Done to-do text: faded to alpha 0.5.
    pub fn todo_done() -> Hsla { ca(craft::N900, 0.5) }
    pub fn scrollbar_thumb() -> Hsla { ca(craft::N1000, 0.22) }
    pub fn shadow() -> Hsla { ca(craft::N1000, 0.08) }
    /// Ring of a code block / callout whose handle menu is open.
    pub fn code_hover_ring() -> Hsla { ca(craft::B500, 0.35) }
    /// The focus decoration: a rounded bar in the text colour.
    pub fn quote_bar() -> Hsla { ca(craft::N900, 0.82) }
    /// The caret is the accent, 2pt wide.
    pub fn caret() -> Hsla { c(craft::B500) }
    pub fn pinned() -> Hsla { c(craft::B500) }
    /// Table header row / column fill.
    pub fn table_header() -> Hsla { c(0xf8f8f8) }
    /// Table grid lines.
    pub fn table_line() -> Hsla { ca(craft::N1000, 0.09) }
    pub fn table_bar_hover() -> Hsla { c(craft::N350) }
    pub fn table_bar_selected() -> Hsla { c(craft::B200) }
    pub fn danger_bar() -> Hsla { c(craft::R350) }
    pub fn danger_cell() -> Hsla { c(craft::R50) }
    pub fn danger() -> Hsla { c(craft::R500) }

    // ---- Quiet.md shell
    /// A light gray tint separates the sidebar from the white page while keeping wallpaper colors quiet.
    /// The document pane paints its own opaque white surface.
    pub fn window() -> Hsla { if crate::native_header::supported() { ca(craft::WINDOW, 0.) } else if crate::theme::glass() { ca(craft::WINDOW, 0.84) } else { ca(craft::WINDOW, 1.) } }
    /// Sidebar row label: text at 85%.
    pub fn sidebar_text() -> Hsla { ca(craft::N900, 0.85) }
    pub fn sidebar_hover() -> Hsla { ca(craft::N1000, 0.045) }
    /// Selected sidebar row (measured #DEDEDF).
    pub fn sidebar_selected() -> Hsla { ca(craft::N1000, 0.072) }
    /// Section headers on the sidebar.
    pub fn sidebar_section() -> Hsla { c(0x333739) }
    /// Chrome glyphs (sidebar rows, header buttons): lighter than text so
    /// they don't pull the eye from the labels and the page. Inferred: the
    /// SF Symbols sit near secondary label; this is between N700 and N600.
    pub fn icon() -> Hsla { ca(craft::N900, 0.6) }
    /// Line above the sidebar footer (1pt, black 3.5%, #E8E8E9 on the sidebar).
    /// Line under the page title.
    /// White pill buttons on the note (back / + / share): hairline.
    pub fn button_border() -> Hsla { ca(craft::N1000, 0.075) }
    /// Inline code chip: #F8F8F8 in a #E2E3E3 rim (measured).
    pub fn code_chip() -> Hsla { c(0xf8f8f8) }
    pub fn code_chip_border() -> Hsla { c(0xe2e3e3) }
    /// Code block surfaces.
    pub fn code_bg() -> Hsla { c(0xfbfbfb) }
    pub fn code_gutter() -> Hsla { c(0xf8f8f8) }
    pub fn code_gutter_line() -> Hsla { c(0xe8e8e8) }
    pub fn code_border() -> Hsla { c(0xeaebeb) }
    pub fn code_line_number() -> Hsla { c(0xc2c2c3) }
    /// Captions under media and code (weight 300).
    #[allow(dead_code)]
    pub fn caption() -> Hsla { c(0x7e8287) }
    /// Menu row highlight (native NSMenu: accent, white label).
    pub fn menu_highlight() -> Hsla { c(craft::B500) }
    /// Menus / popovers body (quick-entry & native menus).
    pub fn popover_bg() -> Hsla { c(0xfafafa) }
    /// Dotted separator: dots at half the text colour.
    pub fn separator_dots() -> Hsla { ca(craft::N900, 0.5) }
    /// Open task box rim.
    pub fn todo_box() -> Hsla { ca(craft::N1000, 0.22) }
    /// Code text: plain tokens and punctuation in code blocks.
    pub fn code_text() -> Hsla { c(0x4c4d50) }
}

#[allow(unused_imports)]
pub use crate::palette::{HUES, FONT_COLORS, BG_COLORS, CALLOUT_BG_COLORS, CELL_BG_COLORS, bg_canon};

/// Display colours for the DocxXML colour names. The names and the values
/// `palette` matches pasted DocxXML HTML against stay as they are; the page
/// shows its own text colours, marker highlights and block-decoration
/// tints (Tailwind v4 ramps).
mod hue {
    /// (text, marker highlight, block fill light, block fill medium, border, swatch)
    pub const fn of(h: &str) -> Option<(u32, u32, u32, u32, u32, u32)> {
        Some(match h.as_bytes() {
            // Text: the block colours (gray #9EA4AA, red #EF052A, orange
            // #FF9200, yellow #F9BD00, green #009E68, blue #004ABD, violet
            // #5700FF) darkened 15%, as coloured text is darkened (#004ABD
            // blocks set #0445AA / #093E8F text, measured).
            b"gray" => (0x868b90, 0xb8c0c5, 0xf5f5f5, 0xebebeb, 0xe0e0e0, 0x9ea4aa),
            b"red" => (0xcb0424, 0xff9898, 0xfef2f2, 0xffe2e2, 0xffc9c9, 0xef052a),
            b"orange" => (0xd97c00, 0xffb86a, 0xfff7ed, 0xffedd4, 0xffd6a8, 0xff9200),
            b"yellow" => (0xd4a100, 0xfff100, 0xfefce8, 0xfef9c2, 0xfff085, 0xf9bd00),
            b"green" => (0x008658, 0x39ff91, 0xecfdf5, 0xd0fae5, 0xa4f4cf, 0x009e68),
            b"blue" => (0x003fa1, 0x72c2ff, 0xeff6ff, 0xdbeafe, 0xbedbff, 0x004abd),
            b"purple" => (0x4a00d9, 0xc49bff, 0xf5f3ff, 0xede9fe, 0xddd6ff, 0x5700ff),
            _ => return None,
        })
    }
}

/// `light-red` → ("red", false), `medium-red` / `red` → ("red", true).
fn split(name: &str) -> Option<(&str, bool)> {
    if let Some(h) = name.strip_prefix("light-") {
        return Some((h, false));
    }
    if let Some(h) = name.strip_prefix("medium-") {
        return Some((h, true));
    }
    hue::of(name).map(|_| (name, true))
}

/// Font colour swatches as the colour panel shows them (first = default).
pub fn text_color(name: &str) -> Option<Hsla> {
    hue::of(name).map(|h| c(h.0))
}

/// Text highlight: the marker colours at 35% (yellow #FFF100 → #FFFAA6
/// measured), 65% for the strong row.
pub fn bg_color(name: &str) -> Option<Hsla> {
    let (h, strong) = split(name)?;
    let m = hue::of(h)?;
    let (r, g, b) = ((m.1 >> 16) as u8, (m.1 >> 8) as u8, m.1 as u8);
    Some(mark(rgbaf(r, g, b, if strong { 0.65 } else { 0.35 })))
}

/// Block background (blocks, table cells): the block decoration, the
/// block colour at 7.5% (#004ABD → #ECF1FA measured); the strong row at 15%.
pub fn block_bg(name: &str) -> Option<Hsla> {
    let (h, strong) = split(name)?;
    let m = hue::of(h)?;
    Some(if strong { mix(m.5, craft::N00, 0.85) } else { mix(m.5, craft::N00, 0.925) })
}

/// Block palette swatch (what the picker shows for a block colour).
pub fn bg_swatch(name: &str) -> Option<Hsla> {
    let (h, strong) = split(name)?;
    let m = hue::of(h)?;
    Some(if strong { c(m.4) } else { c(m.3) })
}

/// Callout fill: the block decoration tint.
pub fn callout_bg(name: &str) -> Option<Hsla> {
    block_bg(name)
}

/// Callout border: one step deeper than its fill.
pub fn callout_border(name: &str) -> Option<Hsla> {
    let h = name.strip_prefix("light-").or_else(|| name.strip_prefix("medium-")).unwrap_or(name);
    let h = if h == "neutral" { "gray" } else { h };
    hue::of(h).map(|m| c(m.4))
}

/// Border picker swatches.
pub fn callout_border_swatch(name: &str) -> Option<Hsla> {
    hue::of(name).map(|m| c(m.5))
}

// ---- layout metrics (pt)

/// The sidebar is 273pt wide plus its 7pt inset; the sunken
/// sidebar keeps that footprint.
pub const SIDEBAR_WIDTH: f32 = 280.0;
pub const SIDEBAR_MIN: f32 = 240.0;
pub const SIDEBAR_MAX: f32 = 800.0;
/// The editor keeps at least this much room when the sidebar is dragged wider.
pub const SIDEBAR_MIN_CONTENT: f32 = 540.0;
/// Left inset of the header controls that sit beside the macOS traffic lights.
pub const TRAFFIC_LIGHTS_W: f32 = if cfg!(target_os = "macos") { 92.0 } else { 16.0 };

/// (font size, line height, margin top, margin bottom) per block.
pub struct BlockMetrics {
    pub size: f32,
    pub line_height: f32,
    pub mt: f32,
    pub mb: f32,
}

/// The text styles on Mac (iOS px × 0.882353): Title 21 / 700,
/// Subtitle 19 / 700, Heading 18 / 600, Strong 16 / 600; H5+ are body size
/// at 600. Line heights are multipliers on the font's natural line
/// height (1.19em); margins are padding-top / -bottom.
pub fn heading_metrics(level: u8) -> BlockMetrics {
    // Paddings are px as pt: measured gaps match them unscaled.
    let (size, lh, mt, mb) = match level {
        1 => (21.0, 27.0, 48.0, 16.0),
        2 => (19.0, 25.0, 36.0, 16.0),
        3 => (18.0, 24.0, 28.0, 12.0),
        4 => (16.0, 25.0, 16.0, 16.0),
        _ => (15.0, 22.0, 13.0, 13.0),
    };
    BlockMetrics { size, line_height: lh, mt, mb }
}

/// Body text: 15pt, 1.25 × natural line height ≈ 22pt; single-line
/// paragraphs repeat every 35pt, so 13pt between blocks.
pub const BODY_SIZE: f32 = 15.0;
pub const BODY_LINE: f32 = 22.0;
pub const BLOCK_GAP: f32 = 13.0;

/// Heading weight per level (Title / Subtitle 700, Heading / Strong 600).
pub fn heading_weight(level: u8) -> gpui::FontWeight {
    if level <= 2 { gpui::FontWeight::BOLD } else { gpui::FontWeight::SEMIBOLD }
}

/// Page title: Quiet.md uses 34 / 55.25 Bold, larger than the 21pt
/// reference `.pageTitle`, to match preview typography; the larger title is
/// an intentional deviation, not a measurement.
pub const TITLE: BlockMetrics = BlockMetrics { size: 34.0, line_height: 55.25, mt: 0.0, mb: 0.0 };
