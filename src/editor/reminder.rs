//! 日期提醒 (`<time>`): an inline chip with the due date in its status
//! colour (red once past due, orange within a week, blue later). Hovering
//! shows the full date; a click opens the editor: a calendar, 设置时间 and
//! 提醒 (how early). Due reminders post a system notification (see
//! `Workspace::check_reminders`). Times are unix ms, days local.

use super::core::Pos;
use super::overlays::{Probe, at};
use super::view::{Editor, Menu};
use crate::model::InlineReminder;
use crate::input::{TextField, TextFieldEvent};
use crate::editor_theme::{Theme, c, craft};
use crate::editor_ui::{ROW_RADIUS, icon, panel, popover};
use gpui::{prelude::FluentBuilder, *};
use std::{cell::Cell, rc::Rc, time::Duration};

pub const DAY: i64 = 86_400_000;
const HOUR: i64 = 3_600_000;
const MINUTE: i64 = 60_000;
/// The 设置时间 list's step.
const SLOT: i64 = 30 * MINUTE;
/// A whole-day reminder is due at 23:59:59 of its day.
const DAY_END: i64 = DAY - 1000;
const WEEK: [&str; 7] = ["一", "二", "三", "四", "五", "六", "日"];

/// 提醒 choices of a timed reminder: notify − expire.
const TIMED: [(&str, i64); 9] = [
    ("事件发生时", 0),
    ("提前 5 分钟", -5 * MINUTE),
    ("提前 15 分钟", -15 * MINUTE),
    ("提前 30 分钟", -30 * MINUTE),
    ("提前 1 小时", -HOUR),
    ("提前 2 小时", -2 * HOUR),
    ("提前 1 天", -DAY),
    ("提前 2 天", -2 * DAY),
    ("提前 1 周", -7 * DAY),
];
/// Of a whole-day one (due 23:59:59): 09:00 of the day or days before.
const NINE: i64 = 9 * HOUR - DAY_END;
const WHOLE: [(&str, i64); 4] = [("当天 (09:00)", NINE), ("提前 1 天 (09:00)", NINE - DAY), ("提前 2 天 (09:00)", NINE - 2 * DAY), ("提前 1 周 (09:00)", NINE - 7 * DAY)];
/// Listed only while chosen (older documents).
const EVENING: (&str, i64) = ("当天 (18:00)", 18 * HOUR - DAY_END);

const POPOVER_W: f32 = 314.;
const CARD_W: f32 = 280.;
const SELECT_W: f32 = 174.;
/// Row pitch (26pt list rows, no gaps), the list's 5 + 5 padding, its height.
const ITEM_H: f32 = crate::editor_ui::MENU_ROW_H;
const LIST_PAD: f32 = 2. * crate::editor_ui::MENU_PAD_Y;
const LIST_H: f32 = 240.;
/// The selects: 30pt inputs.
const SELECT_H: f32 = 30.;
const SHOW_DELAY: Duration = Duration::from_millis(600);

fn off() -> i64 {
    crate::time::local_offset() * 1000
}

/// Local day number of `t`.
pub fn day_of(t: i64) -> i64 {
    (t + off()).div_euclid(DAY)
}

/// Local midnight of the day of `t`.
pub fn day_start(t: i64) -> i64 {
    day_of(t) * DAY - off()
}

fn ymd(t: i64) -> (i64, u32, u32) {
    super::countdown::from_days(day_of(t))
}

fn hm(t: i64) -> (i64, i64) {
    let tod = t - day_start(t);
    (tod / HOUR, tod % HOUR / MINUTE)
}

/// The local week number (weeks start on Monday).
fn weeks(t: i64) -> i64 {
    (day_of(t) + 3).div_euclid(7)
}

/// The chip text (`getTimeString`): 今天 / 明天 / 昨天 / 本周X / 下周X,
/// then M月D日, then Y年M月D日 in another year; " HH:MM" when timed.
pub fn time_string(expire: i64, timed: bool, now: i64) -> String {
    time_string_for(expire, timed, now, crate::settings::english())
}

