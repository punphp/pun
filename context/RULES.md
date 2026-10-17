# Rules: placement, names, presets

Pun has three commands that carry rules: `pun format`, `pun lint` and `pun analyse`. Pun replaces five upstream tools
(PHP-CS-Fixer/Pint, Rector, PHPStan with its extensions, Larastan and the Mago linter), and their rules overlap a lot.
This page decides where every rule lives. Each concept gets exactly one Pun rule in exactly one command, under a Pun name.

The data is in [`crates/pun-config/rules/mapping.toml`](../crates/pun-config/rules/mapping.toml): 1,005 Pun rules and
2,564 upstream entries, each pointing at a Pun rule or carrying a drop/unsupported reason. Every Pun rule keeps its
mapping back to the upstream rule(s) it replaces: source tool, exact upstream ID (CS-Fixer rule name, Rector class FQCN,
PHPStan identifier, Mago rule code) and how options carry over. That one file drives:

- `pun migrate`: translates configs, ignore comments and baselines;
- `pun explain <rule>`: shows the upstream names of a rule with a link to each upstream doc;
- the generated docs page **"Coming from Pint / PHPStan / Rector"**: every upstream rule and the Pun rule that replaces it.

`scripts/check-rule-mapping.py` checks it (names, prefixes, references, upstream sources; §8).

