#!/usr/bin/env sh
# Round-trip harness over the local Garry's Mod install.
# Usage: scripts/roundtrip/roundtrip.sh [models per package] [report dir]
set -eu
root="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$root"
cargo run --release -q -p prybar-roundtrip -- "${1:-25}" "${2:-target/roundtrip}"