fn time_string_for(expire: i64, timed: bool, now: i64, english: bool) -> String {
    let (ey, em, ed) = ymd(expire);
    let (ny, nm, nd) = ymd(now);
    let (e, n) = (i64::from(ed), i64::from(nd));
    let mut s = if ey != ny {
        if english { format!("{ey}-{em:02}-{ed:02}") } else { format!("{ey}年{em}月{ed}日") }
    } else {
        let dw = weeks(expire) - weeks(now);
        if dw >= 2 || dw <= -1 {
            if n - e == 1 && dw == -1 && em == nm { crate::settings::localize("昨天", english).into() } else { if english { format!("{em:02}-{ed:02}") } else { format!("{em}月{ed}日") } }
        } else if e == n {
            crate::settings::localize("今天", english).into()
        } else if n - e == 1 {
            crate::settings::localize("昨天", english).into()
        } else if e - n == 1 {
            crate::settings::localize("明天", english).into()
        } else {
            let w = crate::settings::localize(WEEK[crate::time::weekday(ey, em, ed) as usize], english);
            if dw >= 1 { if english { format!("Next {w}") } else { format!("下周{w}") } } else { if english { format!("This {w}") } else { format!("本周{w}") } }
        }
    };
    if timed {
        let (h, m) = hm(expire);
        s.push_str(&format!(" {h:02}:{m:02}"));
    }
    s
}

pub fn chip_text(r: &InlineReminder, now: i64) -> String {
    time_string(r.expire, !r.whole_day, now)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Expired,
    /// Due within a week.
    Soon,
    Later,
}

pub fn status(expire: i64, now: i64) -> Status {
    if now > expire {
        Status::Expired
    } else if expire > now && expire < now + 7 * DAY {
        Status::Soon
    } else {
        Status::Later
    }
}

/// Text and clock colours of a reminder due at `expire`.
pub fn colors(expire: i64, now: i64) -> (Hsla, Hsla) {
    match status(expire, now) {
        Status::Expired => (c(craft::R400), c(craft::R400)),
        Status::Soon => (c(craft::O400), c(craft::old::O500)),
        Status::Later => (c(craft::B600), Theme::primary()),
    }
}

/// "HH:MM" of a time of day.
pub fn slot_label(t: i64) -> String {
    format!("{:02}:{:02}", t / HOUR, t % HOUR / MINUTE)
}

/// 00:00, 00:30 … 23:30, with an off-grid `set_time` in its place.
pub fn time_slots(set_time: i64) -> Vec<i64> {
    let mut v: Vec<i64> = (0..48).map(|i| i * SLOT).collect();
    if !v.contains(&set_time) {
        let i = v.iter().position(|s| *s > set_time).unwrap_or(v.len());
        v.insert(i, set_time);
    }
    v
}

/// The slot a reminder made timed starts at: the last one not after now
/// (00:00 from 23:30 on).
pub fn current_slot(now: i64) -> i64 {
    let k = (now - day_start(now)) / SLOT;
    if k >= 47 { 0 } else { k * SLOT }
}

/// A typed time ("9:5", "14：30", "3:00 pm") as ms of the day.
pub fn parse_time(s: &str) -> Option<i64> {
    let s = s.replace('：', ":");
    let pm = s.to_lowercase().contains("pm");
    let t: String = s.chars().filter(|c| c.is_ascii_digit() || *c == ':').collect();
    let (h, m) = t.split_once(':')?;
    let ok = |p: &str, lead: u8| !p.is_empty() && p.len() <= 2 && p.bytes().all(|b| b.is_ascii_digit()) && (p.len() == 1 || p.as_bytes()[0] <= lead);
    if !ok(h, b'2') || !ok(m, b'5') {
        return None;
    }
    let (mut h, m): (i64, i64) = (h.parse().ok()?, m.parse().ok()?);
    if h > 23 {
        return None;
    }
    if pm && h < 12 {
        h += 12;
    }
    Some(h * HOUR + m * MINUTE)
}

/// The 提醒 choices; `cur` is the reminder's notify − expire.
pub fn jet_lags(whole_day: bool, cur: i64) -> Vec<(&'static str, i64)> {
    if !whole_day {
        return TIMED.iter().map(|&(label, lag)| (crate::settings::text(label), lag)).collect();
    }
    let mut v = vec![];
    if cur == EVENING.1 {
        v.push((crate::settings::text(EVENING.0), EVENING.1));
    }
    v.extend(WHOLE.iter().map(|&(label, lag)| (crate::settings::text(label), lag)));
    v
}

fn jet_label(whole_day: bool, cur: i64) -> String {
    match jet_lags(whole_day, cur).into_iter().find(|(_, v)| *v == cur) {
        Some((l, _)) => l.into(),
        None => crate::localized_format!("提前 {} 分钟", "{} minutes before", -cur / MINUTE),
    }
}

/// A new reminder: tomorrow, at this half hour.
pub fn default_reminder(now: i64) -> InlineReminder {
    let tod = now - day_start(now);
    let expire = day_start(now) + DAY + tod / SLOT * SLOT;
    InlineReminder { expire, notify: expire, should_notify: true, whole_day: false }
}

