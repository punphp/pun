# Pun task board

One row per piece of work, grouped by the build steps in PLAN.md §3; detail in each plan/ file's Build section. Status: todo / doing / done / blocked.
"doing" rows name the owner (thread or agent) and branch so nobody duplicates work. If a "doing" row has no recent
commits on its branch, treat it as abandoned: check its uncommitted files, run the gate, then commit or revert.

History before the 2026-10-17 reset (every finished Phase 1 row, with its evidence): archive/TASKS-2026-10-17.md.

## Done

| Plan | Task | Owner | Status |
| --- | --- | --- | --- |
| foundations | Repo cleanup, context/ folder, plan walkthroughs (format, lint, analyse, init, output, migrate) | plan-reset thread | done |

## Step 1. Names and config (plan/config.md Build)

| Plan | Task | Owner | Status |
| --- | --- | --- | --- |
| config 1 | New `<command>/<group>/<name>` IDs, group list and sets in mapping.toml; no aliases | - | todo |
| config 2 | Move the 40 code-deleting format rules to lint | - | todo |
| config 3 | New pun.toml schema (`[files]`, presets, sets, levels, groups, overrides, `[analyse]`) | - | todo |
| config 4 | TOML-native option names for format rules | - | todo |
| config 5 | PHP version and ecosystems from composer.json / composer.lock | - | todo |
| config 6 | Remove delegate mode (pun-delegate, `--native-only`); update AGENTS.md | - | todo |
| config 7 | `[files]` + per-command exclude, `pun list-files` | - | todo |
| config 8 | `pun config` on the new schema, `pun explain <rule>` | - | todo |

## Step 2. Shared output (plan/output.md Build)

| Plan | Task | Owner | Status |
| --- | --- | --- | --- |
| output 1-2 | Renderer and the look (brand colours, fallbacks, NO_COLOR, links) | - | todo |
| output 3 | `pun format` on the renderer; Pint's human screen removed | - | todo |
| output 4-6 | GitHub annotations, `pun check` grouped view, exit code 2 | - | todo |

## Step 3, Track A. Format, migrate, init

| Plan | Task | Owner | Status |
| --- | --- | --- | --- |
| format 1 | Presets cut to laravel, psr12, empty | - | todo |
| format 3 | Fewer allocations per token (inline Content type; first attempt stopped, nothing landed) | - | todo |
| format 4-5 | Memory target; 8-core benchmark in CI (laravel-framework under 1 s cold) | - | todo |
| format 6 | Swap `vendor/bin/pint` for Pun in Pinkary's CI | - | todo |
| format 7 | Port the remaining format rules (144 of 208 native) | - | todo |
| migrate 1-3 | Format part on the new names, the Finder, unsupported list with the PR request | - | todo |
| init 1-5 | `pun init` | - | todo |

## Step 3, Track B. Lint (plan/lint.md Build)

| Plan | Task | Owner | Status |
| --- | --- | --- | --- |
| lint 1-2 | Engine on Mago's tree; levels, sets, overrides at run time | - | todo |
| lint 3-6 | Token-path diagnostics, ignore comments, output, Mago rules folded in | - | todo |
| lint 7 | All 542 rules native, in usage order | - | todo |
| migrate 4 | rector.php | - | todo |

## Step 3, Track C. Analyse (plan/analyse.md Build)

| Plan | Task | Owner | Status |
| --- | --- | --- | --- |
| analyse 1-2 | `pun analyse` command, `[analyse]`, presets, output; Pun baseline | - | todo |
| analyse 3 | Finish level 0: ~100 rules left, 40 gaps, corpus diffs 1/6/5/15 | analyse agent | todo (stopped at 43d9857; trait-method and facade-alias WIP uncommitted) |
| analyse 4-5 | Type inference; levels 1-10 as the five presets; strict-rules, deprecation rules | - | todo |
| analyse 6-7 | Larastan with the seven improvements; Pest | - | todo |
| analyse 8 | Result cache | - | todo |
| migrate 5-6 | phpstan.neon; offer the closest preset | - | todo |

## Step 4. `pun check` (plan/commands.md §6)

| Plan | Task | Owner | Status |
| --- | --- | --- | --- |
| check 1-5 | One parse per file, analyse joins, `--fix` loop | - | todo |

## Foundations (plan/foundations.md)

| Plan | Task | Owner | Status |
| --- | --- | --- | --- |
| CI | Get CI on main green (coverage red; runs cancelled by the next push) | - | todo |
| CI | Clippy gate in CI (`-D warnings`) | - | todo |
| CI | Coverage: pun-analyse to 100% (97.59% in CI); regenerate reports/COVERAGE.md | - | todo |
| CI | One branch and PR per thread; rewrite AGENTS.md "Work with other agents" | - | todo |
| CI | CLI polish item 7 (`script` check in CI, skip message in tests/migrate.rs) | - | todo (stopped by user) |
| harness | Lint and analyse modes; pun-harness dev-only; fixtures into pun-fixers | - | todo |
| release | Publish 1.0 | user | after Step 4 |
| later | LSP; install, run, test; native Blade; extension system | - | after 1.0 |
