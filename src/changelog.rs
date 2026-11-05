//! 更新日志: after an update, a quiet card at the foot of the sidebar lists
//! what this version changed, once. The notes are compiled in (no network);
//! a new library (first launch) never sees it.
use std::sync::atomic::{AtomicBool, Ordering};

use gpui::prelude::*;
use gpui::{div, px, AnyElement, ClickEvent, FontWeight, Window, App};

use crate::theme::Theme;
use crate::ui::icon;

const VERSION: &str = "odyssey-build-5";
const RADIUS: f32 = 12.;

/// Each version's notes: (version, [(中文, English)]). Only the running
/// version's are shown; a version without notes shows nothing.
const NOTES: &[(&str, &[(&str, &str)])] = &[
    (VERSION, &[
        ("双括号文档搜索与显示别名", "Document search with double brackets and display aliases"),
        ("正文与表格中的纯文字链接、悬浮预览", "Plain text document links and previews in paragraphs and tables"),
        ("本地独立文档库，更多工作区能力仍在开发", "Independent local library; more workspace features are in development"),
    ]),
];

static PENDING: AtomicBool = AtomicBool::new(false);

fn notes() -> Option<&'static [(&'static str, &'static str)]> { NOTES.iter().find(|(v, _)| *v == VERSION).map(|(_, n)| *n) }

/// At launch, once preferences and themes are loaded: a library that existed
/// before this launch and was last opened by an older version gets the card.
/// Records this version either way, so it shows only once.
pub fn startup(existed: bool) {
    let mut p = crate::settings::prefs();
    if p.seen_version == VERSION { return }
    let upgraded = existed && p.seen_version != VERSION;
    PENDING.store(upgraded && notes().is_some(), Ordering::Relaxed);
    p.seen_version = VERSION.into();
    if let Err(e) = crate::settings::save(&p) { log::warn!("changelog: {e:#}"); }
}

/// The card, while it has not been closed.
pub fn card(on_close: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Option<AnyElement> {
    if !PENDING.load(Ordering::Relaxed) { return None }
    let notes = notes()?;
    let title = crate::settings::tr("Odyssey 开发版", "Odyssey development build");
    let mut list = div().mt(px(6.)).w_full().min_w_0().flex().flex_col().gap(px(4.));
    for &(zh, en) in notes {
        list = list.child(div().w_full().min_w_0().flex().gap(px(6.)).text_size(px(12.)).line_height(px(18.)).text_color(Theme::text_caption())
            .child(div().flex_none().child("·")).child(div().flex_1().min_w_0().child(crate::settings::tr(zh, en))));
    }
    // A continuous-corner white surface; the faint shadow follows its radius.
    Some(crate::ui::continuous_surface(RADIUS, Theme::bg_body()).id("changelog").flex_none().min_w_0().overflow_hidden().mx(px(crate::sidebar::SB_X - 4.)).mb(px(2.)).pl(px(12.)).pr(px(6.)).pt(px(8.)).pb(px(10.))
        .rounded(px(RADIUS)).shadow(crate::theme::shadow_card())
        .child(div().w_full().min_w_0().flex().items_center().justify_between()
            .child(div().text_size(px(13.)).line_height(px(20.)).font_weight(FontWeight::SEMIBOLD).child(title))
            .child(div().id("changelog-close").size(px(20.)).flex().items_center().justify_center().rounded(px(6.)).cursor_pointer()
                .hover(|s| s.bg(Theme::fill_hover())).child(icon("x", 12., Theme::text_caption()))
                .on_click(move |e, w, cx| { PENDING.store(false, Ordering::Relaxed); on_close(e, w, cx); })))
        .child(list)
        .into_any_element())
}
