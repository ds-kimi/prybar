# Windows entry point for the round-trip harness.
# Usage: scripts/roundtrip/roundtrip.ps1 [models per package] [report dir]
param([int]$Count = 25, [string]$Out = 'target/roundtrip')
Set-Location (Join-Path $PSScriptRoot '../..')
cargo run --release -q -p prybar-roundtrip -- $Count $Out
exit $LASTEXITCODE
