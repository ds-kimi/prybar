#!/usr/bin/env sh
# Runs check-limits on throwaway trees so the exemption marker keeps
# meaning exactly one limit, with a reason, and nothing more.
set -u
here="$(cd "$(dirname "$0")" && pwd)"
fail=0

run() { # name, expected status, file body
    d=$(mktemp -d)
    mkdir -p "$d/crates/x" "$d/scripts/check-limits"
    cp "$here/check-limits.sh" "$d/scripts/check-limits/"
    printf '%b' "$3" > "$d/crates/x/a.rs"
    sh "$d/scripts/check-limits/check-limits.sh" >/dev/null 2>&1
    got=$?
    rm -rf "$d"
    if [ "$got" -ne "$2" ]; then echo "FAIL $1: status $got, want $2"; fail=1; else echo "ok $1"; fi
}

five='fn a() {}\nfn b() {}\nfn c() {}\nfn d() {}\nfn e() {}\n'
run "five fns fail" 1 "$five"
run "marker lifts fns" 0 "// limits: allow fns -- trait impl\n$five"
run "marker needs a reason" 1 "// limits: allow fns\n$five"
run "marker lifts only its limit" 1 "// limits: allow lines -- long match\n$five"
exit $fail
