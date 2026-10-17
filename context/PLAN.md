# Pun plan, by component

Reset on 2026-10-17. This replaces the phase-based plan (now `archive/PLAN-2026-10-17.md`, kept for its detail on
later phases, edge cases and risks). Goal and principles: the project description (README.md, "About Pun"). Decisions
already made: DECISIONS.md. Task board: TASKS.md.

Each component below is a unit one thread can own end to end: its crates, its tests, its gate. The status line says
what the code shows today, checked against the repo at `6e5cc89` / `226fc2e` (main, 2026-10-17), not what older notes
claimed. We go through the components one at a time and rewrite each section as we agree on it.

## Status at a glance

| # | Component | Crates / paths | State | Biggest gap |
| --- | --- | --- | --- | --- |
| C0 | Foundations: workspace, CI, gates, process | Cargo.toml, .github, scripts/coverage.sh, AGENTS.md | Mostly done | CI on main is red or cancelled on recent pushes; clippy not gating; parallel-thread workflow not set up |
| C1 | Token layer | pun-tokens | Done | Token text as spans (perf) |
| C2 | Format engine and fixers | pun-format, pun-fixers | Done (Pint-identical) | Pinkary CI swap; native Blade; 8-core benchmark |
| C3 | Config and rules model | pun-config, rules/mapping.toml | Done for format + lint | `[analyse]` section; `pun explain`; mapping key rename |
| C4 | CLI and output | pun-cli | Done for format, lint, check, migrate, config | `pun analyse` and `pun explain` subcommands |
| C5 | Workspace: discovery, git, cache | pun-workspace | Done | None known |
| C6 | Delegate mode and embedded PHP | pun-delegate, (pun-php) | Delegate done on system PHP; pun-php not started | Embedded libphp; pinned PHARs |
| C7 | Analyse (PHPStan + Larastan) | pun-phpstan, pun-analyse, (pun-laravel) | In progress, actively worked on main | 52 level-0 identifiers ported; not wired into the CLI; Larastan, levels 1-10 not started |
| C8 | Lint (Rector) | pun-cli lint, (pun-refactor) | Started (CS-Fixer risky rules + 17 Pun rules) | Everything Rector: config, printer, rules (31 of 542 lint rules ported) |
| C9 | Migrate | pun-cli migrate, pun-config migrate | Format part done | PHPStan, Rector, baseline, ignore comments, composer/CI rewrites, interactive mode |
| C10 | Editor (pun lsp) | pun-lsp, pun-cli lsp | Formatting-only `pun lsp` shipped; PHPantom prototype not wired | Wiring, shared engine, diagnostics |
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
- CI health: run 56 on main failed and 54-55 were cancelled (2026-10-17); find out why and get main green.
- Clippy gate: drop `continue-on-error`, add `-D warnings` (REQUESTS.md).
- Coverage gate is red: CI measures every crate at 100% except pun-analyse at 97.59% (2066/2117 lines, run 39015344878),
  the crate under active work. The committed reports/COVERAGE.md table is older and lists neither pun-analyse nor pun-lsp.
- Parallel work: AGENTS.md is written for agents sharing one 2-core box and one git index. With several cloud threads,
  each thread works on its own branch and PR per component; TASKS.md rows name the thread. Decide this together (Q1).

## C1. Token layer (pun-tokens)

**Goal.** PHP-CS-Fixer's `Tokens` model in Rust, identical to `token_get_all` for the target PHP version.

**Done.** Own lexer (not mago-syntax), Transformers and custom `CT::T_*` kinds, token analyzers; syntax errors from PHP's
own grammar (bison tables from php-src 8.5.11, 788 oracle snippets). 100% coverage. Details: reference/tokens.md.

**Pending.** Token text as spans into the source buffer instead of one `Vec<u8>` per token (tokenize is about 17% of a
cold run). Pure speed work; no behaviour change.

## C2. Format engine and fixers (pun-format, pun-fixers)

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

**Goal.** One `pun.toml`; every upstream rule mapped to exactly one Pun rule; tiers instead of levels.

**Done.**
- pun.toml only, `[format]` and `[lint]` sections, presets, rule aliases (short names), mapping completeness test.
- Rules design and naming pass (RULES.md): 1,005 Pun rules (208 format, 542 lint, 255 analyse) from 2,564 upstream
  entries; tiers `essential` / `recommended` / `strict` / `pedantic` in mapping.toml.

