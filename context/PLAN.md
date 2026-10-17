# Pun plan, by component

Reset on 2026-10-17. This replaces the phase-based plan (now `archive/PLAN-2026-10-17.md`, kept for its detail on
later phases, edge cases and risks). Goal and principles: the project description (README.md, "About Pun"). Decisions
already made: DECISIONS.md. Task board: TASKS.md.

Each component below is a unit one thread can own end to end: its crates, its tests, its gate. The status line says
what the code shows today, last rechecked against main at `396c4af` (2026-10-17 09:30 UTC), not what older notes
claimed. We go through the components one at a time; each one gets its own detailed file in `context/components/` (start:
`components/format.md`), and its section here shrinks to a summary and a link once we agree on it.

## The decided design (walked through with Pushpak, 2026-10-17)

The one-screen summary of what Pun is going to be. Detail and reasons: `components/commands.md` (who does what),
`components/format.md`, `components/lint.md`, `components/analyse.md`; every decision with its date is in DECISIONS.md.

**Commands: one job each.**
- `pun format`: how the code looks. Layout, casing and spelling only; never changes behaviour and never deletes code.
- `pun lint [--fix]`: is there a better way to write this? Rector, PHP-CS-Fixer's risky fixers, Mago's lint rules, and
  version upgrades (as lint groups; there is no `pun upgrade`).
- `pun analyse`: is this wrong? PHPStan, Larastan and the Pest plugin; it never edits.
- `pun check [--fix]`: all three in one pass (lint fixes, then format, then analyse); the command for CI and agents.
- `pun migrate`: moves pint.json, .php-cs-fixer.php, rector.php and phpstan.neon onto pun.toml completely, and names
  anything not supported yet. It never copies a baseline.
- `pun init`: a new project's pun.toml. It asks three questions: format preset, lint sets, analyse preset.
- No LSP until the last step, after the embedded PHP interpreter (removed from the repo for now).

**Config: pun.toml only.** No pun.toml means stop, with "run `pun migrate`, or `pun init` for a new project".

| | Format | Lint | Analyse |
| --- | --- | --- | --- |
| Preset | `laravel` (default), `psr12`, `empty` | a checklist of sets (`[lint] sets = ["dead-code", "modernize", ...]`), like Rector's prepared sets | `basic` < `essential` < `recommended` (default) < `strict` < `max`, by PHPStan level band |
| Rule names | `format/<group>/<name>` | `lint/<group>/<name>`; one group per upgrade target (`lint/laravel-13/...`) | `analyse/<group>/<name>` |
| Levels | none: a diff is a diff | oxlint: `"off"`, `"warn"`, `"error"` per rule or group; only `error` fails | on or off; every finding is an error |
| Fixes | always applied | `--fix` applies every fix of every enabled rule; `fix = "off"` keeps a rule report-only | never |
| Rule options | TOML-native, not copied from PHP-CS-Fixer | the same | the same |

- One group list is shared by all three commands (topic and ecosystem groups). There are no aliases: renames are
  breaking until 1.0.
- Laravel, Pest and PHPUnit support is built in and turns on by itself from composer.lock, including version-specific
  rules. There is no extension system before 1.0.
- `[php] version` comes from composer.json; the config value is only an override.

**Engines.**
- Format: the PHP-CS-Fixer token-fixer port. `pun check --fix` equals `pint`. 40 code-deleting fixers move to lint.
- Lint: text edits on Mago's syntax tree, then format tidies them. There is no php-parser printer port.
- Analyse: Mago's analyser for inference, with Pun rules mapped one to one from PHPStan's identifiers, and Laravel and
  Pest as built-in Mago plugins. Baselines are Pun's own (`pun analyse --generate-baseline`).

**Output.** Pun's own modern, coloured output (format.md §10). Machine formats stay Pint and PHPStan compatible, and
agent mode prints compact lines. The output for every command is designed in components/output.md.

**Release bar for 1.0.** Everything ships at once. Every lint rule (542) is native. Every PHPStan core rule is native.
Larastan is built in, plus seven improvements on it (typed config, routes, views, translations, env keys, Inertia
pages, fresh model data). There is a result cache, and speed and memory come first. Unported format rules (Q7 of
format.md) are still open.

