#!/bin/zsh
# Retake the 开始指南 screenshots: the app on the desktop picture, with the
# system's corners and shadow and 48pt of wallpaper around it. The window
# floats in the middle of the main screen for about a minute (it never takes
# focus); a borderless wallpaper window behind it hides whatever else is open.
# usage: shared-mock/guides/shots/shots.sh   (after cargo build --release)
set -e
cd "$(dirname "$0")/../../.."
work=$(mktemp -d /tmp/quiet-shots.XXXXXX)
python3 shared-mock/guides/build.py "$work/lib" >/dev/null
# seen_version: no 已更新到 card over the sidebar.
ver=$(sed -n 's/^version = "\(.*\)"$/\1/p' Cargo.toml | head -1)
printf '{"english":true,"theme":"feishu","feishu":true,"seen_version":"%s"}\n' "$ver" > "$work/lib/preferences.json"
printf '{"sidebar_width":240}\n' > "$work/lib/settings.json"
sed 's/^SHOT \(.*\)$/deskshot \1 48/' shared-mock/guides/shots/steps.txt > "$work/steps.txt"
swiftc -O shared-mock/guides/backdrop.swift -o "$work/backdrop" 2>/dev/null
wall=$(swift -e 'import AppKit; print(NSWorkspace.shared.desktopImageURL(for: NSScreen.screens[0])!.path)' 2>/dev/null)
sips -s format png "$wall" --out "$work/wall.png" >/dev/null
# Covers both window sizes in steps.txt plus their margins.
"$work/backdrop" 78 56 1376 896 180 "$work/wall.png" &
bd=$!
sleep 1.5
FDL_NOPARK=1 python3 tools/harness/run.py "$work/steps.txt" "$work/lib" "$work/run" >/dev/null 2>&1 || true
kill $bd 2>/dev/null || true
for n in home toolbar slash; do
  [ -f "$work/run/$n.png" ] || { echo "shots.sh: $n.png missing (see $work/run/log)" >&2; exit 1; }
  sips -Z 1600 -s format jpeg -s formatOptions 80 "$work/run/$n.png" --out "shared-mock/guides/assets/$n.jpg" >/dev/null
done
for n in home editor; do
  [ -f "$work/run/$n.png" ] || { echo "shots.sh: $n.png missing" >&2; exit 1; }
  sips -s format jpeg -s formatOptions 85 "$work/run/$n.png" --out "docs/images/$n.jpg" >/dev/null
done
echo "$work"