/// 设置时间 / 提醒 toggled (`handleNotifyChange`): a reminder made timed
/// takes the current slot, one made whole-day ends its day, and the
/// 提醒 choice resets to the first.
pub fn toggle(r: &InlineReminder, whole_day: bool, should_notify: bool, now: i64) -> InlineReminder {
    let jet = r.notify - r.expire;
    let day = day_start(r.expire);
    let changed_day = whole_day != r.whole_day;
    let tod = match (changed_day, whole_day) {
        (true, false) => current_slot(now),
        (true, true) => DAY_END,
        _ => r.expire - day,
    };
    let expire = day + tod;
    let jet = if changed_day || should_notify != r.should_notify { jet_lags(whole_day, jet)[0].1 } else { jet };
    InlineReminder { expire, notify: expire + jet, should_notify, whole_day }
}

/// Moves the reminder to local day `day`, keeping its time and 提醒.
pub fn on_day(r: &InlineReminder, day: i64) -> InlineReminder {
    let expire = day * DAY - off() + (r.expire - day_start(r.expire));
    InlineReminder { expire, notify: expire + (r.notify - r.expire), ..*r }
}

pub fn at_time(r: &InlineReminder, tod: i64) -> InlineReminder {
    let expire = day_start(r.expire) + tod;
    InlineReminder { expire, notify: expire + (r.notify - r.expire), ..*r }
}

pub fn with_jet(r: &InlineReminder, jet: i64) -> InlineReminder {
    InlineReminder { notify: r.expire + jet, ..*r }
}

pub(crate) fn reminder_label(r: &InlineReminder, sep: &str) -> String {
    if !crate::settings::english() { return r.label(sep); }
    let (y, m, d, h, mi) = crate::time::civil(r.expire.max(0) as u64 / 1000);
    if r.whole_day { format!("{y}-{m:02}-{d:02}") } else { format!("{y}-{m:02}-{d:02}{sep}{h:02}:{mi:02}") }
}

/// The system notification's body.
pub fn notification(r: &InlineReminder) -> String {
    crate::localized_format!("日期提醒：{}", "Reminder: {}", reminder_label(r, " "))
}

// ---- UI

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Drop {
    Time,
    Jet,
}

pub(super) struct Panel {
    at: Pos,
    /// The calendar's page.
    month: (i64, u32),
    drop: Option<Drop>,
}

pub(super) struct RmUi {
    panel: Option<Panel>,
    /// 设置时间's search input.
    time: Entity<TextField>,
    /// The two selects (their lists hang under them).
    selects: [Rc<Cell<Option<Bounds<Pixels>>>>; 2],
    list: ScrollHandle,
    /// Redraws the chips as days and statuses change.
    _tick: Task<()>,
}

impl RmUi {
    pub(super) fn new(window: &mut Window, cx: &mut Context<Editor>) -> (Self, Vec<Subscription>) {
        let time = cx.new(|cx| {
            let mut f = TextField::new("", cx);
            f.size = 13.;
            f.line_height = 20.;
            f
        });
        let subs = vec![cx.subscribe_in(&time, window, |e: &mut Editor, _, ev: &TextFieldEvent, window, cx| e.on_reminder_time(ev, window, cx))];
        let tick = cx.spawn(async |this, cx| {
            loop {
                cx.background_executor().timer(Duration::from_secs(30)).await;
                if this.update(cx, |_, cx| cx.notify()).is_err() {
                    break;
                }
            }
        });
        (RmUi { panel: None, time, selects: Default::default(), list: ScrollHandle::new(), _tick: tick }, subs)
    }
}

pub(super) fn show_delay() -> Duration {
    SHOW_DELAY
}

fn now() -> i64 {
    crate::time::now_ms() as i64
}

/// Records an element's window bounds (last frame).
fn track(cell: &Rc<Cell<Option<Bounds<Pixels>>>>) -> impl IntoElement {
    let cell = cell.clone();
    canvas(move |b, _, _| cell.set(Some(b)), |_, _, _, _| {}).absolute().inset_0()
}

/// The check box: rounded, a hairline rim open, accent with a white check on.
fn checkbox(on: bool) -> Div {
    div()
        .size(px(16.))
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(4.))
        .when(on, |d| d.bg(Theme::primary()).child(icon("check", 12., gpui::white())))
        .when(!on, |d| d.border_1().border_color(Theme::todo_box()).bg(crate::theme::raised(gpui::white())))
}

