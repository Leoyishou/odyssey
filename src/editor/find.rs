//! 查找和替换 (⌘F / ⌘⇧H): the floating box at the top right of the page,
//! match search over every visible text line and the replace commands.

use super::core::{EditKind, Pos, Selection, TITLE};
use super::view::Editor;
use crate::input::TextField;
use crate::model::*;
use crate::feishu_theme::{self as theme, Theme, craft};
use crate::editor_ui::{ROW_RADIUS, TipExt, TipSide, icon, popover};
use gpui::{prelude::FluentBuilder, *};
use std::ops::Range;

pub struct Find {
    /// 替换 tab selected.
    pub replace: bool,
    /// Hits in document order.
    pub matches: Vec<(BlockId, Range<usize>)>,
    pub current: Option<usize>,
}

/// Case-insensitive, non-overlapping hits of `q` in `text`, skipping inline objects.
pub fn find_in(text: &RichText, q: &str) -> Vec<Range<usize>> {
    let fold = |c: char| c.to_lowercase().next().unwrap_or(c);
    let q: Vec<char> = q.chars().map(fold).collect();
    if q.is_empty() {
        return vec![];
    }
    let chars: Vec<(usize, char)> = text.text.char_indices().collect();
    let raw: Vec<Range<usize>> = text.spans().filter(|(_, m)| m.inline.is_some()).map(|(r, _)| r).collect();
    let mut out = vec![];
    let mut i = 0;
    while i + q.len() <= chars.len() {
        if chars[i..i + q.len()].iter().zip(&q).all(|((_, c), k)| fold(*c) == *k) {
            let start = chars[i].0;
            let end = chars.get(i + q.len()).map_or(text.len(), |(o, _)| *o);
            if !raw.iter().any(|r| r.start < end && start < r.end) {
                out.push(start..end);
                i += q.len();
                continue;
            }
        }
        i += 1;
    }
    out
}

const BOX_W: f32 = 404.;
const BTN_W: f32 = 80.;

impl Editor {
    pub fn open_find(&mut self, replace: bool, window: &mut Window, cx: &mut Context<Self>) {
        // Seed the query with a single-line selection, like the browser find bar.
        let (a, b) = self.core.ordered();
        if !self.core.in_block_mode() && a.block == b.block && a.offset < b.offset {
            let s = self.core.text_of(a.block).and_then(|t| t.text.get(a.offset..b.offset)).unwrap_or("").to_string();
            if !s.contains('\n') {
                self.find_field.update(cx, |f, cx| f.set_text(&s, cx));
            }
        }
        self.find = Some(Find { replace, matches: vec![], current: None });
        self.slash = None;
        self.menu = None;
        // 13pt inputs (the fields are made with the 14pt default).
        for f in [&self.find_field, &self.replace_field] {
            f.update(cx, |f, _| {
                f.size = 13.;
                f.line_height = 20.;
            });
        }
        self.refresh_find(true, cx);
        let field = self.find_field.clone();
        window.focus(&field.focus_handle(cx));
        field.update(cx, |f, cx| f.select_all_text(cx));
        self.scroll_to_current(window);
        cx.notify();
    }

    pub fn close_find(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(f) = self.find.take() else { return };
        // Leave the current hit selected.
        if let Some((id, r)) = f.current.and_then(|i| f.matches.get(i).cloned()) {
            self.core.block_sel = None;
            self.core.sel = Selection { anchor: Pos::new(id, r.start), head: Pos::new(id, r.end) };
            self.toolbar_hidden = true;
        }
        window.focus(&self.focus);
        cx.notify();
    }

