# Component: analyse (`pun analyse`)

Status: **under review with Pushpak** (opened 2026-10-17). This file is the plan for the analyser: what it is, how it
works today, what is left, and the questions we still have to settle. Answers go into "Decisions" below and into
DECISIONS.md. PLAN.md C7 points here. Same shape as format.md and lint.md.

## 1. What it is

`pun analyse` (alias `pun analyze`) answers one question: is this code wrong? It reports type errors, undefined symbols,
impossible conditions, dead code that needs types to spot, and misuse of Laravel. It never edits a file (commands.md).

It replaces:

| Upstream | What comes over |
| --- | --- |
| PHPStan core | every rule, natively (decided). The levels become presets (decided) |
| Larastan | built in, nothing to install (decided): models, relations, facades, container, config |
| phpstan-strict-rules, phpstan-deprecation-rules | the type-aware rules; strict-rules' syntax-only bans are lint rules |

The goal: on a project migrated from phpstan.neon, `pun analyse` reports the same problems as `phpstan analyse` (same
file, line and message), so CI can switch with no baseline churn.

## 2. How it works today

- **pun-phpstan** (100% coverage) ports PHPStan's own plumbing: the NEON parser, includes, parameters and schema
  validation, baseline read and write, `ignoreErrors`, `@phpstan-ignore` comments, PHPStan's type model and its
  `describe()` (so messages print types the way PHPStan does), and every built-in function and class exported from
  PHPStan's reflection into `data/symbols.json`.
- **pun-analyse** runs the rules:
  1. It parses with Mago's parser and resolves names.
  2. It collects every symbol of the project and vendor into Mago's codex.
  3. It checks each file in parallel with ported PHPStan rules.
  4. It drops what ignores and the baseline cover.
- **Level 0 is in progress.** It reports 54 PHPStan identifiers so far (undefined functions, classes, methods,
  properties and constants, `new`, typehints, `use`). It is checked against 142 of PHPStan's own rule test classes,
  with 40 known differing cases in `tests/fixtures/rules/gaps.txt`. On the corpus, the differences from PHPStan level 0
  are pinkary 1, larastan 6, pest 5 and laravel-framework 15, and Pun runs 5 to 25 times faster.
- **Not wired up yet.** There is no `pun analyse` command. The only entry point is
  `cargo run --example analyse <dir> <phpstan.neon>`, which reads phpstan.neon, used by `scripts/analyse-corpus.py`.
  The analyse agent stopped at 43d9857, with Larastan facade aliases and a trait fix uncommitted.
- **No type inference yet.** Level 0 mostly needs symbols, not types. Levels 2 and up need PHPStan's inference:
  the type of every expression, narrowing after `if`, `instanceof` and `is_*`, and generics from PHPDoc. Mago's
  analyser is a dependency but isn't used.

## 3. The config

This comes from earlier decisions; none of it is built yet.

```toml
preset = "recommended"          # essential | recommended | strict | pedantic (decided: named presets)

[analyse]
paths = ["app", "tests"]
baseline = "pun-baseline.toml"

[analyse.rules]
"types/missing-iterable-value-type" = "off"      # oxlint-style levels, as in lint
"undefined/method" = { level = "error", magic = true }   # PHPStan's checkMagicMethods as a rule option
```

- **No levels.** `pun migrate` expands a PHPStan level into the exact rules it ran, each with its options. In
  interactive mode it offers the closest preset, with a count of the new problems the preset would report (decided).
- **Laravel turns on by itself** when composer.lock has laravel/framework (decided).
- PHPStan's level 7 to 10 behaviour is four switches: `union-types`, `nullables`, `explicit-mixed`, `implicit-mixed`
  (RULES.md §4). The presets turn them on.

## 4. Rules: what exists

There are 255 analyse rules in rules/mapping.toml, built from 559 PHPStan identifiers plus strict-rules, deprecation-rules
and Larastan. Many PHPStan identifiers merge into one Pun rule (20 `generics.*` identifiers become `invalid-generics`),
and each identifier stays in the mapping as a variant, so migrated ignores and baselines keep their narrow scope.

