use super::view::{Editor, Menu};
use crate::model::BlockId;
use crate::feishu_theme::Theme;
use crate::editor_ui::popover;
use gpui::{prelude::FluentBuilder, *};

impl Editor {
    pub(super) fn open_ordered_number(&mut self, block: BlockId, depth: usize, pos: Point<Pixels>, window: &mut Window, cx: &mut Context<Self>) {
        self.toolbar_hidden = true;
        self.menu = Some(Menu::OrderedNumber { block, pos: self.rel(pos + point(px(0.), px(16.))), edit: false, depth });
        window.focus(&self.focus);
        cx.notify();
    }

    pub(super) fn submit_ordered_number(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(Menu::OrderedNumber { block, edit: true, depth, .. }) = self.menu else { return };
        let Some(n) = parse_number(&self.number_field.read(cx).text, depth) else { return };
        self.core.set_ordered_number(block, n, false);
        self.close_menu_focus(window, cx);
        self.changed(window, cx);
    }

    pub(super) fn ordered_number_menu(&self, block: BlockId, pos: Point<Pixels>, edit: bool, depth: usize, cx: &mut Context<Self>) -> Option<AnyElement> {
        let (number, previous) = self.core.ordered_number_options(block)?;
        let mut panel = popover().id("ordered-number-menu").p(px(8.))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation());
        if edit {
            let valid = parse_number(&self.number_field.read(cx).text, depth).is_some();
            panel = panel.w(px(240.)).flex().flex_col().gap(px(8.))
                .child(div().text_size(px(14.)).child(crate::settings::text("新编号为")))
                .child(div().h(px(32.)).px(px(8.)).flex().items_center().border_1().border_color(Theme::primary()).rounded(px(4.)).child(self.number_field.clone()))
                .child(div().id("ordered-number-confirm").h(px(32.)).flex().items_center().justify_center().rounded(px(4.))
                    .bg(if valid { Theme::primary() } else { Theme::fill_hover() })
                    .text_color(if valid { gpui::white() } else { Theme::text_disabled() }).text_size(px(14.)).child(crate::settings::text("确定"))
                    .when(valid, |d| d.cursor_pointer().on_mouse_down(MouseButton::Left, cx.listener(|e, _, w, cx| {
                        cx.stop_propagation(); e.submit_ordered_number(w, cx);
                    }))));
        } else {
            panel = panel.w(px(184.));
            for (i, label) in [crate::settings::text("继续之前的编号"), crate::settings::text("开始新列表"), crate::settings::text("修改编号值")].into_iter().enumerate() {
                let enabled = match i { 0 => previous.is_some_and(|n| n != number), 1 => number != 1, _ => true };
                panel = panel.child(div().id(("ordered-number-action", i)).h(px(32.)).px(px(12.)).flex().items_center().rounded(px(4.)).text_size(px(14.))
                    .text_color(if enabled { Theme::text() } else { Theme::text_disabled() }).child(label)
                    .when(enabled, |d| d.cursor_pointer().hover(|s| s.bg(Theme::fill_hover())).on_mouse_down(MouseButton::Left, cx.listener(move |e, _, w, cx| {
                        cx.stop_propagation();
                        if i == 2 {
                            e.number_field.update(cx, |f, cx| { f.set_text(super::core::ordered_marker(number, depth).trim_end_matches('.'), cx); f.select_all_text(cx); });
                            e.menu = Some(Menu::OrderedNumber { block, pos, edit: true, depth });
                            w.focus(&e.number_field.read(cx).focus_handle(cx));
                            cx.notify();
                        } else {
                            e.core.set_ordered_number(block, if i == 0 { previous.unwrap_or(1) } else { 1 }, i == 0);
                            e.close_menu_focus(w, cx);
                            e.changed(w, cx);
                        }
                    }))));
            }
        }
        Some(super::overlays::at(pos, Corner::TopLeft, panel))
    }
}

// The sequence input uses the same depth cycle as its marker.
fn parse_number(text: &str, depth: usize) -> Option<u32> {
    let text = text.trim().to_ascii_lowercase();
    if text.is_empty() { return None }
    let n = match depth % 3 {
        0 => { if !text.bytes().all(|c| c.is_ascii_digit()) { return None } text.parse::<u32>().ok()? },
        1 => text.bytes().try_fold(0u32, |n, c| {
            if !c.is_ascii_lowercase() { return None }
            n.checked_mul(26)?.checked_add(u32::from(c - b'a') + 1)
        })?,
        _ => {
            let mut total = 0i64;
            let mut previous = 0i64;
            for c in text.bytes().rev() {
                let value = match c { b'i' => 1, b'v' => 5, b'x' => 10, b'l' => 50, b'c' => 100, b'd' => 500, b'm' => 1000, _ => return None };
                total = total.checked_add(if value < previous { -value } else { value })?;
                previous = value;
            }
            let n = u32::try_from(total).ok()?;
            if super::core::ordered_marker(n, depth).trim_end_matches('.') != text { return None }
            n
        }
    };
    (n > 0).then_some(n)
}