## Status at a glance

| # | Component | Crates / paths | State | Biggest gap |
| --- | --- | --- | --- | --- |
| C0 | Foundations: workspace, CI, gates, process | Cargo.toml, .github, scripts/coverage.sh, AGENTS.md | Mostly done | No CI run on main has finished green today; coverage red (pun-analyse); parallel-thread workflow not set up |
| C1 | Token layer | pun-tokens | Done | Fewer allocations per token (perf; first attempt stopped) |
| C2 | Format engine and fixers | pun-format, pun-fixers | Done (Pint-identical) | Pinkary CI swap; native Blade; 8-core benchmark |
| C3 | Config and rules model | pun-config, rules/mapping.toml | Done for format + lint | Tiers accepted but not applied; `[analyse]` section; `pun explain` |
| C4 | CLI and output | pun-cli | Done for format, lint, check, migrate, config | `pun analyse` and `pun explain` subcommands |
| C5 | Workspace: discovery, git, cache | pun-workspace | Done | None known |
| C6 | Delegate mode and embedded PHP | pun-delegate, (pun-php) | Delegate done on system PHP (incl. `@pun-ignore-file`); pun-php not started | Embedded libphp; pinned PHARs |
| C7 | Analyse (PHPStan + Larastan) | pun-phpstan, pun-analyse, (pun-laravel) | In progress, actively worked on main | 54 identifiers, about 100 level-0 rules left; not in the CLI; Larastan, levels 1-10 not started |
| C8 | Lint (Rector) | pun-cli lint, (pun-refactor) | Started (CS-Fixer risky rules + 17 Pun rules) | Everything Rector: config, printer, rules (31 of 542 lint rules ported) |
| C9 | Migrate | pun-cli migrate, pun-config migrate | Format part and interactive picker done | PHPStan, Rector, baseline, ignore comments, composer/CI rewrites |
| C10 | Editor (LSP) | - | Removed; last step, after the PHP interpreter (user, 2026-10-17) | Everything, when its turn comes |
| C11 | Parity harness, corpus, fuzz | pun-harness, scripts/, corpus/ | Done for format | Analyse and lint harness; move harness out of shipped crates |
| C12 | Docs site and brand | docs/, design/ | Built | Keep in sync as commands land |
| C13 | Release and distribution | dist-workspace.toml, packaging/, release.yml | Configured, not published | Gated on C7, C8, C9 (decision) |
| C14 | Later: install, run, test | (pun-pm, pun-serve, pun-test) | Not started | Scope question: in 1.0 or after? |

## C0. Foundations: workspace, CI, gates, process

**Goal.** A workspace where several threads can work at once without stepping on each other, and every claim is gated.

**Done.**
- Cargo workspace, 11 crates, `punphp-<x>` package names, Mago pinned at `=1.57.0`.
- CI (`.github/workflows/ci.yml`): tests, rustfmt gate, coverage gate (`scripts/coverage.sh`, 100% per crate), parity,
  fuzz; cargo-dist release workflow on tags only.
- Oracle pinned in `.oracle/` via `scripts/oracle.sh` (reference/oracle.md); corpus of 7 projects (`corpus/manifest.toml`).

**Pending.**
- CI health: the last finished runs on main failed (coverage) and the newest ones were cancelled by the next push
  (runs 70-72): with agents pushing every few minutes, no run completes. Per-component PRs would fix both.
- Clippy: the job fails on errors but warnings don't gate yet; add `-- -D warnings` (REQUESTS.md).
- CLI polish item 7: a `script` check in CI and the skip message in tests/migrate.rs; then the full gates under
  `CLAUDECODE=1` (TASKS.md).
- Coverage gate is red: CI measures every crate at 100% except pun-analyse at 97.59% (2066/2117 lines, run 39015344878),
  the crate under active work. The committed reports/COVERAGE.md table is older and does not list pun-analyse.
- Parallel work: AGENTS.md is written for agents sharing one 2-core box and one git index. With several cloud threads,
  each thread works on its own branch and PR per component; TASKS.md rows name the thread. Decide this together (Q1).

## C1. Token layer (pun-tokens)

**Goal.** PHP-CS-Fixer's `Tokens` model in Rust, identical to `token_get_all` for the target PHP version.

