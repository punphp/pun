# Oracle

What every `pun format` change is judged against. Installed into `.oracle/` (gitignored)
by `scripts/oracle.sh`; corpus checkouts into `corpus/` by `scripts/corpus.sh`.

## Pinned versions

| component | version | ref |
|---|---|---|
| PHP | 8.5.11 (system, NTS) | |
| Composer | 2.10.3 | |
| laravel/pint | 1.29.3, `dev-feat/blade-support` (what Pinkary's composer.lock pins) | `cef1b260c611f92e66902d678092e26932c9ea00` |
| friendsofphp/php-cs-fixer | v3.95.17 (bundled in Pint's PHAR and its composer.lock) | `0ee88422118f3cc59c8c3def222ba7f1493b6d5b` |
| prettier / prettier-plugin-blade / prettier-plugin-tailwindcss | 3.9.10 / 3.3.3 / 0.8.1 (only for `Pint/laravel_blade`, installed into `corpus/pinkary/node_modules`) | |
| Node | v22.23.3 (system) | |

- Oracle binary: `php .oracle/pint/builds/pint` (the committed PHAR, which is what Pinkary runs).
- Pint source with dev deps: `.oracle/pint` (used by `scripts/pint-boot.php` to resolve configs).
- PHP-CS-Fixer directly: `php .oracle/php-cs-fixer/php-cs-fixer` (source checkout with `tests/`
  and PHPUnit, at the bundled ref). The copy inside Pint is `.oracle/pint/vendor/friendsofphp/php-cs-fixer`.
- Pint overrides `PhpCsFixer\FixerFactory` (`.oracle/pint/overrides/FixerFactory.php`) to register
  two custom fixers: `Pint/laravel_blade` (Prettier via Node) and `Pint/phpdoc_type_annotations_only`
  (priority 56, not enabled by any preset).

## Scripts

| script | what |
|---|---|
| `scripts/oracle.sh` | install/verify `.oracle/` (checks the PHAR bundles the pinned php-cs-fixer ref) |
| `scripts/corpus.sh [name...]` | shallow checkouts per `corpus/manifest.toml`; builds derived `pinkary-dirty` |
| `scripts/effective-rules.php [pint.json]` | effective fixer list (run order, options, priority, risky, class, file) |
| `scripts/rank.php <pint.json> <dir>...` | per-fixer changed-file counts, each fixer alone |
| `scripts/export-fixtures.php <effective.json> <outdir>` | upstream data-provider test cases to JSON |
| `scripts/perturb.php <dir>` | make a deliberately unformatted copy (anti-laravel config + token noise, `php -l` checked) |

## Harness

```
cargo build -p punphp-harness
target/debug/pun-harness format                         # every corpus project, regenerates docs/scoreboard.md
target/debug/pun-harness format --corpus pinkary        # one project (repeatable)
target/debug/pun-harness format --fixer single_quote --corpus pinkary-dirty --diffs 5
```

Per project: copies `corpus/<name>` twice into a temp dir (no `.git`; `node_modules` symlinked),
runs `php .oracle/pint/builds/pint` in one and `target/debug/pun format` in the other (missing
binary = identity), byte-compares every file, prints counts and the first `--diffs` unified diffs
(`--- oracle` / `+++ pun`). Exit 1 on any differing file.

`--fixer <name>` writes a pint.json with the project's finder keys, `"preset": "empty"` and only
that rule, configured with the options from the project's own effective rule list (`true` if the
project doesn't enable it), and passes it as `--config <file>` to both pint and pun. The scoreboard
is not touched in this mode.

## Fixtures

`crates/pun-harness/fixtures/<fixer>.json`: per test method with a `#[DataProvider]`, its params
(name/type/default), `#[RequiresPhp]`, `declared_in`, the PHP body (so you can see how args are
used, e.g. whitespace config), and every case `{provider, key, args}`. Objects are dumped as
`{"__class": ..., props}`; non-UTF-8 strings as `{"__base64": ...}`. Includes non-fix providers
(e.g. `provideInvalidConfigurationCases`); filter on `method`.