| | Count |
| --- | ---: |
| Analyse rules | 255 |
| By group: correctness / types / dead-code / laravel / undefined / deprecated / strict / phpdoc / meta | 89 / 74 / 27 / 20 / 16 / 12 / 8 / 7 / 2 |
| By preset: essential / recommended / strict / pedantic / opt-in | 96 / 135 / 19 / 4 / 1 |
| PHPStan identifiers by level: 0 / 1 / 2 / 3 / 4 / 5 / 6 / opt-in | 318 / 7 / 97 / 36 / 57 / 8 / 6 / 30 |
| Running today | 54 PHPStan identifiers (part of level 0) |

These groups still follow the old intent-style list. With topic groups (lint Q3), `undefined/method` and
`types/argument` read naturally. The rename comes with the lint and format renames.

## 5. PHPStan's CLI, flag for flag

| PHPStan | Pun |
| --- | --- |
| `phpstan analyse [paths]` | `pun analyse [paths]` |
| `-c phpstan.neon` | `--config pun.toml`; phpstan.neon is read only by `pun migrate` |
| `-l 6` | `--preset strict` (no levels) |
| `--generate-baseline` | `pun analyse --generate-baseline` (writes pun-baseline.toml) |
| `--error-format=json/table/github/gitlab/checkstyle/junit/raw/teamcity` | `--format`, the same formats |
| `--memory-limit`, `--xdebug`, `--debug` | not needed |
| `clear-result-cache` | `--no-cache` / `pun cache clear` |
| `--pro`, `--watch` | `--watch` like `pun format` |

## 6. Updated plan

Done: config, ignores and baselines; the built-in symbols; the type model and descriptions; level-0 symbol rules
(54 identifiers); parallel scanning.

Left, in order (pending the answers below):

1. **Wire it up.** Add the `pun analyse` command, `[analyse]` in pun.toml, the reporters (format.md §10, agent output
   and PHPStan's machine formats), and its place in `pun check` (commands.md §4).
2. **Finish level 0.** Port the remaining level-0 rules, empty gaps.txt, and bring the corpus differences to 0.
3. **Type inference** (Q1). This is the big piece: expression types, narrowing and PHPDoc generics. Levels 2 to 10
   depend on it.
4. **Larastan, built in.** Models (attributes from migrations, casts, accessors), relations, facades, the container,
   config, and macros.
5. **Levels 1 to 10** as rules plus the four switches, then strict-rules and deprecation-rules.
6. **`pun migrate` for phpstan.neon**: level to rules, parameters to options, ignoreErrors and baseline to Pun IDs,
   and every unsupported extension named.
7. **Speed and memory**: a result cache, so a warm run re-checks only changed files and the files that depend on them.

## 7. Questions

1. **Type inference.** Port PHPStan's own inference (its scope, type narrowing and PHPDoc resolution) function by
   function, or build on Mago's analyser and translate its types?
   *Recommend: port PHPStan's. Messages must print the same types PHPStan prints, and narrowing differences change
   which errors appear. pun-phpstan already has PHPStan's type model, and Mago's 329 checks only match 190 PHPStan
   identifiers.*
2. **Messages.** Keep PHPStan's message text word for word (with Pun's rule ID instead of the identifier)?
   *Recommend: yes for 1.0. It keeps parity testable and familiar; nicer wording can come later, as a rule option.*
3. **Presets.** Four presets: `essential`, `recommended`, `strict`, `pedantic`? You mentioned five in the user journey.
   *Recommend: four. `essential` already is the loose one (about PHPStan level 0 to 1).*
4. **Which extensions are built in.** Larastan is (decided). What about strict-rules, deprecation-rules,
   phpstan-phpunit, the Pest plugin and phpstan-mockery?
   *Recommend: strict-rules, deprecation-rules, phpstan-phpunit and the Pest plugin built in, turned on from
   composer.lock like Laravel. Anything else is named as not supported.*
5. **A project's own PHPStan code** (custom rules and type extensions written in PHP). Pun can't run them.
   *Recommend: `pun migrate` names each one as not supported. Stub files (`stubFiles`) are plain PHP, so they keep
   working.*
6. **Baseline format.** Should migrate convert phpstan-baseline.neon into pun-baseline.toml with Pun IDs, and should
   `--generate-baseline` write the TOML file?
   *Recommend: yes. One config language, and the baseline still keeps each identifier's narrow scope.*
7. **Larastan's model knowledge.** Larastan learns model properties by reading migrations. Do the same, without
   running PHP?
   *Recommend: yes, read migrations like Larastan. Running `artisan` would need PHP and a database.*