**Done.** Own lexer (not mago-syntax), Transformers and custom `CT::T_*` kinds, token analyzers; syntax errors from PHP's
own grammar (bison tables from php-src 8.5.11, 788 oracle snippets). 100% coverage. Details: reference/tokens.md.

**Pending.** Fewer allocations per token. A first attempt (2026-10-17) was stopped with nothing landed; its profile
changed the approach: a cold laravel-framework run makes 45M allocations, tokenizing is 0.87 s of it (lexing only 0.18 s,
transformers 0.5 s), and 98% of tokens are 22 bytes or less. So instead of spans: a small inline `Content` type
(24 bytes, 22 inline, else boxed) as `Token.content`, plus three cheaper fixes (token clones in braces_position and
array_indentation, `rhs.to_vec()` in php_parser.rs, per-token lowercasing in native_type_declaration_casing). The work
in progress lived in a worktree on the agents' box, not in git. Land only if cold time improves by 5% or more.

## C2. Format engine and fixers (pun-format, pun-fixers)

**Detailed plan and open questions: components/format.md** (covers C1, C2, C5 and the format side of C6).

**Goal.** `pun format` byte-identical to Pint 1.29.3 / PHP-CS-Fixer 3.95.17, much faster.

**Done.**
- All 147 Pinkary rules plus `new_with_parentheses` and `Pint/phpdoc_type_annotations_only`; laravel and psr12 presets,
  @PSR1/@PSR2/@PSR12; Blade through Prettier workers like Pint.
- 0 differing files on all 7 corpus projects for laravel and psr12 (reports/scoreboard.md); mutation fuzzer including
  CRLF and mixed line endings: 0 differing; 166-rule end-to-end suite.
- laravel-framework, own pint.json, fully native: 3.7 s cold / 0.07 s warm vs Pint 190 s (2-core profiling build).
- `// @php-cs-fixer-ignore` and `@pun-ignore-file` honoured natively.

