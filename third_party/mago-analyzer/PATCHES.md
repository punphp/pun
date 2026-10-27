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
unions except the literal guards below, and non-numeric loose comparisons. Short-circuit operators,
coalesce, exponentiation, bitwise operators,
increments, decrements, and compound assignments remain unsupported. These cases return no native
value and invalidate the live native scope. They never fall back to the full value or guessed `mixed`.
Explicit scalar casts have the separate bounded contract below.

### Deliberate availability limits

Unsupported syntax invalidates native availability before its children run. A supported operator with
unsupported operand types invalidates availability when its native transfer consumes those operands.
Completed operand artifacts remain valid records of their earlier evaluations. Subsequent assignments
do not recover that scope. This scope-wide rule avoids claiming that an unsupported operation left
aliases or locals unchanged. Each separately analyzed named function can initialize its own native scope.

Loops, exceptional control flow, references, arrays, members, class scopes, methods,
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

These choices leave general condition filtering and joining, reference effects, member state, callback scope
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

### Scalar conditional state

The conditional slice supports `if`, `elseif`, `else`, and ternaries with direct scalar guards,
negation, strict literal comparisons, and ordered integer comparisons. It uses the existing body walk.
`utils/conditional.rs` supplies native true-branch state after that walk visits the condition.
`statement/if.rs` carries false-branch state through the ordered `elseif` chain and records native
snapshots beside the existing continuing variable scopes. Bare and scalar-valued returns do not
contribute a continuing snapshot. Missing variables on one edge retain their when-defined type.
An unsupported contributing edge makes the joined native state unavailable.

Full PHPDoc-aware types still choose which `if` edges continue, including pruned branches that the
full walk visits for diagnostics. Those visits retain their native observations. A body skipped by
the full reconciler does not acquire synthetic observations or a guessed native continuation.
The supported policy is PHPStan's default `treatPhpDocTypesAsCertain=true`.

Ternaries have two distinct rules. Their native result selects values by native condition truthiness.
Their continuation selects one edge only for a full literal boolean, not for a truthy integer or string.
For native `false` with PHPDoc `true`, `flag ? (value = 1) : (value = 2)` therefore has native result
`2` but continuing native `value` equal to `1`. `expression/conditional.rs` publishes the native result
after the full result setter, which invalidates any previous native artifact at that span.

`native.rs` filters the bounded scalar maps without changing full locals or assertions. Strict integer
and string comparisons preserve PHPStan's decided-comparison behavior: an equal edge intersects with
the literal, while a full-decided unequal edge becomes `never` or leaves native state unchanged.
Ordered integer guards intersect ranges using checked boundaries. Native joins normalize the complete
truthy and falsy string partition without changing Mago's full-type combiner.

The authority is PHPStan 2.3.1 at `cb3b7e99489b2e3789ee2eec10501cb7a72580ea`:
`Analyser/StmtHandler/IfHandler.php`, `Analyser/ExprHandler/TernaryHandler.php`,
`Analyser/ExprHandler/Helper/IdenticalNarrowingHelper.php`, `MutatingScope::applySpecifiedTypes()`,
and `ScopeOps::finishMerge()`. The `native-lane/conditions.php`, `condition-guards.php`,
`condition-edges.php`, `condition-boundaries.php`, and `condition-wrappers.php` fixtures have fresh
`dumpNativeType` captures.
Public Analyzer tests compare artifact and observer queries against those captures. They also compare
full artifacts, diagnostics, observer visits, and return-provider counts with native tracking disabled.
Pure wrappers retain any repeated operand visits already made by the full conditional analyzer.

Mixed truthiness, floating-point guard subtraction, additional numeric or casing refinements on native
string guards, effectful wrapped conditions, short-circuit conditions, general arithmetic conditions,
loops, arrays, objects, and exceptional effects remain unsupported. Existing supported named-call
transfers are unchanged. No second walk, provider call,
observer call, native-dependent rule activation, or coverage exclusion is added.

On unchanged native source `632c633a`, the first public regression fails at `conditions.php:6`:
the true branch reports no native type instead of PHPStan's `true`. The five new fixtures retain
**178 actual PHPStan native records**. The original scalar, operator, and cast captures are unchanged.
For each fixture, the oracle replaces only `expression; // native` with
`\PHPStan\dumpNativeType(expression);`, preserving source lines. From the repository root, its command is:

```sh
timeout 900 nice php .oracle/phpstan/vendor/phpstan/phpstan/phpstan.phar analyse \
	--configuration=/tmp/pun-native-conditional-research/phpstan.neon --level=3 \
	--error-format=json --no-progress --memory-limit=1G --debug \
	/tmp/pun-native-conditions-evidence/conditions.php
```

The disposable configuration sets `phpVersion: 80400` and a scratch `tmpDir` only.
The final JSON line supplies unmodified `phpstan.dumpNativeType` records. Exit 1 is expected.
The other four fixtures use the same command with their respective scratch filenames.

The clean-profile run passes **60 focused public tests**: native lane 16, callback gather 10,
array slice 6, debug 10, variables 4, dead code 4, mixed subtraction 9, and integer-division overflow 1.
Scoped rule sweeps pass Operators (12 cases) and Variables (51 cases). Scoped level sweeps pass
Comparison (404 cases), DeadCode (114 cases), and Variables (94 cases), retaining their existing gaps.
These sweeps are regression checks, not complete PHPStan parity.

The instrumented native command above also selects `--test variables --test dead_code
--test mixed_subtraction`. The separate arithmetic command selects
`--test coverage -- literal_integer_division_overflow_matches_php_and_phpstan`.
Rule and level runs select `--test rules` or `--test levels`, with `PUN_RULES_FILTER` or
`PUN_LEVELS_FILTER` set to the named directory and a trailing slash. All use the private target
`/home/user/workspace/target-native-conditions`, one Cargo job, two Rayon threads, and
`cargo llvm-cov --dep-coverage mago-analyzer --no-report` under `timeout 900 nice`.

Fresh LCOV export from those executables covers **397/397 changed analyzer executable lines**
against `632c633a`: native transfers 356, if statements 22, ternaries 13, conditional utility 5,
and if-scope initialization 1. Changed public test executable lines are **61/61**.
The measurement uses fresh profiles and unfiltered LLVM records, with no exclusions or private-helper
tests. This is changed-line coverage, not branch coverage or whole-analyzer coverage.

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

## Coalesce RHS narrowing and failed variable reads

`expression/binary/null_coalesce.rs` reconciles the already-evaluated LHS into the inverse-isset scope
before evaluating the RHS. PHPStan's `CoalesceHandler` skips this certainty reduction for a surely-set
plain `??` operand. Its `AssignOpHandler` still applies the counterfactual reduction for `??=`.
Mago's synthetic binary preserves the assignment operator's three-byte span, distinguishing it from
the two-byte plain operator without a second expression walk or a context-wide assignment flag.

`expression/variable.rs` returns the shared codex error value for an ordinary read when the live scope
knows the variable is absent. PHPStan's `VariableHandler::createTypeCallback` returns `ErrorType` in
that case. Isset and unset probes retain their existing handling, and reference creation still runs
before the read result is selected. Pun converts the marked mixed value to its existing `Type::Error`.
This preserves the error through self-coalescing assignment instead of replacing it with `mixed` or
the LHS's earlier concrete type. The passive native lane gains no new transfer.
Pun's level helper still treats this error as implicit mixed when level 10 checks it, matching
`RuleLevelHelper::findTypeToCheckImplementation()` before its mixed/error fallback.

The existing `coalesce_scope` oracle fixture includes nullable RHS reads, known-set plain operators,
counterfactual assignment operators, optional self-assignment, and absent self-assignment. Its saved
diagnostics come from PHPStan 2.3.1 through `scripts/export-analyse-expected.py`.

## Named variadic reference arguments

`invocation/analyzer.rs::find_named_parameter_offset` binds an unmatched named argument to the final
variadic parameter. PHPStan's `ArgumentsNormalizer::reorderArgs()` and `ArgumentsHandler::processArgs()`
use that parameter for both type checking and reference writes. Retaining only the argument's allowed-read
observation would hide the initial undefined read but leave subsequent reads incorrectly undefined.

Pun's resolved argument observer also reads callable-signature parameters. Closure parameters have no
separate native reflection in PHPStan; typed closure and callable output references can create variables,
while a named function's non-nullable native reference remains an input. The public `variables` fixture
checks positional, named, variadic, arrow, and typed-callable outputs alongside ordinary value and native
input-reference controls. Expectations come from PHPStan 2.3.1.
## Generic array keys after unset

`src/statement/unset.rs:130–150,322–335,360,368–395` ports generic key subtraction from the
same PHPStan 2.3.1 revision. The references are `ArrayType::unsetOffset()` at lines 519–537,
`TypeCombinator::doRemove()`, and `TemplateTypeTrait::tryRemove()` at lines 399–423.

A single literal integer or string offset removes that value from a generic array's key type.
The existing strict literal reconciler supplies integer range splitting, finite string removal,
and the non-empty-string refinement for removal of the empty string. No second scalar evaluator
or rule-local type reconstruction is added. Empty key domains become empty constant arrays.
Unknown offsets and unions of offsets do not subtract a particular key.

Template keys keep their name, defining scope, intersection metadata, and union flags while their
bounds narrow recursively. A compatible literal offset on a generic template array returns an
ordinary array, matching PHPStan's `new self` in `ArrayType`. Disjoint and unknown offsets keep
the outer template identity. Template arrays with additional intersections or known-item
refinements do not enter this new projection path.

An open shape retains its empty known-item map after its last explicit key is removed. This
prevents a later unset from mistaking the remaining open shape for a generic array and subtracting
its extra keys. The public repeated-unset control distinguishes these two representations.

### Generic unset verification

Public fixtures `unset/generic-keys.php` and `unset/generic-key-identities.php` use PHPStan's
actual JSON diagnostics at level 0 and PHP version 80400. Their tests compare every message,
identifier, line, tip, and ignorable flag. No diagnostic is filtered or suppressed.

Before this patch, `unset/template-arrays.php:38` reports `array<int, int>` instead of
`array<int<min, -1>|int<1, max>, int>`. After the patch, that fixture passes in full.
All 18 oracle diagnostics in `generic-key-identities.php` match. Those controls include finite
template exhaustion, unchanged template arguments, scalar value templates, numeric-string key
normalization, nonnumeric string keys, and repeated unsets on an open shape.

