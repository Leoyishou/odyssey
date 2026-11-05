//! 阅读模式 and the 文档信息 row under the title.
//!
//! Reading mode: no handles, toolbars or header entries, no caret, and
//! nothing typed reaches the document, while selecting and copying still
//! work. Edits are refused at the key context ("EditorRO" binds only moving,
//! selecting, copying and Esc) and by skipping the input handler; anything a
//! click still manages to change is rolled back from a snapshot by
//! `reject_read_only_edit`, with a reminder toast.

use super::view::{Editor, EditorEvent};
use crate::store::InfoRow;
use crate::editor_ui::ToastKind;
use gpui::*;

pub const READ_ONLY_HINT: &str = "当前为阅读模式，请切换为编辑模式后重试";

impl Editor {
    pub fn set_read_only(&mut self, on: bool, cx: &mut Context<Self>) {
        if self.read_only == on {
            return;
        }
        self.read_only = on;
        self.ro_doc = on.then(|| self.core.doc.clone());
        self.core.break_coalescing();
        self.menu = None;
        self.slash = None;
        self.marked = None;
        self.hover = None;
        self.img.hover = None;
        self.gap_hover = None;
        self.tables.insert = None;
        self.hide_link_card(cx);
        if on {
            self.core.clear_block_sel();
            self.core.cell_sel = None;
        }
        cx.notify();
    }

    /// Called for every `Changed` the editor emits: in reading mode an edited
    /// document goes back to the snapshot (folding and renaming are allowed,
    /// they are kept in the snapshot as they happen) and true is returned.
    pub fn reject_read_only_edit(&mut self, cx: &mut Context<Self>) -> bool {
        if !self.read_only {
            return false;
        }
        let Some(doc) = self.ro_doc.clone() else { return false };
        if doc == self.core.doc {
            return false;
        }
        self.core.restore(doc);
        self.toast_kind(ToastKind::Info, READ_ONLY_HINT, cx);
        cx.notify();
        true
    }

    /// Folding in reading mode: let it through the snapshot.
    pub(super) fn keep_read_only_fold(&mut self) {
        if let Some(doc) = &mut self.ro_doc {
            *doc = self.core.doc.clone();
        }
    }

    /// The sidebar renamed the document: the title is not an edit here.
    pub(super) fn keep_read_only_title(&mut self) {
        if let Some(doc) = &mut self.ro_doc {
            doc.title = self.core.doc.title.clone();
        }
    }

    /// Keys nothing is bound to in reading mode: a plain E switches to
    /// editing (the 编辑模式 shortcut "E"), typing gets the reminder.
    pub(super) fn read_only_key(&mut self, ev: &KeyDownEvent, cx: &mut Context<Self>) {
        let k = &ev.keystroke;
        let m = k.modifiers;
        if m.platform || m.control || m.function {
            if m.platform && !m.control && matches!(k.key.as_str(), "v" | "x" | "z" | "b" | "i" | "u" | "k") {
                self.toast_kind(ToastKind::Info, READ_ONLY_HINT, cx);
            }
            return;
        }
        if k.key == "e" && !m.shift && !m.alt {
            cx.stop_propagation();
            cx.emit(EditorEvent::Mode(false));
            return;
        }
        let typing = k.key_char.as_ref().is_some_and(|c| !c.is_empty()) || matches!(k.key.as_str(), "enter" | "backspace" | "delete" | "tab");
        if typing {
            cx.stop_propagation();
            self.toast_kind(ToastKind::Info, READ_ONLY_HINT, cx);
        }
    }

    // ---- 文档信息 row

    fn set_info(&mut self, info: InfoRow, cx: &mut Context<Self>) {
        self.info = info;
        cx.emit(EditorEvent::Info(info));
        cx.notify();
    }

    pub(super) fn show_info(&mut self, cx: &mut Context<Self>) {
        let info = InfoRow { hidden: false, ..self.info };
        self.set_info(info, cx);
    }

}
