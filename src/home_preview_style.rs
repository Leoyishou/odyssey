//! Document presentation shared by the editor and library thumbnails: the
//! type scale, container geometry and the mapping from legacy colour names.
use crate::model::BlockKind;
use crate::editor_theme::{self as theme, Theme};
use gpui::*;

pub const SCALE: f32 = 0.28;
pub const RADIUS: f32 = 10.;
// Keep the existing callout's closed border: a focus decoration is a separate
// quote treatment, not a replacement for a stored border colour.
pub const CALLOUT_RADIUS: f32 = 8.;
pub const INSET: f32 = 16.;
pub const ICON: f32 = 22.;
pub const ICON_GAP: f32 = 12.;
// Native Styling Basics sample, measured at 2x; these are not PageStyles CSS.
pub const QUOTE_INSET: f32 = 14.;
pub const QUOTE_BAR: f32 = 4.;
pub const QUOTE_PAD: f32 = 4.;
pub const PARAGRAPH_GAP: f32 = 11.;
pub const LIST_GAP: f32 = 5.;
pub const CODE_HEADER: f32 = 36.;
pub const CODE_GUTTER: f32 = 40.;
pub const CODE_GAP: f32 = 16.;

pub fn typography(kind: &BlockKind) -> theme::BlockMetrics {
    match kind {
        BlockKind::Heading { level, .. } => theme::heading_metrics(*level),
        BlockKind::Code { .. } => theme::BlockMetrics { size: 13., line_height: 20., mt: 16., mb: 16. },
        _ => theme::BlockMetrics { size: theme::body_size(), line_height: theme::body_line(), mt: 0., mb: PARAGRAPH_GAP },
    }
}

pub fn weight(kind: &BlockKind) -> FontWeight {
    if crate::settings::feishu() { return if matches!(kind, BlockKind::Heading { .. }) { FontWeight::MEDIUM } else { FontWeight::NORMAL }; }
    match kind { BlockKind::Heading { level: 1 | 2, .. } => FontWeight::BOLD, BlockKind::Heading { .. } => FontWeight::SEMIBOLD, _ => FontWeight::NORMAL }
}

pub fn gap(kind: &BlockKind) -> (f32, f32) {
    match kind {
        BlockKind::Heading { .. } => { let m = typography(kind); (m.mt, m.mb) }
        BlockKind::Bullet | BlockKind::Ordered { .. } | BlockKind::Todo { .. } => (0., LIST_GAP),
        BlockKind::Table(_) => (11., 11.),
        BlockKind::Code { .. } | BlockKind::Callout { .. } | BlockKind::Image { .. } | BlockKind::File(_) | BlockKind::Grid => (16., 16.),
        BlockKind::Quote => (8., 14.),
        BlockKind::Divider => (12., 12.),
        _ => (0., PARAGRAPH_GAP),
    }
}

// Retain the user's hue while giving old saturated highlight colours the
// same light/dark surface contrast as the document's other decorations.
pub fn highlight(c: Hsla) -> Hsla { c }
pub fn fill(name: Option<&str>) -> Hsla { name.and_then(theme::callout_bg).unwrap_or_else(transparent_black) }
pub fn border(name: Option<&str>) -> Hsla { name.and_then(theme::callout_border).unwrap_or_else(transparent_black) }
pub fn code_surface() -> Hsla { Theme::code_bg() }
pub fn code_header() -> Hsla { Theme::code_gutter() }
pub fn table_line() -> Hsla { Theme::line_border() }
pub fn display_fill(value: Hsla) -> Hsla { value }
pub fn display_ink(value: Hsla) -> Hsla { value }
