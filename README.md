# Pun

Pun is a PHP code formatter written in Rust that produces byte-for-byte the same output as
[Laravel Pint](https://github.com/laravel/pint), only much faster. It reads your existing `pint.json`, ports
Pint's and PHP-CS-Fixer's fixers natively, and is checked against the real Pint on real projects (Pinkary,
laravel/framework, Pest, Larastan, ...) on every commit.

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
pun format              # fix the project, like `pint`
pun format --test       # check only; exits 1 if anything would change
pun format app/Models   # specific paths; Pint's flags (--dirty, --diff=main, --blade, --format, ...) work too
```

**`pint` alias.** Invoked as `pint`, Pun behaves exactly like Pint (same flags, same output, ignores
`pun.toml`). Opt in with a symlink, e.g. `ln -s "$(command -v pun)" ~/.local/bin/pint`.

**Config.** Pun reads `pint.json` as is. `pun config --init` writes an equivalent `pun.toml` so it can be the
only config file; `pun config --show` prints the resolved configuration. `pun.toml` settings override
`pint.json`.

**Rules Pun hasn't ported, custom fixers.** If an enabled rule has no native port, or the project configures
PHP-CS-Fixer with `.php-cs-fixer.php` (custom fixer classes), Pun runs every file through the real PHP-CS-Fixer in
long-lived `php` workers, using the project's `vendor/laravel/pint` (or `vendor/friendsofphp/php-cs-fixer`, or a Pint
PHAR in `$PUN_PINT`), so output stays identical to Pint while discovery, caching and reporting stay Pun's. `--explain`
(or `-v`) says why; `--native-only` skips those rules and lists them. Details: [docs/oracle.md](docs/oracle.md#delegate-mode).

**Extra rules.** Opt-in `Pun/` rules ported from Mago (Laravel and Pest idioms, redundant code, ...) are
listed in [docs/MAGO-EXTRAS.md](docs/MAGO-EXTRAS.md). None is in a preset, so default output stays identical
to Pint; enable them in `pun.toml`.

**Lint.** `pun lint --mago [paths...]` reports Mago's lint-only Laravel rules (no auto-fix, never edits).

**Watch.** `pun format --watch [paths...]` does a normal run, then re-formats files as they change (same
discovery, config and exclusions, Pint's output per run). Pun's own writes don't trigger runs.

**Editors.** `pun lsp` is a formatting-only language server on stdio: it formats the open document exactly like
`pun format --stdin` with the document's path, so the project's config and exclusions apply (config is read at
startup; restart the server after editing it).

- VS Code: any generic LSP client extension, with command `pun`, arguments `["lsp"]`, document selector / language
  `php`; then pick that extension as the default formatter for PHP.
- Neovim (0.11+):
  ```lua
  vim.lsp.config('pun', { cmd = { 'pun', 'lsp' }, filetypes = { 'php' }, root_markers = { 'pint.json', 'pun.toml', 'composer.json', '.git' } })
  vim.lsp.enable('pun')
  -- format: vim.lsp.buf.format({ name = 'pun' }), e.g. from a BufWritePre autocmd
  ```
- PhpStorm (LSP4IJ plugin): Settings > Languages & Frameworks > Language Servers > `+`: name `Pun`, command
  `pun lsp`; under Mappings > File type add `PHP`. Reformat Code then goes through Pun.

## Parity

Per-project parity with the pinned Pint (benchmarks to follow): [docs/scoreboard.md](docs/scoreboard.md).
