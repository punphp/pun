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
- pun: `pun check --test` in a scratch copy (no .git, node_modules symlinked) after `pun migrate --yes`.
- pun-format: `pun format --test`, same scratch copy (`pun` also runs lint and the analyser).
- mago: `mago fmt --check .` in the same scratch copy, default config. Mago's style is not Pint's: a speed and memory
  reference, not a parity check. Without a path and without mago.toml it checks nothing.

Memory is the peak summed RSS of the command's whole process tree (PHP, Node and worker processes included), sampled
every 20 ms, and never below the command's own maxrss (wait4). Exit codes are ignored (1 = files would change).
Prints a per-run table, then medians.

BENCH=analyse and BENCH=lint (format is the default) compare Pun's analyser and lint engine the same way, adding each
tool's error count and Pun's memory ratio to the medians (AGENTS.md "Analyse and lint modes"):

    BENCH=analyse SETUP_ONLY=1 scripts/bench.py pinkary   # scratch copy + composer install, no measurement
    BENCH=analyse scripts/bench.py pinkary                # phpstan-cold, phpstan-warm, pun, mago
    BENCH=lint BENCH_RESULTS=lint.json TOOLS=rector-full RUNS=1 scripts/bench.py laravel-framework
                                                          # rector-ported, rector-full, pun, mago; runs accumulate
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
TOOLS = ["pint-parallel", "pint", "pun", "pun-format", "mago"]


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


def run_once(cmd: list[str], cwd: Path, stdout=subprocess.DEVNULL, env=None) -> tuple[float, int]:
    start = time.perf_counter()
    p = subprocess.Popen(cmd, cwd=cwd, stdout=stdout, stderr=subprocess.DEVNULL, env=env)
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


