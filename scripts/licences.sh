#!/bin/sh
# Regenerate the dependency table in THIRD_PARTY.md from cargo metadata.
# Everything between the table header and "## 字体" is replaced; the rest stays.
set -e
cd "$(dirname "$0")/.."
cargo metadata --locked --format-version 1 | python3 -c '
import json, sys, re
m = json.load(sys.stdin)
ws = set(m["workspace_members"])
deps = sorted((p for p in m["packages"] if p["id"] not in ws), key=lambda p: p["name"].lower())
lines = []
for p in deps:
    lic = p.get("license") or "—"
    lines.append("| " + p["name"] + " | " + p["version"] + " | " + lic + " |")
src = open("THIRD_PARTY.md").read()
src = re.sub(r"共 [0-9]+ 个包", "共 " + str(len(deps)) + " 个包", src)
head = src[: src.index("| 包 | 版本 | 许可 |")] + "| 包 | 版本 | 许可 |\n|---|---|---|\n"
tail = src[src.index("## 字体") :]
open("THIRD_PARTY.md", "w").write(head + "\n".join(lines) + "\n\n" + tail)
print("THIRD_PARTY.md:", len(deps), "packages")
'
