# Plan: `pun migrate`

Status: **decided, not built for the new design** (2026-10-17). Decisions from the format, lint, analyse, init and
command walkthroughs, gathered here. Decisions with dates: DECISIONS.md.

## 1. What it is

`pun migrate` moves a project's pint.json, .php-cs-fixer(.dist).php, rector.php and phpstan.neon onto one pun.toml.
It moves everything Pun supports, and names everything it doesn't. The proof is the harness: `pun migrate` followed by
`pun check --fix` gives the same files as the old tools.

## 2. Today (pun-cli migrate.rs, pun-config migrate.rs, main at 0800a99)

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

1. **Format part on the new names** (after config.md Build 1-4): upstream names to `format/<group>/<name>`, options
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
(PLAN.md §5).
