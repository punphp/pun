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

## Never-valued generic arguments

`comparator/generic_comparator.rs` accepts a specialized `never` argument before variance checks,
matching PHPStan 2.3.1's `TemplateTypeVariance::isValidVariance()`. Assignment preserves that
argument instead of widening it to the containing type's parameter. The public `generic_defaults`
fixture compares empty and populated constructors, a readonly property assignment, and subsequent
offset checks against the actual PHPStan oracle. This does not change unresolved placeholders.

## Subtracted mixed values

`TMixed` stores an optional shared `TUnion` of excluded values. `TMixed::subtract()` combines repeated
exclusions, `get_subtracted_type()` exposes the excluded set, and `with_subtracted_type()` normalizes
a `never` exclusion to ordinary mixed. `TMixed::unset_offset()` removes the empty array from the
excluded set because unsetting an offset may produce an empty array. Exclusions such as integer `0`
survive. `get_error()` and `TMixed::is_error()` retain their existing interfaces and behavior.

These transfers follow PHPStan 2.3.1's
[`MixedType`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/MixedType.php#L182-L188)
and its [`subtract()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/MixedType.php#L855-L866).
`ErrorType::subtract()` remains a no-op. Direct subtraction of a mixed value from ordinary mixed
returns `never`, including when the removed mixed itself has an exclusion.

The shared union transfer removes newly included values from the excluded set. Two mixed exclusions
intersect in a union and combine in an intersection. Integer differences use Mago's existing range
arithmetic. Unsupported partial scalar differences retain no exclusion in a union, matching
[`TypeCombinator::intersectWithSubtractedType()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/TypeCombinator.php#L958-L1034).
For example, `mixed~0 | 13` remains `mixed~0`, `mixed~0 | 0` becomes mixed, and
`mixed~string | 'x'` becomes mixed. Plain mixed and error markers still follow operand order.
Containment compares excluded sets in reverse. Nullability transfers retain other exclusions.
Existing nonnull and truthy mixed constraints contribute their excluded sets to these operations.

Pun converts the excluded set to `Type::SubtractedMixed`, rather than changing a dump's text.
Its public type combinators, acceptance results, acceptance reasons, and precise descriptions retain
the exclusion. As in `MixedType::traverse()`, traversal does not visit the excluded set.
A subtracted mixed accepts every input, but a concrete `0`
does not accept `mixed~0`, matching `MixedType::accepts()` and `isAcceptedBy()`.

The recursive field changes `TMixed` from `Copy` to `Clone`. Three union transfers clone borrowed
mixed values. One analyzer reconciler arm reconstructs its already-moved atomic without changing
the reconciliation result. Equality, hashes, ordering, and serde include the excluded set.
Old serialized values default to no exclusion. No codex atomic variant, manifest, or Mago version changes.

`pun-phpstan/tests/mixed_subtraction.rs` owns the public Pun type-operation tests, so that crate's
coverage run exercises its mixed implementation independently. `pun-analyse/tests/mixed_subtraction.rs`
checks codex operations and Analyzer flow. Both use the same captured PHPStan oracle under
`pun-analyse/tests/fixtures/mixed-subtraction`. The Analyzer regression injects the narrowed value
through an expression hook, independently of the unset producer. Copies and
joins retain `mixed~0`, joining `0` restores ordinary mixed, and an offset read returns ordinary mixed.
The same assertions run with native tracking enabled and disabled.

The fixture oracle is regenerated with:

```sh
php crates/pun-analyse/tests/fixtures/mixed-subtraction/oracle.php \
	/absolute/path/to/phpstan/vendor/autoload.php
php /absolute/path/to/phpstan.phar analyse --no-progress --error-format=json -l 0 \
	crates/pun-analyse/tests/fixtures/mixed-subtraction/flow.php |
	jq '{messages: [.files[].messages[] | select(.identifier == "phpstan.dumpType") | {line, message}]}' \
	> crates/pun-analyse/tests/fixtures/mixed-subtraction/flow.json
```

This slice does not change unset producers, keyed-array conversion, constant-array order, failed
reads, or the external worker codec. Arbitrary object subtraction remains outside this bounded port.

Verification on the isolated `e5d409f3` base: all 50 `punphp-phpstan` tests and 24 focused Analyzer
tests pass. LLVM line coverage reaches all 235 changed executable lines in codex, the mechanical
analyzer adjustment, and Pun types. Total changed production coverage is 236/237 lines.
The remaining line is the new excluded-set conversion expression in `rules/types/convert.rs`.
The public Analyzer test supplies exclusions through a hook, but Pun's public runner has no such
input hook. That conversion needs the separately owned unset producer for end-to-end coverage.

### Exclusion-aware Pun queries

Pun's string, float, boolean, array, list, iterable, iterable-at-least-once, callable, null, and void
queries return `No` only when the excluded type contains the complete queried domain. A literal
string exclusion does not exclude all strings. The list domain is the intersection of an array
with nonnegative integer keys and the list accessory. The iterable domain includes objects, so
excluding all arrays still leaves iterable values possible.

These checks follow
[`MixedType`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/MixedType.php#L892-L950).
Compound queries dispatch each member's public query. Union and benevolent-union results retain
PHPStan's method-specific aggregation. `TemplateMixedType` inherits plain mixed queries rather
than querying its bound's excluded set. Domain types are constructed only for a subtracted mixed.

The owning-crate regression checks 1,140 fresh PHPStan query results without duplicating the
Analyzer tests. All 53 `punphp-phpstan` tests and 22 focused Analyzer tests pass. A fresh target
running only `punphp-phpstan` covers 45/45 executable lines added by this query slice, and 155/155
changed Pun type lines including the preceding mixed-subtraction implementation.

## Constant-array source kind

PHPStan 2.3.1 distinguishes `ConstantArrayType` from an `ArrayType` intersected with
`HasOffsetValueType`. Mago represents both as a keyed array with known entries and extra parameters.
The source kind now survives in `UnionFlags::CONSTANT_ARRAY_KEY` on the extra-key union.
`TKeyedArray::is_constant_shape()` also recognizes sealed arrays, which have no extra parameters.

The PHPDoc builder marks open shapes with explicit fields. A shape declared without any fields
remains generic, matching PHPStan's `TypeNodeResolver`. Removing the final field of an existing
constant array retains the marker, matching `ConstantArrayType::recreate()`.

The changes are in `ttype/flags.rs`, `ttype/atomic/array/keyed.rs:139–145`, and
`ttype/builder.rs:797–803`. Union equality includes the flag. The borrowed and owned union paths
in `ttype/mod.rs` retain it only when both live key unions carry it. The combiner's known-entry
branch at `ttype/combiner.rs:941–948` keeps fields optional when joining two declared open shapes
instead of merging those fields into the unrelated extra-key domain.

The reference remains PHPStan source revision `cb3b7e99489b2e3789ee2eec10501cb7a72580ea`, especially
`PhpDoc/TypeNodeResolver.php`, `Type/Constant/ConstantArrayType.php::unsetOffset()`, and
`Type/Accessory/HasOffsetValueType.php::unsetOffset()`. No scalar subtraction, array intersection,
dependency version, or registry-cache changes are included.

Public Analyzer controls are in `pun-analyse/tests/fixtures/unset/offset-metadata.php`. They cover
empty declared arrays, shapes emptied by mutation, copies, overwrites, narrowed values, generic
integer and string offsets, finite union offsets, template arrays, both join orders, and inline
annotations. The mixed-key controls also distinguish the alias from an explicit `int|string` union.