The combined public run passes 34 tests and retains two failing tests. Array metadata (1),
array slicing (5), callable origins (2), debug types (10), and duplicate keys (9) all pass.
The unset suite passes 7 of 9 tests. The exported Arrays harness passes. The exported UnsetRule
harness matches 9 of 10 cases and retains the existing nested-invalid-offset failure.

Fresh changed executable-line coverage against `712ce49a` is **51/51**. The six public test
executables above provide the profiles. No coverage exclusion or private-helper test is used.
The instrumented command is:

```sh
CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=2 CARGO_TARGET_DIR=target-unset-rule \
	CARGO_LLVM_COV_TARGET_DIR="$PWD/target-unset-rule/coverage" \
	timeout 900 nice cargo llvm-cov test --dep-coverage mago-analyzer \
	-p punphp-analyse --test unset --test array_metadata --test array_slice \
	--test duplicate_array_keys --test debug --test callable_origins --no-report
```

### Retained unset boundaries

The failing tests keep these oracle expectations active:

- `generic-keys.php:98–99`: mixed and benevolent array keys require `mixed~0`. Mago's mixed
  representation cannot retain an excluded literal.
- `generic-keys.php:108`: the public converter renders an empty open shape as a generic array.
- `generic-keys.php:116`: an `isset`-refined generic array needs key subtraction together with
  `hasOffsetValue(5, string)`. Mago's known-item representation also describes open constant
  shapes, whose extra keys must not be subtracted. This patch does not guess which source produced it.
- `generic-keys.php:137–138`: PHPStan seals a template constant-array mutation because the
  `TemplateConstantArrayType` constructor omits its bound's unsealed metadata from the parent.
  The current template-bound representation does not preserve that distinction.

Generic non-key offsets still need shared PHP scalar-to-key conversion. This patch does not add
an engine-wide error type, widen incoming declarations, change a gap list, or activate a rule mapping.

## Virtual offset-unset write-back

`src/statement/unset.rs:89–97,125,309–314,326–501` ports the virtual assignment from PHPStan
2.3.1, revision `cb3b7e99489b2e3789ee2eec10501cb7a72580ea`. Its sources are
`Analyser/StmtHandler/UnsetHandler.php:110–151`, `Analyser/ExprHandler/AssignHandler.php:1803–1850`,
`Type/ArrayType.php:460–517`, `NullType::setExistingOffsetValueType()`, and
`ConstantArrayTypeBuilder::setOffsetValueType()`.

The unset receiver comes from its stored expression result, not just a cached local. A scalar
receiver becomes `get_error()`. The write-back walks already-evaluated containing offsets from
the inside out and updates their mutable root. It never evaluates an index again, dispatches an
expression observer, or replaces an incoming expression artifact. Each union keeps its source flags.
Null remains null, including a null containing receiver.

Finite union keys join single-key write alternatives instead of overwriting every key strongly.
`src/expression/assignment/mod.rs:64` exposes the existing array updater within the crate for this
transfer. No second expression evaluator is added. Root writes invalidate descendants and variable
certainty through the existing `BlockContext` APIs, then record the virtual assignment.

Generic containing arrays use `ArrayType::setExistingOffsetValueType()` rather than ordinary assignment.
They keep their key type, listness, and existing non-empty guarantee. A write does not assert that an
offset exists. Written shapes update their present keys in each original shape, retaining required keys.
Optional writes combine their values without making an existing required key optional.
The constant-array extractor recognizes sealed keyed arrays, fixed lists, and template bounds.
Shape alternatives stay separate inside Mago so its array combiner does not discard their non-empty
guarantee. Pun's PHPStan type combinator performs the final union reduction.

This patch requires the maintained codex error marker and Pun's conversion of `TMixed::is_error()`
to `Type::Error`. Validation imports the codex parent-default and error-marker commits as dependencies.
A disposable workspace copies only the converter's error-marker clause into the older Pun source.
Neither the validation dependencies nor that converter change belong to this patch.

### Virtual unset verification

The unchanged analyzer leaves the scalar root as `3`, preserves invalid nested leaves as `1`,
fails to propagate valid nested removals, and emits the extra `unset.php:18` diagnostic.
The patch makes all ten exported `Variables/UnsetRuleTest` cases pass, including that regression.
Public oracle controls also prove copy isolation, sibling preservation, reassignment, cached-offset
invalidation, one index increment, weak union writes, nullable parents, and property roots.

`unset/write-back.json` contains all 63 diagnostics from the actual PHPStan 2.3.1 oracle, at
level 0 and PHP version 80400. The public test retains every diagnostic and its flags.
It currently matches 55 of 63 diagnostics and fails on the boundaries listed below.
No gap, ignore, expectation filter, or rule-local suppression is added.

`unset/generic-parents.json` records 12 fresh oracle diagnostics, all matching the public analyzer.
Its asymmetric controls cover preserved required child keys, scalar errors, empty written shapes,
list parents, optional union writes, fixed-list values, template bounds, and copies.
The ordinary assignment updater previously made generic parents non-empty and optionalized a required
surviving child key. The dedicated existing-offset transfer fixes both differences.

The combined public run passes 37 tests and retains three failing tests. Array metadata (1),
array slicing (5), callable origins (2), debug types (10), duplicate keys (9), and the scoped
UnsetRule harness (1) pass. The unset suite passes 9 of 12 tests. Exported Arrays, DefinedVariable,
CompactVariables, ReadOnlyProperty, and OffsetAccessWithoutDimForReading fixtures also pass.
The existing generic-key oracle failures remain unchanged.

The third retained public failure checks the existing `Properties/data/bug-13856.php` fixture.
A fresh PHPStan run confirms its two exported diagnostics, including `unset.offset` at line 73 on
`ArrayObject<*NEVER*, *NEVER*>`. The scoped MissingReadOnlyPropertyAssignRule harness reproduces
only that missing diagnostic. Shared constructor, property, and offset inference is a separate patch.

Fresh changed executable-line coverage is **162/162** for this write-back patch and **213/213**
for generic key subtraction plus write-back against `712ce49a`. No exclusion is used.
The visibility-only module change adds no executable line. The measurement uses fresh profiles
from these public tests and exports LCOV from their seven current executables:

```sh
CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=2 CARGO_TARGET_DIR=target-unset-rule \
	CARGO_LLVM_COV_TARGET_DIR="$PWD/target-unset-rule/coverage" \
	PUN_RULES_FILTER=Variables/UnsetRuleTest \
	timeout 900 nice cargo llvm-cov test --dep-coverage mago-analyzer \
	-p punphp-analyse --test unset --test array_metadata --test array_slice \
	--test duplicate_array_keys --test debug --test callable_origins --test rules \
	--no-report --no-fail-fast -- --test-threads=2
```

Validation uses the disposable workspace's manifest for the converter clause described above.
Profiles are merged with `llvm-profdata merge -sparse`. `llvm-cov export -format=lcov` reads
those profiles and the exact executables from that run, without older binaries or profiles.

### Remaining write-back boundaries

- `write-back.php:81–83`: the weak-write values and optionality match, but shape key order does not.
  Mago stores keyed items in a `BTreeMap`. Pun restores source order only for identical key sets,
  so mutated union shapes lose their original order.
- `write-back.php:98`: PHPStan's UnsetRule reads the statement-entry scope for both targets.
  Pun observes the second target after the first virtual mutation and misses its original scalar error.
  The rule needs an entry-scope type-read contract. Deferring mutation or evaluating targets again
  would break the stored-result transfer.
- `write-back.php:127,165,185,194`: the shared offset and variable readers still supply synthetic
  null for missing accesses, absent variables, and the null arm of nullable offset reads.
  PHPStan supplies ErrorType and discards error-valued arms when another offset-read arm succeeds.
  The write-back preserves the received types instead of guessing whether a null is synthetic.
  The independent real-null controls prevent replacing every null with ErrorType.

Unsealed constant-array-valued generic parents remain outside the sealed-shape extraction contract.
The existing `generic-keys.php:98–99,108,116,137–138` failures retain their original oracle expectations.
These boundaries and the retained generic-key failures still prevent whole-rule parity.
This patch does not activate a rule mapping or change the shared readers, converter, or observation API.

## Empty-array template inference

`invocation/template_inference.rs` retains empty input arrays for generic container parameters.
PHPStan 2.3.1's `ArrayType::inferTemplateTypes()` infers their iterable key and value as `never`.
Constant shapes still infer only from matching keys, so an absent optional key does not supply a
template value. Pun overlays exported empty-array constructor defaults before invoking this path;
Mago's nullable API-stub default is not the runtime value for ArrayObject and ArrayIterator.
`generic_defaults/empty-arrays.json` retains the actual oracle's 15 diagnostics, including populated
controls, optional shapes, generic empty lists, readonly assignment, and invalid unset offsets.

## Template constant-array unset reconstruction

`src/statement/unset.rs:134–154,167,174,321,470,523–571` distinguishes a declared template
constant-array bound from a generic array with known offsets added by flow analysis.
The declaration comes from `TypeResolutionContext`, matched by template name and defining entity.
The transfer preserves the current inferred items and template identity. It does not replace the
current value with the declaration's bound.

The reference is PHPStan 2.3.1 at revision `cb3b7e99489b2e3789ee2eec10501cb7a72580ea`:

- `Type/Generic/TemplateConstantArrayType.php:23–66`: the parent constructor omits unsealed
  extras. `recreate()` builds the new bound from that sealed parent.
- `Type/Constant/ConstantArrayType.php:1375–1505`: a matching literal removes a key. Finite
  unions and general keys make matching items optional. A match still calls `recreate()` when
  the matching item was already optional. A disjoint offset returns the original type.

`unset_keyed()` reports whether the constant-array branch recreates its type, not whether the
new map differs by value. A template constant-array mutation discards unsealed parameters only
when that flag is true. Unchanged offsets preserve the original displayed bound.
Pun's existing `rules/unset.rs::has_offset()` helper checks a template constant array against
its sealed parent while retaining the unsealed bound in the diagnostic. Statement-entry
observations and the shared expression readers are unchanged.

`unset/template-open-shapes.json` records 20 diagnostics from the real oracle with PHP 8.4.
Its public test covers copies, unchanged literals and unions, required and already-optional keys,
general strings, class and method templates, ordinary open shapes, and an offset-refined generic
array that must not become a sealed template shape. No existing expectation changes.