**Pending.**
- Exit gate from the old plan: replace `vendor/bin/pint` in Pinkary's CI with Pun.
- 8-core benchmark in CI to prove laravel-framework under 1 s cold.
- Native Blade formatter (Blade is about 2.9 s of Pinkary's 4.2 s cold run). Planned after the first release.
- 144 of 208 format rules are ported (`ported = true` in mapping.toml); the rest run through delegate mode (C6).
  Decide which of the 64 must be native for the release (Q4).

## C3. Config and rules model (pun-config, rules/mapping.toml)

**Goal.** One `pun.toml`; every upstream rule mapped to exactly one Pun rule; presets instead of levels.

**Decided 2026-10-17 (to build):** rule IDs become `<command>/<group>/<name>` with one shared group list and no
aliases (format.md §9 has the format table); lint sets; five analyse presets; oxlint-style lint levels; analyse on/off;
`[php] version` read from composer.json; `pun init` (components/init.md).

**Done.**
- pun.toml only, `[format]` and `[lint]` sections, presets, rule aliases (short names), mapping completeness test.
- Rule metadata API `pun_config::rule()` (fix safety, tier, summary, aliases); the 17 `Pun/*` mapping keys now use the
  fixer names (5f64709).
- Rules design and naming pass (RULES.md): 1,005 Pun rules (208 format, 542 lint, 255 analyse) from 2,564 upstream
  entries; tiers `essential` / `recommended` / `strict` / `pedantic` in mapping.toml.

**Pending.**
- Tiers: the top-level `preset` is accepted and validated but applies nothing yet. Trap for whoever implements it: the
  migrate starter writes `preset = "recommended"`, and the harness runs laravel-laravel and larastan (no pint.json) on
  it, so applying the tier must not add fixes there, or the starter must change (REQUESTS.md).
- `[analyse]` section (paths, baseline, rule overrides) for C7.
- `pun explain <rule>` (upstream names and doc links), and the generated "Coming from Pint / PHPStan / Rector" docs page.

## C4. CLI and output (pun-cli)

**Goal.** One `pun` binary whose commands match the tools they replace, with agent-optimised output.

**Decided 2026-10-17 (to build):** Pun's own human output for every command (components/output.md); `pun init`;
`pun analyse`; `--deny-warnings`, `--max-warnings`, `-A/-W/-D`; `pun lint --mago` goes away.

**Done.**
- `pun format` (every Pint flag and reporter, `--dirty`, `--diff`, `--staged`, `--since`, `--stdin`, `--watch`,
  `--explain`, `--native-only`, cache), `pun lint`, `pun check [--fix]`, `pun migrate`, `pun config`.
- `pun lint --fix` / `pun check --fix` report the lint rules they don't apply as remaining, and exit 1 when any remain.
- Agent-optimised output for every command (7fd15f4, 4d47eaa); see the spec below. `pun lint --mago` and
  `pun format --watch` still print their own output.

**Pending.** `pun analyse` subcommand (C7) and joining it into `pun check`; `pun explain` (C3).

### Agent-optimised output

Under a coding agent (laravel/agent-detector v2 env list and `/opt/.devin`, then the agent-detector crate; Cursor's
`CURSOR_TRACE_ID` alone doesn't count; `PUN_AGENT=1|0`, `--agent` / `--no-agent`, `PAO_FORCE` /
`PAO_DISABLE`) every command prints compiler-style lines, not JSON:

```
pun analyse: fail, 3 errors in 2 files (412 files checked, 0.21s)
app/Models/User.php:42:9 analyse.undefined-method Call to undefined method App\Models\User::foo().
app/Http/Kernel.php:12:1 lint.early-return [fix:safe] Use an early return.
+2 more; run with -v for all
```

Summary line first (a passing run prints only that line); `path:line:col rule message` with an optional
`[fix:safe|unsafe]`; bare diff hunks; no ANSI, progress or boxes; lists capped with `+N more`; errors are one line
(`pun format: error: <message>`); `--format=json` stays available; exit codes don't change. Implemented once in
`crates/pun-cli/src/agent.rs`, pinned per command in `crates/pun-cli/tests/agent.rs`.

## C5. Workspace (pun-workspace)

**Goal.** File discovery that copies Pint's finder rules, git integration and the content-hash cache.

**Done.** Finder rules, `--dirty` / `--staged` / `--since`, xxh3 content cache (signature = version, PHP, rules, Blade
lockfiles). 100% coverage. **Pending:** nothing known; analyse and lint will need their own path rules (PHPStan `paths` /
`excludePaths`, Rector `withPaths` / `withSkip`).

## C6. Delegate mode and embedded PHP (pun-delegate, pun-php)

**Goal.** Anything not ported runs through the real upstream code, so compatibility is complete from day one.

**Done.** Unported or custom fixers send the run through PHP-CS-Fixer in long-lived system `php` workers; `--native-only`
skips them and lists them; custom fixer classes in `.php-cs-fixer.php` work (reference/oracle.md "Delegate mode");
`@pun-ignore-file` is honoured there too (6383a00).

**Pending.**
- pun-php: static libphp (ZTS) behind a Rust SAPI, so delegation needs no system PHP. Not started; deferred by decision.
- Pinned, checksum-verified PHARs instead of the project's `vendor/` Pint or `$PUN_PINT`.
- Delegation for analyse (whole run through real PHPStan) and lint (Rector after native rules).
- Decide whether the release needs pun-php or can require system PHP for delegated rules (Q3).

## C7. Analyse (pun-phpstan, pun-analyse, pun-laravel)

Plan walked through with Pushpak: components/analyse.md (decisions at the end). Engine: Mago's analyser (§8);
Larastan built in plus seven improvements (§9); Pest built in (§10).

**Goal.** `pun analyse` reports PHPStan + Larastan's exact error set (file, line, identifier, message), honouring
baselines and `ignoreErrors`, so CI can swap with no baseline churn.

**Done.**
- M2.1 config: NEON parser, includes, parameters, schema validation, baseline read/write, `@phpstan-ignore` handling
  (pun-phpstan, fixtures exported from the real PHAR).
- M2.2 symbols: PHPStan's reflection of every built-in in `data/symbols.json`, PHPStan's type model and `describe()`.
- M2.3 level 0, in progress (the analyse agent stopped at 43d9857): 54 identifiers in `pun_analyse::IDENTIFIERS`
  (calls, `new`, class names, constants, typehints, properties, `use`), trait contexts, rebound closures, ignores,
  parallel scanning. Checked against 142 of PHPStan's own rule test classes, with 40 differing cases listed in
  `tests/fixtures/rules/gaps.txt`, and on the corpus (`scripts/analyse-corpus.py`): pinkary 1 diff, larastan 6,
  pest 5, laravel-framework 15, at 5-25x PHPStan's speed.

**Pending.**
- Finish level 0: about 100 level-0 rules are still unported; empty gaps.txt; corpus diffs to 0. Larastan facade
  aliases and the trait-method declaring class were uncommitted work in progress when the agent stopped.
- Wire it up: `pun analyse` subcommand, `[analyse]` in pun.toml (C3), agent output, `pun check`.
- M2.4 Larastan natively (pun-laravel: models, casts, relations, facades, container, macros). Not started.
- M2.5 Pest plugin; M2.6 levels 1-10, strict-rules, deprecation-rules; M2.7 incremental and LSP diagnostics.
- Coverage: pun-analyse at 97.59% in CI (pun-phpstan is at 100%).
- Decide the release bar: which levels and extensions must be native (Q2).

## C8. Lint (Rector parity)

Plan walked through with Pushpak: components/lint.md (decisions at the end).

**Goal.** `pun lint` reports and `pun lint --fix` writes exactly what `rector process --dry-run` / `rector process` do
for the project's rules, plus CS-Fixer's risky rules and opt-in Mago rules.

**Done.** `pun lint` runs the ported risky PHP-CS-Fixer rules (RULES.md moves all 76 out of format) and the 17 Pun/Mago
rules: 31 of 542 lint rules have `ported = true`; `pun lint --mago` runs 5 Mago Laravel rules behind the `mago-lint` feature.

**Pending (all of Rector).**
- Rector config through `pun migrate` (rector.php: paths, skip, rules, sets, prepared sets, PHP sets).
- ~~A format-preserving printer that matches nikic/php-parser~~ Decided: text edits on Mago's syntax tree instead,
  parity checked after formatting (lint.md decisions).
- Type queries through C7, so type-aware rules see what Rector sees from PHPStan.
- Rules in usage order: Pinkary's rector.php first, then prepared sets, PHP 8.4+ sets, rector-laravel, Pest's Rector
  plugin, PHPUnit sets, each with Rector's own fixtures passing.
- ~~Delegation of unported rules to real Rector; `pun lint --set php85`~~ Decided: unported rules are named, never
  delegated; upgrades are lint groups.
- Release bar decided: all 542 lint rules native before 1.0.

## C9. Migrate (pun migrate)

**Goal.** One command moves a project onto pun.toml and proves nothing changed.

**Done.**
- Interactive picker (bc6d9a2): detects pint.json, .php-cs-fixer.php, phpstan.neon, rector.php and mago.toml, shows a
  checklist on a terminal, `--from pint,phpstan` and `--yes` skip it (as do agent mode and no terminal), prints one line
  per config; PHPStan, Rector and Mago are listed as "not supported yet". No config at all writes a starter pun.toml.
- Harness: projects without pint.json migrate an explicit `{}` (Pint's defaults), not the starter (7722a6d).
- Format part: reads pint.json or `.php-cs-fixer(.dist).php`, writes `[format]` as a delta to the preset plus
risky fixers as `[lint.rules]` with `fix = "apply"`, rewrites `@php-cs-fixer-ignore` to `@pun-ignore-file`, stops on
unsupported rules, is idempotent; the harness proves it (`pun migrate` + `pun check --fix` = Pint on all 7 projects).

**Pending.**
- phpstan.neon to `[analyse]` (levels expanded to explicit rules), `@phpstan-ignore*` rewrites. The baseline is not
  migrated (decided 2026-10-17): migrate tells the user to run `pun analyse --generate-baseline`. Needs C7's config, plus per-rule level data that mapping.toml doesn't have yet, and pun-cli doesn't link
  pun-phpstan today.
- rector.php to `[lint]`. Needs C8's config.
- `.php-cs-fixer.php` Finder not migrated.
- Workflow switch (composer scripts, CI steps, pre-commit hook, editor settings, Laravel Boost guideline): decided
  2026-10-17 to leave it out of Pun's code; `pun-init` / `pun-migrate` agent skills will do it later. `--remove-tools`
  is still open.

## C10. Editor (LSP)

**Removed (user decision, 2026-10-17).** The LSP is the last step, after the embedded PHP interpreter. The formatting-only
`pun lsp`, the PHPantom prototype (`crates/pun-lsp`) and their docs are gone; git history has them. Nothing in the repo
depends on an LSP or on PHPantom until then.

## C11. Parity harness, corpus and fuzz (pun-harness, scripts/)

**Goal.** Every parity and speed claim is reproducible against the pinned real tools.

**Done.** `pun-harness format` on 7 corpus projects writes reports/scoreboard.md; `scripts/fuzz-parity.sh` mutation
fuzzer; `scripts/cli-parity.sh`; exporters that pull fixtures from the real tools (reference/oracle.md).

**Done since the reset.** Projects without pint.json migrate an explicit `{}` (7722a6d).

**Pending.** Analyse and lint modes in the harness (today analyse uses `scripts/analyse-corpus.py`); make pun-harness
dev-only tooling (`publish = false`, out of dist) and move its fixtures into pun-fixers; benchmark rig in CI.

## C12. Docs site and brand (docs/, design/)

**Goal.** A public site and docs that present Pun as 1.0 and stay true to the code.

**Done.** Blume site (`docs/blume.config.ts`, pages in `docs/content/docs/`, homepage `docs/pages/index.astro`);
Sticker Pop design system in `design/` (tokens, components, mascot, logos). **Pending:** update pages as each command
lands (`.claude/skills/blume-update-docs`); the homepage already advertises analyse, lint and migrate.

## C13. Release and distribution

**Goal.** One install on every platform once the four commands exist.

**Done.** cargo-dist config (`dist-workspace.toml`: shell, npm `@punphp/pun`, Homebrew tap `punphp/homebrew-tap`;
6 targets), Composer shim (`packaging/composer`), release workflow on tags. **Pending:** everything after the gate:
name checks on crates.io / Packagist / npm / Homebrew, signing, publishing. Waits on C7, C8 and C9 by decision.

## C14. Later: pun install, pun run, pun test

The old plan's Phases 5-7 (Composer-compatible installer, `pun run` on FrankenPHP, Pest runner). Nothing is built.
Their detailed design stays in `archive/PLAN-2026-10-17.md` until we decide whether they belong in 1.0 (Q6).

## Open questions

These are the calls we need to make while walking through the components; each one changes scope.

1. **Parallel threads (C0).** One thread per component, each on its own branch with a PR into main, instead of agents
   sharing one box and index. Which components run in parallel first?
2. ~~Release bar for analyse and lint (C7, C8)~~ Decided: every lint rule and every PHPStan core rule native, Larastan
   and Pest built in, nothing delegated.
3. **Embedded PHP (C6).** May the release require a system `php` for delegated rules, or is pun-php a release blocker?
4. **Unported format rules (C2).** Are the 64 delegated format rules acceptable at release, or which must be native?
   Still open (format.md Q7). With presets cut to laravel, psr12 and empty, none of the preset rules is unported.
5. ~~LSP dependency (C10)~~ Removed: the LSP is the last step.
6. **1.0 scope (C14).** Memory says one 1.0 release where everything ships at once; the task board says the release needs
   format, analyse, lint and migrate. Are install, run and test part of 1.0 or after it?

## What the old notes got wrong (fixed by this reset)

- The old plan's summary still described two formatting engines and an embedded PHP "from day one"; the Mago formatter
  was removed and libphp is deferred.
- "Decisions for you to confirm" still proposed shipfastlabs as the home; it isn't.
- "Config compatibility" said Pun reads old configs directly and pun.toml is optional; it is now pun.toml only.
- The crate list still had `pint` / `phpstan` / `rector` argv[0] aliases; the `pint` alias is gone.
- Phase numbering (Phase 2 = lint, Phase 3 = refactor) was renamed twice; components replace phases.
- TASKS.md had stale "doing" rows; agent output and interactive migrate have both landed since (7fd15f4, bc6d9a2).
- AGENTS.md said the current scope was Phase 0 + Phase 1 only.
