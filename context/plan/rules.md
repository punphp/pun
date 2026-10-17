# Plan: rule placement, overlaps, naming and the mapping data

Pun replaces five upstream tools (PHP-CS-Fixer/Pint, Rector, PHPStan with its extensions, Larastan, Mago's linter), and
their rules overlap a lot. This file says where each rule lives, how overlaps resolve, how rules are named, how they
are ignored, and what the mapping data holds. Rule IDs, groups, presets, sets and levels: config.md §4-§5. Decisions:
../PLAN.md §2. Formerly context/RULES.md (its full history, including the superseded tier tables, is in git).

The data is [`crates/pun-config/rules/mapping.toml`](../../crates/pun-config/rules/mapping.toml): 1,005 Pun rules and
2,564 upstream entries, each pointing at a Pun rule or carrying a drop or unsupported reason. It drives `pun migrate`
(configs, ignore comments), `pun explain <rule>`, and the generated "Coming from Pint / PHPStan / Rector" docs page.
`scripts/check-rule-mapping.py` checks it (§5).

**Names in this file are today's** (`lint.no-loose-comparison`). The `<command>/<group>/<name>` names come from
Step 1's rename (config.md Build 1); this file is updated with them.

## 1. Placement

| Command | Owns | Never | Fix |
| --- | --- | --- | --- |
| `pun format` | Layout, casing, spelling of the same construct (`array()` to `[]`, quotes, `else if` to `elseif`, `\Foo` to an import), sorting lists whose order means nothing (imports, union types, `implements`, PHPDoc tags) | Changes behaviour; removes code; adds or removes PHPDoc; rewrites one construct into another; changes declarations or member order | Always applied; safe on save |
| `pun lint` | Every other edit, plus every report one file can decide (types may make a fix safer) | Pure layout | Labelled safe, unsafe or none; `--fix` applies all |
| `pun analyse` | Reports that need type inference or other files (types, undefined symbols, inherited PHPDoc, deprecations) | Edits files | None; a report a lint rule can fix names it in `fixed_by` |

The tests, in order; the first match wins:

1. **Risky in PHP-CS-Fixer?** Then lint, labelled unsafe. Pun uses `isRisky()` as is and doesn't reclassify.
2. **Only layout, casing or spelling** (the format column above)? Then format. A Rector or Mago rule that does the same
   as such a fixer maps onto the format rule.
3. **Any other edit?** Lint, labelled safe (keeps behaviour) or unsafe (can change it). This is where the 40 safe
   CS-Fixer fixers that delete or restructure code go (commands.md §5).
4. **A report one file can decide without types?** Lint with no fix (strict-rules' `equal.notAllowed`, Mago's `no-eval`).
5. **Otherwise** it needs types or other files: analyse.

**Exception (parity):** every PHPStan core and Larastan identifier stays in analyse until 1.0, even where test 4 would
pick lint, so `pun analyse` matches `phpstan analyse` on migration day. Two carve-outs are layout and map to format:
`whitespace.bom` (encoding) and `whitespace.fileEnd` (no closing tag). strict-rules and deprecation-rules are placed by
the tests.

**What this means for Pint projects.** Pint's laravel preset runs 4 risky fixers (`no_alias_functions`,
`no_unreachable_default_argument_value`, `ordered_traits`, `php_unit_set_up_tear_down_visibility`), Pinkary's pint.json
10 more, and the laravel preset 11 of the 40 code-deleting fixers. All of them are lint rules. `pun migrate` writes them
under `[lint.rules]` at `error`, and `pun check --fix` runs them in the same priority-ordered token pass as format, so it
writes Pint's bytes. `pun format` alone never renames classes, reorders traits or adds `declare(strict_types=1)`.

## 2. Overlaps

The mapping has 105 overlap clusters and 18 linked analyse reports. Every cluster ends as one Pun rule in one command.

