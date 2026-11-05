//! 文档信息 dialog: cut down to what a local file has (概览 times and
//! 文档统计). 访问记录 / 操作记录 / 隐私设置 and the
//! owner / interaction stats are cloud-only.

use crate::app::Workspace;
use crate::model::Document;
use crate::theme::Theme;
use crate::ui::{format_date, format_relative, icon};
use gpui::{prelude::FluentBuilder, *};

#[derive(Clone, Debug, PartialEq)]
pub struct DocInfo {
    pub created: u64,
    pub modified: u64,
    pub words: usize,
    pub chars: usize,
    pub size: u64,
}

fn is_cjk(c: char) -> bool {
    matches!(c as u32,
        0x3400..=0x4dbf | 0x4e00..=0x9fff | 0xf900..=0xfaff | 0x20000..=0x2fa1f
        | 0x3040..=0x30ff | 0xac00..=0xd7af)
}

/// 字数: each CJK character and full-width punctuation mark counts as one,
/// a run of Latin letters / digits counts as one word; ASCII punctuation and
/// spaces don't count.
pub fn word_count(s: &str) -> usize {
    let mut n = 0;
    let mut in_word = false;
    for c in s.chars() {
        let wordy = c.is_alphanumeric() && !is_cjk(c) || (in_word && matches!(c, '\'' | '’' | '-' | '.' | '_'));
        if wordy {
            if !in_word {
                n += 1;
            }
            in_word = true;
            continue;
        }
        in_word = false;
        if c.is_whitespace() || c.is_ascii() || c == '\u{fffc}' {
            continue;
        }
        n += 1;
    }
    n
}

/// `word_count` over blocks (children and table cells included) without
/// joining their text; block boundaries separate words anyway.
pub fn blocks_word_count(blocks: &[crate::model::Block]) -> usize {
    blocks
        .iter()
        .map(|b| {
            let cells = match &b.kind {
                crate::model::BlockKind::Table(t) => t.rows.iter().flatten().map(|c| blocks_word_count(&c.blocks)).sum(),
                _ => 0,
            };
            word_count(&b.text.text) + blocks_word_count(&b.children) + cells
        })
        .sum()
}

/// 字符数: every visible character (letters, digits, CJK, punctuation); no spaces.
pub fn char_count(s: &str) -> usize {
    s.chars().filter(|c| !c.is_whitespace() && *c != '\u{fffc}').count()
}

/// Title plus body, the text the counts run over.
pub fn doc_text(doc: &Document) -> String {
    format!("{}\n{}", doc.title.text, doc.plain_text())
}

pub fn format_size(n: u64) -> String {
    const K: f64 = 1024.;
    let f = n as f64;
    if f < K {
        format!("{n} B")
    } else if f < K * K {
        format!("{:.2} KB", f / K)
    } else if f < K * K * K {
        format!("{:.2} MB", f / K / K)
    } else {
        format!("{:.2} GB", f / K / K / K)
    }
}

impl Workspace {
    pub fn open_doc_info(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.flush(cx);
        let (Some(id), Some(ed)) = (self.current.clone(), self.editor.clone()) else { return };
        let Some(meta) = self.entries.iter().find(|e| e.id == id).map(|e| e.meta.clone()) else { return };
        let doc = &ed.read(cx).core.doc;
        let text = doc_text(doc);
        self.doc_info = Some(DocInfo {
            created: meta.created,
            modified: meta.modified,
            words: word_count(&text),
            chars: char_count(&text),
            size: self.store.size_of(&id, doc),
        });
        self.doc_menu = false;
        self.doc_sub = None;
        window.focus(&self.focus);
        cx.notify();
    }