/// `.reminder-popover-wrapper` box: the second-level panel (white,
/// hairline, radius 12, the panel drop).
fn shell() -> Div {
    panel()
}

impl Editor {
    pub(super) fn reminder_open(&self) -> bool {
        self.rm.panel.is_some() && matches!(self.menu, Some(Menu::Reminder { .. }))
    }

    /// A click on a chip: the reminder editor.
    pub(super) fn open_reminder(&mut self, p: Pos, cx: &mut Context<Self>) {
        let Some(r) = self.core.reminder_at(p) else { return };
        if self.read_only {
            return self.toast(crate::settings::text("当前为阅读模式，请切换为编辑模式后重试"), cx);
        }
        self.hide_button_card(cx);
        self.hide_link_card(cx);
        let (y, m, _) = ymd(r.expire);
        self.rm.panel = Some(Panel { at: p, month: (y, m), drop: None });
        self.sync_reminder_time(&r, cx);
        self.core.clear_block_sel();
        self.slash = None;
        self.dragging = false;
        self.menu = Some(Menu::Reminder { block: p.block, offset: p.offset });
        cx.notify();
    }

    pub(super) fn insert_new_reminder(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(p) = self.core.insert_reminder(&default_reminder(now())) {
            self.changed(window, cx);
            self.open_reminder(p, cx);
        }
    }

    fn sync_reminder_time(&mut self, r: &InlineReminder, cx: &mut Context<Self>) {
        let t = slot_label(r.expire - day_start(r.expire));
        self.rm.time.update(cx, |f, cx| f.set_text(&t, cx));
    }

    fn close_reminder(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.rm.panel = None;
        if matches!(self.menu, Some(Menu::Reminder { .. })) {
            self.menu = None;
        }
        window.focus(&self.focus);
        cx.notify();
    }

    fn apply_reminder(&mut self, f: impl FnOnce(&InlineReminder) -> InlineReminder, window: &mut Window, cx: &mut Context<Self>) {
        let Some(at_) = self.rm.panel.as_ref().map(|p| p.at) else { return };
        let Some(old) = self.core.reminder_at(at_) else {
            self.toast(crate::settings::text("该提醒已被删除。"), cx);
            return self.close_reminder(window, cx);
        };
        let r = f(&old);
        if r != old {
            self.core.set_reminder(at_, &r);
            self.changed(window, cx);
        }
        self.sync_reminder_time(&r, cx);
        cx.notify();
    }

    /// Takes a typed time (on Enter, or when focus leaves the field);
    /// anything else puts the current one back.
    fn commit_reminder_time(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(at_) = self.rm.panel.as_ref().map(|p| p.at) else { return };
        let Some(r) = self.core.reminder_at(at_) else { return };
        let text = self.rm.time.read(cx).text.clone();
        match parse_time(&text) {
            Some(t) if t != r.expire - day_start(r.expire) => self.apply_reminder(|r| at_time(r, t), window, cx),
            _ => self.sync_reminder_time(&r, cx),
        }
    }

    /// A press outside the popover: it closes, keeping a typed time.
    pub(super) fn reminder_blur(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.reminder_open() {
            self.commit_reminder_time(window, cx);
        }
        self.rm.panel = None;
    }

    fn on_reminder_time(&mut self, ev: &TextFieldEvent, window: &mut Window, cx: &mut Context<Self>) {
        match ev {
            TextFieldEvent::Submit => {
                self.commit_reminder_time(window, cx);
                if let Some(p) = &mut self.rm.panel {
                    p.drop = None;
                }
                window.focus(&self.focus);
            }
            TextFieldEvent::Cancel => {
                if self.rm.panel.as_ref().is_some_and(|p| p.drop.is_some()) {
                    if let Some(p) = &mut self.rm.panel {
                        p.drop = None;
                    }
                    if let Some(r) = self.rm.panel.as_ref().and_then(|p| self.core.reminder_at(p.at)) {
                        self.sync_reminder_time(&r, cx);
                    }
                } else {
                    self.close_reminder(window, cx);
                }
            }
            TextFieldEvent::Changed => {
                if let Some(p) = &mut self.rm.panel
                    && p.drop != Some(Drop::Time)
                {
                    p.drop = Some(Drop::Time);
                }
            }
            _ => {}
        }
        cx.notify();
    }

