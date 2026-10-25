# Mago codex 1.57.0: maintained patch

## Source

The library source and normalized manifest come from the published `mago-codex` 1.57.0 package
(checksum `4fad91caa6943ff61d655d1143b864ee265d2e675d0347a9d140ebee6541ba10`). Its
`.cargo_vcs_info.json` identifies upstream revision
[`9656cd55`](https://github.com/carthage-software/mago/tree/9656cd55d51d96bd6dc57f1f1e73c15f22cb7cc3/crates/codex).
Both license files come from that revision. Registry bookkeeping, the package lockfile, external
tests, and benchmarks are omitted, along with their manifest targets and development dependencies.
Sibling Mago dependencies remain pinned to `=1.57.0`. The root manifest patches this crate and
excludes its directory from Pun's workspace members.

## Parent-relative member references

`ttype/expander.rs::expand_member_reference()` backports the codex change from
[`2066b186`](https://github.com/carthage-software/mago/commit/2066b186d2a7cea821e0d4923934032e5f4e32c5),
included in Mago 1.59.0. Before either member lookup or the `::class` shortcut, a case-insensitive
`parent` resolves through `TypeExpansionOptions::self_class` to the class metadata's
`direct_parent_class`. Other names and missing-context fallbacks keep their existing behavior.
No analyzer walk, hook dispatch, native tracking, or dependency version changes.

Public regressions in `pun-analyse/tests/parent_defaults.rs` compare complete diagnostics at levels
0 and 5 with freshly captured PHPStan 2.3.1 output. Distinct ancestor, parent, child, and caller
constants catch incorrect lookup scopes. Controls cover inherited constants and constructors,
mixed-case `parent`, dependent defaults, `parent::class`, `self`, and qualified class names.
`generic_defaults.rs` also checks equal full artifacts and one observer visit per expression with
native tracking enabled and disabled for this fixture. This patch does not close the broader
unspecified-template property gap or change missing-default bounds.

The unchanged 1.57.0 source fails the new public regression: parent defaults become `mixed`,
properties become `int`, and `parent::class` becomes `'parent'`. The patch matches all 24 dumps at
level 0 and all 26 diagnostics at level 5, including two distinguishing argument errors.
Fresh profiles from the six public test binaries cover **7/7 changed codex executable lines**
against the published source and **57/57 changed test executable lines** against `ac32a4b3`,
without exclusions. All other imported codex source is byte-identical to the published package.
The 38-test run uses `cargo llvm-cov --dep-coverage mago-codex --no-report -p punphp-analyse`
with `--test parent_defaults --test generic_defaults --test callback_gather --test debug`
and `--test native_lane --test instantiation`, one Cargo job, and two Rayon threads.

## PHPStan error-valued mixed

`ttype::get_error()` constructs a `TUnion` containing `TMixed::error()`.
`TMixed::is_error()` and `TUnion::is_error()` distinguish the marker from ordinary mixed.
The marker retains mixed's acceptance and access behavior. Its internal ID is `*ERROR*`, and
the existing equality, hash, ordering, and serde implementations include its identity.
JSON objects saved before the marker field existed still deserialize as ordinary mixed.
No `TAtomic` variant or dependency version changes.

PHPStan 2.3.1's
[`ErrorType`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/ErrorType.php#L9-L50)
extends `MixedType` and overrides `subtract()` to return another unconstrained `ErrorType`.
Non-null and truthy narrowing therefore retain the marker. Falsey and empty constraints produce
ordinary constrained mixed. Explicit intersection with a concrete or constrained type keeps that
type rather than the error marker. This differs from subtractive narrowing.

[`TypeCombinator::doUnion()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/TypeCombinator.php#L265-L278)
keeps the first implicit, unconstrained mixed operand. An error joined with `13` stays an error
in either order. An ordinary mixed operand before an error stays ordinary mixed.
The shared combiner and the borrowed, owned, reference-counted, and shape-preserving union paths
retain this order without changing combinations that contain no error marker.
Explicit-mixed provenance is not represented by Mago's existing `TMixed`.

The public `pun-analyse/tests/error_marker.rs` regression supplies the marker through an expression
hook, independently of the unset and undefined-read producers. Its Analyzer type dumps compare with
fresh PHPStan output with native tracking both off and on. Copies and concrete joins retain the
marker. Unknown offset reads return ordinary mixed, matching
[`MixedType::getOffsetValueType()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/MixedType.php#L845-L848).
An overwrite removes the marker. Public type tests also cover union order, intersections,
subtractive narrowing, availability flags, identity in sets, and serde round trips.
The tests adapt display syntax only, including Mago's constrained-mixed representation of falsey values.

The oracle records 28 public type operations and 10 Analyzer dumps. An ordinary-mixed negative
control fails at the six error-bearing dump sites; the error marker matches them all.
The focused regression and the `variables`, `coalesce_scope`, `native_lane`, `generic_defaults`,
and `array_assignment_certainty` controls pass all 26 tests. Fresh profiles from `error_marker`
alone cover **42/42 changed production executable lines** and **235/235 new test executable lines**,
with no exclusions. The run uses a private `CARGO_TARGET_DIR`, one Cargo job, and two Rayon threads:

```sh
CARGO_TARGET_DIR=target-error-marker CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=2 \
	timeout 900 nice cargo llvm-cov --dep-coverage mago-codex --no-report \
	-p punphp-analyse --test error_marker
```

Regenerate the public type oracle with:

```sh
php crates/pun-analyse/tests/fixtures/error-marker/oracle.php \
	/absolute/path/to/phpstan/vendor/autoload.php
```

Capture the Analyzer fixture with PHPStan 2.3.1 at level 0:

```sh
php /absolute/path/to/phpstan.phar analyse --no-progress --error-format=json -l 0 \
	crates/pun-analyse/tests/fixtures/error-marker/flow.php |
	jq '{messages: [.files[].messages[] | select(.identifier == "phpstan.dumpType") | {line, message}]}' \
	> crates/pun-analyse/tests/fixtures/error-marker/flow.json
```

This patch does not add error producers, Pun conversion, array-literal normalization, or iterable
extraction transfers. The external worker complete-type wire format is unchanged. Serde persistence
and in-process type handles retain the marker, but a future worker codec extension needs its own
public transport regression before complete types can carry the marker across that boundary.