    /// The modal (white, radius 16): a 15pt semibold title with the grey
    /// close disc, a quiet selected nav item and soft tiles.
    pub(crate) fn render_doc_info(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let info = self.doc_info.clone()?;
        let section = |title: &'static str| {
            div().text_size(px(13.)).line_height(px(20.)).font_weight(FontWeight::SEMIBOLD).text_color(Theme::text()).child(title)
        };
        let tile = |label: &'static str, value: String, big: bool| {
            div()
                .w(px(278.))
                .h(px(if big { 44. } else { 40. }))
                .pl(px(16.))
                .flex()
                .items_center()
                .gap(px(16.))
                .rounded(px(10.))
                .bg(Theme::bg_overlay())
                .child(div().text_size(px(13.)).line_height(px(20.)).text_color(Theme::text_caption()).child(label))
                .child(
                    div()
                        .text_size(px(if big { 19. } else { 13. }))
                        .line_height(px(if big { 26. } else { 20. }))
                        .when(big, |d| d.font_weight(FontWeight::SEMIBOLD))
                        .text_color(Theme::text())
                        .child(value),
                )
        };
        let grid = |tiles: Vec<Div>| div().mt(px(12.)).flex().flex_wrap().gap_x(px(12.)).gap_y(px(8.)).children(tiles);
        let close = cx.listener(|w, _, _, cx| {
            w.doc_info = None;
            cx.notify();
        });
        let modal = crate::trash::dialog("doc-info")
            .w(px(840.))
            .h(px(562.))
            .flex()
            .flex_col()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(
                div()
                    .flex_none()
                    .h(px(72.))
                    .px(px(24.))
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(Theme::line_divider())
                    .child(div().text_size(px(15.)).line_height(px(22.)).font_weight(FontWeight::SEMIBOLD).child(crate::settings::tr("文档信息", "Document information")))
                    .child(crate::trash::close_button("doc-info-close").mr(px(3.)).on_click(close)),
            )
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .child(
                        div().flex_none().w(px(224.)).h_full().pt(px(12.)).px(px(12.)).border_r_1().border_color(Theme::line_divider()).child(
                            // The one nav item, selected the way the sidebar selects: a quiet pill.
                            div()
                                .h(px(30.))
                                .px(px(10.))
                                .flex()
                                .items_center()
                                .gap(px(8.))
                                .rounded(px(9.))
                                .bg(Theme::fill_hover_strong())
                                .text_size(px(13.))
                                .line_height(px(20.))
                                .text_color(Theme::text())
                                .child(icon("list", 15., Theme::text_code()))
                                .child(crate::settings::text("信息与数据")),
                        ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .p(px(24.))
                            .flex()
                            .flex_col()
                            .child(section(crate::settings::text("概览")))
                            .child(grid(vec![
                                tile(crate::settings::text("创建时间"), format_date(info.created), false),
                                tile(crate::settings::text("最近修改"), format_relative(info.modified), false),
                            ]))
                            .child(div().mt(px(24.)).child(section(crate::settings::text("文档统计"))))
                            .child(grid(vec![
                                tile(crate::settings::text("总字数"), info.words.to_string(), true),
                                tile(crate::settings::text("总字符数"), info.chars.to_string(), true),
                                tile(crate::settings::text("大小"), format_size(info.size), true),
                            ])),
                    ),
            );
        Some(
            deferred(
                div()
                    .id("doc-info-mask")
                    .absolute()
                    .inset_0()
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(crate::trash::scrim())
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|w, _, _, cx| {
                            w.doc_info = None;
                            cx.notify();
                        }),
                    )
                    .child(modal),
            )
            .with_priority(3)
            .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{char_count, format_size, word_count};

    #[test]
    fn counts_like_feishu() {
        assert_eq!(word_count("你好，世界！"), 6);
        assert_eq!(word_count("hello world 2025"), 3);
        assert_eq!(word_count("飞书 docs, v1.5 版本."), 6);
        assert_eq!(char_count("飞书 docs, v1.5"), 11);
        assert_eq!(char_count("  \n"), 0);
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1_898_000), "1.81 MB");
    }
}