    fn reminder_drop(&mut self, d: Drop, window: &mut Window, cx: &mut Context<Self>) {
        let Some(p) = &mut self.rm.panel else { return };
        // The time select is a search field: pressing it again only moves the caret.
        if p.drop == Some(d) {
            if d == Drop::Jet {
                p.drop = None;
            }
            return cx.notify();
        }
        p.drop = Some(d);
        let at_ = p.at;
        if d == Drop::Time {
            let fh = self.rm.time.read(cx).focus_handle(cx);
            window.focus(&fh);
            self.rm.time.update(cx, |f, cx| f.select_all_text(cx));
            if let Some(r) = self.core.reminder_at(at_) {
                let t = r.expire - day_start(r.expire);
                let ix = time_slots(t).iter().position(|s| *s == t).unwrap_or(0);
                let n = time_slots(t).len();
                let max = (n as f32 * ITEM_H + LIST_PAD - LIST_H).max(0.);
                let y = (ix as f32 * ITEM_H - LIST_H / 2. + ITEM_H).clamp(0., max);
                self.rm.list.set_offset(point(px(0.), px(-y)));
            }
        } else {
            window.focus(&self.focus);
        }
        cx.notify();
    }

    /// The hover card: the full date (`LL`, `，HH:mm`), a small popover.
    pub(super) fn reminder_card(&self, at_: Pos, r: &InlineReminder, window: &mut Window, cx: &mut Context<Self>) -> Option<AnyElement> {
        let rect = self.probed(Probe::Button(at_))?;
        let vh = f32::from(window.viewport_size().height);
        // Padding 5 + the 40 card and its border + 10.
        let up = f32::from(rect.bottom()) + 5. + 42. + 10. > vh;
        let el = div()
            .id("rm-card")
            .occlude()
            .pt(px(5.))
            .pb(px(10.))
            .on_hover(cx.listener(|e, h: &bool, _, cx| {
                e.btn.card_over = *h;
                e.sync_button_card(cx);
            }))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(
                popover().w(px(CARD_W)).child(
                    div().mx(px(16.)).h(px(40.)).flex().items_center().justify_center().text_size(px(13.)).line_height(px(20.)).child(reminder_label(r, crate::settings::tr("，", ", "))),
                ),
            );
        let x = rect.center().x - px(CARD_W / 2.);
        Some(if up { at(self.rel(point(x, rect.top())), Corner::BottomLeft, el) } else { at(self.rel(point(x, rect.bottom())), Corner::TopLeft, el) })
    }

