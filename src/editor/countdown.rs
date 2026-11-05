//! 倒计时 block: four coloured digit boxes
//! (天 时 分 秒) under a translucent "wave" that drains with the time left.
//! Hovering shows 设置 (opens 倒计时设置) and 颜色 (a seven-swatch picker).
//! The dialog sets either a length (输入倒计时时长) or an end
//! (选择具体日期); the block stores the length and its start, as a
//! `{duration, startTime}` record.

use super::render::R;
use super::view::{Editor, Menu};
use crate::input::{TextField, TextFieldEvent};
use crate::model::{Block, BlockId, BlockKind, COUNTDOWN_COLORS, Countdown};
use crate::editor_theme::{self as theme, Theme, craft};
use crate::editor_ui::{ROW_RADIUS, TipExt, ToastKind, icon, popover};
use gpui::{prelude::FluentBuilder, *};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    time::{Duration, Instant},
};

const COLOR_NAMES: [&str; 7] = ["红色", "橙色", "黄色", "绿色", "蓝色", "紫色", "灰色"];
const UNITS: [&str; 4] = ["天", "时", "分", "秒"];
/// Largest value each duration field accepts (typed), and how many values
/// its dropdown lists.
const MAX: [u32; 4] = [999, 23, 59, 59];
const OPTIONS: [u32; 4] = [100, 24, 60, 60];
/// `.timer-item.{d,h,m,s}` and `.timer-separate.{d,h,m}` right offsets.
const ITEM_RIGHT: [f32; 4] = [204., 136., 68., 0.];
const SEP_RIGHT: [f32; 3] = [188., 120., 52.];
/// Dropdown row pitch (26pt list rows, no gaps) and list height.
const ITEM_H: f32 = crate::editor_ui::MENU_ROW_H;
const LIST_H: f32 = 240.;
/// Vertical padding of a dropdown list (6 above, 6 below).
const LIST_PAD: f32 = 2. * crate::editor_ui::MENU_PAD_Y;
/// Blocks in the page stay crisp: the card and its tiles at 5.
const RADIUS: f32 = 5.;
/// The hues of `COUNTDOWN_COLORS`, in order.
const HUES: [&str; 7] = ["red", "orange", "yellow", "green", "blue", "purple", "gray"];

/// A stored colour as drawn: the seven named ones in their hue's swatch
/// of the palette; anything else as written.
fn hex(s: &str) -> Hsla {
    if let Some(c) = COUNTDOWN_COLORS.iter().position(|k| k.eq_ignore_ascii_case(s)).and_then(|i| theme::callout_border_swatch(HUES[i])) {
        return c;
    }
    u32::from_str_radix(s.trim_start_matches('#'), 16).map(theme::c).unwrap_or(theme::c(craft::old::O500))
}

/// Selection and focus: the accent.
fn accent() -> Hsla {
    Theme::primary()
}

/// The accent tint behind a chosen value.
fn accent_tint() -> Hsla {
    Theme::fill_turninto_active()
}

/// A neutral rim a step darker than the hairline (hover on inputs and options).
fn rim_hover() -> Hsla {
    theme::ca(craft::N1000, 0.16)
}

/// (y, m, d) of a day number (days since 1970-01-01).
pub(super) fn from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (yoe + era * 400 + i64::from(m <= 2), m, d)
}

fn parse_date(s: &str) -> Option<(i64, u32, u32)> {
    let mut it = s.split('-').map(|p| p.parse::<i64>().ok());
    let (y, m, d) = (it.next()??, it.next()??, it.next()??);
    ((1..=12).contains(&m) && d >= 1 && d <= crate::time::days_in_month(y, m as u32) as i64).then_some((y, m as u32, d as u32))
}

fn parse_time(s: &str) -> Option<(u32, u32)> {
    let (h, m) = s.split_once(':')?;
    let (h, m) = (h.parse::<u32>().ok()?, m.parse::<u32>().ok()?);
    (h < 24 && m < 60).then_some((h, m))
}

/// Which list hangs under the dialog's pickers.
#[derive(Clone, Copy, PartialEq)]
pub(super) enum Drop {
    Dur(usize),
    Date,
    Time,
}

pub(super) struct Dialog {
    block: BlockId,
    by_date: bool,
    notify: bool,
    date: (i64, u32, u32),
    time: (u32, u32),
    /// The calendar's page.
    month: (i64, u32),
    drop: Option<Drop>,
}

pub(super) struct CdUi {
    hover: Option<BlockId>,
    palette: Option<BlockId>,
    palette_gen: u64,
    /// Pointer over the 颜色 button / over the picker.
    palette_in: [bool; 2],
    pub(super) dialog: Option<Dialog>,
    fields: [Entity<TextField>; 4],
    focus: FocusHandle,
    /// When each countdown was last drawn: the tick only follows those on
    /// screen.
    drawn: RefCell<HashMap<BlockId, Instant>>,
    /// Seconds left at the last tick, to redraw on change and to notice the end.
    last: HashMap<BlockId, u64>,
    bounds: Rc<RefCell<HashMap<&'static str, Bounds<Pixels>>>>,
    /// The duration list, then the time panel's hour and minute columns.
    lists: [ScrollHandle; 3],
    _tick: Task<()>,
}

