# Build 4 source validation

Validated on macOS Apple Silicon, 2026-11-06.

Command: `FDL_FUZZ_SEEDS=2000 scripts/check.sh`

- Core: 85 passed, 2 ignored.
- Application: 46 passed, 1 ignored.
- Bundled guide round-trip: 1 passed.
- Bundled guide assets match source assets.
- Release clippy, all workspace targets: passed with `-D warnings`.
- Cargo reports future-compatibility notices for upstream `block` and `proc-macro-error2` dependencies.

This validates the independent publication snapshot, not upcoming Vault or four-column features. No public signed/notarized binary is included. Windows and fresh-machine CI results are not claimed.

The source snapshot starts a new Git history to exclude replaced upstream commercial/branding assets from published history. Its upstream base is `s1dashu/quiet-md@52afb284d7132dba1a1efb2944a456aaed9d44f3`; upstream MIT copyright and third-party notices are retained.

GitHub Actions configuration is excluded from this initial publication because the authenticated OAuth grant has no workflow scope. Validation above ran locally; no online CI pass is claimed.
