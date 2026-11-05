# Odyssey build 5 validation

macOS Apple Silicon development preview, 2026-11-06. Upstream baseline: `52afb284d7132dba1a1efb2944a456aaed9d44f3`.

## Automated checks

The independent publication checkout ran `FDL_FUZZ_SEEDS=2000 scripts/check.sh`: 88 core tests and 50 application tests passed, bundled-guide round trip and guide-asset checks passed, and release clippy passed with warnings denied. Upstream `block` and `proc-macro-error2` future-compatibility notices remain. The public source also built with `cargo build --release --locked`.

## Exercised workflows

- Four-column light/dark workspace using the existing quiet.md components; left/right collapse and resize, outline jump and narrow-window auxiliary overlay.
- Body and table editing and undo; table link completion, plain-text reference rendering, hover preview and backlink navigation to the source cell.
- Create/open vault UI, cancellation without a launch request, nonempty-folder protection, Markdown/future-format rejection without changing the selected folder.
- Failed save blocks opening another vault; cancelling the failure dialog leaves unsaved content in the editor.
- Separate real processes edit the same document UUID in two independent libraries, save, quit and reopen; document contents and appearance preferences remain separate. Launch without a path restores the last vault.
- Installed bundle and mounted DMG signature checks; installed bundle launched in an isolated offscreen test library.

UI tests use an offscreen native GPUI harness. Its open-vault action records a request instead of spawning a foreground process; process isolation and restart were tested separately. Native file dialogs, actual system IME and window-server materials remain unverified. This is not a full pass of every acceptance item.

## Release limitations

Local ad-hoc signature only; no Developer ID signature or Apple notarization. macOS may block a downloaded copy on another machine. No security-policy changes or bypasses are provided. Windows is inherited but not validated.

Tabs/splits/history/command palette, persistent heading/block/cell anchors, full alias management, mentions, embeds, graphs and complete import/export/performance coverage remain incomplete. Vault folders use Odyssey DocxXML, not Obsidian Markdown compatibility. A vault opens in a separate process/window after saving the current document; the original window stays open.

Artifact checksums are provided alongside the preview download. The Cargo version is still upstream 0.1.5; the preview identity is `build-5` / CFBundleVersion 5. MIT and third-party notices are retained.
