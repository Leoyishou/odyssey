# Odyssey

Local-first knowledge editor combining Obsidian-style links and workspace navigation with a native rich-text and table editor. Built with Rust + GPUI, based on [quiet.md](https://github.com/s1dashu/quiet-md).

[Preview downloads](https://github.com/Leoyishou/odyssey/releases) · [简体中文](README.md) · [Product boundaries](PRODUCT.md)

## Current status

This is an early development snapshot (local build 6), not a feature-complete release. The Cargo version remains the upstream 0.1.5; it does not imply an Odyssey public release with that version.

Implemented and exercised on macOS Apple Silicon:

- Native rich-text editing and tables inherited from quiet.md.
- Document-link completion using `[[`, `【【`, `［［` and the existing `@` picker.
- Display aliases, selection-to-link with `⌘⇧K`, persistent unresolved links and creation on click.
- Plain-text-colored inline references (no leading document icon or chip), hover previews, existing incoming/outgoing reference lists.
- Links inside tables, navigation back to the source cell, rename identity preservation, undo/redo and save/reopen scenarios.
- Four-column workspace using the existing quiet.md visual components and editor actions; outline and reference lists in the right sidebar.
- Create/open/recent vaults; save before opening a separate vault window; independent library processes and last-vault restoration.
- Independent Odyssey application identity and default data directory; upstream automatic updates are disabled.

Still incomplete: full workspace restore, workspace tabs and splits, persistent block/heading/cell anchors, document-level aliases, unresolved-title disambiguation, unlinked mentions, embeds/cycle protection, graph views and complete import/export/performance regression. The main editor remains rich text; this is **not** a Markdown-only Obsidian vault implementation or a claim of Obsidian plugin compatibility.

Long inline reference labels remain atomic editor objects: if too wide, the object starts on a new line and wraps internally. Surrounding text does not yet flow through that object.

## Build and test

Use the Rust version pinned in `rust-toolchain.toml`. macOS requires Xcode and its Metal toolchain. If Metal compilation reports a missing toolchain, install it with `xcodebuild -downloadComponent MetalToolchain`.

```sh
cargo build --release --locked
ODYSSEY_HOME=/tmp/odyssey-scratch ./target/release/quiet_md
scripts/check.sh
python3 scripts/package-odyssey.py
```

`package-odyssey.py` creates `dist/Odyssey.app` with local ad-hoc signing. It does not install, publish, notarize, or stop applications. Public distribution needs its own signing/notarization process. Upstream release/upload scripts are disabled in this fork.

The new macOS default library is `~/Library/Application Support/Odyssey/Vaults/Default`; existing pure Odyssey legacy vaults stay in place, while mixed Markdown folders are refused before writes. `ODYSSEY_HOME` overrides it; `QUIET_MD_HOME` remains supported for the upstream test harness. Never point development tests at a real user's notes. Data uses DocxXML with stable document UUIDs; plain text is not a lossless export of every rich-text/table feature.

Release acceptance criteria: [docs/ACCEPTANCE.md](docs/ACCEPTANCE.md); release procedure: [docs/RELEASE-CHECKLIST.md](docs/RELEASE-CHECKLIST.md). Prior-build checks are historical, not proof for a new release.

GUI harness instructions: [tools/harness/README.md](tools/harness/README.md). Automated checks cover the workspace tests, bundled-guide round trips and clippy. Screenshots, personal libraries and local session logs are intentionally not included in this public source snapshot.

## Product boundaries

The visual style, typography and controls retain quiet.md; Obsidian guides UX only. The document area retains the native rich-text and table interactions. Workspace navigation around it is being developed toward Obsidian-like behavior. Non-Publish functionality is intended to remain free. Publish is not being developed in this phase; no payment or hosting integration is included, and notes are not uploaded by default.

## License and attribution

MIT: see [LICENSE](LICENSE). Original quiet.md copyright is preserved. Third-party dependencies and assets retain their respective licenses: see [THIRD_PARTY.md](THIRD_PARTY.md).

The Odyssey app icon is an original compass design. The upstream commercial shell icons have been replaced with ISC/MIT editor icons and original geometry. Quiet.md's name and application branding are not claimed by this fork.

Upstream source checkpoint: `52afb284d7132dba1a1efb2944a456aaed9d44f3`. See [ODYSSEY.md](ODYSSEY.md) for the current fork notes and [docs/FORMAT.md](docs/FORMAT.md) for storage extensions.

Build6 adds previewed copy import into a new separate vault, byte-for-byte original Markdown archives and conversion reports. Folder pages preserve hierarchy; resolvable note links and referenced local attachments are copied. This is not native Obsidian storage compatibility. YAML, plugins, note embeds and heading/block anchors are limited; see the Chinese README for details.
