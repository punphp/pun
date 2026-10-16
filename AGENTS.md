# Pun — agent notes

Plan: docs/PLAN.md. Home: github.com/punphp/pun (npm: @punphp/pun). Current scope: Phase 0 + Phase 1 (`pun format`, byte-identical to Pint).

- Package names are `punphp-<x>` (dirs stay crates/pun-<x>, Rust import names stay `pun_<x>` via workspace `package =` aliases). Use `cargo test -p punphp-<x>`.
- Rust toolchain: `source ~/.cargo/env`. Machine has 2 cores / 7 GB: build only your crate (`cargo test -p <crate>`), never `cargo clean`.
- Crate ownership is per agent. Don't edit another agent's crate; if you need an API from it, read it and adapt, or note the request in docs/REQUESTS.md.
- Root Cargo.toml: you may append to `[workspace.dependencies]`; don't reorder or remove.
- Commit only your own paths: `git add <your paths> Cargo.lock && git commit -m ...`. If `.git/index.lock` exists, wait and retry.
- Oracle = real tools via system PHP 8.5 (no Docker, no embedded libphp yet). Pinned oracle installs live in `.oracle/` (gitignored).
- Acceptance project: /home/forge/projects/pinkary.com (read-only — never modify it; copy into corpus/ if you need a writable checkout).
- Use `pcre2` crate where upstream uses preg_*; PHP string semantics (byte strlen, ASCII strtolower) live in pun-tokens `php_compat`.

## Keeping the box alive (2 cores / 7 GB, several agents at once)

- Always `export CARGO_BUILD_JOBS=1` and prefix heavy commands with `nice` and `timeout` (e.g. `timeout 900 nice cargo test ...`).
- One cargo/php command at a time per agent; never run things in the background and never `cargo build --workspace`/`--release`.
- Harness corpus runs: one fixer, one project at a time; prefer Pinkary/laravel-framework subsets while iterating.
- After every commit: `git push origin main` (retry once on failure; never force-push, never rewrite history).

## Tests: 100% coverage, zero tautological tests

- Gate: 100% line coverage per crate, measured by `scripts/coverage.sh` (cargo-llvm-cov). Any line that genuinely cannot be
  reached gets removed, not excluded; the rare justified exclusion is listed in docs/COVERAGE.md with the reason.
- Expected values must come from somewhere other than the code under test: upstream fixtures, output of the real tool
  (`php .oracle/pint/builds/pint`, `.oracle/php-cs-fixer`, `token_get_all`), or hand-derived values for hand-written cases.
  To cover a fixer branch the upstream fixtures miss, write the input, run the oracle on it, and paste the oracle's output
  as the expectation.
- Banned (tautological): asserting a function equals itself or a copy of its own logic; snapshotting current output and
  calling it expected; `assert!(true)`/no assertion; asserting only that something didn't panic when its output is checkable;
  mocks that only verify calls; tests of private helpers whose behaviour is already pinned through the public API.
- Each test must fail on a plausible wrong implementation. If you can't name the bug it catches, delete it.

## Handoff between sessions

- `docs/TASKS.md` "Current state" is the source of truth: set your row to `doing` with your owner/target dir before starting, `done` when pushed.
- Each agent uses its own `CARGO_TARGET_DIR` (e.g. `target-perf`) and pulls with `--rebase` before every push.
- Starting a new session: read TASKS.md, `git log -15`, `git status`, `ps aux | grep -E 'cargo|rustc|php'`. Rows marked doing with no live agent are abandoned work — verify and commit or revert their uncommitted files before continuing.
- The git index is shared by all agents. Never `git add` and commit separately or use `git commit -a`; commit only your files in one step: `git commit -m "..." -- path1 path2`.
