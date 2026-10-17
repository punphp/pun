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
| Speed: profiling build, hot spots, Prettier worker per thread, Mago comparison, scoreboard (laravel-framework 3.7 s cold / 0.07 s warm) | - | done |
| M1.5 PSR-12: Pint psr12 preset + @PSR1/@PSR2/@PSR12, parity 0 on all 7 projects + 7 fuzz seeds | - | done |
| M1.4 Delegate mode: unported/custom fixers via PHP workers, `--native-only` (crates/pun-delegate); laravel-framework own config 0 differing | - | done |
| laravel-framework own pint.json fully native: port its 8 unported rules, native `// @php-cs-fixer-ignore`, cover php_diff.rs:163 (harness 0 differing, 5 fuzz seeds 0) | - | done |
| M1.6 `pun format --watch` + `pun lsp` (formatting-only) + Pint error boxes in pint mode | - | done |
| Verify --dirty/--diff/--repair/--parallel/--staged/--since/--stdin against Pint (crates/pun-cli/tests/flags.rs; `--since` added; stdin+`--dirty`/`--diff` intentionally does not format repo files) | - | done |
| Agent-optimised output for every command (PLAN "Agent-optimised output"): shared detection (agent-detector v2 list, PUN_AGENT, --agent/--no-agent, PAO_FORCE/PAO_DISABLE), compact compiler-style line format (not JSON), capped lists | - | done (agent-detector 0.2.1 env-only + overrides; summary line + capped findings for format/lint/check/migrate/config, one-line errors; tests/agent.rs; parity 0 differing on all 7 corpus projects; coverage punphp 100%; clippy 0; laravel-framework 3.9 s cold / 0.05 s warm vs 3.9 s / 0.06 s for the previous build on the same loaded box, profiling build. Open: `pun lint --mago` (feature-gated) and `pun format --watch` keep their own output) |
| Config rework: pun.toml only (no legacy reads, no `pint` alias), Pun rule IDs from rules/mapping.toml, `pun migrate` format part, `pun check`, risky fixers moved to lint, mapping completeness test; harness runs `pun migrate` then `pun check --fix` vs Pint | config agent (target-config/) | done (harness `pun migrate` + `pun check --fix`: 0 differing on all 7 corpus projects; coverage 100% on punphp, config, delegate, format, harness; clippy 0; laravel-framework 3.5 s cold / 0.06 s warm, profiling build. Open: `@pun-ignore-file` not honoured in delegate mode, .php-cs-fixer.php Finder not migrated, `--fix` does not report non-apply lint rules, Pun/ mapping keys rename in REQUESTS.md) |
| Interactive pun migrate: detect pint/php-cs-fixer/phpstan/rector/mago, multi-select (TTY, not agent mode), `--from`, `--yes`, per-source summary, starter pun.toml when nothing found (crates/pun-cli/src/migrate.rs) | migrate agent (target-migrate/) | done (harness 0 differing on all 7; coverage 100% punphp-cli migrate.rs + pun-config; clippy 0; dialoguer 0.12 no default features: +0.3 MB stripped debug. Open: PHPStan/Rector/Mago migration not started (PHPStan level -> explicit `[analyse]` rules needs per-rule level data mapping.toml lacks, and pun-cli doesn't link pun-phpstan); top-level `preset` tier is validated by pun-config but not applied yet: once tiers apply, the starter's `preset = "recommended"` adds lint rules and breaks harness parity on laravel-laravel/larastan (no pint.json)) |
| CLI polish: open items after config/agent-output/migrate | cli-polish agent (target-polish/) | doing |
| Naming pass on tentative lint/analyse names (docs/RULES.md §3, scripts/check-rule-mapping.py) | naming agent | done |
| Cleanup: /code-review --fix, architecture pass, remove restating comments, cargo fmt, clippy (59); clap: `ValueEnum` for `--engine` and `config --format` instead of string lists + manual mapping (keep `--format`/`--output-format` as strings: Pint prints its own "Format [x] is not supported." box) | cleanup agent (target-cleanup/) | done (Mago formatter removed instead of the `--engine` enum; laravel-framework 3.75 s cold / 0.07 s warm, profiling build) |
| CRLF fuzz diff + native framework benchmark: seed-2 Process.php diff was a stale target/debug/pun (fuzz-parity.sh now refuses stale binaries); crlf + mixed-eol fuzz 16 runs 0 differing; scoreboard re-measured with own pint.json, no flags | - | done |
| Pint-mode polish: Pun-only flags rejected in pint mode, -v exception boxes (class + throw site; PHP frames dropped), --bail abort progress, list pint.json, --preset=nope, LSP config reload, per-file fixer panics, watch exits with its parent, one Node probe (crates/pun-cli, pun-config) | - | done |
| CI workflows pushed (tests, coverage gate, parity, fuzz, cargo-dist on tags only) | - | done |
| Token text as spans (PLAN v0.1 item 2): spans into the shared source buffer instead of one `Vec<u8>` per token (crates/pun-tokens, mechanical pun-fixers edits) | perf agent (target-perf/) | doing |
| Publish: only after format, analyse, lint and migrate all exist (user decision); repo stays private | user | deferred |
| After v0.1 ships: make pun-harness local dev tooling, not a shipped crate. Move to `tools/harness` (or an `xtask`), `publish = false`, drop it from release/dist. Move `crates/pun-harness/fixtures/` to `crates/pun-fixers/tests/fixtures/` (pun-fixers tests read them). Keep the CI parity job calling it | - | todo (after release) |

### Phase 2 (pun lint)

| Task | Owner | Status |
| --- | --- | --- |
| Phase 2 M2.1: PHPStan config + baseline (crates/pun-phpstan, .oracle/phpstan) | lint-config agent (target-lintcfg/) | done |
| Phase 2 M2.2: symbol parity (crates/pun-phpstan) | symbols agent (target-symbols/) | done |
| Phase 2 M2.3: analyse rules, level 0 (crates/pun-phpstan) | analyse agent (target-analyse/) | doing |
| pun lsp = PHPantom + Pun rules (evaluation + prototype): PHPantom embedded in-process, JSON-RPC router (crates/pun-lsp, docs/LSP.md); wiring into `pun lsp` after cleanup, see docs/LSP.md | - | done |
| Rule placement + naming design (docs/RULES.md, crates/pun-config/rules/mapping.toml) | rules-design agent | done |
| Tiered presets (essential/recommended/strict/pedantic): docs/RULES.md §5, `tier` in mapping.toml, checker | presets agent | done |

### Later phases

Phases 2–7 (lint/PHPStan, refactor/Rector, migrate, install/Composer, run/FrankenPHP, test/Pest): todo.
Embedded static libphp (pun-php): deferred, system `php` stands in.

### Decisions

Command layout (user-approved direction, 2026-10-17):

| Command | Does | Parity target |
| --- | --- | --- |
| `pun format` | Fixes layout (Pint / PHP-CS-Fixer rules) | `pint` bytes, output, exit codes |
| `pun lint` / `pun lint --fix` | Reports / applies code rules from `rector.php` (sets, rules, skip). Mago lint rules opt-in under `[lint.mago]`, reported as `mago.*` | `rector process --dry-run` / `rector process` |
| `pun analyse` (alias `analyze`) | Reports type errors, never edits | `phpstan analyse` (+ Larastan) |

- Plan Phase 2 ("pun lint" in docs/PLAN.md) is now `pun analyse`; Phase 3 ("pun refactor") is now `pun lint` with `--fix`.
- `pun lint --fix` does not format afterwards (Rector doesn't); `--format` opt-in flag may run both.
- `pun lint --mago` stays as is.

- One config: `pun.toml` with `[php]`, `[format]`, `[lint]` (from rector.php), `[analyse]` (from phpstan.neon, explicit rules instead of levels; Larastan = `laravel = true`). `pun migrate` writes it, converts the baseline, rewrites `@phpstan-ignore*` / `@php-cs-fixer-ignore` to `@pun-ignore*`, removes old configs and composer deps, and lists unsupported rules up front (`--dry-run`). PHPDoc type tags (`@phpstan-param` etc.) are kept by default; `pun migrate --phpdoc` (opt-in, apps only) rewrites them to `@pun-*`. Pun reads all three forms.
  Open choices: (1) keep reading legacy configs with a "run pun migrate" hint (proposed) or require pun.toml; (2) Pun's own rule names, with `pun explain` mapping (proposed), or upstream names under a prefix.
- `pun lsp` = PHPantom (completion, hover, definitions, references) + Pun (formatting, diagnostics, code actions). Evaluation and prototype in crates/pun-lsp.
- Built-in symbols for `pun analyse` (M2.2): PHPStan's reflection of every built-in (functionMap + deltas, php-8-stubs, its stubs, functionMetadata) is exported once from the pinned PHAR into `crates/pun-phpstan/data/symbols.json` and read by pun-phpstan's own PHPStan type model and `describe()` printer. Not imported into mago-codex's prelude: its types are Mago's (`TUnion`, Mago's printer), so PHPStan's strings would be lost in translation. At M2.3 the data is handed to Mago as a `FileType::Patch` stub over its prelude where Mago's inference needs it; messages always print through pun-phpstan.

### User decisions (2026-10-17)

- Licence: MIT (LICENSE + THIRD_PARTY_NOTICES.md added).
- Release: none until four tools exist: `pun format`, `pun analyse`, `pun lint`, `pun migrate`. No publishing, no tagging.
- Repo stays private.
- **pun.toml only.** Commands no longer read pint.json, .php-cs-fixer.php, phpstan.neon or rector.php. Without pun.toml, Pun stops with "No pun.toml found. Run `pun migrate`." The old-config readers live on only inside `pun migrate` (and the harness, which runs `pun migrate` on a temp copy and then `pun format`; that also tests migrate). The `pint` argv[0] alias mode goes away; `pun format` output stays Pint-identical.
- **Pun's own, modernised rule names.** The upstream-to-Pun mapping is maintained as data (used by `pun migrate`, `pun explain` and a docs page).
- **Remove the Mago-based formatter** (`--engine=pun`, `format_pun`).
- Mago: depend on crates.io releases; no maintainer outreach needed.
- FrankenPHP (plan row 4): my call. Pun links libphp itself first; FrankenPHP is linked in later.
- **Rule placement:** decide hard which rule lives in format, lint or analyse, remove overlaps, take inspiration from other ecosystems (Vite+, Biome, Ruff, Oxc, Deno). Design in docs/RULES.md.

- **Preset tiers accepted:** `essential` < `recommended` (default) < `strict` < `pedantic`, cumulative, one tier for lint+analyse; add-ons `laravel`/`pest`/`phpunit`; no PHPStan-style levels (docs/RULES.md §5).
- **Rules design accepted (docs/RULES.md, all yes):**
  1. Risky fixers move from format to lint. `pun migrate` turns them on with auto-fix, and Pint parity is proven with `pun check --fix`.
  2. Short rule names are accepted in config and ignore comments; output always prints the full ID.
  3. PHPStan core and Larastan identifiers stay in analyse until v1.
  4. Laravel first: Rector's Symfony, Doctrine and downgrade sets are unsupported, and migrate lists them and stops.
  5. Naming pass on the 807 lint and 497 analyse names marked tentative: done (542 lint, 255 analyse rules; RULES.md §3).
  `pun check` / `pun check --fix` will exist.


### Overnight queue (autonomous, in order)

1. Finish running rows: perf, rules (laravel-framework native), watch/LSP.
2. Speed: final round, done only when the scoreboard benchmarks Pint, Pun and Mago with and without the cache. Includes Blade batching.
3. Wave 3 cleanup, run alone with no other agents: /code-review --fix, architecture pass, comment cleanup, cargo fmt, clippy to 0, clap ValueEnum.
3b. Perf, structural (after cleanup, alone): token text as spans into the source buffer instead of one Vec<u8> per token (tokenize is ~17%). Re-measure Pint, Pun and Mago.
4. Full gate: 100% coverage on every crate, corpus parity for laravel and psr12 on 7/7 projects, and fuzz.
5. Phase 2 (`pun lint`, PHPStan parity), starting with M2.1 config and baseline.

Later: in-process Blade formatter (Prettier-compatible), the only way to reach 30x Pint on cold Pinkary.

Not autonomous (needs the user): publishing or tagging a release, making the repo public, the gh `workflow` scope.
