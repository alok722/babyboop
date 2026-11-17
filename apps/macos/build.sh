#!/bin/zsh
set -euo pipefail
cd "$(dirname "$0")"
MODE="${1:-install}"
APP=build/BabyBoop.app
rm -rf build && mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
for arch in arm64 x86_64; do
  mkdir -p "build/$arch"
  swiftc -O -target "$arch-apple-macos13" src/*.swift -o "build/$arch/BabyBoop"
done
lipo -create build/arm64/BabyBoop build/x86_64/BabyBoop -output "$APP/Contents/MacOS/BabyBoop"
cp Resources/Info.plist "$APP/Contents/Info.plist"
cp ../../web/index.html Resources/AppIcon.icns "$APP/Contents/Resources/"
codesign --force --sign - --identifier app.babyboop "$APP"

case "$MODE" in
  --no-install) echo "built $APP" ;;
  --dmg)
    mkdir -p build/dmg && cp -R "$APP" build/dmg/ && ln -s /Applications build/dmg/Applications
    hdiutil create -volname BabyBoop -srcfolder build/dmg -ov -format UDZO build/BabyBoop-macOS.dmg
    echo "built build/BabyBoop-macOS.dmg" ;;
  *)
    mkdir -p ~/Applications && rm -rf ~/Applications/BabyBoop.app && mv "$APP" ~/Applications/ && rm -rf build
    echo "installed ~/Applications/BabyBoop.app" ;;
esac
