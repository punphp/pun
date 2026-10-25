# Mago analyzer 1.57.0: maintained patches

## Source

This directory contains the library source and normalized manifest from the published
`mago-analyzer` 1.57.0 package. The package's `.cargo_vcs_info.json` identifies upstream
revision `9656cd55d51d96bd6dc57f1f1e73c15f22cb7cc3`, directory `crates/analyzer`:

<https://github.com/carthage-software/mago/tree/9656cd55d51d96bd6dc57f1f1e73c15f22cb7cc3/crates/analyzer>

The crates.io package checksum is
`73ccd424bc8dd35ae85d15c2222418281493e58f0b5e88ddb647c00cdb64144d`.
`LICENSE-MIT` and `LICENSE-APACHE` come from the same upstream revision.

The copy excludes registry bookkeeping, the package lockfile, upstream test fixtures,
and benchmarks. The manifest removes those test targets and their development dependencies,
except `indoc`, which the inline loop-convergence tests need.
Every sibling `mago-*` dependency remains pinned to `=1.57.0`.
The root manifest patches `mago-analyzer` and `mago-codex`, excluding both directories from Pun's workspace members.

## Semantic changes

The raw callback gather patch originally changed three source files. The ranges below describe that
patch before the native-state additions documented at the end of this file.

| File | Patched lines | Change |
| --- | --- | --- |
| `src/context/mod.rs` | 68–69, 105, 259–276 | Gather depth and speculative diagnostic/state isolation. |
| `src/expression/mod.rs` | 73, 244 | Suppress expression observer hooks during the speculative walk. |
| `src/invocation/analyzer.rs` | 13, 148–200, 490–508, 531 | Gather under the raw callable parameter and use that signature for template inference only. |

The reference is PHPStan 2.3.1, source revision
`cb3b7e99489b2e3789ee2eec10501cb7a72580ea`:

- [`ArgumentsHandler::gatherClosureArgType()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/ArgumentsHandler.php#L969-L997)
  gathers before sibling arguments specialize the callback's inputs.
- [`ArgumentsHandler`'s final acceptor](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/ArgumentsHandler.php#L734-L744)
  uses those gathered types, not the contextual body result.
- [`ClosureTypeResolver`'s depth limit](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/ExprHandler/Helper/ClosureTypeResolver.php#L158-L164)
  stops speculative body walks at depth two and uses the native declared return.

The gather uses fresh artifacts and a cloned outer block. Shared mutable control-flow scopes
are detached. The live context's namespace, type-resolution context, statement span, property types,
and class-initializer cache are restored before a result or error leaves the gather.
Speculative diagnostics are recorded and discarded. Function and method return providers stay active.

The normal callback walk still publishes contextual body artifacts, argument types, and observer events.
Only the callable type supplied to template inference changes. This distinction preserves precise
inner `dumpType()` results while preventing those contextual literals from fixing output templates.

## Bounds

The patch applies to statically resolved user-defined generic functions and methods with a single
callable parameter signature and direct inline anonymous or arrow functions. The callback parameters
must have non-mixed native declarations. Callbacks without parameters are also supported.
Intrinsic builtin handling is unchanged.

Untyped and native-mixed callback parameters remain on Mago's existing path. Mago's
`analyze_function_like()` discards unresolved mixed-bound templates when initializing those parameters.
Walking under mixed would incorrectly change an identity callback's `T` return to mixed.
Supporting that case requires a separate change to parameter initialization, not a guessed return type.

Other unchanged paths include callable unions, nested closure arguments inside arrays, unpacked arguments,
callback-local templates, parameter-dependent conditional signatures, and closure bind scopes.
External analysis sessions and provider-supplied effective signatures do not use this gather.

Observer isolation covers Pun's current expression observer. A registry with stateful statement,
declaration, or call hooks needs corresponding isolation before using speculative gathers. Expression
hooks that publish observations implement `ExpressionHook::is_observer()`. The registry skips those
hooks during gathering, but runs semantic expression hooks in their existing order. Pun's model-attribute
hook and dynamic return providers therefore retain their type refinements in the speculative signature.

## Verification

Public regression tests live in `crates/pun-analyse/tests/callback_gather.rs` and
`crates/pun-analyse/tests/fixtures/callback-gather/`. The expected diagnostics are PHPStan 2.3.1's
JSON output, without identifier or message substitutions. The upstream `testBug10612` case reads
its expectations from the existing exported rule fixture.

From `crates/pun-analyse/tests/fixtures/callback-gather`, the oracle capture command is:

```sh
php ../../../../../.oracle/phpstan/vendor/bin/phpstan analyse -c phpstan.neon --error-format=json --no-progress |
	jq '.files | with_entries(.key |= split("/")[-1] | .value = .value.messages)' > expected-phpstan.json
```

The focused suite checks contextual inner dumps, assigned outer results, constant and captured returns,
renamed helpers, named method arguments, receiver/class resolution, builtin callbacks, active return
providers, observer isolation, and the recursive gather cutoff. Shared explicit-mixed conversion is
an independent requirement: a semantic mixed-valued array must print as PHPStan's `array<mixed>`,
not merely `array`.

### Recorded results

Validation used Pun `91a52dcc` plus the production dump and Origins commits (`2bad0c75` and `003d9736`).
The unchanged analyzer reports `array<1|2|3, 1|2|3>` at `bug-10612.php:39`.
The patched analyzer's public `run_mago()` reports `array<array-key, mixed>` there.
Both contextual inner dumps retain `array{1}|array{2}|array{3}` and `1|2|3`.

At that revision, the public callback suite had five passing tests and five failures, all from the remaining
`array` versus `array<mixed>` conversion difference. The failures include the original upstream case
at levels 0, 5, and 9. Expectations remain the actual oracle messages. The array-slice suite passes
all five tests, including Methods bug12880, and the debug suite passes all ten tests.

The remaining conversion difference came from `types/convert.rs::source_type(Origin::Call)`, which
supplies the declared `array<RK, RV>` from the fixture's `@return`. At that revision, `restore()` handled
mixed against mixed, but not mixed against a template's bound. The semantic patch does not alter
this separate metadata conversion.

Main integration adds template-bound restoration and resolved method-call provenance in Pun's shared
conversion. The callback suite now passes all ten tests, including the original upstream case at levels
0, 5, and 9 and the named-argument method callback. The precise inner dumps and constant/captured-return
controls remain unchanged. The maintained analyzer patch remains the same three-file change measured below.

The public Analyzer test injects a one-shot call-hook error during a gather and checks propagation.
Its successful second analysis checks that an expression observer sees the callback body once.
Removing observer filtering makes that assertion fail with two visits instead of one.
The builtin list callback, captured constant, and return-provider controls also retain their
unchanged-analyzer behavior. Earlier literal-array builtin precision mismatches are unrelated
baseline behavior and are not repaired by this patch.

Changed executable-line coverage against the published package is **79/79**:
`context/mod.rs` 17/17, `expression/mod.rs` 2/2, and `invocation/analyzer.rs` 60/60.
The error-propagation test covers the fallible gather's error path. No private helper tests or
coverage exclusions are used.

The instrumented test command is:

```sh
CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=2 CARGO_TARGET_DIR=target-callback-gather \
	CARGO_LLVM_COV_TARGET_DIR="$PWD/target-callback-gather/coverage" \
	timeout 900 nice cargo llvm-cov --dep-coverage mago-analyzer --no-report --no-fail-fast \
	-p punphp-analyse --test callback_gather --test array_slice --test debug
```

The measurement intersects added or replaced source lines with LLVM's executable-line records.
It uses `llvm-cov export --format=lcov` with the callback, array-slice, and debug test executables
and the merged profile. This measures the maintained patch, not the entire vendored analyzer.

### Semantic expression hooks during gathers

The original blanket expression-hook guard also skipped Laravel's `ModelAttributes`. A callback returning
an undocumented model column consequently contributed `mixed` even though the migration declares `string`.
The ordinary contextual visit cannot repair the raw signature selected for template inference.

`ExpressionHook::is_observer()` defaults to `false`: ordinary hooks retain upstream's semantic contract.
Pun's definedness hook and the test observers return `true`. `HookContext::raw_callback_gather` carries the
internal dispatch phase; both registry dispatches filter observers before invoking them. This preserves
semantic type updates without publishing speculative observations. Existing native invalidation on
semantic setters and hook skips remains in effect.

The public Laravel regression extends `fixtures/laravel/app/Probe.php` with direct, arrow and ordinary
closure reads of `LinkUntagged::$url`. The model has no property annotation, so the migration-backed hook
must supply its type. `scripts/analyse-spike.py oracle <disposable-pinkary-copy>` captures four `string`
dumps under both `parseModelCastsMethod` settings. Before selective dispatch, both Laravel tests fail:
the direct read is `string`, but both gathered results and the closure body read are `mixed`.

Property resolution caches a read before the after-expression hook runs. When that hook replaces the
expression type, dispatch now synchronizes an existing property cache entry with the replacement. Otherwise
a closure importing captured descendant locals, or a subsequent memoized read in its body, recovers the
pre-hook `mixed`. Entries unchanged by the hook retain their narrowed or assigned types. With both fixes,
all four dumps match the oracle under both cast settings; the combined callback/native/shape/receiver
regressions pass (27 tests). Native values remain unavailable after semantic mutation.

The `coverage/callback-observers` fixture checks the production definedness observer. A callback's raw
array parameter may be empty, while its contextual array is nonempty. Only the contextual visit may
publish variable certainty. PHPStan reports one `nullCoalesce.variable` error for the nonempty input
and none for the unknown-array control. Publishing the speculative visit would lose that distinction.

Fresh instrumentation of `callback_gather`, `laravel`, `native_lane`, and the filtered observer fixture
passes all 22 public tests. Direct `llvm-cov export` over those four executables measures all **27/27**
changed executable production lines: Pun's `infer.rs` 3/3, and the maintained analyzer's expression
dispatch 14/14, hook context 1/1, hook trait 3/3, and registry 6/6. No exclusions are used for this patch.

## Passive native state: bounded slices

`Settings::track_native_types` defaults to `false`. When enabled, the existing analysis walk retains
a second local map and a second expression map. The full lane still owns body traversal, path selection,
diagnostics, and observer events. Native transfers consume the results of that walk. They do not analyze
an expression again, strip comments, choose another path, or widen an unsupported value to `mixed`.
No native-dependent Pun rule is enabled by this patch.

The public queries are `AnalysisArtifacts::get_native_expression_type()`,
`HookContext::get_native_expression_type()`, and `HookContext::get_native_variable_type()`.
Each returns `None` when native tracking is disabled, the expression is unevaluated, or its transfer
is unsupported. `Some(mixed)` is a real PHPStan-native value, not an availability marker.
Expression queries describe the evaluation at that span. A later unsupported operation does not
invalidate earlier, completed evaluations. Local queries describe the current live scope.

### Supported transfers and ownership

- `src/native.rs` defines the supported subset and passive transfers. Root scope starts with an empty
  native map. A named user function starts its own map before default-value evaluation, using
  `type_declaration_metadata`, not PHPDoc-adjusted `type_metadata`. Missing hints mean native `mixed`.
  Scalar, null, and mixed declarations, including nullable scalar declarations, are supported.
  Unsupported default expressions or semantic hooks invalidate that scope before its body starts.
- Resolved scalar literals share the literal value already computed on the authoritative walk.
  These literal values have no PHPDoc input. Direct local reads, parenthesized values, direct scalar
  copies, plain assignments, and reassignment propagate native operands. Reassignment replaces the
  current native value. It does not restore the variable's original declaration.
  Assignment targets named `$this`, `$GLOBALS`, or starting with `$_` remain outside this subset.
- `expression/assignment/mod.rs` distinguishes the assigned local from the assignment expression.
  PHPStan's assignment-position `@var` makes the native local `mixed`, while the assignment expression
  still yields the RHS's native value. A standalone named annotation preserves the previous native local.
  Variable-less non-assignment annotations remain unsupported.
- `expression/call/function_call.rs` reuses the normal resolved target. A statically named user function
  with a supported contract returns its `return_type_declaration_metadata`, or native `mixed` without
  a return hint. Full return providers remain active and run once. Their PHPDoc-aware replacement does
  not replace the native declaration. Builtins, methods, and dynamic targets remain unsupported.
- `context/block.rs` owns native locals. Cloning a block clones its map, not a mutable shared native scope.
  `artifacts.rs` owns native expression values and uses the existing checkpoint operations for insert,
  removal, keep, rollback, merge, and opaque-hook detach/restore. Completed child-body values transfer
  to their parent alongside full values. There are no independent native convergence decisions.
- `invocation/analyzer.rs` creates paired scratch artifacts for raw callback gathers. The existing
  cloned block, detached shared scopes, context restoration, and suppressed expression observers still
  isolate speculative work. Scratch native values are not published. Callback bodies themselves remain
  outside this native subset.

### Scalar operator transfers

The second slice changes only `src/native.rs` in the analyzer. Its pure transfers consume available
native operands after the full walk visits them. No operator calls an expression analyzer, emits a
diagnostic, runs a provider, changes full locals, or chooses a control-flow path.

The reference is PHPStan 2.3.1's `Reflection/InitializerExprTypeResolver.php`: `getUnaryPlusType()`,
`getUnaryMinusType()`, `getPlusType()`, `getMinusType()`, `getMulType()`, `getDivType()`, `getModType()`,
`resolveCommonMath()`, and `resolveConcatType()`. The bounded transfers use Mago's existing scalar
types, constructors, and truthiness queries.

- Unary `+` and `-` accept single general or literal integers and finite floats. Checked integer
  arithmetic preserves constants and promotes overflow to a float, including negation of `PHP_INT_MIN`.
- Binary `+`, `-`, `*`, and `/` accept those same numeric shapes. Exact constant integer division
  retains an integer. General integer division by `1` or `-1`, and division of integer zero, yields
  native `int`. Other general integer division yields `int|float`. An integer zero multiplier folds
  to zero, including float zero when the other operand is a float. A float zero multiplier does not
  receive that extra general-operand fold, matching PHPStan.
- Integer `%` accepts a known nonzero integer divisor. Constants retain PHP's dividend sign.
  A general integer dividend yields the bounded remainder range, or literal zero for divisors `1`
  and `-1`. Division or modulo by known zero and non-finite results are unavailable.
- `!` and non-short-circuit `xor` use native truthiness. Strict comparisons accept single general
  or literal integers, finite floats, general or literal strings, booleans, and null. Numeric loose
  equality, ordering, and spaceship comparisons are supported. Direct reads of the same local,
  including parenthesized reads, retain PHPStan's equality result without a second evaluation.
  Integer constant comparisons do not round through floats.
- String concatenation accepts strings only. Constants concatenate bytes. Empty operands preserve
  the other string, and general results retain non-empty or non-falsy guarantees. Mago flattens concat
  trees without visiting intermediate binary nodes. The native transfer folds the same visited leaves
  iteratively and publishes no synthetic artifacts or observer events for those intermediate nodes.

Implicit arithmetic coercion from strings, booleans, or null is unsupported. So are arithmetic on ranges
or unions, float modulo, modulo with an unknown divisor, comparisons of refined non-literal strings or
unions, and non-numeric loose comparisons. Short-circuit operators, coalesce, exponentiation, bitwise operators,
increments, decrements, and compound assignments remain unsupported. These cases return no native
value and invalidate the live native scope. They never fall back to the full value or guessed `mixed`.
Explicit scalar casts have the separate bounded contract below.

### Deliberate availability limits

Unsupported syntax invalidates native availability before its children run. A supported operator with
unsupported operand types invalidates availability when its native transfer consumes those operands.
Completed operand artifacts remain valid records of their earlier evaluations. Subsequent assignments
do not recover that scope. This scope-wide rule avoids claiming that an unsupported operation left
aliases or locals unchanged. Each separately analyzed named function can initialize its own native scope.

Branches, loops, joins, exceptional control flow, references, arrays, members, class scopes, methods,
closures, arrows, captures, globals, statics, unset, dynamic calls, and undefined local reads
are unsupported. Functions with by-reference returns or parameters, variadics, output parameters,
assertions, generators, accessed globals, or non-scalar parameter declarations do not initialize native
state and are not supported native call targets. Non-scalar native returns are also unavailable.
External analysis sessions do not initialize native state.

Read-only expression observers retain their normal visits and can inspect both values. Semantic hook
setters, opaque mutable artifact or block access, and skipped statements, expressions, or calls invalidate
native availability and cached native values. Their effects have no native transfer contract yet.
Opaque mutation preserves the paired checkpoint history even when a hook replaces all artifacts.
The existing limitation on speculative stateful statement, declaration, and call hooks still applies.

These choices leave condition filtering and joining, reference effects, member state, callback scope
entry, native invocation specialization, and explicit native-aware semantic hook contracts for later
slices. They do not settle the separate architecture choice between a maintained Mago patch and a
separate scope engine.

### Native oracle and regression checks

`crates/pun-analyse/tests/native_lane.rs` uses the public `Analyzer` and public native queries, not
Pun's inference or conversion. `fixtures/native-lane/expected-phpstan.json` contains PHPStan 2.3.1's
unmodified native dump messages. The tests adapt scalar display syntax and union ordering only.

From the repository root, capture the oracle with a disposable directory:

```sh
mkdir -p /tmp/pun-native-oracle
sed -E 's/^([[:space:]]*)(.*); \/\/ native$/\1\\PHPStan\\dumpNativeType(\2);/' \
	crates/pun-analyse/tests/fixtures/native-lane/scalars.php > /tmp/pun-native-oracle/scalars.php
php .oracle/phpstan/vendor/bin/phpstan analyse --level=0 --error-format=json --no-progress \
	/tmp/pun-native-oracle/scalars.php > /tmp/pun-native-oracle/result.json
jq '[.files[].messages[] | select(.identifier == "phpstan.dumpNativeType") | {line,message,identifier}]' \
	/tmp/pun-native-oracle/result.json > crates/pun-analyse/tests/fixtures/native-lane/expected-phpstan.json
```

The oracle exits nonzero because native dumps are diagnostics. An unrelated implicitly-nullable
parameter deprecation is excluded by identifier. The native expectations are never generated by Pun.

The instrumented native and regression checks use a private target directory:

```sh
CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=2 CARGO_TARGET_DIR=/tmp/pun-native-lane-target \
	CARGO_LLVM_COV_TARGET_DIR=/tmp/pun-native-lane-target/coverage \
	timeout 900 nice cargo llvm-cov --dep-coverage mago-analyzer --no-report \
	-p punphp-analyse --test native_lane --test callback_gather --test array_slice --test debug
```

Changed-line measurement compares this slice with local main `030a143d`, including the new `native.rs`.
Export LCOV directly from the four instrumented test executables and the merged profile, then intersect
added or replaced source lines with executable `DA` records. The `cargo llvm-cov --dep-coverage` report
filter excludes this vendored path, so its empty report is not a coverage result.

The first slice's clean-profile run passes all 33 tests: native lane 8/8, callback gathers 10/10,
array slice 5/5, and debug 10/10. The native suite compares 29 actual PHPStan native dumps, including
nullable defaults, reassignment, copies, typed and untyped returns, assignment expressions, standalone
annotations, and nested annotated assignments. It checks 27 unsupported body transitions, unsupported
parameter and call contracts, semantic mutations during bodies and default initialization, hook skips,
paired checkpoint restoration, raw-gather observer isolation, and single-call full return providers.
Native-on and native-off runs retain identical full expression values, issues, and observer counts.

Changed analyzer executable-line coverage is **229/229 (100%)**, measured against `030a143d` with no
coverage exclusions or private-helper tests. This is line coverage of this maintained-source slice,
not branch coverage or whole-analyzer coverage.

| Source file | Covered changed executable lines |
| --- | ---: |
| `src/artifacts.rs` | 56/56 |
| `src/context/block.rs` | 1/1 |
| `src/expression/assignment/mod.rs` | 9/9 |
| `src/expression/call/function_call.rs` | 17/17 |
| `src/expression/mod.rs` | 7/7 |
| `src/invocation/analyzer.rs` | 1/1 |
| `src/lib.rs` | 4/4 |
| `src/native.rs` | 105/105 |
| `src/plugin/context.rs` | 14/14 |
| `src/settings.rs` | 1/1 |
| `src/statement/function_like/mod.rs` | 5/5 |
| `src/statement/mod.rs` | 3/3 |
| `src/utils/docblock.rs` | 6/6 |

### Scalar operator verification

`fixtures/native-lane/operators.php` adds 107 pinned PHPStan native dumps in
`expected-operators-phpstan.json`. Capture them with the oracle command above, substituting
`operators.php` for `scalars.php` and `expected-operators-phpstan.json` for `expected-phpstan.json`.
The original 29 scalar expectations are unchanged. Display adaptation also normalizes float spelling,
range spacing, and PHPStan's `non-falsy-string` to Mago's `truthy-string`. PHPStan's parenthesized
benevolent numeric union displays as Mago's `int|float`. This does not enable native-dependent rules
or add PHPStan's benevolent-union checking behavior to Mago.

On unchanged native source `20d3e19c`, the public operator test fails at line 10: `+$integer` returns
`None` instead of PHPStan's `int`. The completed slice passes all 35 tests: native lane 10/10, callback
gathers 10/10, array slice 5/5, and debug 10/10. Native tracking on and off retain equal public full
artifacts, issues, and observer visit counts. Both artifact queries and live observer queries match
all 136 native dumps. Thirty-five additional unsupported operator shapes check absence at the failed
expression and after reassignment. The previous unsupported arithmetic default and assignment cases
now use shifts, while supported arithmetic defaults and assignments have positive oracle checks.

The oracle caught another distinction during development: `0 / $integer` yields native `int`, not
`int|float`. That case remains in the fixture. A direct constant `PHP_INT_MIN / -1` probe exposed an
existing panic in the full analyzer's remainder calculation. That code is outside this slice and is
unchanged. The fixture instead annotates the existing minimum-valued local on another assignment.
PHPStan preserves its native constant while Mago's full local becomes general `int`, so the public
test can verify native minimum division and modulo without that independent full-lane panic.

The clean-profile run uses the instrumented command above with the private target
`/tmp/pun-native-operators-target`. LCOV export from the four test executables and the merged profile,
intersected with added or replaced lines against `20d3e19c`, reports **232/232 changed analyzer
executable lines** in `src/native.rs` and **123/123 changed test executable lines** in `native_lane.rs`.
There are no coverage exclusions or private-helper tests. These are line-coverage measurements, not
branch coverage or a claim of complete native operator parity.

### Scalar casts

The third passive-native slice changes only `src/native.rs` in the analyzer. It permits `(int)`,
`(float)`, `(bool)`, and `(string)`, including the `integer`, `double`, `boolean`, and `binary` aliases.
The transfer consumes the operand's existing native artifact after the authoritative walk visits it.
It performs no expression analysis, observer or provider dispatch, full-type lookup, or full-state mutation.
Native queries remain opt-in and default-off.

PHPStan 2.3.1's `InitializerExprTypeResolver::getCastType()` delegates to `toInteger()`, `toFloat()`,
`toBoolean()`, or `toString()` on the operand type. The corresponding implementations are in
`IntegerType`, `IntegerRangeType`, `FloatType`, `StringType`, `BooleanType`, `NullType`, `MixedType`,
and the `Constant/Constant*Type` classes at the pinned source revision above.

- Single native integers, finite floats, strings, booleans, null, and unconstrained mixed are supported.
  Identity casts retain the native type. Integer ranges retain their bounds under integer casts and
  become general float under float casts. Boolean casts reuse Mago's pure truthiness queries.
- General boolean conversions retain PHPStan's `0|1`, `0.0|1.0`, or `''|'1'`, not a widened integer,
  float, or string. Unconstrained native mixed casts to the requested general scalar type, as PHPStan
  specifies. This does not substitute mixed for an unavailable input.
- Constant strings use the leading numeric prefix, not whole-string `is_numeric()` or Unicode trim.
  The prefix accepts PHP's six ASCII whitespace bytes, a sign, decimal digits, an optional fraction,
  and a complete exponent. An incomplete exponent is ignored. Arbitrary bytes after the prefix are
  ignored. Non-numeric strings become numeric zero. The existing pure `string_to_int()` handles the
  valid prefix, with an explicit zero result for an infinite parsed string value.
- Finite float-to-int casts truncate and wrap modulo 2^64. Overflowing numeric strings instead saturate
  at the integer limits unless they parse as infinity, which casts to zero. Rust's saturating `as i64`
  is not a substitute for the float case.
- Constant integer strings reuse `mago_word::i64_word()`. Nonzero finite float strings use PHP's default
  `precision=14`, including rounding and uppercase exponent spelling, rather than Mago's shortest Ryu
  representation. Custom PHP INI precision is not modeled. `ConstantFloatType::toString()` deliberately
  gives both `'-0'|'0'` for either zero sign, and the native transfer preserves that union.
- A general float string retains `numeric-string&uppercase-string`. The shared `get_string_with_props()`
  constructor discards casing for numeric strings, so this transfer uses `TString::general_with_props()`
  directly. The test formatter reads the stored casing because Mago's string display also omits it.

Union inputs, including nullable parameters and unions produced by prior casts, remain unsupported.
So do non-finite literal operands or float results, constrained mixed, non-scalar operands, and other
cast syntax. General or ranged integer-to-string casts are unavailable: PHPStan's decimal-integer-string
accessory has no exact Mago scalar representation. These cases poison the current native scope through
the existing invalidation path. Reassignment does not recover availability. No flow, reference, member,
callback, call, parameter-entry, or native-dependent rule support is added.

`fixtures/native-lane/casts.php` and `expected-casts-phpstan.json` contain **133 actual PHPStan native
dumps**. Capture them with the existing native-oracle recipe, substituting `casts.php` and
`expected-casts-phpstan.json`. The oracle is PHPStan **2.3.1** at
`cb3b7e99489b2e3789ee2eec10501cb7a72580ea`. No expectation comes from Pun.

Discriminating oracle results include:

| Expression or input | PHPStan native result |
| --- | --- |
| `(int) 9223372036854775808.0` | `-9223372036854775808` |
| `(int) '9223372036854775808'` | `9223372036854775807` |
| `(int) 1e20` | `7766279631452241920` |
| `(int) -9223372036854777856.0` | `9223372036854773760` |
| `(int) '1e999'` | `0` |
| `(float) "\v\f-12.5e-1rest"` | `-1.25` |
| `(int) '1e+'` | `1` |
| `(int) "\xc2\xa012"` versus `(int) "12\xff"` | `0` versus `12` |
| `(bool) '0'` versus `(bool) '00'` | `false` versus `true` |
| `(string) 0.0` and `(string) -0.0` | `'-0'\|'0'` |
| `(string) 1.2345678901234567` | `'1.2345678901235'` |
| `(string) 1e-4` versus `(string) 1e-5` | `'0.0001'` versus `'1.0E-5'` |

The unchanged native source at `11a57541` fails the public cast test on line 10: `(int) $integer`
returns `None` instead of PHPStan's `int`. The patched public Analyzer passes all 133 dumps through
both artifact and live observer queries. The fixture also checks PHPDoc distinctions, scalar copies,
reassignments, nested casts, range casts, and retained string properties. Nineteen additional unsupported
cast contexts check absence at the cast and after reassignment. The original 29 scalar and 107 operator
expectations are unchanged.

A fresh-profile run passes **37 tests**: native lane 11/11, callback gathers 10/10, array slice 5/5,
debug 10/10, and the focused integer-division-overflow regression 1/1. Native enabled and disabled
retain identical full artifacts, issues, and observer visit counts. Every visited expression in the
cast fixture is observed once. The default-off setting is also asserted through the public API.

The instrumented commands above use private target `/tmp/pun-native-casts-target` for this slice.
The first command runs `--test native_lane --test callback_gather --test array_slice --test debug`.
A separate `--test coverage -- literal_integer_division_overflow_matches_php_and_phpstan` command
runs the existing arithmetic regression. Both use one Cargo job and two Rayon threads.
Fresh LCOV export from those five executables, intersected with added or replaced lines against
`11a57541`, measures **152/152 changed analyzer executable lines** and **39/39 changed test executable
lines**. There are no exclusions, gap entries, or private-helper tests. This is changed-line coverage,
not branch coverage or whole-analyzer coverage.

## Literal integer division overflow

`src/expression/binary/arithmetic.rs` uses checked remainder and division when folding integer division.
After the existing zero check, signed division overflow produces a literal float through the normal
numeric-result path. The matching `PHP_INT_MIN % -1` case produces literal integer zero instead of
falling back to general `int`. Other arithmetic precision and error-result handling are unchanged.

The reference is PHPStan 2.3.1's `Reflection/InitializerExprTypeResolver.php`, `getDivType()` and
`getModType()`, at the source revision above. `getDivType()` explicitly preserves the float result
when integer division has no remainder but overflows. PHP 8.5.12 on this 64-bit host agrees.

The public Analyzer regression is
`crates/pun-analyse/tests/coverage.rs::literal_integer_division_overflow_matches_php_and_phpstan`.
Its fixture directory is `tests/fixtures/coverage/integer-division-overflow/` in that crate.
The PHP file has no annotations. `(-9223372036854775807 - 1)` constructs the actual minimum integer.
The direct spelling `-9223372036854775808` is a separate control because PHP parses its positive
literal as a float before negation.

On unchanged arithmetic source `3d85e90f`, this public test panics at `arithmetic.rs:863:34`:
`attempt to calculate the remainder with overflow`. The first expression reaches that code without
an annotation, alternate walk, or file skip. The patched analyzer returns float
`9.223372036854776E+18` for minimum division by `-1` and integer `0` for the corresponding modulo.

The committed expectations come from the pinned PHPStan executable and actual PHP evaluation.
All 13 numeric results match their PHPStan dump and PHP runtime value. The controls include both
signed integer neighbors, minimum division by `1` and `-2`, ordinary integral division, positive and
negative fractional float division, negative modulo, and the direct overflowing literal spelling.
The two zero-divisor controls produce PHP `DivisionByZeroError` and PHPStan `*ERROR*`. The test
checks Mago's located `invalid-operand` diagnostics and unavailable native values for those controls.
Mago's pre-existing mixed full-type fallback after the invalid pair is not PHPStan `ErrorType` parity
and remains outside this patch. General non-integral integer division precision is also unchanged.

Capture the PHPStan expectations from the repository root with the same pinned executable:

```sh
mkdir -p /tmp/pun-native-arithmetic-oracle
sed -E 's/^([[:space:]]*)(.*); \/\/ result$/\1\\PHPStan\\dumpType(\2);/' \
	crates/pun-analyse/tests/fixtures/coverage/integer-division-overflow/literals.php \
	> /tmp/pun-native-arithmetic-oracle/literals.php
php .oracle/phpstan/vendor/bin/phpstan analyse --level=0 --error-format=json --no-progress \
	/tmp/pun-native-arithmetic-oracle/literals.php > /tmp/pun-native-arithmetic-oracle/result.json
jq '[.files[].messages[] | select(.identifier == "phpstan.dumpType") | {line,message,identifier}]' \
	/tmp/pun-native-arithmetic-oracle/result.json
```

The oracle exits 1 because dumps are diagnostics. The PHP runtime expectations evaluate each marked
expression independently with `eval('return ' . $expression . ';')`, recording `get_debug_type()` and
the value, or the caught exception's class and message. JSON uses `JSON_PRESERVE_ZERO_FRACTION`.
Neither expectation file is generated from Pun.

The focused test compares native tracking enabled and disabled: full expression types, references,
and issues are equal. The unchanged native suite also compares all public full artifacts and observer
counts, preserving the existing 29 scalar and 107 operator oracle expectations.

The clean-profile validation passes **36 tests**: the focused regression, native lane 10/10, callback
gathers 10/10, array slice 5/5, and debug 10/10. It uses one Cargo job, two Rayon threads, and the private
target `/tmp/pun-native-operators-target`:

```sh
CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=2 CARGO_TARGET_DIR=/tmp/pun-native-operators-target \
	CARGO_LLVM_COV_TARGET_DIR=/tmp/pun-native-operators-target/coverage \
	timeout 900 nice cargo llvm-cov --dep-coverage mago-analyzer --no-report \
	-p punphp-analyse --test coverage -- literal_integer_division_overflow_matches_php_and_phpstan
CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=2 CARGO_TARGET_DIR=/tmp/pun-native-operators-target \
	CARGO_LLVM_COV_TARGET_DIR=/tmp/pun-native-operators-target/coverage \
	timeout 900 nice cargo llvm-cov --dep-coverage mago-analyzer --no-report \
	-p punphp-analyse --test native_lane --test callback_gather --test array_slice --test debug
```

LCOV export from these five executables and the merged fresh profile, intersected with added or
replaced source lines against `3d85e90f`, reports **9/9 changed analyzer executable lines** and
**104/104 changed test executable lines**. No coverage exclusions, gap entries, or private-helper tests
are used. This measures changed executable lines, not branch coverage or the whole analyzer.

## Full expression arithmetic and global constants

These changes use the same Mago 1.57.0 package and PHPStan 2.3.1 revisions listed above.
They preserve the separate signed-division overflow correction documented above.
No dependency versions change.

| File | Patched lines | Change |
| --- | --- | --- |
| `src/expression/binary/arithmetic.rs` | 749–756, 785, 802–824, 1069 | Keep non-finite float results and evaluate literal float remainders after PHP integer conversion. |
| `src/expression/constant_access.rs` | 35–42, 98–116 | Restrict namespace fallback, honor authoritative metadata, and reuse evaluated declaration values. |
| `src/context/mod.rs` | 68–69, 107 | Retain evaluated global constant values across function-local artifacts in one file analysis. |
| `src/statement/constant.rs` | 47–51 | Save initializer values produced by the full expression analyzer. |

Pun's `crates/pun-analyse/src/codebase.rs:213–240` supplies exact numeric builtin metadata from
PHPStan's exported symbols. The metadata includes `INF`, `NAN`, and the platform unions for
`PHP_INT_SIZE`, `PHP_INT_MAX`, and `PHP_INT_MIN`. Explicit metadata takes precedence over Mago's
platform approximation. That approximation remains the fallback for builtin PHPDoc types.
Platform handling uses the selected symbol's name and builtin flag, not the alias at its use site.

The arithmetic reference is PHPStan's
[`InitializerExprTypeResolver::getModType()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Reflection/InitializerExprTypeResolver.php#L1397-L1455).
Both scalar operands become integers before `%`. Integer operands remain integers throughout,
including values above `2^53`. Float conversion follows PHP's `zend_dval_to_lval`: truncate,
wrap modulo `2^64`, and convert non-finite values to zero. Divisor checks use the converted value,
so `0.9`, `INF`, and `NAN` are zero divisors. The signed minimum modulo `-1` is zero, not a Rust panic.
Float arithmetic retains `INF` and `NAN` instead of discarding them as general `float`.

The constant reference is
[`ConstantResolver::resolveConstant()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/ConstantResolver.php#L66-L95)
and its `resolvePredefinedConstant()` implementation. A qualified or imported missing constant
cannot fall back to a global constant with the same short name. Evaluated initializer values are
keyed by the selected declaration's span, including its file identity. Explicit declared types
retain their precedence. No variable type is replaced with a declaration: reassignment and assertion
narrowing still use the actual inferred value.

### Arithmetic and constant verification

The new public `duplicate_array_keys::full_arithmetic_and_constant_resolution_match_phpstan` test
compares all 43 diagnostics in `fixtures/duplicate-array-keys/expected-full-constants.json`.
The capture command, from that fixture directory, is:

```sh
php /home/user/workspace/repo/.oracle/phpstan/vendor/bin/phpstan analyse \
	-c full-constants.neon --no-progress --error-format=json |
	jq '.files |= with_entries(.key |= split("/")[-1])' > expected-full-constants.json
```

Before the patch, the five duplicate diagnostics for `OVERFLOW`, `NONFINITE`, `MODULO`, and
`QUALIFIED_SIZE` are absent. Runtime overflow and non-finite subtraction produce general `float`.
After the patch, all nine duplicate-array-key public tests pass. The new fixture also checks
runtime expressions, both float wrap directions, integer precision, builtin aliases, source
shadowing, missing qualified constants, zero divisors, unknown operands, reassignment, and assertion
narrowing. The retained division control still produces `9.223372036854776E+18`.

The combined suites pass **36 public tests and three unit tests**: duplicate array keys, array
slicing, callable origins, debug shape metadata, and callback gathers. The exported Arrays fixtures
match **5/5 duplicate-key cases and 1/1 offset case**, with no new failures. The old worktree's
runner still exits 101 because its gap list contains the now-passing `testDuplicateKeys` case.
Main already removed that stale gap. This patch does not edit the list.

Fresh changed executable-line coverage against this slice's parent is **49/49**, without exclusions:
arithmetic 24/24, constant access 16/16, declaration recording 5/5, context initialization 1/1,
and Pun's builtin metadata 3/3. Coverage comes from the nine public duplicate-key tests alone.
The instrumented command is:

```sh
CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=2 CARGO_TARGET_DIR=target-array-duplicate-keys \
	CARGO_LLVM_COV_TARGET_DIR="$PWD/target-array-duplicate-keys/coverage" \
	timeout 900 nice cargo llvm-cov test --dep-coverage mago-analyzer \
	-p punphp-analyse --test duplicate_array_keys --no-report
```

The measurement merges that executable's fresh raw profile with `llvm-profdata merge -sparse`,
then uses `llvm-cov export -format=lcov` directly. The intersection with added production lines
includes `codebase.rs` and the four analyzer files in the table. Cargo's dependency-name report
filter excludes this local patched path, so its empty report is not the measurement.

### Global initializer resolution before inference

The original file-context cache misses a declaration after the consuming function or in a
scanned-only file. `const KEY = 5.9 % 2.0` therefore gives a copied variable `float|int`, although
the array rule independently recovers its value `1`.

Pun's `codebase::resolve_initializers()` now resolves referenced global declarations before the
normal inference walk. It uses the same production expression evaluator as array initializer
recovery, not another arithmetic implementation. The resolver retains only already-loaded global
declaration files alongside the analysed sources. Composer's lazy scan rounds do not change.
Only referenced initializers and their dependencies are evaluated. Other function bodies and
declarations in a scanned file are not analysed.

Each evaluation keeps the original file identity, resolved imports, and namespace. It runs an
isolated expression statement against the declaration metadata, without publishing diagnostics
or observer events. A declaration-span recursion guard removes the active approximation while
resolving dependencies, matching `ConstantResolver`'s mixed recursive lookup. Completed values
become authoritative declaration metadata, so ordinary assignments and later assertions see the
same types. Global PHPDoc `@var` tags do not widen the reflected value. Builtin platform unions
keep their existing metadata and are not reevaluated.

When Mago's mixed operand loses the scanner's general `int|float` bound, the resolver preserves
that bound with PHPStan's benevolent-union marker. The marker survives dependent numeric joins.
Neither path manufactures a literal value for a cycle. Failed evaluations retain the original
metadata. This does not implement general runtime evaluation of `define()` expressions that
depend on a caller's local variables or function-specific dynamic return providers.

`tests/constant_initializers.rs` compares full public diagnostics with PHPStan 2.3.1, including
forward declarations, scanned dependencies, copies, reassignment, assertion narrowing, platform
unions, recursive declarations, global PHPDoc, namespace identity, and literal `define()` values.
The lazy-autoload control has an unreferenced file with a conflicting constant: scanning that
file would change the observed value from `1` to `99`.

Capture the expectations from `crates/pun-analyse/tests/fixtures/constant-initializers/`:

```sh
php /home/user/workspace/repo/.oracle/phpstan/vendor/bin/phpstan analyse \
	-c phpstan.neon --no-progress --error-format=json |
	jq '.files |= with_entries(.key |= split("/")[-1])' > expected-phpstan.json
```

The `global.neon`, `cycles.neon`, and `autoload.neon` captures use the same command with their
matching `expected-*.json` names. The autoload oracle explicitly scans the selected declaration;
Pun's public control discovers that file through Composer instead. Array key and unpacking code,
providers, native state, conversion, mappings, and gap lists are unchanged.

Verification matches all 29 fresh oracle diagnostics, including 28 full-type dumps. The combined
public regressions pass 73 tests, and the exported `Constants/` and `Arrays/` rule filters pass
without changing saved expectations. Fresh LLVM profiles cover all 165 changed executable lines
in `codebase.rs`, measured against `9bc30d06` without exclusions.

## Closure::fromCallable provider backport

Pun's `rules/return_types.rs` registers the return provider from Mago 1.59.0 commit
[`3ea59634580e5986c267d99e7fb3ccbf6ee26587`](https://github.com/carthage-software/mago/commit/3ea59634580e5986c267d99e7fb3ccbf6ee26587).
The provider uses the callable cast and parameter-dependent signature APIs already present in 1.57.0.
It resolves positional or named `callback`, clones each resolved signature, and changes only
`is_closure`. An unresolved alternative leaves the whole call on its declared Closure fallback.
No vendored analyzer source or dependency version changes for this backport.

`crates/pun-analyse/tests/closure_from_callable.rs` checks public analyser output against PHPStan 2.3.1. Its fixtures
cover immediate and assigned closures, invocation, strings, arrays, invokable objects, typed callbacks,
named arguments, optional and variadic parameters, references, generic and conditional targets,
inheritance, visibility, staticness, and unresolved alternatives. The two `phpstan*.neon` files
under `crates/pun-analyse/tests/fixtures/closure-from-callable/` describe the oracle runs at levels zero and five.

A wider dump probe also exposes an existing representation boundary: Mago's unknown signature prints
`Closure(mixed ...): mixed`, whereas PHPStan prints `Closure(): mixed`. For a union with a completely
unresolved ordinary string, the upstream provider also drops known signature alternatives by falling
back for the whole call. PHPStan retains those alternatives. The unknown-callback contract tests check
mixed invocation results and the absence of spurious argument errors, not exact dumps of these
unresolved signature shapes. No expected message is rewritten to Mago's notation.
The shared renderer also prints static closures as `Closure(...)` rather than PHPStan's
`static-Closure(...)`. Binding controls check nullability without claiming to fix that spelling.

The existing raw callback, observer, native-state, and integer-overflow patches remain necessary.
Native method results remain unavailable. This backport does not implement NoDiscard or ordered shapes.

## PHPStan variable scope transfers

`context/variable_scope.rs` ports variable-holder certainty and conditional holders alongside the
running expression types. Its references are PHPStan 2.3.1's `ScopeOps::mergeVariableHolders()`,
`createConditionalExpressions()` and `matchConditionalExpressions()` at the revision above.
The state clones with `BlockContext`; it does not share mutable branch state or perform another
syntax walk to reconstruct control flow.

Continuing `if`, switch and loop edges join actual incoming scopes. Disjoint guard values can retain
both a variable's presence and its explicit absence in the opposite branch. Assignments invalidate
dependent holders. Include/extract and standalone annotations update certainty at their transfer
points. Builtin input-reference parameters do not create variables, while output references still do.
Try/catch snapshots follow throw points even when exception diagnostics are disabled. Impossible
branches still publish diagnostic observations, but their break, continue and throw edges do not
contribute to the enclosing flow.

Unreachable `else` bodies reconcile the negated conditions before publishing observations. An exhaustive
`int|string` split therefore passes `never` to an `assertNever()` call, while an incomplete split retains
the uncovered type. Only actual assignment statements defer their target's `@var` handling to the
assignment analyzer. A standalone annotated global read receives its type before observation, without
creating an absent function-local variable. The original four-mode variable expectations remain unchanged.

The changes span `artifacts.rs`, block/if/loop contexts, assignment and variable analysis, function
calls and invocation post-processing, include/throw, reconciliation, statement control flow and
annotation handling. Pun's `rules/variables.rs` consumes the live observations for DefinedVariableRule
and CompactVariablesRule. Missing observations remain unknown, never definitely absent.
Overlapping guard-type subtraction remains conservative.

Public `variables` tests compare the four-configuration matrix, flow fixture and transfer controls
with fresh PHPStan diagnostics. The exported Variables fixture group covers both rules. The inline
`convergence` tests also check the incoming-edge joins. Integration retains native locals and paired
artifacts, and keeps the definedness hook observational so speculative callback gathers cannot publish
its events. No dependency pins or native-dependent rule activation change.

## New-object template defaults

`expression/instantiation.rs` and `utils/template.rs` backport the analyzer changes from Mago
[`2066b186`](https://github.com/carthage-software/mago/commit/2066b186d2a7cea821e0d4923934032e5f4e32c5),
included in 1.59.0. Dependencies stay at 1.57.0. Declared defaults replace unresolved placeholders after
constructor inference and scalar widening, so a literal default remains literal. Resolution follows
declaration order and substitutes earlier parameters into later defaults. Inherited-constructor lookup
now distinguishes absent inference from inferred `mixed`. Its class-member caller keeps its existing
mixed fallback. Neither expression dispatch nor native tracking changes.

`tests/generic_defaults.rs` in `pun-analyse` checks PHPStan 2.3.1's exact dumps and argument diagnostics,
including empty constructors, narrower defaults, dependent properties, inherited constructors, self
constants, and raw callback returns. The inferred-mixed case reads the public raw type because Pun's
existing display drops implicit mixed from `list<mixed>`. Public Analyzer checks retain one observer
visit per expression and equal full artifacts with native tracking enabled and disabled. The no-default
controls preserve Mago's placeholders and SPL's `never`, not PHPStan's different bound defaults.

The backport does not resolve the unspecified-template property gap. PHPStan can retain symbolic
dependent templates in constructor-bearing object dumps while resolving their properties; Mago
substitutes the arguments eagerly. Literal constructor inference also keeps Mago's existing widening.
The separate maintained [codex patch](../mago-codex/PATCHES.md) resolves `parent::VALUE` and
`parent::class` defaults through the declaring class's direct parent, using the same upstream commit.

Fresh coverage against `00a825d6` measures **30/30 changed analyzer executable lines** and
**169/169 test executable lines**, without exclusions. The instrumented private-target run passes
all 36 tests in `generic_defaults`, `callback_gather`, `debug`, `native_lane`, and `instantiation`.

## Array assignment certainty

`src/expression/assignment/array_assignment.rs::analyze()` invalidates certainty at the final root
transfer. Invalidating the root also removes stale absence and conditional holders for its descendant
paths. The written root loses its possibly-undefined flags. Other locals retain their correlations.
This happens after every index is read and after the fallible property update, not during the
intermediate array-type calculations.

Without this transfer, `$a[$t] = []` can retain the absence established by `!isset($a[$t])`.
Input-reference handling then skips `array_unshift()`'s mutation. The existing level-4
`Arrays/DeadForeachRuleTest::testBug2457` consequently reports a false `foreach.emptyArray` at line 24.
PHPStan 2.3.1 reports no errors. No loop or reporting-rule changes are needed.

The public `array_assignment_certainty` tests add precise non-loop mutation checks for assigned
dynamic and nested paths. Both dumps change from `list<int>` to PHPStan's `non-empty-list<int>`.
The controls retain unrelated correlated absence, forget a mutated discriminator, and report an
undefined index. A public Analyzer observer also checks that a root used as its own nested index
remains absent during the index read and becomes defined only after the write.

Capture the oracle from `crates/pun-analyse/tests/fixtures/array-assignment-certainty`:

```sh
php /home/user/workspace/repo/.oracle/phpstan/vendor/bin/phpstan analyse \
	-c phpstan.neon --no-progress --error-format=json |
	jq '.files |= with_entries(.key |= split("/")[-1])' > expected-phpstan.json
```

Fresh LLVM profiles from the focused tests and their array, variable, callback, dead-code, and native
controls cover **4/4 changed production executable lines**. No exclusions or gap entries change.

## Assignment certainty and null-coalesce scopes

`expression/assignment/mod.rs` clears the two possibly-undefined flags on a local's assigned type.
The source expression keeps its flags. PHPStan's
[`AssignHandler`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/ExprHandler/AssignHandler.php#L1361-L1371)
assigns a direct local with certainty `yes`, independently of the source value's availability.
This prevents an optional array offset or a coalesced property from making its assigned local optional.

`expression/binary/null_coalesce.rs` joins the isset-success scope with the RHS scope through the shared
variable-scope operations. It narrows the existing root local without evaluating the LHS twice.
An absent root stays absent. A `never` RHS contributes no continuing scope, and the surviving LHS is
non-null rather than truthy. PHPStan's
[`CoalesceHandler`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/ExprHandler/CoalesceHandler.php#L88-L105)
uses the same join, including RHS variable effects when the LHS is statically known.
The expression's result-type selection and the skipped RHS's exception isolation stay unchanged.
The passive native lane gains no transfers.

`crates/pun-analyse/tests/coalesce_scope.rs` compares public diagnostics and type dumps with PHPStan 2.3.1.
The fixture covers copied offsets, optional creation and reassignment, known-null and known-set operands,
an absent LHS, self-assignment, and a terminating RHS. Regenerate its oracle with
`scripts/export-analyse-expected.py crates/pun-analyse/tests/fixtures/coalesce-scope`.
The unchanged level-1 and level-4 Variables fixtures also cover bug-10577, pr-4372, bug-14459, and
unnecessary-null-coalesce-side-effects. No existing fixture expectation or gap is changed by this patch.

## Template array-offset refinement

`src/reconciler/mod.rs::adjust_array_type` refines the live constraints of template parameters through
the same offset transfer as ordinary arrays. It preserves surviving union members and propagates the
whole refined inner union into its parent once. Same-identity template offsets intersect their live
constraints explicitly because the general template comparator identifies parameters by name and scope.

The PHPStan 2.3.1 references are `MutatingScope::specifyExpressionTypeInPlace`,
`TypeCombinator::intersect`, `TemplateUnionType::filterTypes`, and
`ConstantArrayType::setOffsetValueType`. PHPStan combines compatible open shapes before refinement.
The patch reuses Mago's shared combiner on the current constraint. A declared shape template's
constant-array constructor omits open extras, and its offset builder produces a plain shape;
other template forms retain their name, defining entity, and intersections. Declaration metadata
establishes only the shape classification. Its original values never replace the refined constraint.
This distinction prevents a general array with newly known offsets from becoming a sealed shape.
An alternative that already satisfies the offset constraint retains its template identity, matching
`TypeCombinator::intersect`'s supertype check before the constant-array builder.

Public oracle fixtures live in `tests/fixtures/renamed-named-argument/` under `pun-analyse`.
`template_offset.php` reproduces the missing named-argument warning after an offset assertion.
`template_offset_controls.php` checks open and closed alternatives, branch joins, reassignment,
unrelated offsets, nested templates, tuple and list constraints, class templates, and general arrays.
Run its `oracle.php` generator with the pinned PHPStan 2.3.1 PHAR, then run
`cargo llvm-cov --dep-coverage mago-analyzer --no-report -p punphp-analyse --test renamed_named_argument`.

`reconcile_keyed_types` also invokes the parent transfer for an unchanged, defined offset when every
live parent alternative proves that the offset is required. Cached or fallback value types alone do
not prove existence. Unreachable values, undefined values, negative existence assertions, and empty
checks cannot take this path. The existing changed-variable, certainty, and descendant-invalidation
gates remain unchanged, as do native state, invocation, and plugin dispatch.
`template_offset_certainty.php` checks absent and optional offsets, optional siblings, unreachable
conditions, closed union alternatives, required list offsets, and string offsets against PHPStan.

## Resolved argument observations

Expression hooks receive `before_arguments()` after invocation resolution and before argument analysis.
Pun's variable observer uses the resolved parameter metadata instead of resolving a receiver before its
expression has run. This matches PHPStan's MethodCallHandler and ArgumentsHandler ordering and removes
the observer's duplicate function/method lookup. Computed, chained and nullsafe receivers now use the
same output-reference contract as local receivers. Non-nullable native input references still report
undefined reads; PHPStan's built-in input/output classification remains authoritative.

The dispatcher applies the existing observer exclusion during raw callback gathering. It does not call
expression hooks twice, perform a second expression walk, or introduce a native-type transfer.
`pun-analyse/tests/variables.rs` checks computed reference receivers against four fresh PHPStan 2.3.1
configuration captures, including ordinary input-reference controls.
