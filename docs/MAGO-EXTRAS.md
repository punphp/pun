# Mago extras: what Mago 1.57.0 has that PHP-CS-Fixer 3.95.17 / Pint don't

Pun depends on Mago 1.57.0 (`mago-syntax`, `mago-formatter`, ...). Mago also ships a linter (`mago-linter`,
~185 rules, about 70 with auto-fixes), an analyzer, and a width-aware formatter. This file lists the Mago
capabilities with **no equivalent** among PHP-CS-Fixer's fixers (`src/Fixer/**`) or Pint's custom fixers
(`app/Fixers`: `Pint/laravel_*`, `Pint/phpdoc_type_annotations_only`), and says which ones Pun should adopt.

Sources: `/tmp/mago` at tag 1.57.0 (`crates/linter/src/rule/**`, `crates/formatter/src/settings.rs`,
`crates/analyzer/src/**`), `.oracle/php-cs-fixer/src/Fixer/**`, `.oracle/pint/app/Fixers`.

Pun adds every extra as an **opt-in** `Pun/<name>` rule. None is in any preset, so default output stays
byte-identical to Pint.

## Already covered by PHP-CS-Fixer (not extras)

These Mago fixers do the same thing as an existing CS-Fixer rule, so there is nothing to add:
array-style (`array_syntax`), str-contains / str-starts-with (`modernize_strpos`), no-is-null (`is_null`),
use-compound-assignment (`long_to_shorthand_operator`), yoda-conditions (`yoda_style`), readable-literal
(`numeric_literal_separator`), explicit-octal (`octal_notation`), braced-string-interpolation
(`explicit_string_variable`), combine-consecutive-issets, prefer-arrow-function (`use_arrow_functions`),
prefer-static-closure (`static_lambda`), prefer-pre-increment (`increment_style`), prefer-self-return-type
(`self_accessor`), no-redundant-else (`no_useless_else`), inline-variable-return (`return_assignment`),
no-redundant-readonly (`no_redundant_readonly_property`), no-null-property-init
(`no_null_property_initialization`), no-redundant-final (`no_unneeded_final_method`), no-protected-in-final
(`protected_to_private`), redundant-static (`self_static_accessor`), no-redundant-yield-from
(`yield_from_array_to_yields`), no-redundant-string-concat (`no_useless_concat_operator`), no-redundant-use
(`no_unused_imports`), no-noop (`no_empty_statement`), no-empty-comment, no-redundant-binary-string-prefix
(`no_binary_string`), no-short-bool-cast, no-trailing-space (`no_trailing_whitespace_in_comment`),
lowercase-keyword, lowercase-type-hint (`native_type_declaration_casing`), no-alias-function
(`no_alias_functions`), no-fully-qualified-global-* (`global_namespace_import`), no-short-opening-tag
(`full_opening_tag`), strict-types (`declare_strict_types`), strict-behavior (`strict_param`),
switch-continue-to-break, deprecated-cast (`short_scalar_cast`), deprecated-switch-semicolon
(`switch_case_semicolon_to_colon`), explicit-nullable-param (`nullable_type_declaration_for_default_null_value`),
strict-assertions (`php_unit_strict`), use-specific-assertions (`php_unit_construct`), assertion-style
(`php_unit_test_case_static_method_calls`).

## Mago-only capabilities

Fix = Mago ships an auto-fix. Effort = how hard it is to bring into Pun: S = token-level fixer of about 100
lines, M = needs scope or argument parsing, L = needs a CST or type information.

### Linter rules with auto-fixes

