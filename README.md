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
pun migrate             # once: pint.json (or .php-cs-fixer.php) -> pun.toml; --dry-run shows the diff
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
Pun rule IDs ([docs/RULES.md](docs/RULES.md)): the preset plus what differs from it under `[format.rules]`, and the
risky fixers under `[lint.rules]` with `fix = "apply"`, since they changed behaviour unattended. It rewrites
`// @php-cs-fixer-ignore rule` comments to `// @pun-ignore-file format.rule`, lists any rule Pun can't run before
writing anything (and then writes nothing), and is idempotent. `pun config` prints the resolved configuration.

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
what `pint` wrote; `pun analyse` joins it once it exists. Output is Pint's for all three.

**Rules Pun hasn't ported.** If an enabled rule has no native port, Pun runs every file through the real PHP-CS-Fixer in
long-lived `php` workers, using the project's `vendor/laravel/pint` (or `vendor/friendsofphp/php-cs-fixer`, or a Pint
PHAR in `$PUN_PINT`), so output stays identical to Pint while discovery, caching and reporting stay Pun's. `--explain`
(or `-v`) says why; `--native-only` skips those rules and lists them. Details: [docs/oracle.md](docs/oracle.md#delegate-mode).

**Extra rules.** Opt-in `Pun/` rules ported from Mago (Laravel and Pest idioms, redundant code, ...) are
listed in [docs/MAGO-EXTRAS.md](docs/MAGO-EXTRAS.md). They are lint rules (`no-only`, `prefer-fake-helper`, ...)
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

Per-project parity with the pinned Pint (benchmarks to follow): [docs/scoreboard.md](docs/scoreboard.md).
