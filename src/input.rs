//! Single-line text field used for search, link URLs and the code language
//! picker. Rendering reuses the editor's rich text element.

use crate::editor::text::{LayoutCache, RichTextElement, TextSpec};
use crate::model::{Align, BlockId, RichText};
use crate::theme::Theme;
use gpui::*;
use std::ops::Range;
use std::sync::atomic::{AtomicU64, Ordering};
use unicode_segmentation::UnicodeSegmentation;

actions!(
    field,
    [Backspace, Delete, Left, Right, SelectLeft, SelectRight, Home, End, SelectAll, Copy, Cut, Paste, Submit, ShiftSubmit, Cancel, Up, Down, Swallow, Tab, ShiftTab]
);

pub fn bind_keys(cx: &mut App) {
    let c = Some("TextField");
    cx.bind_keys([
        KeyBinding::new("backspace", Backspace, c),
        KeyBinding::new("delete", Delete, c),
        KeyBinding::new("left", Left, c),
        KeyBinding::new("right", Right, c),
        KeyBinding::new("shift-left", SelectLeft, c),
        KeyBinding::new("shift-right", SelectRight, c),
        KeyBinding::new("cmd-left", Home, c),
        KeyBinding::new("cmd-right", End, c),
        KeyBinding::new("home", Home, c),
        KeyBinding::new("end", End, c),
        KeyBinding::new("cmd-a", SelectAll, c),
        KeyBinding::new("cmd-c", Copy, c),
        KeyBinding::new("cmd-x", Cut, c),
        KeyBinding::new("cmd-v", Paste, c),
        KeyBinding::new("enter", Submit, c),
        KeyBinding::new("shift-enter", ShiftSubmit, c),
        KeyBinding::new("escape", Cancel, c),
        KeyBinding::new("up", Up, c),
        KeyBinding::new("down", Down, c),
        KeyBinding::new("ctrl-p", Up, c),
        KeyBinding::new("ctrl-n", Down, c),
        KeyBinding::new("tab", Tab, c),
        KeyBinding::new("shift-tab", ShiftTab, c),
        KeyBinding::new("cmd-z", Swallow, c),
        KeyBinding::new("cmd-shift-z", Swallow, c),
    ]);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextFieldEvent {
    Changed,
    Submit,
    /// Shift+Enter (查找: previous match).
    ShiftSubmit,
    Cancel,
    Up,
    Down,
    /// Backspace with nothing to delete (an image caption selects its block).
    BackspaceEmpty,
    /// Left with a caret at the start / Right with a caret at the end.
    LeftEdge,
    RightEdge,
    /// Tab / Shift+Tab when `tab_events` is set (moving between fields).
    Tab,
    ShiftTab,
}

static NEXT_ID: AtomicU64 = AtomicU64::new(u64::MAX - 1_000_000);

pub struct TextField {
    focus: FocusHandle,
    pub text: String,
    sel: Range<usize>,
    reversed: bool,
    marked: Option<Range<usize>>,
    placeholder: SharedString,
    cache: LayoutCache,
    id: BlockId,
    pub size: f32,
    pub line_height: f32,
    pub placeholder_color: Hsla,
    pub color: Hsla,
    pub align: Align,
    /// Wrap long text onto more lines instead of scrolling it.
    pub wrap: bool,
    /// Enter and pasted newlines insert line breaks; Up/Down move by line.
    pub multiline: bool,
    pub mono: bool,
    /// Longest text in chars (longer input is cut off).
    pub max_len: Option<usize>,
    /// Only ASCII digits get in (number inputs).
    pub digits: bool,
    /// Tab / Shift+Tab emit events instead of reaching the parent.
    pub tab_events: bool,
    /// Draw no caret even when focused (a field that takes keys before it
    /// looks focused, like the emoji search).
    pub hide_caret: bool,
    selecting: bool,
    /// Typing hides the mouse pointer until it moves (see the editor's).
    pointer_hidden: bool,
}

impl EventEmitter<TextFieldEvent> for TextField {}

impl Focusable for TextField {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl TextField {
    pub fn new(placeholder: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        TextField {
            focus: cx.focus_handle(),
            text: String::new(),
            sel: 0..0,
            reversed: false,
            marked: None,
            placeholder: placeholder.into(),
            cache: Default::default(),
            id: BlockId(NEXT_ID.fetch_add(1, Ordering::Relaxed)),
            // macOS's control text: 13pt.
            size: 13.,
            line_height: 20.,
            placeholder_color: Theme::text_placeholder(),
            color: Theme::text(),
            align: Align::Left,
            wrap: false,
            multiline: false,
            mono: false,
            max_len: None,
            digits: false,
            tab_events: false,
            hide_caret: false,
            selecting: false,
            pointer_hidden: false,
        }
    }

    pub fn set_placeholder(&mut self, s: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.placeholder = s.into();
        cx.notify();
    }

    pub fn set_text(&mut self, s: &str, cx: &mut Context<Self>) {
        self.text = s.to_string();
        self.sel = self.text.len()..self.text.len();
        self.marked = None;
        cx.notify();
    }

    pub fn select_all_text(&mut self, cx: &mut Context<Self>) {
        self.sel = 0..self.text.len();
        self.reversed = false;
        cx.notify();
    }

    pub fn is_focused(&self, window: &Window) -> bool {
        self.focus.is_focused(window)
    }

    fn cursor(&self) -> usize {
        if self.reversed { self.sel.start } else { self.sel.end }
    }

    fn move_to(&mut self, o: usize, cx: &mut Context<Self>) {
        self.sel = o..o;
        self.reversed = false;
        cx.notify();
    }

    fn select_to(&mut self, o: usize, cx: &mut Context<Self>) {
        if self.reversed {
            self.sel.start = o;
        } else {
            self.sel.end = o;
        }
        if self.sel.end < self.sel.start {
            self.reversed = !self.reversed;
            self.sel = self.sel.end..self.sel.start;
        }
        cx.notify();
    }

    fn prev(&self, o: usize) -> usize {
        self.text.grapheme_indices(true).rev().find_map(|(i, _)| (i < o).then_some(i)).unwrap_or(0)
    }

    fn next(&self, o: usize) -> usize {
        self.text.grapheme_indices(true).find_map(|(i, _)| (i > o).then_some(i)).unwrap_or(self.text.len())
    }

    fn replace(&mut self, range: Range<usize>, s: &str, cx: &mut Context<Self>) {
        let mut s = if self.multiline { s.replace("\r\n", "\n").replace('\r', "\n") } else { s.replace(['\n', '\r'], " ") };
        if self.digits {
            s.retain(|c| c.is_ascii_digit());
        }
        if let Some(max) = self.max_len {
            let room = max.saturating_sub(self.text.chars().count() - self.text[range.clone()].chars().count());
            if let Some((cut, _)) = s.char_indices().nth(room) {
                s.truncate(cut);
            }
        }
        self.text.replace_range(range.clone(), &s);
        let o = range.start + s.len();
        self.sel = o..o;
        self.reversed = false;
        cx.emit(TextFieldEvent::Changed);
        cx.notify();
    }

    fn backspace(&mut self, _: &Backspace, _: &mut Window, cx: &mut Context<Self>) {
        if self.text.is_empty() {
            cx.emit(TextFieldEvent::BackspaceEmpty);
            return;
        }
        if self.sel.is_empty() {
            let p = self.prev(self.cursor());
            self.select_to(p, cx);
        }
        self.replace(self.sel.clone(), "", cx);
    }

    fn delete(&mut self, _: &Delete, _: &mut Window, cx: &mut Context<Self>) {
        if self.sel.is_empty() {
            let n = self.next(self.cursor());
            self.select_to(n, cx);
        }
        self.replace(self.sel.clone(), "", cx);
    }

    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        if self.sel.is_empty() && self.cursor() == 0 {
            cx.emit(TextFieldEvent::LeftEdge);
        }
        let o = if self.sel.is_empty() { self.prev(self.cursor()) } else { self.sel.start };
        self.move_to(o, cx);
    }

    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        if self.sel.is_empty() && self.cursor() == self.text.len() {
            cx.emit(TextFieldEvent::RightEdge);
        }
        let o = if self.sel.is_empty() { self.next(self.cursor()) } else { self.sel.end };
        self.move_to(o, cx);
    }

    fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        let p = self.prev(self.cursor());
        self.select_to(p, cx);
    }

    fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        let n = self.next(self.cursor());
        self.select_to(n, cx);
    }

    fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
    }

    fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.text.len(), cx);
    }

    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.select_all_text(cx);
    }

    fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        if !self.sel.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(self.text[self.sel.clone()].to_string()));
        }
    }

    fn cut(&mut self, _: &Cut, _: &mut Window, cx: &mut Context<Self>) {
        if !self.sel.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(self.text[self.sel.clone()].to_string()));
            self.replace(self.sel.clone(), "", cx);
        }
    }

    fn paste(&mut self, _: &Paste, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(t) = cx.read_from_clipboard().and_then(|i| i.text()) {
            self.replace(self.sel.clone(), &t, cx);
        }
    }

    fn submit(&mut self, _: &Submit, _: &mut Window, cx: &mut Context<Self>) {
        if self.multiline {
            return self.replace(self.sel.clone(), "\n", cx);
        }
        cx.emit(TextFieldEvent::Submit);
    }

    fn shift_submit(&mut self, _: &ShiftSubmit, _: &mut Window, cx: &mut Context<Self>) {
        if self.multiline {
            return self.replace(self.sel.clone(), "\n", cx);
        }
        cx.emit(TextFieldEvent::ShiftSubmit);
    }

    fn swallow(&mut self, _: &Swallow, _: &mut Window, cx: &mut Context<Self>) {
        if !self.multiline {
            cx.propagate();
        }
    }

    fn tab(&mut self, _: &Tab, _: &mut Window, cx: &mut Context<Self>) {
        match (self.tab_events, self.multiline) {
            (true, _) => cx.emit(TextFieldEvent::Tab),
            (false, false) => cx.propagate(),
            _ => {}
        }
    }

    fn shift_tab(&mut self, _: &ShiftTab, _: &mut Window, cx: &mut Context<Self>) {
        match (self.tab_events, self.multiline) {
            (true, _) => cx.emit(TextFieldEvent::ShiftTab),
            (false, false) => cx.propagate(),
            _ => {}
        }
    }

    /// Move the caret a line up (`-1`) or down; past the first/last line it
    /// goes to the start/end.
    fn move_line(&mut self, dir: f32, cx: &mut Context<Self>) {
        let target = {
            let cache = self.cache.borrow();
            let Some(l) = cache.get(&self.id) else { return };
            let at = l.position_for(self.cursor());
            let y = at.y + l.line_height * (0.5 + dir);
            let first = l.position_for(0).y;
            let last = l.position_for(self.text.len()).y;
            if y < first {
                0
            } else if y > last + l.line_height {
                self.text.len()
            } else {
                l.index_for(point(at.x, y)).min(self.text.len())
            }
        };
        self.move_to(target, cx);
    }

    fn cancel(&mut self, _: &Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(TextFieldEvent::Cancel);
    }

    fn up(&mut self, _: &Up, _: &mut Window, cx: &mut Context<Self>) {
        if self.multiline {
            return self.move_line(-1., cx);
        }
        cx.emit(TextFieldEvent::Up);
    }

    fn down(&mut self, _: &Down, _: &mut Window, cx: &mut Context<Self>) {
        if self.multiline {
            return self.move_line(1., cx);
        }
        cx.emit(TextFieldEvent::Down);
    }

    fn index_for(&self, p: Point<Pixels>) -> usize {
        self.cache.borrow().get(&self.id).map(|l| l.index_for(p)).unwrap_or(0).min(self.text.len())
    }

    fn on_down(&mut self, ev: &MouseDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        window.focus(&self.focus);
        let i = self.index_for(ev.position);
        if ev.click_count >= 2 {
            self.select_all_text(cx);
            return;
        }
        self.selecting = true;
        if ev.modifiers.shift { self.select_to(i, cx) } else { self.move_to(i, cx) }
    }

    fn on_move(&mut self, ev: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.selecting && ev.pressed_button == Some(MouseButton::Left) {
            let i = self.index_for(ev.position);
            self.select_to(i, cx);
        }
    }

    fn on_up(&mut self, _: &MouseUpEvent, _: &mut Window, _: &mut Context<Self>) {
        self.selecting = false;
    }

    // utf16 helpers
    fn to16(&self, o: usize) -> usize {
        self.text[..o.min(self.text.len())].encode_utf16().count()
    }

    fn from16(&self, o16: usize) -> usize {
        let mut n = 0;
        for (i, ch) in self.text.char_indices() {
            if n >= o16 {
                return i;
            }
            n += ch.len_utf16();
        }
        self.text.len()
    }

    fn range16(&self, r: &Range<usize>) -> Range<usize> {
        self.to16(r.start)..self.to16(r.end)
    }

    fn range8(&self, r: &Range<usize>) -> Range<usize> {
        self.from16(r.start)..self.from16(r.end)
    }
}