8. **Result cache.** Should a warm `pun analyse` re-check only changed files and their dependents before 1.0?
   *Recommend: yes. Speed and memory are a priority, and this is where analysers spend most of their time.*

## 8. How PHPStan and Mago analyse, and what Pun builds on (2026-10-17)

**PHPStan.**
- `NodeScopeResolver` walks each file's syntax tree (nikic/php-parser) and carries a `Scope`, which holds the type of
  every variable at that point.
- `TypeSpecifier` narrows types after `if`, `instanceof`, `is_*` and assertions.
- Rules are small classes that get a node plus the scope and return errors with an identifier. Levels just choose
  which rules run and how strictly types are compared.
- Extensions (dynamic return types, property and method reflection) are how Larastan teaches it Laravel's magic.
- Baselines are a NEON list of (message, identifier, path, count).

**Mago's analyser** (mago-analyzer 1.57, about 106k lines of Rust, modelled on Psalm/Hakana):
- It walks the same kind of tree with a block context holding variable types.
- A reconciler with formula algebra does narrowing, as Psalm does. Generics and templates come from PHPDoc.
- It reports issues by code.
- A plugin system has providers (function, method and property return types, assertions, throws) and hooks (calls,
  expressions, statements, declarations). It already ships library plugins for the stdlib, PSL and PSR container.
- mago-reporting has a baseline: strict (exact lines) or loose (counts per file, code and message, the default), with
  pruning of outdated entries.

**Pun's choice (assistant's call, delegated by Pushpak).** Build analyse on Mago's analyser, not a port of PHPStan's
inference.
- Pushpak relaxed the two things that needed a PHPStan port: messages can be reworded, and migrate doesn't carry
  baselines over. What must hold is one Pun rule for every PHPStan rule, and that already holds in rules/mapping.toml
  (every one of the 559 identifiers maps to a Pun rule, or is listed as dropped or unsupported with a reason).
- Mago's inference, narrowing and generics exist today. A port of PHPStan's would be the largest single piece of
  work in Pun.
- **Rules:** where a Mago issue code is the same check as a PHPStan rule (190 codes), Pun reports it under the Pun
  rule. The other PHPStan rules are written as Pun checks that read Mago's inferred types, the same way the 54
  level-0 rules already read Mago's codex.
- **Laravel** (Larastan's knowledge) is a built-in Mago plugin in Pun. Providers give return and property types for
  models, relations, facades, the container and config; hooks add the Laravel-specific checks.
- **Parity test:** PHPStan's own rule fixtures and the corpus, compared on file, line and Pun rule, not message text.
  Differences go in gaps.txt, as today.
- **Baseline:** `pun analyse --generate-baseline` writes pun-baseline.toml on mago-reporting's baseline, loose by
  default. `pun migrate` does not copy phpstan-baseline.neon; it tells the user to generate a new one.
  `ignoreErrors` in phpstan.neon is config, not a baseline, so migrate still converts it to Pun rule IDs.
- **Kept from pun-phpstan:** the NEON reader, includes and `ignoreErrors` handling (for migrate), and
  `@phpstan-ignore` comment parsing (migrate rewrites those comments). Its PHPStan type model and `describe()` are no
  longer needed for messages.
- **Risk:** Mago infers differently from PHPStan in edge cases, so some PHPStan errors will be missing or extra. The
  parity test measures this, and each gap gets fixed in a Pun check or a Mago plugin.

## Decisions

From earlier rounds (2026-10-17, Pushpak):
- Every PHPStan core rule runs natively; Larastan is built in.
- No levels. Named presets for new projects; migrate writes exact rules and offers the closest preset.
- Laravel, Pest and PHPUnit turn on automatically from composer.lock.
- Analyse never edits files; `pun check` runs it after lint and format.

Answered 2026-10-17 (Pushpak):
- **Q1 engine: assistant's call.** The requirement is one-to-one rule compatibility, so that every PHPStan rule has a Pun
  rule in the mapping. The choice: Mago's analyser (§8).
- **Q2 messages:** Pun may reword them.
- **Q6 baseline:** its own feature (`pun analyse --generate-baseline`). Migrate never copies a PHPStan baseline.
- **Presets:** named presets instead of levels; how many (four or more) is still open.
