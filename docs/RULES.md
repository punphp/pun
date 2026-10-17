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

Status: design. Supersedes the "Rule names" bullet in docs/PLAN.md ("Update, Oct 17"). Mago rules are no longer reported as
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

## 5. Presets

Presets are named, explicit lists that compose. `strict` includes `recommended`; `laravel` adds to whatever else is on.
Each rule's `presets` field in the mapping is the source of truth.

| Preset | format | lint | analyse |
| --- | --- | --- | --- |
| `laravel` / `psr12` / `per` / `symfony` / `empty` | Pint's presets, minus their risky rules (laravel 126 of 130) | - | - |
| `recommended` | - | 195 rules: Rector dead-code, code-quality, early-return and php-version sets (`modernize`, gated by `[php] version`); Mago correctness/safety/security reports; safe Pun rules; `no-unused-ignore` | 211 rules: everything PHPStan levels 0–5 report, plus `invalid-ignore-comment` |
| `strict` | - | +69: type-declaration, privatization, coding-style, PHPUnit code-quality; `no-loose-comparison`, `require-strict-param`, `require-strict-types`; the strict-rules bans (`no-empty`, `no-short-ternary`, `no-dynamic-name`, …) | +19 rules (level 6 `missing-*-type`, strict-rules type checks, deprecation-rules) and all four type switches on, which equals PHPStan `max` + strict-rules + deprecation-rules |
| `laravel` | - | 95: rector-laravel code-quality/collection/testing/type-declarations sets, plus version sets gated by the locked laravel/framework; Pest coding-style; Mago Laravel/Pest rules; Pun's Laravel/Pest rules; Pint-laravel's 4 risky rules | 20 Larastan rules |

```toml
[php]
version = "8.4"

[format]
preset = "laravel"                       # one Pint preset; format rules are on/off + options
rules.ordered-imports = { sort-algorithm = "alpha" }

[lint]
presets = ["recommended", "laravel"]
rules.prefer-early-return = "error"             # severity
rules.no-loose-comparison = { fix = "apply" }
rules.dead-code = "off"                  # a group name works where a rule name does

[analyse]
presets = ["recommended", "laravel"]
union-types = true
nullables = true
rules.undefined-method = { magic = true }
rules.missing-iterable-value-type = "off"
```

Rector sets map onto groups. Migrate writes `rules.<group> = "on"` plus explicit `"off"` entries for the group members
the set did not contain. When the set and the group differ too much, it writes the explicit rule list instead, so migration
day is exact in both cases. Pinkary's rector.php (dead-code, code-quality, type-declarations, privatization, early-return,
Pest coding-style, php sets, skip `AddOverrideAttributeToOverriddenMethodsRector`) becomes those groups (`dead-code`,
`complexity`, `types`, `modernize`, …) with `off` entries for non-members, plus `rules.add-override-attribute = "off"`.
Rules from those sets that are now format rules (e.g. `RemoveAlwaysElse` → `format.no-useless-else`) are added to `[format]`.

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
no-loose-comparison = { group = "suspicious", fix = "mixed", severity = "warn", presets = ["strict"], ported = true, status = "decided", note = "…" }

[rules.analyse]
unused-private-method = { group = "dead-code", severity = "error", presets = ["recommended"], fixed_by = ["lint.no-unused-private-method"], status = "decided" }

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
  `pun_only`. Still to add (config agent, once the loader exists): a test that fails when an oracle rule or identifier is
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
- **Oxc / oxlint:** Biome-style categories (`correctness` on by default, `pedantic` opt-in), which became `recommended` vs
  `strict`.
- **Deno (`fmt` / `lint` / `check`):** `check` means "everything CI needs", and the formatter has no options beyond a preset.
- **ESLint flat config:** presets are plain composable lists, not inheritance chains.
- **Go (`gofmt` / `vet`):** the formatter never changes behaviour and is never configurable per line, so format rules are
  ignored only per file or region.