    /// The editor (`intact` popover): calendar, 设置时间, 提醒.
    pub(super) fn reminder_popover(&self, block: crate::model::BlockId, offset: usize, window: &mut Window, cx: &mut Context<Self>) -> Option<AnyElement> {
        let p = self.rm.panel.as_ref().filter(|p| p.at == Pos::new(block, offset))?;
        let r = self.core.reminder_at(p.at)?;
        let rect = self.probed(Probe::Button(p.at))?;
        let now = now();

        // Calendar, Sunday first.
        let (vy, vm) = p.month;
        let first = crate::time::days_from_civil(vy, vm, 1);
        let start = first - i64::from((crate::time::weekday(vy, vm, 1) + 1) % 7);
        let today = day_of(now);
        let picked = day_of(r.expire);
        let nav = |key: &'static str, name: &'static str, delta: i64| {
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
                        if let Some(p) = &mut e.rm.panel {
                            let k = p.month.0 * 12 + i64::from(p.month.1) - 1 + delta;
                            p.month = (k.div_euclid(12), k.rem_euclid(12) as u32 + 1);
                        }
                        cx.notify();
                    }),
                )
        };
        let header = div()
            .pl(px(8.))
            .pb(px(10.))
            .flex()
            .items_center()
            .justify_between()
            .child(div().text_size(px(13.)).line_height(px(22.)).font_weight(FontWeight::SEMIBOLD).child(crate::localized_format!("{vy}年 {vm}月", "{vy}-{vm:02}")))
            .child(div().flex().gap(px(4.)).child(nav("rm-prev", "chevron-left", -1)).child(nav("rm-next", "chevron-right", 1)));
        let mut weeks_row = div().mb(px(4.)).flex().justify_between();
        for w in [crate::settings::text("日"), crate::settings::text("一"), crate::settings::text("二"), crate::settings::text("三"), crate::settings::text("四"), crate::settings::text("五"), crate::settings::text("六")] {
            weeks_row = weeks_row.child(div().size(px(32.)).flex().items_center().justify_center().text_size(px(12.)).text_color(Theme::text_placeholder()).child(w));
        }
        let mut body = div().h(px(248.)).flex().flex_col().justify_between().child(weeks_row);
        for row in 0..6 {
            let mut el = div().flex().justify_between();
            for col in 0..7 {
                let day = start + row * 7 + col;
                let (y, m, d) = super::countdown::from_days(day);
                let sel = day == picked;
                let is_today = day == today;
                // Day cells: radius 8; the chosen day on the accent, today in
                // accent semibold, a grey hover.
                el = el.child(
                    div()
                        .id(("rm-day", day as usize))
                        .size(px(32.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded(px(8.))
                        .text_size(px(13.))
                        .cursor_pointer()
                        .text_color(if sel {
                            gpui::white()
                        } else if is_today {
                            Theme::primary()
                        } else if (y, m) == (vy, vm) {
                            Theme::text()
                        } else {
                            Theme::text_placeholder()
                        })
                        .when(sel, |s| s.bg(Theme::primary()).font_weight(FontWeight::SEMIBOLD))
                        .when(is_today && !sel, |s| s.font_weight(FontWeight::SEMIBOLD))
                        .when(!sel, |s| s.hover(|s| s.bg(Theme::fill_hover())))
                        .child(d.to_string())
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |e, _, window, cx| {
                                if let Some(p) = &mut e.rm.panel {
                                    p.month = (y, m);
                                    p.drop = None;
                                }
                                e.apply_reminder(|r| on_day(r, day), window, cx);
                            }),
                        ),
                );
            }
            body = body.child(el);
        }
        let calendar = div().mt(px(12.)).px(px(10.)).child(div().p(px(16.)).child(header).child(body));

        // 设置时间 / 提醒
        let timed = !r.whole_day;
        let check_row = |key: &'static str, on: bool, label: &'static str| {
            div().id(key).h(px(22.)).mb(px(8.)).flex().items_center().cursor_pointer().child(checkbox(on).mr(px(8.))).child(div().line_height(px(22.)).child(label))
        };
        let open = p.drop;
        // The input: 30 tall, radius 9, white, hairline; the accent while open.
        let select = |ix: usize, active: bool| {
            div()
                .relative()
                .w(px(SELECT_W))
                .h(px(SELECT_H))
                .pl(px(10.))
                .pr(px(8.))
                .flex()
                .items_center()
                .rounded(px(9.))
                .border_1()
                .border_color(if active { Theme::primary() } else { Theme::line_border() })
                .bg(crate::theme::raised(Theme::bg_body()))
                .cursor_pointer()
                .when(!active, |d| d.hover(|s| s.border_color(crate::editor_theme::ca(craft::N1000, 0.16))))
                .child(track(&self.rm.selects[ix]))
        };
        let chevron = || div().ml(px(8.)).flex_none().child(icon("chevron-down", 12., Theme::text_caption()));
        let time_focused = self.rm.time.read(cx).is_focused(window);
        let time_select = select(0, open == Some(Drop::Time) || time_focused)
            .id("rm-time")
            .child(div().flex_1().min_w_0().child(self.rm.time.clone()))
            .child(chevron())
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|e, _, window, cx| {
                    cx.stop_propagation();
                    e.reminder_drop(Drop::Time, window, cx);
                }),
            );
        let jet = r.notify - r.expire;
        let jet_select = select(1, open == Some(Drop::Jet))
            .id("rm-jet")
            .child(div().flex_1().min_w_0().overflow_hidden().text_size(px(13.)).line_height(px(20.)).child(jet_label(r.whole_day, jet)))
            .child(chevron())
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|e, _, window, cx| {
                    cx.stop_propagation();
                    e.reminder_drop(Drop::Jet, window, cx);
                }),
            );
        let (whole, notify) = (r.whole_day, r.should_notify);
        let options = div()
            .border_t_1()
            .border_color(Theme::line_divider())
            .px(px(24.))
            .pb(px(24.))
            .text_size(px(13.))
            .child(
                div()
                    .mt(px(20.))
                    .child(check_row("rm-set-time", timed, crate::settings::text("设置时间")).on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |e, _, window, cx| {
                            e.apply_reminder(|r| toggle(r, !whole, notify, now), window, cx);
                        }),
                    ))
                    .when(timed, |d| d.child(time_select)),
            )
            .child(
                div()
                    .mt(px(16.))
                    .child(check_row("rm-notify", notify, crate::settings::text("提醒")).on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |e, _, window, cx| {
                            e.apply_reminder(|r| toggle(r, whole, !notify, now), window, cx);
                        }),
                    ))
                    .when(notify, |d| d.child(jet_select)),
            );

        let el = div()
            .id("rm-popover")
            .key_context("ReminderPopover")
            .occlude()
            .pt(px(5.))
            .pb(px(10.))
            // A press in the popover settles a typed time and closes the
            // open list; the pressed control may reopen one.
            .capture_any_mouse_down(cx.listener(|e, ev: &MouseDownEvent, window, cx| {
                let hit = |i: usize| e.rm.selects[i].get().is_some_and(|b| b.contains(&ev.position));
                let (in_time, in_jet) = (hit(0), hit(1));
                if !in_time {
                    e.commit_reminder_time(window, cx);
                }
                if let Some(p) = &mut e.rm.panel {
                    let keep = match p.drop {
                        Some(Drop::Time) => in_time,
                        Some(Drop::Jet) => in_jet,
                        None => false,
                    };
                    if !keep {
                        p.drop = None;
                    }
                }
            }))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_key_down(cx.listener(|e, ev: &KeyDownEvent, window, cx| {
                if ev.keystroke.key == "escape" {
                    cx.stop_propagation();
                    e.close_reminder(window, cx);
                }
            }))
            .child(shell().w(px(POPOVER_W)).child(calendar).child(options));
        let h = 5. + 12. + 32. + 32. + 248. + 1. + 20. + 30. + 16. + 30. + 24. + 10. + if timed { SELECT_H } else { 0. } + if notify { SELECT_H } else { 0. };
        let vh = f32::from(window.viewport_size().height);
        let view_top = f32::from(self.list.viewport_bounds().top());
        let below = vh - f32::from(rect.bottom());
        let above = f32::from(rect.top()) - view_top;
        let up = below < h && above > below;
        let x = rect.center().x - px(POPOVER_W / 2.);
        Some(if up { at(self.rel(point(x, rect.top())), Corner::BottomLeft, el) } else { at(self.rel(point(x, rect.bottom())), Corner::TopLeft, el) })
    }

    /// The open select's list.
    pub(super) fn reminder_layers(&self, window: &mut Window, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let Some(p) = self.rm.panel.as_ref().filter(|_| self.reminder_open()) else { return vec![] };
        let (Some(d), Some(r)) = (p.drop, self.core.reminder_at(p.at)) else { return vec![] };
        let ix = if d == Drop::Time { 0 } else { 1 };
        let Some(a) = self.rm.selects[ix].get() else { return vec![] };
        let set_time = r.expire - day_start(r.expire);
        let jet = r.notify - r.expire;
        let items: Vec<(String, i64, bool)> = match d {
            Drop::Time => time_slots(set_time).into_iter().map(|t| (slot_label(t), t, t == set_time)).collect(),
            Drop::Jet => jet_lags(r.whole_day, jet).into_iter().map(|(l, v)| (l.to_string(), v, v == jet)).collect(),
        };
        let mut col = div().id(("rm-list", ix)).max_h(px(LIST_H)).py(px(LIST_PAD / 2.)).overflow_y_scroll().when(d == Drop::Time, |d| d.track_scroll(&self.rm.list));
        let n = items.len();
        // List rows: 32 tall, 13pt, grey hover; the current choice
        // in the accent on its tint.
        for (i, (label, v, on)) in items.into_iter().enumerate() {
            col = col.child(
                div()
                    .id(("rm-opt", i))
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
                    .when(on, |s| s.bg(Theme::fill_turninto_active()).text_color(Theme::primary()))
                    .when(!on, |s| s.hover(|s| s.bg(Theme::fill_hover())))
                    .child(label)
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |e, _, window, cx| {
                            cx.stop_propagation();
                            if let Some(p) = &mut e.rm.panel {
                                p.drop = None;
                            }
                            match d {
                                Drop::Time => e.apply_reminder(|r| at_time(r, v), window, cx),
                                Drop::Jet => e.apply_reminder(|r| with_jet(r, v), window, cx),
                            }
                            window.focus(&e.focus);
                        }),
                    ),
            );
        }
        let w = f32::from(a.size.width).max(166.);
        let h = (n as f32 * ITEM_H + LIST_PAD).min(LIST_H) + 2.;
        let vh = f32::from(window.viewport_size().height);
        let below = f32::from(a.bottom()) + 4.;
        let y = if below + h > vh - 8. { f32::from(a.top()) - 4. - h } else { below };
        let list = super::countdown::list_box().w(px(w)).occlude().child(col).on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation());
        vec![deferred(anchored().position_mode(AnchoredPositionMode::Window).position(point(a.left(), px(y))).child(list)).with_priority(3).into_any_element()]
    }
}

