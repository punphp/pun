# AGENTS.md

Pun is one Rust binary for the PHP toolchain. `pun format` produces the same bytes as Laravel Pint. `pun analyse` (parity with PHPStan and Larastan) is in progress.

Read these for more:

- `context/PLAN.md`: the source of truth. Every decision, the build order with each row's status, and the open questions. Start here.
- `context/plan/`: one plan per part, each with its Build list. `plan/rules.md` covers rule placement, naming and the upstream mapping.
- `README.md`: the behavior users see.

## Match the upstream tool

Pun's machine formats (`--format=...`), messages, and exit codes must match the pinned upstream tool: Pint, PHP-CS-Fixer, or PHPStan, except that a run error (Pun couldn't run) exits 2 where Pint exits 1 (`context/plan/output.md`). The human output is Pun's own (`context/plan/output.md`), so it does not copy upstream's screen. If you are unsure what upstream does, run it. `scripts/oracle.sh` installs the pinned tools into `.oracle/`. They run on the system PHP 8.5, with no Docker.

- Pint: `php .oracle/pint/builds/pint`
- PHP-CS-Fixer: `.oracle/php-cs-fixer`
- PHPStan: `.oracle/phpstan`

Port upstream code function by function. Use the `pcre2` crate wherever upstream calls `preg_*`. Use `pun_tokens::php_compat` for PHP string behavior, such as byte-based `strlen` and ASCII-only `strtolower`.

Corpus projects live in `corpus/`. `corpus/manifest.toml` lists them, and `scripts/corpus.sh <name>` checks one out. `/home/forge/projects/pinkary.com` is the acceptance project. Never modify it. To get a writable checkout, copy it into `corpus/`.

Run these checks in this order, cheapest first:

```sh
cargo test -p punphp-<x>
cargo test -p punphp-fixers <substring>
cargo build -p punphp -p punphp-harness
target/debug/pun-harness format --corpus pinkary --diffs 5
scripts/cli-parity.sh "ARGS"...
scripts/fuzz-parity.sh
scripts/coverage.sh punphp-tokens
```

- Package `punphp-<x>` lives in `crates/pun-<x>` and is imported as `pun_<x>`.
- `PUN_FIXERS_ONLY=a,b` makes `build.rs` compile only fixers `a` and `b`.
- `pun-harness` compares Pint with `pun migrate` followed by `pun check --fix`. Add `--fixer <name>` to test one rule.
- `scripts/cli-parity.sh` compares the exit codes, machine formats, and resulting files of Pint and `pun` byte for byte.
- `scripts/fuzz-parity.sh` is a mutation fuzzer that compares Pun with Pint. It needs a freshly built `target/debug/pun`.
- `scripts/coverage.sh` reports line coverage for the crates you name. Without arguments, it covers every crate.

## Benchmark Pun against Pint and Mago

`scripts/bench.py [project...]` measures wall-clock time and peak memory for every run of four tools on each corpus project. It prints a per-run table, then a table of medians with Pun's speedup over each tool. Every tool is check-only, nothing writes to the corpus, and Pun never reads its cache.

1. `scripts/oracle.sh` installs Pint into `.oracle/pint`. `scripts/corpus.sh` checks out the projects in `corpus/`.
2. `cargo build --profile profiling -p punphp` builds `target/profiling/pun`. `PUN_BIN=path` selects another binary.
3. Put the Mago 1.57.0 release binary (the version the Mago crates pin) under `.oracle/mago/`, or pass `MAGO_BIN=path`.
4. Run `scripts/bench.py`. `RUNS` (default 3) sets the measured runs per tool, `WARMUP` (default 1) the untimed runs before them, and `TOOLS` a subset of `pint-parallel,pint,pun,mago`.

The four tools:

- `pint-parallel`: `pint --test --parallel` inside `corpus/<name>`, with the project's own `pint.json`.
- `pint`: `pint --test`, one process, in the same place.
- `pun`: `pun check --test --no-cache` in a scratch copy of the project (no `.git`, `node_modules` symlinked), after `pun migrate --yes` has written its `pun.toml`.
- `mago`: `mago fmt --check .` in the same scratch copy, with Mago's default config. Mago's style is not Pint's, so its numbers are a speed and memory reference, not a parity check. Without the `.` and without a `mago.toml`, Mago checks no files.

Memory is the peak summed RSS of the whole process tree, sampled every 20 ms and never below the command's own maxrss, so Pint's parallel PHP workers and Pun's Prettier workers count. The script sets `PUN_AGENT=0` and `PUN_IGNORE_PHP_CONSTRAINT=1`, and ignores exit codes.

