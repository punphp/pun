#!/usr/bin/env bash
# Runs every punphp-analyse test target and its unit tests: builds them once, then runs the binaries in parallel, because
# `cargo test` runs its 170-odd test binaries one after another.
#
#   scripts/analyse-tests.sh            every target, `levels` and `rules` included
#   SKIP="levels rules" scripts/analyse-tests.sh
#
# JOBS (default 6) binaries run at once. Logs go to $OUT (default target/analyse-tests/), one per target.
# Exits 1 when a target fails and lists the failing targets.
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
out=${OUT:-$root/target/analyse-tests}
jobs=${JOBS:-6}
skip=" ${SKIP:-} "
export PUN_AGENT=0

cd "$root/crates/pun-analyse"
targets=()
for f in tests/*.rs; do
    name=$(basename "$f" .rs)
    [[ $skip == *" $name "* ]] || targets+=("$name")
done
args=(--lib)
for t in "${targets[@]}"; do args+=(--test "$t"); done

# The build prints each binary's path as a compiler-artifact message.
cargo test -p punphp-analyse "${args[@]}" --no-run --message-format=json \
    | python3 -c '
import json, sys
for line in sys.stdin:
    m = json.loads(line)
    if m.get("reason") == "compiler-artifact" and m.get("executable") and m["profile"]["test"]:
        print(m["target"]["name"], m["executable"])
' > "$out.list.tmp" || exit 2

rm -rf "$out"
mkdir -p "$out"
# The two slowest targets start first, so they overlap the rest.
{ grep -E '^(levels|rules) ' "$out.list.tmp" || true; grep -vE '^(levels|rules) ' "$out.list.tmp"; } > "$out/list"
rm "$out.list.tmp"
xargs -P "$jobs" -L 1 sh -c '"$1" > "$2/$0.log" 2>&1; echo "$0 $?" >> "$2/results"' < <(sed "s|\$| $out|" "$out/list")

failed=$(awk '$2 != 0 { print $1 }' "$out/results" | sort)
echo "$(wc -l < "$out/results") targets, $(printf '%s' "$failed" | grep -c . || true) failed"
if [ -n "$failed" ]; then
    echo "$failed"
    exit 1
fi
