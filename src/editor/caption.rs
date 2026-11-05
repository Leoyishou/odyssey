//! Code block names and image descriptions: a single-line field per block,
//! created when the block is first rendered and kept in step with the
//! document (undo, other edits) while it isn't being typed in.

use super::core::{self, Pos};
use super::view::{Editor, EditorEvent};
use crate::input::{TextField, TextFieldEvent};
use crate::model::{Block, BlockId, BlockKind};
use crate::editor_theme::Theme;
use gpui::*;

/// The name a new code block gets.
pub const CODE_CAPTION: &str = "代码块";

/// The text a caption field shows for a block, if the block has one.
fn caption_text(b: &Block) -> Option<String> {
    match &b.kind {
        BlockKind::Code { caption, .. } => Some(caption.clone().unwrap_or_else(|| crate::settings::text(CODE_CAPTION).into())),
        BlockKind::Image { caption, .. } => Some(caption.clone().unwrap_or_default()),
        _ => None,
    }
}

/// (block, is a code block, caption text) for the captioned blocks drawn in
/// `b`'s list row.
pub(super) fn captions_in(b: &Block) -> Vec<(BlockId, bool, String)> {
    fn walk(b: &Block, out: &mut Vec<(BlockId, bool, String)>) {
        if let Some(t) = caption_text(b) {
            out.push((b.id, matches!(b.kind, BlockKind::Code { .. }), t));
        }
        if let BlockKind::Table(t) = &b.kind {
            for cell in t.rows.iter().flatten() {
                cell.blocks.iter().for_each(|c| walk(c, out));
            }
        }
        b.children.iter().for_each(|c| walk(c, out));
    }
    let mut out = vec![];
    // The children of a flattened row are rows of their own.
    if super::render::flattens(b) {
        out.extend(caption_text(b).map(|t| (b.id, false, t)));
    } else {
        walk(b, &mut out);
    }
    out
}

impl Editor {
    /// Creates or refreshes the fields of `captions_in`'s blocks.
    pub(super) fn ensure_captions(&mut self, found: Vec<(BlockId, bool, String)>, window: &mut Window, cx: &mut Context<Self>) {
        for (id, code, text) in found {
            if let Some((f, _)) = self.captions.get(&id) {
                // A field being typed in is ahead of the document until its
                // change events are delivered.
                if f.read(cx).text != text && !f.read(cx).focus_handle(cx).is_focused(window) {
                    f.update(cx, |f, cx| f.set_text(&text, cx));
                }
                continue;
            }
            let f = cx.new(|cx| {
                let mut f = TextField::new(if code { "请输入代码块名称" } else { "输入图片描述" }, cx);
                // The caption: 13pt in the caption grey (its weight 300 needs
                // a weight on TextField; the field draws regular).
                f.color = Theme::caption();
                f.size = 13.;
                f.line_height = if code { 30. } else { 20. };
                // Image descriptions are centred and wrap (pre-wrap).
                if !code {
                    f.align = crate::model::Align::Center;
                    f.wrap = true;
                }
                f.set_text(&text, cx);
                f
            });
            let subs = vec![cx.subscribe_in(&f, window, move |e, _, ev: &TextFieldEvent, window, cx| e.on_caption_event(id, ev, window, cx))];
            self.captions.insert(id, (f, subs));
        }
    }

    fn is_code(&self, id: BlockId) -> bool {
        self.core.block(id).is_some_and(|b| matches!(b.kind, BlockKind::Code { .. }))
    }

    pub(super) fn caption_field(&self, id: BlockId) -> Option<Entity<TextField>> {
        self.captions.get(&id).map(|(f, _)| f.clone())
    }

    fn on_caption_event(&mut self, id: BlockId, ev: &TextFieldEvent, window: &mut Window, cx: &mut Context<Self>) {
        match ev {
            TextFieldEvent::Changed => {
                let Some((f, _)) = self.captions.get(&id) else { return };
                let text = f.read(cx).text.clone();
                self.core.checkpoint(core::EditKind::Typing);
                if let Some(b) = self.core.block_mut(id) {
                    match &mut b.kind {
                        BlockKind::Code { caption, .. } | BlockKind::Image { caption, .. } => *caption = Some(text),
                        _ => {}
                    }
                }
                self.core.dirty = false;
                cx.emit(EditorEvent::Changed);
                cx.notify();
            }
            // Enter / ↓ / Esc leave a code name for the start of the code.
            TextFieldEvent::Submit | TextFieldEvent::Down | TextFieldEvent::Cancel if self.is_code(id) => {
                window.focus(&self.focus);
                self.core.clear_block_sel();
                self.core.cell_sel = None;
                self.core.sel = core::Selection::caret(Pos::new(id, 0));
                self.changed(window, cx);
            }
            // Image description: Enter starts a paragraph below the image;
            // Esc, ↑ and Backspace on an empty description select the image.
            TextFieldEvent::Submit => {
                window.focus(&self.focus);
                let nid = self.core.insert_paragraph_after(id);
                self.core.clear_block_sel();
                self.core.cell_sel = None;
                self.core.sel = core::Selection::caret(Pos::new(nid, 0));
                self.changed(window, cx);
            }
            TextFieldEvent::Cancel | TextFieldEvent::Up | TextFieldEvent::BackspaceEmpty => {
                window.focus(&self.focus);
                self.core.select_block(id);
                self.changed(window, cx);
            }
            _ => {}
        }
    }
}
