# Windows entry point: the rules live in check-limits.sh, run through Git Bash.
$script = Join-Path $PSScriptRoot 'check-limits.sh'
& bash $script
exit $LASTEXITCODE