Fresh instrumented public tests cover **38/38 changed executable lines** against this follow-up's
parent: 32 in the maintained transfer and 6 in Pun's unset offset helper. No exclusions or
private-helper tests contribute to that count. The combined run uses the seven-target command
above directly in this worktree, not the earlier disposable manifest.
It passes 38 tests and retains three failures: the remaining generic-key dumps, bug-13856, and the
write-back fixture's shared reader, statement-entry observation, and shape-order differences.
Separate exported Arrays, DefinedVariable, CompactVariables, and ReadOnlyProperty checks pass.
The exported UnsetRule cases match 10 of 10, and the generic-key diagnostics improve from 30 of 36
exact matches to 32 of 36.

Validation uses the exact `expression/variable.rs` and `types/convert.rs` error-reader versions
from main `d357ffbb`. Those validation-only changes are not part of this follow-up. On this base,
the write-back fixture still matches 55 of 63 oracle diagnostics, and bug-13856 still lacks its
line-73 diagnostic. All saved expectations remain active.

### Remaining representation boundaries

The original `generic-keys.php:137–138` diagnostics now match. Four dump mismatches remain:

- Lines 98–99 require `mixed~0`. Neither maintained `TMixed` nor Pun's `Type::Mixed` stores a
  subtracted literal. A shared representation and its operations are required.
- Line 108 retains an empty known-item map with unsealed parameters. The converter's
  `array()` shape guard requires a nonempty map, so it renders that value as a generic array.
- Line 116 requires key subtraction together with `hasOffsetValue(5, string)`. Pun's accessory
  model has no `HasOffsetValue`. Mago also uses the same keyed-array representation for declared
  open shapes and generic arrays refined by offsets. Those cases require distinct unset
  transfers. This patch does not guess from the map contents or restore a wider declaration.

Indirect template bounds and unsealed constant-array-valued generic parents remain outside this
direct keyed-bound correction. Whole-rule parity remains incomplete. The gap list and rule mapping
are unchanged.

## Failed-read arguments and string offsets

These corrections build on `1cc0e2fe`, which retains ErrorType for failed variable and offset reads
and discards failed union arms when another offset-read arm succeeds. Real null values remain null.
The reference is PHPStan 2.3.1 at `cb3b7e99489b2e3789ee2eec10501cb7a72580ea`:
`Type/ErrorType.php`, `Rules/FunctionCallParametersCheck.php`, `Type/StringType.php:70–84`, and
`Type/Constant/ConstantStringType.php::getOffsetValueType()`.

`src/invocation/arguments.rs::verify_argument_type()` returns before type verification for an ErrorType
input. It does not skip argument evaluation, binding, reference checks, or reference output writes,
which run separately. Genuine string, mixed, null, and false argument errors remain active.
This is commit `6aee6d16`.

`src/utils/expression/array.rs::handle_array_access_on_string()` returns ErrorType when the evaluated
index type cannot contain an integer. The predicate visits union members and template constraints.
It does not use argument compatibility: PHP accepts integers for float parameters, but a float index
has PHPStan `isInteger()` certainty No. Numeric strings are also definitely noninteger index types.
Integer-capable unions, mixed, scalar, numeric, and array-key types retain the existing read value.
During isset analysis, the helper uses the same absent-value placeholder as keyed-array reads so
coalesce takes the fallback branch. Index diagnostics still use the original evaluated index type.
No index is evaluated again. This is commit `80b661cd`.

### Failed-read verification and remaining boundaries

Public tests are `crates/pun-analyse/tests/failed_arguments.rs` and `failed_string_offsets.rs`.
Their fixtures and fresh PHPStan JSON outputs live in `tests/fixtures/failed-reads/` under that crate.
The argument fixture uses level 9; the string-offset fixture uses level 0. Both use PHP version 80400.

Before the argument correction, the maintained analyzer reports 12 argument-type errors where the
oracle reports six. The corrected verifier retains exactly those six genuine errors, plus independent
binding and reference diagnostics. The test also checks index evaluation and reference output types.
The initial string fixture reproduces 11 wrong results among 20 dumps. The expanded regression matches
all 29 oracle dumps, including invalid literal and general indexes, template bounds, copies, float
postincrement, valid and uncertain indexes, and coalesce. The copied string-offset compound-assignment
control has no secondary `assignOp.invalid` diagnostic.

Validation on `1cc0e2fe` plus these two corrections passes all 53 non-unset public checks in the focused
matrix, including callbacks, native state, variables, coalesce inputs and scopes, incoming isset,
array metadata, array slicing, assignment certainty, dead code, and the scoped exported
`Arrays/OffsetAccessAssignOpRuleTest` fixtures. This uses the harness on that base, not later main's
stricter combined level sweep. Loop-convergence tests pass 9/9.

The unchanged raw array-access tests pass 2/3. `negated_isset_narrows_parent_array_type` retains two
extra `MixedArgument` diagnostics from echo's separate
`common/construct.rs::verify_construct_input_type()`. That verifier is outside these corrections.
A fresh PHPStan level-9 control reports only the two primary offset diagnostics; PHPStan's
`Rules/Cast/EchoRule.php` skips ErrorType inputs.

The same base retains three public unset failures: `readonly_array_access_offset_keeps_constructor_arguments`,
`generic_key_subtraction_matches_phpstan`, and `virtual_unset_write_back_matches_phpstan`.
They concern constructor defaults, generic key metadata, shape order, and statement-entry observations.
The failed-read values at `write-back.php:127,165,185,194` are corrected by the shared-reader dependency.
No saved diagnostic, raw test expectation, or gap list changes in these follow-ups.

Fresh LLVM profiles cover **15/15 changed executable lines** across the two corrections:
`invocation/arguments.rs` 2/2 and `utils/expression/array.rs` 13/13. The measurement intersects the
added source lines with LCOV records from the focused matrix's current executables. It uses no
coverage exclusions. These corrections do not change the passive native-state implementation.

### Generic offsets retain their own array kind

The next correction closes `generic-keys.php:108` and `:116`. It uses the codex constant-array
marker instead of inferring source kind from the presence of known keys or restoring a declaration.
The template branch in `statement/unset.rs:134–162` now uses that marker too. A template generic
array refined by `isset()` becomes an ordinary array after compatible literal-key subtraction.
A template constant array still retains its identity when its bound is recreated.

`unset_keyed()` at `statement/unset.rs:516–546` separates the two transfers:

- `ArrayType::unsetOffset()` subtracts a compatible literal from the generic key domain.
- `HasOffsetValueType::unsetOffset()` removes an assertion only for its exact literal key.
  General and finite-union offsets retain that assertion.
- `NonEmptyArrayType::unsetOffset()` drops standalone non-emptiness. A surviving required offset
  establishes non-emptiness again.
- `ConstantArrayType::unsetOffset()` retains the existing shape transfer, including empty open shapes.

Pun converts required offsets of a generic array to `HasOffsetValue` accessories without replacing
the generic value type. This preserves a narrowed offset such as `string` inside an `int|string`
array. The accessory has value-only traversal and no PHPDoc node. Public type operations and the
Analyzer fixture use fresh PHPStan 2.3.1 expectations. Non-array offset-value comparisons remain
outside this array-origin accessory port. Invalid-offset readers, unset entry observations,
and default generic constructors are unchanged.

`subtract_key()` delegates a mixed key domain to `TMixed::subtract()`. This supplies the
`mixed~0` values required by `generic-keys.php:98–99` without changing the shared mixed operations.
PHPStan's `ArrayType` constructor normalizes the benevolent `array-key` alias to mixed. Mago's
`TScalar::ArrayKey` retains that source distinction, so the transfer applies the same normalization.
An explicit `int|string` union retains its two members and uses scalar subtraction instead.
Unsetting an offset on a mixed receiver delegates to `TMixed::unset_offset()`, which can remove an
excluded empty array. Copies retain their own exclusions, repeated literal unsets combine them,
and a runtime key does not subtract its entire type. The public runner exercises the converter's
excluded-set branch as well as the statement producer.

The repeated-scalar control in `unset/entry-reads.php:9` depends on the shared raw offset reader,
not the entry observer. `ConstantArrayType::getOffsetValueType()` converts a matching `ErrorType`
to ordinary mixed when another explicit key survives. A singleton array takes the all-keys path
and retains `ErrorType`. The virtual unset assignment consumes that stored receiver type.
Mago's raw known-offset reader currently returns the error value unchanged in both cases.
Normalizing only the displayed type cannot repair the value written back into its parent.

### Verification of generic offset metadata

Before this correction, the first 17 public controls matched only 4 oracle diagnostics. The final
fixture matches all 27 fresh PHPStan diagnostics. The original `generic-keys.php` matches all 36
unchanged diagnostics, including lines 98, 99, 108, and 116. The array-origin accessory oracle also
covers 13 types and 88 asymmetric supertype comparisons.

All 50 `punphp-phpstan` tests and 36 neighboring Analyzer tests pass. The exported UnsetRule cases
match 10 of 10. The neighboring Arrays harness matches all five duplicate-key cases and its
offset-access case. On this older validation base, the complete `unset` binary passes 12 tests
and retains two failures: the already-fixed constructor-default fixture and older write-back
entry, failed-read, and shape-order behavior. Those tests and expectations remain unchanged.

Fresh LLVM profiles from 22 exact current executables cover **106/106 changed production lines**:
64 in Pun, 21 in this analyzer transfer, and 21 in codex. No exclusions or private-helper tests
are used. An isolated public `generic_key_subtraction_matches_phpstan` run also hits the mixed
dependency's `convert.rs:407` excluded-set expression twice. That run uses no injection hook.

The instrumented runs use one Cargo job, two Rayon threads, a private target directory, and
`cargo llvm-cov test --dep-coverage mago-analyzer,mago-codex --no-report`. The Analyzer run includes
`unset`, `mixed_subtraction`, `array_metadata`, `array_slice`, `duplicate_array_keys`, `debug`,
`callable_origins`, `error_marker`, and `rules`, with `PUN_RULES_FILTER=Variables/UnsetRuleTest`.
The separate `punphp-phpstan` run includes every package test.

## Failed-read construct inputs

