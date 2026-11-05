#!/bin/sh
# Before every commit: tests, bundled guides and clippy.
# Longer fuzzing: FDL_FUZZ_SEEDS=2000 scripts/check.sh
set -e
cd "$(dirname "$0")/.."
exec python3 scripts/check.py