#[cfg(test)]
mod tests {
    use super::{DAY, HOUR, MINUTE, current_slot, day_start, default_reminder, jet_lags, parse_time, status, time_slots, time_string_for, toggle, Status};
    use crate::model::InlineReminder;
    use crate::time::local_to_unix;

    fn t(y: i64, m: u32, d: u32, h: u32, mi: u32) -> i64 {
        local_to_unix(y, m, d, h, mi) * 1000
    }

    fn time_string(expire: i64, timed: bool, now: i64) -> String { time_string_for(expire, timed, now, false) }

    #[test]
    fn chip_text() {
        // Thursday 2026-10-22, 10:00.
        let now = t(2026, 10, 22, 10, 0);
        assert_eq!(time_string(t(2026, 10, 22, 14, 0), true, now), "今天 14:00");
        assert_eq!(time_string(t(2026, 10, 23, 0, 0), false, now), "明天");
        assert_eq!(time_string(t(2026, 10, 21, 9, 5), true, now), "昨天 09:05");
        assert_eq!(time_string(t(2026, 10, 25, 9, 0), false, now), "本周日");
        assert_eq!(time_string(t(2026, 10, 20, 9, 0), false, now), "本周二");
        assert_eq!(time_string(t(2026, 10, 27, 9, 0), false, now), "下周二");
        assert_eq!(time_string(t(2026, 11, 3, 9, 0), false, now), "11月3日");
        assert_eq!(time_string(t(2026, 10, 12, 9, 0), false, now), "10月12日");
        assert_eq!(time_string(t(2027, 1, 3, 9, 0), true, now), "2027年1月3日 09:00");
    }