impl CdUi {
    pub(super) fn new(window: &mut Window, cx: &mut Context<Editor>) -> (Self, Vec<Subscription>) {
        let fields: [Entity<TextField>; 4] = std::array::from_fn(|i| {
            cx.new(|cx| {
                let mut f = TextField::new("", cx);
                f.digits = true;
                f.max_len = Some(if i == 0 { 3 } else { 2 });
                f.size = 13.;
                f.line_height = 20.;
                f
            })
        });
        let subs = fields
            .iter()
            .enumerate()
            .map(|(i, f)| {
                cx.subscribe_in(f, window, move |e: &mut Editor, _, ev: &TextFieldEvent, window, cx| match ev {
                    TextFieldEvent::Submit => {
                        e.cd_normalize(cx);
                        if let Some(d) = &mut e.cd.dialog {
                            d.drop = None;
                        }
                        window.focus(&e.cd.focus);
                        cx.notify();
                    }
                    TextFieldEvent::Cancel => e.cd_escape(window, cx),
                    TextFieldEvent::Changed => {
                        if let Some(d) = &e.cd.dialog
                            && d.drop == Some(Drop::Dur(i))
                            && let Ok(v) = e.cd.fields[i].read(cx).text.parse::<u32>()
                            && v < OPTIONS[i]
                        {
                            scroll_to(&e.cd.lists[0], v, OPTIONS[i]);
                        }
                        cx.notify();
                    }
                    _ => {}
                })
            })
            .collect();
        let tick = cx.spawn(async |this, cx| {
            loop {
                cx.background_executor().timer(Duration::from_millis(250)).await;
                if this.update(cx, |e, cx| e.tick_countdowns(cx)).is_err() {
                    break;
                }
            }
        });
        let ui = CdUi {
            hover: None,
            palette: None,
            palette_gen: 0,
            palette_in: [false; 2],
            dialog: None,
            fields,
            focus: cx.focus_handle(),
            drawn: RefCell::default(),
            last: HashMap::new(),
            bounds: Rc::default(),
            lists: std::array::from_fn(|_| ScrollHandle::new()),
            _tick: tick,
        };
        (ui, subs)
    }
}

/// Scrolls a dropdown column so value `v` (of `n`) is its first row.
fn scroll_to(list: &ScrollHandle, v: u32, n: u32) {
    let max = (n as f32 * ITEM_H + LIST_PAD - LIST_H).max(0.);
    list.set_offset(point(px(0.), px(-(v as f32 * ITEM_H).min(max))));
}

/// Records an element's window bounds under `key` (last frame).
fn track(map: &Rc<RefCell<HashMap<&'static str, Bounds<Pixels>>>>, key: &'static str) -> impl IntoElement {
    let m = map.clone();
    canvas(
        move |b, _, _| {
            m.borrow_mut().insert(key, b);
        },
        |_, _, _, _| {},
    )
    .absolute()
    .inset_0()
}

