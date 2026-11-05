#!/usr/bin/env python3
"""Shared macOS and Windows checks; run inside the platform build environment."""
import os
from pathlib import Path
import subprocess

os.chdir(Path(__file__).resolve().parent.parent)
def cargo(*args, env=None):
    subprocess.run(["cargo", *args], check=True, env=env)

cargo("test", "--release", "--workspace", "--locked")
cargo("test", "--release", "--locked", "-p", "fdl_core", "library_round_trips", "--", "--ignored",
      env={**os.environ, "FDL_ROUNDTRIP_DIR": str(Path("shared-mock/library/docs").resolve())})
source = Path("shared-mock/guides/assets")
bundled = Path("shared-mock/library/assets/guides")
for image in source.rglob("*"):
    if image.is_file():
        target = bundled / image.relative_to(source)
        assert target.read_bytes() == image.read_bytes(), f"Bundled guide asset is stale: {target}"
print("Bundled guide assets match their sources")
cargo("clippy", "--release", "--workspace", "--locked", "--all-targets", "--", "-D", "warnings")
