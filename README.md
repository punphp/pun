# Pun

Pun formats, lints, and analyses PHP 8.4+ projects. It is written in Rust and reads one configuration file, `pun.toml`.

| Command | Purpose |
| --- | --- |
| `pun format` | Format code with Pint's layout rules |
| `pun lint` | Report code issues; `--fix` applies available fixes |
| `pun analyse` | Report type errors without editing files |
| `pun check` | Run format, lint, and analyse; `--fix` applies format and lint fixes |
| `pun migrate` | Convert existing tool configs to `pun.toml` |
| `pun init` | Create `pun.toml` for a new project |

**Status: beta.** `pun format` matches Pint byte for byte on the parity corpus. `pun lint` and `pun analyse` are
previews: PHPStan, Larastan, and Rector support is incomplete.

Pint compatibility is checked against pinned upstream tools on a project corpus and with mutation fuzzing. See the [parity report](context/reports/scoreboard.md) and the rule reference under [docs](docs/content/docs/index.mdx).

## Install

```sh
composer require --dev punphp/pun
vendor/bin/pun --version
vendor/bin/pun check
```

The package (`composer.json` and `composer/bin/pun`) is a PHP shim that downloads the native binary for your platform on first run, checks its SHA-256 checksum, and runs it (Linux and macOS, x86_64 and arm64).

PHP files need no PHP or Node runtime. Blade formatting requires Node.js (or Bun) and the Prettier packages listed in the [installation guide](docs/content/docs/01-installation.mdx).

## Start a project

Commands below are written as `pun`; in a project, run them as `vendor/bin/pun`.

For an existing Pint, PHP-CS-Fixer, Rector, or PHPStan configuration:

```sh
pun migrate --dry-run
pun migrate
```

Migration reports unsupported items and writes the supported configuration. Review those warnings before replacing your existing tools.
For a project without tool configs, run `pun init` instead.

Check the project, then apply available fixes:

```sh
pun check
pun check --fix
```

`pun format` changes layout only. Risky and code-deleting Pint fixers belong to lint, so use `pun check --fix` to apply a migrated Pint configuration.

## Configure rules

Rule IDs use `<command>/<group>/<name>`. Omit the command inside its rules table:

```toml
[format]
preset = "laravel"
exclude = ["build"]

[format.rules]
"operators/concat-spacing" = { spacing = "one" }
"arrays/syntax" = false

[lint.rules]
"style/require-final-class" = "error"
"types/no-loose-comparison" = { level = "warn", fix = "off" }
```

`pun config` prints the resolved configuration. `pun explain <rule>` describes a rule and its upstream names.
Pun never falls back to PHP for an unsupported rule. Enabling one is a configuration error.

Checks exit 0 when clean, 1 for findings, and 2 when Pun cannot run. Coding agents get compact output automatically.
See the [command reference](docs/content/docs/commands/format.mdx), [configuration reference](docs/content/docs/configuration/pun-toml.mdx), and [CI guide](docs/content/docs/guides/ci.mdx).

## Develop

Building Pun itself needs Rust 1.99 or later: `cargo build --release -p punphp` writes `target/release/pun`. [AGENTS.md](AGENTS.md) lists the checks and the upstream oracles.

## Credits

Pun would not be possible without [Mago](https://github.com/carthage-software/mago), [PHP-CS-Fixer](https://github.com/PHP-CS-Fixer/PHP-CS-Fixer), [Laravel Pint](https://github.com/laravel/pint), [PHPStan](https://github.com/phpstan/phpstan), [Larastan](https://github.com/larastan/larastan), and [Rector](https://github.com/rectorphp/rector).

A special thank-you to Mago's maintainers and contributors. Pun uses Mago's parser and analysis libraries directly and ports rules from its linter.

To the people behind all these projects: thank you for the years of care in your code, tests, and documentation. Pun has come this far because you shared your work.

Pun ports behaviour, test fixtures and data from these projects. Their licences apply to the ported parts; the copyright notices and licence texts are in [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md), and those of the Rust crates in the binary are in [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md).

| Project | Used for | Licence |
| --- | --- | --- |
| PHP-CS-Fixer | Fixer logic and upstream fixtures (pun-tokens, pun-fixers) | MIT |
| Laravel Pint | Presets, custom fixers, CLI output | MIT |
| php-src | Parser tables generated from `Zend/zend_language_parser.y` (pun-format lint), union-type sorting (pun-phpstan) | Zend Engine License 2.00 |
| PHPStan, nette/neon, nette/schema, nette/di | Config, NEON, schema and baseline behaviour (pun-phpstan) | MIT / BSD-3-Clause |
| Larastan | Laravel analysis behaviour and test fixtures (pun-analyse) | MIT |
| Rector, rector-phpunit, rector-laravel, pest-plugin-rector | Lint rules and their fixtures (pun-lint) | MIT |
| PHPUnit, nikic/php-parser | Declarations the lint fixtures need (pun-lint tests) | BSD-3-Clause |
| JetBrains phpstorm-stubs | Built-in symbols (through PHPStan and Mago) | Apache-2.0 |
| PCRE2 | Regular expressions (linked statically) | BSD-3-Clause WITH PCRE2-exception |
| sebastian/diff | Diff algorithm (pun-cli php_diff) | BSD-3-Clause |
| Mago | Parser, analyser and linter crates (dependencies; mago-analyzer, mago-codex, mago-syntax and mago-phpdoc-syntax vendored with patches under `third_party/`) | MIT / Apache-2.0 |
| agent-detector (crates.io) | Coding-agent detection for agent output (pun-cli, dependency) | MIT |
| dialoguer, console (crates.io) | `pun migrate` checklist (pun-cli, dependency) | MIT |
