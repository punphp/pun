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

Pint compatibility is checked against pinned upstream tools on a project corpus and with mutation fuzzing.
PHPStan, Larastan, and Rector support is incomplete. See the [parity report](context/reports/scoreboard.md) and the rule reference under [docs](docs/content/docs/index.mdx).

## Build from source

Requires Rust 1.99 or later. Release packages are not published yet.

```sh
cargo install --locked --git https://github.com/punphp/pun punphp
pun --version
```

PHP files need no PHP or Node runtime. Blade formatting requires Node.js and the Prettier packages listed in the [installation guide](docs/content/docs/01-installation.mdx).

## Start a project

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
