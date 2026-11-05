# Four-column migration map (build 5)

Visual constraint: retain quiet.md typography, themes, icon style, controls, radii and spacing. Obsidian informs workspace UX only, not a replacement skin. The explicitly requested plain inline link style remains.

The central document editor keeps its existing rich-text, block, slash-menu and table behavior. Move existing controls without replacing their commands, state, save gates or focus behavior.

| Existing entry | Four-column destination | Existing behavior to retain |
|---|---|---|
| Document new buttons / Cmd-N | Tool rail | `NewDoc` → `Workspace::new_doc` → `save_before` → create/open/focus |
| Home entry | Left navigation | `GoHome` → `show_view(View::Home)`; keep the live editor and its autosave |
| Search entry / Cmd-J | Left navigation | `FocusSearch` → `open_search` → `pick_search` → `open/try_open`; restore editor focus on close |
| Document tree rows | Left navigation | `doc_row` → `open/try_open` with the existing save gate |
| Tree context menu | Left navigation | Keep `duplicate_doc`, `open_move`, `toggle_pin/star`, `start_rename`, `ask_delete` and new-window handling |
| Settings / Cmd-comma | Tool rail | Keep `OpenSettings`/`show_view(Settings)` and settings page state |
| Title, cover, content, slash/block/table controls | Main editor | Keep existing Editor entity, Core, command listeners, undo and Changed/save flow |
| Outline | Right auxiliary area | Reuse `entries`, catalogue state, scroll spy, `jump_to_heading`, `Catalogue` persistence event |
| Backlinks/outgoing references | Right auxiliary area | Reuse `Links`, `set_links`, `refs_list` handlers, fold/more/tab state and `OpenDoc`, `OpenDocAt`, `HideRefs` events |
| Find in current document / Cmd-F | Main editor | Keep separate from whole-vault search |
| Navigation back/forward | New workspace capability | Not currently implemented; do not repurpose editor undo or recent-document sorting |

## Integration constraints

- Keep auxiliary rendering in the owning Editor context so existing GPUI listeners and state are reused. Do not duplicate outline/reference logic in Workspace.
- Allocate the right column once. Existing pinned-outline page margins must not narrow the main document a second time. Recompute widths on resize and standard/wide/full page modes.
- Preserve the document-end blank click target when moving references out of `Item::Below`.
- Nonediting auxiliary actions must not introduce document dirty state. Outline navigation that unfolds content must retain its existing Changed/save behavior.
- Keep old visibility/pinning preferences explicit; do not silently repurpose a per-document preference as a global workspace setting.
- Vault UI must validate selected folders before any Store or bundled-guide writes. Current settings/themes/shortcuts are process-global, so naive in-process root switching is unsafe.

## Remaining regressions

Build 5 exercised outline jump, source-cell navigation, body/table undo, both sidebar resizing/collapse and failed-save blocking. See RELEASE-CHECKLIST.md and ACCEPTANCE.md. Still to broaden: outline tracking and fold persistence; reference count/tab/collapse/more/hide/restore; source-cell navigation; footer caret placement; focus and editor shortcuts after sidebar interaction; failed-save behavior on document/vault changes; narrow window/resize; two libraries with identical document IDs; layout restore. Existing build-4 editor tests do not prove these new layout behaviors.