impl R<'_> {
    pub(super) fn countdown(&self, b: &Block, c: &Countdown, cx: &mut Context<Editor>) -> AnyElement {
        let ed = self.ed;
        let id = b.id;
        let now = crate::time::now_ms();
        if !self.ghost {
            ed.cd.drawn.borrow_mut().insert(id, Instant::now());
        }
        let left = c.remaining(now);
        let parts = [left / 86_400, left / 3600 % 24, left / 60 % 60, left % 60];
        let color = hex(&c.color);
        let progress = if c.duration > 0. { (left as f32 / c.duration as f32).clamp(0., 1.) } else { 0. };
        let menu_on = matches!(&ed.menu, Some(Menu::Block { block, .. }) | Some(Menu::Add { block, .. }) if *block == id);
        let selected = !menu_on && self.selected.contains(&id);

        let mut content = div().relative().w(px(256.)).h(px(74.)).whitespace_nowrap();
        for (i, v) in parts.iter().enumerate() {
            content = content.child(
                div().absolute().top_0().right(px(ITEM_RIGHT[i])).flex().flex_col().items_center().child(
                    div()
                        .min_w(px(52.))
                        .h(px(52.))
                        .px(px(6.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded(px(RADIUS))
                        .bg(color)
                        .text_size(px(26.))
                        .font_weight(FontWeight::BOLD)
                        .text_color(gpui::white())
                        .child(format!("{v:02}")),
                )
                .child(div().mt(px(4.)).text_size(px(13.)).line_height(px(20.)).text_color(color).child(crate::settings::text(UNITS[i]))),
            );
        }
        for r in SEP_RIGHT {
            content = content.child(
                div()
                    .absolute()
                    .top_0()
                    .right(px(r))
                    .w(px(16.))
                    .h(px(54.))
                    .flex()
                    .justify_center()
                    .text_size(px(28.))
                    .line_height(px(46.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(color)
                    .child(":"),
            );
        }
        // `.wave-wrapper`: a white 15% wash over the boxes, its top falling
        // from above the boxes (all the time left) to their foot (none).
        let top = 49. - 60. * progress;
        content = content.child(
            div()
                .absolute()
                .top_0()
                .left_0()
                .right_0()
                .h(px(52.))
                .overflow_hidden()
                .child(div().absolute().left_0().right_0().top(px(top)).bottom_0().bg(theme::rgbaf(255, 255, 255, 0.15))),
        );

        let tools = !self.ghost && !ed.read_only && (ed.cd.hover == Some(id) || ed.cd.palette == Some(id));
        let tool = |key: &'static str, name: &'static str| {
            div()
                .id((key, id.0))
                .size(px(24.))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(RADIUS))
                .cursor_pointer()
                .hover(|s| s.bg(Theme::fill_hover_strong()))
                .child(icon(name, 18., Theme::text_caption()))
        };
        let toolbar = tools.then(|| {
            div()
                .absolute()
                .top_0()
                .right_0()
                .p(px(8.))
                .flex()
                .child(tool("cd-set", "pen-line").on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |e, _, window, cx| {
                        cx.stop_propagation();
                        e.open_countdown(id, window, cx);
                    }),
                ))
                .child(
                    tool("cd-color", "palette")
                        .ml(px(8.))
                        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                        .on_hover(cx.listener(move |e, h: &bool, _, cx| e.cd_palette_hover(id, 0, *h, cx))),
                )
        });
        let picker = (tools && ed.cd.palette == Some(id)).then(|| {
            let mut row = div().mt(px(8.)).flex();
            for (i, hexs) in COUNTDOWN_COLORS.iter().enumerate() {
                let sw = hex(hexs);
                let active = c.color.eq_ignore_ascii_case(hexs);
                row = row.child(
                    // Colour chips: radius 6; the chosen one in an
                    // accent rim, a neutral one under the pointer.
                    div()
                        .id(("cd-swatch", i))
                        .when(i > 0, |d| d.ml(px(4.)))
                        .size(px(22.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .border_2()
                        .rounded(px(6.))
                        .cursor_pointer()
                        .when(active, |d| d.border_color(accent()).bg(crate::theme::raised(gpui::white())))
                        .when(!active, |d| d.border_color(sw).bg(sw).hover(|s| s.border_color(rim_hover()).bg(crate::theme::raised(gpui::white()))))
                        .child(div().size(px(16.)).rounded(px(4.)).bg(sw))
                        .tip_above(COLOR_NAMES[i], None)
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |e, _, window, cx| {
                                cx.stop_propagation();
                                e.set_countdown_color(id, hexs, window, cx);
                            }),
                        ),
                );
            }
            popover()
                .id(("cd-palette", id.0))
                .absolute()
                .top(px(8.))
                .right(px(36.))
                .pt(px(10.))
                .px(px(12.))
                .pb(px(12.))
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .on_hover(cx.listener(move |e, h: &bool, _, cx| e.cd_palette_hover(id, 1, *h, cx)))
                .child(div().text_size(px(12.)).line_height(px(20.)).text_color(Theme::text_caption()).child(crate::settings::text("颜色")))
                .child(row)
        });

        div()
            .id(("countdown", id.0))
            .relative()
            .w_full()
            .pt(px(20.))
            .pb(px(16.))
            .flex()
            .justify_center()
            // A crisp card: 1pt hairline, radius 5.
            .rounded(px(RADIUS))
            .border_1()
            .border_color(if menu_on {
                Theme::code_hover_ring()
            } else if selected {
                Theme::primary()
            } else {
                Theme::button_border()
            })
            .when(menu_on, |d| {
                d.child(div().absolute().top(px(-3.)).left(px(-3.)).right(px(-3.)).bottom(px(-3.)).rounded(px(RADIUS + 3.)).border_2().border_color(Theme::code_hover_ring()))
            })
            .when(!self.ghost, |d| d.on_hover(cx.listener(move |e, h: &bool, _, cx| e.cd_hover(id, *h, cx))))
            .child(content)
            .children(toolbar)
            .children(picker)
            .into_any_element()
    }
}

impl Editor {
    fn cd_hover(&mut self, id: BlockId, hovered: bool, cx: &mut Context<Self>) {
        if hovered {
            self.cd.hover = Some(id);
        } else if self.cd.hover == Some(id) {
            self.cd.hover = None;
        }
        cx.notify();
    }

