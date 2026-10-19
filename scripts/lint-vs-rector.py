#!/usr/bin/env python3
"""`pun lint` against Rector, per file and rule, on corpus projects (context/PLAN.md Track B).

    scripts/lint-vs-rector.py [project...]     # default: pinkary
    PUN_BIN=target/release/pun scripts/lint-vs-rector.py laravel-framework

Uses scripts/bench.py's lint setup: a scratch copy under BENCH_SCRATCH (default target-bench/) with composer's
vendor/, the paths of the project's rector.php, `pun-bench-rector.php` with exactly the Rector classes of pun-lint's
ported rules, and `pun-rector-lint.toml` with the Pun IDs of those classes at `error`. Runs Rector (`--dry-run
--output-format=json`) and `pun lint --format=json -v`, maps each applied Rector class to its Pun ID through rules/mapping.toml, and prints the
(file, rule) pairs only one tool reports, grouped by rule. Exits 1 when they differ.
"""

import importlib.util
import json
import os
import subprocess
import sys
import tomllib
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
spec = importlib.util.spec_from_file_location("bench", ROOT / "scripts/bench.py")
bench = importlib.util.module_from_spec(spec)
spec.loader.exec_module(bench)


def pairs_of_rector(out: str, pun_id: dict[str, str]) -> set[tuple[str, str]]:
    data = json.loads(out[out.index("{"):])
    if data.get("fatal_errors"):
        sys.exit(f"Rector: {data['fatal_errors']}")
    return {(d["file"], pun_id[c]) for d in data.get("file_diffs", []) for c in d["applied_rectors"]}


def pairs_of_pun(out: str) -> set[tuple[str, str]]:
    return {(f["name"], r) for f in json.loads(out)["files"] for r in f.get("appliedFixers", [])}


def compare(name: str, pun: str, env: dict) -> bool:
    work = bench.setup(name)
    classes = bench.ported_rector_rules()
    bench.write_configs("lint", name, work, classes)
    mapping = tomllib.loads((ROOT / "crates/pun-config/rules/mapping.toml").read_text())
    pun_id = {e["class"]: e["pun"] for t, rules in mapping["upstream"].items() if "rector" in t
              for e in rules.values() if isinstance(e, dict) and "class" in e and "pun" in e}
    rector = subprocess.run(["php", str(bench.RECTOR), "process", "--dry-run", "--clear-cache", "--no-progress-bar",
                             "--output-format=json", "--memory-limit=-1", "--config", "pun-bench-rector.php"],
                            cwd=work, env=env, capture_output=True, text=True)
    # Only the rules with a Rector class: the Mago-ported ones have nothing to compare with.
    ids = sorted(set(pun_id[c] for c in classes))
    (work / "pun-rector-lint.toml").write_text(
        "[lint.rules]\n" + "".join(f'"{i.removeprefix("lint/")}" = "error"\n' for i in ids))
    ours = subprocess.run([pun, "lint", "--config", "pun-rector-lint.toml", "--format=json", "-v", "--no-cache",
                           *bench.LINT[name]], cwd=work, env=env, capture_output=True, text=True)
    if ours.returncode not in (0, 1):
        sys.exit(f"pun lint exited {ours.returncode}: {ours.stderr.strip()[-500:]}")
    theirs, mine = pairs_of_rector(rector.stdout, pun_id), pairs_of_pun(ours.stdout)
    by_rule = defaultdict(list)
    for f, r in sorted(theirs - mine):
        by_rule[r].append(f"  rector only  {f}")
    for f, r in sorted(mine - theirs):
        by_rule[r].append(f"  pun only     {f}")
    print(f"{name}: {len(ids)} rules, {len(theirs & mine)} (file, rule) pairs agree, {len(theirs - mine)} Rector only, "
          f"{len(mine - theirs)} Pun only")
    for r in sorted(by_rule):
        print(r)
        print("\n".join(by_rule[r]))
    return not by_rule


def main() -> None:
    pun = os.environ.get("PUN_BIN", str(ROOT / "target/debug/pun"))
    env = {k: v for k, v in os.environ.items() if k not in bench.AGENT_ENV}
    env.update({"PUN_AGENT": "0", "PUN_IGNORE_PHP_CONSTRAINT": "1", "XDEBUG_MODE": "off"})
    same = [compare(name, pun, env) for name in sys.argv[1:] or ["pinkary"]]
    sys.exit(0 if all(same) else 1)


if __name__ == "__main__":
    main()
