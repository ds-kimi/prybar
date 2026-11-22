#!/bin/sh
# Removes the temp keychain and decoded secrets; safe to run when nothing exists.
tmp="${RUNNER_TEMP:-/tmp}/prybar-sign"
if [ -f "$tmp/sign.keychain-db" ]; then
  security delete-keychain "$tmp/sign.keychain-db" 2>/dev/null || true
fi
rm -rf "$tmp"