    /// The palette opens on hover; leaving the button or the picker closes
    /// it a moment later unless the pointer reached the other.
    fn cd_palette_hover(&mut self, id: BlockId, part: usize, hovered: bool, cx: &mut Context<Self>) {
        self.cd.palette_gen += 1;
        self.cd.palette_in[part] = hovered;
        if self.cd.palette_in.contains(&true) {
            self.cd.palette = Some(id);
            return cx.notify();
        }
        let generation = self.cd.palette_gen;
        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(Duration::from_millis(120)).await;
            let _ = this.update(cx, |e, cx| {
                if e.cd.palette_gen == generation && !e.cd.palette_in.contains(&true) {
                    e.cd.palette = None;
                    cx.notify();
                }
            });
        })
        .detach();
    }

    fn set_countdown_color(&mut self, id: BlockId, color: &str, window: &mut Window, cx: &mut Context<Self>) {
        let same = matches!(self.core.block(id).map(|b| &b.kind), Some(BlockKind::Countdown(c)) if c.color == color);
        if same {
            return;
        }
        self.core.checkpoint(super::core::EditKind::Other);
        if let Some(BlockKind::Countdown(c)) = self.core.block_mut(id).map(|b| &mut b.kind) {
            c.color = color.to_string();
        }
        self.changed(window, cx);
    }

    /// Redraws on-screen countdowns when a digit changes, and shows
    /// 倒计时已结束 when one reaches zero.
    fn tick_countdowns(&mut self, cx: &mut Context<Self>) {
        let now = crate::time::now_ms();
        let recent: Vec<BlockId> = {
            let mut drawn = self.cd.drawn.borrow_mut();
            drawn.retain(|_, t| t.elapsed() < Duration::from_secs(5));
            drawn.iter().filter(|(_, t)| t.elapsed() < Duration::from_millis(1500)).map(|(id, _)| *id).collect()
        };
        let (mut dirty, mut ended) = (false, false);
        for id in recent {
            let Some(BlockKind::Countdown(c)) = self.core.block(id).map(|b| &b.kind) else { continue };
            let left = c.remaining(now);
            let prev = self.cd.last.insert(id, left);
            if prev != Some(left) {
                dirty = true;
                ended |= prev.is_some_and(|p| p > 0) && left == 0 && c.notify;
            }
        }
        if ended {
            self.toast_kind(ToastKind::Success, crate::settings::text("倒计时已结束"), cx);
        }
        if dirty {
            cx.notify();
        }
    }

    // ---- 倒计时设置

    pub(super) fn open_countdown(&mut self, id: BlockId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(BlockKind::Countdown(c)) = self.core.block(id).map(|b| b.kind.clone()) else { return };
        let (y, m, d, h, mi) = crate::time::civil(crate::time::now_ms() / 1000);
        let date = parse_date(&c.date).unwrap_or((y, m, d));
        let time = parse_time(&c.time).unwrap_or((h, mi));
        for (i, f) in self.cd.fields.clone().iter().enumerate() {
            f.update(cx, |f, cx| f.set_text(&c.dhms[i].to_string(), cx));
        }
        self.menu = None;
        self.slash = None;
        self.cd.palette = None;
        self.cd.palette_in = [false; 2];
        self.cd.dialog = Some(Dialog { block: id, by_date: c.by_date, notify: c.notify, date, time, month: (date.0, date.1), drop: None });
        self.cd_set_mode(c.by_date, cx);
        window.focus(&self.cd.focus);
        cx.notify();
    }

    fn close_countdown(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.cd.dialog.take().is_some() {
            window.focus(&self.focus);
            cx.notify();
        }
    }

    fn cd_escape(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match &mut self.cd.dialog {
            Some(d) if d.drop.is_some() => {
                d.drop = None;
                cx.notify();
            }
            _ => self.close_countdown(window, cx),
        }
    }

    fn cd_set_mode(&mut self, by_date: bool, cx: &mut Context<Self>) {
        if let Some(d) = &mut self.cd.dialog {
            d.by_date = by_date;
        }
        let color = if by_date { theme::c(craft::N500) } else { Theme::text() };
        for f in self.cd.fields.clone() {
            f.update(cx, |f, cx| {
                f.color = color;
                cx.notify();
            });
        }
        cx.notify();
    }

    /// Empty fields read 0, and values past a field's range clamp to it
    /// (the add-on's input-number on blur).
    fn cd_normalize(&mut self, cx: &mut Context<Self>) {
        for (i, f) in self.cd.fields.clone().iter().enumerate() {
            let t = f.read(cx).text.clone();
            let v = t.parse::<u32>().unwrap_or(0).min(MAX[i]).to_string();
            if v != t {
                f.update(cx, |f, cx| f.set_text(&v, cx));
            }
        }
    }

    fn cd_value(&self, i: usize, cx: &App) -> u32 {
        self.cd.fields[i].read(cx).text.parse::<u32>().unwrap_or(0).min(MAX[i])
    }

    fn confirm_countdown(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.cd_normalize(cx);
        let Some(d) = &self.cd.dialog else { return };
        let now = crate::time::now_ms();
        let dhms: [u32; 4] = std::array::from_fn(|i| self.cd_value(i, cx));
        let (y, m, dd) = d.date;
        let (h, mi) = d.time;
        let duration = if d.by_date {
            let end = crate::time::local_to_unix(y, m, dd, h, mi) * 1000;
            if end < now as i64 {
                self.toast_kind(ToastKind::Error, crate::settings::text("设置的倒计时时间必须晚于当前 GMT+8 的时间"), cx);
                return;
            }
            (end - now as i64) as f64 / 1000. + 1.
        } else {
            f64::from(dhms[0]) * 86_400. + f64::from(dhms[1]) * 3600. + f64::from(dhms[2]) * 60. + f64::from(dhms[3]) + 1.
        };
        let (id, by_date, notify) = (d.block, d.by_date, d.notify);
        self.core.checkpoint(super::core::EditKind::Other);
        if let Some(BlockKind::Countdown(c)) = self.core.block_mut(id).map(|b| &mut b.kind) {
            c.duration = duration;
            c.start = now;
            c.notify = notify;
            c.by_date = by_date;
            c.dhms = dhms;
            c.date = format!("{y:04}-{m:02}-{dd:02}");
            c.time = format!("{h:02}:{mi:02}");
        }
        self.cd.last.remove(&id);
        self.close_countdown(window, cx);
        self.changed(window, cx);
    }

    fn cd_open_drop(&mut self, drop: Drop, cx: &mut Context<Self>) {
        let Some(d) = &mut self.cd.dialog else { return };
        d.drop = Some(drop);
        match drop {
            Drop::Dur(i) => {
                let v = self.cd.fields[i].read(cx).text.parse::<u32>().unwrap_or(0);
                scroll_to(&self.cd.lists[0], v.min(OPTIONS[i] - 1), OPTIONS[i]);
            }
            Drop::Date => d.month = (d.date.0, d.date.1),
            Drop::Time => {
                let (h, m) = d.time;
                scroll_to(&self.cd.lists[1], h, 24);
                scroll_to(&self.cd.lists[2], m, 60);
            }
        }
        cx.notify();
    }

    /// The dialog and its open dropdown (window-level layers).
    pub(super) fn countdown_overlays(&self, window: &mut Window, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let Some(d) = &self.cd.dialog else { return vec![] };
        let vp = window.viewport_size();
        let mut out = vec![self.cd_dialog(d, vp, cx)];
        out.extend(d.drop.and_then(|drop| self.cd_dropdown(d, drop, vp, cx)));
        out
    }

    fn cd_dialog(&self, d: &Dialog, vp: Size<Pixels>, cx: &mut Context<Self>) -> AnyElement {
        let by_date = d.by_date;
        let bounds = &self.cd.bounds;
        // The two choices as option cards: hairline, the accent rim when chosen.
        let option = |key: &'static str, active: bool| {
            div()
                .id(key)
                .p(px(16.))
                .border_1()
                .border_color(if active { accent() } else { Theme::line_border() })
                .rounded(px(10.))
                .when(!active, |d| d.cursor_pointer().hover(|s| s.border_color(rim_hover())))
        };
        let title = |on: bool, label: &'static str| {
            div().flex().items_center().child(crate::transfer::radio(on)).child(div().ml(px(8.)).line_height(px(22.)).child(label))
        };
        let open = d.drop;

        // 输入倒计时时长
        let mut items = div().mt(px(12.)).flex();
        for i in 0..4 {
            let active = open == Some(Drop::Dur(i));
            let key: &'static str = ["cd-d", "cd-h", "cd-m", "cd-s"][i];
            let picker = div()
                .id(key)
                .relative()
                .w(px(50.))
                .h(px(30.))
                .pl(px(12.))
                .flex()
                .items_center()
                .overflow_hidden()
                .border_1()
                .rounded(px(8.))
                .cursor_pointer()
                .when(by_date, |d| d.bg(theme::c(craft::N200)).border_color(Theme::line_border()))
                .when(!by_date, |d| {
                    d.bg(crate::theme::raised(Theme::bg_body())).border_color(if active { accent() } else { Theme::line_border() }).when(!active, |d| d.hover(|s| s.border_color(rim_hover())))
                })
                .child(div().w_full().child(self.cd.fields[i].clone()))
                .child(track(bounds, key))
                .when(by_date, |d| {
                    // `.picker-input-cover`: a disabled field only switches the option.
                    d.child(div().id((key, 1usize)).absolute().inset_0().occlude().on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|e, _, _, cx| e.cd_set_mode(false, cx)),
                    ))
                })
                .when(!by_date, |d| {
                    d.on_mouse_down(MouseButton::Left, cx.listener(move |e, _, _, cx| e.cd_open_drop(Drop::Dur(i), cx)))
                });
            items = items.child(
                div()
                    .flex()
                    .items_center()
                    .when(i > 0, |d| d.ml(px(16.)))
                    .child(picker)
                    .child(div().ml(px(8.)).line_height(px(22.)).child(crate::settings::text(UNITS[i]))),
            );
        }
        let duration = option("cd-opt-dur", !by_date)
            .on_mouse_down(MouseButton::Left, cx.listener(|e, _, _, cx| e.cd_set_mode(false, cx)))
            .child(title(!by_date, crate::settings::text("输入倒计时时长")))
            .child(items);

        // 选择具体日期
        let (y, m, dd) = d.date;
        let (h, mi) = d.time;
        let text = |key: &'static str, s: String, on: bool| {
            div()
                .id(key)
                .relative()
                .h(px(20.))
                .line_height(px(20.))
                .rounded(px(3.))
                .when(!by_date, |d| d.bg(theme::c(craft::N200)))
                .when(by_date && on, |d| d.bg(accent_tint()))
                .when(by_date, |d| d.hover(|s| s.bg(accent_tint())))
                .child(s)
                .child(track(bounds, key))
        };
        let stamp = div()
            .id("cd-stamp")
            .relative()
            .w(px(278.))
            .h(px(30.))
            .pl(px(12.))
            .pr(px(12.))
            .flex()
            .items_center()
            .border_1()
            .rounded(px(8.))
            .cursor_pointer()
            .when(!by_date, |d| d.bg(theme::c(craft::N200)).border_color(Theme::line_border()).text_color(theme::c(craft::N500)))
            .when(by_date, |d| {
                d.bg(crate::theme::raised(Theme::bg_body())).border_color(if open.is_some() { accent() } else { Theme::line_border() }).when(open.is_none(), |d| d.hover(|s| s.border_color(rim_hover())))
            })
            .child(track(bounds, "cd-stamp"))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .child(text("cd-date", format!("{y:04}-{m:02}-{dd:02}"), open == Some(Drop::Date)).when(by_date, |d| {
                        d.on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|e, _, _, cx| {
                                cx.stop_propagation();
                                e.cd_open_drop(Drop::Date, cx);
                            }),
                        )
                    }))
                    .child(div().w(px(10.)))
                    .child(text("cd-time", format!("{h:02}:{mi:02}"), open == Some(Drop::Time)).when(by_date, |d| {
                        d.on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|e, _, _, cx| {
                                cx.stop_propagation();
                                e.cd_open_drop(Drop::Time, cx);
                            }),
                        )
                    })),
            )
            .child(div().ml(px(10.)).child(icon("calendar", 16., theme::c(craft::N500))))
            .when(by_date, |d| d.on_mouse_down(MouseButton::Left, cx.listener(|e, _, _, cx| e.cd_open_drop(Drop::Date, cx))));
        let stamp_opt = option("cd-opt-date", by_date)
            .mt(px(16.))
            .on_mouse_down(MouseButton::Left, cx.listener(|e, _, _, cx| e.cd_set_mode(true, cx)))
            .child(title(by_date, crate::settings::text("选择具体日期")))
            .child(div().mt(px(12.)).flex().child(stamp));

        // Footer
        let notify = d.notify;
        // The check box: rounded, a hairline rim open, accent with a white check on.
        let check = div()
            .size(px(16.))
            .flex_none()
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(4.))
            .when(notify, |d| d.bg(accent()).child(icon("check", 12., gpui::white())))
            .when(!notify, |d| d.border_1().border_color(Theme::todo_box()).bg(crate::theme::raised(gpui::white())));
        // Buttons: 30 tall, radius 9; primary the accent in white
        // semibold, secondary white with a hairline and a 1px drop.
        let button = |key: &'static str, label: &'static str, primary: bool| {
            div()
                .id(key)
                .ml(px(8.))
                .h(px(30.))
                .px(px(14.))
                .flex()
                .items_center()
                .rounded(px(9.))
                .border_1()
                .cursor_pointer()
                .line_height(px(20.))
                .when(primary, |d| {
                    d.bg(accent())
                        .shadow(theme::shadow_button())
                        .border_color(accent())
                        .text_color(gpui::white())
                        .font_weight(FontWeight::SEMIBOLD)
                        .hover(|s| s.bg(theme::c(craft::B600)).border_color(theme::c(craft::B600)))
                        .active(|s| s.bg(theme::c(craft::B700)).border_color(theme::c(craft::B700)))
                })
                .when(!primary, |d| {
                    d.bg(crate::theme::raised(gpui::white())).border_color(Theme::button_border()).shadow(theme::shadow_button()).hover(|s| s.bg(theme::c(craft::N100))).active(|s| s.bg(theme::c(craft::N200)))
                })
                .child(label)
        };
        let footer = div()
            .mt(px(24.))
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .id("cd-notify")
                    .flex()
                    .items_center()
                    .cursor_pointer()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|e, _, _, cx| {
                            if let Some(d) = &mut e.cd.dialog {
                                d.notify = !d.notify;
                            }
                            cx.notify();
                        }),
                    )
                    .child(check)
                    .child(div().ml(px(8.)).line_height(px(22.)).child(crate::settings::text("倒计时结束时显示气泡提醒"))),
            )
            .child(
                div()
                    .flex()
                    .child(button("cd-cancel", crate::settings::text("取消"), false).on_click(cx.listener(|e, _, window, cx| e.close_countdown(window, cx))))
                    .child(button("cd-ok", crate::settings::text("确定"), true).on_click(cx.listener(|e, _, window, cx| e.confirm_countdown(window, cx)))),
            );

        let close = div()
            .id("cd-close")
            .absolute()
            .top(px(22.))
            .right(px(20.))
            .size(px(28.))
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(8.))
            .cursor_pointer()
            .hover(|s| s.bg(Theme::fill_hover_strong()))
            .active(|s| s.bg(Theme::fill_pressed()))
            .child(icon("x", 18., Theme::text_caption()))
            .on_click(cx.listener(|e, _, window, cx| e.close_countdown(window, cx)));
        // The dialog: white, radius 16, the deep soft drop; 13pt body.
        let dialog = div()
            .id("cd-dialog")
            .relative()
            .w(px(600.))
            .rounded(px(16.))
            .bg(crate::theme::raised(Theme::bg_body()))
            .shadow(theme::shadow_dialog())
            .occlude()
            .text_size(px(13.))
            .text_color(Theme::text())
            // A click anywhere in the dialog first closes the open list and
            // settles typed values; the clicked control may reopen one.
            .capture_any_mouse_down(cx.listener(|e, _, _, cx| {
                e.cd_normalize(cx);
                if let Some(d) = &mut e.cd.dialog {
                    d.drop = None;
                }
                cx.notify();
            }))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(div().p(px(24.)).pr(px(56.)).text_size(px(15.)).line_height(px(24.)).font_weight(FontWeight::SEMIBOLD).child(crate::settings::text("倒计时设置")))
            .child(close)
            .child(div().px(px(24.)).pb(px(24.)).child(duration).child(stamp_opt).child(footer));
        let layer = div()
            .id("cd-mask")
            .key_context("CountdownDialog")
            .track_focus(&self.cd.focus)
            .occlude()
            .w(vp.width)
            .h(vp.height)
            .flex()
            .items_center()
            .justify_center()
            .bg(theme::rgbaf(0, 0, 0, 0.55))
            .font_family(theme::UI_FONT)
            .on_key_down(cx.listener(|e, ev: &KeyDownEvent, window, cx| {
                if ev.keystroke.key == "escape" {
                    cx.stop_propagation();
                    e.cd_escape(window, cx);
                }
            }))
            .on_mouse_down(MouseButton::Left, cx.listener(|e, _, window, cx| e.close_countdown(window, cx)))
            .child(dialog);
        deferred(anchored().position_mode(AnchoredPositionMode::Window).position(point(px(0.), px(0.))).child(layer)).with_priority(10).into_any_element()
    }

    fn cd_dropdown(&self, d: &Dialog, drop: Drop, vp: Size<Pixels>, cx: &mut Context<Self>) -> Option<AnyElement> {
        let anchor_key = match drop {
            Drop::Dur(i) => ["cd-d", "cd-h", "cd-m", "cd-s"][i],
            Drop::Date => "cd-stamp",
            Drop::Time => "cd-time",
        };
        let a = *self.cd.bounds.borrow().get(anchor_key)?;
        let (panel, w, h): (AnyElement, f32, f32) = match drop {
            Drop::Dur(i) => {
                let cur = self.cd.fields[i].read(cx).text.parse::<u32>().ok();
                (self.cd_list(0, OPTIONS[i], cur, move |e, v, window, cx| {
                    e.cd.fields[i].update(cx, |f, cx| f.set_text(&v.to_string(), cx));
                    if let Some(d) = &mut e.cd.dialog {
                        d.drop = None;
                    }
                    window.focus(&e.cd.focus);
                    cx.notify();
                }, cx)
                .w(px(64.))
                .into_any_element(), 64., 240.)
            }
            Drop::Time => {
                let (hh, mm) = d.time;
                let col = |e: &Editor, ix: usize, n: u32, cur: u32, cx: &mut Context<Editor>| {
                    e.cd_list(ix, n, Some(cur), move |e, v, _, cx| {
                        if let Some(d) = &mut e.cd.dialog {
                            if ix == 1 {
                                d.time.0 = v;
                            } else {
                                d.time.1 = v;
                            }
                        }
                        cx.notify();
                    }, cx)
                    // A bare column inside the panel's shell (no fill of its
                    // own, so the shell's rounded corners show).
                    .w(px(56.))
                    .bg(gpui::transparent_black())
                    .border_0()
                    .shadow(vec![])
                    .rounded(px(0.))
                };
                let panel = list_box()
                    .flex()
                    .overflow_hidden()
                    .child(col(self, 1, 24, hh, cx))
                    .child(div().w(px(1.)).h(px(240.)).bg(Theme::line_divider()))
                    .child(col(self, 2, 60, mm, cx));
                (panel.into_any_element(), 115., 242.)
            }
            Drop::Date => (self.cd_calendar(d, cx).into_any_element(), 278., 316.),
        };
        let x = f32::from(a.left()).min(f32::from(vp.width) - w - 8.).max(8.);
        let below = f32::from(a.bottom()) + 4.;
        let y = if below + h > f32::from(vp.height) - 8. { f32::from(a.top()) - 4. - h } else { below };
        let layer = div().occlude().on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation()).child(panel);
        Some(deferred(anchored().position_mode(AnchoredPositionMode::Window).position(point(px(x), px(y))).child(layer)).with_priority(11).into_any_element())
    }

    /// One scrolling column of 0…n-1 (two digits), the current value in the accent.
    fn cd_list(
        &self,
        ix: usize,
        n: u32,
        cur: Option<u32>,
        pick: impl Fn(&mut Editor, u32, &mut Window, &mut Context<Editor>) + Clone + 'static,
        cx: &mut Context<Self>,
    ) -> Div {
        let mut col = div().id(("cd-list", ix)).h(px(LIST_H)).py(px(LIST_PAD / 2.)).overflow_y_scroll().track_scroll(&self.cd.lists[ix]);
        for v in 0..n {
            let on = cur == Some(v);
            let pick = pick.clone();
            // List rows: 32 tall, 13pt, grey hover; the current value
            // in the accent (on its tint in the time columns).
            col = col.child(
                div()
                    .id(("cd-item", v as usize))
                    .flex_none()
                    .mx(px(5.))
                    .h(px(ITEM_H))
                    .px(px(8.))
                    .flex()
                    .items_center()
                    .rounded(px(ROW_RADIUS))
                    .line_height(px(20.))
                    .text_size(px(13.))
                    .cursor_pointer()
                    .text_color(if on { accent() } else { Theme::text() })
                    .when(on && ix > 0, |d| d.bg(accent_tint()))
                    .when(!(on && ix > 0), |d| d.hover(|s| s.bg(Theme::fill_hover())))
                    .child(if ix == 0 { v.to_string() } else { format!("{v:02}") })
                    .on_mouse_down(MouseButton::Left, cx.listener(move |e, _, window, cx| pick(e, v, window, cx))),
            );
        }
        list_box().child(col)
    }

    fn cd_calendar(&self, d: &Dialog, cx: &mut Context<Self>) -> impl IntoElement {
        let (vy, vm) = d.month;
        let (ty, tm, td, _, _) = crate::time::civil(crate::time::now_ms() / 1000);
        let first = crate::time::days_from_civil(vy, vm, 1);
        let start = first - i64::from(crate::time::weekday(vy, vm, 1));
        let nav = |key: &'static str, name: &'static str, delta: i32| {
            div()
                .id(key)
                .size(px(24.))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(ROW_RADIUS))
                .cursor_pointer()
                .hover(|s| s.bg(Theme::fill_hover_strong()))
                .child(icon(name, 16., Theme::text_caption()))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |e, _, _, cx| {
                        if let Some(d) = &mut e.cd.dialog {
                            let k = d.month.0 * 12 + i64::from(d.month.1) - 1 + i64::from(delta);
                            d.month = (k.div_euclid(12), k.rem_euclid(12) as u32 + 1);
                        }
                        cx.notify();
                    }),
                )
        };
        let header = div()
            .pt(px(15.))
            .pr(px(15.))
            .pb(px(10.))
            .pl(px(23.))
            .flex()
            .items_center()
            .justify_between()
            .child(div().text_size(px(13.)).line_height(px(22.)).font_weight(FontWeight::SEMIBOLD).child(crate::localized_format!("{vy}年 {vm}月", "{vy}-{vm:02}")))
            .child(div().flex().gap(px(4.)).child(nav("cd-prev", "chevron-left", -1)).child(nav("cd-next", "chevron-right", 1)));
        let mut weeks = div().mb(px(4.)).flex().justify_between();
        for w in [crate::settings::text("一"), crate::settings::text("二"), crate::settings::text("三"), crate::settings::text("四"), crate::settings::text("五"), crate::settings::text("六"), crate::settings::text("日")] {
            weeks = weeks.child(div().w(px(32.)).h(px(32.)).flex().items_center().justify_center().text_size(px(12.)).text_color(Theme::text_placeholder()).child(w));
        }
        let mut body = div().flex().flex_col().gap(px(4.)).child(weeks);
        for r in 0..6 {
            let mut row = div().flex().justify_between();
            for c in 0..7 {
                let day = start + r * 7 + c;
                let (y, m, dd) = from_days(day);
                let sel = (y, m, dd) == d.date;
                let today = (y, m, dd) == (ty, tm, td);
                let inside = (y, m) == (vy, vm);
                // Day cells: radius 8; the chosen day on the accent, today in
                // accent semibold, a grey hover.
                row = row.child(
                    div()
                        .id(("cd-day", day as usize))
                        .size(px(32.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded(px(8.))
                        .text_size(px(13.))
                        .cursor_pointer()
                        .text_color(if sel {
                            gpui::white()
                        } else if today {
                            accent()
                        } else if inside {
                            Theme::text()
                        } else {
                            Theme::text_placeholder()
                        })
                        .when(sel, |d| d.bg(accent()).font_weight(FontWeight::SEMIBOLD))
                        .when(today && !sel, |d| d.font_weight(FontWeight::SEMIBOLD))
                        .when(!sel, |d| d.hover(|s| s.bg(Theme::fill_hover())))
                        .child(dd.to_string())
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |e, _, _, cx| {
                                if let Some(d) = &mut e.cd.dialog {
                                    d.date = (y, m, dd);
                                    d.drop = None;
                                }
                                cx.notify();
                            }),
                        ),
                );
            }
            body = body.child(row);
        }
        list_box().w(px(278.)).child(header).child(div().px(px(15.)).pb(px(16.)).child(body))
    }
}

/// The dropdown shell (`ud__select-dropdown`): the popover — the menu
/// material, hairline, radius 12, the menu drop.
pub(super) fn list_box() -> Div {
    popover()
}
