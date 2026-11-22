#!/usr/bin/env sh
# Enforces the repo rules that rustfmt and clippy cannot:
# at most 4 fns per .rs file, at most 30 lines per fn, no trailing spaces.
# A file may lift one limit with `// limits: allow <fns|lines|files> -- <reason>`
# in its first 5 lines; every exemption is printed so none goes unnoticed.
set -u
root="$(cd "$(dirname "$0")/../.." && pwd)"
status=0

for f in $(find "$root/crates" "$root/services" "$root/tests" "$root/fuzz/fuzz_targets" "$root/fuzz/examples" -name '*.rs' -not -path '*/target/*' 2>/dev/null); do
    awk -v file="$f" '
        # The marker must name a known limit and give a reason after `--`.
        NR <= 5 && /\/\/ limits: allow/ {
            if (match($0, /allow (fns|lines|files) -- [^[:space:]]/)) {
                kind = substr($0, RSTART + 6, RLENGTH - 6); sub(/ .*/, "", kind)
                allow[kind] = 1
                printf "exempt %s: %s\n", kind, file > "/dev/stderr"
            } else { printf "%s:%d: bad limits marker (need allow <fns|lines|files> -- <reason>)\n", file, NR; bad = 1 }
        }
        # Brace depth is tracked per fn so nested blocks and closures count
        # toward the enclosing fn, which is what the 30-line rule means.
        /^[[:space:]]*(pub(\([^)]*\))? )?(const )?(async )?(unsafe )?fn [A-Za-z0-9_]+/ {
            fns++
            if (!infn) { infn = 1; start = NR; depth = 0; opened = 0 }
        }
        infn {
            line = $0
            gsub(/"([^"\\]|\\.)*"/, "", line)
            n = gsub(/\{/, "{", line); depth += n; if (n) opened = 1
            depth -= gsub(/\}/, "}", line)
            if (opened && depth <= 0) {
                len = NR - start + 1
                if (len > 30 && !allow["lines"]) { printf "%s:%d: fn is %d lines (max 30)\n", file, start, len; bad = 1 }
                infn = 0
            } else if (!opened && line ~ /;[[:space:]]*$/) { infn = 0 }
        }
        END {
            if (fns > 4 && !allow["fns"]) { printf "%s: %d fns (max 4)\n", file, fns; bad = 1 }
            exit bad
        }
    ' "$f" || status=1
done

if grep -rnI --exclude-dir=target --exclude-dir=.git -E '[[:space:]]+$' \
    "$root/crates" "$root/services" "$root/scripts" "$root/fuzz/fuzz_targets" "$root/fuzz/examples" "$root/.github" "$root/Cargo.toml" "$root/README.md" "$root/CONTRIBUTING.md" 2>/dev/null; then
    echo "trailing whitespace found"
    status=1
fi

# Flat folders hide structure; past 10 files a folder must be split by domain.
for d in $(find "$root/crates" "$root/services" "$root/tests" -type d -not -path '*/target*' 2>/dev/null); do
    n=$(find "$d" -maxdepth 1 -name '*.rs' | wc -l)
    if [ "$n" -gt 10 ] && find "$d" -maxdepth 1 -name '*.rs' -exec head -n 5 {} + | grep -qE '// limits: allow files -- [^[:space:]]'; then
        echo "exempt files: $d" >&2
    elif [ "$n" -gt 10 ]; then
        echo "$d: $n .rs files (max 10)"
        status=1
    fi
done

exit $status
