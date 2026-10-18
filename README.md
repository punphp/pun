# Pun

Pun is a PHP code formatter written in Rust that produces byte-for-byte the same output as
[Laravel Pint](https://github.com/laravel/pint), only much faster. `pun migrate` turns your `pint.json` into a
`pun.toml`, Pun ports Pint's and PHP-CS-Fixer's fixers natively, and it is checked against the real Pint on real
projects (Pinkary, laravel/framework, Pest, Larastan, ...) on every commit.

## Install (after v0.1)

```sh
# Shell (Linux, macOS)
curl -LsSf https://github.com/punphp/pun/releases/latest/download/punphp-installer.sh | sh

# Homebrew
brew install punphp/tap/pun

# npm
npm install --save-dev @punphp/pun

# Composer (downloads the native binary on first run)
composer require --dev punphp/pun
```

Prebuilt binaries: Linux x64/arm64 (glibc and musl), macOS x64/arm64.

## Usage

```sh
pun migrate             # once: pick the detected configs to migrate -> pun.toml; --dry-run shows the diff
pun check --fix         # what `pint` did: format + the risky rules pint.json enabled
pun format              # layout only (behaviour-safe; what format-on-save runs)
pun format --test       # check only; exits 1 if anything would change
pun format app/Models   # specific paths; Pint's flags (--dirty, --diff=main, --blade, --format, ...) work too
pun lint [--fix]        # the [lint] rules (today: PHP-CS-Fixer's risky fixers)
```

**Config.** Pun reads only `pun.toml`. Without one every command stops with "No pun.toml found. Run `pun migrate`."
(unless `--preset` or `--no-config` is given: both explicitly ask for a built-in preset, `--no-config` for laravel, as in
Pint; `--preset` takes `laravel`, `psr12` or `empty`, and Pint's `per` and `symfony` exit 2 naming `empty` plus
`[format.rules]`, which `pun migrate --preset per` writes). Rule names in `pun.toml` are Pun IDs (`<command>/<group>/<name>`, the command implied inside its own table); there are
no aliases, so an earlier Pun name or a PHP-CS-Fixer name gets an error naming its Pun ID. `pun migrate` reads pint.json the way Pint does (`extend`, `--config`,
`--preset`) or, without one, a `.php-cs-fixer(.dist).php` through PHP-CS-Fixer itself, and writes a `pun.toml` with
Pun rule IDs ([context/plan/rules.md](context/plan/rules.md)): the preset plus what differs from it under `[format.rules]`, and the
risky and code-deleting fixers under `[lint.rules]` at `"error"`, so `pun check --fix` applies them as Pint did. It rewrites
`// @php-cs-fixer-ignore rule` comments to `// @pun-ignore-file format/<group>/<name>`, lists any rule Pun can't run before
writing anything (and then writes nothing), and is idempotent. On a terminal it first lists the configs it found
(pint.json, .php-cs-fixer.php, phpstan.neon, rector.php, mago.toml) as a checklist, all checked, and migrates only
what you keep checked; `--from pint,phpstan` picks without asking, and `--yes`, agent mode or no terminal migrate
everything found. PHPStan, Rector and Mago are listed as "not supported yet" and left untouched. With no config at all
it writes a starter pun.toml (`[format] preset = "laravel"`). `pun config` prints the
resolved configuration.

```toml
[format]
preset = "laravel"            # Pint's preset; its risky and code-deleting rules are lint rules
exclude = ["build"]

[format.rules]
"operators/concat-spacing" = { spacing = "one" }
"arrays/syntax" = false

[lint.rules]
"style/require-final-class" = "error"                           # applied by `pun lint --fix` / `pun check --fix`
"types/no-loose-comparison" = { level = "warn", fix = "off" }   # reported by `pun lint` / `pun check`, never applied
```

**Format, lint, check.** `pun format` runs only behaviour-safe rules that change layout, casing or
spelling. Risky PHP-CS-Fixer fixers (`strict_comparison`, `final_class`, `declare_strict_types`, ...) and the 40 that
delete or restructure code (`no_unused_imports`, `no_superfluous_phpdoc_tags`, ...) are lint rules: `pun lint` reports
what they would change, `pun lint --fix` applies every enabled rule but those with `fix = "off"`. `pun check` runs format and lint in one pass
(fixers in PHP-CS-Fixer's priority order, as Pint runs them), so `pun check --fix` on a migrated project writes exactly
what `pint` wrote; `pun analyse` joins it once it exists. The human output is Pun's own (one line per changed file,
one summary line); exit codes and the `--format` reporters are Pint's.

**Coding agents.** Under a coding agent (detected from the agent's environment variables with
[laravel/agent-detector](https://github.com/laravel/agent-detector)'s list, then the
[agent-detector](https://crates.io/crates/agent-detector) crate; Cursor's `CURSOR_TRACE_ID` alone doesn't count, as
Cursor sets it in human terminals too) every command prints a summary line and one line per finding instead of
the human output, with no colours, boxes or progress. The list stops after 20 items (`-v` shows all, with diffs):

```
pun check: fail, 1 error, 2 files with fixes (412 checked, 0.21s)
app/A.php format/arrays/syntax, format/tags/blank-line-after-opening
app/B.php format/arrays/syntax
app/B.php lint/style/no-alias-function [fix:unsafe] Master functions shall be used instead of aliases.
app/D.php:2 syntax-error Parse error: syntax error, unexpected token ";" on line 2.
```

A clean run prints only the summary line (`pun format: pass (412 checked, 0.21s)`); run errors are one line
(`pun format: error: ...`). Exit codes are the same as without an agent. An explicit `--format` keeps its reporter
(`--format=agent` is Pint's JSON). The override order is `--agent` / `--no-agent`, then `PUN_AGENT=1|0`, then PAO's
`PAO_DISABLE` / `PAO_FORCE`, then detection. Without an agent the output is unchanged.

**Rules Pun hasn't ported.** Pun never runs PHP for a rule it lacks. A `pun.toml` that enables one stops `pun format`,
`pun lint` and `pun check` with exit code 2 and a line naming the rule (`rule not supported by Pun yet: ...`). Remove
it, or send a PR.

**Extra rules.** Opt-in `Pun/` rules ported from Mago (Laravel and Pest idioms, redundant code, ...) are
listed in [context/reference/mago-extras.md](context/reference/mago-extras.md). They are lint rules (`pest/no-only`, `laravel/prefer-fake-helper`, ...)
in no preset, so default output stays identical to Pint; enable them under `[lint.rules]`.
`pun lint --mago [paths...]` reports Mago's lint-only Laravel rules (no auto-fix, never edits).

**Watch.** `pun format --watch [paths...]` does a normal run, then re-formats files as they change (same
discovery, config and exclusions, the usual output per run). Pun's own writes don't trigger runs.

## Parity

Per-project parity with the pinned Pint (benchmarks to follow): [context/reports/scoreboard.md](context/reports/scoreboard.md).

## Credits

Pun ports behaviour, test fixtures and data from these projects. Their licences apply to the ported parts.

| Project | Used for | Licence |
| --- | --- | --- |
| PHP-CS-Fixer | Fixer logic and upstream fixtures (pun-tokens, pun-fixers, pun-harness/fixtures) | MIT |
| Laravel Pint | Presets, custom fixers, CLI output | MIT |
| php-src | Parser tables generated from `Zend/zend_language_parser.y` (pun-format lint) | PHP License 3.01 |
| PHPStan, nette/neon, nette/schema, nette/di | Config, NEON, schema and baseline behaviour (pun-phpstan) | MIT / BSD-3-Clause |
| sebastian/diff | Diff algorithm (pun-cli php_diff) | BSD-3-Clause |
| Mago | Parser, formatter, linter crates (dependencies) | MIT / Apache-2.0 |
| agent-detector (crates.io) | Coding-agent detection for agent output (pun-cli, dependency) | MIT |
| dialoguer, console (crates.io) | `pun migrate` checklist (pun-cli, dependency) | MIT |
