# Pun: Implementation Plan

Oct 16, 2026 · @Pushpak · updated Oct 17, 2026 (command layout, one config, editor; Phase 1 status)

## Summary

Pun is a single Rust binary that replaces Pint, PHP-CS-Fixer, PHPStan + Larastan, Rector, Composer and Pest's runner for PHP 8.4+ projects, built on Mago's crates, with output identical to the tools it replaces. Priority order: `pun format` (Pint), `pun lsp` (editor), `pun analyse` (PHPStan), `pun lint` (Rector), `pun migrate` (one-time move to pun.toml), `pun install` (Composer-compatible package manager), `pun run index.php` on FrankenPHP, then `pun test`.

The acceptance test for every phase: run Pun and the original tool on Pinkary (and a fixed corpus of Laravel projects) and get byte-identical output, identical diagnostics, or identical test results.

### The core decisions

1. **Rust, built on Mago crates.** Mago's lexer, parser, AST, name resolver, codex, analyzer, linter, reporting, composer and text-edit crates are MIT/Apache-2.0 and published to crates.io. Pun depends on them as libraries, contributes fixes upstream, and only forks a crate if upstream blocks a compatibility requirement.
2. **Two formatting engines, not one.** Mago's formatter is a Wadler-style pretty-printer that reprints the whole file. Pint is PHP-CS-Fixer, a token fixer that only edits what violates a rule. A pretty-printer can never be byte-identical to a token fixer, which is why Mago's own Pint preset (and tools built on it, like wick) still produce a reformat diff on Pint-formatted code. So Pun's default engine is a Rust port of PHP-CS-Fixer's token model and fixers (running on Mago's lexer), and Mago's pretty-printer is an opt-in "Pun style" engine.
3. **An embedded PHP runtime from day one.** Pun statically links libphp (the same approach FrankenPHP uses) behind a Rust SAPI. It is needed to evaluate PHP config files (`.php-cs-fixer.php`, `rector.php`), boot Laravel apps for type information, run tests, and serve HTTP. It also powers **delegate mode**: any rule, fixer, extension or Rector rule not yet ported runs inside the embedded PHP using the real upstream code. That gives 100% compatibility on day one, with the native share growing release by release.
4. **A differential test harness is the main product asset.** Every change is checked against pinned versions of the real tools on a fixed corpus. Bun's formatter claims parity by passing 99% of Prettier's test suite; Pun does the same with PHP-CS-Fixer's, PHPStan's, Rector's and Pest's own fixtures, plus real-world repos.
5. **Laravel is first class.** A shared `pun-laravel` crate (models, migrations, facades, macros, container bindings, config) feeds the analyzer, the refactor rules and the test runner. Larastan behaviour is a native feature, not an extension.
6. **PHP 8.4+ projects only.** Pun refuses (with a clear message) projects whose composer.json allows PHP below 8.4. This matches Pest 5's own baseline and removes a large class of legacy-syntax edge cases. The parser still accepts all syntax up to 8.5 and tracks 8.6.

&#91;embedded content: Pun roadmap · 8 phases, 7 releases\]

Each phase ends in a release that is useful on its own. With a second team, Phase 7 (`pun test`) can start alongside Phase 2, since the test runner depends only on the embedded runtime and Mago's parser.

## Update, Oct 17: command layout, one config, editor

These decisions replace earlier wording where they conflict.

### Commands

| Command | Does | Replaces | Parity target |
| --- | --- | --- | --- |
| `pun format` | Fixes layout (Pint / PHP-CS-Fixer rules) | `pint` | Bytes, output and exit codes of `pint` |
| `pun lint`, `pun lint --fix` | Reports / applies code rules (Rector rules; Mago lint rules opt-in under `[lint.mago]`, reported as `mago.*`) | `rector process --dry-run`, `rector process` | Rector's diff and written files |
| `pun analyse` (alias `analyze`) | Reports type errors, never edits | `phpstan analyse` + Larastan | PHPStan's error list, baselines, ignores |
| `pun lsp` | Editor server: PHPantom for completion, hover, definitions and references; Pun for formatting, diagnostics and quick fixes | Intelephense / PHPStorm inspections for Pun users | Pint-identical formatting, same diagnostics as the CLI |
| `pun migrate` | One-time move of a project onto pun.toml | Hand-editing configs | Zero behaviour change on migration day |

So Phase 2 is `pun analyse` and Phase 3 is `pun lint`. `pun lint --fix` does not format afterwards, matching Rector; `--format` runs both in one pass.

### One config: pun.toml

Long term a project has one file:

```toml
[php]
version = "8.4"

[format]                      # from pint.json / .php-cs-fixer.php
preset = "laravel"
rules.ordered_imports = { sort_algorithm = "alpha" }

[lint]                        # from rector.php
sets = ["php84", "laravel-12", "dead-code"]
rules = ["early-return"]

[analyse]                     # from phpstan.neon; explicit rules, no levels
paths = ["app", "tests"]
baseline = "pun-baseline.toml"
laravel = true                # Larastan
rules.undefined-method = "error"
rules.mixed-argument = "off"
```

- **No levels.** A PHPStan level becomes the explicit list of rules it enables, written out by `pun migrate`. After that, rules turn on and off one by one, the way mago.toml works.
- **Exact parity is the migration safety net, not the end state.** Pun matches Pint, PHPStan and Rector exactly so a project can switch with zero diff. After migration Pun is free to use its own rule names, messages and rule combinations.
- **Compat read stays.** Without pun.toml, Pun keeps reading pint.json, phpstan.neon and rector.php directly so trying Pun needs no changes, and prints a one-line hint to run `pun migrate`. *(Proposed; confirm.)*
- **Rule names.** `format.*`, `lint.*`, `analyse.*` and `mago.*`, with Pun's own kebab-case names (`analyse.undefined-method` rather than `method.notFound`). `pun explain <rule>` shows the upstream tool's name and docs. *(Proposed; confirm.)*

### pun migrate (replaces the Phase 4 design)

1. `pun migrate --dry-run` reads pint.json, .php-cs-fixer.php, phpstan.neon, the baseline and rector.php. It prints the pun.toml it would write and **lists every rule, extension and Rector rule Pun can't run natively, before anything changes**.
2. Writes pun.toml, with levels expanded into explicit rules.
3. Converts phpstan-baseline.neon to pun-baseline.toml under Pun rule names.
4. Rewrites tool comments in code: `@phpstan-ignore <id>` and `@phpstan-ignore-next-line` become `@pun-ignore <rule>` and `@pun-ignore-next-line`, and `// @php-cs-fixer-ignore <rule>` becomes `// @pun-ignore format.<rule>`.
5. **PHPDoc type tags stay by default.** `@phpstan-param`, `@phpstan-return`, `@psalm-var` and the rest are type information that PhpStorm, Intelephense, Psalm and downstream users of a library rely on. `pun migrate --phpdoc` (opt-in, meant for applications, not libraries) rewrites them to `@pun-*`. Pun reads all three forms everywhere. Once `pun lsp` covers the editor side, `--phpdoc` can become the default for `"type": "project"`.
6. Proves parity: runs the old tools and Pun and diffs. Any difference blocks the switch.
7. Switches the workflow: composer scripts, CI steps, pre-commit hook, editor settings, Laravel Boost guidelines. `--remove-tools` deletes the old configs and composer dev dependencies.

The format part (`pint.json` to `[format]`, `@php-cs-fixer-ignore` rewrites) can ship now; `pun config --init` already writes `[format]`. The lint and analyse parts land with Phases 2 and 3.

### Agent-optimised output (every command)

Like [Laravel PAO](https://laravel.com/blog/introducing-laravel-pao-cleaner-output-for-ai-agents), every `pun` command detects when an AI agent runs it. Humans see unchanged output; agents get compact, structured results.

- **Detection.** One shared function ports laravel/agent-detector v2's env list (`CLAUDECODE`, `CLAUDE_CODE`, `CURSOR_AGENT`, `CODEX_SANDBOX`/`CODEX_THREAD_ID`, `GEMINI_CLI`, `AMP_CURRENT_THREAD_ID`, `OPENCODE`, `COPILOT_CLI`, `AUGMENT_AGENT`, `CLINE_ACTIVE`, `AI_AGENT`, and the rest), kept in sync with upstream. `pun format` already does this for Pint's agent reporter. Overrides: `PUN_AGENT=1|0` and `--agent` / `--no-agent`. PAO's `PAO_FORCE` and `PAO_DISABLE` are honoured too.
- **Output under an agent.**
  - One JSON object on stdout: `{"tool":"pun","command":"analyse","result":"fail","summary":{...},"items":[...]}`.
  - Each item has `file`, `line`, `rule` (Pun rule ID), `message`, plus `fix` / `fixable` and `diff` where they apply.
  - No ANSI, progress bars, dot leaders or boxes.
  - Long lists are capped, with a `"truncated": N` count and a hint to pass `-v` for the rest.
  - Errors are JSON too: `{"result":"error","message":...}`.
- **Per command.**
  - `format`: changed files, plus the rules applied per file.
  - `lint`: rule, location, safe or unsafe fix, and the diff.
  - `analyse`: error count plus per-file errors, the shape PAO normalises PHPStan into.
  - `migrate`: the plan, unsupported rules, and files rewritten.
  - `test` (later): counts, durations, failures with assertion, file and line.
- **Exit codes don't change** under an agent, so agents can still rely on them.
- Implemented once in pun-cli and shared by every subcommand, with a test per command that runs the binary with `CLAUDECODE=1` and asserts the JSON shape.

## Decisions for you to confirm

I made every other call myself. These change the shape of the project. Row 2 is now settled (config migration); the other four still need your call.

| # | Decision | My default | Alternative |
| --- | --- | --- | --- |
| 1 | Home and license | MIT, under shipfastlabs until v0.1 proves the format engine | Pitch it inside Laravel from day one |
| 2 | Meaning of `pun migrate` | Confirmed. Config migration: read pint.json, phpstan.neon, rector.php, composer scripts and CI, write pun.toml, verify parity. PHP version upgrades live in `pun lint --set php85` | `pun migrate` upgrades the code itself to PHP 8.5 (Rector sets + Laravel Shift style) |
| 3 | Relationship with Mago | Depend on crates.io releases, upstream every fix, talk to Seifeddine Gmati (Mago's maintainer) early about Pun's needs | Hard fork of Mago into the Pun repo |
| 4 | How FrankenPHP is "built in" | Phase A: Pun links libphp itself (Rust SAPI). Phase B: FrankenPHP compiled as a Go c-archive and linked into the same binary, sharing one libphp | Ship FrankenPHP as a version-pinned sidecar binary that Pun downloads and manages |
| 5 | Default formatter output | Byte-identical to Pint (compat engine) whenever pint.json or .php-cs-fixer.php exists or no config exists | Default to Mago-style "Pun style" and treat Pint parity as an import mode |

One more thing to check before code is written: the name "pun" on crates.io, Packagist, npm and Homebrew.

## Landscape (October 2026)

The gap Pun fills: Mago is fast but not a drop-in; the incumbent tools are compatible but slow and need PHP. Nothing today is both.

| Tool | State today | What it means for Pun |
| --- | --- | --- |
| [Bun lint / format](https://github.com/oven-sh/bun/issues/2246) | Merged Oct 12, 2026. Reads existing ESLint, oxlint, Prettier and .editorconfig configs; ships `--staged` modes. `bun fmt` [passes 99% of Prettier's test suite](https://x.com/bunjavascript/status/2109861465100632567), and about 47k of its \~100k lines build on Oxc/Biome's formatter | The template: read existing configs, reuse other projects' components, prove parity against the incumbent's own test suite, add `--staged` |
| [Mago](https://github.com/carthage-software/mago) | v1.48.1, Rust, MIT/Apache-2.0. Formatter, linter, analyzer in one binary, no PHP needed. Lints a 7M LOC project in 0.88s vs 25.3s for PHP-CS-Fixer ([benchmarks](https://mago.carthage.software/latest/en/)). Has a Laravel lint integration and a `laravel` formatter preset | The component base. But its Laravel preset is not byte-identical to Pint ([wick's note](https://packagist.org/packages/cresset/wick)), its config is mago.toml, and its analyzer does not emit PHPStan's identifiers or honour phpstan baselines |
| [Pint](https://laravel-news.com/blade-formatting-in-laravel-pint) | 1.3x, a PHP-CS-Fixer wrapper with presets laravel, psr12, per, symfony, empty. 1.30 added opt-in Blade formatting through Node, Prettier and prettier-plugin-blade | Byte parity target for `pun format`. Blade needs its own plan |
| [PHP-CS-Fixer](https://github.com/PHP-CS-Fixer/PHP-CS-Fixer) | 3.95.x, supports PHP 7.4 to 8.5 | The fixer source we port, and its per-fixer test fixtures are our parity suite |
| [PHPStan](https://phpstan.org/blog/phpstan-2-0-released-level-10-elephpants) + [Larastan](https://laraplugins.io/plugins/nunomaduro/laravel-code-analyse) | PHPStan 2.x with levels 0 to 10; Larastan 3.10 supports Laravel 13 | Parity target for `pun analyse` |
| [Rector](https://sourceforge.net/projects/rector.mirror/files/2.6.1/) | 2.6.x; rules now bound to installed composer package versions | Parity target for `pun lint` |
| [Pest 5](https://laravel-news.com/pest-5) | 5.3.2 (Oct 12, 2026). Requires PHP 8.4 + PHPUnit 13. TIA engine replays unaffected tests from cache; Taylor reported Laravel Cloud's 19,000+ tests going from 3 minutes to 5 seconds | Parity target for `pun test`; TIA must be matched natively, not just wrapped |
| [FrankenPHP](https://www.phparch.com/2026/07/frankenphp-in-production-worker-mode-embedded-binaries-and-real-performance/) | Go + Caddy, embeds libphp via cgo, worker mode, static single-binary builds | The runtime half of Pun |
| Rust libphp embedding | [ext-php-rs](https://docs.rs/crate/ext-php-rs/latest/source/guide/src/advanced/embedded_php.md) has an `embed` feature; [bext-php](https://docs.rs/bext-php/) shows a custom SAPI with a ZTS worker pool in Rust | Proof that the Rust SAPI approach works; we reuse the pattern |

Pinkary, the acceptance project, runs Pint, Rector, PHPStan and Pest (with 100% type coverage) through composer scripts ([repo](https://github.com/pinkary-project/pinkary.com)), so it exercises every phase.

## Architecture and Mago reuse

Pun is a Cargo workspace where Mago supplies the language layer and Pun adds the compatibility layer, the embedded PHP runtime, the test runner and the CLI. Rule of thumb: if Mago has it, Pun uses it; Pun only writes what exists to match another tool's exact behaviour.

&#91;embedded content: Pun architecture · 6 commands, 3 layers\]

Every command runs on Pun's compatibility layer, which is built on Mago's crates and hands anything not yet ported to the embedded PHP runtime; the harness checks all of it against the real tools.

### Mago components Pun uses

| Mago crate | Used by | What Pun does with it |
| --- | --- | --- |
| mago-syntax, mago-syntax-core | every command | Lexer, parser, AST. The compat formatter maps Mago's tokens onto PHP's `token_get_all` token kinds instead of writing a second lexer |
| mago-span, mago-bytes, mago-word | every command | Spans and byte-native source handling, so invalid UTF-8 and binary strings survive untouched |
| mago-database | every command | In-memory file database, shared by format, lint and refactor in one run |
| mago-names | format, lint, refactor | Name resolution for `ordered_imports`, `no_unused_imports`, `fully_qualified_strict_types` and every phpdoc fixer |
| mago-docblock | format, lint | Docblock parsing for phpdoc fixers and `@var` / `@phpstan-*` tags |
| mago-casing | format, refactor | Case conversion helpers |
| mago-composer | all | composer.json and autoload maps; PHP version constraint check |
| mago-php-version | all | Feature gating by target PHP version |
| mago-codex, mago-prelude | lint, refactor | Symbol table for project + vendor code and built-in PHP stubs |
| mago-analyzer, mago-algebra | lint, refactor | Type inference engine under `pun analyse` and type-aware Rector rules |
| mago-linter | lint | Its rules ship as opt-in extras (`[lint.mago]`); its Laravel integration seeds `pun-laravel` |
| mago-collector | lint | Suppression pragmas; extended to read `@phpstan-ignore` and `@phpstan-ignore-next-line` |
| mago-guard | test | Evaluates Pest `arch()` expectations statically, without booting PHP |
| mago-reporting | all | Diagnostic model; Pun adds Pint, PHPStan, checkstyle, JUnit, GitHub and SARIF output |
| mago-text-edit | format, refactor | Applying edits and fixes |
| mago-formatter | format, refactor | The opt-in "Pun style" engine, and printing new nodes that Rector-style rules create |
| mago-orchestrator, mago-allocator | all | Parallel pipeline and allocator, adopted if their APIs fit; otherwise rayon |

### Crates Pun writes

| Crate | Job |
| --- | --- |
| pun-cli | `pun` binary, subcommands, argv\[0\] aliases (`pint`, `phpstan`, `rector`, `pest`) for drop-in use |
| pun-config | pun.toml schema plus importers: pint.json, .php-cs-fixer(.dist).php, phpstan.neon(.dist) via a NEON parser, rector.php, phpunit.xml, tests/Pest.php |
| pun-workspace | File discovery that copies each tool's finder rules, git integration (gix), content-hash cache (blake3), `--dirty`, `--diff`, `--staged` |
| pun-tokens | PHP-CS-Fixer's `Tokens` model in Rust: token\_get\_all-compatible kinds, the Transformers that produce `CT::T_*` custom tokens, and the token analyzers fixers rely on |
| pun-fixers | Ported fixers, each tagged with PHP-CS-Fixer name, priority, risky flag, config schema and the upstream version it matches |
| pun-format | Runs fixer pipelines in priority order, rule sets (Pint's laravel and psr12 presets, @PSR12), engines (compat, pun-style) |
| pun-phpstan | PHPStan compatibility layer over mago-analyzer: levels, error identifiers, exact message text, baselines, ignoreErrors |
| pun-laravel | Laravel knowledge shared by lint, refactor and test (details in Phase 2) |
| pun-refactor | Rule engine, rule sets, format-preserving printer matching nikic/php-parser |
| pun-php | Embedded libphp (static, ZTS) behind a Rust SAPI; worker pool; sandboxed config evaluation |
| pun-delegate | Bridges that run real Pint fixers, PHPStan, Rector rules and Pest inside pun-php |
| pun-test | Discovery, scheduler, TIA, reporters, Pest bridge |
| pun-serve | FrankenPHP integration behind pun run |
| pun-harness | Differential testing against pinned upstream tools; corpus manager; parity scoreboard |

Third-party crates: rayon, ignore (from ripgrep), gix, blake3, pcre2 (PHP regex semantics), similar (diffs), clap, serde, toml, insta (snapshot tests), notify (watch mode), ext-php-rs (Zend bindings).

## Compatibility strategy

Every Pun command runs each unit of work (a fixer, an analysis rule, a Rector rule, a test file) on one of two paths, and a differential harness proves both paths give the upstream tool's exact output.

### Native path

Rust code on Mago's AST and Pun's token model. No PHP process, no PHP install. This is where the speed comes from.

### Delegate path

The real upstream PHP code, run inside Pun's embedded libphp. Pun bundles pinned PHARs of Pint, PHP-CS-Fixer, PHPStan, Larastan and Rector in its runtime cache (downloaded on first use, checksum-verified). It is used when:

- a fixer, Rector rule or PHPStan extension has not been ported yet
- the user's config loads custom PHP code (custom fixers, third-party PHPStan extensions, custom Rector rules)
- the user passes `--delegate=<tool>` to compare or work around a native bug

`pun <command> --explain` prints which units ran natively and which were delegated. Delegation is per unit, not per run. For the formatter, consecutive native fixers run on Pun's token stream, and at a delegated fixer Pun prints code, runs that one fixer through PHP-CS-Fixer's API in embedded PHP, and re-lexes. If the harness ever shows a segment boundary changing output, that file falls back to whole-file delegation.

### Config compatibility

Pun reads existing configs as they are. JSON, NEON, XML and TOML are parsed in Rust. PHP config files are executed once in embedded PHP with stub `PhpCsFixer\Config`, `RectorConfig` and Pest classes that record every call, then serialized to JSON and cached by file hash. Configs that do more than call the builder (loops, env reads) still work because they really run. pun.toml is optional and overrides imported settings key by key.

### Version pinning

Parity is only meaningful against a fixed version. Each Pun release declares the upstream versions it matches (for example Pint 1.3x.y, PHP-CS-Fixer 3.95.z, PHPStan 2.1.w, Rector 2.6.v, Pest 5.3.u). By default Pun matches the version locked in the project's composer.lock where it supports it, and warns otherwise.

### The differential harness (pun-harness)

1. **Oracle runners.** Docker images with pinned PHP + Pint, PHP-CS-Fixer, PHPStan + Larastan, Rector and Pest. The harness runs the oracle and Pun on the same input and diffs bytes, diagnostics or test results.
2. **Upstream fixtures.** Every fixer test case in PHP-CS-Fixer's repo, PHPStan's rule tests, Rector's fixture files and Pest's own test suite are imported as Pun tests (MIT, with attribution). Pass rate per tool is published.
3. **Real-world corpus.** Pinkary first, then laravel/framework, laravel/laravel, Pint, Pest, Larastan and 30+ popular Laravel apps and packages pinned to commits. For formatting, the oracle is Pint run on each repo with the repo's own pint.json (or the laravel preset).
4. **Fuzzing.** Mutated corpus files (whitespace, comments, line endings, nesting) through both oracle and Pun to find divergence the fixtures miss.
5. **Gate.** CI blocks any merge that drops a parity number. A native port is only marked "native by default" at 100% fixture parity and zero corpus diffs; until then it ships delegated.

### Porting with AI agents

Most of the work is mechanical: read a PHP fixer and its tests, write the Rust equivalent, make the harness pass. That loop suits coding agents well because the harness is an objective checker. Plan for an agent-per-fixer workflow, with humans reviewing the token model, the analyzer layer and anything the harness flags as flaky.

## Phase 0: Foundations

Goal: the harness, the corpus and a working embedded PHP exist before any fixer is ported. Everything later is measured against them.

- [ ] Cargo workspace, CI on macOS arm64/x64 and Linux x64/arm64 (glibc + musl), release pipeline via cargo-dist, install script, Homebrew tap, Composer package that downloads the binary on first run (the pattern Mago uses)
- [ ] Pin Mago crates; spike: parse the whole corpus with mago-syntax and record parse failures and timings
- [ ] **Token parity spike:** map Mago lexer output to PHP `token_get_all` kinds and diff against real `token_get_all` on the entire corpus. Target: identical token stream (kind, text, line) on every file. If Mago's lexer cannot be mapped cleanly, write pun-tokens' own lexer and keep Mago for everything else
- [ ] pun-harness v1: Docker oracle images, corpus manifest pinned by commit, byte-diff runner, HTML scoreboard
- [ ] pun-php v1: static libphp 8.5 (ZTS) built with static-php-cli, Rust SAPI, run a PHP file, capture output and errors, survive fatal errors without killing the host
- [ ] Config evaluation sandbox: run .php-cs-fixer.php and rector.php with recording stubs, serialize to JSON
- [ ] pun.toml schema v0 and `pun config --show` (resolved config after imports)
- [ ] Benchmark rig: hyperfine on the corpus, cold and warm cache, peak RSS, published on every main build

**Exit gate:** token stream parity at 100% on the corpus (or a decision to own the lexer), embedded PHP runs Pint's own test suite, harness reproduces Pint's output on Pinkary from a clean container.

## Phase 1: pun format

Goal: `pun format` produces byte-identical output to the pinned Pint version on Pinkary and the corpus, with Pint's laravel and psr12 presets and every PHP-CS-Fixer rule available (native or delegated), at least 30x faster than Pint.

### Status (Oct 17)

| Milestone | State | Evidence |
| --- | --- | --- |
| M1.1 Token layer | Done | token_get_all parity; PHP's own grammar for syntax errors (bison tables from php-src 8.5.11, 788 oracle snippets) |
| M1.2 laravel preset | Done | 147 Pinkary rules plus new_with_parentheses and Pint/phpdoc_type_annotations_only; upstream fixtures plus oracle cases per rule; 166-rule end-to-end suite through the real binary |
| M1.3 CLI parity | Done | Every flag and reporter checked against Pint 1.29.3 (scripts/cli-parity.sh, tests/flags.rs, tests/output.rs), including error boxes, colours, -v diffs and the prettier check. Pun extras: `--since`, `--staged`, `--stdin`, `--explain`, `--native-only`, `--no-cache` |
| M1.4 Delegate mode | Done, with system PHP | Unported or custom fixers send the whole run through PHP-CS-Fixer in long-lived `php` workers (laravel-framework's own pint.json matched before its 8 missing rules were ported) |
| M1.5 PSR-12 | Done | psr12 preset and @PSR1/@PSR2/@PSR12 expanded like PHP-CS-Fixer's RuleSet; 0 differing on all 7 corpus projects |
| M1.6 Cache, watch, editor | Done | Content-hash cache (xxh3, signature = version, PHP, rules, Blade lockfiles); `--watch`; formatting-only `pun lsp` |
| Parity | Done | 0 differing on all 7 corpus projects (laravel and psr12); mutation fuzzer incl. CRLF/mixed line endings: 0 differing |
| Coverage | Done | 100% lines on every crate, no tautological tests |
| Cleanup (review, architecture, comments, fmt, clippy) | In progress | |

Speed (profiling build, shared 2-core box; docs/scoreboard.md):

| Run | Pint | Pun cold | Pun warm | Mago |
| --- | --- | --- | --- | --- |
| laravel-framework, own pint.json (3027 files) | 190 s | 3.7 s (~50x) | 0.07 s | 1.1 s, no cache |
| Pinkary PHP only | | 0.33 s | 0.03 s | 0.13 s |
| Pinkary with Blade | 13.4 s | 4.2 s (3.2x) | 0.2 s (67x) | |

### How the compat engine works

PHP-CS-Fixer's model, reproduced exactly:

1. Lex the file into PHP tokens (kinds identical to `token_get_all` for the target PHP version).
2. Run the Transformers that retag tokens into custom kinds (array square braces, nullable `?`, union and intersection `|` / `&`, named argument colons, attribute brackets, and the rest).
3. Sort enabled fixers by priority, then by name, exactly as PHP-CS-Fixer does. Run each fixer once, in that order, on the shared token list. No fixpoint loop, because upstream has none.
4. Each fixer checks `isCandidate` first (cheap token-kind bitset check), then edits tokens through the same operations upstream has: insert, clear, override, ensure whitespace.
5. Clear empty tokens between fixers as upstream does, then print.

Porting rules: keep upstream's structure function by function, so a diff between the PHP and the Rust stays reviewable. Use the `pcre2` crate wherever upstream calls `preg_*`, never Rust's `regex`, because PHP regexes use lookbehind, backreferences and PCRE-specific flags. Reproduce PHP string semantics (`strlen` is bytes, `strtolower` is ASCII-only, `strcmp` / `strcasecmp` / `strnatcasecmp` ordering) in one small `php_compat` module.

### Remaining for v0.1

1. **Cleanup pass.** Code review with fixes, architecture tidy-up, comment removal, fmt and clippy, with every gate re-run. In progress.
2. **Token text as spans** into the source buffer instead of one `Vec<u8>` per token. Tokenizing is about 17% of a cold run, so this is the next structural speed step. Re-measure against Mago.
3. **8-core benchmark in CI** to prove laravel-framework is under 1 s cold and under 100 ms warm; it's about 1 s by extrapolation today.
4. **Embedded PHP for delegate mode** (pun-php: static libphp, Rust SAPI) plus a pinned, checksum-verified Pint PHAR, so delegation needs no system PHP and no vendored Pint. Today it uses the system `php` and the project's `vendor/` Pint, or `$PUN_PINT`.
5. **`pun migrate`, format part**: pint.json / .php-cs-fixer.php to `[format]`, and `@php-cs-fixer-ignore` rewrites (see the Oct 17 update).
6. **Release gate.** CI workflows pushed (needs the gh `workflow` scope), `vendor/bin/pint` swapped for Pun in Pinkary's CI, then tag v0.1 and publish to crates.io, npm, Homebrew and Packagist. Publishing needs your approval.
7. **After v0.1:** pun-harness becomes local dev tooling (`tools/harness`, `publish = false`), and its fixtures move to pun-fixers.

### Blade

Pint formats `.blade.php` only when `Pint/laravel_blade` is on, through Node, Prettier and prettier-plugin-blade. Pun does the same with one long-lived Prettier worker per thread and the same pre-flight checks and messages. Blade is now the cold-run bottleneck on Pinkary (about 2.9 s of the 4.2 s), and 30x on cold Pinkary is out of reach while it goes through Node. The fix is a native Blade formatter (a Blade parser plus Prettier-compatible HTML printing). Evaluate Bun's Prettier-compatible formatter (Oxc/Biome based) as the base, with Pint's Prettier output as the oracle. Planned after v0.1.

### Pun style engine

`--engine=pun` uses mago-formatter with its laravel preset: whole-file pretty-printing, faster again, deterministic, but not Pint-identical. Opt-in for teams that want to move past Pint's output.

### Targets

- `pun format --check` on laravel/framework, cold cache, 8 cores: under 1 second (about 1 s by extrapolation; confirm in CI)
- Same run, warm cache: under 100 ms (0.07 s on 2 cores, met)
- Peak memory: under 300 MB (about 50 MB, met)
- Pinkary: at least 30x faster than `pint --test` (67x warm, met; 3.2x cold, blocked by Blade via Node)

**Exit gate (v0.1 release):** 100% byte parity on Pinkary and the corpus (met), 100% of the laravel preset's upstream fixtures passing natively (met), every other rule available natively or delegated (met), CLI drop-in verified by replacing `vendor/bin/pint` in Pinkary's CI (to do).

## Editor: pun lsp

Goal: one editor server for Pun users that is fast, low on memory, and shows the same results as the CLI.

- **PHPantom** (MIT, Rust, built on mago =1.57.0 like Pun) provides completion, hover, go-to-definition, references and symbols, including generics, `@phpstan-*` annotations, array shapes and Laravel (Eloquent, Blade, Artisan). It starts in about 1 s and uses about 59 MB on a 21k-file project.
- **Pun** provides formatting (Pint-identical, already shipped), diagnostics (syntax errors, `[lint.mago]` rules, later `pun analyse` and `pun lint`), and quick fixes from `pun lint --fix` rules.
- **Integration:** embed PHPantom as a library in `crates/pun-lsp`, merge capabilities, and route each request to its owner. If PHPantom has no usable library API, send the small upstream patch it needs (for you to approve before anything leaves the repo). A child-process proxy is the fallback.
- **Long term,** hover types and diagnostics should come from one type engine. Work with PHPantom upstream on sharing Mago's codex, and move in step with it on Mago version bumps.
- **Milestones:**
  - L1, evaluation plus prototype in crates/pun-lsp (in progress).
  - L2, `pun lsp` switches to it, with editor setup docs for VS Code, Neovim, PhpStorm (LSP4IJ) and Zed.
  - L3, diagnostics from `pun analyse` as Phase 2 lands.
  - L4, quick fixes from `pun lint --fix` as Phase 3 lands.
  - L5, once the editor side covers it, `pun migrate --phpdoc` becomes the default for applications.

## Phase 2: pun analyse

Goal: `pun analyse` reports the same errors as PHPStan + Larastan, meaning the same (file, line, identifier, message) set at the configured level, honouring the same baseline and ignoreErrors, so a project can swap `phpstan analyse` for `pun analyse` in CI with no baseline churn. This is the hardest phase; v1 targets levels 0 to 6 with Larastan at full parity, then 7 to 10.

### Why exact messages matter

PHPStan baselines and `ignoreErrors` match on message text (regex) and identifier, with counts per file. If one word in a message differs, a baseline entry stops matching and CI fails. So pun-phpstan owns a type printer that describes types exactly as PHPStan does (`list<string>`, `non-empty-string`, `array<int, App\Models\User>`, generic and union ordering), and every message template is copied from the pinned PHPStan version.

### Milestones

1. **M2.1 Config.** NEON parser; `includes` including vendor extension.neon files found via phpstan/extension-installer; parameters (level, paths, excludePaths, ignoreErrors, baseline, bootstrapFiles, scanFiles, scanDirectories, stubFiles, treatPhpDocTypesAsCertain, reportUnmatchedIgnoredErrors and the rest); `@phpstan-ignore` comment handling via mago-collector; baseline read and `--generate-baseline` write in PHPStan's format.
2. **M2.2 Symbol parity.** PHPStan's function signature map and stubs imported at build time into mago-codex, so built-in functions have PHPStan's exact signatures rather than Mago's prelude where they differ.
3. **M2.3 Rule mapping, level by level.** For every PHPStan rule enabled at levels 0 to 6: map it to an existing Mago analyzer issue (rewriting code and message), or implement it in pun-phpstan using Mago's type inference, or mark it delegated. Parity is measured per level on the corpus and on PHPStan's own rule test fixtures.
4. **M2.4 Laravel (pun-laravel).** Native Larastan behaviour: model attributes from migrations and schema dumps, casts, `Attribute` accessors, relationships, scopes, Eloquent builder and collection generics, facades resolved to their roots, container resolution (`app(Foo::class)`), config and env helpers, macros, plus Larastan's own rules. Like Larastan, some facts need the booted app; Pun boots it once in embedded PHP, dumps a runtime manifest (bindings, facade roots, macro signatures, config keys) and caches it by composer.lock and config hash. `--no-boot` skips this for untrusted code.
5. **M2.5 Pest.** Native equivalent of Pest 5's PHPStan plugin: `it()`, `test()`, `expect()` and `$this` inside test closures, plus its Pest-specific rules.
6. **M2.6 Levels 7 to 10, strict-rules, deprecation-rules.** Same method.
7. **M2.7 Incremental.** Per-file dependency graph so a change re-analyzes only dependents; reuse Mago's incremental analysis engine where it fits. `pun analyse --watch` and LSP diagnostics.

### Delegation in lint

PHPStan rules run inside PHPStan's own scope and type system, so delegating a single rule while the rest runs natively is not possible. The rule: if the config loads an extension or custom rule Pun has not ported, `pun analyse` runs real PHPStan in embedded PHP for the whole run (still with Pun's discovery, caching and reporting) and says why. `--native-only` runs Pun's analysis and lists what was skipped. Common extensions (Larastan, strict-rules, deprecation-rules, phpstan-phpunit, Pest's plugin) are ported so most Laravel projects never delegate.

### Extras beyond PHPStan

Mago's linter rules (including its Laravel rules) are available under `[lint.mago]`, off by default so the default output stays PHPStan-identical. Syntax errors come free from the parser, replacing `php -l` loops.

**Exit gate (v0.2):** Pinkary produces the identical error list at its configured level with its baseline; 20+ corpus apps identical at levels 0 to 6; ported rules pass PHPStan's own fixtures; at least 10x faster than PHPStan on a cold run.

## Phase 3: pun lint

Goal: `pun lint --dry-run` prints the same diff as `rector process --dry-run` for the project's rector.php, and `pun lint` writes the same files, then formats them in the same pass.

### What has to match

Rector parses with nikic/php-parser, asks PHPStan for types, changes nodes, then uses a format-preserving printer: untouched code keeps its original bytes, changed nodes are reprinted by php-parser's standard printer. So Pun needs three things to match byte for byte:

1. **Types:** rules query types through pun-phpstan, so a rule sees what Rector would see from PHPStan, including Laravel facts from pun-laravel.
2. **Rule logic:** ported from Rector rule by rule, same node matching and same configurable options.
3. **Printing:** a Rust format-preserving printer that copies php-parser's rules: which original tokens are kept, how changed nodes are reprinted (spacing, parentheses, `array()` vs `[]`, attribute and argument layout, comment re-attachment). It is checked against the oracle independently of any rule.

After rules run, Pun runs `pun format` on changed files in memory, so the common `rector && pint` sequence becomes one read and one write per file.

### Milestones

1. **M3.1 Config.** rector.php through the config sandbox: `withPaths`, `withSkip`, `withRules`, `withConfiguredRule`, `withSets`, `withPreparedSets`, `withPhpSets`, composer-based sets, the type coverage / dead code / code quality levels, `withImportNames`, parallel and cache options.
2. **M3.2 Printer.** Format-preserving printer passes a round-trip test (parse, no change, print equals input) on the whole corpus, then php-parser's own printer fixtures.
3. **M3.3 Rules, in usage order.** First, the rules active in Pinkary's rector.php. Then the prepared sets (dead code, code quality, coding style, type declarations, privatization, early return, instanceof), the PHP sets that matter for 8.4+ code, driftingly/rector-laravel, Pest 5's Rector plugin (60 rules) and the PHPUnit sets. Each rule lands with Rector's own fixture files passing (before/after pairs).
4. **M3.4 Delegate.** Unported or custom rules run through real Rector in embedded PHP on the file after native rules, then the result is re-parsed; the harness confirms the order matches Rector's.
5. **M3.5 PHP upgrades.** `pun lint --set php85` (and php86 once released) applies the PHP-version sets; this is where code upgrades live, keeping `pun migrate` about tooling.

**Exit gate (v0.3):** Pinkary dry-run diff identical to Rector; corpus round-trip printing byte-identical; ported rules pass Rector fixtures; at least 10x faster than Rector on a cold run.

## Phase 4: pun migrate

> Superseded in part by "pun migrate" in the Oct 17 update: migrate now writes pun.toml, converts the baseline and rewrites ignore comments. The workflow steps below (eligibility, inventory, parity proof, CI and composer script switch, Boost guidelines) still apply.

Goal: one command moves an existing Laravel project onto Pun, proves nothing changed, and leaves a clear report. Pun already reads the old configs directly, so migrate is about switching the workflow, not translating rules.

`pun migrate` runs these steps and stops before writing anything unless `--write` is passed:

1. **Check eligibility.** composer.json must require PHP 8.4 or newer. Below that, Pun stops and prints the upgrade path (the PHP sets in `pun lint` can run on that project as the one exception).
2. **Inventory.** Read composer.lock for Pint, PHP-CS-Fixer, PHPStan, Larastan, extensions, Rector and Pest versions; find pint.json, .php-cs-fixer(.dist).php, phpstan.neon(.dist), phpstan-baseline.neon, rector.php, phpunit.xml, tests/Pest.php.
3. **Coverage report.** For every configured rule, fixer, extension and Rector rule: native, delegated, or unsupported. Shows how much of the project will run natively.
4. **Parity proof.** Run each old tool and its Pun replacement on the project and diff: formatting bytes, lint errors, refactor diff, test results. Any difference blocks the switch and is printed.
5. **Write pun.toml** with only what differs from the imported configs (usually just pinned compat versions).
6. **Switch the workflow.** Rewrite composer scripts (`lint`, `test:lint`, `refactor`, `test:types`, `test:unit`, `test`), GitHub Actions and GitLab CI steps (setup-php plus vendor/bin calls become a setup-pun step plus pun calls), add a pre-commit hook (`pun format --staged && pun analyse --staged`), and update editor settings that point at vendor/bin/pint.
7. **Agent guidelines.** If Laravel Boost is installed, add a Pun guideline so coding agents call `pun` instead of the old binaries.
8. **Optional cleanup.** `--remove-tools` drops the replaced dev dependencies. Default keeps them, so teams can run both during a transition and roll back by reverting one commit.

Also: `pun migrate --from mago` reads mago.toml for teams already on Mago, and `pun migrate --check` runs only steps 2 to 4 for CI dashboards.

**Exit gate (v0.4):** Pinkary migrates with zero parity diffs and its CI passes with only Pun installed.

## Phase 5: pun install

Goal: `pun install`, `pun update`, `pun add` and `pun remove` are drop-in for Composer: the same composer.lock (same resolution, same content-hash, same JSON formatting), the same vendor/ tree and byte-identical autoload files, several times faster through parallel downloads and a global cache. A new crate, pun-pm, holds the resolver, installer and autoload dumper.

### What has to match

1. **Resolver.** Port Composer's SAT-based solver and its default policy (highest version, stability, minimum-stability, prefer-stable), constraint parsing as composer/semver does it (stability flags, `@dev`, inline `as` aliases, branch aliases, dev branches), platform packages (`php`, `ext-*`, `lib-*`, `composer-plugin-api`, `config.platform` overrides), `replace`, `provide` and `conflict`. Port, don't reinvent: a different solver that finds an equally valid set still writes a different lock.
2. **Lock file.** content-hash computed from the same composer.json keys, same key order, same JSON encoding.
3. **Sources.** Packagist metadata v2 with checksum-verified dist archives, git source fallback (gix), path and artifact repositories, private repositories (Satis, Private Packagist, VCS) and auth.json credentials (http-basic, github-oauth, gitlab-token, bearer).
4. **vendor/ output.** installed.json, installed.php, the autoload\_\*.php files, autoload\_static.php, platform\_check.php and vendor/bin proxies byte-identical to Composer's. Classmap scanning uses Mago's lexer with Composer's class-map rules; `--optimize`, `--classmap-authoritative` and `--apcu` supported.
5. **Scripts and events.** Composer scripts plus post-install-cmd, post-update-cmd and post-autoload-dump (Laravel's `package:discover`) run in embedded PHP.

### Composer plugins

Plugins are PHP code that hooks into Composer's internals, so they are the hardest part. The common ones are reimplemented natively: composer/installers (custom install paths), phpstan/extension-installer (its generated config), pestphp/pest-plugin (plugin manifest), php-http/discovery. If a project allows any other plugin, that run falls back to the real Composer PHAR inside embedded PHP, with Pun's download cache in front, and Pun names the plugin that caused it.

### Speed

Global content-addressed cache; files cloned (APFS clonefile, Linux reflink) or hard-linked into vendor/ where safe, copied otherwise; parallel metadata and archive downloads; no resolution at all when composer.lock's content-hash matches composer.json.

### Also in this phase

`pun audit` (Packagist security advisories and abandoned packages), `pun outdated`, `pun why`, and a `composer` alias that accepts Composer's common flags (`--no-dev`, `--prefer-dist`, `--optimize-autoloader`, `--ignore-platform-req`, `--prefer-lowest`).

### Edge cases

- GitHub API rate limits and token prompts
- Dev branches and branch aliases that move between runs
- `--prefer-lowest` in CI matrices
- `config.platform` disagreeing with the runtime's PHP version and extensions
- Windows long paths, and symlinks for path repositories
- Harness determinism: the oracle and Pun must resolve against the same Packagist state, so the harness serves a frozen metadata mirror

**Exit gate (v0.5):** `pun install` from Pinkary's lock gives an identical vendor/ tree and identical autoload files; `pun update` against the frozen mirror writes Composer's lock byte for byte across the corpus; cold installs at least 3x faster than Composer.

## Phase 6: pun run on FrankenPHP

Goal: `pun run index.php` runs PHP through FrankenPHP inside the Pun binary, the way `bun run index.ts` runs a file. After this phase a Laravel project needs no system PHP and no Composer.

1. **What `pun run` does.** `pun run script.php` executes a CLI script on FrankenPHP's embedded PHP. `pun run public/index.php` (any HTTP entry point, or with `--serve`) starts FrankenPHP's server in worker mode on it. `pun run <name>` runs a composer script and `pun x <bin>` runs a vendor binary.
2. **FrankenPHP in the binary.** Spike first: FrankenPHP compiled as a Go c-archive and linked into the Rust binary, sharing one libphp with pun-php. Hazards: two runtimes handling signals, and TSRM threads created by both Go and Rust. Fallback: a version-pinned FrankenPHP sidecar managed by Pun (decision 4).
3. **Runtime manager.** The runtime has existed internally since Phase 0; now `pun runtime install 8.4|8.5` exposes it, with pinned static builds and a broad extension set (mbstring, intl, pdo\_mysql, pdo\_pgsql, pdo\_sqlite, redis, gd, zip, bcmath, pcntl, sockets, sodium, opcache, pcov). The project's PHP minor version comes from composer.json. Check whether Mago's PHP version manager and extension installer can be reused first.
4. **Laravel modes.** `pun run public/index.php --watch` restarts workers on file changes; Octane's FrankenPHP integration works unchanged; HTTPS on localhost through Caddy.
5. **Later:** `pun build` for single-binary apps using FrankenPHP's embedding.

**Exit gate (v0.6):** Pinkary installs with `pun install` and serves through `pun run public/index.php` with no system PHP or Composer installed.

## Phase 7: pun test

Goal: `pun test` runs a Pest 5 suite with identical results (same tests, same pass, fail, skip and todo counts, same failures) and is faster than `pest --parallel` cold and at least as fast as Pest's TIA warm.

Tests are PHP, so they always execute in PHP. What Pun replaces is everything around execution: discovery, process management, scheduling, impact analysis, reporting and the static checks Pest runs as plugins.

### Design

- **Static discovery.** Parse tests/ with Mago: `it`, `test`, `describe`, `dataset`, `->with()`, `uses()`, hooks, `->skip()`, `->todo()`, `->group()`, and PHPUnit classes and attributes. `pun test --list` and filters (`--filter`, `--group`, `--dirty`) work without booting PHP.
- **Workers.** Each worker is a process running embedded PHP, not a thread. Laravel tests rely on global state and many extensions are not thread-safe, so process isolation is the safe default. Workers boot Pest and the Laravel app once, then take test files over IPC.
- **Prefork on Linux.** Boot the app once in a parent process, then fork workers from it so each skips framework boot. On macOS, forking after PHP has started threads is unsafe, so workers spawn normally there.
- **Bridge first.** Workers drive the real Pest runtime through a small PHP bridge plugin that takes file lists and streams JSON results. Pest's per-process `TEST_TOKEN` is set exactly as `--parallel` does, so Laravel's parallel testing hooks (per-process databases) keep working.
- **Native scheduling.** Duration-balanced scheduling from recorded timings (reading Pest's shard timing file), `--shard=n/m`, `--bail`, retries.
- **TIA, natively.** Per-test-file dependency graph from coverage (pcov compiled into Pun's runtime), pre-filtered by the static import graph from mago-codex so obviously unaffected files are skipped before any PHP runs. Same semantics as Pest's TIA: replay cached results and cached coverage for unaffected tests. Reading Pest's TIA baseline artifacts is a stretch goal.
- **Static plugins in Rust.** Type coverage (`--type-coverage --min=100`, which Pinkary uses) computed from the AST; `arch()` expectations evaluated with mago-guard.
- **Reporters.** Pest's default and compact printers byte-for-byte, JUnit, TeamCity, GitHub annotations.
- **Developer loop.** `pun test --watch` re-runs affected tests on save using the TIA graph; `--staged` and `--changed=<ref>`.

### Runtime choice per project

Pun reads `ext-*` requirements from composer.json. If its embedded runtime has every required extension and matches the project's PHP minor version, it uses the embedded runtime; otherwise it falls back to the system `php` with the same orchestration and says why. Browser tests (Pest's browser plugin) and Dusk run on the system runtime in v1.

### Milestones

1. **M5.1** Bridge mode with Pun's workers and reporters; Pinkary suite identical.
2. **M5.2** Static discovery, filters, sharding, prefork.
3. **M5.3** Native TIA and watch mode.
4. **M5.4** Type coverage and arch in Rust; coverage reports via php-code-coverage in the bridge.

**Exit gate (v1.0):** Pinkary results and type coverage identical to Pest; corpus suites identical; at least 2x faster than `pest --parallel` cold on Pinkary; a fresh clone goes from `pun install` to a green `pun test` with no system PHP or Composer installed.

## Weird edge cases

The ones most likely to break parity are the first three: upstream output depends on the PHP version and ini settings the tool ran under, upstream is not always idempotent, and PHP's string and regex semantics differ from Rust's. Each row says how Pun handles it and whether it ships in v1.

| Area | Edge case | Plan |
| --- | --- | --- |
| Format | Fixer output depends on the PHP version Pint ran under (version checks inside fixers, `token_get_all` changing between versions) | Pun emulates a target runtime version (composer.lock platform, or `[compat] php`); the oracle records the version it used. v1 |
| Format | PHP-CS-Fixer runs each fixer once, so a second Pint run can change more | Replicate the single pass exactly; never "improve" it into a fixpoint. v1 |
| Format | `short_open_tag` ini changes how `<?` tokenizes | Default to the oracle's ini (off), configurable. v1 |
| Format | Regex fixers on invalid UTF-8: `preg_*` with the `u` flag returns null and the fixer silently skips | Same failure path through pcre2, tested with invalid-UTF-8 fixtures. v1 |
| Format | Alignment fixers measure with byte length, so multibyte text "misaligns" upstream | Match byte semantics. v1 |
| Format | Sorting (imports, class elements, phpdoc tags) uses `strcmp`, `strcasecmp`, `strnatcasecmp` | One `php_compat` module with exact ports, fuzzed against PHP. v1 |
| Format | Files with syntax errors | Report and skip, exactly as Pint does, same exit code. v1 |
| Format | Inline HTML, `?>` at end of file, `<?=` | Covered by token parity spike. v1 |
| Format | `__halt_compiler()` with trailing binary data | Bytes after it are never touched. v1 |
| Format | Flexible heredoc and nowdoc indentation | Dedicated fixtures; fixers that reindent must match upstream. v1 |
| Format | CRLF, mixed line endings, BOM, no final newline, git autocrlf | Byte-level fixtures per case. v1 |
| Format | Which files Pint picks (vendor, node\_modules, storage, bootstrap/cache, hidden dirs, VCS-ignored files, symlinks, `in` / `exclude` / `notName` / `notPath`) | Copy the pinned finder rules; a fixture repo full of tricky paths. v1 |
| Format | `--staged` with partially staged files | Format the index version, write it to the index, apply the same edit to the working tree only if it applies cleanly; otherwise report. v1 |
| Format | Custom fixers registered in .php-cs-fixer.php | Delegate mode. v1 |
| Format | Multi-megabyte generated files (IDE helpers, language files) | Stream and parallelize; warn above a size limit. v1 |
| Format | Blade with `Pint/laravel_blade` on | Same Node toolchain as Pint in v1; native later |
| Lint | `reportUnmatchedIgnoredErrors`: if Pun misses an error the baseline expects, CI fails | Parity must hold in both directions (no extra and no missing errors). v1 |
| Lint | `bootstrapFiles` that define constants or class aliases | Run them in embedded PHP and import the symbols. v1 |
| Lint | Booting the app for Larastan touches .env, cache or the database | Boot with the same env PHPStan would see; catch failures and fall back to static facts with a warning; `--no-boot`. v1 |
| Lint | Duplicate class names in vendor, conditional declarations, `class_alias`, `eval` | Resolve in Composer autoload order, as PHPStan does. v1 |
| Lint | Classmap, files autoload, `exclude-from-classmap`, path repositories, monorepos | mago-composer plus fixtures per layout. v1 |
| Lint | Baseline paths relative to the baseline file, Windows separators, regex escaping | Port PHPStan's matcher exactly. v1 |
| Lint | Output order under parallel analysis | Sort deterministically, as PHPStan does. v1 |
| Refactor | Comments and docblocks on removed or moved nodes | Port php-parser's comment re-attachment rules. v1 |
| Refactor | Rector output that Pint would change back | Fused pass: refactor then format in memory. v1 |
| Refactor | `withSkip` with globs and rule-plus-path pairs | Port Rector's skipper. v1 |
| Test | Order-dependent tests pass or fail depending on which files share a process | `--batching=pest` reproduces Pest's distribution; Pun reports suspected order dependence. v1 |
| Test | `exit()`, fatal errors, extension segfaults inside a worker | Restart the worker, rerun the file in isolation, report clearly. v1 |
| Test | `#[RunInSeparateProcess]` | Fresh process per test. v1 |
| Test | Per-process databases (SQLite file or in-memory, MySQL / Postgres per token) | Same `TEST_TOKEN` scheme as Pest; cleanup on Ctrl+C. v1 |
| Test | php.ini drift between system PHP and Pun's runtime (memory\_limit, timezone, error\_reporting, opcache) | Runtime defaults follow php.ini-development; phpunit.xml `<ini>` and `<env>` applied. v1 |
| Runtime | Required extension missing from the embedded build (imagick, grpc, swoole) | Fall back to system PHP with a message. v1 |
| Runtime | glibc vs musl builds, allocator speed on musl | Ship both; use mimalloc. v1 |
| Runtime | macOS signing and notarization of a binary with embedded PHP | Part of the release pipeline from Phase 0. v1 |
| Platform | Windows: static libphp is hard there | Format, lint, refactor native on Windows; test and run use system PHP. Tier 2 |
| Security | Configs, bootstrap files and app boot execute project code | Same trust as today's tools; `--no-exec` for untrusted pull requests runs native-only with JSON / NEON configs. v1 |
| Language | PHP 8.6 lands around November 2026 with new syntax | Track Mago's parser; oracle images updated when PHP-CS-Fixer supports 8.6. Rolling |

## Out of scope for v1

These are deliberately not shipped in the first versions; each is either low demand among PHP 8.4+ Laravel projects or a project of its own.

- Projects that allow PHP below 8.4 (except running the PHP upgrade sets)
- Pint's per, symfony and empty presets, and PHP-CS-Fixer's @PER-CS, @Symfony and @PhpCsFixer sets as native targets (delegate mode still runs them)
- Psalm and PHP\_CodeSniffer / PHPCBF compatibility
- Native Blade formatting (v1 uses Pint's Node path)
- Native execution of third-party PHPStan extensions, custom Rector rules and uncommon Composer plugins (delegate mode covers them)
- Browser tests and Dusk inside Pun's runtime (system PHP in v1)
- Test runtime and `pun run` on Windows
- Single-binary app builds (`pun build`)

## Risks

The biggest risk is PHPStan parity: it is a type system with years of behaviour, and exact message parity is unforgiving. The plan contains it by shipping format first, scoping lint v1 to levels 0 to 6, and always having delegate mode.

| Risk | Impact | Mitigation |
| --- | --- | --- |
| PHPStan parity takes far longer than planned | `pun analyse` slips | Level-by-level parity with published numbers; delegate mode keeps lint usable; ship format and test independently |
| Mago's analyzer models types differently from PHPStan in ways a mapping layer can't fix | Some rules need deeper changes | Upstream changes behind options first; a scoped fork of mago-analyzer only as a last resort |
| Mago API churn (it moves fast: 1.0 in late 2025, 1.48 now) | Breakage on upgrades | Pin versions, upgrade on a schedule with the harness as the gate, contribute tests upstream |
| Mago's lexer can't be mapped onto `token_get_all` cleanly | Compat formatter needs its own lexer | Decided in the Phase 0 spike; a PHP lexer is a few weeks of work |
| Upstream tools change behaviour every release | Parity drifts | Pin per Pun release; harness runs nightly against new upstream releases and opens issues |
| Embedded PHP and FrankenPHP in one binary conflict (signals, threads) | `pun run` delayed | Sidecar fallback (decision 4) |
| Binary size grows past 100 MB with runtime and extensions | Slow installs | Core binary has no PHP; runtimes download on demand into a shared cache |
| Delegate mode is slower than the original tools (PHP boot per run) | Bad first impression on unported rules | Keep a warm embedded PHP per run, load PHARs once, show native coverage in `--explain` |
| Maintainers of Pint, Pest, PHPStan or Mago see Pun as hostile | Ecosystem friction | Credit and contribute upstream, use their test suites openly, talk to maintainers before launch |
| Scope (six products) overwhelms a small team | Nothing ships | Each phase ends in a release that is useful on its own; agents do the mechanical porting with the harness as judge |

## Success metrics and acceptance

Pun publishes a parity scoreboard on every main build, the way Bun quotes its Prettier pass rate. A release ships only when its row is green.

| Release | Parity metric | Speed metric |
| --- | --- | --- |
| v0.1 format | 100% byte-identical on Pinkary + corpus; 100% of laravel preset fixer fixtures native | 30x faster than `pint --test` on Pinkary |
| v0.2 lint | Identical error set on Pinkary; levels 0 to 6 identical on 20+ apps | 10x faster than PHPStan cold |
| v0.3 refactor | Identical dry-run diff on Pinkary; corpus round-trip printing identical | 10x faster than Rector cold |
| v0.4 migrate | Pinkary migrates with zero parity diffs; CI passes with only Pun | One command |
| v0.5 install | Identical composer.lock and vendor/ across the corpus | 3x faster than Composer cold |
| v0.6 run | Pinkary installed and served by Pun alone | No system PHP or Composer needed |
| v1.0 test | Identical results and type coverage on Pinkary and corpus suites | 2x faster than `pest --parallel` cold |

Also tracked per build: share of rules running natively per tool, peak memory, warm-cache times, and the number of corpus repos with any diff.

### The Pinkary acceptance test

Run in a clean container at a pinned Pinkary commit:

```bash
# old toolchain (oracle)
vendor/bin/pint --test; vendor/bin/rector --dry-run; vendor/bin/phpstan analyse; vendor/bin/pest --type-coverage --min=100; vendor/bin/pest --parallel

# Pun
pun format --check; pun lint --dry-run; pun analyse; pun test --type-coverage --min=100; pun test
```

Pass means: identical exit codes, identical formatted bytes after `pint` vs `pun format`, identical Rector diff, identical PHPStan error list, identical type coverage percentage, identical test outcome counts and failure list.

## Sources

- [Bun issue #2246: lint and format merged](https://github.com/oven-sh/bun/issues/2246)
- [Bun announcement: bun fmt](https://x.com/bunjavascript/status/2109861465100632567)
- [Mago repository](https://github.com/carthage-software/mago)
- [Mago site and benchmarks](https://mago.carthage.software/latest/en/)
- [Mago formatter configuration reference](https://mago.carthage.software/main/en/tools/formatter/configuration-reference/)
- [mago 1.41.0 Cargo.toml (crate list)](https://docs.rs/crate/mago/1.41.0/source/Cargo.toml)
- [mago-linter on docs.rs](https://docs.rs/mago-linter)
- [wick (Mago-based Pint-style formatter, compatibility note)](https://packagist.org/packages/cresset/wick)
- [Mago 1.28.0 release notes via dependabot](https://github.com/carthage-software/cel-php/pull/98)
- [Blade formatting in Laravel Pint](https://laravel-news.com/blade-formatting-in-laravel-pint)
- [PHP-CS-Fixer](https://github.com/PHP-CS-Fixer/PHP-CS-Fixer)
- [PHPStan 2.0 release](https://phpstan.org/blog/phpstan-2-0-released-level-10-elephpants)
- [Larastan releases](https://laraplugins.io/plugins/nunomaduro/laravel-code-analyse)
- [Rector 2.6.1 release notes](https://sourceforge.net/projects/rector.mirror/files/2.6.1/)
- [Pest 5 release](https://laravel-news.com/pest-5)
- [Pest on Packagist](https://packagist.org/packages/pestphp/pest)
- [Pest TIA docs](https://pestphp.com/docs/tia)
- [Pest support policy](https://pestphp.com/docs/support-policy)
- [FrankenPHP in production (php\[architect\])](https://www.phparch.com/2026/07/frankenphp-in-production-worker-mode-embedded-binaries-and-real-performance/)
- [ext-php-rs embedded PHP guide](https://docs.rs/crate/ext-php-rs/latest/source/guide/src/advanced/embedded_php.md)
- [bext-php (Rust SAPI with worker pool)](https://docs.rs/bext-php/)
- [Pinkary repository](https://github.com/pinkary-project/pinkary.com)
- [PHP 8.5 release page](https://www.php.net/releases/8.5/en.php)
