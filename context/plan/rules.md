# Plan: rule placement, overlaps, naming and the mapping data

Pun replaces five upstream tools (PHP-CS-Fixer/Pint, Rector, PHPStan with its extensions, Larastan, Mago's linter), and
their rules overlap a lot. This file says where each rule lives, how overlaps resolve, how rules are named, how they
are ignored, and what the mapping data holds. Rule IDs, groups, presets, sets and levels: config.md §4-§5. Decisions:
../PLAN.md §2. Formerly context/RULES.md (its full history, including the superseded tier tables, is in git).

The data is [`crates/pun-config/rules/mapping.toml`](../../crates/pun-config/rules/mapping.toml): 1,005 Pun rules and
2,564 upstream entries, each pointing at a Pun rule or carrying a drop or unsupported reason. It drives `pun migrate`
(configs, ignore comments), `pun explain <rule>`, and the generated "Coming from Pint / PHPStan / Rector" docs page.
`scripts/check-rule-mapping.py` checks it (§5).

**Names in this file are from before the rename** (`lint.no-loose-comparison`, now `lint/types/no-loose-comparison`).
Step 1 (config.md Build 1-2) gave every rule its `<command>/<group>/<name>` ID in mapping.toml; the names it had to
invent are under Decisions at the end of this file.

## 1. Placement

| Command | Owns | Never | Fix |
| --- | --- | --- | --- |
| `pun format` | Layout, casing, spelling of the same construct (`array()` to `[]`, quotes, `else if` to `elseif`, `\Foo` to an import), sorting lists whose order means nothing (imports, union types, `implements`, PHPDoc tags) | Changes behaviour; removes code; adds or removes PHPDoc; rewrites one construct into another; changes declarations or member order | Always applied; safe on save |
| `pun lint` | Every other edit, plus every report one file can decide (types may make a fix safer) | Pure layout | Labelled safe, unsafe or none; `--fix` applies all |
| `pun analyse` | Reports that need type inference or other files (types, undefined symbols, inherited PHPDoc, deprecations) | Edits files | None; a report a lint or format rule can fix belongs to that rule, not to analyse |

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

The mapping has 105 overlap clusters. Every cluster ends as one Pun rule in one command.

| # | Overlap | Home | The others at migrate time |
| --- | --- | --- | --- |
| O1 | Format-safe CS-Fixer fixer and Rector or Mago (`no_unused_imports`, Mago's `unused-import`) | The format rule, or the lint rule if the fixer is one of the 40 | Enabling the Rector or Mago rule enables it. CS-Fixer's options win; others are dropped and migrate prints them |
| O2 | Risky CS-Fixer fixer and anything (`modernize_strpos`, Rector's `StrContains`, Mago's `str-contains`) | One lint rule, unsafe | All map to it |
| O3 | Rector, Mago and Pun with no CS-Fixer rule (`prefer-first-class-callable`) | One lint rule | All map to it; where strategies differ, the rule covers both |
| O4 | A syntax-only strict-rules ban and a fixer (`empty.notAllowed`, Mago's `no-empty`) | Lint | `ignoreErrors` entries and `@phpstan-ignore` comments for the identifier move to the lint rule |
| O5 | A type-aware report and a fixer (PHPStan `method.unused`, Rector's `RemoveUnusedPrivateMethod`) | The lint (or format) rule; analyse keeps only the identifiers no rule fixes (Pushpak, 2026-11-02) | The PHPStan class and identifiers map to the fixer, so `ignoreErrors`, `@phpstan-ignore` and `pun migrate` move to it; `pun analyse` no longer reports them |
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

**One to one** (2026-10-18, §7). Each lint rule maps to exactly one in-scope upstream rule (a Rector rule, a risky
PHP-CS-Fixer fixer, or a strict-rules class), and each analyse rule to exactly one PHPStan `Rule` class. Mago's
rules are the exception: a Mago-derived rule (`origin = "mago"`) keeps its Mago mapping and has no other source, and a
Mago rule may also map onto a rule that has a primary source. Format rules may still carry several sources. The old
`merged` field is gone: its 109 lint merges were split (Rector's return-type strategies, the rector-laravel
`*PropertyTo*Attribute` rules, ...), and PHPStan's identifiers are `variant`s of the rule of the class that emits
them (§5).

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
  `pest-plugin-rector`, `rector-phpunit`, `rector-symfony`, `rector-doctrine`, `rector-downgrade-php`,
  `phpstan-classes` (one entry per `Rule` class of PHPStan and its extensions: `class`, `package`, `identifiers`,
  `level`, `gated_by`; keyed by the short class name, `Namespace\Short` on a clash), `phpstan` (by
  identifier; `level` = lowest level that emits it; `pattern = true` for `%s` identifiers), `phpstan-parameters`,
  `phpstan-strict-rules`, `phpstan-deprecation-rules`, `larastan`, `mago-linter`, `mago-analyzer`.
- **Upstream keys** are the exact upstream IDs; Rector and Pest entries also carry the fully qualified `class`.
  `options` says how upstream options carry over.
- **`variant`** on PHPStan-family entries keeps the original identifier when one Pun rule replaces several, so `-v`
  output can print it and migrated `ignoreErrors` keep their narrow scope.
- **Pinned upstream versions** in `[meta] upstream`: PHP-CS-Fixer 3.95.17, Rector 2.8.4 (with rector-phpunit
  dev-main@22c6dae bundled), rector-laravel 2.7.3, pest-plugin-rector 5.x-dev@8c1b780, PHPStan 2.3.1, strict-rules
  2.0.12, deprecation-rules 2.0.5, Larastan 3.10.0, Mago 1.57.0. reference/oracle.md says how to fetch each.

**What Step 1 changes in the data** (config.md Build 1-3): rule keys become `<group>/<name>` under `[rules.<command>]`
and every `pun`, `fixed_by` and `conflicts` reference becomes a full `<command>/<group>/<name>`; `[meta] groups` holds
the fixed group list with descriptions; `aliases` goes; `severity` goes (lint rules default to `error`, analyse rules
have none); `tier` becomes `sets = [...]` on lint rules and `preset = "basic" | ... | "max"` on analyse rules (from the
PHPStan `level`); `presets` on format rules keeps only `laravel` and `psr12`; `fix` keeps safe / unsafe / mixed / none
as a label; the 40 rules move from `[rules.format]` to `[rules.lint]`.

**The checker** (`scripts/check-rule-mapping.py`) fails on: a name that isn't kebab-case or isn't unique in its group, a
group not in the list, a lint name without its prefix, a dangling `pun` / `fixed_by` / `conflicts` / `merged` reference,
an upstream entry that isn't exactly one of pun / drop / unsupported, a Rector entry without `class`, a rule nothing maps
to (unless `pun_only`), and a set or preset that enables both sides of a conflict. Against the inventories in
`rules/upstream/*.json` it also fails on: an in-scope Rector rule with no `pun`, a deprecated or out-of-scope one
with a `pun`, a Rector class listed twice, a PHPStan class missing from `[upstream.phpstan-classes]` or with other
identifiers than its inventory row, a lint or analyse rule with no or several primary sources (Mago-derived rules
excepted), a Rector-backed rule in an upgrade group below PHP 8.4 or Laravel 12, a PHPStan identifier with no entry,
and an identifier whose rule is not the rule of a class that emits it. An upstream bump regenerates the inventories,
so every new rule needs a decision.

## 6. Counts

| Upstream (version) | Entries | → format | → lint | → analyse | dropped | unsupported |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| PHP-CS-Fixer 3.95.17 | 300 | 167 | 115 | - | 18 (deprecated → successor) | - |
| Pint's own fixers | 2 | 1 | 1 | - | - | - |
| Pun `Pun/*` (Mago ports) | 17 | - | 17 | - | - | - |
| Rector core 2.8.4 | 523 | 13 | 266 | - | 2 | 242 |
| rector-laravel 2.7.3 | 141 | - | 94 | - | - | 47 |
| pest-plugin-rector 5.x-dev@8c1b780 | 61 | - | 59 | - | - | 2 |
| rector-phpunit (bundled) | 116 | - | 81 | - | - | 35 |
| rector-symfony / doctrine / downgrade-php | 267 | - | - | - | - | 267 |
| PHPStan-family `Rule` classes | 432 | 2 | 17 | 385 | - | 28 |
| PHPStan 2.3.1 core identifiers | 632 | 2 | - | 608 | 7 | 15 |
| PHPStan level parameters | 18 | - | - | 13 options + 5 switches | - | - |
| phpstan-strict-rules 2.0.12 identifiers | 44 | 1 | 16 | 27 | - | - |
| phpstan-deprecation-rules 2.0.5 identifiers | 10 | - | - | 10 | - | - |
| Larastan 3.10.0 identifiers | 20 | - | - | 20 | - | - |
| Mago 1.57.0 linter | 190 | 28 | 141 | - | 2 | 19 |
| Mago 1.57.0 analyzer codes | 329 | - | - | 190 | - | 139 |
| **Pun rules** | | **168** | **703** | **385** | | |

Ported today: 125 of 168 format rules and 50 of 703 lint rules (`ported = true`). In scope: 493 Rector rules (482 →
lint, 11 → format) and 397 PHPStan classes (378 → analyse, 17 → lint, 2 → format).

## 7. The one-to-one mapping (2026-10-18)

**In scope.** A Rector rule is in scope when a planned set registers it (the inventories' `in_scope`): Rector's prepared
sets (`dead-code`, `code-quality`, `coding-style`, `type-declaration` and its docblocks set, `early-return`,
`privatization`, `naming`, `instanceof`, `rector-preset`), the PHP sets from `php84` up, rector-laravel's quality sets
and its version sets from `laravel-120` up (`livewire-40` too), rector-phpunit's quality sets and `phpunit110` up, and
Pest's `coding-style`. It is out when Rector deprecated it (`DeprecatedInterface` or `@deprecated` in rector-src 2.7.0
or main), when only upgrade sets below PHP 8.4, Laravel 12 or PHPUnit 11 register it, when only other sets
(`named-args`, `if`, `datetime-to-carbon`, `php-polyfills`, Lumen, old package majors) register it, or when no set
does. A PHPStan class is in scope when a config registers it (O9 keeps `phpstanApi.*` out). The inventories are
crates/pun-config/rules/upstream/*.json; scripts/check-rule-mapping.py enforces all of this.

**Removed lint rules** (165; their Rust code, if any, stays until the perf rounds end):

| Reason | Rules | IDs (without `lint/`) |
| --- | ---: | --- |
| Only in PHP upgrade sets below 8.4 | 56 | `php-5.3/no-http-server-vars`, `php-5.4/no-call-time-reference`, `php-5.4/no-zero-break`, `php-5.5/no-preg-e-modifier`, `php-5.5/prefer-self-class`, `php-7.0/no-break-outside-loop`, `php-7.0/no-call-user-method`, `php-7.0/no-duplicate-default`, `php-7.0/no-empty-list`, `php-7.0/no-list-on-string`, `php-7.0/no-mktime-without-args`, `php-7.0/no-this-on-static-method`, `php-7.0/prefer-spaceship`, `php-7.0/prefer-throwable-handler`, `php-7.1/no-arithmetic-on-string`, `php-7.1/no-array-assign-to-string`, `php-7.1/prefer-is-iterable`, `php-7.1/prefer-multi-catch`, `php-7.2/no-create-function`, `php-7.2/no-each`, `php-7.2/no-get-class-null`, `php-7.2/no-string-assert`, `php-7.2/require-parse-str-result`, `php-7.2/require-string-define-name`, `php-7.3/no-case-insensitive-constant`, `php-7.3/no-heredoc-marker-in-body`, `php-7.3/prefer-array-key-first-last`, `php-7.3/prefer-is-countable`, `php-7.3/prefer-setcookie-options`, `php-7.3/require-regex-dash-escape`, `php-7.3/require-string-needle`, `php-7.4/add-null-property-default`, `php-7.4/no-hebrevc`, `php-7.4/no-mb-strrpos-encoding-offset`, `php-7.4/no-money-format`, `php-7.4/no-reflection-export`, `php-7.4/no-restore-include-path`, `php-7.4/prefer-addslashes`, `php-7.4/require-nested-ternary-parens`, `php-8.0/add-inherited-param`, `php-8.0/no-unused-catch-variable`, `php-8.0/prefer-get-debug-type`, `php-8.0/require-static-set-state`, `php-8.1/no-set-accessible`, `php-8.1/prefer-native-enum`, `php-8.1/prefer-readonly-property`, `php-8.2/add-skip-dots-flag`, `php-8.2/no-utf8-encode-decode`, `php-8.2/prefer-readonly-class`, `php-8.3/add-const-type`, `php-8.3/prefer-dynamic-class-constant`, `php-8.3/prefer-json-validate`, `php-8.3/prefer-ldap-uri`, `refactor/add-argument`, `refactor/prefer-constructor-promotion`, `refactor/replace-static-with-function` |
| In no Rector set | 39 | `complexity/no-dirname-dir-concat`, `laravel/no-cache-minutes`, `laravel/no-factory-model-property`, `laravel/prefer-arr-get`, `laravel/prefer-blank-filled`, `laravel/prefer-command-components`, `laravel/prefer-forwards-calls`, `laravel/prefer-json-response`, `laravel/prefer-route-callable`, `laravel/prefer-typed-config`, `laravel/prefer-where-date-shorthand`, `laravel/prefer-where-like`, `laravel/prefer-where-null`, `php-8.1/no-null-builtin-argument`, `phpunit/add-array-has-key-assert`, `phpunit/prefer-direct-assertion`, `refactor/change-constant-visibility`, `refactor/change-method-visibility`, `refactor/change-property-type`, `refactor/remove-argument`, `refactor/remove-interface`, `refactor/remove-method-argument`, `refactor/remove-trait-use`, `refactor/rename-attribute`, `refactor/rename-property`, `refactor/rename-static-method`, `refactor/rename-string`, `refactor/replace-argument-default`, `refactor/replace-attribute-key`, `refactor/replace-constant`, `refactor/replace-interface`, `refactor/replace-method-with-function`, `refactor/replace-method-with-static`, `refactor/replace-new-with-static`, `refactor/replace-static-with-new`, `refactor/replace-string-with-constant`, `refactor/wrap-return`, `types/no-phpstan-phpdoc-tag`, `types/prefer-literal-bool-phpdoc` |
| Deprecated upstream (Rector 2.7.0 or main) | 31 | `complexity/no-count-in-for`, `complexity/no-foreach-array-copy`, `complexity/no-inner-function`, `complexity/no-lonely-if`, `complexity/no-redundant-is-object`, `complexity/no-redundant-null-coalesce`, `complexity/prefer-collapsed-if`, `complexity/prefer-declared-property`, `complexity/prefer-explicit-comparison`, `complexity/prefer-if-statement`, `complexity/prefer-stripos`, `complexity/prefer-ternary`, `php-7.0/no-list-order-dependency`, `php-7.3/require-json-throw-on-error`, `php-8.0/prefer-match`, `php-8.4/prefer-deprecated-attribute`, `php-8.4/prefer-property-hook`, `phpunit/no-named-data-provider-keys`, `phpunit/require-mock-suffix`, `refactor/add-interface-by-trait`, `refactor/prefer-pipe-operator`, `refactor/remove-annotation`, `refactor/rename-deprecated-method`, `refactor/replace-function-with-method`, `refactor/replace-value-with-constant`, `style/no-fluent-setters`, `style/no-setter-return-this`, `style/prefer-names-from-type`, `types/add-array-assert`, `types/add-param-type`, `types/add-phpdoc-array-type` |
| Only in Laravel sets below 12 (or Lumen) | 19 | `laravel-10/no-model-dates-property`, `laravel-10/prefer-assert-sent-times`, `laravel-10/prefer-dispatch-sync`, `laravel-10/prefer-expression-get-value`, `laravel-10/prefer-facade-fake`, `laravel-11/prefer-assert-see-html`, `laravel-11/prefer-geometry-column`, `laravel-11/prefer-queueable-trait`, `laravel-5.7/add-login-event-guard`, `laravel-5.7/add-mock-console-output-false`, `laravel-5.7/add-parent-boot-call`, `laravel-5.7/prefer-permanent-redirect`, `laravel-5.7/prefer-where-date-time`, `laravel-5.8/prefer-deferrable-provider`, `laravel-8/add-param-default`, `laravel-8/add-parent-register-call`, `laravel-9/prefer-attribute-accessor`, `laravel/prefer-class-factories`, `laravel/prefer-lumen-array-routes` |
| Only in PHPUnit sets below 11 | 9 | `phpunit/add-does-not-perform-assertion`, `phpunit/add-prophecy-trait`, `phpunit/no-deprecated-api`, `phpunit/no-test-prefix-data-provider`, `phpunit/require-parent-test-constructor`, `phpunit/require-public-data-provider`, `refactor/change-param-type`, `refactor/change-return-type`, `refactor/rename-annotation` |
| Split: each upstream rule got its own ID | 4 | `laravel/prefer-if-helpers`, `laravel/prefer-php-attributes`, `types/add-property-type`, `types/add-return-type` |
| Only in sets outside the prepared sets | 3 | `complexity/prefer-named-attribute-argument`, `laravel/prefer-carbon`, `php-8.0/no-mb-strpos-check` |
| Package upgrade set below the current major | 3 | `laravel/prefer-cashier-stripe`, `laravel/prefer-faker-method`, `laravel/prefer-livewire-attributes` |
| Gone upstream | 1 | `pest/no-and-expectation` |

`types/add-param-type`, `types/add-phpdoc-array-type` and `phpunit/no-deprecated-api` were split too; their table row
is the reason most of their Rector rules went.

**Renamed** (their Rector rule is in an in-scope set, but the old upgrade group is below the floor):
`laravel-9/add-factory-extends-phpdoc` → `laravel/add-factory-extends-phpdoc`, `php-7.1/no-extra-arguments` →
`functions/no-extra-arguments`, `php-7.4/prefer-property-exists` → `complexity/prefer-property-exists`,
`php-8.0/no-required-after-optional` → `functions/no-required-after-optional`. Groups removed: `laravel-5.7` ...
`laravel-11` and `php-5.3` ... `php-8.3` except `php-8.1` (Mago's `prefer-first-class-callable`).

**Added lint rules** (286): 284 split off a rule that had several primary sources (the upstream rule that kept the old
ID is the one closest to its name; CS-Fixer and Pun's Mago ports keep theirs), and 2 in-scope Rector rules nothing
mapped (`classes/add-dynamic-properties`, `laravel/prefer-closure-only-bind`).

| Source | Rules | IDs (without `lint/`) |
| --- | ---: | --- |
| Rector core | 144 | `arrays/prefer-array-append`, `classes/add-dynamic-properties`, `classes/no-null-property-init`, `classes/no-protected-constant-in-final`, `classes/no-protected-method-in-final`, `classes/no-protected-property-in-final`, `classes/no-redundant-readonly-property`, `classes/prefer-new-self`, `classes/prefer-self-over-static`, `classes/prefer-serialize-method`, `classes/prefer-unserialize-method`, `complexity/no-redundant-nullable-check`, `complexity/prefer-coalesce-chain`, `complexity/prefer-coalesce-over-empty-ternary`, `complexity/prefer-coalesce-over-foreach`, `complexity/prefer-empty-array-identical`, `complexity/prefer-in-array-over-foreach`, `complexity/prefer-in-array-over-search`, `complexity/prefer-not-in-array`, `complexity/prefer-simple-condition`, `complexity/require-attribute-argument-order`, `control-flow/no-else-after-return`, `control-flow/prefer-returned-condition`, `dead-code/no-always-true-instanceof-assert`, `dead-code/no-bool-ternary`, `dead-code/no-cast-in-concat`, `dead-code/no-condition-exact-return`, `dead-code/no-double-self-assignment`, `dead-code/no-empty-loop`, `dead-code/no-false-ternary`, `dead-code/no-future-php-version-check`, `dead-code/no-implode-ternary`, `dead-code/no-mirror-assignment`, `dead-code/no-null-default-argument`, `dead-code/no-null-default-named-argument`, `dead-code/no-null-tag-value`, `dead-code/no-overwritten-assignment`, `dead-code/no-tautology-ternary`, `dead-code/no-typed-property-instanceof`, `dead-code/no-unused-test-method-param`, `dead-code/no-useless-constructor`, `dead-code/no-useless-variable`, `functions/no-call-user-func-method`, `functions/no-unused-closure-variable`, `functions/require-strict-array-search`, `functions/require-strict-in-array`, `operators/prefer-boolean-and-or`, `php-8.4/prefer-array-all`, `php-8.4/prefer-array-any`, `php-8.4/prefer-array-find-key`, `phpdoc/no-mixed-overruled-by-native`, `phpdoc/no-useless-param`, `phpdoc/no-useless-readonly`, `phpdoc/no-useless-return`, `phpdoc/no-useless-var`, `phpdoc/no-void-on-magic-method`, `phpunit/add-closure-return-type-from-assert`, `phpunit/add-data-provider-iterator-return-type`, `phpunit/add-property-type-container-get`, `phpunit/add-property-type-create-mock`, `phpunit/add-property-type-setup`, `phpunit/add-property-type-setup-docblock`, `phpunit/add-return-type-from-mock-object`, `phpunit/add-test-void-return`, `refactor/replace-function-with-constant`, `statements/add-safe-strict-types`, `strings/no-single-argument-sprintf`, `strings/prefer-empty-string-identical`, `style/prefer-class-keyword`, `style/prefer-class-name-constant`, `style/prefer-early-return-of-variable`, `style/prefer-early-return-over-prepared-value`, `types/add-array-return-type`, `types/add-arrow-function-return-type`, `types/add-bool-return-type`, `types/add-bool-return-type-from-const`, `types/add-closure-never-return`, `types/add-closure-return-type`, `types/add-closure-void-return`, `types/add-datetime-property-type`, `types/add-doctrine-repository-return-type`, `types/add-fluent-return-type`, `types/add-function-void-return`, `types/add-getter-return-phpdoc`, `types/add-magic-method-return-type`, `types/add-method-void-return`, `types/add-nullable-return-type`, `types/add-numeric-return-type`, `types/add-numeric-return-type-from-scalar`, `types/add-param-phpdoc-from-callable-call`, `types/add-param-phpdoc-from-dim-fetch`, `types/add-param-phpdoc-from-local-calls`, `types/add-param-phpdoc-from-reference-assign`, `types/add-param-type-array-any-all`, `types/add-param-type-array-dim-fetch`, `types/add-param-type-array-function`, `types/add-param-type-array-map`, `types/add-param-type-array-reduce`, `types/add-param-type-arrow-dim-fetch`, `types/add-param-type-data-provider`, `types/add-param-type-dim-fetch-key`, `types/add-param-type-iterable-method-call`, `types/add-param-type-method-call`, `types/add-param-type-parent-call`, `types/add-param-type-property`, `types/add-param-type-spl-fixed-array`, `types/add-param-type-sprintf`, `types/add-param-type-variable-call`, `types/add-private-method-array-return-type`, `types/add-property-type-assigns`, `types/add-property-type-behat-context`, `types/add-property-type-constructor`, `types/add-property-type-get-repository`, `types/add-property-type-jms-object`, `types/add-property-type-jms-scalar`, `types/add-property-type-setter-getter`, `types/add-return-phpdoc-common-object`, `types/add-return-phpdoc-dim-assigned-object`, `types/add-return-phpdoc-direct-array`, `types/add-return-phpdoc-json-array`, `types/add-return-phpdoc-scalar-array`, `types/add-return-type-from-cast`, `types/add-return-type-from-constant`, `types/add-return-type-from-get-repository`, `types/add-return-type-from-native-call`, `types/add-return-type-from-new`, `types/add-return-type-from-new-array`, `types/add-return-type-from-param`, `types/add-return-type-from-parent`, `types/add-return-type-from-property`, `types/add-return-type-from-symfony-serializer`, `types/add-return-type-from-ternary`, `types/add-return-type-from-try-catch`, `types/add-return-type-from-typed-call`, `types/add-return-type-from-yield`, `types/add-string-return-type`, `types/add-string-return-type-from-scalar`, `types/add-union-return-type`, `types/add-var-phpdoc-from-constructor-param`, `types/add-var-phpdoc-from-default`, `types/add-var-phpdoc-from-dim-fetch`, `types/add-var-phpdoc-from-getter`, `types/no-nullsafe-on-non-null`, `types/prefer-identical-same-type` |
| rector-phpunit | 45 | `phpunit/add-data-provider-allow-mock-objects`, `phpunit/add-depended-test-return-type`, `phpunit/add-instanceof-assert-for-argument`, `phpunit/add-mock-intersection-param`, `phpunit/add-param-type-depends`, `phpunit/add-stub-intersection-type`, `phpunit/add-will-return-callback-param-type`, `phpunit/no-assert-contains-only`, `phpunit/no-assert-is-type`, `phpunit/no-bare-mock-assign`, `phpunit/no-entity-mock`, `phpunit/no-final-constructor-override`, `phpunit/no-float-delta-parameter`, `phpunit/no-negated-assertion`, `phpunit/prefer-assert-array-has-key`, `phpunit/prefer-assert-count`, `phpunit/prefer-assert-instanceof`, `phpunit/prefer-assert-instanceof-over-empty`, `phpunit/prefer-assert-null-true-false`, `phpunit/prefer-assert-string-contains`, `phpunit/prefer-assert-true-false`, `phpunit/prefer-attribute-over-annotation`, `phpunit/prefer-covers-attribute`, `phpunit/prefer-create-stub-argument`, `phpunit/prefer-create-stub-expression`, `phpunit/prefer-create-stub-in-coalesce`, `phpunit/prefer-data-provider-attribute`, `phpunit/prefer-depends-attribute`, `phpunit/prefer-inline-create-stub`, `phpunit/prefer-merged-will-return-callback`, `phpunit/prefer-mock-intersection-return`, `phpunit/prefer-narrow-will-return-callback`, `phpunit/prefer-narrow-with-consecutive`, `phpunit/prefer-requires-attribute`, `phpunit/prefer-same-for-scalars`, `phpunit/prefer-simple-callback-assert`, `phpunit/prefer-specific-true-false-assertion`, `phpunit/prefer-specific-will-method`, `phpunit/prefer-standalone-asserts-over-callback`, `phpunit/prefer-stub-var`, `phpunit/prefer-test-prefix`, `phpunit/prefer-test-with-attribute`, `phpunit/prefer-this-call`, `phpunit/prefer-ticket-attribute`, `phpunit/prefer-with-is-instance-of` |
| pest-plugin-rector | 44 | `pest/no-expect-exception`, `pest/no-focused-test`, `pest/no-negated-boolean-matcher`, `pest/prefer-comparison-matchers`, `pest/prefer-filesystem-matchers`, `pest/prefer-not-modifier`, `pest/prefer-strict-equality-matchers`, `pest/prefer-to-be-alpha`, `pest/prefer-to-be-alpha-numeric`, `pest/prefer-to-be-between`, `pest/prefer-to-be-digits`, `pest/prefer-to-be-directory`, `pest/prefer-to-be-empty`, `pest/prefer-to-be-file`, `pest/prefer-to-be-in`, `pest/prefer-to-be-infinite`, `pest/prefer-to-be-instance-of`, `pest/prefer-to-be-json`, `pest/prefer-to-be-list`, `pest/prefer-to-be-lowercase`, `pest/prefer-to-be-nan`, `pest/prefer-to-be-true-false`, `pest/prefer-to-be-truthy-falsy`, `pest/prefer-to-be-uppercase`, `pest/prefer-to-be-url`, `pest/prefer-to-be-uuid`, `pest/prefer-to-contain`, `pest/prefer-to-contain-equal`, `pest/prefer-to-contain-only-instances-of`, `pest/prefer-to-end-with`, `pest/prefer-to-equal-canonicalizing`, `pest/prefer-to-equal-with-delta`, `pest/prefer-to-have-count`, `pest/prefer-to-have-key`, `pest/prefer-to-have-keys`, `pest/prefer-to-have-length`, `pest/prefer-to-have-properties`, `pest/prefer-to-have-property`, `pest/prefer-to-have-same-size`, `pest/prefer-to-match`, `pest/prefer-to-match-array`, `pest/prefer-to-match-object`, `pest/prefer-to-start-with`, `pest/prefer-type-matchers` |
| rector-laravel | 41 | `laravel/add-relation-query-closure-type`, `laravel/add-tap-closure-type`, `laravel/prefer-abort-if`, `laravel/prefer-aliases-attribute`, `laravel/prefer-appends-attribute`, `laravel/prefer-backoff-attribute`, `laravel/prefer-closure-only-bind`, `laravel/prefer-collected-by-attribute`, `laravel/prefer-collects-attribute`, `laravel/prefer-command-hidden-attribute`, `laravel/prefer-connection-attribute`, `laravel/prefer-date-format-attribute`, `laravel/prefer-delay-attribute`, `laravel/prefer-delete-when-missing-models-attribute`, `laravel/prefer-description-attribute`, `laravel/prefer-error-bag-attribute`, `laravel/prefer-fail-on-timeout-attribute`, `laravel/prefer-fillable-attribute`, `laravel/prefer-guarded-attribute`, `laravel/prefer-help-attribute`, `laravel/prefer-hidden-attribute`, `laravel/prefer-job-connection-attribute`, `laravel/prefer-max-exceptions-attribute`, `laravel/prefer-preserve-keys-attribute`, `laravel/prefer-query-builder-call`, `laravel/prefer-queue-attribute`, `laravel/prefer-report-if`, `laravel/prefer-request-server`, `laravel/prefer-signature-attribute`, `laravel/prefer-status-assertion-method`, `laravel/prefer-stop-on-first-failure-attribute`, `laravel/prefer-table-attribute`, `laravel/prefer-throw-if`, `laravel/prefer-timeout-attribute`, `laravel/prefer-touches-attribute`, `laravel/prefer-tries-attribute`, `laravel/prefer-unique-for-attribute`, `laravel/prefer-validation-rule-array`, `laravel/prefer-visible-attribute`, `laravel/prefer-without-incrementing-attribute`, `laravel/prefer-without-timestamps-attribute` |
| phpstan-strict-rules (lint-shaped classes) | 11 | `classes/no-static-constructor-call`, `control-flow/no-for-init-overwrite`, `functions/require-strict-function-call`, `operators/no-loose-equality`, `style/no-dynamic-method-call`, `style/no-dynamic-method-callable`, `style/no-dynamic-property`, `style/no-dynamic-static-call`, `style/no-dynamic-static-callable`, `style/no-dynamic-static-property`, `style/no-variable-variable` |
| PHP-CS-Fixer | 1 | `operators/prefer-shorthand-operator` |

**Analyse rules.** 236 added and 113 removed: each analyse rule is now one PHPStan class. An old rule survived when one
class's rule took its name; the others' identifiers moved to the rule of the class that emits them (old → new):
`types/offset-on-non-array` → `arrays/invalid-destructuring`, `deprecated/invalid-attribute` → `attributes/invalid-class-attribute`, `deprecated/attribute` → `attributes/invalid-class-constant-attribute`, `undefined/attribute` → `attributes/invalid-class-constant-attribute`, `classes/invalid-method-declaration` → `classes/abstract-method-in-concrete-class`, `attributes/missing-override` → `classes/incompatible-override`, `classes/invalid-method` → `classes/incompatible-override`, `functions/impure-in-pure` → `classes/incompatible-override`, `types/missing-tentative-return-type` → `classes/incompatible-override`, `attributes/invalid-override` → `classes/incompatible-property-override`, `attributes/missing-property-override` → `classes/incompatible-property-override`, `classes/invalid-property` → `classes/incompatible-property-override`, `classes/static-access-mismatch` → `classes/incompatible-property-override`, `classes/invalid-static-property` → `classes/internal-static-property-access`, `classes/duplicate-enum-value` → `classes/invalid-enum`, `classes/missing-enum-case` → `classes/invalid-enum`, `classes/invalid-inheritance` → `classes/invalid-extends`, `classes/phpdoc-final-violation` → `classes/invalid-extends`, `classes/readonly-class-mismatch` → `classes/invalid-extends`, `undefined/interface` → `classes/invalid-implements`, `types/method-call-on-non-object` → `classes/invalid-method-callable`, `classes/inaccessible-constructor` → `classes/invalid-new`, `classes/unexpected-constructor-arguments` → `classes/invalid-new`, `types/new-on-non-object` → `classes/invalid-new`, `classes/call-to-abstract-method` → `classes/invalid-static-method-callable`, `types/static-call-on-non-object` → `classes/invalid-static-method-callable`, `types/useless-void-cast` → `dead-code/discarded-function-result`, `dead-code/instanceof-always-false` → `dead-code/impossible-instanceof`, `dead-code/instanceof-always-true` → `dead-code/impossible-instanceof`, `types/invalid-instanceof` → `dead-code/impossible-instanceof`, `dead-code/redundant-type-check` → `dead-code/impossible-type-check`, `dead-code/condition-always-constant` → `dead-code/number-comparison-always-constant`, `dead-code/condition-always-false` → `dead-code/number-comparison-always-constant`, `dead-code/condition-always-true` → `dead-code/number-comparison-always-constant`, `dead-code/never-written-property` → `dead-code/unused-private-property`, `dead-code/write-only-property` → `dead-code/unused-private-property`, `dead-code/left-operand-always-constant` → `dead-code/xor-always-constant`, `dead-code/right-operand-always-constant` → `dead-code/xor-always-constant`, `deprecated/enum-case` → `deprecated/class-constant`, `types/conflicting-flag-arguments` → `functions/conflicting-filter-var-flags`, `classes/internal-symbol-access` → `functions/internal-function-callable`, `types/inaccessible-callable` → `functions/invalid-callable-call`, `functions/duplicate-closure-use` → `functions/invalid-closure-use`, `laravel/no-public-model-accessor` → `laravel/no-public-scope-or-accessor`, `laravel/no-public-model-scope` → `laravel/no-public-scope-or-accessor`, `laravel/undefined-console-argument` → `laravel/undefined-console-input`, `laravel/undefined-console-option` → `laravel/undefined-console-input`, `operators/non-boolean-left-operand` → `operators/non-boolean-and-operand`, `operators/non-boolean-right-operand` → `operators/non-boolean-and-operand`, `operators/non-numeric-operand` → `operators/non-numeric-unary-minus`, `dead-code/redundant-conditional-type` → `phpdoc/invalid-conditional-return`, `phpdoc/undefined-conditional-type-subject` → `phpdoc/invalid-conditional-return`, `phpdoc/invalid-param-tag` → `phpdoc/invalid-param-callable-tag`, `types/not-callable` → `phpdoc/invalid-param-callable-tag`, `undefined/type-alias` → `phpdoc/invalid-type-alias`, `phpdoc/invalid-param-closure-this` → `phpdoc/type-mismatch`, `arrays/isset-offset` → `statements/invalid-empty`, `statements/invalid-variable-use` → `statements/invalid-empty`, `functions/argument-count` → `types/argument`, `functions/void-result-used` → `types/argument`, `phpdoc/unresolvable-type` → `types/argument`, `types/argument-by-ref` → `types/argument`, `types/duplicate-named-argument` → `types/argument`, `types/invalid-argument` → `types/argument`, `types/invalid-argument-order` → `types/argument`, `types/invalid-bitmask-argument` → `types/argument`, `types/invalid-constant-argument` → `types/argument`, `types/missing-argument` → `types/argument`, `types/named-argument-not-allowed` → `types/argument`, `types/static-closure-not-allowed` → `types/argument`, `types/undefined-named-argument` → `types/argument`, `types/unresolvable-template-type` → `types/argument`, `types/missing-return-value` → `types/arrow-function-return`, `types/return-value-in-void` → `types/arrow-function-return`, `types/invalid-this` → `types/assign-to-this`, `imports/prefixed-class-reference` → `types/catch-non-throwable`, `types/constant` → `types/class-constant-value`, `classes/invalid-constant-value` → `types/define-value`, `types/generator-void-return` → `types/generator`, `types/php-version-too-low` → `types/invalid-arrow-function-signature`, `types/invalid-nullsafe` → `types/invalid-assignment-target`, `types/invalid-generics` → `types/invalid-class-generics`, `classes/trait-used-as-type` → `types/invalid-closure-signature`, `functions/implicitly-nullable-parameter` → `types/invalid-closure-signature`, `functions/required-after-optional` → `types/invalid-closure-signature`, `phpdoc/unbound-template-type` → `types/invalid-closure-signature`, `types/unresolvable-native-type` → `types/invalid-closure-signature`, `undefined/parent` → `types/invalid-closure-signature`, `statements/invalid-expression` → `types/invalid-comparison`, `types/invalid` → `types/invalid-inc-dec`, `phpdoc/shadowed-template-type` → `types/invalid-method-tag-template`, `undefined/class` → `types/invalid-pipe`, `types/invalid-nullable-type` → `types/invalid-union-type`, `types/unset-hooked-property` → `types/invalid-unset`, `types/missing-callable-signature` → `types/missing-constant-type`, `types/missing-generic-type` → `types/missing-constant-type`, `types/missing-iterable-value-type` → `types/missing-constant-type`, `types/return-in-never-function` → `types/missing-return`, `types/array-values-on-empty` → `types/redundant-array-values`, `types/redundant-array-values-on-list` → `types/redundant-array-values`, `dead-code/unused-return-type` → `types/too-wide-arrow-function-return`, `types/unused-property-type` → `types/too-wide-arrow-function-return`, `dead-code/array-filter-always-empty` → `types/useless-array-filter`, `types/array-filter-on-empty` → `types/useless-array-filter`, `types/property-fetch-on-non-object` → `undefined/assigned-property`, `types/static-property-assign` → `undefined/assigned-static-property`, `types/static-property-on-non-object` → `undefined/assigned-static-property`, `classes/invalid-class-constant` → `undefined/class-constant`, `types/class-constant-on-non-object` → `undefined/class-constant`, `casing/name-case-mismatch` → `undefined/group-import`, `classes/self-outside-class` → `undefined/instanceof-class`, `classes/invalid-static-call` → `undefined/static-method`, `classes/trait-in-interface` → `undefined/trait`.

**Links dropped.** `fixed_by` from `analyse/dead-code/redundant-null-coalesce` (its lint rule's Rector rule is
deprecated) and from `types/missing-param-type`, `missing-property-type`, `missing-return-type` (their lint
parents were split; the new analyse rules link the split rules where one fits). Conflicts dropped:
`pest/prefer-chained-expect` / `no-and-expectation` and `phpunit/prefer-named-data-provider-keys` /
`no-named-data-provider-keys` (their other side is gone).

**Decisions** (assistant's calls where the request was silent):

- Pun's `Pun/*` ports count as Mago-derived, so a rule with one keeps its Mago mapping and its Rector sources split.
- A shared rule keeps its ID for the source closest to its name; `style/prefer-compound-assignment` stays with
  `CombinedAssign` (pun-lint's sample) and `long_to_shorthand_operator` takes `operators/prefer-shorthand-operator`.
- Strict-rules classes that are syntax only stay lint rules (O4) and split into one rule each.
- Rector-backed refactor codemods get no set; `naming` has no rules (all six are deprecated upstream).
- An identifier emitted by several classes points at one: core PHPStan before the extensions, then the class with the
  fewest identifiers, then the lowest level.
- An analyse rule's preset is the band of its class's level; strict-rules and deprecation-rules classes without a
  level keep the old preset, else `strict` / `recommended`.

## Borrowed from

- **Biome:** formatter and linter split, `check` as the one command, region ignores, `lint/<group>/<rule>` IDs.
- **Ruff:** one rule per idea whatever its origin, upstream codes redirected to it; `modernize` gated by target version.
- **oxlint:** `off` / `warn` / `error` per rule or category, `--deny-warnings`, `--max-warnings`, overrides per path.
- **Vite+ (`vp check`):** one config file and one `check` that runs every step in one process.
- **Rector:** prepared sets you tick, which became lint sets.
- **ESLint, typescript-eslint, Biome:** the `recommended` and `strict` names.
- **Go (`gofmt` / `vet`):** the formatter never changes behaviour and is never configured per line.

## Decisions

Step 1, config.md Build 1-2 (wf/names, 2026-10-17), assistant's calls where the plan is silent:

- **Groups beyond format.md §8.** Lint and analyse add the topic groups `complexity`, `dead-code`, `debug`,
  `deprecated`, `performance`, `refactor` (configurable codemods only), `security`, `style` and `undefined`, and one
  upgrade group per target, `php-5.3` ... `php-8.6` and `laravel-5.7` ... `laravel-12`, from the rule's Rector set or
  `gated_by` version. `doctrine` is an ecosystem group like `laravel`, `pest` and `phpunit`. The full list with a
  description each is `[meta.groups]` in mapping.toml.
- **The old `correctness` / `suspicious` groups** are not groups any more: they became the `intent` field (config.md
  §4, "intent stays as data"), and the rule took a topic group.
- **Lint `sets`** come from the Rector sets of the upstream rules mapped to the rule (`dead-code`, `code-quality`,
  `coding-style`, `early-return`, `privatization`, `naming`, `type-declaration` as `types`, the PHP and Laravel level
  sets as `modernize`). A rule with no Rector source takes the set its old group and tier suggest: dead and unused code
  → `dead-code`, style policy → `coding-style`, likely bugs and simplifications → `code-quality`; old `pedantic` and
  opt-in style rules stay in no set. The codemods in `refactor` are in no set.
- **Analyse `preset`** is the band of the lowest PHPStan level that emits any identifier mapped to the rule (basic 0-1,
  essential 2-3, recommended 4-5, strict 6-8, max 9-10). Rules with no PHPStan level (strict-rules, deprecation-rules,
  Larastan, Mago-only) take their old tier: `essential` → `basic`, `recommended` → `recommended`, `strict` → `strict`,
  `pedantic` → `max`. `imports/prefixed-class-reference` (Mago only, opt-in before) has no preset.
- **The pun.toml reader** accepts a full ID or `<group>/<name>` inside its own `[<command>.rules]` table. Any other
  name is an error; when the name is an upstream key (any casing or separator), an old `format.` / `lint.` /
  `analyse.` name, an old bare name, or a rule of the other command, the error names the Pun ID: ``pun.toml:
  `array-syntax` is `format/arrays/syntax` in pun.toml``. A name several rules could mean is not guessed.

**Invented lint names for the 40 moved rules** (commands.md §4; the format.md §8 names don't follow the lint verb rule):

| PHP-CS-Fixer | Lint ID | | PHP-CS-Fixer | Lint ID |
| --- | --- | --- | --- | --- |
| `no_unused_imports` | `imports/no-unused` | | `combine_consecutive_issets` | `control-flow/prefer-combined-isset` |
| `no_useless_else` | `control-flow/no-useless-else` | | `combine_consecutive_unsets` | `control-flow/prefer-combined-unset` |
| `no_useless_return` | `control-flow/no-useless-return` | | `simplified_if_return` | `control-flow/prefer-bool-return` |
| `no_empty_statement` | `statements/no-empty` | | `simplified_null_return` | `control-flow/prefer-bare-return` |
| `no_superfluous_elseif` | `control-flow/no-superfluous-elseif` | | `return_assignment` | `control-flow/no-return-assignment` |
| `no_useless_nullsafe_operator` | `operators/no-useless-nullsafe` | | `ternary_to_null_coalescing` | `operators/prefer-coalesce` |
| `no_null_property_initialization` | `classes/no-null-property-default` | | `assign_null_coalescing_to_coalesce_equal` | `operators/prefer-coalesce-assign` |
| `no_redundant_readonly_property` | `classes/no-redundant-readonly` | | `switch_continue_to_break` | `control-flow/no-switch-continue` |
| `lambda_not_used_import` | `functions/no-unused-closure-use` | | `return_to_yield_from` | `control-flow/prefer-yield-from` |
| `general_attribute_remove` | `refactor/remove-attribute` | | `protected_to_private` | `classes/no-protected-in-final` |
| `no_superfluous_phpdoc_tags` | `phpdoc/no-superfluous-tags` | | `self_static_accessor` | `classes/prefer-self-in-final` |
| `general_phpdoc_annotation_remove` | `refactor/remove-phpdoc-annotation` | | `stringable_for_to_string` | `classes/add-stringable` |
| `general_phpdoc_tag_rename` | `refactor/rename-phpdoc-tag` | | `ordered_class_elements` | `classes/require-member-order` |
| `phpdoc_no_access` | `phpdoc/no-access` | | `php_unit_method_casing` | `phpunit/require-method-casing` |
| `phpdoc_no_package` | `phpdoc/no-package` | | `php_unit_data_provider_method_order` | `phpunit/require-data-provider-order` |
| `phpdoc_no_empty_return` | `phpdoc/no-empty-return` | | `php_unit_attributes` | `phpunit/prefer-attributes` |
| `phpdoc_no_useless_inheritdoc` | `phpdoc/no-useless-inheritdoc` | | `php_unit_internal_class` | `phpunit/require-internal` |
| `phpdoc_add_missing_param_annotation` | `phpdoc/add-missing-param` | | `php_unit_size_class` | `phpunit/require-size` |
| `phpdoc_to_comment` | `phpdoc/no-misplaced` | | `php_unit_test_class_requires_covers` | `phpunit/require-covers` |
| `phpdoc_tag_no_named_arguments` | `phpdoc/require-no-named-arguments` | | `Pint/phpdoc_type_annotations_only` | `laravel/require-phpdoc-types-only` |

`protected_to_private` merges with Mago's `no-protected-in-final` and Rector's privatization rules (§2), so it takes
Mago's name. The three `general_*` rules do nothing until configured, so they are `refactor` codemods.

**Other invented names.** Analyse names drop the word their group now carries (`argument-type` → `types/argument`,
`invalid-attribute` → `attributes/invalid`, `useless-cast` → `casts/useless`, `invalid-ignore-comment` →
`comments/invalid-ignore`). `no-excessive-complexity` → `complexity/no-excessive-cyclomatic` (it counts cyclomatic
complexity; the group already says complexity), `require-string-style` → `strings/require-interpolation-style`, and
the Pun-only `no-unused-ignore` → `comments/no-unused-ignore`.
