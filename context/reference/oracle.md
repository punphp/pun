# Oracle

What every `pun format` change is judged against. Installed into `.oracle/` (gitignored)
by `scripts/oracle.sh`; corpus checkouts into `corpus/` by `scripts/corpus.sh`.

## Pinned versions

| component | version | ref |
|---|---|---|
| PHP | 8.5.12 (system, NTS) | |
| Composer | 2.10.3 | |
| laravel/pint | v1.32.1 (the latest release; it ships the Blade formatter and `Pint/phpdoc_type_annotations_only`) | `006bd7825eb01daf92959d060b89cd9514fe4253` |
| friendsofphp/php-cs-fixer | v3.95.25 (bundled in Pint's PHAR and its composer.lock) | `2cdfc1f3daf173d83a1ebb6177949b58c95de6ff` |
| rector/rector | 2.8.4 (the latest release; Pinkary's composer.lock still pins 2.6.1; the released build in `.oracle/rector-bin`, locked by `scripts/oracle-rector/composer.lock`) | |
| phpstan/phpstan for Rector | 2.3.5 (both Rector oracle locks; rector/rector 2.8.4 requires `^2.3.5`; pinned separately from the analyse oracle's PHPStan) | |
| rectorphp/rector-src | 2.8.4 (source and `rules-tests/` fixtures in `.oracle/rector`) | `0ddd8e3c8de9c32e9c12396a7f50234bdba559d0` |
| rector/rector-phpunit | dev-main@22c6dae (bundled in rector/rector 2.8.4) | |
| driftingly/rector-laravel | 2.7.3 (`.oracle/rector-ecosystem`, locked by `scripts/oracle-rector-ecosystem/composer.lock`; its tests and stubs at source commit `699f25b`) | |
| pestphp/pest-plugin-rector | 5.x-dev (no release yet; `.oracle/rector-ecosystem`, same lock, `minimum-stability: dev` with `prefer-stable`) | `8c1b780c5e551cb78150ee91ceb73d34cd029946` |
| prettier / prettier-plugin-blade / prettier-plugin-tailwindcss | 3.9.10 / 3.3.3 / 0.8.1 (only for `Pint/laravel_blade`, installed into `corpus/pinkary/node_modules`) | |
| Node | v22.23.3 (system) | |

- Oracle binary: `php .oracle/pint/builds/pint` (the PHAR committed at the tag).
- Pint source with dev deps: `.oracle/pint` (used by `scripts/pint-boot.php` to resolve configs).
- PHP-CS-Fixer directly: `php .oracle/php-cs-fixer/php-cs-fixer` (source checkout with `tests/`
  and PHPUnit, at the bundled ref). The copy inside Pint is `.oracle/pint/vendor/friendsofphp/php-cs-fixer`.
- Rector: `php .oracle/rector-bin/vendor/bin/rector process --config <rector.php>` (see "Rector" below). Rule
  sources and fixtures: `.oracle/rector/rules/` and `.oracle/rector/rules-tests/`.
- Pint overrides `PhpCsFixer\FixerFactory` (`.oracle/pint/overrides/FixerFactory.php`) to register
  three custom fixers: `Pint/laravel_blade` (Prettier via Node, or Bun in a project with a Bun lock file),
  `Pint/phpdoc_type_annotations_only` (priority 56, not enabled by any preset) and `Pint/prettier_cache_fingerprint`
  (a no-op that carries the Prettier package versions into the cache signature; Pun has no port, nothing enables it).
  The override also wraps `no_unused_imports`, `fully_qualified_strict_types`, `global_namespace_import` and
  `declare_strict_types` so they skip `*.blade.php` files (Pun: `BLADE_UNSAFE` in `pun-format`). Pint 1.32 caches
  results (`setUsingCache(true)`, cache file in the system temp dir); it changes no output.

## Scripts

| script | what |
|---|---|
| `scripts/oracle.sh` | install/verify `.oracle/` (checks the PHAR bundles the pinned php-cs-fixer ref) |
| `scripts/corpus.sh [name...]` | shallow checkouts per `corpus/manifest.toml`; builds derived `pinkary-dirty` |
| `scripts/effective-rules.php [pint.json]` | effective fixer list (run order, options, priority, risky, class, file) |
| `scripts/export-fixtures.php <effective.json> <outdir>` | upstream data-provider test cases to JSON |
| `scripts/cli-parity.sh [--project DIR] [--tty [--cols N]] [--env K=V] "ARGS"...` | byte-compare Pint and `pun migrate` + `pun format` (stdout, stderr, exit code, files) per argument set, same directory |
| `scripts/perturb.php <dir>` | make a deliberately unformatted copy (anti-laravel config + token noise, `php -l` checked) |

## Rector

The lint oracle (lint.md §6). A Pun lint rule ports one Rector rule; its tests are the rule's own fixtures from
`.oracle/rector/rules-tests/<Set>/Rector/<Node>/<Rule>/Fixture*/`, copied verbatim into
`crates/pun-lint/tests/rector/<Rule>/`. A fixture is the input, a `-----` line and the expected output, or only the
input when the rule must leave it alone; the rule's `config/configured_rule.php` gives the PHP version. Parity is
`format(pun lint --fix) == format(expected)`.

Both Rector oracles pin PHPStan 2.3.5, the version Rector 2.8.4 resolves to. It is pinned apart from the analyse
oracle's PHPStan, and the two can differ in their types, so use the Rector oracle rather than the analyse oracle for
lint expectations.

For an input the fixtures miss, run Rector on it and paste its output as the expectation
(`crates/pun-lint/tests/oracle/`):

```php
<?php // rector.php next to a src/ holding the inputs
use Rector\Config\RectorConfig;
return RectorConfig::configure()
    ->withPaths([__DIR__.'/src'])
    ->withRules([\Rector\CodeQuality\Rector\Assign\CombinedAssignRector::class])
    ->withPhpVersion(80400);
```

`php .oracle/rector-bin/vendor/bin/rector process --no-progress-bar --no-diffs --clear-cache --config rector.php`

A case that uses PHPUnit (every rector-phpunit rule, and the core rules whose tests load `tests/phpunit-stubs`) needs
PHPUnit's classes loaded before Rector requires its own `stubs-rector/` (`BootstrapFilesIncluder::requireRectorStubs()`),
as in a project with PHPUnit installed. Otherwise the stub `TestCase`, which has only `createMock()`, wins over
`withAutoloadPaths()`. Without PHPUnit, inherited asserts, `createStub()` and `setUp()` are not found, and the output
looks like an upstream change but is not one. Load PHPUnit first with a bootstrap file that registers an autoloader
for `tests/phpunit-stubs`, through `->withBootstrapFiles([...])`. Rector also reads `vendor/composer/installed.json`
next to the config for the versions of the packages a rule depends on (PHPUnit 12.5, Laravel 13).

Autoload the classes the test names with `->withAutoloadPaths([...])`: the rule's `Source/`, and `Stubs/` where the
rule's test directory has one (RenameClassRector and AnnotationToAttributeRector hold rector-src's `stubs/` classes
there, Doctrine's among them). Without them Rector resolves no annotation class and leaves every Doctrine annotation
alone, which also looks like an upstream change but is not one.

## Harness

```
cargo build -p punphp-harness
target/debug/pun-harness format                         # every corpus project, regenerates context/reports/scoreboard.md
target/debug/pun-harness format --corpus pinkary        # one project (repeatable)
target/debug/pun-harness format --fixer single_quote --corpus pinkary-dirty --diffs 5
```

Per project: copies `corpus/<name>` twice into a temp dir (no `.git`; `node_modules` symlinked),
runs `php .oracle/pint/builds/pint` in one and `target/debug/pun migrate` then `pun check --fix` in the other
(missing binary = identity; so every run also proves the migration), byte-compares every file (pun.toml aside), prints counts and the first `--diffs` unified diffs
(`--- oracle` / `+++ pun`). Exit 1 on any differing file.

`--fixer <name>` writes a pint.json with the project's finder keys, `"preset": "empty"` and only
that rule, configured with the options from the project's own effective rule list (`true` if the
project doesn't enable it), and passes it as `--config <file>` to pint and to `pun migrate`. The scoreboard
is not touched in this mode.

## No delegate mode

Delegate mode (crate `pun-delegate`, `--native-only`, `PUN_PINT`) ran rules Pun hadn't ported through the real
PHP-CS-Fixer in `php` workers. It is removed (PLAN.md §2 "No PHP fallback"; in git history). A rule in `pun.toml`
without a native fixer now stops `pun format`, `pun lint` and `pun check` with exit 2 and
the line `<pun.toml>: rule not supported by Pun yet: <ids>`. System `php` stays the test oracle, and `pun migrate` runs it
once to read a `.php-cs-fixer(.dist).php`'s resolved rules through the project's own PHP-CS-Fixer
(`crates/pun-cli/src/php_cs_fixer_rules.php`).

## Fixtures

`crates/pun-harness/fixtures/<fixer>.json`: per test method with a `#[DataProvider]`, its params
(name/type/default), `#[RequiresPhp]`, `declared_in`, the PHP body (so you can see how args are
used, e.g. whitespace config), and every case `{provider, key, args}`. Objects are dumped as
`{"__class": ..., props}`; non-UTF-8 strings as `{"__base64": ...}`. Includes non-fix providers
(e.g. `provideInvalidConfigurationCases`); filter on `method`.

## CLI output parity

`scripts/cli-parity.sh` and `crates/pun-cli/tests/output.rs` compare every output mode against the PHAR: summary,
`-v` diffs and syntax-error snippets, `-q`, `--format=` json/txt/agent/checkstyle/junit/xml/gitlab (stdout and
`--output-to-file`), colours (`--ansi`, `--no-ansi`, TTY via `script`, NO_COLOR/FORCE_COLOR, Collision's TTY-only
highlighting), terminal width (COLUMNS or `stty -a`), abort boxes and the prettier pre-flight check.

Syntax-error messages are PHP's own: for a file that fails to lint Pun runs the local `php` once
(`token_get_all(..., TOKEN_PARSE)`); without `php` it falls back to its parser's wording (same layout).
The gitlab reporter's fixer descriptions/docs paths are generated: `php scripts/fixer-metadata.php --gitlab >
crates/pun-cli/src/gitlab_fixers.json`.

Intentional differences:

- `-vvv`: Pint's PHAR wrapper (Box) prints a "Box Requirements Checker" block on stderr. It is not Pint output;
  Pun doesn't print it (cli-parity.sh strips it before comparing).
- Missing prettier packages for `Pint/laravel_blade`: interactive Pint asks whether to install them; Pun always
  takes the non-interactive answer (no) and aborts with Pint's message.
- Cache: Pint 1.32 caches (`setUsingCache(true)`) and writes PHP-CS-Fixer's cache file to `--cache-file`, pint.json's
  `cache-file` or the system temp dir. The file records the PHP runtime's version, and Pun has no cache, so Pun writes
  no such file; every run checks every file. Output is the same: a dry run caches no file that needs fixing.
- Exceptions other than Pint's `abort()` (unreadable path, bad config) are rendered by Symfony with an
  "In File.php line N:" header naming PHP source; Pun prints the message without that box.

## PHPStan (Phase 2, crate `pun-phpstan`)

| component | version | ref |
|---|---|---|
| phpstan/phpstan | 2.3.5 (the latest release, Pun's parity target; PHAR, vendor prefixed `_PHPStan_95d9e8b05\`) | phpstan-src `95d9e8b0578ebf75439a3744a845ef25417028fa` |
| larastan/larastan | v3.14.0 (the latest release) | |
| phpstan/phpstan-deprecation-rules | ^2.0 (Pinkary requires it) | |
| nette/neon | v3.3.4 (bundled in the PHAR) | `bb88bf3a54dd21bf4dbddb5cd525d7b0c61b7cda` |
| nette/di / nette/schema | 3.1.10 / 1.2.5 (bundled in the PHAR) | |

- `.oracle/phpstan`: installed by `scripts/oracle.sh` from the committed `scripts/oracle-phpstan/composer.{json,lock}` (exactly `phpstan/phpstan: 2.3.5`, `larastan/larastan: 3.14.0`,
  `phpstan/phpstan-deprecation-rules: ^2.0`; `composer install` there. Oracle binary:
  `php .oracle/phpstan/vendor/phpstan/phpstan/phpstan.phar`. Its `vendor/` also serves the corpus parity
  tests (symlinked into a farm, so `%rootDir%` and `includes: vendor/...` resolve as in a real project).
- `.oracle/neon`: nette/neon source + tests at the bundled tag (`git fetch --depth 1 https://github.com/nette/neon.git v3.3.4`).
- `.oracle/phpstan-src`: phpstan-src at the tag 2.3.5 (`scripts/oracle.sh` fetches it; its `tests/` give data
  providers, its `src/Rules/` and `conf/` the PHPStan inventory).
- phpstan/phpstan-strict-rules 2.0.12 and phpstan/phpstan-deprecation-rules 2.0.5 are the versions the rule mapping's
  inventory read (`crates/pun-config/rules/upstream/phpstan-extensions.json`).
- `.oracle/phpstan-phar`: an extracted copy of the PHAR (`scripts/oracle.sh` extracts it), for reading only (nothing runs it).

All exporters run the PHAR's real classes (private methods through reflection, test data providers with stub
parents); expected values are never written by hand. Re-run one after touching its cases, then the tests.

| script | writes (under `crates/pun-phpstan/tests/fixtures/`) |
|---|---|
| `php scripts/export-neon-fixtures.php .oracle/neon > neon.json` | every Neon decode/encode nette/neon's `.phpt` tests make; with `neon-extra-inputs.json` as 2nd arg, decodes those strings (`neon-extra.json`) |
| `php scripts/phpstan-config.php <phar> <cwd> [-c f] [-l n] [--generated-config f] [--generate-baseline f] [path...]` | (stdout) resolved parameters + allConfigFiles, or PHPStan's stderr error, as `phpstan analyse` gets them |
| `php scripts/export-phpstan-config-fixtures.php [case...]` | `config/<case>/expected.json` from `case.json`, and `config-corpus/<name>.json` (pinkary, larastan, laravel-pint, laravel-framework-src/-types) |
| `php scripts/export-schema-fixtures.php` | `schema.json`: parametersSchema validation (ContainerFactory::validateParameters) |
| `php scripts/export-baseline-fixtures.php` | `baseline.json`: BaselineNeon/PhpErrorFormatter output and what PHPStan reads back |
| `php scripts/export-ignore-fixtures.php` | `comments.json` (RichParser::getLinesToIgnore), `local-processor.json` (LocalIgnoresProcessor), `ignore.json` (IgnoredErrorHelper + result processing) |
| `php scripts/export-local-ignore-fixtures.php` | `local.json`: real `phpstan analyse` on `local/*.php`, comments blanked vs. kept |
| `php scripts/export-php-helpers-fixtures.php` | `php.json`: regex (Nette Strings::match), preg_quote, FileHelper, relative paths, fnmatch, (string) casts, `phpfile/ok-*.php` |
| `php scripts/export-phpstan-symbols.php` | `../data/symbols.json` (every built-in function, method, class and constant as PHPStan 2.3.5 reflects them on PHP 8.4, with the 8.5 differences), `type-strings.json` (every signature-map type string through TypeStringResolver + `describe()`), `reflected-types.json` (every type in those signatures, `toPhpDocNode()` and its descriptions) |
| `php scripts/export-phpstan-symbols.php probes` | `type-probes.json` (hand-picked and generated type strings: descriptions and iterable key/value), `type-super.json` (`isSuperTypeOf()` of every ordered pair within each atom set) |

### How PHPStan resolves a built-in function's signature

`NativeFunctionReflectionProvider` asks `SignatureMapProviderFactory` for a provider: on PHP >= 8.0 that is
`Php8SignatureMapProvider`, which reads the function (or method) from the bundled php-8-stubs
(`vendor/phpstan/php-8-stubs/stubs`, picking the declaration whose `#[Since]`/`#[Until]` attributes match
`phpVersion`). If `resources/functionMap.php` (with the `functionMap_php74delta.php` ... `php85delta.php`
deltas applied up to `phpVersion`) also has the name, its typed variants are merged over the stub's native one
(`getMergedSignatures()`: functionMap types refine, variants come from the map). A name without a php-8 stub
falls back to the functionMap alone (`FunctionSignatureMapProvider`). PHPDoc types come from
`StubPhpDocProvider` (PHPStan's own `stubs/*.stub`), and each parameter and the return become
`TypehintHelper::decideType(signature type, PHPDoc type)`; the reflection's doc comment (jetbrains/phpstorm-stubs,
through BetterReflection) only adds `@throws` and deprecation. `resources/functionMetadata.php`
adds `hasSideEffects`/purity. Dynamic return type extensions are not part of the signature (M2.3).

Pun does not re-implement that chain: the exporter asks PHPStan's own `ReflectionProvider` for every built-in
and stores the result (`crates/pun-phpstan/data/symbols.json`); `pun-phpstan`'s type parser (`phpdoc.rs`),
resolver (`resolve.rs`) and type model (`types.rs`: `TypeCombinator` union/intersect, `isSuperTypeOf`,
`describe()`) read it back and are tested against the fixtures above.

### `pun analyse` (crate `pun-analyse`, M2.3)

Rule fixtures and corpus parity run the real PHAR at level 0. Two environment traps, both handled by the scripts:

- PHPStan 2.3.5 detects AI agents (`src/Internal/AgentDetector.php`: `AI_AGENT`, `CLAUDECODE`,
  `AMP_CURRENT_THREAD_ID`, ...) and switches its output format; the scripts unset those variables.
- PHP 8.5's `max_memory_limit` (512M in php.ini) caps `--memory-limit`, also in worker processes, which don't
  inherit `-d`. The scripts set `PHP_INI_SCAN_DIR=:target-analyse/php-ini` with `max_memory_limit = -1` there.

| script | writes |
|---|---|
| `php scripts/export-rule-fixtures.php [--resume] [--budget=SECONDS]` | `crates/pun-analyse/tests/fixtures/rules/<Dir>/<Test>.json`: every `analyse()` call of every level-0 RuleTestCase (data providers expanded, `RequiresPhp` applied), re-run as `phpstan analyse --level 0` on the same files, plus copies of the data files under `data/` |
| `php scripts/export-phpstan-api-stub.php > crates/pun-analyse/data/phpstan-api.php` | the PHAR's unprefixed classes (PHPStan\, PhpParser\ with its `class_alias()` compatibility names, phpdoc-parser, BetterReflection), bodies stripped; Pun scans it when an analysed file names those namespaces |
| `python3 scripts/analyse-corpus.py setup\|oracle\|diff\|rank <project>` | corpus checkouts under `target-analyse/corpus` (larastan at tag v3.14.0), PHPStan's level-0 JSON in `target-analyse/oracle/<p>.json` (cold, one worker), and the per-identifier diff against `examples/analyse` |

`tests/rules.rs` compares only the identifiers `pun_analyse::IDENTIFIERS` lists; cases that still differ are
listed, with reasons, in `tests/fixtures/rules/gaps.txt` (the list must be exact).

What Mago 1.57 provides: parsing, name resolution (`ResolvedNames`), the codex (`scan_program` +
`populate_codebase`, with the prelude for built-ins), and per-expression types from
`Analyzer::analyze_with_artifacts` (`AnalysisArtifacts::expression_types`, final types per span; no scope
snapshot per statement). Gaps found:

- `MetadataFlags::USER_DEFINED` is set only for Host files: vendored symbols need `!is_built_in()`.
- Constructors are inherited only from "consistent" parents, and private parent methods are dropped; Pun's own
  `rules/members.rs` follows `ClassReflection::getMethod()` (own, trait with `as` aliases, parents, interfaces;
  built-in classes from `symbols.json`).
- `CONSISTENT_CONSTRUCTOR` is set by both `@consistent-constructor` and a final constructor.
- An object of an unknown class is typed `mixed`, so `new $object` on it can't report the missing class.
- No class aliases (`class_alias()`), so the API stub declares them as subclasses; Laravel facade aliases that
  Larastan registers at runtime (`use Blade;`) are not known yet.
- Built-ins come from PHPStan's reflection (`symbols.json`), not Mago's prelude, for every rule decision.

## The rule mapping's inventories

`crates/pun-config/rules/upstream/{rector-core,rector-ecosystem,phpstan-core,phpstan-extensions}.json` list every
upstream rule at the pinned versions above: its class, package, sets (Rector) or identifiers and level (PHPStan),
whether a planned set registers it (`in_scope`, with `out_of_scope_reason`), and whether it is deprecated. Deprecation
was read from the source (`DeprecatedInterface`, `@deprecated`) at the pinned tag and at rector-src 2.7.0 and main, and
from getrector.com's rule pages. `scripts/check-rule-mapping.py` checks mapping.toml against them (rules.md §5, §7).
Rector core and rector-phpunit come from `.oracle/rector-bin`, rector-laravel and Pest from `.oracle/rector-ecosystem`,
PHPStan from `.oracle/phpstan` and `.oracle/phpstan-src`.
