#!/bin/sh
# Maintainer view of the analytics, including crash reports, as JSON.
# The token never sits in the repo: it comes from PRYBAR_TELEMETRY_TOKEN or
# the macOS Keychain item "prybar-telemetry-token".
# Usage: sh scripts/telemetry/stats.sh [summary|daily|crashes|all]  (default: all)
set -eu
url="${PRYBAR_TELEMETRY_API:-https://prybar-telemetry.progamerkimi.workers.dev}"
tok="${PRYBAR_TELEMETRY_TOKEN:-}"
if [ -z "$tok" ] && command -v security >/dev/null; then
  tok=$(security find-generic-password -a prybar -s prybar-telemetry-token -w 2>/dev/null || true)
fi
[ -n "$tok" ] || { echo "no token: set PRYBAR_TELEMETRY_TOKEN" >&2; exit 2; }
json=$(curl -fsS -H "Authorization: Bearer $tok" "$url/private.json")
part="${1:-all}"
if [ "$part" = all ]; then printf '%s\n' "$json"
else printf '%s' "$json" | python3 -I -c "import json,sys;print(json.dumps(json.load(sys.stdin)['$part'],indent=1))"; fi