**Pending.**
- `[analyse]` section (paths, baseline, rule overrides) for C7.
- `pun explain <rule>` (upstream names and doc links), and the generated "Coming from Pint / PHPStan / Rector" docs page.
- Mapping key rename for the 17 `Pun/*` entries; expose `[rules.lint]` metadata as an API (REQUESTS.md).

## C4. CLI and output (pun-cli)

**Goal.** One `pun` binary whose commands match the tools they replace, with agent-optimised output.

**Done.**
- `pun format` (every Pint flag and reporter, `--dirty`, `--diff`, `--staged`, `--since`, `--stdin`, `--watch`,
  `--explain`, `--native-only`, cache), `pun lint`, `pun check [--fix]`, `pun migrate`, `pun config`, `pun lsp`
  (formatting only).
- Agent-optimised output for every command (7fd15f4, 4d47eaa); see the spec below. `pun lint --mago` and
  `pun format --watch` still print their own output.

**Pending.** `pun analyse` subcommand (C7) and joining it into `pun check`; `pun explain` (C3).

### Agent-optimised output

Under a coding agent (laravel/agent-detector v2 env list, `PUN_AGENT=1|0`, `--agent` / `--no-agent`, `PAO_FORCE` /
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
skips them and lists them; custom fixer classes in `.php-cs-fixer.php` work (reference/oracle.md "Delegate mode").

**Pending.**
- pun-php: static libphp (ZTS) behind a Rust SAPI, so delegation needs no system PHP. Not started; deferred by decision.
- Pinned, checksum-verified PHARs instead of the project's `vendor/` Pint or `$PUN_PINT`.
- Known gap: `@pun-ignore-file` is not honoured in delegate mode.
- Delegation for analyse (whole run through real PHPStan) and lint (Rector after native rules).
- Decide whether the release needs pun-php or can require system PHP for delegated rules (Q3).

## C7. Analyse (pun-phpstan, pun-analyse, pun-laravel)

**Goal.** `pun analyse` reports PHPStan + Larastan's exact error set (file, line, identifier, message), honouring
baselines and `ignoreErrors`, so CI can swap with no baseline churn.

**Done.**
- M2.1 config: NEON parser, includes, parameters, schema validation, baseline read/write, `@phpstan-ignore` handling
  (pun-phpstan, fixtures exported from the real PHAR).
- M2.2 symbols: PHPStan's reflection of every built-in in `data/symbols.json`, PHPStan's type model and `describe()`.
- M2.3 level 0, in progress: 52 identifiers in `pun_analyse::IDENTIFIERS` (calls, instantiation, class names, members,
  params, scope) checked against PHPStan's own rule tests; differing cases listed in
  `tests/fixtures/rules/gaps.txt`; corpus diff script `scripts/analyse-corpus.py`. Another agent is pushing this to
  main right now (226fc2e, 07:42 UTC).

**Pending.**
- Finish level 0 and close gaps.txt; corpus parity at level 0.
- Wire it up: `pun analyse` subcommand, `[analyse]` in pun.toml (C3), agent output, `pun check`.
- M2.4 Larastan natively (pun-laravel: models, casts, relations, facades, container, macros). Not started.
- M2.5 Pest plugin; M2.6 levels 1-10, strict-rules, deprecation-rules; M2.7 incremental and LSP diagnostics.
- Coverage: pun-analyse at 97.59% in CI (pun-phpstan is at 100%).
- Decide the release bar: which levels and extensions must be native (Q2).

## C8. Lint (Rector parity)

**Goal.** `pun lint` reports and `pun lint --fix` writes exactly what `rector process --dry-run` / `rector process` do
for the project's rules, plus CS-Fixer's risky rules and opt-in Mago rules.

**Done.** `pun lint` runs the ported risky PHP-CS-Fixer rules (RULES.md moves all 76 out of format) and the 17 Pun/Mago
rules: 31 of 542 lint rules have `ported = true`; `pun lint --mago` runs 5 Mago Laravel rules behind the `mago-lint` feature.

**Pending (all of Rector).**
- Rector config through `pun migrate` (rector.php: paths, skip, rules, sets, prepared sets, PHP sets).
- A format-preserving printer that matches nikic/php-parser (round-trip identical on the corpus, then php-parser's
  fixtures). This is the hard core of the component.
- Type queries through C7, so type-aware rules see what Rector sees from PHPStan.
- Rules in usage order: Pinkary's rector.php first, then prepared sets, PHP 8.4+ sets, rector-laravel, Pest's Rector
  plugin, PHPUnit sets, each with Rector's own fixtures passing.
- Delegation of unported rules to real Rector; `pun lint --set php85`.
- Decide the release bar: which rule sets must be native (Q2).

## C9. Migrate (pun migrate)

**Goal.** One command moves a project onto pun.toml and proves nothing changed.

**Done.** Format part: reads pint.json or `.php-cs-fixer(.dist).php`, writes `[format]` as a delta to the preset plus
risky fixers as `[lint.rules]` with `fix = "apply"`, rewrites `@php-cs-fixer-ignore` to `@pun-ignore-file`, stops on
unsupported rules, is idempotent; the harness proves it (`pun migrate` + `pun check --fix` = Pint on all 7 projects).

**Pending.**
- phpstan.neon to `[analyse]` (levels expanded to explicit rules), baseline to pun-baseline.toml, `@phpstan-ignore*`
  rewrites. Needs C7's config.
- rector.php to `[lint]`. Needs C8's config.
- Interactive mode (detect tools, multi-select, `--from`, `--yes`, starter pun.toml): the task row said "doing", but no
  code landed; treat as not started.
- `.php-cs-fixer.php` Finder not migrated; `--fix` doesn't report non-apply lint rules.
- Workflow switch: composer scripts, CI steps, pre-commit hook, editor settings, Laravel Boost guideline, `--remove-tools`.

## C10. Editor (pun lsp)

**Goal.** One language server: PHPantom for code intelligence, Pun for formatting, diagnostics and fixes.

**Done.** `pun lsp` serves Pint-identical formatting. `crates/pun-lsp` embeds PHPantom in-process with a JSON-RPC router,
merged capabilities, syntax-error and Mago diagnostics, fix-all; 100% coverage on its own (reference/lsp.md).

**Pending.** Wire `pun lsp` to `pun_lsp::run()`; extract the shared formatting engine from pun-cli (REQUESTS.md);
diagnostics from `pun analyse`; quick fixes from `pun lint --fix`; editor setup docs; PHPantom is a git dependency
(blocks crates.io publishing) and grows the binary from about 8 to 47 MB (Q5).

## C11. Parity harness, corpus and fuzz (pun-harness, scripts/)

**Goal.** Every parity and speed claim is reproducible against the pinned real tools.

**Done.** `pun-harness format` on 7 corpus projects writes reports/scoreboard.md; `scripts/fuzz-parity.sh` mutation
fuzzer; `scripts/cli-parity.sh`; exporters that pull fixtures from the real tools (reference/oracle.md).

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
2. **Release bar for analyse and lint (C7, C8).** Which PHPStan levels and extensions, and which Rector sets, must be
   native before the release, and which may be delegated?
3. **Embedded PHP (C6).** May the release require a system `php` for delegated rules, or is pun-php a release blocker?
4. **Unported format rules (C2).** Are the 64 delegated format rules acceptable at release, or which must be native?
5. **LSP dependency (C10).** Ship with PHPantom as a git dependency (and the 47 MB binary), or wait for its release?
6. **1.0 scope (C14).** Memory says one 1.0 release where everything ships at once; the task board says the release needs
   format, analyse, lint and migrate. Are install, run and test part of 1.0 or after it?

## What the old notes got wrong (fixed by this reset)

- The old plan's summary still described two formatting engines and an embedded PHP "from day one"; the Mago formatter
  was removed and libphp is deferred.
- "Decisions for you to confirm" still proposed shipfastlabs as the home; it isn't.
- "Config compatibility" said Pun reads old configs directly and pun.toml is optional; it is now pun.toml only.
- The crate list still had `pint` / `phpstan` / `rector` argv[0] aliases; the `pint` alias is gone.
- Phase numbering (Phase 2 = lint, Phase 3 = refactor) was renamed twice; components replace phases.
- TASKS.md had two "doing" rows: agent output (it landed in 7fd15f4) and interactive migrate (no code landed).
- AGENTS.md said the current scope was Phase 0 + Phase 1 only.
