#!/bin/sh
# Builds Prybar.app around an already built prybar-gui binary.
# Usage: macos-bundle.sh <prybar-gui binary> <version> <out dir>
set -eu
bin="$1"; version="$2"; out="$3"
here="$(cd "$(dirname "$0")/../.." && pwd)"
app="$out/Prybar.app"
rm -rf "$app"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources"
cp "$bin" "$app/Contents/MacOS/prybar-gui"
# Tags look like v1.2.3; the bundle wants the bare number.
sed "s/__VERSION__/${version#v}/g" "$here/crates/prybar-gui/bundle/Info.plist" > "$app/Contents/Info.plist"
plutil -lint "$app/Contents/Info.plist" >/dev/null
echo "$app"
