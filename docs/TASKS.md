# Pun task board

Owner = agent working on it. Status: todo / doing / done / blocked.

## Current state (update when you start or finish a task)

Status: todo / doing / done / blocked. "doing" rows name the owner so a new session doesn't duplicate work.
Background agents die with the session that started them: if a "doing" row's agent no longer exists
(no recent commits, no cargo/php processes in `ps`), treat the row as abandoned. Check `git status` for its
uncommitted files, run the parity gate, then commit or revert them before you take it over.

### Phase 1 / v0.1 (pun format)

| Task | Owner | Status |
| --- | --- | --- |
| All 147 Pinkary rules + new_with_parentheses + Pint/phpdoc_type_annotations_only | - | done |
| 17 opt-in Pun/ (Mago) rules, `pun lint --mago` | - | done |
| Full-config parity: 0 differing on all 7 corpus projects | - | done |
| CLI output identical to Pint (all --format reporters, colours, -v diffs, prettier check) | - | done |
| Lint = PHP's own grammar (TOKEN_PARSE parity, PHP error text) | - | done |
| Mutation fuzzer `scripts/fuzz-parity.sh` (23 seeds, 0 diffs); per-rule e2e suite (166 rules) | - | done |
| Content-hash cache (`--cache-file`, `--no-cache`, default ~/.cache/pun) | - | done |
| Speed: profiling build, hot spots, Blade/Prettier batching, Mago comparison, scoreboard | perf agent (target-perf/) | doing |
| M1.5 PSR-12: Pint psr12 preset + @PSR1/@PSR2/@PSR12, parity 0 on all 7 projects + 7 fuzz seeds | - | done |
| M1.4 Delegate mode: unported/custom fixers via PHP, `--native-only` (crates/pun-delegate) | delegate agent (target-delegate/) | doing |
| M1.6 `--watch` + formatting-only LSP | - | todo |
| Verify --dirty/--diff/--repair/--parallel/--staged/--since/--stdin against Pint | flags agent (target-flags/) | doing |
| Cleanup: /code-review --fix, architecture pass, remove restating comments, cargo fmt, clippy (59); clap: `ValueEnum` for `--engine` and `config --format` instead of string lists + manual mapping (keep `--format`/`--output-format` as strings: Pint prints its own "Format [x] is not supported." box) | - | todo (after the doing rows) |
| CI workflows (.github/ untracked: gh token lacks `workflow` scope) | user | blocked |
| Publish v0.1 (tag, LICENSE, crates, npm @punphp, Homebrew, Packagist, public repo) | user | needs approval |
| After v0.1 ships: make pun-harness local dev tooling, not a shipped crate. Move to `tools/harness` (or an `xtask`), `publish = false`, drop it from release/dist. Move `crates/pun-harness/fixtures/` to `crates/pun-fixers/tests/fixtures/` (pun-fixers tests read them). Keep the CI parity job calling it | - | todo (after release) |

### Later phases

Phases 2–7 (lint/PHPStan, refactor/Rector, migrate, install/Composer, run/FrankenPHP, test/Pest): todo.
Embedded static libphp (pun-php): deferred, system `php` stands in.

### Open questions for the user

- Drop the `--engine=pun` (Mago formatter) path? ("remove unnecessary things about formatter")
- "creat a r": release, README or PR?
