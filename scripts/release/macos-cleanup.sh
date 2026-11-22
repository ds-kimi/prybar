#!/bin/sh
# Removes the temp keychain and decoded secrets; safe to run when nothing exists.
# Assumes a GitHub-hosted runner, thrown away after the job, so the keychain
# search list is not restored.
tmp="${RUNNER_TEMP:-/tmp}/prybar-sign"
if [ -f "$tmp/sign.keychain-db" ]; then
  security delete-keychain "$tmp/sign.keychain-db" 2>/dev/null || true
fi
rm -rf "$tmp"
