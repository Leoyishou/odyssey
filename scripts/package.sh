#!/bin/zsh
# Build, sign, notarize and package Quiet.md as a .dmg for distribution.
#
#   scripts/package.sh [version]
#
# Needs a "Developer ID Application" certificate in the keychain and a
# notarytool keychain profile. Create the profile once:
#
#   xcrun notarytool store-credentials quiet-md \
#     --apple-id <你的 Apple ID> --team-id <Team ID> --password <App 专用密码>
#
# Environment:
#   CODESIGN_ID    证书名，默认自动取钥匙串里的第一个 "Developer ID Application"
#   NOTARY_PROFILE notarytool 的 profile 名，默认 quiet-md
#   BUNDLE_ID      bundle identifier，默认 app.quietmd.mac（发布前改成你自己的反向域名）
set -e
cd "$(dirname "$0")/.."
version=${1:-$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)}
bundle_id=${BUNDLE_ID:-app.quietmd.mac}
profile=${NOTARY_PROFILE:-quiet-md}
sign_id=${CODESIGN_ID:-$(security find-identity -v -p codesigning | sed -n 's/.*"\(Developer ID Application:.*\)"/\1/p' | head -1)}
[ -n "$sign_id" ] || {
  echo "package.sh: 钥匙串里没有 Developer ID Application 证书。" >&2
  echo "  Apple Developer Program 会员可以在 developer.apple.com 申请，导入后用 security find-identity 确认。" >&2
  exit 1
}

out=dist
app="$out/quiet.md.app"
dmg="$out/quiet.md-$version.dmg"

echo "== 编译 $version"
cargo build --release --locked

echo "== 组装 $app"
rm -rf "$out"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources"
cp target/release/quiet_md "$app/Contents/MacOS/quiet_md"
# .icns：Finder 和 Dock 用图标文件，不是裸 PNG。
iconset=$(mktemp -d)/icon.iconset
mkdir -p "$iconset"
for s in 16 32 128 256 512; do
  sips -z $s $s assets/branding/app-icon.png --out "$iconset/icon_${s}x${s}.png" >/dev/null
  sips -z $((s * 2)) $((s * 2)) assets/branding/app-icon.png --out "$iconset/icon_${s}x${s}@2x.png" >/dev/null
done
iconutil -c icns "$iconset" -o "$app/Contents/Resources/app-icon.icns"
cat > "$app/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
  <key>CFBundleName</key><string>Quiet.md</string>
  <key>CFBundleDisplayName</key><string>Quiet.md</string>
  <key>CFBundleIdentifier</key><string>$bundle_id</string>
  <key>CFBundleExecutable</key><string>quiet_md</string>
  <key>CFBundleIconFile</key><string>app-icon</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleShortVersionString</key><string>$version</string>
  <key>CFBundleVersion</key><string>$version</string>
  <key>NSHighResolutionCapable</key><true/>
  <key>LSMinimumSystemVersion</key><string>13.0</string>
  <key>NSDocumentsFolderUsageDescription</key><string>Quiet.md 把文档库放在「文稿」里的 Quiet.md 文件夹。</string>
</dict></plist>
PLIST

echo "== 签名（hardened runtime + 时间戳）"
codesign --force --options runtime --timestamp --sign "$sign_id" "$app"
codesign --verify --strict --verbose=2 "$app"

echo "== 打包 $dmg"
rm -rf "$out/dmg-root"
mkdir -p "$out/dmg-root"
cp -R "$app" "$out/dmg-root/"
ln -s /Applications "$out/dmg-root/Applications"
hdiutil create -quiet -volname "Quiet.md" -srcfolder "$out/dmg-root" -ov -format UDZO "$dmg"
rm -rf "$out/dmg-root"
codesign --force --timestamp --sign "$sign_id" "$dmg"

echo "== 公证（需要网络，通常几分钟）"
xcrun notarytool submit "$dmg" --keychain-profile "$profile" --wait
xcrun stapler staple "$dmg"
xcrun stapler validate "$dmg"

echo "== 验收"
spctl -a -vvv -t install "$dmg" || true
codesign -dv --verbose=4 "$app" 2>&1 | grep -E "Identifier|Authority|TeamIdentifier|flags"
echo
echo "好了：$dmg（$(du -h "$dmg" | cut -f1)）"
