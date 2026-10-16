#!/usr/bin/env bash
# Line coverage per crate (cargo-llvm-cov). Gate: 100% per crate, see AGENTS.md "Tests".
#
#   scripts/coverage.sh                    # every crate; rewrites the generated part of docs/COVERAGE.md
#   scripts/coverage.sh punphp-tokens pun-config   # some crates (package or dir name); updates only their rows
#   PUN_FIXERS_ONLY=pun_no_only scripts/coverage.sh # one fixer: its file(s) + the analyzers it touches;
#                                                   # gate is the fixer's own files; docs/COVERAGE.md untouched
#   COVERAGE_VERBOSE=1 ...                 # print every file, not only the ones below 100%
#
# Instrumented builds live in target/coverage (reused between runs, never the shared target/llvm-cov-target),
# serialised with flock so concurrent agents don't mix profraw files. Exclusions: lines `- path:FROM-TO reason`
# under "## Exclusions" in docs/COVERAGE.md are not counted.
# Needs: rustup component add llvm-tools-preview; cargo install cargo-llvm-cov --locked
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
source ~/.cargo/env 2>/dev/null || true
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
export CARGO_LLVM_COV_TARGET_DIR="$ROOT/target/coverage"
OUT="$CARGO_LLVM_COV_TARGET_DIR/results"
mkdir -p "$OUT"
exec 9>"$CARGO_LLVM_COV_TARGET_DIR/.lock"
flock 9

