# Mago extras: what Mago 1.57.0 has that PHP-CS-Fixer 3.95.17 / Pint don't

Pun depends on Mago 1.57.0 (`mago-syntax`, `mago-linter`, ...). Mago also ships a linter (`mago-linter`,
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
| use-dedicated-expectation (Pest) | `expect(is_array($x))->toBeTrue()` → `expect($x)->toBeArray()` | fix | medium | M | **implemented** (exact-matcher subset, risky) |
| use-simpler-expectation (Pest) | `expect($a > $b)->toBeTrue()` → `expect($a)->toBeGreaterThan($b)` | fix | medium | M | **implemented** (comparisons, not negations) |
| no-only (Pest) | remove `->only()` from test chains | fix | medium (a lint concern more than a format one) | S | **implemented** |
| prefer-test-attribute (PHPUnit) | `test_foo()` → `#[Test] foo()` | fix | low (Laravel uses `test_` and Pest) | M (imports) | skip; CS-Fixer can do it in two steps (`php_unit_test_annotation` + `php_unit_attributes`) |
| final-controller | make controller classes `final` | fix | medium | S | **implemented** (risky; CS-Fixer `final_class` is all-or-nothing) |
| prefer-array-spread | `array_merge($a, $b)` → `[...$a, ...$b]` | fix | medium | M | **implemented** (risky: PHP ≥ 8.1) |
| prefer-first-class-callable | `fn ($x) => $this->m($x)` → `$this->m(...)` | fix (unsafe) | medium | M | **implemented** (risky, methods only like Mago's default; functions skipped: internal functions throw on extra args) |
| prefer-explode-over-preg-split | `preg_split('/,/', $s)` → `explode(',', $s)` | fix | low | S | **implemented** (limit bug fixed) |
| string-style | concatenation ↔ interpolation | fix | low | M | skip (style taste) |
| no-redundant-isset | `isset($a, $a['b'])` → `isset($a['b'])` | fix | low | M | **implemented** |
| no-redundant-literal-return | `if ($x === null) return null; return $x;` → `return $x;` | fix | low | M | **implemented** |
| constant-condition | drop `if (true)` / `if (false)` | fix | low | M | skip: rare outside debugging, and `if (false) { ... }` is an IDE-stub idiom Mago has to special-case |
| no-self-assignment | delete `$a = $a;` | fix | low | S | **implemented** (variables only) |
| no-empty-loop | remove loops with empty bodies | fix (unsafe) | low | S | skip (side effects in conditions) |
| no-redundant-nullsafe | `?->` where `??` / `isset` already null-checks | fix | low | M | **implemented** (CS-Fixer only handles `$this?->`) |
| no-redundant-parentheses | broader than CS-Fixer's control-paren rule | fix | low | M | skip (the mago formatter does this) |
| sensitive-parameter | add `#[\SensitiveParameter]` to `$password`-like params | fix | medium | S | **implemented** |
| invalid-open-tag | `<php?` → `<?php` | fix | low | S | skip (rare) |
| no-hash-emoji | `#️⃣` comment → `#` | fix | none | S | skip |

### Lint-only rules (no fix in Mago)

| Group | Rules | Laravel relevance | Recommendation |
|---|---|---|---|
| Laravel | prefer-casts-method, prefer-anonymous-migration, prefer-view-array, no-request-all, middleware-in-routes | high | **`pun lint --mago`** (see below) |
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
`sort_class_methods`. Pun's formatter stays byte-identical to Pint, so none of this is adopted (an opt-in
`--engine=pun` built on mago-formatter was removed).

### Analyzer-backed fixes

`mago analyze --fix` can add or remove `#[\Override]` (`override_attribute.rs`), prefix unused parameters,
and drop redundant `??`, ternaries and comparisons. Every one of these needs a codebase-wide type model, so
none of them fits a per-file token fixer. Effort L; skip. Users can run `mago analyze` themselves.

## Implemented in Pun

Seventeen opt-in auto-fixers live in `crates/pun-fixers/src/fixers/pun_*.rs`. They use the same `Fixer` trait and
build.rs registration as the ports and run at priority 0. Each is a token-level port of the Mago rule (Mago's
fixes are CST text edits, so its code can't be reused directly). Mago's own test cases are carried over as unit
tests; where Pun deliberately differs from Mago, the test asserts Pun's output and says why.

| Rule | Mago rule | Rewrites | Deviations from Mago |
|---|---|---|---|
| `Pun/prefer_dedicated_status_assertion` | prefer-dedicated-status-assertion | `->assertStatus(404)` → `->assertNotFound()` (25 codes) | none; skips calls with comments inside |
| `Pun/prefer_fake_helper` | prefer-fake-helper | `$this->faker` → `fake()` in classes named `*Factory` | also skips write contexts (`= `, `isset`/`unset`, `[]`), where `fake()` would not compile |
| `Pun/prefer_array_validation_rules` | prefer-array-validation-rules | `'a' => 'required\|string'` → `'a' => ['required', 'string']` in `rules()` methods | only string-literal keys, and only when the value is the whole literal |
| `Pun/no_implicit_model_query` | no-implicit-model-query | `User::where(...)` → `User::query()->where(...)` | none (risky: a name heuristic, same method and facade lists as Mago; resolves `use ... as` aliases) |
| `Pun/use_specific_expectations` | use-specific-expectations | `expect($x)->toBe(true\|false\|null)` → `->toBeTrue()` / `->toBeFalse()` / `->toBeNull()` | ports only the rewrites that keep the test's meaning. Not ported: `toEqual(...)` (loose → strict), `toBe([])`/`toBe('')` → `toBeEmpty()` (looser), and `not->toBeFalse()` → `toBeTrue()` (wrong for non-bools). Skips `namespace\expect(..)` like the other Pest rules (Mago fixes it in the global namespace) |
| `Pun/use_dedicated_expectation` | use-dedicated-expectation | `expect(is_array($x))->toBeTrue()` → `expect($x)->toBeArray()` (14 `is_*`/`file_exists`/`is_dir`), `expect(str_starts_with($s, 'a'))->toBeTrue()` → `expect($s)->toStartWith('a')` (+ `str_ends_with`, `property_exists`); keeps `->not` | risky: `toStartWith`/`toEndWith`/`toBeFile`/`toBeDirectory` also require a string, `toHaveProperty` an object (`property_exists(Foo::class, ...)` is skipped). Not ported: `is_file` → `toBeFile` (that is `file_exists`, true for directories), `is_readable`/`is_writable` → `toBeReadable`/`toBeWritable` (no such Pest matchers), `ctype_alpha`/`ctype_alnum` (Pest casts to string first), `is_resource` (PHPUnit also accepts closed resources). Only `expect(f(...))->toBeTrue()` directly, not after `->and()` |
| `Pun/use_simpler_expectation` | use-simpler-expectation | `expect($a > $b)->toBeTrue()` → `expect($a)->toBeGreaterThan($b)`, likewise `>=` `<` `<=`, `===` → `toBe`, `!==` → `not->toBe`, `instanceof Foo` → `toBeInstanceOf(Foo::class)` | only when the comparison is the root of the `expect()` argument. Not ported: `expect(!$x)->toBeTrue()` → `toBeFalse()` (`!$x` is true for every falsy value), matchers after `->not`/`->and()` (Mago's fix drops the `not`), `instanceof $var` (`$var::class` fails for class-name strings) |
| `Pun/no_only` | no-only | `test(...)->only()` → `test(...)` for `test`/`it`/`describe`/`todo`/hooks | also removes the whitespace before `->only()` so a broken chain doesn't leave a lone `;` |
| `Pun/final_controller` | final-controller | `class UserController` → `final class UserController` (not abstract/final, name ends in `Controller`) | risky (subclasses and mocks in other files break). Skips a class named exactly `Controller` (Laravel ≤ 10's concrete base controller) and controllers extended in the same file |
| `Pun/prefer_array_spread` | prefer-array-spread | `array_merge($a, [1], [], $b)` → `[...$a, 1, ...$b]` | risky: needs PHP ≥ 8.1, and a spread also accepts `Traversable`s where `array_merge()` throws. The last inlined literal keeps its line layout instead of being glued onto one line |
| `Pun/prefer_explode_over_preg_split` | prefer-explode-over-preg-split | `preg_split('/, /', $s)` → `explode(', ', $s)` | a `-1`/`0`/`null` limit is dropped: Mago keeps `-1`, which `explode()` reads as "all but the last piece". Skips empty patterns (`explode('')` throws) and non-literal limits; keeps the call's qualification and quote style |
| `Pun/prefer_first_class_callable` | prefer-first-class-callable | `fn ($x) => $this->m($x)` / `function ($x) { return Foo::m($x); }` → `$this->m(...)` / `Foo::m(...)` (methods only, Mago's default) | risky, as in Mago: the callable now receives every argument the caller passes. On laravel-framework it turns `get: fn ($value) => Carbon::parse($value)` into `Carbon::parse(...)`, which then gets the model attributes as `$timezone`; on Pinkary, `->each(fn (string $file) => $disk->delete($file))` becomes `->each($disk->delete(...))`, and `FilesystemAdapter::delete()` picks up the collection key through `func_get_args()`. Also skips return types, by-ref returns, by-ref/defaulted/attributed params, `fn (...$a) => f($a)`, closures calling a method on a variable they don't `use` by value, and calls whose receiver is a parameter (`fn ($x) => $x->equals($x)`, where Mago would read an outer `$x`) |
| `Pun/no_redundant_isset` | no-redundant-isset | `isset($a, $a['b'])` → `isset($a['b'])` (prefix or duplicate access paths) | none |
| `Pun/no_redundant_literal_return` | no-redundant-literal-return | `if ($x === null) { return null; } return $x;` → `return $x;` (with `elseif` arms / `else { return $x; }`) | float literals are not treated as literals (`-0.0 === 0.0`, but returning `$x` returns `-0.0`); alternative syntax is skipped |
| `Pun/no_self_assignment` | no-self-assignment | deletes `$a = $a;` | variables only. Mago also deletes `$this->x = $this->x;` (as "potentially unsafe"); Pun doesn't: `__set`, property hooks, Eloquent mutators and readonly properties give it effects |
| `Pun/sensitive_parameter` | sensitive-parameter | adds `#[\SensitiveParameter]` to params named `*password`/`*token`/`*secret`/`*apikey`/`*api_key` unless boolean-typed or already marked | none (it is Mago's name heuristic, so lexer `$token`s get marked too; harmless). Attribute syntax needs PHP ≥ 8.0, the effect 8.2 |
| `Pun/no_redundant_nullsafe` | no-redundant-nullsafe | `$user?->name ?? 'Guest'` / `isset($user?->profile)` → `->` | only when the property fetch is the whole left side of `??` (`'a' . $x?->b ?? 'c'` is `('a' . $x?->b) ?? 'c'`) |

Tests: `PUN_FIXERS_ONLY=$(ls crates/pun-fixers/src/fixers/pun_*.rs | xargs -n1 basename | sed 's/.rs$//' | paste -sd,) cargo test -p punphp-fixers --lib pun_`
(34 tests: Mago's cases plus edge cases). All twelve new rules were also run together on copies of Pinkary,
laravel-framework and Pest; every changed file still passes `php -l`.

Mago auto-fixes still not ported: constant-condition (see the table), plus the "skip" rows above.

### Enabling

None of these rules is in a preset, so default output is unchanged. They are lint rules: enable them under
`[lint.rules]` in `pun.toml`, with `fix = "apply"` for `pun lint --fix` / `pun check --fix` to apply them (risky ones
are marked):

```toml
[lint.rules]
prefer-dedicated-status-assertion = { fix = "apply" }
prefer-fake-helper = { fix = "apply" }
prefer-array-validation-rules = { fix = "apply" }
no-implicit-model-query = { fix = "apply" }          # risky
prefer-specific-expectations = { fix = "apply" }
prefer-dedicated-expectation = { fix = "apply" }     # risky
prefer-simpler-expectation = { fix = "apply" }
no-only = { fix = "apply" }
require-final-controller = { fix = "apply" }         # risky
prefer-array-spread = { fix = "apply" }              # risky (PHP >= 8.1)
prefer-explode-over-preg-split = { fix = "apply" }
prefer-first-class-callable = { fix = "apply" }      # risky
no-redundant-isset = { fix = "apply" }
no-redundant-literal-return = { fix = "apply" }
no-self-assignment = { fix = "apply" }
add-sensitive-parameter = { fix = "apply" }
no-redundant-nullsafe = { fix = "apply" }
```

`pun migrate` turns a pint.json `"Pun/prefer_fake_helper": true` into `prefer-fake-helper = { fix = "apply" }`.

## Lint path

`pun lint --mago [paths...]` runs Mago 1.57's five lint-only Laravel rules (no-request-all, prefer-casts-method,
prefer-anonymous-migration, prefer-view-array, middleware-in-routes) through the `mago-linter` crate, with the
Laravel integration on. It only reports (`path:line:col: level [rule] message` plus Mago's help line), never
edits, and exits 1 when it finds something. Files come from the same discovery as `pun format` (pint.json
excludes apply; Blade files and files with syntax errors are skipped). The code is `crates/pun-cli/src/lint.rs`.

It is behind the `mago-lint` cargo feature, off by default: `mago-linter` is ~65k lines (plus mago-names,
-collector, -extension, -casing, -phpdoc-syntax, -word, -bytes), about 2 minutes of extra single-core debug
build, which every `pun-cli` build on the shared box would otherwise pay. Build it with
`cargo build -p punphp --features mago-lint`; release builds that should ship the subcommand need the same
flag. On Pinkary it reports nothing (484 files); a scratch controller using `$request->all()`,
`view(...)->with(...)` and `protected $casts` gets all three reported.

Other lint-only groups (correctness, security, `no-debug-symbols`) would only need their codes added to
`RULES` in `lint.rs`. Until then, run `mago lint` itself with the `laravel`/`pest`/`phpunit` integrations enabled
in `mago.toml`.
