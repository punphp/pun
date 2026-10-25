# Mago analyzer 1.57.0: raw callback gathers

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

Only three source files differ from the published package. Line ranges below refer to this copy.

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
hooks that supply semantic types must not be treated as observers. Pun's dynamic return providers
use the separate provider APIs and are not suppressed.

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

The public callback suite has five passing tests and five failures, all from the remaining
`array` versus `array<mixed>` conversion difference. The failures include the original upstream case
at levels 0, 5, and 9. Expectations remain the actual oracle messages. The array-slice suite passes
all five tests, including Methods bug12880, and the debug suite passes all ten tests.

`types/convert.rs::source_type(Origin::Call)` supplies the declared `array<RK, RV>` from
the fixture's `@return`. Its `restore()` handles mixed against mixed, but not mixed against
a template's bound. The semantic patch does not alter this separate metadata conversion.

The public Analyzer test injects a one-shot call-hook error during a gather and checks propagation.
Its successful second analysis checks that an expression observer sees the callback body once.
Removing the two observer guards makes that assertion fail with two visits instead of one.
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
