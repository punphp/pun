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
Pint). Rule names in `pun.toml` are Pun IDs; earlier Pun names still work, and a PHP-CS-Fixer name gets an error naming
its Pun ID. `pun migrate` reads pint.json the way Pint does (`extend`, `--config`,
`--preset`) or, without one, a `.php-cs-fixer(.dist).php` through PHP-CS-Fixer itself, and writes a `pun.toml` with
Pun rule IDs ([context/RULES.md](context/RULES.md)): the preset plus what differs from it under `[format.rules]`, and the
risky fixers under `[lint.rules]` with `fix = "apply"`, since they changed behaviour unattended. It rewrites
`// @php-cs-fixer-ignore rule` comments to `// @pun-ignore-file format.rule`, lists any rule Pun can't run before
writing anything (and then writes nothing), and is idempotent. On a terminal it first lists the configs it found
(pint.json, .php-cs-fixer.php, phpstan.neon, rector.php, mago.toml) as a checklist, all checked, and migrates only
what you keep checked; `--from pint,phpstan` picks without asking, and `--yes`, agent mode or no terminal migrate
everything found. PHPStan, Rector and Mago are listed as "not supported yet" and left untouched. With no config at all
it writes a starter pun.toml (`preset = "recommended"`, `[format] preset = "laravel"`). `pun config` prints the
resolved configuration.

```toml
[format]
preset = "laravel"            # Pint's preset; its risky rules are lint rules
exclude = ["build"]

[format.rules]
concat-space = { spacing = "one" }
array-syntax = false

[lint.rules]
require-final-class = { fix = "apply" }   # applied by `pun lint --fix` / `pun check --fix`
no-loose-comparison = "warn"              # reported by `pun lint` / `pun check`, never applied
```

**Format, lint, check.** `pun format` runs only behaviour-safe rules. Risky PHP-CS-Fixer fixers
(`strict_comparison`, `final_class`, `declare_strict_types`, ...) are lint rules: `pun lint` reports what they would
change, `pun lint --fix` applies the ones with `fix = "apply"`. `pun check` runs format and lint in one pass
(fixers in PHP-CS-Fixer's priority order, as Pint runs them), so `pun check --fix` on a migrated project writes exactly
what `pint` wrote; `pun analyse` joins it once it exists. Output is Pint's for all three (outside coding agents, see below).

**Coding agents.** Under a coding agent (detected from the agent's environment variables with
[laravel/agent-detector](https://github.com/laravel/agent-detector)'s list, then the
[agent-detector](https://crates.io/crates/agent-detector) crate; Cursor's `CURSOR_TRACE_ID` alone doesn't count, as
Cursor sets it in human terminals too) every command prints a summary line and one line per finding instead of
Pint's output, with no colours, boxes or progress. The list stops after 20 items (`-v` shows all, with diffs):

```
pun check: fail, 1 error, 2 files with fixes (412 checked, 0.21s)
app/A.php format.array-syntax, format.blank-line-after-opening-tag
app/B.php format.array-syntax
app/B.php lint.no-alias-function [fix:unsafe] Master functions shall be used instead of aliases.
app/D.php:2 syntax-error Parse error: syntax error, unexpected token ";" on line 2.
```

A clean run prints only the summary line (`pun format: pass (412 checked, 0.21s)`); run errors are one line
(`pun format: error: ...`). Exit codes are the same as without an agent. An explicit `--format` keeps its reporter
(`--format=agent` is Pint's JSON). The override order is `--agent` / `--no-agent`, then `PUN_AGENT=1|0`, then PAO's
`PAO_DISABLE` / `PAO_FORCE`, then detection. Without an agent the output is unchanged.

**Rules Pun hasn't ported.** If an enabled rule has no native port, Pun runs every file through the real PHP-CS-Fixer in
long-lived `php` workers, using the project's `vendor/laravel/pint` (or `vendor/friendsofphp/php-cs-fixer`, or a Pint
PHAR in `$PUN_PINT`), so output stays identical to Pint while discovery, caching and reporting stay Pun's. `--explain`
(or `-v`) says why; `--native-only` skips those rules and lists them. Details: [context/reference/oracle.md](context/reference/oracle.md#delegate-mode).

**Extra rules.** Opt-in `Pun/` rules ported from Mago (Laravel and Pest idioms, redundant code, ...) are
listed in [context/reference/mago-extras.md](context/reference/mago-extras.md). They are lint rules (`no-only`, `prefer-fake-helper`, ...)
in no preset, so default output stays identical to Pint; enable them under `[lint.rules]`.
`pun lint --mago [paths...]` reports Mago's lint-only Laravel rules (no auto-fix, never edits).

**Watch.** `pun format --watch [paths...]` does a normal run, then re-formats files as they change (same
discovery, config and exclusions, Pint's output per run). Pun's own writes don't trigger runs.

**Editors.** `pun lsp` is a formatting-only language server on stdio: it formats the open document exactly like
`pun format --stdin` with the document's path, so the project's config and exclusions apply (config is read at
startup; restart the server after editing it).

- VS Code: any generic LSP client extension, with command `pun`, arguments `["lsp"]`, document selector / language
  `php`; then pick that extension as the default formatter for PHP.
- Neovim (0.11+):
  ```lua
  vim.lsp.config('pun', { cmd = { 'pun', 'lsp' }, filetypes = { 'php' }, root_markers = { 'pun.toml', 'composer.json', '.git' } })
  vim.lsp.enable('pun')
  -- format: vim.lsp.buf.format({ name = 'pun' }), e.g. from a BufWritePre autocmd
  ```
- PhpStorm (LSP4IJ plugin): Settings > Languages & Frameworks > Language Servers > `+`: name `Pun`, command
  `pun lsp`; under Mappings > File type add `PHP`. Reformat Code then goes through Pun.

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