> **Revised 2026-10-17 (walkthrough with Pushpak).** Where this page and the decisions below disagree, the decisions win;
> the sections are kept for their reasoning and data. Summary: PLAN.md "The decided design"; detail in plan/.
> - **IDs (§3):** `<command>/<group>/<name>` (`format/imports/ordered`, `lint/laravel/no-request-all`), with one topic
>   and ecosystem group list shared by all commands. There are **no aliases**; renames are breaking until 1.0. Format
>   names are in format.md §9. The verb prefixes for lint (`no-`, `prefer-`, `add-`, `require-`) and the vocabulary stay.
> - **Placement (§1):** format may change only layout, casing and spelling. 40 safe fixers that delete or restructure
>   code (`no_unused_imports`, `no_useless_else`, ...) move to lint (commands.md §5).
> - **Groups (§4):** lint's intent groups (`suspicious`, `style`, ...) become data; the name uses topic groups.
>   Upgrades get one lint group per target (`lint/laravel-13/...`).
> - **Fix safety (§4):** `pun lint --fix` applies every fix of every enabled rule; `--unsafe`, `fix = "apply"` and
>   `"safe-only"` are gone. `fix = "off"` keeps a rule report-only, and safe/unsafe stays a label in the output.
> - **Severity (§4):** lint follows oxlint (`"off" | "warn" | "error"`, only `error` fails, `--deny-warnings`,
>   `--max-warnings`). Analyse rules are on or off; every finding is an error.
> - **Presets (§5):** lint and analyse no longer share a tier. Lint uses a checklist of sets (Rector's prepared sets);
>   analyse has five presets, `basic` < `essential` < `recommended` < `strict` < `max`. Add-ons are built in and turn
>   on from composer.lock.
> - **Baselines (§7, §8):** migrate does not copy PHPStan baselines; `pun analyse --generate-baseline` writes Pun's own.

Status: design. Supersedes the "Rule names" bullet in context/PLAN.md ("Update, Oct 17"). Mago rules are no longer reported as
`mago.*`; they get Pun names like everything else. Nothing here changes runtime code yet.

## 1. Placement principle

| Command | Owns | Never | Fix |
| --- | --- | --- | --- |
| `pun format` | Changes that provably keep behaviour without type information: layout, token casing, syntax spelling, import order, PHPDoc layout | Changes behaviour; needs types | Always applied; safe on save |
| `pun lint` | Every other change to code shape, plus every report that can be decided from the file alone (type information may make a fix safer) | Pure layout | Per rule: `safe`, `unsafe`, `mixed` or `none` |
| `pun analyse` | Reports that need type inference or symbols from other files (types, undefined symbols, inherited PHPDoc, deprecations) | Edits files | None. A report that something can fix names that rule in `fixed_by` |

How to place a rule. Apply the tests in order; the first that matches wins:

1. **Is it PHP-CS-Fixer-safe?** For fixers, "changes behaviour" means exactly what PHP-CS-Fixer's `isRisky()` says. Pun
   does not reclassify. A non-risky fixer goes to `format`. A risky fixer goes to `lint` with an `unsafe` fix. A Rector or
   Mago rule that does the same thing as a non-risky fixer maps onto the format rule.
2. **Does it edit code?** If so it goes to `lint`, with the fix marked `safe` (keeps behaviour), `unsafe` (can change
   behaviour; needs `--unsafe`) or `mixed` (safe where analysis proves it, unsafe otherwise).
3. **Is it a report decidable from one file without types?** Then `lint` with `fix = "none"`. Examples: strict-rules'
   `equal.notAllowed`, Mago's `no-eval`.
4. **Otherwise it needs types or other files, so it goes to `analyse`.**

**Exception (parity):** every PHPStan core and Larastan identifier stays in `analyse`, even where test 3 would pick lint, so
`pun analyse` reproduces `phpstan analyse` on migration day. There are two carve-outs: `whitespace.bom` and
`whitespace.fileEnd` are layout and map to `format.encoding` / `format.no-closing-tag`. phpstan-strict-rules and
phpstan-deprecation-rules are optional extensions, so they are placed by the tests.

Why `format` keeps PHP-CS-Fixer's safe code-shape fixers (`no_useless_else`, `no_unused_imports`, `ordered_class_elements`)
instead of moving them to lint the way Ruff makes import sorting a lint rule (`I001`): Pint runs them as formatting, and
`pun format` must reproduce Pint. They also keep behaviour, which is the only promise format makes.

### The one hard consequence: Pint's risky rules leave `format`

Pint's `laravel` preset turns on 4 risky fixers: `no_alias_functions`, `no_unreachable_default_argument_value`,
`ordered_traits` and `php_unit_set_up_tear_down_visibility`. Pinkary's pint.json adds 10 more: `final_class`,
`final_internal_class`, `final_public_method_for_abstract_class`, `strict_comparison`, `declare_strict_types`,
`modernize_types_casting`, `mb_str_functions`, `array_push`, `date_time_immutable` and `self_accessor`. That makes 14 of
Pinkary's 147 rules. Under this design they are lint rules with unsafe fixes, and:

- `pun migrate` moves them from `[format]` to `[lint]` and promotes their fix to applied (`fix = "apply"`, see §5), because
  the project already ran them unattended.
- The Pint parity target becomes **`pun check --fix` ≡ `pint`**, not `pun format` ≡ `pint`. On a project migrated from
  pint.json alone, `pun check --fix` runs exactly those 147 rules. The harness runs `pun migrate` on a temp copy that has
  only the pint.json, then `pun check --fix`.
- `pun format` and LSP format-on-save no longer rename classes (`psr_autoloading`), reorder traits or add
  `declare(strict_types=1)` on every save.

This conflicts with the earlier "`pun format` output stays Pint-identical" decision, so it is the first open question in §10.

## 2. Overlap resolution

The mapping has 105 overlap clusters (54 land in format, 51 in lint) and 18 linked analyse reports. The rules:

| # | Overlap | Home | What happens to the others at migrate time |
| --- | --- | --- | --- |
| O1 | Non-risky CS-Fixer fixer ∩ Rector / Mago (e.g. `no_useless_else` = Rector `RemoveAlwaysElse` = Mago `no-redundant-else`) | That `format` rule | Enabling the Rector/Mago rule enables the format rule. The CS-Fixer options win; Mago/Rector options are dropped, and migrate prints them |
| O2 | Risky CS-Fixer fixer ∩ anything (e.g. `modernize_strpos` = Rector `StrContains/StartsWith/EndsWith` = Mago `str-contains`) | One `lint` rule, fix `unsafe` | All map to it. If the project ran the fixer unattended (Pint or Rector), migrate promotes the fix to applied |
| O3 | Rector ∩ Mago lint ∩ Pun (e.g. `prefer-first-class-callable`) | One `lint` rule | All map to it. Where strategies differ, the merged rule covers both (`no-redundant-nullsafe`: Mago's null-check case plus Rector's non-null-type case) |
| O4 | A syntax-only strict-rules ban ∩ a fixer (e.g. `empty.notAllowed` = Mago `no-empty` = Rector `DisallowedEmptyRuleFixer`) | `lint` | `ignoreErrors` and baseline entries for the identifier are rewritten to the lint rule |
| O5 | A type-aware report ∩ a fixer (e.g. PHPStan `method.unused` vs Rector `RemoveUnusedPrivateMethod`) | Two rules, linked: `analyse.unused-private-method` reports, `lint.no-unused-private-method` fixes; the analyse rule lists it in `fixed_by` | `pun analyse` alone still reports it (PHPStan parity). `pun check` shows one diagnostic, the lint one with its `[fix:…]` tag. This is not a duplicate: one rule reports, the other rewrites |
| O6 | A fixer whose safety depends on types (`strict_comparison` vs Rector `UseIdenticalOverEqualWithSameType`) | One `lint` rule, fix `mixed` | The rule applies the safe subset by default and the rest with `--unsafe` |
| O7 | Rules that contradict each other (`simplified_null_return` vs Rector `ExplicitReturnNull`, `ternary_to_elvis_operator` vs `no-short-ternary`, `single_import_per_statement` vs `group_import`) | Both stay | `[meta] conflicts` in the mapping. `pun config check` and `pun migrate` refuse configs that enable both, like Ruff's lint-vs-format conflict warnings |
| O8 | Upstream deprecated / no-op rules | Successor | Mapped to the successor if there is one; otherwise dropped, with a note in the migrate output |
| O9 | Out of scope: rector-symfony, rector-doctrine, rector-downgrade-php; Mago's WordPress/Psl/Symfony/Drupal integrations; PHPStan `phpstanApi.*` | None | `unsupported`. `pun migrate --dry-run` lists them before anything changes, as PLAN step 1 requires |

### Worked example: strict comparison

| Upstream | Kind | Becomes |
| --- | --- | --- |
| PHP-CS-Fixer `strict_comparison` (risky) | fixer, all `==` | `lint.no-loose-comparison`, unsafe part |
| Rector `UseIdenticalOverEqualWithSameTypeRector` (code-quality) | fixer, only when both sides have the same type | `lint.no-loose-comparison`, safe part |
| Mago `identity-comparison` | report | `lint.no-loose-comparison` |
| phpstan-strict-rules `equal.notAllowed`, `notEqual.notAllowed` | report, syntax only | `lint.no-loose-comparison` |

Home: `lint`. The check needs no types to report, and a fix exists. Fix: `mixed`. `pun lint --fix` rewrites the cases where
analysis proves the operand types are identical; `--unsafe` rewrites the rest. Migrate: a pint.json with `strict_comparison`
promotes the whole fix to applied. A rector.php with code-quality enables the rule with only the safe part applied. A
phpstan.neon that includes strict-rules enables it with no automatic fix, and its `ignoreErrors` / baseline entries for
`equal.notAllowed` become `lint.no-loose-comparison` entries in pun-baseline.toml.

Other notable resolutions: `strict_param` + Rector `StrictInArray`/`StrictArraySearch` + strict-rules `function.strict` +
Mago `strict-behavior` → `lint.require-strict-param` (mixed). `declare_strict_types` + Rector `DeclareStrictTypes`/`SafeDeclareStrictTypes`
+ Mago `strict-types` → `lint.require-strict-types` (mixed). `backtick_to_shell_exec` + Rector `ShellExecFunctionCallOverBackticks`
+ strict-rules `backtick.notAllowed` + Mago `no-shell-execute-string` → `format.backtick-to-shell-exec`. Three of Rector's four `privatization`
rules + Mago `no-protected-in-final` → `format.protected-to-private`. The full list is the output of
`pun explain --overlaps`, generated from the mapping.

## 3. Rule IDs and names

- **ID = `<command>.<name>`**, for example `format.ordered-imports`, `lint.prefer-early-return`, `analyse.undefined-method`. The
  ID is printed on every output line (`path:line:col <id> message [fix:safe]`, see PLAN "Agent-optimised output"), so it is
  kept short. Groups live in the data, not in the ID.
- **Names are kebab-case and unique across all commands**, so a name alone is never ambiguous. Configs and ignore comments
  may use the bare name; output always prints the full ID.
- **Style per command:**
  - format rules name the construct they normalise (`ordered-imports`, `array-indentation`). They are the kebab-case form
    of the CS-Fixer name, which Pint users already know.
  - lint rules start with a verb: `no-*` (remove or ban), `prefer-*` (replace with a better form), `require-*` / `add-*`
    (insert).
  - analyse rules name the problem (`undefined-method`, `argument-type`, `missing-return-type`).
- **Short.** The ID is on every output line, so names are cheap in tokens: two to four words, no upstream mechanism
  (`add-return-type`, not `return-type-from-strict-native-call`).
- **Familiar.** Where Biome, ESLint, oxlint, Ruff or typescript-eslint has a name for the same concept, Pun uses it
  (`no-lonely-if`, `no-unsafe-finally`, `no-unreachable-loop`, `no-useless-catch`, `no-multi-assign`, `prefer-spread`,
  `no-negated-condition`, `require-naming-convention`), else Mago's name, else PHPStan's wording (`undefined-method`).
  The reader is a Laravel developer who has never used Rector or PHPStan.
- **Modernised where the upstream name is opaque or describes a mechanism.** Examples:

| Upstream | Pun |
| --- | --- |
| CS-Fixer `strict_comparison` / `modernize_strpos` / `psr_autoloading` / `random_api_migration` | `lint.no-loose-comparison` / `lint.prefer-str-contains` / `lint.require-class-matches-file` / `lint.prefer-random-int` |
| Rector `ChangeNestedIfsToEarlyReturnRector` / `RemoveUnusedPrivateMethodRector` / `ClassPropertyAssignToConstructorPromotionRector` | `lint.prefer-early-return` / `lint.no-unused-private-method` / `lint.prefer-constructor-promotion` |
| PHPStan `method.notFound` / `missingType.iterableValue` / `deadCode.unreachable` | `analyse.undefined-method` / `analyse.missing-iterable-value-type` / `analyse.unreachable-code` |
| Larastan `larastan.noEnvCallsOutsideOfConfig` / `larastan.relationExistence` | `analyse.no-env-outside-config` / `analyse.undefined-relation` |
| strict-rules `plus.leftNonNumeric` … `unaryMinus.nonNumeric` (15 ids) | `analyse.non-numeric-operand` (one rule) |

- **Lint prefixes.** `no-` removes or bans something. `prefer-X` replaces code with the better form X. `add-` inserts
  something that can be inferred (types, attributes, PHPDoc). `require-` reports, and may insert, something a policy
  says must be there (`require-todo-reference`, `require-static-data-provider`). Rector's PHP-version rules are mostly
  "use newer syntax X" and are all `prefer-X` (`prefer-array-find`, `prefer-pipe-operator`, `prefer-clamp`); the ones
  that fix code a newer PHP removed are `no-X` (`no-each`, `no-create-function`, `no-money-format`).
- **Refactor prefixes.** Configurable codemods in the `refactor` group (Rector `RenameMethod`, `FuncCallToStaticCall` &
  co., which do nothing until configured with a mapping) are named after the edit they make: `rename-` (rename a symbol),
  `replace-` (swap one construct for a configured other), `remove-` (delete configured calls, arguments, traits),
  `change-` (change a configured type or visibility) and `wrap-` (wrap a configured return value). These five prefixes
  are only valid in the `refactor` group; every other lint rule uses `no-`/`prefer-`/`add-`/`require-`.
- **Analyse names** name the problem, usually `<vocabulary word>-<thing>` (`undefined-method`, `unused-result`,
  `invalid-attribute`) or `<thing>-<problem>` (`argument-type`, `condition-always-true`).
- **Renames keep working.** Every rule renamed in the naming pass lists its earlier name under `aliases`; configs,
  ignore comments and baselines that use an alias still resolve.

### Vocabulary

Each word means one thing, in every command:

| Word | Means | Examples |
| --- | --- | --- |
| `unused` | declared, assigned or computed, and never read | `no-unused-assignment`, `unused-private-method`, `unused-result`, `no-unused-ignore` |
| `undefined` | referenced, but not declared anywhere | `undefined-method`, `undefined-variable`, `undefined-parent` |
| `missing` | something required or expected is absent (lint inserts it with `add-`/`require-`) | `missing-return-type`, `missing-argument`, `missing-throws` |
| `redundant` | has no effect because something else already guarantees it: an earlier check, a default, the native type, a modifier | `no-redundant-nullsafe`, `no-redundant-default-argument`, `redundant-type-check`, `redundant-null-coalesce` |
| `useless` | has no effect on its own; removing it never changes behaviour | `no-useless-cast`, `no-useless-override`, `no-useless-continue`, `useless-void-cast` |
| `dead` | a branch or handler that can never run: its condition is decided, or the exception is never thrown | `no-dead-catch`, `dead-catch`, `no-dead-php-version-check` |
| `unreachable` | statements after `return`/`throw`/`exit` (kept from ESLint and PHPStan) | `no-unreachable-code`, `unreachable-code` |
| `invalid` | malformed, or contradicts the declaration it belongs to (a PHP or PHPDoc error) | `invalid-attribute`, `invalid-var-tag`, `invalid-phpdoc` |
| `impossible` / `always-true` / `always-false` | a check whose result is known from the types | `impossible-type-check`, `condition-always-false` |
| `deprecated` | uses something marked deprecated | `deprecated-method`, `no-deprecated-phpunit-api` |

PHP-CS-Fixer's own words stay in format names (`no-superfluous-phpdoc-tags`, `no-unneeded-braces`), because format names
are the CS-Fixer names Pint users know. `unnecessary` is not used: Larastan's `noUnnecessaryCollectionCall` and
PHPStan's `nullCoalesce.unnecessary` became `redundant-collection-call` and `redundant-null-coalesce`.

### Merges

The naming pass merged upstream rules that are the same rule: 109 lint and 82 analyse rules now carry several upstream
sources, listed in their `merged` field. A merge is right when the sources make the same edit or report the same
problem and a user would never want one without the other. Examples: Rector's 33 return-type inference strategies →
`add-return-type`; 31 rector-laravel `*PropertyTo*Attribute` rules → `prefer-php-attributes`; pest-plugin-rector's
`UseToBe*` matchers → Mago's `use-dedicated-expectation` (Pun `prefer-dedicated-expectation`); Mago `no-redundant-math` + Rector `RemoveDeadZeroAndOneOperation`
→ `no-useless-math`; CS-Fixer `void_return` + Rector's four void rules → `add-void-return-type`; PHPStan's 20
`generics.*` identifiers → `invalid-generics` and its 20 "supported only on PHP X" identifiers → `php-version-too-low`.
Rules stay separate when the fix differs in safety, or when merging would widen a CS-Fixer fixer beyond what Pint does
(so `pun check --fix` stays equal to Pint): Rector's PHPUnit narrow-asserts rules are `prefer-comparison-assertion`, not
part of CS-Fixer's `php_unit_construct` (`prefer-specific-assertions`). Each merged PHPStan identifier stays in the mapping as a `variant` (§8), so migrated ignores and baselines keep their
narrow scope.

## 4. Groups, severity, fix safety

Groups are data used for docs, `pun explain`, and selecting rules in config. They are not part of the ID.

| Command | Groups |
| --- | --- |
| format (208) | `whitespace`, `control-flow`, `phpdoc`, `comments`, `imports`, `casing`, `syntax`, `operators`, `classes`, `functions`, `strings`, `testing`, `blade` |
| lint (542) | `correctness`, `suspicious`, `style`, `complexity`, `performance`, `security` (Biome's intent groups), plus `dead-code`, `types`, `modernize` (Ruff's `UP`; gated by `[php] version`), `laravel`, `testing` (PHPUnit/Pest), `refactor` (configurable codemods such as Rector `RenameMethod`, which do nothing until configured) |
| analyse (255) | `undefined`, `types`, `dead-code`, `phpdoc`, `correctness`, `strict`, `deprecated`, `laravel`, `meta` (ignore-comment errors, `dumpType` debugging) |

- **Severity:** `error` (fails the run), `warn` (printed; fails only with `--error-on-warnings`, as in Biome), `info`, or
  `off`. Lint rules default to `warn` and analyse rules to `error`. Format has no severities: `format --check` fails on any
  diff.
- **Fix safety (lint):** `safe` fixes run with `--fix`; `unsafe` fixes need `--fix --unsafe` (Biome `--unsafe`, Ruff
  `--unsafe-fixes`); `mixed` applies its safe subset with `--fix`; `none` only reports. A per-rule override
  `fix = "apply" | "safe-only" | "off"` changes this, like Biome's per-rule `fix`. Migrate writes `fix = "apply"` for the
  unsafe rules the old setup already ran unattended.
- **No levels.** A PHPStan level becomes explicit rules. Its level 1–6 parameters become rule options, recorded in
  `[upstream.phpstan-parameters]`: for example `checkMaybeUndefinedVariables` → `undefined-variable = { maybe = true }`
  and `reportMagicMethods` → `undefined-method = { magic = true }`. Levels 7–10 change how every type rule compares types,
  so they become four named `[analyse]` switches: `union-types` (L7), `nullables` (L8), `explicit-mixed` (L9) and
  `implicit-mixed` (L10). They are named settings, not a ladder.
- **`pun migrate` writes the underlying rules, not a level or a preset.** For `level: 6` it lists every analyse rule PHPStan level 6 enables, each with its option values. Strict-rules, deprecation-rules and Larastan add their rules the same way, and the needed type switches are set. So pun.toml shows exactly what runs, and a project tunes it rule by rule afterwards. Tiers (§5) are for new projects and for `pun.toml` written by hand; `pun migrate --preset` writes the closest tier plus overrides instead, with the same resolved rules.

## 5. Presets: four tiers plus add-ons

Pun has no numbered levels. A preset is one of four **tiers**, named by intent, from gentle to maximal. Each tier
includes the one below it. One tier selects rules for `pun lint` and `pun analyse` together. Format is separate: it
keeps Pint's presets (`laravel`, `psr12`, `per`, `symfony`, `empty`), and tiers never touch it.

| Tier | Intent | Examples | lint (core / with all add-ons) | analyse (core / with all add-ons) |
| --- | --- | --- | ---: | ---: |
| `essential` | Definite bugs only: code that crashes or misbehaves at runtime. Near-zero false positives, so it can go on a legacy codebase on day one | `undefined-class`, `undefined-method`, `undefined-variable`, `argument-count`, `method-call-on-non-object`, `invalid-inheritance`, `readonly-property-write`, `php-version-too-low`; lint: removed PHP APIs (`no-each`, `no-create-function`, `require-implode-separator`, gated by `[php] version`), `no-missing-format-argument`, `no-unsafe-finally`, `no-tainted-data`; removed PHPUnit APIs (`no-at-matcher`, `prefer-create-mock`) | 35 / 53 | 91 / 96 |
| `recommended` | **The default for new projects.** Adds likely bugs, dead and unused code, redundant checks, deprecated usage, safe modernisation (Rector's php sets as `prefer-*` rules, gated by `[php] version`) and, with `laravel = true`, Laravel correctness | `unused-private-method`, `condition-always-true`, `return-type`, `argument-type`, `deprecated-method`, `phpdoc-type-mismatch`; lint: the dead-code group, `no-redundant-nullsafe`, `prefer-match`, `prefer-readonly-property`, `no-eval`, `no-debug-calls`, `no-unused-ignore`; Larastan's `no-env-outside-config`, `redundant-collection-call` | 171 / 219 | 217 / 231 |
| `strict` | Adds type and strictness policy: required native types, strict comparison and `strict_types`, the strict-rules bans, privatisation, simplifying code-quality rewrites and early-return style. Turns on the `union-types` and `nullables` switches | `missing-param-type`, `missing-return-type`, `missing-iterable-value-type`, `non-boolean-condition`; lint: `add-return-type`, `add-param-type`, `no-loose-comparison`, `require-strict-types`, `require-strict-param`, `no-empty`, `no-short-ternary`, `prefer-early-return`, `prefer-property-over-local-getter`, `no-nested-ternary`; most rector-laravel code-quality and PHPUnit code-quality rules | 257 / 376 | 232 / 250 |
| `pedantic` | Everything Pun has that is not opt-in only and does not contradict another rule. Turns on `explicit-mixed` and `implicit-mixed` | `missing-throws`, `internal-symbol-access`; lint: `require-naming-convention`, `no-excessive-complexity` and the other metric limits, `require-final-class`, `prefer-arrow-function`, `prefer-static-closure`, `require-qualified-native-function`, `no-else-clause`, `require-phpdoc` | 328 / 483 | 234 / 254 |

Counts are cumulative rules enabled. "Core" is with no add-on on; "with all add-ons" adds `laravel`, `pest` and
`phpunit`. They are printed by `scripts/check-rule-mapping.py`, so this table can be checked against the data. Not in any
tier (opt-in only, `tier = "none"`): 59 lint rules and 1 analyse rule. These are the configurable codemods in the
`refactor` group (`rename-*`, `replace-*`, `remove-*`, `change-*`, `wrap-*`, `add-argument`, plus
`no-restricted-functions`, which does nothing until configured), one-time upgrade codemods that are wrong to re-run
(`no-cache-minutes` multiplies by 60; `prefer-class-factories`, `prefer-cashier-stripe`, Lumen routes), the
architectural choices in rector-laravel's static-to-injection set (`prefer-constructor-injection`,
`prefer-facade-over-app`, `prefer-request-validate`), `analyse.prefixed-class-reference` (scoped PHARs only), and the
losing side of each conflict below.

**Where a rule's tier comes from.** The default is set by its group: correctness and undefined → `essential`;
suspicious, dead-code, deprecated, security and modernize → `recommended`; types and strict → `strict`; style,
naming, complexity limits and performance → `pedantic`; refactor → `none`. Rules are then moved where the default is
wrong. A rule moves down to `essential` when it reports code that fails on the configured PHP, PHPUnit or Laravel
version (`no-each`, `no-at-matcher`, `prefer-geometry-column`), and up when its group is right but its check depends
on PHPDoc, a deprecation, or a heuristic (`invalid-generics`, `required-after-optional` and `unhandled-match` are
correctness rules in `recommended`). Lint `complexity` splits three ways: redundant-code rewrites go to `recommended`,
Rector code-quality simplifications to `strict`, and metric limits to `pedantic`. Each rule's `tier` field in the
mapping is the source of truth.

**Conflicts.** No tier enables both sides of a `[meta] conflicts` pair; the checker enforces it. The side chosen for
each pair:

| Pair | In the tiers | Opt-in only | Why |
| --- | --- | --- | --- |
| `no-short-ternary` / `prefer-short-ternary` | `no-short-ternary` (strict) | `prefer-short-ternary` | strict-rules bans `?:`; Pint does not enable `ternary_to_elvis_operator` |
| `prefer-chained-expect` / `no-and-expectation` | `prefer-chained-expect` (strict, pest) | `no-and-expectation` | pest-plugin-rector's coding-style set ships the chaining rule |
| `prefer-named-data-provider-keys` / `no-named-data-provider-keys` | `prefer-named-data-provider-keys` (pedantic, phpunit) | `no-named-data-provider-keys` | PHPUnit 11+ passes string keys as named arguments |
| `format.simplified-null-return` / `lint.require-consistent-return` | `require-consistent-return` (strict) | format rule is in no Pint preset | Tiers do not reach format |
| `format.single-import-per-statement` / `format.group-import` | - | - | Format only; Pint presets pick `single-import-per-statement` |

**PHPStan levels, for orientation.** A tier is not a level, and `pun migrate` never converts a level to a tier (§4). The
closest tier for each level (analyse only; rule options such as level 1's `maybe` and `magic` stay rule options):

| PHPStan | Closest Pun tier | Notes |
| --- | --- | --- |
| level 0–1 | `essential` | 88 of the 126 rules level 0 reports. The rest are PHPDoc and deprecation checks, which are in `recommended` |
| level 2–5 | `recommended` | PHPDoc validity (2), return and property types (3), dead code and always-true conditions (4), argument types (5) |
| level 6 | `strict` | `missing-*-type` |
| level 7–8 | `strict` | `union-types`, `nullables` switches |
| level 9–10 | `pedantic` | `explicit-mixed`, `implicit-mixed` switches |
| + phpstan-strict-rules | `strict` | Syntax-only bans are lint rules (`no-loose-comparison`, `no-empty`, …) |
| + phpstan-deprecation-rules | `recommended` | |
| + Larastan | `laravel` add-on | Undefined relations, console arguments and model appends are `essential`; most of the rest `recommended`; Octane, auth-in-request-scope and config-type checks `strict` |

For lint the nearest Rector analogy: the dead-code and php sets ≈ `recommended`; code-quality, type-declaration,
early-return, privatization, strict-rules and most of coding-style ≈ `strict`; naming and Mago's metric and style
rules ≈ `pedantic`.

### Add-ons

Three add-ons combine with any tier: `laravel` (rector-laravel, Larastan, Mago's Laravel rules, Pun's Laravel rules),
`pest` (pest-plugin-rector, Mago/Pun Pest rules) and `phpunit` (rector-phpunit, CS-Fixer `php_unit_*`). An add-on rule
lists its add-on in `presets` and still has a tier. It runs when its add-on is on **and** the selected tier includes its
tier. So `recommended` + `laravel` gives Laravel correctness and dead-code rules (`undefined-relation`,
`no-useless-with-call`, `no-request-all`), and `strict` + `laravel` adds the rector-laravel code-quality rewrites.
Gated Laravel rules (`gated_by = "laravel-version"`) also need the locked laravel/framework version. Add-ons are
switched on in pun.toml (`laravel = true`); `pun init` sets them from composer.json. The four risky rules Pint's
laravel preset runs (`no-alias-function`, `no-unreachable-default-argument-value`, `require-ordered-traits`,
`require-protected-setup`) are not in the `laravel` add-on (`require-protected-setup` is a `phpunit` rule). They are tiered like any other rule, and migrate writes them
explicitly for projects that ran Pint (§1).

### Composition

```toml
preset = "recommended"                   # tier for lint and analyse; omitted = "recommended"
laravel = true                           # add-ons: laravel, pest, phpunit
pest = true

[php]
version = "8.4"

[format]
preset = "laravel"                       # a Pint preset; tiers do not apply to format
rules.ordered-imports = { sort-algorithm = "alpha" }

[lint]
rules.prefer-early-return = "error"      # severity, on top of the tier
rules.no-loose-comparison = { fix = "apply" }
rules.dead-code = "off"                  # a group name works where a rule name does

[analyse]
preset = "strict"                        # per-command override of the tier
union-types = false                      # switches override the tier's default
rules.missing-iterable-value-type = "off"
rules.undefined-method = { magic = true }
```

Resolution, per command:

1. **Tier.** `[lint] preset` / `[analyse] preset`, else the top-level `preset`, else `recommended`. `preset = "none"`
   enables nothing, so only the rules listed in the file run (what `pun migrate` writes).
2. **Add-ons.** Add the rules of each add-on that is on, up to the same tier. `[lint] laravel = false` overrides the
   top-level value for one command.
3. **Gates.** Drop rules whose `gated_by` does not hold for `[php] version` or the locked Laravel version.
4. **Switches.** `[analyse]` type switches default to the tier's (`strict`: `union-types`, `nullables`; `pedantic`:
   also `explicit-mixed`, `implicit-mixed`) and can be set either way.
5. **Overrides,** Biome/ESLint style: group entries first, then rule entries, so `rules.dead-code = "off"` plus
   `rules.no-unused-assignment = "warn"` keeps that one rule. A value is a severity (`"error"`, `"warn"`, `"info"`), `"on"`
   (default severity), `"off"`, or a table of options plus `severity` / `fix`. Turning on an opt-in-only rule works
   the same way.
6. **Conflicts.** `pun config check` refuses a result that has both sides of a `[meta] conflicts` pair, including a
   format rule from the Pint preset against a lint rule. A tier on its own never does.

The data: `tier` on every lint and analyse rule, add-on names in `presets`, and `[meta] tiers`, `addons`,
`tier_switches` in the mapping.

### Migrate and tiers

`pun migrate` writes `preset = "none"` and the explicit rule list (§4), so pun.toml shows exactly what runs. `pun migrate
--preset` writes a short config instead: the tier and add-ons closest to the resolved rule set, plus overrides for the
difference. The algorithm:

1. Resolve the upstream configs to the exact rule set R per command (the same set plain `pun migrate` writes), with
   options and switches.
2. For each tier T and add-on combination A, compute the diff between R and `resolve(T, A)`. Pick the pair with the
   fewest override lines. On a tie, pick the lower tier and fewer add-ons. Lint and analyse are chosen separately; when
   they pick the same tier it is written once at the top level.
3. Write `preset = T`, the add-ons, `rules.<x> = "off"` for each rule in the tier but not in R, `rules.<y> = "on"` (or its
   options table) for each rule in R but not in the tier, and every switch whose value differs from the tier default.
4. Resolve the written file again and compare it with R, rule by rule and option by option. Any difference is an
   error: migrate writes nothing and prints the diff. The short form must be equivalent, never approximately equal.

Rector sets map onto groups the same way in both modes. A set becomes `rules.<group> = "on"` plus `"off"` entries for
the group members the set does not contain, or the explicit rule list when the set and the group differ too much.
Pinkary's rector.php (dead-code, code-quality, type-declarations, privatization, early-return, Pest coding-style, php
sets, skip `AddOverrideAttributeToOverriddenMethodsRector`) becomes those groups (`dead-code`, `complexity`, `types`,
`modernize`, …) with `off` entries for non-members, plus `rules.add-override-attribute = "off"`. Rules from those
sets that are now format rules (e.g. `RemoveAlwaysElse` → `format.no-useless-else`) are added to `[format]`.

