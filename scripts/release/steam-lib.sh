#!/bin/sh
# Copies the Steam API redistributable that steamworks-sys linked against
# next to the binaries, where the loader looks for it (@loader_path on
# macOS, $ORIGIN rpath on Linux, the exe folder on Windows).
# Usage: steam-lib.sh <target> <dest dir>...
set -eu
target="$1"
shift
lib=$(find "target/$target/release/build" -path '*steamworks-sys-*/out/*' \
  \( -name libsteam_api.dylib -o -name libsteam_api.so -o -name steam_api64.dll \) | head -n 1)
if [ -z "$lib" ]; then
  echo "steam-lib: no Steam API library under target/$target/release/build" >&2
  exit 1
fi
for dest in "$@"; do
  cp "$lib" "$dest/"
done