The follow-up to `30804564` changes only
`src/common/construct.rs::verify_construct_input_type()` in production. It returns before type
verification when the input is ErrorType, following PHPStan 2.3.1's `Rules/Cast/EchoRule.php` and
`PrintRule.php`, which skip ErrorType inputs. The failed read retains its primary diagnostic.
`analyze_construct_inputs()` still evaluates inputs, processes subsequent echo values, and checks
purity independently. Print still produces its ordinary result. No reader, native transfer, Pun
query, or unset behavior changes.

`crates/pun-analyse/tests/failed_constructs.rs` uses the fresh level-9, PHP-80400 oracle records in
`tests/fixtures/failed-reads/expected-constructs-phpstan.json`. It checks primary read diagnostics,
genuine invalid echo and print inputs, purity, two echo-argument increments, and print's result.
The fixture covers failed inputs to echo, print, eval, exit, and die. It also retains Mago's existing
null, false, and mixed exit errors, derived from its `int|string` input contract. Those three controls
are not PHPStan compatibility assertions: this PHPStan oracle does not check those exit argument types.
No saved expectation is replaced or suppressed.

Before this correction, the unchanged `negated_isset_narrows_parent_array_type` test fails on two extra
`MixedArgument` diagnostics. Afterward, all three raw array-access tests pass with their original
expected issues, and the nine convergence tests pass. The new public fixture loses exactly ten
failed-read type cascades while keeping its five genuine construct type errors, ten primary read
diagnostics, and independent purity diagnostic. Its dumps remain `2` for the incremented index and
`1` for print's result.

Validation on `30804564` plus this correction passes all 27 focused public tests, including the
11 native-on/off and 10 callback tests, the earlier failed-reader regressions, and both coalesce
regressions. The unrelated unset failures recorded above are not changed or reclassified here.
Fresh profiles cover **3/3 changed LLVM-mapped executable lines**: the condition, early return, and
fall-through closing brace. LCOV counts are 36, 11, and 25, respectively, using only this run's
current executables. No coverage exclusion is used.

## Constant-array error-valued offset reads

PHPStan 2.3.1's `ConstantArrayType::getOffsetValueType()` normalizes an error-valued verified
offset to mixed when another explicit key survives or the shape is unsealed. Its sealed
singleton instead returns the unchanged iterable value, retaining ErrorType. Apply that
distinction in the known-key read path of `utils/expression/array.rs`, only for constant
shapes. Generic-array reads retain their existing behavior.

The distinction affects inference, not just display: repeated invalid nested unsets read the
error-valued child before writing the virtual parent back. The existing public `entry-reads.php`
fixture checks multi-key normalization, singleton preservation and index evaluation; expectations
remain the actual PHPStan output. All 15 public unset tests now pass, including generic-key
metadata and nested write-back. The eight-target reader, metadata, slice and mixed-subtraction
matrix passes all 30 tests. Fresh dependency coverage covers all **6/6 changed executable lines**
without exclusions; the error-normalization assignment executes three times.

The legacy `run_mago()` spike test retains its original unmodified Mago CLI capture. The
maintained engine intentionally differs for ErrorType cascades, as the argument and construct
regressions already require. A fresh PHPStan 2.3.1 capture of the same spike input reports five
primary errors, including the undefined echo input, and no `echo.nonString` error. The spike
comparison now applies that oracle-backed exception only to Mago's secondary mixed-argument
echo warning; all primary Mago codes, lines and messages remain exact. No runtime compatibility
setting or supported CLI behavior changes.

## General array writes do not assert a literal arm