    #[test]
    fn english_chip_text() {
        let now = t(2026, 10, 22, 10, 0);
        assert_eq!(time_string_for(t(2026, 10, 22, 14, 0), true, now, true), "Today 14:00");
        assert_eq!(time_string_for(t(2026, 10, 23, 0, 0), false, now, true), "Tomorrow");
        assert_eq!(time_string_for(t(2026, 10, 21, 9, 5), true, now, true), "Yesterday 09:05");
        assert_eq!(time_string_for(t(2026, 10, 25, 9, 0), false, now, true), "This Sun");
        assert_eq!(time_string_for(t(2026, 10, 27, 9, 0), false, now, true), "Next Tue");
        assert_eq!(time_string_for(t(2026, 11, 3, 9, 0), false, now, true), "11-03");
        assert_eq!(time_string_for(t(2027, 1, 3, 9, 0), true, now, true), "2027-01-03 09:00");
    }

    #[test]
    fn statuses() {
        let now = t(2026, 10, 22, 10, 0);
        assert_eq!(status(now - 1, now), Status::Expired);
        assert_eq!(status(now + 6 * DAY, now), Status::Soon);
        assert_eq!(status(now + 8 * DAY, now), Status::Later);
    }

    #[test]
    fn times() {
        assert_eq!(parse_time("9:5"), Some(9 * HOUR + 5 * MINUTE));
        assert_eq!(parse_time("14：30"), Some(14 * HOUR + 30 * MINUTE));
        assert_eq!(parse_time("3:00 pm"), Some(15 * HOUR));
        assert_eq!(parse_time("24:00"), None);
        assert_eq!(parse_time("12:60"), None);
        assert_eq!(parse_time("930"), None);
        let v = time_slots(9 * HOUR + 15 * MINUTE);
        assert_eq!(v.len(), 49);
        assert_eq!(v[19], 9 * HOUR + 15 * MINUTE);
        assert_eq!(current_slot(t(2026, 10, 22, 9, 59)), 9 * HOUR + 30 * MINUTE);
        assert_eq!(current_slot(t(2026, 10, 22, 23, 40)), 0);
    }

    #[test]
    fn toggles() {
        let now = t(2026, 10, 22, 10, 10);
        let r = default_reminder(now);
        assert_eq!(r.expire, t(2026, 10, 23, 10, 0));
        let w = toggle(&r, true, true, now);
        assert!(w.whole_day);
        assert_eq!(w.expire, t(2026, 10, 23, 23, 59) + 59_000);
        assert_eq!(w.notify, t(2026, 10, 23, 9, 0));
        assert_eq!(w.notify - w.expire, jet_lags(true, 0)[0].1);
        let back = toggle(&w, false, true, now);
        assert_eq!(back.expire, day_start(r.expire) + 10 * HOUR);
        assert_eq!(back.notify, back.expire);
        let off = toggle(&InlineReminder { notify: r.expire - HOUR, ..r }, false, false, now);
        assert_eq!(off.notify, off.expire);
    }
}