def bench_format() -> None:
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
            # A phpstan include from vendor/ (the checkout has none) would make `pun migrate` fail; format needs neither.
            for extra in [*work.glob("phpstan*.neon*"), work / "rector.php"]:
                extra.unlink(missing_ok=True)
            subprocess.run([pun, "migrate", "--yes"], cwd=work, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            files = sum(1 for p in src.rglob("*.php") if not {"vendor", "node_modules", ".git"} & set(p.parts))
            commands = {
                "pint-parallel": (["php", str(pint), "--test", "--parallel"], src),
                "pint": (["php", str(pint), "--test"], src),
                "pun": ([pun, "check", "--test"], work),
                "pun-format": ([pun, "format", "--test"], work),
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

# ---- BENCH=analyse and BENCH=lint -------------------------------------------------------------------------------

SCRATCH = Path(os.environ.get("BENCH_SCRATCH", ROOT / "target-bench"))
PHPSTAN = ROOT / ".oracle/phpstan/vendor/phpstan/phpstan/phpstan.phar"
RECTOR = ROOT / ".oracle/rector-ecosystem/vendor/bin/rector"  # rector 2.6.1 with rector-laravel and Pest's plugin
# project -> (PHPStan includes, paths, extra parameter lines): the project's own config at level 0, without its
# ignoreErrors and baseline (scripts/analyse-corpus.py's configs, plus laravel-pint).
ANALYSE = {
    "pinkary": (["vendor/larastan/larastan/extension.neon", "vendor/phpstan/phpstan/conf/bleedingEdge.neon"],
                ["app", "config", "bootstrap", "database/factories", "routes"], []),
    "larastan": (["vendor/phpstan/phpstan/conf/bleedingEdge.neon", "vendor/phpstan/phpstan-deprecation-rules/rules.neon",
                  "extension.neon"], ["src"], ["phpVersion: 80200"]),
    "laravel-framework": ([], ["src"], ["excludePaths: [src/Illuminate/Testing/ParallelRunner.php, 'src/*/views/*']"]),
    "pest": (["phpstan-pest-extension.neon", "vendor/pestphp/pest-plugin-phpstan/extension.neon"], ["src"], []),
    "laravel-pint": (["vendor/larastan/larastan/extension.neon"], ["app", "bootstrap", "config", "resources"], []),
}
# project -> its PHPStan excludePaths, for pun-bench-analyse.toml
ANALYSE_EXCLUDE = {"laravel-framework": ["src/Illuminate/Testing/ParallelRunner.php", "src/*/views/*"]}
# project -> the paths of its rector.php
LINT = {
    "pinkary": ["app", "bootstrap/app.php", "config", "database", "public", "tests"],
    "laravel-framework": ["config", "src", "tests", "types"],
    "dev-null": ["app", "bootstrap/app.php", "config", "database", "public", "routes", "tests"],
}
MODES = {
    "analyse": (["phpstan-cold", "phpstan-warm", "pun", "mago"], ANALYSE),
    "lint": (["rector-ported", "rector-full", "pun", "mago"], LINT),
}
# The corpus larastan commit needs PHPStan 2.3; the oracle pins PHPStan 2.3.1 and Larastan v3.10.0, so analyse that tag.
TAGS = {"larastan": ("https://github.com/larastan/larastan.git", "v3.10.0")}
# Packages without a composer.lock that require packages which require themselves: their own version.
ROOT_VERSION = {"pest": "5.99.0", "laravel-framework": "13.99.0"}
# Packages replaced by copies of the oracle's, so every project runs PHPStan 2.3.1 from its own vendor/ (Larastan stays
# the project's). Copies, not symlinks: PHPStan loads the vendor/autoload.php next to its real path.
PINNED = ["phpstan/phpstan", "phpstan/phpstan-deprecation-rules"]
# PHPStan 2.2 switches to its agent output format when it sees one of these (src/Internal/AgentDetector.php).
AGENT_ENV = ["AUGMENT_AGENT", "AMP_CURRENT_THREAD_ID", "AI_AGENT", "CURSOR_TRACE_ID", "CURSOR_AGENT", "GEMINI_CLI",
             "CODEX_SANDBOX", "CODEX_THREAD_ID", "OPENCODE_CLIENT", "OPENCODE", "CLAUDECODE", "CLAUDE_CODE", "REPL_ID"]


def php_files(work: Path, paths: list[str]) -> int:
    """The PHP files `pun lint` and `pun analyse` walk: no vendor/, node_modules/, dot dirs or Blade."""
    n = 0
    for rel in paths:
        p = work / rel
        if p.is_file():
            n += 1
            continue
        for f in p.rglob("*.php"):
            parts = f.relative_to(work).parts
            if not f.name.endswith(".blade.php") and not {"vendor", "node_modules"} & set(parts) \
                    and not any(x.startswith(".") for x in parts):
                n += 1
    return n


def setup(name: str) -> Path:
    """Scratch copy of corpus/<name> with composer's vendor/ (made once, reused); PHPStan and Larastan pinned."""
    work = SCRATCH / name
    if not (work / "vendor/autoload.php").is_file():
        work.parent.mkdir(parents=True, exist_ok=True)
        if name in TAGS:
            shutil.rmtree(work, ignore_errors=True)
            subprocess.run(["git", "-c", "advice.detachedHead=false", "clone", "-q", "--depth", "1", "-b", TAGS[name][1], TAGS[name][0], str(work)],
                           check=True)
            shutil.rmtree(work / ".git")
        else:
            work.mkdir(exist_ok=True)
            subprocess.run(["rsync", "-a", "--exclude", ".git", "--exclude", "node_modules", "--exclude", "vendor",
                            f"{ROOT / 'corpus' / name}/", f"{work}/"], check=True)
        env = {**os.environ, "COMPOSER_ROOT_VERSION": ROOT_VERSION[name]} if name in ROOT_VERSION else None
        r = subprocess.run(["composer", "install", "--no-scripts", "--no-interaction", "--no-progress",
                            "--ignore-platform-reqs", "-q"], cwd=work, env=env, capture_output=True, text=True)
        if r.returncode:
            raise RuntimeError(f"composer install failed: {r.stderr.strip()[-500:]}")
    for pkg in PINNED:
        dst, src = work / "vendor" / pkg, ROOT / ".oracle/phpstan/vendor" / pkg
        if not dst.exists() and not dst.is_symlink():
            continue
        if dst.is_symlink() or (dst / "composer.json").read_bytes() != (src / "composer.json").read_bytes():
            dst.unlink() if dst.is_symlink() else shutil.rmtree(dst)
            shutil.copytree(src, dst)
    return work


def write_configs(mode: str, name: str, work: Path, rector_rules: list[str]) -> None:
    if mode == "analyse":
        includes, paths, extra = ANALYSE[name]
        for kind in ["cold", "warm"]:
            lines = (["includes:"] + [f"    - {i}" for i in includes]) if includes else []
            lines += ["parameters:", "    level: 0", f"    tmpDir: {work / '.bench-tmp' / kind}",
                      f"    paths: [{', '.join(paths)}]"] + [f"    {l}" for l in extra]
            (work / f"pun-bench-{kind}.neon").write_text("\n".join(lines) + "\n")
        (work / "pun-bench-mago.toml").write_text(
            "[source]\npaths = [" + ", ".join(f'"{p}"' for p in paths) + ']\nincludes = ["vendor"]\n')
        # Pun reads only pun.toml: the same paths and excludes, PHPStan level 0's preset.
        exclude = ", ".join(f'"{e}"' for e in ANALYSE_EXCLUDE.get(name, []))
        (work / "pun-bench-analyse.toml").write_text(
            '[analyse]\npreset = "basic"\npaths = [' + ", ".join(f'"{p}"' for p in paths) + f"]\nexclude = [{exclude}]\n")
    else:
        # Only the lint rules that have a Rector class, at `error`: the rules Rector runs here too, for a like-for-like
        # comparison (Pun's other rules, from PHP-CS-Fixer, Mago and PHPStan, have no Rector counterpart).
        (work / "pun-bench-lint.toml").write_text(
            "[lint.rules]\n" + "".join(f'"{i.removeprefix("lint/")}" = "error"\n' for i in rector_mapped_rules()))
        # The pinned Rector needs the project's classes, loaded after its own: Composer prepends its loader, which
        # would let the project's rector-laravel shadow the pinned one.
        (work / "pun-bench-autoload.php").write_text(
            "<?php\n\n$loader = require __DIR__.'/vendor/autoload.php';\n$loader->unregister();\n"
            "$loader->register(false);\n")
        (work / "pun-bench-rector.php").write_text(rector_config(name, rector_rules))


# The configurable Rector classes whose Pun rule's default options are a Rector configuration, key for key: they run
# with that configuration (Rector's own defaults), as Pun runs its rule without options.
CONFIGURED = {
    "Pest\\Rector\\Rules\\ChainExpectCallsRector": "['merge_different_variables' => true]",
    "Rector\\CodeQuality\\Rector\\FunctionLike\\SimplifyUselessVariableRector": "['only_direct_assign' => false]",
    "Rector\\Php55\\Rector\\String_\\StringClassNameToClassConstantRector": "[]",
    "Rector\\TypeDeclaration\\Rector\\Property\\TypedPropertyFromAssignsRector": "['inline_public' => false]",
    "RectorLaravel\\Rector\\Class_\\AddHasFactoryToModelsRector": "[]",
    "RectorLaravel\\Rector\\MethodCall\\EloquentOrderByToLatestOrOldestRector":
        "['allowed_patterns' => ['*_at', '*_date', '*_on']]",
    "RectorLaravel\\Rector\\PropertyFetch\\OptionalToNullsafeOperatorRector": "['exclude_methods' => []]",
    "RectorLaravel\\Rector\\StaticCall\\EloquentMagicMethodToQueryBuilderRector": "['exclude_methods' => []]",
}


def rector_config(name: str, rector_rules: list[str]) -> str:
    """A rector.php that runs `rector_rules` on the paths of `name`'s rector.php."""
    paths = ", ".join(f"__DIR__.'/{p}'" for p in LINT[name])
    rules = ", ".join(f"\\{c}::class" for c in rector_rules)
    # rector-laravel's node visitors (ArrayDimFetchContextNodeVisitor, which ServerVariableToRequestFacadeRector
    # and RequestVariablesToRequestFacadeRector read): every rector-laravel set imports this file, and without it
    # those rules rewrite assignments to superglobals into calls, which PHP rejects.
    laravel = RECTOR.parent.parent / "driftingly/rector-laravel/config/config.php"
    sets = f"    ->withSets(['{laravel}'])\n" if laravel.exists() else ""
    configured = "".join(f"\\{c}::class => {CONFIGURED[c]}, " for c in rector_rules if c in CONFIGURED)
    return ("<?php\n\n$config = Rector\\Config\\RectorConfig::configure()\n"
            f"    ->withPaths([{paths}])\n"
            f"{sets}"
            # Only the classes the Rector that runs has (a project without Pest's plugin lacks its rules), and none
            # that needs a configuration: unconfigured, Rector fails on every file, and Pun's port does nothing. The
            # CONFIGURED ones are registered after, with their configuration.
            f"    ->withRules(array_values(array_filter([{rules}], fn ($c) => class_exists($c)\n"
            "        && ! is_subclass_of($c, \\Rector\\Contract\\Rector\\ConfigurableRectorInterface::class))))\n"
            "    ->withPhpVersion(80400)\n"
            # Rector's 120 s per batch of files times out on a busy machine, and a timed-out batch's files go
            # unprocessed.
            "    ->withParallel(timeoutSeconds: 900);\n"
            f"foreach ([{configured}] as $class => $configuration) {{\n"
            "    if (class_exists($class)) {\n"
            "        $config->withConfiguredRule($class, $configuration);\n"
            "    }\n"
            "}\n\n"
            "return $config;\n")


def native_rules() -> list[str]:
    """The IDs of pun-lint's native rules, read from each rule's `fn id`, that rules/mapping.toml marks ported (a rule
    file not yet in `rules::all()` is not)."""
    import re
    import tomllib
    ids = set()
    for f in (ROOT / "crates/pun-lint/src/rules").rglob("*.rs"):
        ids.update(re.findall(r'fn id\(&self\) -> &\'static str \{\s*"(lint/[^"]+)"', f.read_text()))
    lint = tomllib.loads((ROOT / "crates/pun-config/rules/mapping.toml").read_text())["rules"]["lint"]
    return sorted(i for i in ids if lint.get(i.removeprefix("lint/"), {}).get("ported"))


def rector_mapped_rules() -> list[str]:
    """The ported native lint rules that map to at least one Rector class."""
    import tomllib
    ids = set(native_rules())
    mapping = tomllib.loads((ROOT / "crates/pun-config/rules/mapping.toml").read_text())
    return sorted({entry["pun"] for table, rules in mapping["upstream"].items() if "rector" in table
                   for entry in rules.values() if isinstance(entry, dict) and entry.get("pun") in ids and "class" in entry})


def ported_rector_rules() -> list[str]:
    """The Rector classes of the rules pun-lint has ported: its rule IDs, looked up in rules/mapping.toml."""
    import tomllib
    ids = native_rules()
    mapping = tomllib.loads((ROOT / "crates/pun-config/rules/mapping.toml").read_text())
    classes = [entry["class"] for table, rules in mapping["upstream"].items() if "rector" in table
               for entry in rules.values() if isinstance(entry, dict) and entry.get("pun") in ids and "class" in entry]
    if not classes:
        sys.exit(f"no Rector class in rules/mapping.toml for {ids}")
    # Rector applies rules to a node in the order they are registered, and the first that sets a return type wins
    # (AddClosureVoidReturnTypeWhereNoReturnRector before AddClosureNeverReturnTypeRector): register them in the
    # order of Rector's prepared sets (src/Config/Level), then the rest by name.
    import re
    rank: dict[str, int] = {}
    for level in ["DeadCode", "CodeQuality", "CodingStyle", "TypeDeclaration", "TypeDeclarationDocblocks"]:
        text = (ROOT / f".oracle/rector/src/Config/Level/{level}Level.php").read_text()
        for name in re.findall(r"^\s+(\w+)::class,", text, re.M):
            rank.setdefault(name, len(rank))
    return sorted(classes, key=lambda c: (rank.get(c.rsplit("\\", 1)[-1], len(rank)), c))


def count(tool: str, out: str) -> int | None:
    """The tool's error count: PHPStan's errors, Pun's analyse errors, Mago's issues, Rector's changed files, the
    files Pun's lint flags."""
    import json
    try:
        data = json.loads(out[min(i for i in (out.find("{"), out.find("[")) if i >= 0):])
    except ValueError:
        return None
    try:
        return count_of(tool, data)
    except (KeyError, TypeError):
        return None


def count_of(tool: str, data) -> int:
    if tool.startswith("phpstan"):
        return data["totals"]["errors"] + data["totals"]["file_errors"]
    if tool.startswith("rector"):
        # Rector 2.8 lists the changed files but leaves totals.changed_files at 0.
        return max(data["totals"]["changed_files"], len(data.get("changed_files", [])))
    if tool == "mago":
        return len(data["issues"] if isinstance(data, dict) else data)
    # `pun analyse --format=json` is PHPStan's json; `pun lint --format=json` lists the files with problems.
    return data["totals"]["file_errors"] if "totals" in data else len(data["files"])


def ratio(other: float, pun: float) -> str:
    """How many times Pun's time or memory fits in the other tool's: above 1 Pun is faster or smaller."""
    r = other / pun
    return f"{r:.1f}x" if r >= 1 else f"{r:.2f}x"


def bench_mode(mode: str) -> None:
    import json
    all_tools, configs = MODES[mode]
    pun = os.environ.get("PUN_BIN", str(ROOT / "target/profiling/pun"))
    mago = os.environ.get("MAGO_BIN") or next((str(p) for p in sorted((ROOT / ".oracle/mago").glob("**/mago")) if p.is_file()), None)
    runs = int(os.environ.get("RUNS", "3"))
    warmup = int(os.environ.get("WARMUP", "1"))
    tools = os.environ.get("TOOLS", ",".join(all_tools)).split(",")
    projects = sys.argv[1:] or list(configs)
    if "pun" in tools and not os.access(pun, os.X_OK):
        sys.exit(f"missing {pun}: cargo build --profile profiling -p punphp")
    if "mago" in tools and not mago:
        sys.exit("missing Mago: put the 1.57.0 binary under .oracle/mago/ or set MAGO_BIN (or drop it from TOOLS)")
    upstream = PHPSTAN if mode == "analyse" else RECTOR
    if not upstream.is_file():
        sys.exit(f"missing {upstream}: run scripts/oracle.sh")
    rector_rules = ported_rector_rules() if mode == "lint" else []
    env = {k: v for k, v in os.environ.items() if k not in AGENT_ENV}
    env.update({"PUN_AGENT": "0", "PUN_IGNORE_PHP_CONSTRAINT": "1", "XDEBUG_MODE": "off"})

    # BENCH_RESULTS=file.json keeps the runs of earlier calls (split by project, tool or run) and adds these runs to
    # them; the tables cover every run in the file. Delete the file to start over.
    results_file = os.environ.get("BENCH_RESULTS")
    store = json.loads(Path(results_file).read_text()) if results_file and Path(results_file).is_file() else {}
    results = store.setdefault(mode, {})
    skipped = []
    for name in projects:
        if name not in configs:
            skipped.append(f"{name}: no {mode} config in scripts/bench.py")
            continue
        if not (ROOT / "corpus" / name).is_dir():
            skipped.append(f"{name}: no corpus/{name}")
            continue
        try:
            work = setup(name)
        except (RuntimeError, subprocess.CalledProcessError) as e:
            skipped.append(f"{name}: {e}")
            continue
        write_configs(mode, name, work, rector_rules)
        if os.environ.get("SETUP_ONLY") == "1":
            print(f"set up {work}", file=sys.stderr)
            continue
        paths = configs[name][1] if mode == "analyse" else configs[name]
        phpstan = ["php", "vendor/phpstan/phpstan/phpstan.phar", "analyse", "--no-progress", "--error-format=json", "--memory-limit=-1", "-c"]
        rector = ["process", "--dry-run", "--clear-cache", "--no-progress-bar", "--output-format=json",
                  "--memory-limit=-1", "--config"]
        commands = {
            "phpstan-cold": phpstan + ["pun-bench-cold.neon"],
            "phpstan-warm": phpstan + ["pun-bench-warm.neon"],
            "rector-ported": ["php", str(RECTOR)] + rector + ["pun-bench-rector.php", "--autoload-file",
                                                           "pun-bench-autoload.php"],
            # The project's own Rector install: its rector.php loads sets (Pest's, for one) from its vendor/.
            "rector-full": ["php", "vendor/bin/rector"] + rector + ["rector.php"],
            "pun": [pun, "analyse", "--config", "pun-bench-analyse.toml", "--format=json"] if mode == "analyse"
                   else [pun, "lint", "--config", "pun-bench-lint.toml", "--format=json", *paths],
            "mago": [mago or "mago", "--config", str(work / "pun-bench-mago.toml"), "analyze",
                     "--reporting-format", "json"] if mode == "analyse"
                    else [mago or "mago", "lint", "--reporting-format", "json", *paths],
        }
        for tool in tools:
            cmd = commands[tool]
            out_path = work / ".bench-tmp" / f"{mode}-{tool}.out"
            out_path.parent.mkdir(exist_ok=True)

            def once() -> tuple[float, int]:
                if tool == "phpstan-cold":
                    shutil.rmtree(work / ".bench-tmp" / "cold", ignore_errors=True)
                with open(out_path, "w") as out:
                    return run_once(cmd, work, out, env)

            # The warm row needs one run that fills PHPStan's result cache.
            for _ in range(max(warmup, 1) if tool == "phpstan-warm" else warmup):
                once()
            timed = []
            for i in range(1, runs + 1):
                t, m = once()
                timed.append((t, m))
                print(f"{name} {tool} run {i}: {t:.2f} s, {m / 1048576:.0f} MB", file=sys.stderr, flush=True)
            n = count(tool, out_path.read_text(errors="replace"))
            if n is None:
                print(f"{name} {tool}: no count in {out_path}", file=sys.stderr)
            earlier = results.get(f"{name}|{tool}", {}).get("runs", [])
            results[f"{name}|{tool}"] = {"files": php_files(work, paths), "runs": earlier + timed, "count": n}
    if os.environ.get("SETUP_ONLY") == "1":
        for s in skipped:
            print(f"skipped {s}")
        return
    if results_file:
        Path(results_file).write_text(json.dumps(store, indent=1) + "\n")

    order = [p for p in configs if any(k.startswith(p + "|") for k in results)]
    shown = [t for t in all_tools if any(k.endswith("|" + t) for k in results)]
    print("| project | tool | run | time | peak memory |")
    print("|---|---|---:|---:|---:|")
    for name in order:
        for tool in shown:
            r = results.get(f"{name}|{tool}")
            for i, (t, m) in enumerate(r["runs"] if r else [], 1):
                print(f"| {name} | {tool} | {i} | {t:.2f} s | {m / 1048576:.0f} MB |")
    print()
    med = {k: (statistics.median(r[0] for r in v["runs"]), statistics.median(r[1] for r in v["runs"]))
           for k, v in results.items() if v["runs"]}
    head = "| project | files |" + "".join(f" {t} time | {t} mem | {t} count |" for t in shown)
    if "pun" in shown:
        head += "".join(f" pun vs {t} time | pun vs {t} mem |" for t in shown if t != "pun")
    print(head)
    print("|---|---:|" + "---:|" * (head.count("|") - 3))
    for name in order:
        files = next(results[f"{name}|{t}"]["files"] for t in shown if f"{name}|{t}" in results)
        row = f"| {name} | {files} |"
        for t in shown:
            k = f"{name}|{t}"
            c = results.get(k, {}).get("count")
            row += (f" {med[k][0]:.2f} s | {med[k][1] / 1048576:.0f} MB |" if k in med else " - | - |") \
                + (f" {c} |" if c is not None else " - |")
        p = med.get(f"{name}|pun")
        if p:
            for t in shown:
                if t != "pun":
                    o = med.get(f"{name}|{t}")
                    row += f" {ratio(o[0], p[0])} | {ratio(o[1], p[1])} |" if o else " - | - |"
        print(row)
    for s in skipped:
        print(f"skipped {s}")


def main() -> None:
    mode = os.environ.get("BENCH", "format")
    if mode == "format":
        bench_format()
    elif mode in MODES:
        bench_mode(mode)
    else:
        sys.exit(f"BENCH={mode}: expected format, analyse or lint")


if __name__ == "__main__":
    main()