PHPStan 2.3.1's
[`ArrayType::setOffsetValueType()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/ArrayType.php#L362-L428)
adds `HasOffsetValueType` only for a single constant offset. A general string can contain a
literal arm in Mago's representation. For example, a guarded lowercase string can be stored as
`non-empty-lowercase-string|''`. Its empty-string arm is not a definitely written offset.

`expression/assignment/array_assignment.rs` now rejects partial literal extraction in both root
and nested writes. General writes also retain existing generic-array offset values, as
[`HasOffsetValueType::setOffsetValueType()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/Accessory/HasOffsetValueType.php#L182-L197)
requires. Constant shapes still widen overlapping entries. The converter is unchanged: a required
known item on a generic array still means `hasOffsetValue`.

## Bounded foreach chains retain array exit states

[`ForeachHandler::tryProcessUnrolledConstantArrayForeach()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/StmtHandler/ForeachHandler.php#L658-L841)
passes each constant entry's exit state into the next entry. Treating the entries as one union
instead makes an unset look like a union-key unset. PHPStan deliberately retains offset accessories
for that union operation, so changing `unset_keyed()` would break non-loop controls.

`statement/loop/foreach.rs` ports the sequential chain for array exits on one sealed list with
one to sixteen required entries. Direct value variables and optional direct key variables are
supported. Each iteration uses fresh artifacts, detached shared block state, and the existing
speculative gather's observer and diagnostic isolation. The normal loop walk still supplies
diagnostics, expression observations, and non-array locals.

Nested loops, references, statement hooks, destructured or indirect targets, optional entries,
shape unions, associative iterators, and bodies with loop exits retain the existing loop path.
The patch does not claim PHPStan's complete loop-unrolling contract or change generic unset rules.

Public regressions live in `crates/pun-analyse/tests/array_mutation_states.rs` and
`tests/fixtures/array-mutation-states/`. The original exported `testBug11146` and `testBug5298`
expectations are unchanged. Fresh PHPStan 2.3.1 captures check general and literal writes, unrelated
definite offsets, literal and union unsets, zero and multiple iterations, and replay bounds.

The fixture capture command, from its directory, is:

```sh
env -u AUGMENT_AGENT -u AMP_CURRENT_THREAD_ID -u AI_AGENT -u CURSOR_TRACE_ID -u CURSOR_AGENT \
	-u GEMINI_CLI -u CODEX_SANDBOX -u CODEX_THREAD_ID -u OPENCODE_CLIENT -u OPENCODE \
	-u CLAUDECODE -u CLAUDE_CODE -u REPL_ID \
	php ../../../../../.oracle/phpstan/vendor/bin/phpstan analyse -c phpstan.neon \
	--error-format=json --no-progress |
	jq '{files: (.files | with_entries(.key |= split("/")[-1]))}' > expected-phpstan.json
```

Validation on `7a973a30` plus this patch passes all 77 focused public tests, including the 6 new
tests and all 34 freshly captured diagnostics. The complete level-4 number-comparison file passes
its 37 cases. The level-3 property-assignment file has zero new failures across 90 cases, but its
harness fails because two unrelated passing cases remain in that base's gap list: `testBug3383`
and `testIntegerRangesAndConstants`. The patch does not change the shared gap list.

Fresh profiles from the current executables cover **73/73 changed executable lines**, without
exclusions: 9 in array assignment and 64 in foreach. The generic-array accessory guard and the
constant-shape widening control both execute. The dependency is instrumented explicitly with
`cargo llvm-cov --dep-coverage mago-analyzer --no-report`; an unfiltered LCOV report retains the
vendored source paths.

## Negative guards retain mixed exclusions

`simple_negated_assertion_reconciler.rs` uses the shared `TMixed::subtract()` for negative
scalar, array, object, named-object, and null type assertions on mixed values. PHPStan 2.3.1's
`MixedType::subtract()` unions repeated exclusions. Its `ErrorType::subtract()` retains error
identity. The maintained API provides both contracts. The outer union retains its existing flags.
Falsey mixed values also retain their finite positive domain through `get_empty()` and the shared
intersection API. A negative guard must not widen a falsey value into an unrestricted complement.

`negated_assertion_reconciler.rs` routes mixed literal comparisons through the same producer.
Strict inequalities retain their constant exclusions. Loose comparisons do not claim strict
type exclusions. Non-mixed literals keep the existing specialized handler. Predicate evaluation,
reference processing, certainty, native dispatch, and Pun's conversion layer are unchanged.

The public `mixed_subtraction` fixture `guards.php` contains the actual PHPStan 2.3.1 records for
negative predicates, copies, repeated guards, short-circuit conditions, terminating branches,
joins, reassignment, strict literals, null, truthiness, and failed reads. It also checks that a
partial offset-domain exclusion permits unset while the complete excluded domain rejects it.
Its Analyzer control compares all full expression types and issues with native tracking on and
off, without an injection hook.

The exact public oracle comparison retains three unresolved array descriptions at lines 30,
39, and 41. PHPStan's `IsArrayFunctionTypeSpecifyingExtension` creates explicit mixed key and
value parameters, producing `array<mixed, mixed>`. Mago's assertion uses placeholders, and Pun's
current converter produces implicit mixed parameters and describes the exclusion as `array`.
Those saved oracle records remain active. No converter, representation, mapping, or gap changes
are part of this patch.

One further active record at line 153 exposes the positive guard's falsey join: its `is_int`
alternative contributes int instead of 0. `simple_assertion_reconciler.rs` still replaces mixed
with the asserted type without intersecting its positive falsey constraint. That producer is
outside this negative-assertion patch.

The mutation control at lines 168 and 169 exposes the shared API's array subtraction boundary.
After unset, PHPStan removes the empty array from the excluded array domain, leaving only
non-empty arrays excluded. Codex's `mixed/subtraction.rs::remove()` currently handles integer
partial differences but retains the complete array exclusion. The subsequent ArrayAccess guard
therefore produces an extra unset error. The oracle record and negative control remain active.
This shared API correction is outside the analyzer-producer ownership of this patch.

This producer does not add iterable, callable, structural-accessory, or generic-constraint
complements. Loose coercive comparisons also need separate transfers. A fresh PHPStan probe of
`$value != 0` requires `mixed~(0|0.0|'0'|false|null)`, not just `mixed~0`. The unchanged resource
predicate control returns mixed, matching PHPStan.

## Falsey guard intersections and explicit array assertions

The follow-up closes the three producer and transfer failures recorded above. The original
`guards.php` regression now matches all 36 PHPStan 2.3.1 records, without changed expectations.

`simple_assertion_reconciler.rs` intersects a falsey mixed value with the asserted type instead
of replacing that value with the entire asserted domain. The intersection uses the finite
falsey domain and strict assertion containment. In particular, integer zero does not satisfy
`is_float()`. Subsequent positive float and bool guards retain their surviving literals.

`assertion.rs` supplies explicit mixed key and value types for builtin `is_array()` assertions,
matching `IsArrayFunctionTypeSpecifyingExtension::specifyTypes()`. The prelude annotation does
not carry that distinction. A user-defined function named `is_array` retains its own assertions.
Unconstrained keyed-array refinements keep these source-owned parameters. Pun's atomic converter
reads the mixed identity from Codex, not an annotation flag or the reported type string.
The positive array intersection also absorbs constant-array alternatives covered by its generic
array. Re-unioning those alternatives would normalize away the explicit mixed key again.

Codex's accompanying patch retains explicit mixed through subtraction, unset, unions,
intersections, copies, and distinct-array joins. It also subtracts the empty-array domain
from an array exclusion after unset, retaining only non-empty arrays in that exclusion.

Fresh controls are in `mixed-subtraction/array-guards.php` and `guard-types.php`. Their oracle
records remain strict. Two additional records expose separate existing contracts: the shared
type reporter adds parentheses around a single non-empty-array exclusion at array-guards.php:74;
and Mago infers a string assertion from an unannotated user function at line 137, which PHPStan
does not infer. This patch does not change `restore_metadata`, the shared type reporter, or
inferred function assertions.

## Declared assertions at invocation boundaries

`invocation/post_process.rs` does not publish body-inferred conditional assertions as call-site
contracts. It checks `FunctionLikeMetadata::assertions_inferred`, which the scanner sets only
when no explicit assertions exist. The populator clears that flag when a method inherits a
declared assertion. It does not inherit a parent's body predicate as a declared child contract.
Body-inferred metadata remains available to callback providers.

PHPStan's `FuncCallHandler::specifyTypes()` checks registered type-specifying extensions,
conditional return types, and reflected PHPDoc assertions. It does not inspect the callee's
body for an argument predicate. `BetterReflectionProvider` builds those assertions from the
resolved PHPDoc. Both paths are pinned at `cb3b7e99489b2e3789ee2eec10501cb7a72580ea`.

Only the inferred conditional maps are omitted. Argument evaluation, reference output,
exception processing, unconditional assertions, and plugin assertions keep their existing paths.
Declared `@phpstan-assert`, `@phpstan-assert-if-true`, and `@phpstan-assert-if-false` contracts
still apply, including inherited method contracts and opposite-branch exclusions.

`mixed-subtraction/call-assertions.php` records 28 fresh PHPStan 2.3.1 diagnostics for functions,
methods, closures, assignments inside arguments, and reference output. Public analyzer controls
compare those records with native tracking enabled and disabled. `inherited-assertions.php`
adds four oracle records for an overriding method with its own inferred predicate and an
overriding method without one. Neither inherits the parent's inferred assertion.
The existing strict record at `array-guards.php:137` now matches mixed. Its line 74
type-description mismatch is unchanged.

## File-local global constant values

`expression/constant_access.rs` prefers an encountered user declaration's local value when its atoms
differ from canonical initializer metadata. Forward reads and reads from other files retain reflection
unions. Equal atoms keep the canonical union markers, including existing benevolent numeric bounds.
Built-in constants keep their authoritative metadata and platform handling.

`expression/call/function_call.rs` records an already-scanned `define()` declaration's visited value
in that same overlay. The declaration span must match the call, so another two-argument call cannot
define or refine a constant. This reuses existing argument artifacts without a second observer walk.

The public `pun-analyse/tests/attributes.rs` regressions compare both reflection and local-flow results
with PHPStan 2.3.1. Global initializer arrays, duplicate keys, reassigned copies, namespace lookup,
scanned declarations, and precise arithmetic share the same controls. This patch does not change
class-constant union storage or add cycle resolution semantics.

## Raw class-constant values and late-bound declarations

The class-constant resolver and cached receiver consume Codex's canonical union without converting it
to a single atom. Named, `self`, and `parent` reads retain the raw initializer even when a native or
PHPDoc declaration is broader or incompatible. An overridable late-bound receiver instead exposes
the effective declaration type. Final receiver classes and final untyped constants retain raw values.
Final typed constants still expose their declared type through an overridable receiver.

Declaration-expression analysis remains independent: a PHPDoc compatibility diagnostic checks the
runtime expression type, not the canonical reflection union. The external metadata writer uses its
existing union wire encoding directly. No extension protocol or dependency version changes are needed.

The fresh PHPStan 2.3.1 controls in `class-constant-initializers/declared.php` distinguish each selection
and verify that a mismatched `@var string` reports runtime value 1 while exact reads retain `1|4`.

## Expression callbacks without provider metadata

`ExpressionHook` requires `Send + Sync`, not `Provider`. The registry stores expression callbacks and
dispatches their hook methods without reading provider metadata. The thread-safety requirements stay
unchanged. Existing providers can still implement both traits.

Pun's internal initializer scope uses this callback contract without an unused `Provider::meta()`
implementation. The public class-initializer controls exercise the callback through ordinary analysis,
including lexical `self` and `parent` arithmetic. No new analysis entrypoint or metadata calls are added.

## Raw object constant readers and member lookup

Class-constant resolution and cached receivers use `expand_class_constant_value()` to keep raw
initializer objects bare. The class-access wrapper no longer expands an already-resolved value
again. Global constant reads preserve that identity for canonical values, while encountered local
declarations retain ordinary constructor inference.

Method-template collection and property localization specialize a temporary receiver through
`specialize_named_object_for_member_lookup()`. A bare `Box` therefore stays bare when read as a
constant, but its members still use the declared template defaults or bounds. Explicit receiver
arguments and class-relative template scopes retain their existing paths.

The source contract is PHPStan 2.3.1's
[`ObjectType::getClassReflection()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/ObjectType.php#L1952-L1968),
with default selection in
[`TemplateTypeHelper::resolveTemplateTypes()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/Generic/TemplateTypeHelper.php#L64-L68).
Public controls in `initializer-objects/` distinguish raw values, native and PHPDoc declarations,
constructor flow, repeated property reads, and member return types.

## Literal-array reads inside destructuring targets

`expression/assignment/mod.rs` preserves PHPParser's target roles when preparing list values.
`ParserAbstract::fixupArrayDestructuring()` recursively converts short and legacy arrays inside a
short-array target, but stops at explicit `list()`. Arrays inside that explicit list remain literals.
The item reference marker belongs to PHPParser's `ArrayItem`, not its value expression.

PHPStan 2.3.1's `AssignHandler::doPrepareTarget()` defers List_ children. Its `applyWrite()` prepares
each child after evaluating the RHS and earlier list items. A literal-array child takes the ordinary
expression-read fallback, so its keys and nested values are evaluated without assigning its variables.
Assignments within those literals retain their normal effects. Target keys are evaluated once.
The owning sources are pinned at `cb3b7e99489b2e3789ee2eec10501cb7a72580ea`.

Pun's `rules/infer.rs` adapter observes reference-item values when an array is actually evaluated as
a literal, matching `ArrayHandler::processExpr()`. The broad undefined-allowed reference selection
does not suppress those reads. Destructuring writes do not fire literal-array expression hooks.
Call-result processing, canonical initializer inference and reference-alias storage are unchanged.

Public controls are in `tests/assignment_target_reads.rs` and `fixtures/assignment-target-reads/`.
The capture uses PHPStan's public Analyser at levels 0, 1, 2, 7, 8, 9 and 10. Reachable cases compare
the variable, assignment, nullsafe, append-read, parser and type-dump diagnostics. Purity collectors
are unchanged. Separate boundary controls compare read observations without claiming support for
unreachable syntax-rule callbacks or dynamic-variable write identities. Complete oracle diagnostics,
including those independent contracts, remain in the capture.

## Failed method-call results

`expression/call/method_call.rs` stores `ErrorType` when method analysis leaves no result.
The fallback runs after argument analysis, callable-signature providers, and magic return providers.
It does not replace a resolved result, including private methods and real `never` returns.
Unknown dynamic selectors instead produce mixed. Existing nullsafe short circuits retain their results.
The ordinary method resolver marks unresolved `method_exists` assertions as mixed: PHPStan's
[`HasMethodType`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/Accessory/HasMethodType.php#L160-L174)
provides a permissive dummy reflection rather than a failed lookup. Actual reflected methods keep their signatures.

`expression/call/static_method_call.rs` uses the same missing-reflection fallback. An invalid class operand
instead produces mixed: PHPStan first converts that operand to `ErrorType`, whose method reflection is permissive.
The shared invocation owner stays unchanged. Dynamic function and callable resolution have separate fallback
contracts, so a missing invocation artifact alone cannot select a universal call result.

The source contracts are PHPStan 2.3.1 at `cb3b7e99489b2e3789ee2eec10501cb7a72580ea`:

- [`MethodCallHandler::resolveReturnType()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/ExprHandler/MethodCallHandler.php#L453-L505)
  distinguishes failed reflection, dynamic names, and nullsafe propagation.
- [`MethodCallReturnTypeHelper::methodCallReturnType()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/ExprHandler/Helper/MethodCallReturnTypeHelper.php#L43-L58)
  filters union members by method availability before selecting a reflected return.
- [`StaticCallHandler::resolveReturnType()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/ExprHandler/StaticCallHandler.php#L497-L580)
  converts expression receivers to object or class-string object types before method lookup.
- [`MethodCallCheck::check()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Rules/Methods/MethodCallCheck.php#L51-L61)
  retains unknown-class errors. Pun's ordinary receiver check now retains them too.

The public `analyse()` regressions in `crates/pun-analyse/tests/failed_methods.rs` compare complete diagnostics,
tips, and ignorable flags against fresh PHPStan JSON in `fixtures/failed-methods/`. Only capture paths are shortened.
The original matrix changes from 18 diagnostics, with 17 exact matches, to all 28 PHPStan diagnostics.
Raw relative initializer calls change from 3 to all 11 diagnostics without changing their receiver identities.
The boundary fixture checks copied failures, overwritten values, mixed and object receivers, unions, dynamic names,
documented magic methods, static operands, callable controls, nullsafe short circuits, terminating calls,
and `method_exists` assertions on named and unnamed objects.

The dependency's public-Analyzer test `failed_method_fallback_preserves_active_signature_and_return_providers`
uses a real framed PHP worker. It checks callable-signature providers on otherwise absent instance and static
methods, magic return providers, a native Rust return provider, and a first-class callable invocation.

Fresh LLVM instrumentation covers all 23 added executable production lines without exclusions: eight in
Pun's receiver check, four in instance calls, seven in static calls, and four in the ordinary method resolver.
The instrumented public regressions compare 80 complete diagnostics and the engine's actual `never` result
against 81 PHPStan records. Pun's separate `never` display is deliberately not used as an engine expectation.
The final snapshot passes 59 selected public tests, the active-provider test, and all Methods, Functions,
and Operators rule and level partitions: 764 rule cases and 1,508 level cases. Existing gap allowances are
unchanged; no executed gap became stale.

This patch does not change function-call fallback, constant-string dynamic method diagnostics, general nullsafe
chain reconstruction, undocumented magic reflection, or Pun's `never` display. Those independent contracts
remain outside this patch.

## Failed named function results

`expression/call/function_call.rs` stores `ErrorType` when an ordinary named call has no result after argument
and invocation processing. Reflected and provider results remain intact, and the existing after-call hooks
retain their opportunity to replace the result. Dynamic callable expressions and the shared invocation owner
remain unchanged.

This follows PHPStan 2.3.1's
[`FuncCallHandler::resolveReturnType()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/ExprHandler/FuncCallHandler.php#L883-L885):
absent function reflection returns `ErrorType`, even when `function_exists()` suppresses the primary missing-function
diagnostic. This differs from the permissive dummy method reflection supplied by `method_exists()`.

The public regression in `crates/pun-analyse/tests/failed_functions.rs` compares complete diagnostics, tips and
ignorability with PHPStan captures at levels 0, 2 and 10. It covers copied failure values, evaluated arguments,
imports and qualified names, function-existence guards, resolved functions, built-ins and a closure control.
Regenerate its expectations with `tests/fixtures/failed-functions/capture.php` and the pinned PHPStan PHAR.

## Explicit mixed offset results

`utils/expression/array.rs` preserves the explicit flag when reading an offset from mixed.
The result is a fresh unconstrained mixed, not a copy of the receiver's truthiness, non-nullness,
or subtracted types. Nested reads therefore keep explicit identity without inheriting receiver constraints.
The existing never-from-loop handling remains unchanged. Unknown properties and methods still produce
implicit mixed even when their receiver is explicit mixed.

This follows PHPStan 2.3.1's
[`MixedType::getOffsetValueType()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/MixedType.php#L845-L848).
The public controls in `crates/pun-analyse/tests/explicit_mixed_producers.rs` also exercise declarations,
the real prelude, assignment copies, overwritten values, closure captures, and both union orders.
No assignment, capture, or combiner changes are needed once the producers supply the existing flag.
The fixture README records the separate native-property, clone, falsey-join, and invalid `array_pop`
boundaries rather than treating them as explicit-mixed parity.

## Parameter-dependent first-class callback returns

`expression/partial_application/function_partial_application.rs` preserves conditional return types
when constructing a first-class function signature. `invocation/analyzer.rs` binds that callback's
named parameters to the surrounding call's expected inputs before resolving its conditional return.
The specialized signature replaces the provisional callback bounds when inferring the outer call.
Ordinary generic callbacks retain their existing inference path.

This follows PHPStan 2.3.1's `GenericParametersAcceptorResolver::resolve()` and
`ResolvedFunctionVariantWithOriginal::resolveConditionalTypesForParameter()`: parameter names and
passed argument types remain available until conditional return resolution.
The public `callback_gather` controls compare fifteen fresh PHPStan observations and the unchanged
level-9 bug-11056 fixture. They cover positive and negated conditions, named arguments, a nonzero
parameter index, unknown strings, copied callables, and ordinary generic and arrow controls.
The callback, callable-origin, explicit-mixed producer, and native-lane suites pass 33 public tests.
Fresh LLVM profiles cover all 32 changed executable engine lines without exclusions.

Recapture the conditional observations from the repository root:

```sh
php .oracle/phpstan/vendor/bin/phpstan analyse \
	--configuration=crates/pun-analyse/tests/fixtures/callback-gather/phpstan.neon \
	--no-progress --error-format=json \
	crates/pun-analyse/tests/fixtures/callback-gather/conditional.php |
	jq '[.files[].messages[]]'
```

## Nested constant-list foreach back edges

`statement/loop/foreach.rs` extends the bounded array-exit chain above to nested loops and local
continue exits, following PHPStan 2.3.1's
[`ForeachHandler::tryProcessUnrolledConstantArrayForeach()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/StmtHandler/ForeachHandler.php#L658-L841).
Each required entry receives the preceding entry's fallthrough and local-continue scopes. A
nonlocal continue does not become an inner back edge. The existing sixteen-entry limit remains;
a nested speculative chain also respects PHPStan's product limit of eight. Normal assignment
invalidates dependent offset caches when binding each key and value. Speculative artifacts and
callbacks stay isolated; the ordinary walk still owns diagnostics and non-array observations.

`reconciler/mod.rs` resolves a selector from its current scope before reading or refining its
containing array, as
[`MutatingScope::specifyExpressionTypeInPlace()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/MutatingScope.php#L3773-L3820)
does. Only a single constant key identifies a named offset; union selectors do not establish
every alternative. The original dynamic expression id remains the cache identity. Positive
isset assertions use the same key resolution. A false isset narrows a required nullable offset
to null. A nullable offset that might be absent leaves the containing array unchanged: null
from an iterable-value fallback is not evidence that the entry exists. Optional non-nullable
offset removal and other value assertions keep their existing refinement paths. This follows
[`DefaultNarrowingHelper::createIssetSingleSubjectNonTrueTypes()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/ExprHandler/Helper/DefaultNarrowingHelper.php#L633-L795).
The public `coalesce_scope.rs` controls compare repeated missing-offset probes, generic arrays
and lists, required and optional nullable entries, optional integer entries, and nested offsets.

Public controls in `tests/constant_foreach.rs` retain the original `bug-13643.php` fixture and
compare complete diagnostics with fresh PHPStan captures under `tests/fixtures/constant-foreach/`.
They cover conditional writes and continue forms, already-integer, non-integer and missing
entries, asymmetric key/value assignment, loop budgets, ordinary observations and literal-selector
scopes. References, breaks, returns, hooks, optional entries, shape unions, associative iterators
and non-direct targets still use the ordinary path. In particular, this does not fix the separate
ordinary-path over-refinement after unconditional writes through a by-reference union selector.

## Invocation bottom identity

`invocation/resolver.rs` keeps a complete `TUnion` when resolving parameter-linked return types and
conditional branches. The selected branch's implicit, explicit accepting, or nonaccepting never
identity survives the outer union reconstruction. An unresolved condition combines the actual branch
types through the shared union operation. It does not mark every signature-derived bottom explicit.

This follows PHPStan 2.3.1's
[`ConditionalTypeForParameter`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/ConditionalTypeForParameter.php)
and [`ConditionalType`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/ConditionalType.php),
which return the selected type rather than reconstructing a never type. The public Analyzer controls
in `crates/pun-analyse/tests/never_identity.rs` compare selected, negated, and unresolved branches with
fresh scope captures. Call evaluation, closure return inference, termination facts, and the external
wire protocol are separate owners and remain unchanged here.

## Never values and call termination

`invocation/return_type_fetcher.rs` resolves the declaration before consulting return providers.
Its result retains both the selected value type and whether the resolved declaration is explicitly
never-returning. `expression/call/mod.rs` requires every possible target to terminate, with no
mixed or nullable continuation. Provider-selected implicit never remains an observable value type.

PHPStan's `TypehintHelper::decideType()` retains an explicit PHPDoc never even when the native
declaration returns object, array or void. The declaration compatibility fallback preserves that
bottom instead of replacing it with the native return. Argument-driven template inference also
retains the input never kind when extracting residual atoms, so forwarded callable returns keep
their actual identity without turning implicit bottom into explicit never.

This ports PHPStan 2.3.1's `FuncCallHandler::processExpr()`: resolved acceptors establish call
termination before dynamic return rewrites. PHPStan models `call_user_func()` and
`call_user_func_array()` forwarding as value rewrites of mixed-returning builtins; Mago's generic
forwarding signatures must not introduce an intrinsic exit edge. Nullsafe calls retain
`NullsafeMethodCallHandler`'s non-terminating expression result, even for a non-null receiver.
Expression statements separately terminate for explicit never values, following `ExpressionHandler`.

`AnalysisArtifacts::call_terminations` keeps these evaluation facts separate from copied types.
Function and closure artifacts transfer the facts to their parent. The existing changed-entry
checkpoint implementation now also handles boolean observations, preserving nested rollback,
merge, speculative replay and opaque-hook mutations without cloning complete maps on entry.
Call and pipe evaluations start with a checkpoint-aware false fact before expression hooks run.
Skipped or unresolved loop conditions and increments cannot retain a previous terminating result;
removing the entry instead would let a checkpoint merge restore the old true fact.
Pun's flow and finally checks consume the resolved facts. Unused-call checks suppress only
explicit never results. Throw, exit and die produce nonaccepting never; a terminating closure body
without return values infers non-accepting never, as `ClosureTypeResolver` does.

`crates/pun-analyse/tests/never_calls.rs` compares public diagnostics with the pinned PHPStan
capture under `tests/fixtures/never-calls/`. The fixture distinguishes implicit return values,
native and PHPDoc declarations, conditional named arguments, callable invocation, forwarding
inside and outside other calls, nullsafe calls and their arguments, loop bodies and finally blocks.
The public Analyzer controls in `native_lane.rs` also replace replayed conditions and increments
through hooks, checking skipped calls, replacement values and unresolved targets with native
tracking both enabled and disabled.

## Terminating expression bottom identity

`expression/construct/exit.rs` and `expression/throw.rs` emit nonaccepting never, matching PHPStan
2.3.1's [`ExitHandler`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/ExprHandler/ExitHandler.php#L68-L79)
and [`ThrowHandler`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/ExprHandler/ThrowHandler.php#L49-L60).
This changes three type emissions, not termination facts, throw edges, or argument evaluation.
Nonaccepting never remains explicit.

The earlier clone note misattributed the accepting result of `fn () => exit()` to arrow inference.
PHPStan's [`ClosureTypeResolver`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/ExprHandler/Helper/ClosureTypeResolver.php#L672-L688)
retains the body type. The installed 2.3.1 PHAR and source tag agree. Correcting the expression
producers also corrects their arrow and forwarded return types without normalizing arrow returns.

The public Analyzer regression in `crates/pun-analyse/tests/never_identity.rs` uses
`tests/fixtures/never-identity/expected-producers.json`, captured by `capture-producers.php` with
the pinned PHPStan bootstrap. It checks direct exit, die, and throw expressions, hinted and unhinted
arrows, statement-body closures, named never calls, forwarding, intrinsic call facts, and statement
continuation. Dynamic return controls retain implicit, explicit accepting, and nonaccepting never
through unhinted arrows. A blanket arrow normalization would fail those controls. Arrow inference,
native inference, clone inference, and shared combinators remain unchanged.

## Edited-body declaration ownership

The class, interface, trait, enum, method, and function duplicate guards compare against
`CodebaseMetadata::declaration_span()`. A current-file anonymous-signature snapshot records the
scanner-selected span of each named declaration owned by that file. This lets body edits change
declaration ends and positions without causing the analyzer to skip the edited body as a duplicate.
Populated named signatures and their project ownership remain shared. Without a refresh, the original
span remains authoritative. Another file's declaration is never remapped, and same-file duplicates
still compare against only the selected declaration.

The public controls in `pun-analyse/tests/closure_freshness.rs` exercise all six guards, moved bodies,
same-file duplicates, and a declaration owned by another project file. The exact Rector regression in
`pun-lint/tests/moved_closure.rs` requires both these ownership checks and fresh anonymous signatures.
Closure, invocation, scope, and plugin lookup APIs retain their existing signatures and lifetimes.

The native CLI now matches the supplied Rector 2.6.1 expectation byte for byte and is idempotent.
All 63 focused and adjacent public controls pass. Fresh profiles cover **6/6 changed analyzer
executable lines** against `e1c66a25`, without exclusions. The full commands, snapshot ownership,
and combined changed-line coverage are recorded in
[`mago-codex/PATCHES.md`](../mago-codex/PATCHES.md#current-file-anonymous-signatures).

## Clone result object guarantees

`expression/clone.rs` follows PHPStan 2.3.1's
[`CloneHandler::resolveCloneType()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/ExprHandler/CloneHandler.php#L40-L43)
and [`CloneTypeTraverser`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/Traverser/CloneTypeTraverser.php).
The result intersects the operand with `object`, then changes outer `$this` members to `static`.
Union and intersection members are visited. Object type arguments and template bounds are not traversed
when replacing `$this`. Operand evaluation and the existing clone diagnostic collector remain unchanged.

Unconstrained templates retain their scope and mixed bound with an object intersection.
Other template bounds retain their identity after intersecting the bound with object.
Callable signatures use the existing `HasMethod('__invoke')` object with the original callable payload
as an intersection member. The callable cast reads that payload without losing its signature.
No shared type normalization, native inference, or converter changes are included.

The public `clone_inference` regression compares 28 observations with fresh, identical PHPStan 2.3.1
and 2.2.7 captures under `tests/fixtures/clone/`. It checks explicit and implicit mixed operands,
object/scalar unions, nullable and invalid operands, enum and callable results, template bounds,
`$this` unions, and assignment effects. Observers check one visit per clone and operand.
The diagnostic comparison covers `clone.nonObject` and the operand's `function.notFound` at level 7,
not unrelated captured diagnostics. Mago's existing enum clone diagnostic remains intentional.
The `$this` union comes from a flow join: a PHPDoc `$this|Item` parameter has a separate scanner gap.

An operand that is already never retains its complete union before the per-atomic reconstruction.
PHPStan's object intersection and clone traversal preserve implicit, explicit accepting, and
nonaccepting never. Newly impossible clones, such as `clone 1`, still produce implicit never.
This preservation belongs to clone inference, not shared combinators or call-termination producers.

`capture-never.php` captures all three PHPStan 2.3.1 identities directly and through a dynamic
return extension on mixed-returning functions. The matching public Analyzer hooks supply the same
operands. The regression checks operand and clone identity, single evaluation, and statement
continuation. Real `call_user_func()` named and closure operands also retain nonaccepting never
without an intrinsic termination fact. Their clone statements stop execution, but a clone nested
inside another call does not stop continuation. The scalar control retains implicit never and
continues. Regenerate these expectations with `capture-never.php` and the pinned PHPStan bootstrap.

## Receiver-aware native property observations

The passive native lane now enters bounded method scopes and transfers scalar/mixed and simple
named-object parameters without consulting PHPDoc replacements. Instance property observations
reuse `resolver/property.rs::resolve_declared_property` and the original declaration metadata.
Full property types, declarations, diagnostics and inferred full return types are unchanged.

PHPStan's native property reflection is receiver-dependent. For `ThisType`/`StaticType`,
[`CallbackUnresolvedPropertyPrototypeReflection::transformPropertyWithStaticType()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Reflection/Type/CallbackUnresolvedPropertyPrototypeReflection.php#L80-L101)
reuses transformed PHPDoc metadata when it equals the native type; mixed equality ignores
explicitness. Consequently an undocumented declared-mixed property read through `$this` or its
copy has implicit native mixed, while an ordinary named-object receiver retains explicit native
mixed. A real `@var mixed` or `@var int` changes this comparison; removing comments cannot model it.
Untyped properties remain native implicit mixed even when documented. Method prototypes instead
compare their effective return with the native return, preserving the observed documented-mixed
method result through `$this`.

Mixed offsets derive their explicitness from the native operand, never its full property type.
Assignment copies and initial reference aliases transfer that value. Arrow and by-value closure
captures transfer independent native holders; by-reference closure feedback transfers the shared
full holder, matching PHPStan's observed native explicitness change. These are evaluation-only
changes: closure metadata, lookup, overlays, callbacks and scanner machinery are not modified.

The public `native_properties` regression compares 47 fresh observations from PHPStan 2.3.1 and
Rector's PHPStan 2.2.7, including native availability, type and explicit identity, full-type equality
with native tracking disabled, and immutable declaration metadata. Regenerate both captures with
`tests/fixtures/explicit-mixed/oracle.php`, passing each version's bootstrap and `native-properties.php`.
The unchanged native engine has all 47 observations unavailable.
Seven additional dual-version observations in `native-receiver-controls.php` cover scalar native
declarations, a constructed expression receiver, and the different native identities of a documented
mixed method called through `$this` and an ordinary receiver.

This is not a complete native engine. Native callable values remain unavailable, but creating an
unexecuted closure preserves unaffected outer holders. Unsupported by-reference bodies invalidate
the outer continuation. Dynamic/nullsafe members, magic/hooked or class-annotated properties, nonscalar property or
method results, method arguments, constructors, generic/compound receiver domains and reference
writes remain unavailable. Constructor-free named instantiation is supported. No comment-blanked
fallback or direct-property syntax exception is used.

## Simultaneous ASCII string casing guarantees

Concatenation and positive string assertions preserve `TStringCasing::Both` through the codex's
independent casing guarantees. PSL splice retains the guarantees shared by the source and replacement.
Unicode and grapheme case conversion still sets only its requested guarantee; it does not assume
that an input unchanged by ASCII case conversion stays unchanged by Unicode case conversion.
Pun's type conversion emits both PHPStan string accessories.

Complete snapshots use protocol 2.1. String casing tags 0, 1, and 2 still mean unspecified,
lowercase, and uppercase. Tag 3 means both. Both header readers reject protocol 2.0 instead of
silently dropping the extra guarantee. The casing snapshot tests check fixed legacy bytes,
all four round trips, current-version acceptance, and old-version rejection.

This patch does not add a `str_repeat` return-type provider or change literal byte storage.

Fresh profiles cover **16/16 changed analyzer executable lines**, **99/99 changed production lines**
including codex and Pun conversion, and **294/294 changed Rust test lines**, without exclusions.
The 38 selected Pun tests pass. The standalone analyzer suite passes 420 of 426 tests.
The same six failures occur on pristine `19a99280` (419 of 425 tests pass): three destructuring tests,
the static-interface-call test, and two foreach-variable tests. The new casing snapshot test passes.
The standalone run adds protocol coverage to the codex run. Temporary Cargo patches point `mago-codex`
and `mago-syntax` at their maintained directories:

```sh
cargo llvm-cov --manifest-path third_party/mago-analyzer/Cargo.toml --dep-coverage mago-codex \
	--no-report --lib --features mago-codex/serde -- --test-threads=2
```

Coverage is the intersection of changed lines against `19a99280` and LLVM's executable-line records.
The reports use `llvm-cov export` directly because cargo-llvm-cov's dependency report filters out path dependencies.

The combined casing and `str_repeat` replay on `a147e34b` preserves the current NeverKind conversion
and separate call-termination facts. The current native-lane replay, inspection, convergence, never,
and callback controls pass in the 212-test Analyzer selection. All 19 protocol-module tests pass,
including the new casing tags and rejection of protocol 2.0. This replay does not rerun the complete
standalone engine suite described above. Fresh private profiles cover **16/16 changed analyzer
production lines**, **441/441 combined production lines**, and **422/422 Rust test lines** against
`a147e34b`, without exclusions, using one Cargo job, one Rayon thread, and `CARGO_INCREMENTAL=0`.

## Boolean operand scopes

Boolean expressions retain their evaluated post scope and both filtered branch scopes.
`&&` takes its truthy override from the evaluated RHS. `||` takes its falsey override from the RHS.
The opposite branches apply specifications to the merged operand scope. Nested expressions reuse
the captured scopes instead of applying an earlier operand's assertions to reassigned variables.
`if` joins the actual continuing branch types through the existing variable-scope merge service.

These contracts follow PHPStan 2.3.1's
[`BooleanAndHandler`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/ExprHandler/BooleanAndHandler.php),
[`BooleanOrHandler`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/ExprHandler/BooleanOrHandler.php),
and [`BooleanNarrowingHelper`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/ExprHandler/Helper/BooleanNarrowingHelper.php).
The RHS walk still occurs when the LHS has a known Boolean result, matching PHPStan's captured
assignment effects. Excluded mixed values use the shared intersection operation during positive
assertion reconciliation, so narrowing to an object does not discard its negative constraints.
Public controls retain the original negative-branch expectations and cover assignments, nested
conditions, returns, ternaries, scalar guards, and generic or receiver-specific object identities.

## Named-object exclusions in protocol 2.2

Named-object snapshots append an exclusion-presence flag after the remapped-parameters flag.
When present, the payload is a nested union: request snapshots retain its request-local handle;
complete provider responses omit handles, as they do for other nested unions. Existing object,
atomic, and message tags retain their meanings. Protocol 2.1 already introduced casing tag 3;
the new object layout uses 2.2. Both header readers reject 2.0 and 2.1 before decoding their payloads.

The encoder and decoder use the existing union recursion and member-count limits. Exclusions stay
separate from positive intersection children, including unions and nested negative constraints.
Real PHP worker and public Analyzer controls cover ordinary objects, flow-narrowed exclusions,
nested provider round trips, and rejected over-depth, oversized, or empty union payloads. Rejected
responses retain the existing provider warning and native-analysis fallback.
Their semantic controls use PHPStan 2.3.1's retained `object-intersections` captures: joining
`mixed~A` with `C~A&C` yields `mixed~A&C`, whereas joining it with `C` or `A&C` yields `mixed`.

## Nonempty concatenation truthiness

`expression/binary/concat.rs` refines the concatenation of two nonempty strings to a non-falsy
string. Both the aggregate and pairwise paths check this conjunction before propagating an
existing truthy operand. Aggregate queries require every alternative on each side to be nonempty.
The existing nonempty fallback, literal folding, literal origin, and independent casing guarantees
remain unchanged.

The branch order follows PHPStan 2.3.1's
[`InitializerExprTypeResolver::resolveConcatType()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Reflection/InitializerExprTypeResolver.php#L582-L600).
The public `pun-analyse/tests/string_casing.rs` regression runs the unchanged research probe through
`run()`. Its `fixtures/string-casing/concat-expected.json` contains 29 freshly captured level-0
PHPStan diagnostics. Before the correction, 11 types differ and 18 controls match. After the
correction, all 29 match, including asymmetric operands, partly-nonempty unions, empty and zero
suffixes, literal-origin strings, casing accessories, and flattened chains.

The repeat controls retain PHPStan's separate behavior: `str_repeat($nonempty, 2)` remains
`non-empty-string`, not `non-falsy-string`. Casing-union normalization and native inference are
unchanged. The existing casing, guarantees, repeat, and native-lane selection passes all 34 tests.

Fresh raw LLVM profiles from the new public regression alone cover **6/6 changed production
executable lines** against `ea303bb4`, without exclusions. The maintained analyzer is explicitly
instrumented with `--dep-coverage mago-analyzer`. The private target uses one Cargo job, one Rayon
thread, one test thread, debug level 1, and no incremental compilation:

```sh
export CARGO_TARGET_DIR=/tmp/pun-concat-target
export CARGO_LLVM_COV_TARGET_DIR="$CARGO_TARGET_DIR"
export CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=1 RUST_TEST_THREADS=1
export CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=1 CARGO_PROFILE_TEST_DEBUG=1
export CARGO_NET_OFFLINE=true
timeout 900 nice cargo llvm-cov --offline --no-report --dep-coverage mago-analyzer \
	-p punphp-analyse --test string_casing --test string_guarantees --test str_repeat \
	--test native_lane -- --test-threads=1
```

## Boolean operand definition certainty

Evaluated Boolean scopes retain variable state and possible-variable membership alongside their
local types. The post-expression join uses the incoming `VariableScope` snapshots before adding
RHS-only cached types. An output argument evaluated only on the RHS therefore remains possibly
defined when the LHS can short-circuit. Branch transfers restore the captured certainty state as
well as the evaluated types, including nested expressions and explicit absence after `unset`.
The object-exclusion transfer and the continuing `if` branch type join remain unchanged.

The original public `variables` flow fixture retains both PHPStan 2.3.1 diagnostics for `$matches`
at lines 99 and 100. Fresh captures also confirm the unchanged transfer-fixture expectations.
The existing live-scope observer adds `&&`, `||`, post-expression, and nested-RHS output-argument
controls. Their expected certainty follows independent PHPStan 2.3.1 reads, not cached type presence.

All 51 tests pass across `variables`, `object_intersections`, `incoming_isset`, `closure_from_callable`,
`closure_freshness`, `never_calls`, and `never_identity`. The four original variable configurations
also match fresh PHPStan 2.3.1 captures without changing their expectations. Fresh profiles cover
**40/40 changed analyzer executable production lines** against `ea303bb457f6342e24d948076bfbca4ef785d0a1`,
without exclusions. The measurement intersects the changed lines with a direct LLVM LCOV export.
The analyzer dependency is explicitly instrumented:

```sh
cargo llvm-cov --offline --dep-coverage mago-analyzer --no-report -p punphp-analyse \
	--test variables --test object_intersections --test incoming_isset \
	--test closure_from_callable --test closure_freshness --test never_calls --test never_identity
```

## Nested variable-name observations

Read and assignment-target preparation analyze a nested variable's name through the expression
dispatcher. Inner indirect names therefore receive ordinary hooks even when the outer target
allows undefined variables, as in assignment, isset, empty, coalescing and unset.

Pun records a nested target's certainty before evaluating its name, matching DefinedVariableRule's
incoming scope. Indirect names retain their post-name observations, including assignment side
effects. The existing public variable fixture includes both forms, three-level nesting, defined
and maybe-defined names, and multiline name diagnostics. Expectations come from PHPStan 2.3.1
in all four existing variable configurations; the earlier fixture records remain unchanged.

## Numeric-string accessory retention

String assertion reconciliation transfers the independently retained non-empty accessory, not
nonemptiness implied by numericness. Numeric refinement still proves semantic nonemptiness.
Pun's type conversion reads the retained-accessory query, so bare numeric-string stays bare while
an explicitly intersected non-empty accessory survives assignments and branch merges.

The native float-to-string constructor marks nonemptiness as implied by numericness, matching
PHPStan 2.3.1's `FloatType::toString()` (`src/Type/FloatType.php:129–136`). Its numeric, uppercase,
and semantic nonempty guarantees are unchanged. The existing native-cast oracle stays unchanged.

String scalar snapshots retain the existing semantic flag at bit 2. Bit 4 records numeric-implied
nonemptiness without a separate accessory. The decoder reconstructs the constructor's accessory
argument from both flags. The existing casing wire matrix checks bare numeric flags 21 alongside
explicit numeric flags 5 for every casing state. Object layouts and message version headers are
not changed by this patch.

The source contract and public PHPStan 2.3.1 regression are documented in
`mago-codex/PATCHES.md`, under “Numeric-string nonempty accessories”.

## Literal string-offset reads

`src/utils/expression/array.rs::handle_array_access_on_string()` retains the byte selected by a
single inferred integer literal from a known literal string. Negative offsets count from the end.
The reference is PHPStan 2.3.1's `src/Type/Constant/ConstantStringType.php:440–484` at
`cb3b7e99489b2e3789ee2eec10501cb7a72580ea`. Its valid offsets are `-strlen` through `strlen - 1`,
where `strlen` counts PHP bytes, not Unicode characters.

An out-of-range read returns ErrorType, including reads from an empty string. Isset analysis uses
the existing absent-value placeholder so coalesce selects its fallback. Definitely noninteger
indexes still fail before literal extraction; numeric strings, booleans, floats, and null are not
coerced into integer indexes. The helper consumes the already inferred index without evaluating it again.

The public `variables` regression appends four-mode PHPStan captures to
`crates/pun-analyse/tests/fixtures/variables.json` without changing its 236 earlier records.
Controls cover positive and negative bounds, a large index, empty strings, noninteger indexes,
multibyte and binary receivers, defined and maybe-defined names, receiver and index reassignment,
an exact-offset write, union receivers, and coalesce fallbacks. The existing nested-name, multiline,
and array-offset controls remain intact.

This patch does not implement PHPStan's finite-range character unions for nonliteral integer indexes,
specialize template-bound indexes, or reconstruct a whole string after character writes. These paths
retain their existing inference; the exact-offset write control checks the existing stored offset only.
For example, fresh PHPStan yields `''|'f'|'o'` for `'foo'[$index]` when `$index` has general type `int`;
that range-based inference is not part of this correction. The patch does not establish complete
DefinedVariableRule or CompactVariablesRule parity.

## Failed static property reads

`expression/access/static_property_access.rs` emits `ErrorType` for an unresolved static property,
not null. A failed alternative in a finite name union contributes the same error type.
The existing union combiner preserves that identity alongside successful property alternatives.
Real null properties retain their declared value and decrement deprecation diagnostics.

This follows PHPStan 2.3.1's
[`StaticPropertyFetchHandler::composeResult()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/ExprHandler/StaticPropertyFetchHandler.php#L170-L223).
`InvalidIncDecOperationRule` checks the completed operand read before the virtual property write.
Pun already captures that read through its expression hook. No diagnostic filter, inc/dec rule,
conversion, or shared combiner change is needed.

Pun's property checker also applies the virtual `PropertyAssignNode` check to direct static names.
Expression names already receive that check. Both paths reuse the same receiver and visibility checker,
matching `AccessStaticPropertiesInAssignRule` rather than omitting the direct name's second finding.

The public `statements` controls compare direct reads, finite dynamic reads, mixed successful and
failed alternatives, all four inc/dec forms, actual null and nullable properties, valid names,
unknown names, visibility, and instance-property access with fresh PHPStan captures at levels 2 and 8.
The original `static-property-names/boundaries.php:61` control retains its four primary property
findings without the spurious `preDec.type` on null. Existing receiver and visibility resolution,
null receiver handling, and the separate error-path fallback remain unchanged.

The level-2 operand capture matches all 52 diagnostics, including tips and ignorability.
The strict level-8 capture retains two separate missing `assign.propertyType` findings for `++`
of a property declared null. `types/properties.rs::increment()` checks write access and readonly
rules, but explicitly omits the assigned-value type check. That rule needs the written value,
which differs from a postfix expression's result. This patch does not duplicate that rule in
`inc_dec.rs` or change its expectation. The complete statements run therefore has 21 passing
tests and one failing new operand test. All original statements tests pass.

A supplemental level-10 oracle capture also exposes the unchanged error-path fallback for static
access to an instance property: PHPStan reports `preDec.type` on mixed, while the analyzer retains
never. The missing-property producer fix does not change this distinct resolver failure.
Neither remaining boundary requires a conversion change.

Fresh LLVM profiles from public `run()` calls cover all five added executable production lines,
without exclusions. The existing dispatch after removal of the direct-name restriction executes
101 times. The analyzer dependency is explicitly instrumented with `--dep-coverage mago-analyzer`.
