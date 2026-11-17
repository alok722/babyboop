#!/bin/zsh
set -euo pipefail
cd "$(dirname "$0")/.."
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
swift scripts/make_icon.swift "$TMP/icon_1024.png"

SET="$TMP/AppIcon.iconset"
mkdir -p "$SET"
for s in 16 32 128 256 512; do
  sips -z $s $s "$TMP/icon_1024.png" --out "$SET/icon_${s}x${s}.png" >/dev/null
  sips -z $((s * 2)) $((s * 2)) "$TMP/icon_1024.png" --out "$SET/icon_${s}x${s}@2x.png" >/dev/null
done
iconutil -c icns "$SET" -o apps/macos/Resources/AppIcon.icns

for s in 16 32 48 256; do sips -z $s $s "$TMP/icon_1024.png" --out "$TMP/ico_$s.png" >/dev/null; done
mkdir -p apps/windows/BabyBoop/Assets apps/windows-rs/assets
python3 -I - "$TMP" apps/windows/BabyBoop/Assets/AppIcon.ico <<'EOF'
import struct, sys
tmp, out = sys.argv[1], sys.argv[2]
sizes = [16, 32, 48, 256]
pngs = [open(f"{tmp}/ico_{s}.png", "rb").read() for s in sizes]
offset = 6 + 16 * len(sizes)
head = struct.pack("<HHH", 0, 1, len(sizes))
entries = b""
for s, data in zip(sizes, pngs):
    entries += struct.pack("<BBBBHHII", s % 256, s % 256, 0, 0, 1, 32, len(data), offset)
    offset += len(data)
open(out, "wb").write(head + entries + b"".join(pngs))
EOF
cp apps/windows/BabyBoop/Assets/AppIcon.ico apps/windows-rs/assets/AppIcon.ico

mkdir -p site/assets
sips -z 512 512 "$TMP/icon_1024.png" --out site/assets/icon-512.png >/dev/null
echo "icons written"
