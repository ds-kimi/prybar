#!/usr/bin/env sh
# Enforces the repo rules that rustfmt and clippy cannot:
# at most 4 fns per .rs file, at most 30 lines per fn, no trailing spaces.
set -u
root="$(cd "$(dirname "$0")/../.." && pwd)"
status=0

for f in $(find "$root/crates" "$root/services" "$root/tests" "$root/fuzz/fuzz_targets" "$root/fuzz/examples" -name '*.rs' -not -path '*/target/*' 2>/dev/null); do
    awk -v file="$f" '
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
                if (len > 30) { printf "%s:%d: fn is %d lines (max 30)\n", file, start, len; bad = 1 }
                infn = 0
            } else if (!opened && line ~ /;[[:space:]]*$/) { infn = 0 }
        }
        END {
            if (fns > 4) { printf "%s: %d fns (max 4)\n", file, fns; bad = 1 }
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
    if [ "$n" -gt 10 ]; then
        echo "$d: $n .rs files (max 10)"
        status=1
    fi
done

exit $status
