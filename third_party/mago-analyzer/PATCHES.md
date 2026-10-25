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
and benchmarks. The manifest removes those test targets and their development dependencies.
Every sibling `mago-*` dependency remains a crates.io dependency pinned to `=1.57.0`.
The root manifest patches only `mago-analyzer` and excludes this directory from Pun's workspace members.

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

## Passive native state: bounded first slice

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

### Deliberate availability limits

The first unsupported producer or transfer invalidates native availability for the current scope before
its children run. Subsequent assignments do not recover that scope. This scope-wide rule is conservative:
it avoids claiming that an unsupported operation left aliases or locals unchanged. Each separately
analyzed named function can still initialize its own supported native scope.

Branches, loops, joins, exceptional control flow, references, arrays, members, class scopes, methods,
closures, arrows, captures, globals, statics, unset, operators, dynamic calls, and undefined local reads
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

The final clean-profile run passes all 33 tests: native lane 8/8, callback gathers 10/10,
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
