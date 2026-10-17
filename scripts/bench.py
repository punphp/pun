#!/usr/bin/env python3
"""Time and peak memory of Pint (parallel and serial), Pun and Mago's formatter on corpus projects.

    scripts/bench.py                              # every project, every tool
    scripts/bench.py pinkary laravel-framework    # some projects
    RUNS=5 WARMUP=0 scripts/bench.py              # measured runs per tool (default 3), untimed warm-ups (default 1)
    TOOLS=pun,mago scripts/bench.py               # some tools: pint-parallel, pint, pun, mago
    PUN_BIN=target/release/pun MAGO_BIN=mago scripts/bench.py

Setup: scripts/oracle.sh (or its Pint part) for .oracle/pint; scripts/corpus.sh for corpus/;
`cargo build --profile profiling -p punphp` for target/profiling/pun; the Mago 1.57.0 release binary (the version
Pun's mago crates pin) under .oracle/mago/ or MAGO_BIN. Every tool is check-only and nothing writes to the corpus.

- pint-parallel: `pint --test --parallel` in corpus/<name>, with the project's pint.json.
- pint: `pint --test` (one process), same place.
- pun: `pun check --test --no-cache` in a scratch copy (no .git, node_modules symlinked) after `pun migrate --yes`.
- mago: `mago fmt --check .` in the same scratch copy, default config. Mago's style is not Pint's: a speed and memory
  reference, not a parity check. Without a path and without mago.toml it checks nothing.

Memory is the peak summed RSS of the command's whole process tree (PHP, Node and worker processes included), sampled
every 20 ms, and never below the command's own maxrss (wait4). Exit codes are ignored (1 = files would change).
Prints a per-run table, then medians.
"""

import os
import shutil
import statistics
import subprocess
import sys
import tempfile
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PROJECTS = ["pinkary", "pinkary-dirty", "laravel-framework", "laravel-laravel", "laravel-pint", "pest", "larastan"]
TOOLS = ["pint-parallel", "pint", "pun", "mago"]


def tree_rss_bytes(root: int) -> int:
    out = subprocess.run(["ps", "-A", "-o", "pid=,ppid=,rss="], capture_output=True, text=True).stdout
    kids: dict[int, list[int]] = {}
    rss: dict[int, int] = {}
    for line in out.splitlines():
        pid, ppid, kb = map(int, line.split())
        kids.setdefault(ppid, []).append(pid)
        rss[pid] = kb
    total, stack = 0, [root]
    while stack:
        p = stack.pop()
        total += rss.get(p, 0)
        stack.extend(kids.get(p, []))
    return total * 1024


def run_once(cmd: list[str], cwd: Path) -> tuple[float, int]:
    start = time.perf_counter()
    p = subprocess.Popen(cmd, cwd=cwd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    peak = 0
    while True:
        pid, _, usage = os.wait4(p.pid, os.WNOHANG)
        if pid:
            break
        peak = max(peak, tree_rss_bytes(p.pid))
        time.sleep(0.02)
    elapsed = time.perf_counter() - start
    p.returncode = 0
    maxrss = usage.ru_maxrss if sys.platform == "darwin" else usage.ru_maxrss * 1024
    return elapsed, max(peak, maxrss)


def main() -> None:
    pun = os.environ.get("PUN_BIN", str(ROOT / "target/profiling/pun"))
    pint = ROOT / ".oracle/pint/builds/pint"
    mago = os.environ.get("MAGO_BIN") or next((str(p) for p in sorted((ROOT / ".oracle/mago").glob("**/mago")) if p.is_file()), None)
    runs = int(os.environ.get("RUNS", "3"))
    warmup = int(os.environ.get("WARMUP", "1"))
    tools = os.environ.get("TOOLS", ",".join(TOOLS)).split(",")
    projects = sys.argv[1:] or PROJECTS
    if not os.access(pun, os.X_OK):
        sys.exit(f"missing {pun}: cargo build --profile profiling -p punphp")
    if not pint.is_file():
        sys.exit(f"missing {pint}: run scripts/oracle.sh")
    if "mago" in tools and not mago:
        sys.exit("missing Mago: put the 1.57.0 binary under .oracle/mago/ or set MAGO_BIN (or drop it from TOOLS)")

    env = {**os.environ, "PUN_AGENT": "0", "PUN_IGNORE_PHP_CONSTRAINT": "1"}
    os.environ.update(env)
    scratch = Path(tempfile.mkdtemp())
    detail = ["| project | tool | run | time | peak memory |", "|---|---|---:|---:|---:|"]
    summary = {}
    try:
        for name in projects:
            src = ROOT / "corpus" / name
            if not src.is_dir():
                print(f"skip {name}: no corpus/{name}", file=sys.stderr)
                continue
            work = scratch / name
            shutil.copytree(src, work, symlinks=True, ignore=shutil.ignore_patterns(".git", "node_modules"))
            if (src / "node_modules").is_dir():
                (work / "node_modules").symlink_to((src / "node_modules").resolve())
            subprocess.run([pun, "migrate", "--yes"], cwd=work, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            files = sum(1 for p in src.rglob("*.php") if not {"vendor", "node_modules", ".git"} & set(p.parts))
            commands = {
                "pint-parallel": (["php", str(pint), "--test", "--parallel"], src),
                "pint": (["php", str(pint), "--test"], src),
                "pun": ([pun, "check", "--test", "--no-cache"], work),
                "mago": ([mago or "mago", "fmt", "--check", "."], work),
            }
            for tool in tools:
                cmd, cwd = commands[tool]
                for _ in range(warmup):
                    run_once(cmd, cwd)
                results = []
                for i in range(1, runs + 1):
                    t, m = run_once(cmd, cwd)
                    results.append((t, m))
                    detail.append(f"| {name} | {tool} | {i} | {t:.2f} s | {m / 1048576:.0f} MB |")
                    print(f"{name} {tool} run {i}: {t:.2f} s, {m / 1048576:.0f} MB", file=sys.stderr, flush=True)
                summary[(name, tool)] = (files, statistics.median(r[0] for r in results), statistics.median(r[1] for r in results))
    finally:
        shutil.rmtree(scratch, ignore_errors=True)

    print("\n".join(detail))
    print()
    head = "| project | files |" + "".join(f" {t} time | {t} mem |" for t in tools)
    if "pun" in tools:
        head += "".join(f" pun vs {t} |" for t in tools if t != "pun")
    print(head)
    print("|---|---:|" + "---:|" * (head.count("|") - 3))
    for name in projects:
        cells = [summary.get((name, t)) for t in tools]
        if not any(cells):
            continue
        files = next(c[0] for c in cells if c)
        row = f"| {name} | {files} |"
        for c in cells:
            row += f" {c[1]:.2f} s | {c[2] / 1048576:.0f} MB |" if c else " - | - |"
        pun_cell = summary.get((name, "pun"))
        if pun_cell:
            for t in tools:
                if t != "pun":
                    other = summary.get((name, t))
                    row += f" {other[1] / pun_cell[1]:.1f}x |" if other else " - |"
        print(row)


if __name__ == "__main__":
    main()
