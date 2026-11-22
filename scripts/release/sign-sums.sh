#!/bin/sh
# Signs SHA256SUMS with the minisign update key so updaters can trust it.
# Key and password come only from step-level env (GitHub secrets); the key
# file is written owner-only under RUNNER_TEMP and removed by
# sign-sums-cleanup.sh even when this step fails.
# Usage: sign-sums.sh <SHA256SUMS>
set -eu
sums="$1"
umask 077
tmp="${RUNNER_TEMP:-/tmp}/prybar-update-sign"
mkdir -p "$tmp"
printf '%s\n' "$UPDATE_SIGNING_KEY" > "$tmp/update.key"
# minisign reads the password from stdin when it is not a terminal; an
# unencrypted key (minisign -G -W) needs none.
printf '%s\n' "${UPDATE_SIGNING_KEY_PASSWORD:-}" |
  minisign -S -s "$tmp/update.key" -m "$sums" -x "$sums.minisig" \
    -t "prybar release $(basename "$sums")"
rm -f "$tmp/update.key"