Pinkary is the slowest project for Pun because Blade formatting runs through Prettier workers. `pun --max-processes=N` caps Pun's threads, and its workers with them. Record published timings in `context/reports/scoreboard.md`.

### Analyse and lint modes

`BENCH=analyse` and `BENCH=lint` measure the same way and print the same two tables. The medians table adds each tool's error count and Pun's memory ratio. `BENCH=format` is the default and behaves as described above.

- Setup: each project gets a scratch copy with `composer install` under `BENCH_SCRATCH` (default `target-bench/`), made once and reused. PHPStan 2.2.7 is copied from `.oracle/phpstan` into the copy's `vendor/`. `SETUP_ONLY=1` makes the copies without measuring, so run it through a normal heavy slot first.
- Pun runs as `pun analyse --config pun-bench-analyse.toml --format=json` and `pun lint --config pun-bench-lint.toml --format=json` (every native rule at `error`), from `target/profiling/pun` or `PUN_BIN`.
- `BENCH_RESULTS=file.json` keeps runs across calls and adds each call's runs to them. Use it to split a slow project by tool (`TOOLS=`) or by run, so one call fits a 10-minute slot. Delete the file to start over.
- The script sets `XDEBUG_MODE=off` and removes the coding-agent variables that switch PHPStan's output format.

Analyse mode runs on pinkary, larastan (its v3.10.0 tag), laravel-framework, pest and laravel-pint. It uses each project's PHPStan includes and paths at level 0, without ignores or baseline. Its tools are:

- `phpstan-cold`: the result cache is deleted before every run.
- `phpstan-warm`: the result cache is warm. This is a reference row.
- `pun`: Pun's analyser on the same config.
- `mago`: `mago analyze` on the same paths, with `vendor/` as includes.

Pun runs fewer rules than PHPStan level 0, so the counts only show that the tools did comparable work.

Lint mode runs on pinkary and laravel-framework, on the paths of each project's rector.php. Its tools are:

- `rector-ported`: `.oracle/rector-ecosystem` (Rector with rector-laravel) with exactly the Rector classes of pun-lint's rules. The list comes from `check --rules` and `rules/mapping.toml`, so it grows as rules are ported.
- `rector-full`: the project's own Rector and rector.php. This is a reference row.
- `pun`: `pun lint` with every native rule.
- `mago`: `mago lint`.

Every Rector run uses `--dry-run --clear-cache`.

## Architecture

- `pun-tokens` is the base crate. It holds a lexer that matches `token_get_all`, PHP-CS-Fixer's transformers, and `TokensAnalyzer`.
- `pun-format` runs the fixers (the `Fixer` trait) over one token list, in PHP-CS-Fixer's priority order. It also lints with PHP's own grammar. `scripts/gen-php-grammar.py` generates `php_grammar.rs`.
- `pun-fixers` holds one file per fixer in `src/fixers/`, and `build.rs` registers them. Its `Pun/*` rules are opt-in ports from Mago (`context/reference/mago-extras.md`).
- A fixer's tests come from three sources: upstream fixtures exported to JSON, oracle output in `tests/cases/<fixer>.json` (create it with `scripts/oracle-case.php`), and `tests/metadata.json` (the priority and risky flag from Pint).
- `pun-config` reads `pun.toml` and nothing else. `rules/mapping.toml` maps Pun rule IDs to upstream names, and `scripts/check-rule-mapping.py` checks it. `migrate.rs` converts `pint.json` and `.php-cs-fixer.php`. `docs/presets/laravel.json` is compiled into the crate, so do not move it.
- Pun never runs PHP for a rule it lacks. A rule in `pun.toml` without a native port is a config error (exit 2) that names the rule. System PHP is only the test oracle, and `pun migrate` uses it to read a `.php-cs-fixer.php`.
- `pun-cli` builds the `pun` binary. The crate holds Pint-identical reporters, the compact output for coding agents (`agent.rs`), the cache, `--watch`, and the interactive `migrate`.
- `pun-phpstan` ports PHPStan's config, ignore, and type-string behavior. Its built-in symbols are in `data/symbols.json`. `pun-analyse` builds `pun analyse` on `pun-phpstan` and Mago's analyzer.
- `pun-harness` is local tooling. It regenerates `context/reports/scoreboard.md`.
- The commands divide the work like this. `format` changes layout only. `lint` applies code rules, including PHP-CS-Fixer's risky fixers. `analyse` reports type errors. `check` runs all three in one pass.
- The Mago crates are pinned to `=1.57.0`. Bump them together.
- Harness-only environment variables: `PUN_IGNORE_PHP_CONSTRAINT=1` (for corpora that allow PHP below 8.4), `PUN_BIN`, `PUN_HARNESS_ROOT`, and `PUN_PROFILE`.
- Every test that spawns `pun` sets `PUN_AGENT=0`. Tests of agent mode are the exception.