| Mago rule | What it does | Fix | Laravel relevance | Effort | Recommendation |
|---|---|---|---|---|---|
| prefer-dedicated-status-assertion (Laravel) | `->assertStatus(404)` → `->assertNotFound()` (25 codes) | fix | high (every feature test) | S | **implemented** |
| prefer-fake-helper (Laravel) | `$this->faker` → `fake()` in `*Factory` classes | fix | high (factories) | S | **implemented** |
| prefer-array-validation-rules (Laravel) | `'required\|string'` → `['required', 'string']` in `rules()` | fix | high (form requests) | M | **implemented** |
| no-implicit-model-query (Laravel) | `User::where(...)` → `User::query()->where(...)` | fix | high (Eloquent, IDE/static-analysis friendliness) | S | **implemented** (heuristic, risky) |
| use-specific-expectations (Pest) | `->toBe(true\|false\|null)` → `->toBeTrue()` etc. | fix | high (Pest is Laravel's default test runner) | M | **implemented** (exact-semantics subset) |
| use-dedicated-expectation (Pest) | `expect(is_array($x))->toBeTrue()` → `expect($x)->toBeArray()` | fix | medium | M | later |
| use-simpler-expectation (Pest) | `expect($a > $b)->toBeTrue()` → `expect($a)->toBeGreaterThan($b)` | fix | medium | M | later |
| no-only (Pest) | remove `->only()` from test chains | fix | medium (a lint concern more than a format one) | S | later |
| prefer-test-attribute (PHPUnit) | `test_foo()` → `#[Test] foo()` | fix | low (Laravel uses `test_` and Pest) | M (imports) | skip; CS-Fixer can do it in two steps (`php_unit_test_annotation` + `php_unit_attributes`) |
| final-controller | make controller classes `final` | fix | medium | S | later (CS-Fixer `final_class` is all-or-nothing) |
| prefer-array-spread | `array_merge($a, $b)` → `[...$a, ...$b]` | fix | medium | M | later (needs PHP ≥ 8.1 gating) |
| prefer-first-class-callable | `fn ($x) => strlen($x)` → `strlen(...)` | fix (unsafe) | medium | M | skip: internal functions throw on extra args |
| prefer-explode-over-preg-split | `preg_split('/,/', $s)` → `explode(',', $s)` | fix | low | S | later |
| string-style | concatenation ↔ interpolation | fix | low | M | skip (style taste) |
| no-redundant-isset | `isset($a, $a['b'])` → `isset($a['b'])` | fix | low | M | later |
| no-redundant-literal-return | `if ($x === null) return null; return $x;` → `return $x;` | fix | low | M | later |
| constant-condition | drop `if (true)` / `if (false)` | fix | low | M | later |
| no-self-assignment | delete `$a = $a;` | fix | low | S | later |
| no-empty-loop | remove loops with empty bodies | fix (unsafe) | low | S | skip (side effects in conditions) |
| no-redundant-nullsafe | `?->` where `??` / `isset` already null-checks | fix | low | M | later (CS-Fixer only handles `$this?->`) |
| no-redundant-parentheses | broader than CS-Fixer's control-paren rule | fix | low | M | skip (the mago formatter does this) |
| sensitive-parameter | add `#[\SensitiveParameter]` to `$password`-like params | fix | medium | S | later |
| invalid-open-tag | `<php?` → `<?php` | fix | low | S | skip (rare) |
| no-hash-emoji | `#️⃣` comment → `#` | fix | none | S | skip |

### Lint-only rules (no fix in Mago)

| Group | Rules | Laravel relevance | Recommendation |
|---|---|---|---|
| Laravel | prefer-casts-method, prefer-anonymous-migration, prefer-view-array, no-request-all, middleware-in-routes | high | lint path later (see below) |
| Correctness | no-dead-store, no-unused-closure-capture, no-unused-global/static, no-assign-in-condition, no-missing-format-argument, identity-comparison, suspicious-explode-arguments, no-empty-catch-clause, loop-does-not-iterate | medium | lint path later |
| Maintainability | cyclomatic-complexity, halstead, kan-defect, excessive-nesting, too-many-methods/properties, no-boolean-flag-parameter | low | out of scope for a formatter |
| Safety / security | no-eval, no-global, no-debug-symbols (`dd`, `dump`, `ray`), no-literal-password, no-insecure-comparison, tainted-data-to-sink, disallowed-functions | medium (`no-debug-symbols` is high) | lint path later |
| Naming / clarity | class-name, method-name, variable-name, no-nested-ternary, no-variable-variable, tagged-todo, ... | low | out of scope |
| WordPress / PSL / Symfony / Drupal | about 20 framework-specific rules | none | skip |

### Formatter (mago-formatter settings with no CS-Fixer equivalent)

PHP-CS-Fixer never reflows code by line width. Mago's formatter is a Prettier-style printer:
`print_width`, width-driven breaking of argument, parameter, array and attribute lists,
`method_chain_breaking_style`, `preserve_breaking_*`, `align_named_arguments`, `align_parameters`,
`array_table_style_alignment`, `always_break_named_arguments_list`, `|>` pipe-expression breaking, and
`sort_class_methods`. Pun already exposes all of this as `--engine=pun` (whole-file reprint with Mago's Pint
preset). That output is deliberately not Pint-identical, so there is nothing more to port here.

### Analyzer-backed fixes

`mago analyze --fix` can add or remove `#[\Override]` (`override_attribute.rs`), prefix unused parameters,
and drop redundant `??`, ternaries and comparisons. Every one of these needs a codebase-wide type model, so
none of them fits a per-file token fixer. Effort L; skip. Users can run `mago analyze` themselves.

## Lint path

`mago-linter` 1.57.0 is on crates.io. Beyond what Pun already builds (regex, schemars, strum and tracing are
in Cargo.lock through mago-formatter), it only adds the `mago-names`, `mago-collector`, `mago-extension`,
`mago-casing`, `mago-phpdoc-syntax`, `mago-word` and `mago-bytes` crates. Until Pun has a lint path, run
`mago lint` itself for the lint-only rules above, with the `laravel`/`pest`/`phpunit` integrations enabled in
`mago.toml`.
