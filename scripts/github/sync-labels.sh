#!/bin/sh
# Make the repo labels match .github/labels.yml.
# Usage: sh scripts/github/sync-labels.sh [owner/repo]   (needs gh, authenticated)
set -eu
repo="${1:-ds-kimi/prybar}"
file="$(dirname "$0")/../../.github/labels.yml"
# GitHub defaults we replaced (documentation -> docs) or do not use.
for old in documentation invalid; do
  gh label delete "$old" -R "$repo" --yes 2>/dev/null || true
done
# Each line is a flow mapping: {name: X, color: Y, description: Z}.
grep '^- {' "$file" | sed -E 's/^- \{name: "?([^,"]*)"?, color: ([0-9a-f]{6}), description: (.*)\}$/\1|\2|\3/' |
while IFS='|' read -r name color desc; do
  gh label create "$name" -R "$repo" --color "$color" --description "$desc" --force
done