impl EntityInputHandler for TextField {
    fn text_for_range(
        &mut self,
        range: Range<usize>,
        actual: &mut Option<Range<usize>>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<String> {
        let r = self.range8(&range);
        actual.replace(self.range16(&r));
        Some(self.text[r].to_string())
    }

    fn selected_text_range(&mut self, _: bool, _: &mut Window, _: &mut Context<Self>) -> Option<UTF16Selection> {
        Some(UTF16Selection { range: self.range16(&self.sel), reversed: self.reversed })
    }

    fn marked_text_range(&self, _: &mut Window, _: &mut Context<Self>) -> Option<Range<usize>> {
        self.marked.as_ref().map(|r| self.range16(r))
    }

    fn unmark_text(&mut self, _: &mut Window, _: &mut Context<Self>) {
        self.marked = None;
    }

    fn replace_text_in_range(&mut self, range: Option<Range<usize>>, s: &str, _: &mut Window, cx: &mut Context<Self>) {
        self.pointer_hidden = true;
        let r = range.map(|r| self.range8(&r)).or(self.marked.clone()).unwrap_or(self.sel.clone());
        self.marked = None;
        self.replace(r, s, cx);
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        s: &str,
        new_sel: Option<Range<usize>>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.pointer_hidden = true;
        let r = range.map(|r| self.range8(&r)).or(self.marked.clone()).unwrap_or(self.sel.clone());
        self.text.replace_range(r.clone(), s);
        self.marked = (!s.is_empty()).then(|| r.start..r.start + s.len());
        let sel = new_sel
            .map(|ns| {
                let base16 = self.to16(r.start);
                self.from16(base16 + ns.start)..self.from16(base16 + ns.end)
            })
            .unwrap_or(r.start + s.len()..r.start + s.len());
        self.sel = sel;
        self.reversed = false;
        cx.emit(TextFieldEvent::Changed);
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range: Range<usize>,
        _: Bounds<Pixels>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let r = self.range8(&range);
        let cache = self.cache.borrow();
        let l = cache.get(&self.id)?;
        let a = l.position_for(r.start);
        let b = l.position_for(r.end);
        Some(Bounds::from_corners(a, point(b.x, b.y + l.line_height)))
    }

    fn character_index_for_point(&mut self, p: Point<Pixels>, _: &mut Window, _: &mut Context<Self>) -> Option<usize> {
        Some(self.to16(self.index_for(p)))
    }
}

impl Render for TextField {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focused = self.focus.is_focused(window);
        let mut spec = TextSpec::body();
        spec.doc_font = false;
        spec.size = self.size;
        spec.line_height = self.line_height;
        spec.color = self.color;
        spec.wrap = self.wrap;
        spec.mono = self.mono;
        let el = RichTextElement {
            id: self.id,
            text: RichText::plain(self.text.clone()),
            spec,
            align: self.align,
            placeholder: Some(crate::settings::label(self.placeholder.clone()).into()),
            placeholder_color: Some(self.placeholder_color),
            placeholder_dx: 0.,
            selection: (focused && !self.sel.is_empty()).then(|| self.sel.clone()),
            caret: Some(self.cursor()),
            marked: self.marked.clone(),
            cache: self.cache.clone(),
            show_caret: focused && !self.hide_caret && self.sel.is_empty(),
            highlight: None,
            found: vec![],
            eq_counter: 0,
            button_edit: None,
        };
        let entity = cx.entity();
        let focus = self.focus.clone();
        let pointer_hidden = self.pointer_hidden;
        div()
            .id(("text-field", self.id.0))
            .track_focus(&self.focus)
            .key_context("TextField")
            .cursor(CursorStyle::IBeam)
            .relative()
            .flex_1()
            .min_w_0()
            .overflow_hidden()
            .on_action(cx.listener(Self::backspace))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::left))
            .on_action(cx.listener(Self::right))
            .on_action(cx.listener(Self::select_left))
            .on_action(cx.listener(Self::select_right))
            .on_action(cx.listener(Self::home))
            .on_action(cx.listener(Self::end))
            .on_action(cx.listener(Self::select_all))
            .on_action(cx.listener(Self::copy))
            .on_action(cx.listener(Self::cut))
            .on_action(cx.listener(Self::paste))
            .on_action(cx.listener(Self::submit))
            .on_action(cx.listener(Self::shift_submit))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::up))
            .on_action(cx.listener(Self::down))
            .on_action(cx.listener(Self::swallow))
            .on_action(cx.listener(Self::tab))
            .on_action(cx.listener(Self::shift_tab))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_down))
            .on_mouse_move(cx.listener(Self::on_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_up))
            .text_color(self.color)
            .child(el)
            .child(
                canvas(
                    |_, _, _| (),
                    move |bounds, _, window, cx| {
                        window.handle_input(&focus, ElementInputHandler::new(bounds, entity.clone()), cx);
                        if pointer_hidden {
                            window.set_window_cursor_style(CursorStyle::None);
                            let e = entity.clone();
                            window.on_mouse_event(move |_: &MouseMoveEvent, phase, _, cx| {
                                if phase == DispatchPhase::Bubble {
                                    e.update(cx, |f, cx| {
                                        f.pointer_hidden = false;
                                        cx.notify();
                                    });
                                }
                            });
                        }
                    },
                )
                .absolute()
                .size_full(),
            )
    }
}
