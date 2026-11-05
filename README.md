# Odyssey

Local-first notes with a native rich-text/table editor and Obsidian-style knowledge links.

[简体中文](README.zh-CN.md) · [MIT license](LICENSE)

## Preview status

This is the **build 4 source snapshot**, based on [quiet.md](https://github.com/s1dashu/quiet-md) commit `52afb284d7132dba1a1efb2944a456aaed9d44f3`. It is an early preview, not a complete Obsidian replacement.

Implemented in this snapshot:

- Native rich-text and table editing inherited from quiet.md.
- Wiki-link completion in body text and table cells; aliases, previews and navigation.
- Uncreated-link targets with persistent identity and creation on navigation.
- Inline document links rendered as colored text without leading icons.
- Esc dismisses completion until a new trigger is entered.
- Independent default library at `~/Documents/Odyssey`; upstream automatic updates disabled.

Vault management, four-column workspace, tabs/splits, persistent block anchors, embeds and graphs remain planned work. See [PRODUCT.md](PRODUCT.md). Do not interpret roadmap items as implemented features.

Notes are stored as DocxXML, not Markdown files. Opening an Obsidian vault or loading its plugins is not supported by this preview. Back up important notes before testing.

## Build and test

The toolchain is pinned in `rust-toolchain.toml`. On macOS use Xcode and its Metal toolchain. Rust package/binary names remain `quiet_md` internally.

```sh
cargo build --release --locked
QUIET_MD_HOME=/tmp/odyssey-dev ./target/release/quiet_md
scripts/check.sh
FDL_FUZZ_SEEDS=2000 scripts/check.sh
```

Use an isolated scratch library for testing. No notarized Odyssey binary is published here. Windows is inherited upstream code and has not been validated for this snapshot.

## Direction

Four-column workspace: tool ribbon, navigation, original document editor, contextual sidebar. Existing quiet.md interactions retain their behavior when relocated. Local editing stays free; a future optional Publish service may be paid. Publish is outside the current phase.

## Credits and license

Fork of quiet.md by s1dashu. Original copyright retained; Odyssey changes are MIT. See [THIRD_PARTY.md](THIRD_PARTY.md) for dependency and asset licenses. Commercial upstream shell icons have been replaced. The upstream name and original app branding are not Odyssey branding.