declare -A DIR_OF=()
ALL=()
for toml in crates/*/Cargo.toml; do
  pkg=$(sed -n 's/^name *= *"\(.*\)"/\1/p' "$toml" | head -1)
  dir=$(dirname "$toml")
  DIR_OF[$pkg]=$dir; DIR_OF[$(basename "$dir")]=$dir
  ALL+=("$pkg")
done

ONLY="${PUN_FIXERS_ONLY:-}"
if [ $# -eq 0 ]; then
  if [ -n "$ONLY" ]; then set -- punphp-fixers; else set -- "${ALL[@]}"; fi
fi

status=0
for arg in "$@"; do
  dir="${DIR_OF[$arg]:-}"
  [ -n "$dir" ] || { echo "unknown crate: $arg" >&2; exit 2; }
  pkg=$(sed -n 's/^name *= *"\(.*\)"/\1/p' "$dir/Cargo.toml" | head -1)
  echo "== $pkg ($dir)${ONLY:+ PUN_FIXERS_ONLY=$ONLY}" >&2
  cargo llvm-cov clean --profraw-only
  # `report` maps every test/bin executable under the target dir; a stale one (older source) would add phantom
  # uncovered lines, so drop them all. Only the executables relink; libraries and dependencies are reused.
  find "$CARGO_LLVM_COV_TARGET_DIR/debug" "$CARGO_LLVM_COV_TARGET_DIR/debug/deps" -maxdepth 1 -type f -perm -u+x ! -name '*.*' -delete 2>/dev/null || true
  tests_ok=1
  filter=()
  # One fixer: only its own unit tests + the upstream fixture runner + tests/cases.rs oracle_* tests (which PUN_FIXERS_ONLY already narrows), so
  # "analyzers touched" means touched by this fixer, not by the analyzers' own unit tests.
  if [ -n "$ONLY" ]; then filter=(-- upstream_fixtures oracle_); for n in ${ONLY//,/ }; do filter+=("$n::"); done; fi
  timeout "${COVERAGE_TIMEOUT:-3600}" nice cargo llvm-cov --no-report -p "$pkg" --no-fail-fast "${filter[@]}" >&2 || tests_ok=0
  lcov="$OUT/$pkg${ONLY:+.only}.lcov"
  nice cargo llvm-cov report --lcov --output-path "$lcov" >&2
  python3 - "$ROOT" "$dir" "$pkg" "$lcov" "$tests_ok" "$ONLY" <<'PY' || status=1
import json, os, re, sys
root, cdir, pkg, lcov, tests_ok, only = sys.argv[1:]
verbose = bool(os.environ.get("COVERAGE_VERBOSE"))
doc = os.path.join(root, "docs/COVERAGE.md")
text = open(doc).read() if os.path.exists(doc) else ""

excl = {}
sec = re.search(r"^## Exclusions\n(.*?)(?=^## |\Z)", text, re.M | re.S)
for m in re.finditer(r"^- `?([^\s:`]+):(\d+)(?:-(\d+))?`?", sec.group(1) if sec else "", re.M):
    excl.setdefault(m.group(1), set()).update(range(int(m.group(2)), int(m.group(3) or m.group(2)) + 1))

files, cur = {}, None
for line in open(lcov):
    if line.startswith("SF:"):
        cur = os.path.relpath(line[3:].strip(), root); files.setdefault(cur, {})
    elif line.startswith("DA:") and cur:
        n, c = line[3:].split(",")[:2]
        files[cur][int(n)] = files[cur].get(int(n), 0) + int(c)

def ranges(ns):
    out, ns = [], sorted(ns)
    for n in ns:
        if out and n == out[-1][1] + 1: out[-1][1] = n
        else: out.append([n, n])
    return ", ".join(f"{a}" if a == b else f"{a}-{b}" for a, b in out)

rows = []
for f, da in sorted(files.items()):
    if not f.startswith(cdir + "/src/"): continue
    lines = {n: c for n, c in da.items() if n not in excl.get(f, ())}
    miss = [n for n, c in lines.items() if c == 0]
    rows.append((f, len(lines), len(lines) - len(miss), miss))

if only:
    names = [n.strip() for n in only.split(",") if n.strip()]
    mine = lambda f: any(f == f"{cdir}/src/fixers/{n}.rs" or f.startswith(f"{cdir}/src/fixers/{n}/") for n in names)
    gate = [r for r in rows if mine(r[0])]
    touched = [r for r in rows if r[0].startswith(f"{cdir}/src/analyzers/") and r[2] > 0]
    if not gate: print(f"no files for PUN_FIXERS_ONLY={only}", file=sys.stderr); sys.exit(2)
else:
    gate, touched = rows, []

def pct(t, h): return 100.0 if t == 0 else 100.0 * h / t
def table(rs, title, all_rows, show_missing):
    print(f"\n{title}\n{'file':70} {'lines':>6} {'hit':>6} {'%':>7}")
    for f, t, h, miss in rs:
        if all_rows or h < t:
            print(f"{f:70} {t:6} {h:6} {pct(t, h):6.2f}%" + (f"  missing: {ranges(miss)}" if miss and show_missing else ""))

T, H = sum(r[1] for r in gate), sum(r[2] for r in gate)
table(gate, f"{pkg}{' PUN_FIXERS_ONLY=' + only if only else ''}: {pct(T, H):.2f}% ({H}/{T} lines)", verbose or bool(only), True)
if touched: table(touched, "analyzers touched (informational; not gated here)", True, verbose)
if tests_ok != "1": print(f"\n{pkg}: TESTS FAILED")
ok = tests_ok == "1" and H == T

if not only:
    res = os.path.join(os.path.dirname(lcov), "summary.json")
    summary = json.load(open(res)) if os.path.exists(res) else {}
    summary[pkg] = {"lines": T, "hit": H, "tests_ok": tests_ok == "1",
                    "files": [[f, t, h, ranges(m)] for f, t, h, m in rows if h < t]}
    json.dump(summary, open(res, "w"), indent=1)
    gen = ["<!-- coverage:begin (generated by scripts/coverage.sh; edit outside the markers) -->",
           "| crate | lines | hit | line % | tests |", "|---|---:|---:|---:|---|"]
    for p, s in sorted(summary.items()):
        gen.append(f"| {p} | {s['lines']} | {s['hit']} | {pct(s['lines'], s['hit']):.2f}% | {'ok' if s['tests_ok'] else 'FAILED'} |")
    gen.append("\nUncovered lines:\n")
    any_miss = False
    for p, s in sorted(summary.items()):
        for f, t, h, m in s["files"]:
            any_miss = True
            gen.append(f"- `{f}` {pct(t, h):.2f}% ({t - h} lines): {m}")
    if not any_miss: gen.append("- none")
    gen.append("<!-- coverage:end -->")
    block = "\n".join(gen)
    if "<!-- coverage:begin" in text:
        text = re.sub(r"<!-- coverage:begin.*?<!-- coverage:end -->", lambda _: block, text, flags=re.S)
    else:
        text = "# Coverage\n\n" + block + "\n\n## Exclusions\n\n- none\n" + text
    open(doc, "w").write(text)
sys.exit(0 if ok else 1)
PY
done
exit $status