    /// Recompute hits. `from_caret` picks the first hit at or after the caret,
    /// otherwise the current hit stays where it was in the document.
    pub fn refresh_find(&mut self, from_caret: bool, cx: &App) {
        let Some(old) = self.find.take() else { return };
        let q = self.find_field.read(cx).text.clone();
        let units = self.core.units();
        let mut matches = vec![];
        for u in units.iter().filter(|u| u.text) {
            if let Some(t) = self.core.text_of(u.id) {
                matches.extend(find_in(t, &q).into_iter().map(|r| (u.id, r)));
            }
        }
        let order = |id: BlockId| units.iter().position(|u| u.id == id).unwrap_or(usize::MAX);
        let anchor = if from_caret {
            let h = self.core.ordered().0;
            Some((order(h.block), h.offset))
        } else {
            old.current.and_then(|i| old.matches.get(i)).map(|(id, r)| (order(*id), r.start))
        };
        let current = (!matches.is_empty()).then(|| {
            anchor
                .and_then(|a| matches.iter().position(|(id, r)| (order(*id), r.start) >= a))
                .unwrap_or(0)
        });
        self.find = Some(Find { replace: old.replace, matches, current });
    }

    /// Hits inside one text line, for painting.
    pub fn find_hits(&self, id: BlockId) -> Vec<(Range<usize>, bool)> {
        let Some(f) = &self.find else { return vec![] };
        f.matches
            .iter()
            .enumerate()
            .filter(|(_, (b, _))| *b == id)
            .map(|(i, (_, r))| (r.clone(), f.current == Some(i)))
            .collect()
    }

    fn step_find(&mut self, forward: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(f) = &mut self.find else { return };
        let n = f.matches.len();
        if n == 0 {
            return;
        }
        f.current = Some(match f.current {
            Some(i) if forward => (i + 1) % n,
            Some(i) => (i + n - 1) % n,
            None => 0,
        });
        self.scroll_to_current(window);
        cx.notify();
    }

    /// Bring the current hit into view (centered when it was off screen).
    fn scroll_to_current(&mut self, window: &mut Window) {
        let Some((id, r)) = self.find.as_ref().and_then(|f| f.matches.get(f.current?).cloned()) else { return };
        self.reveal = Some((super::render::Reveal::Find(Pos::new(id, r.start)), 0));
        window.refresh();
    }

    fn replace_range(&mut self, id: BlockId, r: Range<usize>, with: &str) {
        let Some(t) = self.core.text_mut(id) else { return };
        let mut marks = t.slice(r.clone()).runs.first().map(|run| run.marks.clone()).unwrap_or_default();
        marks.inline = None;
        if id == TITLE {
            marks = Marks::default();
        }
        let mut ins = RichText::default();
        ins.push(with, marks);
        t.replace(r, &ins);
    }

    fn after_replace(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.core.fix_selection();
        if self.core.dirty {
            self.core.dirty = false;
            cx.emit(super::view::EditorEvent::Changed);
        }
        self.refresh_find(false, cx);
        self.scroll_to_current(window);
        cx.notify();
    }

    fn replace_one(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some((id, r)) = self.find.as_ref().and_then(|f| f.matches.get(f.current?).cloned()) else { return };
        let with = self.replace_field.read(cx).text.clone();
        self.core.checkpoint(EditKind::Other);
        self.replace_range(id, r.clone(), &with);
        // Keep "current" on the hit that followed the replaced one.
        if let Some(f) = &mut self.find {
            let next = f.current.map_or(0, |i| i + 1);
            f.current = f.matches.get(next).map(|_| next).or(f.current);
            let delta = with.len() as isize - r.len() as isize;
            for (b, m) in f.matches.iter_mut() {
                if *b == id && m.start >= r.end {
                    *m = (m.start as isize + delta) as usize..(m.end as isize + delta) as usize;
                }
            }
        }
        self.after_replace(window, cx);
    }

    fn replace_all(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(f) = &self.find else { return };
        if f.matches.is_empty() {
            return;
        }
        let matches = f.matches.clone();
        let with = self.replace_field.read(cx).text.clone();
        self.core.checkpoint(EditKind::Other);
        for (id, r) in matches.into_iter().rev() {
            self.replace_range(id, r, &with);
        }
        self.after_replace(window, cx);
    }