**Names.** `recommended` and `strict` come from ESLint and typescript-eslint (`recommended`, `strict`) and Biome
(`recommended`). `pedantic` comes from Clippy, where it is the opt-in group that is noisy on purpose. `essential` is
Pun's: "only what is certainly broken", for adopting Pun on old code.

## 6. `pun check`: recommended, yes

`pun check` is `format --check` + `lint` + `analyse` in one process, sharing one parse and one symbol index, like
`biome check` and `vp check`. It is the command CI, pre-commit and agents should run.

- `pun check` reports everything and exits non-zero on any format diff, any lint `error` or any analyse `error`.
- `pun check --fix` applies lint fixes (safe, plus `apply`-promoted ones), then format. It never analyses-then-edits.
  This is the Pint parity command (§1) and the replacement for `rector process && pint`.
- Linked pairs (O5) collapse: when a lint rule in `fixed_by` reports the same span, only the lint diagnostic prints.
- It takes the same flags as the single commands (`--staged`, `--dirty`, paths, `--agent`). `pun migrate` rewrites
  composer scripts and CI to `pun check`.
- The single commands stay for editors (format on save), for `pun analyse` parity, and for running one stage.

## 7. Ignore comments

| Form | Applies to |
| --- | --- |
| `$x = foo(); // @pun-ignore analyse.undefined-function` | the line it trails |
| `// @pun-ignore lint.no-loose-comparison` on its own line | the next line (PHPStan's `@phpstan-ignore` rule) |
| `// @pun-ignore-next-line <id>` | the next line, explicitly |
| `// @pun-ignore-file <id>` before the first statement | the whole file. Format rules can only be ignored this way, since a format rule cannot be half-applied to a line. This keeps PHP-CS-Fixer's `@php-cs-fixer-ignore` semantics |
| `// @pun-ignore-start <id>` … `// @pun-ignore-end` | a region (Biome/Mago region ignores) |

- One or more IDs, comma-separated; a bare name is accepted. A group name (`lint.dead-code`) is allowed. A bare
  `@pun-ignore` with no ID is an error, because silent blanket ignores are what PHPStan's `ignore.unmatched*` exists to catch.
- Free text after ` -- ` is the reason: `// @pun-ignore lint.no-eval -- legacy plugin loader`.
- `/* */` and `/** */` forms work too, so PHPDoc-placed `@phpstan-ignore` ports 1:1.
- Unused ignores are reported by `lint.no-unused-ignore` (safe fix: delete). Malformed ones are reported by
  `analyse.invalid-ignore-comment` (from PHPStan `ignore.parseError`).
- Migrate rewrites `@phpstan-ignore <id>` → `@pun-ignore <pun id>` through the mapping. When several upstream ids map to
  one Pun rule they are deduplicated. `@phpstan-ignore-next-line` (no id) becomes `@pun-ignore-next-line` with the ids
  PHPStan actually reported on that line, and `@php-cs-fixer-ignore <rule>` becomes `@pun-ignore-file format.<name>`.

## 8. The mapping data

`crates/pun-config/rules/mapping.toml` (TOML, so the config crate's existing `toml` dependency reads it):

```toml
[rules.lint]
no-loose-comparison = { group = "suspicious", fix = "mixed", severity = "warn", tier = "strict", ported = true, status = "decided", note = "…" }

[rules.analyse]
unused-private-method = { group = "dead-code", severity = "error", tier = "recommended", fixed_by = ["lint.no-unused-private-method"], status = "decided" }

[upstream.php-cs-fixer]
strict_comparison = { pun = "lint.no-loose-comparison", risky = true, options = "none", status = "decided" }

[rules.lint]
add-void-return-type = { …, aliases = ["add-function-void-return-type-where-no-return", …], merged = ["php-cs-fixer:void_return", "rector:AddVoidReturnTypeWhereNoReturn", …] }

[upstream.rector]   # keys: class name without the "Rector" suffix; fully qualified on a clash
RemoveUnusedPrivateMethod = { class = "Rector\\DeadCode\\Rector\\ClassMethod\\RemoveUnusedPrivateMethodRector", pun = "lint.no-unused-private-method", sets = ["dead-code"], options = "none", status = "decided" }

[upstream.rector-symfony]
AccessTypeAnnotationToAttribute = { unsupported = "Symfony rules: out of scope (Laravel-first); migrate lists them", sets = ["annotations-to-attributes"], status = "decided" }
```

- Sections: `[upstream.php-cs-fixer]`, `pint`, `pun` (today's `Pun/*`), `rector`, `rector-laravel`, `pest-plugin-rector`,
  `rector-phpunit`, `rector-symfony`, `rector-doctrine`, `rector-downgrade-php`, `phpstan` (by error identifier; `level`
  = lowest level that emits it; `pattern = true` for `%s` identifiers), `phpstan-parameters`, `phpstan-strict-rules`,
  `phpstan-deprecation-rules`, `larastan`, `mago-linter`, `mago-analyzer`.
- `options` says how upstream options carry over. Common values: `same (option keys kebab-cased)`; `none`;
  `configurable: options carry 1:1` for Rector's configurable rules; and, for merged rules, which tool's options win.
- Upstream keys are the exact upstream IDs (CS-Fixer rule name, PHPStan identifier, Mago rule code). Rector and Pest
  entries also carry the fully qualified `class`.
- `variant` (on `[upstream.phpstan]`, `phpstan-strict-rules`, `phpstan-deprecation-rules`, `larastan` entries): the
  original PHPStan identifier, present whenever the Pun rule replaces several identifiers (`"generics.notSubtype" = { pun =
  "analyse.invalid-generics", variant = "generics.notSubtype", … }`). Output under `-v` prints it
  (`analyse.invalid-generics (generics.notSubtype)`), and migrated `ignoreErrors`, `@phpstan-ignore` and baseline
  entries keep their narrow scope (for example `invalid-generics:generics.notSubtype`) instead of ignoring the whole rule.
  To be implemented by the config agent.
- Rule fields added by the naming pass: `aliases` (earlier Pun names, still accepted), `merged` (all upstream sources of a
  rule built by merging), `pun_only = true` (no upstream source; today only `lint.no-unused-ignore`).
- `status`: every entry is `decided`. The naming pass (Oct 17) named the 807 lint and 497 analyse rules and the Mago
  analyzer mappings that were marked `tentative`; 163 more Mago analyzer codes now map onto analyse rules.
- Upstream versions are pinned in `[meta] upstream`: PHP-CS-Fixer 3.95.17, Rector 2.6.1 (Pinkary's lock),
  driftingly/rector-laravel 2.5.0, pest-plugin-rector 5.x-dev, PHPStan 2.2.7, strict-rules 2.0.12, deprecation-rules 2.0.5,
  Larastan 3.10.0, Mago 1.57.0. The Rector oracle is at `.oracle/rector` (composer).
- **Maintenance:** the mapping is hand-maintained data. `scripts/check-rule-mapping.py` fails on a `tentative` entry, a
  name that is not kebab-case or not unique across commands (aliases included), a lint name without its prefix (§3), a
  dangling `pun` / `fixed_by` / `conflicts` / `merged` reference, an upstream entry that is not exactly one of
  pun / drop / unsupported, a Rector entry without `class`, and a rule that no upstream entry maps to unless it is
  `pun_only`. It also checks the tiers (§5): every lint and analyse rule has a valid `tier`, add-on `presets` name a
  `[meta] addons` entry, each tier's resolved set contains the one below it, and no tier enables both sides of a conflict;
  it prints the rule count per tier. Still to add (config agent, once the loader exists): a test that fails when an oracle rule or identifier is
  missing from `[upstream.*]`, so bumping an upstream forces a decision for every new rule.

## 9. Summary

### Counts: upstream → Pun command

| Upstream (version) | Entries | → format | → lint | → analyse | dropped | unsupported |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| PHP-CS-Fixer 3.95.17 (76 risky) | 300 | 206 | 76 | - | 18 (deprecated → successor) | - |
| Pint custom fixers | 2 | 2 | - | - | - | - |
| Pun `Pun/*` | 17 | - | 17 | - | - | - |
| Rector core 2.6.1 | 512 | 48 | 437 | - | 24 (deprecated; Rector-internal post-rectors) | 3 (Rector-internal, Nette) |
| rector-laravel 2.5.0 | 122 | - | 122 | - | - | - |
| pest-plugin-rector | 61 | - | 61 | - | - | - |
| rector-phpunit | 113 | 9 | 100 | - | 3 | 1 (Rector-internal) |
| rector-symfony / doctrine / downgrade-php | 267 | - | - | - | - | 267 |
| PHPStan 2.2.7 core identifiers | 559 | 2 | - | 534 | 8 (playground) | 15 (`phpstanApi.*`, `phpParser.*`) |
| PHPStan level parameters | 18 | - | - | 13 rule options + 5 switches | - | - |
| phpstan-strict-rules 2.0.12 | 44 | 1 | 16 | 27 | - | - |
| phpstan-deprecation-rules 2.0.5 | 10 | - | - | 10 | - | - |
| Larastan 3.10.0 | 20 | - | - | 20 | - | - |
| Mago 1.57.0 linter | 190 | 39 | 130 | - | 2 | 19 (WordPress/Psl/Symfony) |
| Mago 1.57.0 analyzer codes | 329 | - | - | 190 | - | 139 (no PHPStan-parity equivalent; not a migrate input) |
| **Pun rules** | | **208** | **542** | **255** | | |

Before the naming pass: 902 lint and 568 analyse rules. The pass folded 354 lint rules into other rules (13 of them into
format rules such as `format.phpunit-attributes`) and 313 analyse rules into others, and dropped 6 Rector-internal or
out-of-scope ones.
144 of the 208 format rules and 31 of the 542 lint rules are ported today (`ported = true`).

### Overlaps found and how they were resolved

| Overlap | Count | Resolution |
| --- | ---: | --- |
| Safe CS-Fixer fixer ∩ Rector and/or Mago | 54 clusters | O1: the format rule. E.g. `format.no-useless-else` ← Rector `RemoveAlwaysElse`, Mago `no-redundant-else` |
| Risky CS-Fixer fixer ∩ Rector, Mago and/or strict-rules | 27 clusters | O2: one lint rule, unsafe. E.g. `lint.prefer-str-contains` ← `modernize_strpos`, Rector `StrContains/StartsWith/EndsWith`, Mago `str-contains/str-starts-with` |
| Pun / Mago / Rector / strict-rules clusters without a CS-Fixer rule | 24 | O3/O4: one lint rule. Pun's 17 are merged with their Mago originals; `prefer-casts-method` ← rector-laravel `ModelCastsPropertyToCastsMethod` + Mago; `prefer-early-return` ← Rector + Mago `prefer-early-return` |
| strict-rules syntax-only ids | 16 ids → 9 lint rules | O4: `no-loose-comparison`, `require-strict-param`, `no-empty`, `no-short-ternary`, `no-dynamic-name`, `no-loop-variable-overwrite`, `no-explicit-constructor-call`, `no-implicit-array-creation`, `require-array-filter-callback` |
| strict-rules / PHPStan layout ids ∩ format | 3 | `backtick.notAllowed`, `whitespace.bom`, `whitespace.fileEnd` → format rules |
| Type-aware report ∩ fixer | 18 linked pairs | O5: analyse reports, `fixed_by` names the fixer. `unused-private-method/property/constant`, `unreachable-code`, `duplicate-array-key`, `useless-cast`, `missing-return-type` (`add-return-type`, `add-void-return-type`), `missing-property-type`, `missing-override-attribute`, `deprecated-cast` → `format.short-scalar-cast`, … |
| Type-dependent fix safety | 3 | O6: `mixed`, for `no-loose-comparison`, `require-strict-param` and `require-strict-types` |
| Contradicting rules | 5 pairs | O7: `[meta] conflicts`; config check refuses both on |
| Mago analyzer ∩ PHPStan | 190 | Mapped onto the PHPStan-parity analyse rule; the other 139 are listed as unsupported |
| Naming-pass merges | 109 lint, 82 analyse | Same edit or same problem from several upstream rules → one rule; sources in `merged` (§3 Merges) |

## 10. Decisions the user needs to make

1. **Pint's risky rules leave `format` (§1).** My call: yes. `pun format` becomes behaviour-safe, and Pint parity is proven
   on `pun check --fix`. Pinkary moves 14 rules to lint and Pint's laravel preset moves 4. The alternative keeps a
   `format.compat` exception list so that `pun format` alone stays Pint-identical, at the cost of format-on-save being able
   to change behaviour.
2. **Bare names in config and ignore comments** (`@pun-ignore no-eval`). My call: accepted, printed as full IDs. The
   alternative requires full IDs everywhere.
3. **PHPStan core identifiers stay in analyse even when syntax-only** (parity exception). My call: yes, until v1. After
   v1 they can move to lint like strict-rules did.
4. **Out-of-scope Rector packages** (Symfony, Doctrine, downgrade; 267 rules) are `unsupported`: migrate stops and lists
   them. Confirm Pun stays Laravel-first here.
5. ~~32 lint names predate the prefix rule~~ Resolved (coordinator): renamed, old names kept as aliases. Examples:
   early-return → `prefer-early-return`, final-class → `require-final-class`, declare-strict-types → `require-strict-types`.

## Borrowed from

- **Biome:** formatter vs linter split, intent groups, safe/unsafe fixes with a per-rule `fix` override,
  `--error-on-warnings`, `check` as the one command, region ignores.
- **Ruff:** one rule per concept regardless of origin, with upstream codes redirected to it (Ruff's rule redirects); unsafe
  fixes behind a flag; `modernize` gated by target version like `UP` + `target-version`; refusing lint rules that fight the
  formatter.
- **Vite+ (`vp fmt` / `vp lint` / `vp check`):** one config file and one `check` command that runs all stages in one process.
- **Oxc / oxlint, Clippy:** categories by intent (`correctness` on by default, `pedantic` opt-in), which became the four tiers.
- **ESLint / typescript-eslint, Biome:** the `recommended` and `strict` names; per-rule overrides on top of a preset.
- **Deno (`fmt` / `lint` / `check`):** `check` means "everything CI needs", and the formatter has no options beyond a preset.
- **ESLint flat config:** presets are plain composable lists, not inheritance chains.
- **Go (`gofmt` / `vet`):** the formatter never changes behaviour and is never configurable per line, so format rules are
  ignored only per file or region.
