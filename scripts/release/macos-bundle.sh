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
# Apple wants up to three dot-separated integers; anything else (rc tags,
# slashes) would break the plist or the sed below, so refuse it.
number="${version#v}"
if ! printf '%s' "$number" | grep -Eq '^[0-9]+(\.[0-9]+){0,2}$'; then
  echo "bundle version must look like v1.2.3, got: $version" >&2
  exit 1
fi
sed "s/__VERSION__/$number/g" "$here/crates/prybar-gui/bundle/Info.plist" > "$app/Contents/Info.plist"
plutil -lint "$app/Contents/Info.plist" >/dev/null
echo "$app"