    pub(super) fn on_find_event(&mut self, ev: &crate::input::TextFieldEvent, window: &mut Window, cx: &mut Context<Self>) {
        use crate::input::TextFieldEvent as E;
        match ev {
            E::Changed => {
                self.refresh_find(true, cx);
                self.scroll_to_current(window);
                cx.notify();
            }
            E::Submit => self.step_find(true, window, cx),
            E::ShiftSubmit => self.step_find(false, window, cx),
            E::Cancel => self.close_find(window, cx),
            _ => {}
        }
    }

    pub(super) fn find_box(&self, window: &Window, cx: &mut Context<Self>) -> Option<AnyElement> {
        let f = self.find.as_ref()?;
        let n = f.matches.len();
        let counter = match f.current {
            Some(i) if n > 0 => format!("{} / {n}", i + 1),
            _ => "0 / 0".to_string(),
        };
        let replace = f.replace;
        // Tabs: the chosen one semibold in the accent over a 2pt accent bar.
        let tab = |id: &'static str, label: &'static str, active: bool, to_replace: bool| {
            div()
                .id(id)
                .relative()
                .h_full()
                .flex()
                .items_center()
                .cursor_pointer()
                .text_size(px(15.))
                .text_color(if active { Theme::primary() } else { Theme::text_caption() })
                .when(active, |d| d.font_weight(FontWeight::SEMIBOLD))
                .when(!active, |d| d.hover(|s| s.text_color(Theme::text())))
                .child(label)
                .when(active, |d| {
                    d.child(div().absolute().bottom_0().left_0().right_0().h(px(2.)).rounded(px(1.)).bg(Theme::primary()))
                })
                .on_click(cx.listener(move |e, _, window, cx| {
                    if let Some(f) = &mut e.find {
                        f.replace = to_replace;
                    }
                    let field = e.find_field.clone();
                    window.focus(&field.focus_handle(cx));
                    cx.notify();
                }))
        };
        // The input: 30 tall, radius 9, white, hairline; accent while focused.
        let input = |field: &Entity<TextField>, suffix: Option<String>| {
            let focused = field.read(cx).is_focused(window);
            div()
                .h(px(30.))
                .px(px(10.))
                .flex()
                .items_center()
                .gap(px(8.))
                .rounded(px(9.))
                .bg(crate::theme::raised(Theme::bg_body()))
                .border_1()
                .border_color(if focused { Theme::primary() } else { Theme::line_border() })
                .child(field.clone())
                .children(suffix.map(|s| {
                    div().flex_none().text_size(px(12.)).line_height(px(20.)).text_color(Theme::text_caption()).child(s)
                }))
        };
        let label = |s: &'static str| div().text_size(px(13.)).line_height(px(20.)).text_color(Theme::text()).child(s);
        let enabled = n > 0;
        // Primary: the accent, white semibold; secondary: a white
        // button (hairline, 1px drop). Both 30 tall, radius 9.
        let button = |id: &'static str, text: &'static str, primary: bool| {
            let (bg, border, color) = match (primary, enabled) {
                (true, true) => (Theme::primary(), Theme::primary(), gpui::white()),
                (true, false) => (theme::ca(craft::B500, 0.4), gpui::transparent_black(), gpui::white()),
                (false, true) => (Theme::bg_body(), Theme::button_border(), Theme::text()),
                (false, false) => (Theme::bg_body(), Theme::button_border(), Theme::text_disabled()),
            };
            div()
                .id(id)
                .flex_none()
                .w(px(BTN_W))
                .h(px(30.))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(9.))
                .border_1()
                .border_color(border)
                .bg(bg)
                .shadow(theme::shadow_button())
                .text_size(px(13.))
                .when(primary, |d| d.font_weight(FontWeight::SEMIBOLD))
                .text_color(color)
                .when(enabled, |d| {
                    d.cursor_pointer().hover(move |s| {
                        if primary { s.bg(theme::c(craft::B600)).border_color(theme::c(craft::B600)) } else { s.bg(theme::c(craft::N100)) }
                    })
                })
                .when(!enabled, |d| d.cursor(CursorStyle::OperationNotAllowed))
                .child(text)
        };
        let prev = button("find-prev", crate::settings::text("上一处"), false)
            .tip_label("Shift + Enter".into(), TipSide::Above)
            .on_click(cx.listener(|e, _, window, cx| e.step_find(false, window, cx)));
        let next = button("find-next", crate::settings::text("下一处"), false)
            .tip_label("Enter".into(), TipSide::Above)
            .on_click(cx.listener(|e, _, window, cx| e.step_find(true, window, cx)));
        let body = div()
            .px(px(20.))
            .pb(px(20.))
            .flex()
            .flex_col()
            .child(
                div()
                    .mt(px(16.))
                    .flex()
                    .flex_col()
                    .when(replace, |d| d.child(label(crate::settings::text("查找"))))
                    .child(div().when(replace, |d| d.mt(px(6.))).child(input(&self.find_field, Some(counter)))),
            )
            .when(replace, |d| {
                d.child(
                    div()
                        .mt(px(14.))
                        .flex()
                        .flex_col()
                        .child(label(crate::settings::text("替换为")))
                        .child(div().mt(px(6.)).child(input(&self.replace_field, None))),
                )
            })
            .child(
                div()
                    .mt(px(20.))
                    .flex()
                    .justify_end()
                    .gap(px(8.))
                    .when(replace, |d| {
                        d.child(
                            button("find-replace-all", crate::settings::text("全部替换"), true)
                                .on_click(cx.listener(|e, _, window, cx| e.replace_all(window, cx))),
                        )
                        .child(
                            button("find-replace", crate::settings::text("替换"), false)
                                .on_click(cx.listener(|e, _, window, cx| e.replace_one(window, cx))),
                        )
                    })
                    .child(prev)
                    .child(next),
            );
        // The popover: the light menu material, hairline, radius 12, and
        // the deeper drop of a second-level panel.
        let el = popover()
            .id("find-box")
            .occlude()
            .absolute()
            .top(px(8.))
            .right(px(16.))
            .w(px(BOX_W))
            .shadow(theme::shadow_panel())
            .cursor(CursorStyle::Arrow)
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(
                div()
                    .relative()
                    .h(px(44.))
                    .px(px(20.))
                    .flex()
                    .gap(px(24.))
                    .border_b_1()
                    .border_color(Theme::line_divider())
                    .child(tab("find-tab", crate::settings::text("查找"), !replace, false))
                    .child(tab("replace-tab", crate::settings::text("替换"), replace, true))
                    .child(
                        div()
                            .id("find-close")
                            .absolute()
                            .top(px(10.))
                            .right(px(10.))
                            .size(px(24.))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(ROW_RADIUS))
                            .cursor_pointer()
                            .hover(|s| s.bg(Theme::fill_hover_strong()))
                            .active(|s| s.bg(Theme::fill_pressed()))
                            .child(icon("x", 16., Theme::text_caption()))
                            .on_click(cx.listener(|e, _, window, cx| e.close_find(window, cx))),
                    ),
            )
            .child(body);
        Some(el.into_any_element())
    }
}

#[cfg(test)]
mod tests {
    use super::find_in;
    use crate::model::*;

    #[test]
    fn find_ignores_case_and_inline_objects() {
        let mut t = RichText::plain("Hello hello ");
        t.push("\u{fffc}", Marks { inline: Some(Inline { kind: InlineKind::Other { tag: "x".into() }, xml: "<x/>".into() }), ..Default::default() });
        t.push("中文中文", Marks::default());
        assert_eq!(find_in(&t, "HELLO"), vec![0..5, 6..11]);
        assert_eq!(find_in(&t, "中文"), vec![15..21, 21..27]);
        assert!(find_in(&t, "\u{fffc}").is_empty());
        assert_eq!(find_in(&RichText::plain("aaa"), "aa"), vec![0..2]);
    }
}
