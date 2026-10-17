# Pun task board

One row per piece of work, grouped by component (PLAN.md C0-C14). Status: todo / doing / done / blocked.
"doing" rows name the owner (thread or agent) and branch so nobody duplicates work. If a "doing" row has no recent
commits on its branch, treat it as abandoned: check its uncommitted files, run the gate, then commit or revert.

History before the 2026-10-17 reset (every finished Phase 1 row, with its evidence): archive/TASKS-2026-10-17.md.

## Current state

| Component | Task | Owner | Status |
| --- | --- | --- | --- |
| C0 | Repo cleanup, context/ folder, component plan | plan-reset thread (claude/repo-cleanup-plan-reset-wu15p3) | doing |
| C0 | Get CI on main green (run 56 failed; 54-55 cancelled) | - | todo |
| C0 | Clippy gate in CI (`-D warnings`) | - | todo |
| C0 | Coverage: pun-analyse to 100% (97.59% in CI); regenerate reports/COVERAGE.md | - | todo |
| C1 | Token text as spans into the source buffer | - | todo |
| C2 | Swap `vendor/bin/pint` for Pun in Pinkary's CI | - | todo |
| C2 | 8-core benchmark in CI (laravel-framework under 1 s cold) | - | todo |
| C3 | `[analyse]` section in pun.toml | - | todo |
| C3 | `pun explain <rule>` and the "Coming from" docs page | - | todo |
| C3 | Rename the 17 `Pun/*` mapping keys; expose `[rules.lint]` metadata | - | todo |
| C6 | `@pun-ignore-file` in delegate mode | - | todo |
| C7 | M2.3 analyse rules, level 0 (52 identifiers so far, gaps.txt) | analyse agent (Amp, pushing to main) | doing |
| C7 | `pun analyse` subcommand, agent output, join `pun check` | - | todo |
| C9 | Interactive `pun migrate` (detect tools, multi-select, `--from`, `--yes`) | - | todo (earlier "doing" row left no code) |
| C9 | Migrate `.php-cs-fixer.php` Finder; `--fix` reports non-apply lint rules | - | todo |
| C10 | Wire `pun lsp` to pun-lsp; extract the shared format engine | - | todo |
| C11 | pun-harness as dev-only tooling; fixtures into pun-fixers | - | todo (after release) |
| C13 | Publish | user | deferred until format, analyse, lint and migrate exist |

The rest of each component (Larastan, levels 1-10, Rector printer and rules, migrate for PHPStan / Rector, LSP
diagnostics) gets rows once we've walked through that component in PLAN.md.
