#!/bin/zsh
set -euo pipefail
cd "$(dirname "$0")"
MODE="${1:-install}"
case "$MODE" in
  install|--install|--no-install|--dmg) ;;
  *) echo "usage: build.sh [install|--no-install|--dmg]" >&2; exit 2 ;;
esac
APP=build/BabyBoop.app
rm -rf build && mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
for arch in arm64 x86_64; do
  mkdir -p "build/$arch"
  swiftc -O -target "$arch-apple-macos13" src/*.swift -o "build/$arch/BabyBoop"
done
lipo -create build/arm64/BabyBoop build/x86_64/BabyBoop -output "$APP/Contents/MacOS/BabyBoop"
cp Resources/Info.plist "$APP/Contents/Info.plist"
if [[ -n "${BABYBOOP_VERSION:-}" ]]; then
  # macOS wants plain numbers here, so 0.2.0-beta.1 is stamped as 0.2.0
  v="${BABYBOOP_VERSION%%-*}"
  [[ "$v" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || { echo "bad BABYBOOP_VERSION: $BABYBOOP_VERSION" >&2; exit 2; }
  plutil -replace CFBundleShortVersionString -string "$v" "$APP/Contents/Info.plist"
  plutil -replace CFBundleVersion -string "$v" "$APP/Contents/Info.plist"
fi
cp ../../web/index.html Resources/AppIcon.icns "$APP/Contents/Resources/"
codesign --force --sign - --identifier app.babyboop "$APP"

case "$MODE" in
  --no-install) echo "built $APP" ;;
  --dmg)
    mkdir -p build/dmg && cp -R "$APP" build/dmg/ && ln -s /Applications build/dmg/Applications
    hdiutil create -volname BabyBoop -srcfolder build/dmg -ov -format UDZO build/BabyBoop-macOS.dmg
    echo "built build/BabyBoop-macOS.dmg" ;;
  install|--install)
    mkdir -p ~/Applications && rm -rf ~/Applications/BabyBoop.app && mv "$APP" ~/Applications/ && rm -rf build
    echo "installed ~/Applications/BabyBoop.app" ;;
esac