## Build budget

1. Run `source ~/.cargo/env`.
2. Size builds to the machine: check `nproc` and free memory, and leave headroom for the subagents running beside you.
3. Prefix long commands with `timeout 900 nice`, for example `timeout 900 nice cargo test -p punphp-tokens`.
4. Build and test only the crates you changed (`-p`). Give each concurrent build its own `CARGO_TARGET_DIR`
   (`target-<name>`) so two cargo runs never wait on one lock or reuse each other's stale artifacts.
5. Delete scratch target dirs and downloads when you no longer need them. Keep `target/`.

Corpus runs cover one fixer on one project. While you iterate, use Pinkary or laravel-framework.

## Tests

Every crate needs 100% line coverage, measured by `scripts/coverage.sh`. Delete any line that no test can reach. A rare, justified exclusion goes in `context/reports/COVERAGE.md` with its reason.

Take each expected value from a source other than the code under test: an upstream fixture, the real tool's output, `token_get_all`, or a value you derived by hand. To cover a branch that upstream's fixtures miss, write an input, run the oracle on it, and paste the oracle's output as the expectation.

Keep a test only if it fails on a plausible wrong implementation. If you cannot name that bug, delete the test. These tests fail the bar:

- A function compared with itself or with a copy of its own logic.
- A snapshot of the current output.
- A test with no assertion.
- A test that asserts only "does not panic" when the output is checkable.
- A mock that only verifies calls.
- A test of a private helper that the public API already covers.

## How work runs

One lead agent drives the build and works on `main`. It spawns subagents for independent work and merges what
they return.

- **Always work on the `main` branch.** Commit on main and push to `origin/main`. Do not create long-lived
  branches, `helper/*` branches or pull requests. Rewrite history or force-push only when Pushpak asks.
- At the start of a session, read `context/PLAN.md` (§0 "Resume here" first), then run `git log -15`,
  `git status` and `git fetch origin`. Pick the next row from §0's priorities.
- Before you start a row, set its status in `context/PLAN.md` §3 to `doing`. When it lands, delete the row and
  update §0's counts.
- **Subagents.** Spawn as many as the work and the machine allow, one per independent slice (a rule group, a
  crate, a bug). Give each a slice no other subagent touches, the files it may edit, and the check that proves it
  done. A subagent that edits files runs in its own git worktree (`isolation: "worktree"`, or
  `git worktree add ../pun-<name> main`) with its own `CARGO_TARGET_DIR`, commits there, and the lead merges the
  result into main, runs the tests and pushes. Remove the worktree after the merge. Read-only subagents (search,
  review, oracle runs) need no worktree.
- Shared files (`context/PLAN.md`, `crates/pun-config/rules/mapping.toml`, each crate's `rules/mod.rs`): subagents
  change only their own lines; the lead resolves conflicts at merge time by keeping both sides.
- Never `git stash`, `git reset --hard` or `git clean` in the main checkout while subagents run. Commit with
  explicit paths: `git commit -m "..." -- path1 path2`.
- When a decision is genuinely Pushpak's, add a `needs Pushpak` row in `context/PLAN.md` §3, take the option
  closest to upstream, and keep going.

## Commits

Write each commit message as a Conventional Commit: `type(scope): subject`. Use a lowercase subject and no trailing period. The types are feat, fix, docs, chore, test, perf, refactor, ci, style, and build.

The only author is Pushpak Chhajed <pushpak1300@gmail.com>. Do not add `Co-authored-by` trailers.

## Docs site

`docs/` is also the public docs site, built with [Blume](https://useblume.dev). To run it, use `bun install && bun run dev` inside `docs/`.

- `docs/content/docs/*.mdx`: the pages.
- `docs/pages/index.astro`: the homepage.
- `docs/theme.css`: the brand tokens. They mirror `design/tokens.json`.
- Internal notes (plan, decisions, tasks, reference) live in `context/`, not `docs/`.

When a change alters a flag, a rule, or behavior, update the matching page under `docs/content/docs/`. The `blume` and `blume-update-docs` skills in `.claude/skills/` cover the tooling.