| # | Overlap | Home | The others at migrate time |
| --- | --- | --- | --- |
| O1 | Format-safe CS-Fixer fixer and Rector or Mago (`no_unused_imports`, Mago's `unused-import`) | The format rule, or the lint rule if the fixer is one of the 40 | Enabling the Rector or Mago rule enables it. CS-Fixer's options win; others are dropped and migrate prints them |
| O2 | Risky CS-Fixer fixer and anything (`modernize_strpos`, Rector's `StrContains`, Mago's `str-contains`) | One lint rule, unsafe | All map to it |
| O3 | Rector, Mago and Pun with no CS-Fixer rule (`prefer-first-class-callable`) | One lint rule | All map to it; where strategies differ, the rule covers both |
| O4 | A syntax-only strict-rules ban and a fixer (`empty.notAllowed`, Mago's `no-empty`) | Lint | `ignoreErrors` entries and `@phpstan-ignore` comments for the identifier move to the lint rule |
| O5 | A type-aware report and a fixer (PHPStan `method.unused`, Rector's `RemoveUnusedPrivateMethod`) | Two rules, linked: analyse reports, lint fixes, the analyse rule lists it in `fixed_by` | `pun analyse` alone still reports it; `pun check` prints only the lint problem |
| O6 | A fixer whose safety depends on types (`strict_comparison`, Rector's `UseIdenticalOverEqualWithSameType`) | One lint rule | Labelled safe where analysis proves it, unsafe otherwise; `--fix` applies both |
| O7 | Rules that contradict each other (§2 conflicts below) | Both stay | `[meta] conflicts`; config loading refuses a config that enables both |
| O8 | Upstream deprecated or no-op rules | The successor | Mapped to it, or dropped with a note in migrate's output |
| O9 | Out of scope: rector-symfony, rector-doctrine, rector-downgrade-php, Mago's WordPress, Psl, Symfony and Drupal rules, PHPStan's `phpstanApi.*` | None | `unsupported`: migrate names each one and skips it |

**Worked example: strict comparison.**

| Upstream | Kind | Becomes |
| --- | --- | --- |
| PHP-CS-Fixer `strict_comparison` (risky) | fixer, every `==` | `lint.no-loose-comparison`, unsafe part |
| Rector `UseIdenticalOverEqualWithSameTypeRector` | fixer, only when both sides have the same type | `lint.no-loose-comparison`, safe part |
| Mago `identity-comparison` | report | `lint.no-loose-comparison` |
| strict-rules `equal.notAllowed`, `notEqual.notAllowed` | report, syntax only | `lint.no-loose-comparison` |

The check needs no types, and a fix exists, so it is lint. `pun lint --fix` rewrites every `==`; the output labels the
ones whose operand types aren't proven identical as unsafe.

Other resolutions: `strict_param` + Rector `StrictInArray`/`StrictArraySearch` + strict-rules `function.strict` + Mago
`strict-behavior` → `lint.require-strict-param`. `declare_strict_types` + Rector `DeclareStrictTypes` + Mago
`strict-types` → `lint.require-strict-types`. `backtick_to_shell_exec` + Rector `ShellExecFunctionCallOverBackticks` +
strict-rules `backtick.notAllowed` + Mago `no-shell-execute-string` → `format.backtick-to-shell-exec`. Three of Rector's
four privatization rules + Mago `no-protected-in-final` → `protected-to-private` (one of the 40, so lint).

**Conflicts** (`[meta] conflicts` in mapping.toml; config loading refuses a config that enables both sides):
`no-short-ternary` / `prefer-short-ternary`, `prefer-chained-expect` / `no-and-expectation`,
`prefer-named-data-provider-keys` / `no-named-data-provider-keys`, `simplified-null-return` /
`require-consistent-return`, `single-import-per-statement` / `group-import`. Which side a lint set turns on is decided
with the set tags (config.md Build 1).

## 3. Naming style and vocabulary

The ID shape (`<command>/<group>/<name>`), the group list and "no aliases" are in config.md §4. Within that shape:

- **Format names** describe the construct (`arrays/syntax`, `imports/ordered`); `no-` only when the rule removes
  something (`whitespace/no-trailing`). The full table is format.md §9.
- **Lint names start with a verb:** `no-` removes or bans; `prefer-X` replaces code with the better form X; `add-`
  inserts something that can be inferred (types, attributes, PHPDoc); `require-` reports, and may insert, what a policy
  says must be there. PHP-version rules are mostly `prefer-X` (`prefer-array-find`, `prefer-clamp`); the ones that fix
  code a newer PHP removed are `no-X` (`no-each`, `no-create-function`).
- **Configurable codemods** (Rector's `RenameMethod`, `FuncCallToStaticCall`, which do nothing until configured) are
  named after their edit: `rename-`, `replace-`, `remove-`, `change-`, `wrap-`. Only codemods use these prefixes.
- **Analyse names** name the problem: `<word>-<thing>` (`undefined-method`, `unused-result`) or `<thing>-<problem>`
  (`argument-type`, `condition-always-true`). With groups this reads `undefined/method`, `types/argument`.
- **The name doesn't repeat the group**, is kebab-case and is unique within its group.
- **Short and familiar.** Two to four words, never the upstream mechanism (`add-return-type`, not
  `return-type-from-strict-native-call`). Where Biome, ESLint, oxlint, Ruff or typescript-eslint has a name for the same
  idea, use it (`no-lonely-if`, `no-useless-catch`, `prefer-spread`); else Mago's; else PHPStan's wording. The reader is
  a Laravel developer who has never used Rector or PHPStan.

| Upstream | Pun (today's name) |
| --- | --- |
| CS-Fixer `strict_comparison` / `modernize_strpos` / `psr_autoloading` | `no-loose-comparison` / `prefer-str-contains` / `require-class-matches-file` |
| Rector `ChangeNestedIfsToEarlyReturnRector` / `ClassPropertyAssignToConstructorPromotionRector` | `prefer-early-return` / `prefer-constructor-promotion` |
| PHPStan `method.notFound` / `missingType.iterableValue` / `deadCode.unreachable` | `undefined-method` / `missing-iterable-value-type` / `unreachable-code` |
| Larastan `larastan.noEnvCallsOutsideOfConfig` / `larastan.relationExistence` | `no-env-outside-config` / `undefined-relation` |
| strict-rules `plus.leftNonNumeric` ... `unaryMinus.nonNumeric` (15 identifiers) | `non-numeric-operand` (one rule) |

**Vocabulary.** Each word means one thing in every command:

| Word | Means | Examples |
| --- | --- | --- |
| `unused` | declared, assigned or computed, never read | `no-unused-assignment`, `unused-private-method`, `no-unused-ignore` |
| `undefined` | referenced, declared nowhere | `undefined-method`, `undefined-variable` |
| `missing` | something required is absent (lint inserts it with `add-` or `require-`) | `missing-return-type`, `missing-argument` |
| `redundant` | no effect because something else already guarantees it | `no-redundant-nullsafe`, `redundant-null-coalesce` |
| `useless` | no effect on its own; removing it never changes behaviour | `no-useless-cast`, `no-useless-override` |
| `dead` | a branch or handler that can never run | `no-dead-catch`, `dead-catch` |
| `unreachable` | statements after `return`, `throw` or `exit` | `no-unreachable-code`, `unreachable-code` |
| `invalid` | malformed, or contradicts its declaration | `invalid-attribute`, `invalid-phpdoc` |
| `impossible` / `always-true` / `always-false` | a check whose result the types decide | `impossible-type-check`, `condition-always-false` |
| `deprecated` | uses something marked deprecated | `deprecated-method` |

`unnecessary` is not used (Larastan's `noUnnecessaryCollectionCall` is `redundant-collection-call`).

**Merges.** 109 lint and 82 analyse rules carry several upstream sources (their `merged` field). A merge is right when
the sources make the same edit or report the same problem and nobody would want one without the other: Rector's 33
return-type strategies → `add-return-type`; 31 rector-laravel `*PropertyTo*Attribute` rules → `prefer-php-attributes`;
CS-Fixer `void_return` + Rector's four void rules → `add-void-return-type`; PHPStan's 20 `generics.*` identifiers →
`invalid-generics`. Rules stay separate when merging would widen a CS-Fixer fixer beyond what Pint does, so
`pun check --fix` stays equal to Pint. Each merged PHPStan identifier stays in the mapping as a `variant` (§5).

## 4. Ignore comments

| Form | Applies to |
| --- | --- |
| `$x = foo(); // @pun-ignore <id>` | the line it trails |
| `// @pun-ignore <id>` on its own line | the next line (PHPStan's `@phpstan-ignore` rule) |
| `// @pun-ignore-next-line <id>` | the next line, explicitly |
| `// @pun-ignore-file <id>` before the first statement | the whole file. The only form for format rules, which can't be half-applied to a line; keeps `@php-cs-fixer-ignore`'s meaning. **Built today** |
| `// @pun-ignore-start <id>` ... `// @pun-ignore-end` | a region |

- One or more IDs, comma-separated; a group is allowed. Whether a short `group/name` is accepted is open
  (../PLAN.md §4 Q2). A bare `@pun-ignore` with no ID is an error.
- Free text after ` -- ` is the reason: `// @pun-ignore lint/security/no-eval -- legacy plugin loader`.
- `/* */` and `/** */` forms work too, so PHPDoc-placed `@phpstan-ignore` ports one to one.
- Unused ignores are reported by `no-unused-ignore` (safe fix: delete); malformed ones by `invalid-ignore-comment`.
- Migrate rewrites `@phpstan-ignore <id>` to `@pun-ignore <pun id>` through the mapping, deduplicating ids that map to
  one rule; `@phpstan-ignore-next-line` (no id) becomes `@pun-ignore-next-line` with the ids PHPStan reported there;
  `@php-cs-fixer-ignore <rule>` becomes `@pun-ignore-file <format id>`.

## 5. The mapping data

```toml
[rules.lint]
no-loose-comparison = { group = "suspicious", fix = "mixed", severity = "warn", tier = "strict", ported = true, status = "decided", summary = "…" }

[rules.analyse]
unused-private-method = { group = "dead-code", severity = "error", tier = "recommended", fixed_by = ["lint.no-unused-private-method"], status = "decided" }

[upstream.php-cs-fixer]
strict_comparison = { pun = "lint.no-loose-comparison", risky = true, options = "none", status = "decided" }

[upstream.rector]   # key: class name without the "Rector" suffix; fully qualified on a clash
RemoveUnusedPrivateMethod = { class = "Rector\\DeadCode\\Rector\\ClassMethod\\RemoveUnusedPrivateMethodRector", pun = "lint.no-unused-private-method", sets = ["dead-code"], options = "none", status = "decided" }

[upstream.rector-symfony]
AccessTypeAnnotationToAttribute = { unsupported = "Symfony rules: out of scope (Laravel-first)", status = "decided" }
```

- **Sections:** `[upstream.php-cs-fixer]`, `pint`, `pun` (today's `Pun/*`), `rector`, `rector-laravel`,
  `pest-plugin-rector`, `rector-phpunit`, `rector-symfony`, `rector-doctrine`, `rector-downgrade-php`, `phpstan` (by
  identifier; `level` = lowest level that emits it; `pattern = true` for `%s` identifiers), `phpstan-parameters`,
  `phpstan-strict-rules`, `phpstan-deprecation-rules`, `larastan`, `mago-linter`, `mago-analyzer`.
- **Upstream keys** are the exact upstream IDs; Rector and Pest entries also carry the fully qualified `class`.
  `options` says how upstream options carry over.
- **`variant`** on PHPStan-family entries keeps the original identifier when one Pun rule replaces several, so `-v`
  output can print it and migrated `ignoreErrors` keep their narrow scope.
- **Pinned upstream versions** in `[meta] upstream`: PHP-CS-Fixer 3.95.17, Rector 2.6.1, rector-laravel 2.5.0,
  pest-plugin-rector 5.x-dev, PHPStan 2.2.7, strict-rules 2.0.12, deprecation-rules 2.0.5, Larastan 3.10.0, Mago 1.57.0.

**What Step 1 changes in the data** (config.md Build 1-3): rule keys become `<group>/<name>` under `[rules.<command>]`
and every `pun`, `fixed_by` and `conflicts` reference becomes a full `<command>/<group>/<name>`; `[meta] groups` holds
the fixed group list with descriptions; `aliases` goes; `severity` goes (lint rules default to `error`, analyse rules
have none); `tier` becomes `sets = [...]` on lint rules and `preset = "basic" | ... | "max"` on analyse rules (from the
PHPStan `level`); `presets` on format rules keeps only `laravel` and `psr12`; `fix` keeps safe / unsafe / mixed / none
as a label; the 40 rules move from `[rules.format]` to `[rules.lint]`.

**The checker** (`scripts/check-rule-mapping.py`) fails on: a name that isn't kebab-case or isn't unique in its group, a
group not in the list, a lint name without its prefix, a dangling `pun` / `fixed_by` / `conflicts` / `merged` reference,
an upstream entry that isn't exactly one of pun / drop / unsupported, a Rector entry without `class`, a rule nothing maps
to (unless `pun_only`), and a set or preset that enables both sides of a conflict. Still to add: a test that fails when
an oracle rule or identifier is missing from `[upstream.*]`, so an upstream bump forces a decision for every new rule.

## 6. Counts

| Upstream (version) | Entries | → format | → lint | → analyse | dropped | unsupported |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| PHP-CS-Fixer 3.95.17 (76 risky) | 300 | 206 | 76 | - | 18 (deprecated → successor) | - |
| Pint's own fixers | 2 | 2 | - | - | - | - |
| Pun `Pun/*` | 17 | - | 17 | - | - | - |
| Rector core 2.6.1 | 512 | 48 | 437 | - | 24 | 3 |
| rector-laravel 2.5.0 | 122 | - | 122 | - | - | - |
| pest-plugin-rector | 61 | - | 61 | - | - | - |
| rector-phpunit | 113 | 9 | 100 | - | 3 | 1 |
| rector-symfony / doctrine / downgrade-php | 267 | - | - | - | - | 267 |
| PHPStan 2.2.7 core identifiers | 559 | 2 | - | 534 | 8 | 15 |
| PHPStan level parameters | 18 | - | - | 13 options + 5 switches | - | - |
| phpstan-strict-rules 2.0.12 | 44 | 1 | 16 | 27 | - | - |
| phpstan-deprecation-rules 2.0.5 | 10 | - | - | 10 | - | - |
| Larastan 3.10.0 | 20 | - | - | 20 | - | - |
| Mago 1.57.0 linter | 190 | 39 | 130 | - | 2 | 19 |
| Mago 1.57.0 analyzer codes | 329 | - | - | 190 | - | 139 |
| **Pun rules** | | **208** | **542** | **255** | | |

These are before the 40 move: after it, format has 168 rules and lint 582. Ported today: 144 of 208 format rules and 31
of 542 lint rules (`ported = true`).

## Borrowed from

- **Biome:** formatter and linter split, `check` as the one command, region ignores, `lint/<group>/<rule>` IDs.
- **Ruff:** one rule per idea whatever its origin, upstream codes redirected to it; `modernize` gated by target version.
- **oxlint:** `off` / `warn` / `error` per rule or category, `--deny-warnings`, `--max-warnings`, overrides per path.
- **Vite+ (`vp check`):** one config file and one `check` that runs every step in one process.
- **Rector:** prepared sets you tick, which became lint sets.
- **ESLint, typescript-eslint, Biome:** the `recommended` and `strict` names.
- **Go (`gofmt` / `vet`):** the formatter never changes behaviour and is never configured per line.
