#!/bin/sh
# Decides whether to sign from six "true"/"false" flags, one per secret.
# None: unsigned release with a notice. Some but not all: fail, because a
# half-configured setup would otherwise ship unsigned builds silently.
set -eu
count=0
for flag in "$@"; do
  if [ "$flag" = "true" ]; then count=$((count + 1)); fi
done
if [ "$count" = "6" ]; then
  echo "enabled=true" >> "$GITHUB_OUTPUT"
elif [ "$count" = "0" ]; then
  echo "::notice::macOS signing secrets not set, shipping unsigned macOS builds"
  echo "enabled=false" >> "$GITHUB_OUTPUT"
else
  echo "::error::only $count of 6 macOS signing secrets are set, see docs/release/macos-signing.md"
  exit 1
fi
