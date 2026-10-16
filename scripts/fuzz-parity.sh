#!/usr/bin/env bash
# Mutation fuzzing: copy (part of) a project, apply style mutations (scripts/mutate.py, deterministic per seed),
# format one copy with real Pint and one with `pun format`, and compare every file. Each difference is
# minimised (scripts/mutate.py minimise) and printed. Exit 1 when any file differs.
#
#   scripts/fuzz-parity.sh <project-dir> <seed> [path...]      # paths relative to the project (default: all)
#   scripts/fuzz-parity.sh corpus/pinkary 7 app tests
#   scripts/fuzz-parity.sh corpus/laravel-framework 3 src/Illuminate/Support
#
# Needs php 8.5, the pinned Pint in .oracle/pint (scripts/oracle.sh) and a built target/debug/pun (or PUN_BIN=<pun>)
# newer than the formatting crates' sources (pun-tokens, -format, -fixers, -config).
# A project whose pint.json enables Pint/laravel_blade needs its node_modules (prettier); it is symlinked.
# FUZZ_EXTRA=crlf (comma list) applies those mutate.py mutations to every file, after its 3 random ones.
# FUZZ_CONFIG=<pint.json> replaces the project's config. PUN_PINT=<Pint PHAR> is passed to pun for rules it delegates.
# composer.json is not copied: Pint ignores it, and Pun would refuse projects allowing PHP < 8.4.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
project="$(cd "$1" && pwd)"; seed="$2"; shift 2
paths=("$@"); [ ${#paths[@]} -eq 0 ] && paths=(.)
export PUN_BIN="${PUN_BIN:-$ROOT/target/debug/pun}"  # also read by mutate.py minimise
# Each agent builds into its own CARGO_TARGET_DIR, so target/debug/pun goes stale: a binary older than the sources
# reports differences the code no longer has.
stale=$(cd "$ROOT/crates" && find pun-tokens pun-format pun-fixers pun-config -name '*.rs' -newer "$PUN_BIN" -print -quit)
[ -z "$stale" ] || { echo "$PUN_BIN is older than $stale: rebuild it or set PUN_BIN" >&2; exit 2; }
w="${FUZZ_DIR:-/tmp/pun-fuzz}/$(basename "$project")-$seed"
rm -rf "$w"; mkdir -p "$w/src"
for f in pint.json package.json; do [ -f "$project/$f" ] && cp "$project/$f" "$w/src/"; done
[ -n "${FUZZ_CONFIG:-}" ] && cp "$FUZZ_CONFIG" "$w/src/pint.json"
for p in "${paths[@]}"; do mkdir -p "$w/src/$(dirname "$p")"; cp -r "$project/$p" "$w/src/$p"; done
find "$w/src" \( -name vendor -o -name node_modules \) -prune -exec rm -rf {} +
echo "seed $seed: $(python3 "$ROOT/scripts/mutate.py" mutate "$w/src" "$seed")"
[ -d "$project/node_modules" ] && ln -s "$project/node_modules" "$w/src/node_modules"
cp -a "$w/src" "$w/pint"; cp -a "$w/src" "$w/pun"
run() { (cd "$1" && env -i PATH="$PATH" HOME="$HOME" ${PUN_PINT:+PUN_PINT="$PUN_PINT"} timeout 1800 nice "${@:2}" >/dev/null 2>&1 || true); }
run "$w/pint" php "$ROOT/.oracle/pint/builds/pint"
run "$w/pun" "$PUN_BIN" format
files=$(find "$w/src" -name '*.php' -not -path '*/node_modules/*' | wc -l)
changed=$(diff -rq --exclude=node_modules "$w/src" "$w/pint" | wc -l || true)
mapfile -t differ < <(diff -rq --exclude=node_modules "$w/pint" "$w/pun" | sed -n 's/^Files .*\/pint\/\(.*\) and .* differ$/\1/p')
echo "seed $seed: files=$files changed-by-pint=$changed differ=${#differ[@]}"
for f in "${differ[@]}"; do
  echo "=== $f"
  diff "$w/pint/$f" "$w/pun/$f" | head -20 || true
  [ "${FUZZ_MINIMISE:-1}" = 1 ] && python3 "$ROOT/scripts/mutate.py" minimise "$w/src/$f" "$w/src"
done
[ ${#differ[@]} -eq 0 ]
