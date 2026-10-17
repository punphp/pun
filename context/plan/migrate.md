# Plan: `pun migrate`

Status: **decided, not built for the new design** (2026-10-17). Decisions from the format, lint, analyse, init and
command walkthroughs, gathered here. Decisions with dates: ../PLAN.md §2.

## 1. What it is

`pun migrate` moves a project's pint.json, .php-cs-fixer(.dist).php, rector.php and phpstan.neon onto one pun.toml.
It moves everything Pun supports, and names everything it doesn't. The proof is the harness: `pun migrate` followed by
`pun check --fix` gives the same files as the old tools.

## 2. Today (pun-cli migrate.rs, pun-config migrate.rs, main at 7a76be5)

- Interactive picker: detects pint.json, .php-cs-fixer.php, phpstan.neon, rector.php and mago.toml, shows a checklist
  on a terminal; `--from pint,phpstan` and `--yes` skip it, as do agent mode and no terminal.
- pint.json and `.php-cs-fixer(.dist).php` become `[format]` as a delta to the preset, risky fixers become
  `[lint.rules]` with `fix = "apply"`; `@php-cs-fixer-ignore` is rewritten to `@pun-ignore-file`; idempotent.
- PHPStan, Rector and Mago are listed as "not supported yet". No config at all writes a starter pun.toml.
- Unsupported format rules stop the migration (they used to go through delegate mode).

## 3. The decided behaviour

**Unsupported items are named, skipped, and Pun asks for a PR.** There is no fallback to PHP:
```
 ✔ pint.json        → [format] preset laravel, 6 rules changed, 3 risky rules → [lint]
 ✔ rector.php       → [lint] sets dead-code, code-quality; 4 rules
 ✔ phpstan.neon     → [analyse] 212 rules (level 6), 14 ignoreErrors
 ⚠ phpdoc_tag_casing                   not supported by Pun yet; skipped
 ⚠ phpstan: Mockery extension          not supported by Pun yet; skipped
   Want them? Send a PR to https://github.com/punphp/pun
 ⚠ phpstan-baseline.neon was not copied. Run `pun analyse --generate-baseline` for a Pun baseline.
```
- **pint.json and .php-cs-fixer.php:** the preset, then a delta of rules with TOML option names. `per` and `symfony`
  presets expand into explicit rules. Risky and code-deleting rules go to `[lint.rules]` at `error`. Custom fixer
  classes are named as unsupported. The Finder (paths, exclude, notName, notPath) moves to `[format]`.
- **rector.php:** paths and skip, prepared sets to `[lint] sets` one to one, single rules to `[lint.rules]`, PHP and
  Laravel level sets to the upgrade groups. Everything at `error`. Symfony, Doctrine and downgrade sets are named as
  unsupported.
- **phpstan.neon:** the level expands into the exact rules it ran, parameters into rule options, `ignoreErrors` into
  Pun ignores, and every extension that isn't built in is named. Larastan, Pest's plugin, phpstan-phpunit,
  strict-rules and deprecation-rules need no config, because they are built in. `stubFiles` carry over.
- **Baselines are never copied.** Migrate tells the user to run `pun analyse --generate-baseline`.
- **Ignore comments:** `@php-cs-fixer-ignore` and `@phpstan-ignore*` are rewritten to Pun's ignore comments with Pun IDs.
- **Interactive extra:** after writing exact rules, migrate offers the closest preset, with a count of the new problems
  the switch would report ("You're on PHPStan level 8; the closest Pun preset is `strict`. Switch?").
- **No workflow edits.** Migrate doesn't touch composer scripts, CI, hooks or editor settings. Future `pun-init` and
  `pun-migrate` agent skills will do that.

## 4. Build

1. **Format part on the new names** (in PLAN.md Step 1, because Step 1's gate runs through migrate; after config.md
   Build 1-4): upstream names to `format/<group>/<name>`, options
   to TOML names, `per`/`symfony` expansion, the 40 moved rules and the risky rules to `[lint.rules]`, the Finder.
   *Done when* `pun migrate` + `pun check --fix` equals Pint on all 7 corpus projects.
2. **Unsupported items:** one ⚠ line per item, the PR request once, the baseline note; exit 0 when the rest migrated.
   *Done when* a test with an unsupported rule, a custom fixer and a baseline pins the output.
3. **The new output** (plan/output.md): one ✔ line per config, ⚠ lines, summary.
4. **rector.php** (after lint.md Build 1-3): paths, skip, sets, rules, upgrade sets.
   *Done when* migrated Pinkary gives the same `pun lint --fix` result as `rector process`, after format.
5. **phpstan.neon** (after analyse.md Build 1): levels to rules, parameters, ignoreErrors, `@phpstan-ignore*` rewrites,
   extension names, baseline note. Needs each analyse rule's level in mapping.toml and pun-cli linking pun-phpstan.
   *Done when* migrated corpus projects report the same errors as PHPStan.
6. **Offer the closest preset** in interactive mode, with the new-problem count.

## Decisions

From 2026-10-17 (Pushpak): migrate is complete and explicit about what it can't move; unsupported items are named and
skipped, with a request to send a PR to punphp/pun, never sent to PHP; baselines are never copied; everything
migrates at `error`; the closest preset is offered interactively; no workflow wiring. Open: `--remove-tools`
(../PLAN.md §4).

Build 1 (wf/migrate-format, 2026-10-17), assistant's calls where the plan is silent:

- **Rule sets expand from PHP-CS-Fixer's own definitions.** crates/pun-config/presets/sets.json now holds every set of
  the pinned PHP-CS-Fixer (`RuleSets::getSetDefinitions()`, exported by scripts/export-rule-sets.php), not only the
  three PSR sets, so `per`, `symfony` and any set in a pint.json (`@PER-CS`, `@PHP84Migration`, `@Symfony:risky`)
  expand into explicit rules. The `@auto` sets read the project's composer.json, so they stay unexpanded and are
  named. Side effect: `pun format --preset per` (no pun.toml) names per's unported rules under their Pun IDs instead
  of `@PER-CS`.
- **"Can run" means a native fixer** (`pun_fixers::all`), the same test config 6 uses at run time. A rule without one
  (no Pun ID, deprecated, not ported, or an option TOML can't hold) is named and left out; a preset rule Pun can't run
  is written as `false`, so the migrated pun.toml always runs. The rest is written and migrate exits 0.
- **Interim output until migrate 2-3:** one stderr line per item, `pun migrate: <source>: <name>: <reason>; skipped`
  (`<source>` is the config file, or `preset <name>` without one); in agent mode one `<source> unsupported <name>:
  <reason>` finding under an `ok` summary. The PR request and the baseline note are migrate 2.
- **The `.php-cs-fixer.php` Finder** is read in the same `php -r` run, through reflection on Symfony's Finder:
  `in()` folders become `[files] paths` (relative to the project; the project root itself is left out), `exclude()`
  (without PhpCsFixer\Finder's own `vendor`), `notName()` and `notPath()` become `[format]` keys, as for pint.json.
  `name()` other than `*.php`, `path()`, `depth()`, `contains()`, `notContains()`, `size()`, `date()`, `filter()`,
  `append()`, `followLinks()`, `ignoreVCSIgnored(true)` and a Finder that is not a Symfony Finder are named as `Finder <call>: no pun.toml key for it`.
  An empty Finder list is left out of pun.toml.
