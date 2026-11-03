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

## Parenthesized scanner expressions

`scanner/inference/mod.rs::infer_with_constant_sources()` uses the existing `unwrap_parens()`
before expression dispatch. Nested parentheses preserve the inferred value for every scanner caller,
including recursive bitmask operands. Constant sources and the enclosing class remain unchanged.
No attribute-only inference fallback or dependency version changes are included.

The public `pun-analyse/tests/attributes.rs` regressions compare complete diagnostics with PHPStan
2.3.1. Parenthesized integer flags retain their supported targets and repeatability. String, float,
boolean, null, and unresolved flags remain invalid attribute declarations. The oracle uses
`tests/fixtures/attributes/phpstan-input-flags.neon`; its source revision is
`cb3b7e99489b2e3789ee2eec10501cb7a72580ea`.

## Explicit mixed identity and empty-array subtraction

`TMixed::is_explicit` records PHPStan's explicit mixed identity independently of a containing
union's annotation flags. Its serde default is false, so older serialized metadata remains
readable. Analyzer assertions can set this identity without changing PHPDoc parsing globally.
Subtraction and unset preserve the identity. A mixed union is explicit only when every mixed
operand is explicit; intersection retains explicitness from either mixed operand. ErrorType
keeps its existing identity and precedence.

The array combiner normalizes unconstrained mixed keys through array-key when joining distinct
arrays, matching `ArrayType::getIterableKeyType()` in PHPStan's `TypeCombinator`. Equal array
operands retain their parameters. Mixed value parameters retain their explicitness during a join.

`mixed/subtraction.rs::remove()` now handles array minus empty array. Both keyed arrays and lists
retain their parameter types and become non-empty. This ports `ArrayType::tryRemove()` and
allows `TMixed::unset_offset()` to admit an empty array without admitting excluded non-empty arrays.

The source remains PHPStan 2.3.1 at `cb3b7e99489b2e3789ee2eec10501cb7a72580ea`.
`pun-analyse/tests/fixtures/mixed-subtraction/guard-types.php` records the PHAR's explicitness and
array-domain results. `tests/mixed_subtraction.rs` checks these shared APIs, serde compatibility,
public guard diagnostics, and passive native tracking. No dependency or lockfile changes are needed.

## Inherit declared assertions, not parent body predicates

`populator/docblock.rs` excludes body-inferred conditional assertions from PHPDoc inheritance.
Copying those assertions used to clear `assertions_inferred` on an overriding method, turning
the parent's implementation detail into a declared child contract. A child without its own
predicate could also receive inferred parent assertions while keeping a false origin flag.
Declared parent assertions still override inferred child assertions and remain inheritable.
The PHPStan 2.3.1 controls in `mixed-subtraction/inherited-assertions.php` cover both inferred
parent cases. `call-assertions.php` covers the declared-parent control.

## Initializer ternaries and null coalescing

`scanner/inference/mod.rs::infer_with_constant_sources()` follows PHPStan 2.3.1's
[`InitializerExprTypeResolver.php:289–314`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Reflection/InitializerExprTypeResolver.php#L289-L314).
Null coalescing unions the non-null left type with the right type. A full ternary unions the
truthy part of its middle operand with the else type, without inspecting the condition.
A shorthand ternary uses the condition as that middle operand. Neither form selects a runtime
branch. The existing union combiner normalizes the results against the scanner's empty symbol view.
Deferred constant references retain their existing expansion path.

The public `pun-analyse/tests/attributes.rs` control-flow regression uses a fresh PHPStan 2.3.1
capture from `tests/fixtures/attributes/phpstan-control-flow.neon`. Controls distinguish same
and different branches, falsey and null removal, zero flags, unsupported targets, nested bitmasks,
repeatability, and built-in, global, and class constant references. Unsupported operand expressions
remain unavailable. This patch does not add calls, object construction, comparisons, boolean
operators, precise float initializers, or removal constraints on deferred constant references.

## Resolved global initializer inputs

`scanner/inference/mod.rs::infer_initializer()` exposes the existing inference dispatcher with resolved
global constants and exact independently evaluated operand types. Global lookup selects the resolved
name first. Only an unqualified, unimported name can fall back to the global namespace, and an existing
declaration stops that fallback even when its type is unavailable. Ternary falsey removal and coalesce
null removal therefore operate on dependency values rather than deferred reference atoms.

Pun's global initializer resolver supplies full arithmetic inference for subtrees without ternaries,
coalescing, calls, or construction. The shared scanner combines canonical control-flow trees and
supported arrays. No runtime evaluation of an enclosing tree can replace their reflection unions.
Class attribute arguments with global dependencies use this same initializer service, including
symbol-only declaration files. Other class-scoped initializer behavior remains unchanged.

The public regressions and PHPStan 2.3.1 captures are under
`crates/pun-analyse/tests/fixtures/attributes/phpstan-global-initializer*.neon`.
They distinguish canonical reflection values from file-local values after a declaration, preserve
precise float arithmetic and array values, and check namespace fallback, explicit aliases, unresolved
inputs, target flags, and unsupported calls. Class-constant union storage, arbitrary declaring scopes,
and new cycle semantics are outside this patch.

## Canonical class-constant unions and lexical dependencies

`ClassLikeConstantMetadata.inferred_type` stores a `TUnion`, like global constant metadata.
The scanner, type populator, reference expander, and constant query retain every inferred alternative.
Array-key extraction still requires a singleton literal. The direct-self `never` sentinel remains intact.

`value_type()` separates the raw initializer from native and PHPDoc declarations. Exact reads use
the initializer. Late-bound reads on an overridable receiver use the effective declaration, or mixed
for an untyped non-final constant. A final untyped constant retains its raw value.

`infer_initializer_with_codebase()` reuses the scanner dispatcher with resolved global and class
dependencies. Named holders, `self`, and `parent` resolve before falsey or null removal. Integer bitwise
operations retain literal union alternatives instead of selecting an atom or generalizing to int.
Pun's initializer service evaluates ordinary inherited copies in the lexical declaring class and
publishes their raw value without changing declaration metadata. Class-dependent attribute arguments
use that same service. Constants of already-loaded user classes and interfaces are canonical roots,
including PHPDoc-only wildcard selectors and existing class-string reads. This does not load additional
autoload files or analyze unrelated executable bodies.

The source contract is PHPStan 2.3.1's `InitializerExprTypeResolver.php` at
`cb3b7e99489b2e3789ee2eec10501cb7a72580ea`, including its non-runtime ternary and coalesce unions.
Fresh public controls and oracle captures are in
`crates/pun-analyse/tests/fixtures/class-constant-initializers/`.

Trait rebinding does not use the ordinary declaration-span cache. New trait semantics, expanded cycle
resolution, enum backing unions, arbitrary dynamic holders, calls, construction, and general
initializer-expression expansion remain outside this patch.

## Raw initializer object identity

`TypeExpansionOptions::preserve_initializer_object_identity` keeps a raw initializer's bare named
objects intact, including objects inside shapes and unions. Reference expansion and its cycle guards
still run. Declaration expansion retains its existing treatment of omitted generic arguments.

`expand_class_constant_value()` selects the raw initializer or effective declaration before expansion.
Global and member references enable the identity policy only for inferred values. Declaration-only
references clear it. `specialize_named_object_for_member_lookup()` applies the existing template
defaults and bounds to a temporary receiver, without changing the stored initializer value.

The source contracts are PHPStan 2.3.1's
[`InitializerExprTypeResolver.php:208–214`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Reflection/InitializerExprTypeResolver.php#L208-L214)
and [`ObjectType.php:1952–1968`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/ObjectType.php#L1952-L1968).
Public controls and actual PHPStan captures are in
`crates/pun-analyse/tests/fixtures/initializer-objects/`.

## Finite canonical initializer holders

The scanner's codebase-aware initializer query resolves a named constant selector across finite
class-string and object holders. Exact class strings read raw initializers. Object holders retain
PHPStan's final, native, PHPDoc, and overridable-untyped policies. Missing alternatives do not discard
successful reads. Heterogeneous holders use the first definite reflection only when every alternative
has the constant. The query does not load classes or evaluate declarations itself.

`ttype::initializer` retains the order that reflection observes after the existing semantic union
normalizer. Constant scalars follow other members. `preserve_initializer_union_order` keeps reference
replacements in place without changing ordinary flow expansion. Parenthesized canonical operands
retain their supplied types. Pun's initializer service resolves holder dependencies in postorder and
supplies canonical children to its existing arithmetic and array analyzer.
Trait initializer comparisons remain unordered for unions, including union-valued entries inside
ordered constant arrays.

These contracts come from PHPStan 2.3.1's
[`InitializerExprTypeResolver.php:2665–2810`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Reflection/InitializerExprTypeResolver.php#L2665-L2810),
[`UnionType.php:465–505`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/UnionType.php#L465-L505),
[`UnionType.php:588–640`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/UnionType.php#L588-L640),
and [`TypeCombinator.php:330–615`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/TypeCombinator.php#L330-L615).
Public controls and actual captures are in `crates/pun-analyse/tests/fixtures/initializer-holders/`.
Dynamic selectors retain the unsupported-expression mixed fallback. General object `::class`
accessory types, new autoloading, and additional cycle semantics remain outside this patch.

## Stack-bounded PHPDoc union construction

`builder::get_union_from_type()` evaluates union nodes with an explicit postorder work stack.
Ordinary unions still build the left operand before the right operand and apply `TUnion::from_vec()`
at each original binary node. This retains canonical ordering, never removal, and union metadata resets.

The left-null branch still precedes the right-null branch. Nullable leaves use the existing specialized
constructors. A nullable union applies `as_nullable()` only after its own operands finish, preserving
mixed handling and metadata. The patch does not flatten atoms across nullable boundaries.
Parsing and recursive lowering of non-union constructs remain unchanged. No stack size or provider
budget changes.

The public `pun-analyse/tests/phpdoc_unions.rs` regression checks an ungrouped 129-arm integer union
against PHPStan 2.3.1. Its `strlen()` result is `3`, matching the earlier grouped capture in
`tests/fixtures/strlen/`. Controls cover null positions, parentheses, nullable scalar constructors,
mixed, never, template identity, and template-default member reads. The complete fresh oracle capture
is in `crates/pun-analyse/tests/fixtures/phpdoc-unions/expected-phpstan.json`.

## Explicit mixed declaration producers

`scanner/ttype.rs` lowers a native `mixed` hint with the existing `TMixed::is_explicit` flag.
`ttype/builder.rs` does the same for PHPDoc `mixed`, `*`, and `non-empty-mixed`.
Absent types, inferred unknowns, and omitted generic parameters still use implicit mixed.
The prelude's native mixed returns, including `json_decode` and `unserialize`, use the same scanner.
`get_explicit_mixed()` is a constructor for that existing identity, not a second flag.

An unconstrained mixed member absorbs other PHPDoc union members in either order.
Any implicit mixed member clears explicitness, as in the existing flow combiner.
Other union reductions retain their previous order and metadata behavior.
Mixed array keys use `MixedType::toArrayKey()` semantics instead of retaining explicit mixed identity.
Array values keep their declared identity. This preserves PHPStan's `array<mixed>` display for
`array<mixed, mixed>` without changing the value's explicit flag.

The source contracts are PHPStan 2.3.1's
[`TypeNodeResolver`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/PhpDoc/TypeNodeResolver.php),
[`MixedType::toArrayKey()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/MixedType.php#L745-L748),
and `TypeCombinator::union()`. Public Analyzer and native-type API controls live in
`crates/pun-analyse/tests/explicit_mixed_producers.rs`. Its fixtures contain independent full and native
scope captures from PHPStan 2.3.1 and Rector's PHPStan 2.2.7.

## Impossible native intersections

`scanner/ttype.rs` returns `get_never()` when a native intersection has no intersectable alternatives.
The scanner no longer passes an empty vector to `TUnion::from_vec()`, which panics in debug builds.
Nonempty intersections retain their existing construction, including nested object intersections.
Native declaration spans and metadata flags remain unchanged. PHPDoc construction is outside this patch.

PHPStan 2.3.1's
[`TypehintHelper`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/TypehintHelper.php#L46-L57)
and [`ParserNodeTypeToPHPStanType`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/ParserNodeTypeToPHPStanType.php#L54-L67)
produce never for native intersections containing a non-object operand. `TypeCombinator` removes never
from unions with another member. The public scanner regression in
`crates/pun-analyse/tests/native_intersections.rs` checks seven native signatures directly, without panic
recovery or source rewriting. Pun no longer retries scanning with intersections masked to `mixed`.
Its fixtures capture both native reflection and parser types from PHPStan 2.3.1,
plus complete diagnostics that include a missing-function control after the impossible signatures.

## Never identity

`TUnion` retains PHPStan's implicit, explicit accepting, and explicit nonaccepting never identities in
two `UnionFlags` bits. `get_never()` remains implicit. `get_explicit_never()` and
`get_non_accepting_never()` construct the two explicit kinds. `never_kind()` identifies the kind only
on bottom unions, and `is_explicit_never()` accepts both explicit kinds. These APIs describe a type,
not whether evaluating an expression terminates.

Native and PHPDoc `never` declarations produce the nonaccepting kind, as PHPStan 2.3.1's
[`ParserNodeTypeToPHPStanType`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/ParserNodeTypeToPHPStanType.php#L97-L98)
and [`TypeNodeResolver`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/PhpDoc/TypeNodeResolver.php#L464-L477)
do. Computed impossible intersections and missing template bounds remain implicit. Union and
intersection operations preserve the participating bottom identities, including both operand orders.
Ordinary values do not retain stale bottom flags. Expansion and template substitution carry the
replacement union's identity before reducing its atoms. `clone_with_union()` preserves outer metadata
while taking the replacement's bottom identity.

Serde retains the two bits in the existing numeric `flags` field. Older records without these bits
remain implicit. The external analyzer wire format still encodes only its existing never atom and
union flags, so it loses never identity. This patch does not change the wire version or its readers.
Public type, scanner, template, conditional, and serde controls live in
`crates/pun-analyse/tests/never_identity.rs`, with independently captured PHPStan constructor,
combinator, reflection, and scope observations under `tests/fixtures/never-identity/`.

## Current-file anonymous signatures

`CodebaseMetadata::with_file_closures()` installs every closure and arrow function from a complete
current-file scan. The scan retains namespace aliases, class scopes, nested template parents,
parameters, defaults, attributes, assertions, and native and PHPDoc return declarations. The existing
signature populator resolves those types against the project's symbols. Reference attribution uses
the fresh scan's file signature and named scopes before the temporary scan is dropped.

`FunctionLikeTable` shares the project's populated map and owns file-sized replacements. Fresh entries
always win, including collisions with an old closure's key and annotation changes at an unchanged key.
Every old anonymous entry from a replaced file is hidden, even when the replacement is empty. Lookup,
iteration, equality, and serialization expose the same effective table. Mutable collection operations
explicitly materialize that table and detach it from shared snapshots.

The other metadata collections use copy-on-write backing. Creating an analysis snapshot does not copy
project declarations, symbols, signatures, ancestry maps, or patches. The snapshot owns its anonymous
metadata before `Context`, `ScopeContext`, invocation inference, or plugins borrow it. Their existing
metadata accessors and lifetimes do not change. No metadata is leaked through the syntax arena.

Body edits also move the ends of enclosing classes and functions. A file-local span map lets
`declaration_span()` identify each current scanner-selected declaration without replacing its populated
signature. Declarations owned by another file keep their original coordinates. This ownership map is
temporary analysis state, not a persisted signature. A decoded snapshot needs a fresh file scan before
analysis of edited bodies, just like a project loaded before those edits.

Pun's lint facts prepare separate current native and documented snapshots for every source revision.
The facts cache also checks the file path, PHP version, and retained project identity. Named project
declarations remain unchanged. `populate_codebase_after_body_edit()` retains its existing contract:
it populates mutable merged scans with unchanged declaration signatures, not file-local snapshots.

Public regressions are in `pun-analyse/tests/closure_freshness.rs` and
`pun-lint/tests/moved_closure.rs`. The latter preserves the supplied Rector 2.6.1 bytes in
`tests/fixtures/moved-closure/`, including its CLI configuration. The Analyzer controls check both
directions of same-key parameter and return edits, colliding positions, PHPDoc versus native input,
nested captures, reference and variadic parameters, defaults, named arguments, template parents,
project-only enum symbols, default plugins, deleted keys, persistence, and shared backing. A later
typed expression rejects partial analysis or a suppressed missing-metadata error. Native tracking
enabled and disabled must produce identical observations and diagnostics.

The user-defined raw-gather control changes native parameter types. Normal gathers infer their
callback bodies; only the recursive depth cutoff uses the native declared return. The builtin
`array_map` control changes the declared return and checks the plugin's resulting element type.

Against `e1c66a25`, four public controls fail on successful stale lookups: colliding positions and
same-key native parameters, returns, and PHPDoc parameters. The repaired native CLI applies both
rules in the supplied fixture, matches a fresh Rector 2.6.1 run byte for byte, and changes nothing
on a second run. The focused and adjacent suites pass 63 tests. Their commands are:

```sh
export CARGO_TARGET_DIR=target-closure-freshness CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=2
export RUST_TEST_THREADS=1
timeout 900 nice cargo test -p punphp-analyse -p punphp-lint --test closure_freshness --test moved_closure --test project
timeout 900 nice cargo test -p punphp-analyse --test callback_gather --test generic_defaults --test native_lane --test closure_from_callable
timeout 900 nice cargo test -p punphp-lint --test rector_types closure
timeout 900 nice cargo test -p punphp-lint --test rector_types arrow
```

Fresh profiles from the 16 focused, closure, and arrow controls cover **247/247 changed production
executable lines** against `e1c66a25`: 198 in codex, 6 in analyzer, and 43 in lint facts.
They also cover **538/538 new test executable lines**, without exclusions. The private target uses
one Cargo job and two Rayon threads. Instrument `mago-codex`, `mago-analyzer`, `mago-prelude`,
`pun-lint`, and the public test binaries. The prelude must be instrumented because its codec
instantiates the metadata's serde methods. The LCOV report uses
`cargo llvm-cov report --no-default-ignore-filename-regex --lcov` so maintained path dependencies
and test sources remain visible when intersecting the report with the added diff lines.

## Independent ASCII string casing guarantees

`TStringCasing::Both` records simultaneous `lowercase-string` and `uppercase-string` guarantees.
PHPStan compares a string with PHP's ASCII `strtolower()` and `strtoupper()`, so digits, punctuation,
empty strings, and high bytes can satisfy both. Numeric strings do not imply both: exponent syntax
can contain `e` or `E`. Literal byte storage is unchanged.

String intersections combine requirements. Union widening and concatenation retain shared guarantees.
Both string-intersection paths use `TString::intersect()`. Literal absorption checks the absence of
opposite-case ASCII letters rather than requiring every byte to be a letter. Constructors, IDs,
containment, identity comparisons, and scanner inference retain both guarantees.

This follows PHPStan 2.3.1's
[`ConstantStringType::isLowercaseString()` and `isUppercaseString()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/Constant/ConstantStringType.php#L430-L438).
Fresh public captures live in `crates/pun-analyse/tests/fixtures/string-casing/`.
`string_guarantees.rs` also checks the lattice, literal absorption, both containment directions,
IDs, equality, hashing, ordering, and serde round trips.

The combiner still widens a union of unrelated lower-only and upper-only strings to plain `string`.
PHPStan preserves that disjunction. Numeric-string's existing implied non-empty flag also remains;
the public casing control states non-emptiness explicitly to isolate the casing contract.
Concatenating two unknown non-empty strings still lacks PHPStan's non-falsy refinement. The literal-origin
casing control uses a known truthy suffix so it checks casing without relying on that separate refinement.

The public production-inference regression matches all 28 freshly captured PHPStan diagnostics,
including literal-origin numeric strings with both guarantees. Fresh LLVM profiles cover **73/73 changed
codex executable lines** against `19a99280`, without exclusions. The tests use one Cargo job and two Rayon
threads in a private target directory:

```sh
cargo llvm-cov --dep-coverage mago-codex,mago-analyzer --no-report -p punphp-analyse \
	--test string_casing --test string_guarantees --test native_lane -- --test-threads=2
```

## Decoded PHPDoc literal bytes

`ttype/builder.rs` decodes a PHPDoc string literal before interning its semantic value.
The PHPDoc parser retains the quoted source and its escapes. Previously, `"\xff"` became
the four ASCII bytes `\xff`, not the single byte `ff`. That changed both `str_repeat`'s
finite products and its aggregate byte budget.

The conversion shares `pun_tokens::php_compat::unescape_phpdoc_string()` with Pun's PHPDoc
resolver. That function ports PHPStan PhpDocParser's `StringUnescaper` using PCRE2 and
preserves bytes through quote, hexadecimal, octal, and Unicode escapes. PHP source-string
decoding is not interchangeable: PHPDoc accepts `\Xff`, retains `\$`, and permits surrogate
and extended four-byte Unicode values. The codex manifest adds the existing base crate,
not another literal store. Mago's versions and atomic types do not change.

`crates/pun-analyse/tests/str_repeat.rs` compares complete diagnostics with PHPStan 2.3.1.
Its byte fixture includes escaped PHPDoc union members with distinct invalid UTF-8 bytes,
finite repetition below the budget, fallback at the aggregate boundary, and a later dump.
The oracle script in `tests/fixtures/str-repeat/` regenerates the expectations.

Pun's `ConstantStringType` representation also stores bytes, so the conversion from codex does
not merge distinct invalid UTF-8 values. Switch comparisons preserve PHPStan's ASCII casing
queries rather than comparing decoded Unicode text with the original bytes. The level-4 control
keeps high bytes compatible with both casing accessories and rejects opposite-case ASCII letters.

The isolated `str_repeat` slice matches all 132 captured diagnostics. Its 181 Analyzer controls
and all 70 owning PHPStan tests pass. Fresh LLVM profiles cover **2/2 changed codex executable
lines**, **55/55 shared decoder lines**, and **342/342 changed production lines** against
`73ace689`, without exclusions. The runs use one Cargo job, one Rayon thread, and a private target.
That original run had an unrelated `/a/SXr` Tokens failure. Main's `025e050f` fixes the conditional
PCRE2 modifier contract; all nine Tokens compatibility controls pass in the replay below.

The casing and repetition candidates are also verified together on `a147e34b`. Five fresh PHPStan
captures match byte for byte: casing, repetition, literal bytes, never constructors, and never scopes.
All 70 PHPStan tests, 212 Analyzer tests, 19 protocol tests, and the scoped invalid-binary-operation
level gate pass. The Analyzer tests include the current native, callback, never identity, and call
termination controls. The replay preserves main's never constructors, conversion, and evaluation
facts rather than restoring older files.

Fresh private profiles cover **75/75 changed codex production lines**, **55/55 shared decoder lines**,
**441/441 combined production lines**, and **422/422 Rust test lines** against `a147e34b`, without
exclusions. The runs use one Cargo job, one Rayon thread, and `CARGO_INCREMENTAL=0`. The raw LCOV
comes from the actual replay's 55 test executables, including the maintained dependency's protocol
tests. No earlier worktree's profiles or binaries contribute to this measurement.

## Anonymous classes move with their bodies

An anonymous class's synthetic name includes its source coordinates. On `294c1de3`, moving the
class changed that name without refreshing its class or method metadata. The public Analyzer
returned `Ok` but skipped the arrow inside the class and reported a mixed assignment. The same
arrow in a named class ran once before and after the move.

`with_file_closures()` now owns current anonymous classes and their methods as well as closures.
Its file-local class map supplies the existing class-like queries, including anonymous constructor,
property, and inherited-method lookup. Those queries hide obsolete class keys. `FunctionLikeTable`
hides obsolete anonymous-class method keys during lookup, iteration, serialization, and mutation.
Named declarations still use the span remap and retain foreign-file duplicate guards.

Fresh anonymous classes use the existing hierarchy and signature populators. A temporary dependency
slice contains only their project ancestors, traits, interfaces, imported type-alias sources, and
those dependencies' methods. The slice preserves populated project metadata instead of rescanning
dependency declarations. Only anonymous scopes remain in the snapshot after population. The
project's class, symbol, alias, and signature maps stay shared. No per-file copy of the complete
project class map or additional Analyzer traversal is needed.

`closure_freshness.rs` checks unchanged and moved anonymous classes against the named-class control.
Each arrow runs once, retains its declared parameter and return type, and produces a typed call
result. Analysis also reaches a later scalar expression. Additional controls cover inherited
properties and methods, traits, interfaces, class aliases, imported scalar type aliases, current
template parents, unknown ancestry, deleted keys, persistence, and snapshot mutation. Full
observations and diagnostics remain equal with native tracking enabled and disabled. Scalar
expectations follow the fixture declarations, without a PHPStan or Rector oracle dependency.

The isolated `294c1de3` base passes 71 focused Analyzer and lint controls with one Cargo job and
one Rayon thread. Fresh source-matched profiles cover **86/86 changed production executable lines**
and **200/200 added test executable lines**, without new exclusions. The run instruments
`mago-codex`, `mago-analyzer`, and `mago-prelude` through `cargo llvm-cov --dep-coverage` and keeps
dependency and test paths visible with `report --no-default-ignore-filename-regex --lcov`.

Lint applies PHPStan's anonymous-class finality policy after both the project scan and the current-file
scan. Both native and documented replacements therefore retain the finality needed for `static::`
constant reads. This remains a lint adaptation, not a change to Mago's scanner or the fresh snapshot.
The existing `late_static_constants.php.inc` expectation stays unchanged. `moved_closure.rs` also
checks its anonymous-class section before and after a move, with the constant-return and native
string-return rules separately. Rector 2.6.1 confirms both rules' expected output on both inputs.

## Object intersection construction

`intersect_atomic_types()` returns a proven narrower operand unchanged. Its incomparable-object
constructor flattens ordinary named-object conjunctions and removes exact duplicate constraints.
Generic arguments, template bounds, and static or `$this` identities retain their metadata.
Mixed unions still intersect their excluded types. No exclusion limit or fallback is added.

`is_atomic_intersection_contained_by()` checks both the primary type and its additional conjuncts.
The combiner uses this query instead of its former shallow object comparison. Symbol-kind queries
use the symbol table's lowercase keys without changing source-spelled type names. A subsumed
intersection is removed by its full atomic ID, not its primary class name. The unused shallow
comparison wrapper is deleted.

The source contracts are PHPStan 2.3.1's
[`TypeCombinator.php:2026–2029`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/TypeCombinator.php#L2026-L2029),
[`TypeCombinator.php:2099–2160`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/TypeCombinator.php#L2099-L2160), and
[`TypeCombinator.php:751–783`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/TypeCombinator.php#L751-L783).
Public controls and fresh oracle captures are in `crates/pun-analyse/tests/fixtures/object-intersections/`.
Conditional assertion production is unchanged. The retained flow oracle also records PHPStan's
false-branch exclusions, which differ from Mago's conditional assertion transfer. The direct mixed
union regression checks the intersection contract independently of those conditional assertions.

## Named-object subtraction

`TNamedObject::subtracted_type` stores negative constraints separately from positive intersection
children. Generic arguments, `static`, and `$this` keep their existing identities. IDs and child
queries include the excluded type. Mixed intersections preserve object exclusions, and unions
consume those exclusions through the shared subtractable-type operations.

The owning contracts are PHPStan 2.3.1's
[`ObjectType.php:1776–1833`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/ObjectType.php#L1776-L1833)
and [`TypeCombinator.php:910–1033`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/TypeCombinator.php#L910-L1033).
The mixed union transfer also recognizes Mago's existing truthy and finite falsey representations.
`StringType::tryRemove('')` preserves non-empty strings when a falsey alternative rejoins an exclusion.
Public Analyser and type-query captures are in `tests/fixtures/object-intersections/` in `pun-analyse`.

## Initializer null removal

Initializer coalescing follows PHPStan 2.3.1's `TypeCombinator::removeNull()`
(`src/Type/TypeCombinator.php:209–230`): remove explicit null union members, without subtracting null
from mixed or a template bound. Ordinary flow's `to_non_nullable()` retains its stronger narrowing.
The unchanged `initializer_expressions` public regression checks unsupported-call fallback in both
a direct class constant and a nested array; a fresh PHPStan capture confirms all 19 diagnostic records.

## Incomparable string unions

The string combiner stores each nonliteral string by its complete atomic identity. A proven supertype
absorbs contained alternatives, but independent guarantees remain separate union members. In particular,
`lowercase-string|uppercase-string` is not the simultaneous lower-and-upper guarantee and does not become
plain `string`. Scalar and array-key containers remove every string alternative, regardless of its ID.
The existing literal-count threshold still generalizes an oversized finite union to `string`.

Pairwise comparison follows PHPStan 2.3.1's `TypeCombinator::compareTypesInUnion()`
(`src/Type/TypeCombinator.php:777–845`). Adding the empty string removes a pure nonempty or non-falsy
guarantee while retaining casing. Adding `'0'` downgrades a non-falsy accessory to nonempty without
discarding other guarantees. Any such widening is compared again with the retained alternatives.
The shared `TStringCasing::join()` operation still computes shared guarantees for producers such as
concatenation. It does not construct exact unions.

`pun-analyse/tests/string_casing.rs` exercises both operand orders, assignment, branch merges,
incomparable numeric, literal-origin, casing and nonempty accessories, and empty/zero-string controls.
Its complete diagnostics come from the independent PHPStan 2.3.1 capture in
`fixtures/string-casing/capture-unions.php`. Public type API tests also check absorption by scalar
containers and keep the casing-join table separate from exact union expectations.

## Numeric-string nonempty accessories

`TString::is_non_empty` remains a semantic guarantee: numeric strings are always nonempty.
`is_non_empty_implied` records when numericness supplies that guarantee without an independent
non-empty accessory. Constructors, numeric refinement, intersection, and IDs retain this distinction.
`get_string_with_props()` preserves an independently requested accessory instead of replacing the
string with the bare numeric static type. Semantic containment still uses the nonempty guarantee.

PHPStan 2.3.1's
[`AccessoryNumericStringType::isNonEmptyString()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/Accessory/AccessoryNumericStringType.php#L313-L316)
returns yes, but
[`TypeNodeResolver.php:332–341`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/PhpDoc/TypeNodeResolver.php#L332-L341)
constructs bare numeric-string without `AccessoryNonEmptyStringType`.
[`IntersectionType::equals()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/IntersectionType.php#L377-L405)
retains component identity, so `numeric-string` and `non-empty-string&numeric-string` remain distinct.

The public `string_casing` regression captures both declarations, assignments, ternary directions,
branch merges, and `is_numeric()` refinements from the real PHPStan 2.3.1 at level 7. Both numeric
forms satisfy a non-empty parameter; the empty-string argument remains an error. Existing casing,
concatenation, repeat, and native controls retain their expectations. Union normalization remains
the combiner's separate responsibility.

## Plain-object receiver unions

`scrape_type_properties()` uses full containment when named objects have different `is_static` or
`is_this` flags, just as it does for intersections and exclusions. Class ancestry alone cannot
distinguish an ordinary instance, a late-static receiver, and the current instance. The lowercase
symbol-kind lookup remains intact. The class-name shortcut still handles equal receiver identities.

PHPStan 2.3.1's `ObjectType::isSuperTypeOf()`, `StaticType::isSuperTypeOf()`, and
`ThisType::isSuperTypeOf()` define the receiver ordering. Public `TypeCombinator::union()` probes
select the ordinary class for ordinary/static and ordinary/`$this`, and static for static/`$this`,
in both operand orders. These are declared-type contracts, not expectations for `new self()`.
The existing public codex union test checks all six orders using source-spelled and lowercase names.
Its invariant generic, intersection, and exclusion controls remain unchanged.

The unchanged Rector 2.6.1 fixture `merged_static_and_trait_anonymous.php.inc` requires
`Once::instance(): self`, not `static`. Its fresh PHPStan 2.2.7-backed Rector capture confirms that
expectation. No lint rule, expected output, or gap allowance changes in this repair.

## Exact instantiation identities

`TNamedObject::is_exact` represents PHPStan's per-type `ClassReflection::asFinal()` override.
Named `new`, `new self()`, `new parent()`, and resolved literal class strings produce exact objects.
`new static()` keeps its late-static identity. The supported native named-instantiation path records
the same override. No declaration becomes final, and canonical initializer `new` identities are unchanged.

An exact parent object does not contain a child instance or a broad same-class object. A declared
parent type still contains an exact child. Static accepts an exact instance in its class hierarchy,
whereas `$this` still requires the current-instance identity. The contracts are PHPStan 2.3.1's
[`NewHandler::exactInstantiation()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Analyser/ExprHandler/NewHandler.php#L505-L533),
[`ObjectType::isSuperTypeOf()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/ObjectType.php#L640-L705),
and the existing static and `$this` supertype queries. PHPStan 2.2.7 uses the same exact-instantiation override.

The combiner retains complete named objects instead of reconstructing them from a class-name bucket.
Structural equality, hashing, ordering, and serde include exactness, including exact objects nested in
invariant generic arguments. Printed IDs remain ordinary class names. Broad generic combination retains
its existing variance policy without discarding receiver flags, exclusions, or the object's other metadata.
The unpublished protocol 2.2 named-object layout appends an exactness flag after its optional exclusion.
Real PHP worker round trips retain exactness, source casing, string casing, and nested exclusions.
Protocol 2.0 and 2.1 remain rejected before payload decoding.

`object_intersections.rs` checks exact constructor unions, same-class alternatives, declared parent
parameters, copies, reassignment, and both branch orders against a fresh PHPStan 2.3.1 capture.
It also checks exact/static/`$this` combinations, invariant generic identity, serde defaults, and full
versus native artifacts. Both unchanged Rector fluent-return and named-argument fixtures exercise this
shared metadata rather than a lint-only observation.

Boundary: Pun's separate `pun_phpstan::types::Type::Object` has no per-type final override. Conversion
to that API cannot retain exactness for later type operations. The codex and Analyzer artifacts,
in-process type handles, and complete worker snapshots retain it. Compact named-object provider
responses still describe broad objects. This patch does not add a second Pun type representation.

The new constructor fixture therefore checks the public `run_mago()` Analyzer-backed API against
the unmodified PHPStan capture, translating only PHPStan's `static(Receiver)` notation to codex's
`Receiver&static`. The existing public Pun diagnostic fixtures still run unchanged. Pun's debug-type
conversion remains an explicit boundary: it can collapse exact parent/child alternatives after
discarding the reflection override. Native exactness is checked inside a supported function scope;
top-level class declarations invalidate the deliberately partial native observation state.

Negative public union controls retain unrelated generic classes, unknown nominal types, and traits,
matching fresh PHPStan `TypeCombinator` probes in both orders. The shallow object comparator no
longer has a dead enum-container arm: its sole caller handles every enum container before calling
this helper. Enum inputs still pass through the existing named-container comparison path.

Nested exactness is structural codex metadata. PHPStan's `GenericObjectType::equals()` ignores the
reflection override, and its union can collapse `Box<exact Service>` and `Box<broad Service>` to
one object. Codex conservatively retains both internal alternatives rather than overwriting either
through a printed-ID key; their common printed ID is still `Box<Service>`. The nested-identity
control checks this metadata-retention contract, not PHPStan's internal alternative count.

## Union member order is observable metadata

`TUnion::from_vec()` retains input order while removing redundant `never` members. The native hint
scanner already supplies members in written order. Sorting those members for the equality fast path
lost that order before analysis, so later conversion could not recover it after a dynamic call
invalidated the declaration origin. Structural named-object retention made this loss visible in
`renamed_named_argument`: `OtherContract|Contract` incorrectly selected `Contract::run()`.

PHPStan 2.3.1 keeps a separate sorted description view in
[`UnionType`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/UnionType.php#L69-L77).
Its
[`getUnresolvedMethodPrototype()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/UnionType.php#L778-L804)
visits the original members, and
[`UnionTypeMethodReflection::getDeclaringClass()`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Reflection/Type/UnionTypeMethodReflection.php#L32-L35)
uses the first method. Codex now preserves that source-owned contract without changing the collector
or restoring members that refinement or reassignment removed.

Union equality and hashing remain order-independent. IDs still sort a separate list, and named-object
metadata remains structural, including exactness, receiver identity, generic arguments, and exclusions.
The union tests check both orders, redundant `never` removal, stable printed IDs, and equal hashes.
The nullable hash controls explicitly reverse their input instead of relying on constructor sorting.
The unchanged public named-argument fixtures cover written order, copies, reassignment, safe calls,
and refinement. A fresh PHPStan 2.3.1 capture matches every existing fixture expectation.

The separate Pun `Type::Object` conversion still loses exactness, as documented above. A supplementary
named-argument probe for assigned exact constructor unions produces two extra subtype warnings at
that boundary. This order repair preserves codex exactness but does not add a collector-specific fix.

## Decoded quoted PHPDoc array keys

`builder::get_array_key_from_shape_key()` decodes a quoted shape key before interning its bytes
and applying the existing `ArrayKey::from_string()` numeric conversion. The maintained PHPDoc syntax
crate retains the complete quoted token in `ShapeKey::String::raw`; bare identifiers and float
spellings have no quoted token and retain their existing values. The conversion shares
`pun_tokens::php_compat::unescape_phpdoc_string()` with scalar PHPDoc literals, preserving PHPStan's
single-quote and double-quote escape grammars without guessing the discarded quote kind.

The source contracts are PHPStan 2.3.1's bundled `TypeParser::parseArrayShapeKey()` and
`StringUnescaper::unescapeString()`, followed by `TypeNodeResolver::resolveArrayShapeOffsetType()`.
Public controls in `pun-analyse/tests/duplicate_array_keys.rs` check scanner metadata as hexadecimal
key/value pairs and compare complete duplicate-key diagnostics with a fresh PHPStan capture.
The fixtures cover distinct binary bytes, single versus double quotes, decoded-equal overwrites,
numeric and leading-zero keys, bare keys, scanner `key-of`, inline `@var`, and nested literal spreads.
Object-shape property construction, class identities, source-comment decoding, and general inference
are unchanged. Bare float spellings are parser controls, not PHPStan grammar parity claims.

Fresh profiles from the complete 12-test duplicate-key suite cover **7/7 changed production
executable lines**: three in this helper and all four PHPDoc syntax constructors, without exclusions.
The syntax changes are compared with the published 1.57.0 source rather than counting the unchanged
vendored source as new code. Instrument `mago-prelude` too: its shared generic instantiations otherwise
leave parser methods without execution counts in the report.

```sh
cargo llvm-cov --dep-coverage mago-codex,mago-analyzer,mago-phpdoc-syntax,mago-prelude \
	--no-report -p punphp-analyse --test duplicate_array_keys
cargo llvm-cov report --no-default-ignore-filename-regex --lcov
```

## Immediate duplicate class-like constants

`scanner::class_like::scan_class_like()` keeps the last immediate constant declaration's complete
metadata. This includes its source span, native and PHPDoc types, visibility, and initializer, not
only its inferred value. The CST still contains every declaration for independent duplicate and
initializer checks. Global constants and parent, trait, and interface merge precedence are unchanged.

PHPStan 2.3.1 pins BetterReflection `b79afc9c50ddf06e06b838132e4cdef09e3cff03`, whose
[`ReflectionClass::createImmediateConstants()`](https://github.com/ondrejmirtes/BetterReflection/blob/b79afc9c50ddf06e06b838132e4cdef09e3cff03/src/Reflection/ReflectionClass.php#L897-L910)
overwrites the name-keyed entry in source order for every class-like, including ordinary classes.
PHPStan's
[`DuplicateDeclarationHelper`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Rules/Classes/DuplicateDeclarationHelper.php#L26-L61)
separately checks all AST declarations. Its
[`ConflictingTraitConstantsRule`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Rules/Traits/ConflictingTraitConstantsRule.php#L67-L100)
compares each written initializer with the selected trait expression in the consuming class context.

`pun-analyse/tests/trait_constant_initializers.rs` runs the original `bug-14250.php` unchanged and
asymmetric declaration controls through the public Analyzer API. Complete PHPStan 2.3.1 captures and
their producer are under `tests/fixtures/trait-constant-initializers/`. They retain contextual file
names and every diagnostic, including native-type, visibility, and duplicate errors. The existing
trait reuse, inheritance, source-order, and initializer captures remain unchanged.

`OverridingConstantRule` in `pun-analyse/src/rules/constants.rs` resolves the selected reflection
before checking visibility, as PHPStan's `processSingleConstant()` does. Both visibility and level-3
type checks use that reflection and its declaring context. Diagnostic locations still use the CST
declaration. Prototype selection and the final-override class display are unchanged. The complementary
public, private, protected, and trait-reuse controls compare all diagnostics against a separate capture.

## Representable conditional guard differences

`TUnion::difference()` exposes the existing bounded subtraction used for mixed exclusions to
the analyzer's conditional-holder owner. Unrepresentable differences retain the incoming type,
so holder creation can reject guards that do not discriminate branches.

Boolean subtraction also returns the opposite literal for `bool` minus a literal boolean.
This follows PHPStan 2.3.1's `TypeCombinator::remove()` and `BooleanType::tryRemove()` rather
than treating the broad boolean as a discriminating guard. The public
`pun-analyse/tests/conditional_type_correlation.rs` capture checks both the false-only remainder
and the overlapping true branch. Integer-range, literal-union, array, and object guard controls
use the same API without a complete combiner or subtraction rewrite.

## Same-key array-shape discrimination

`shapes_are_discriminated()` applies PHPStan 2.3.1's constant-array reduction boundary
to shapes with identical keys as well as asymmetric keys. Equal multi-key shapes can
merge across one unrelated field, but a second unrelated field preserves their alternatives.
Singleton shapes with disjoint values also remain separate. This retains sibling-offset
correlations without forcing all ternary arrays into shape-preserving unions.

The source contract is `ConstantArrayType::legacyIsKeysSupersetOf()` and
`TypeCombinator::haveDisjointConstantScalarValues()` at PHPStan source revision
`cb3b7e99489b2e3789ee2eec10501cb7a72580ea`. The public
`pun-analyse/tests/shape_discrimination.rs` regression compares 58 complete upstream
records across operand orders, sibling reads, legitimate reductions, and ordinary unions.

## Explicit purity after stub patches

`FunctionLikeMetadata::declared_purity()` distinguishes an absent annotation from an explicit
pure or impure annotation. Stub patches retain their explicit flag mask alongside effective
flags, so an impure override remains distinguishable from an unspecified declaration.
The analyzer uses that distinction for PHPStan's receiver-local side-effect decision.
The public `receiver_invalidation` target checks both purity override directions against
fresh PHPStan 2.3.1 output for a scanned class and an analyzed caller.

## Definite constant-array intersections

`intersect_union_types()` intersects the explicit fields of two constant-array shapes
instead of rejecting every pair that needs refinements from both operands. The source
contract is PHPStan 2.3.1's `TypeCombinator::intersectDefiniteConstantArrays()` at
revision `cb3b7e99489b2e3789ee2eec10501cb7a72580ea`.

Each shape's extra key and value types apply only to the other shape's absent fields.
A missing or impossible required field rejects the intersection. An impossible optional
extra field is omitted. Both open tails intersect, while a sealed operand closes the result.
List counts, non-empty constraints, and callable conjuncts remain separate from the fields.
Generic arrays with asserted offsets retain the existing intersection path.

The public `definite_array_intersections` target compares structural type records against
the real PHPStan type API. Its capture makes closed input shapes explicitly sealed because
PHPDoc notation alone retains legacy `Maybe` sealedness. The separate
`assignment_derived_conditions` target retains complete public analyser records, including
the compound-RHS and bounded multi-consequence regressions that need this intersection.

This port does not add a representation for multiple retained array conjuncts or for legacy
unknown sealedness. The capture rejects multiple constant-array projections instead of
silently describing them as a generic array. List inputs use the public `TList` constructor
after field scanning because explicit-key list PHPDoc syntax is a separate parser boundary.

## Union construction retains void

Both shared union-combination paths preserve `void`, matching PHPStan 2.3.1's
`TypeCombinator::union`. Only eligible source-call expression results project void to null.
Raw invocation-target unions, callable-value calls and ordinary value unions cannot make that
syntax-dependent decision. PHPStan's public type API returns `int|void`, `(callable(): mixed)|void`
and `void|null` for those unions in either operand order, while `never` is absorbed.
The existing callable-origin finalization control now retains its extra void member accordingly.

The public `call_scope_observations` result-transport fixture checks named-call null values,
raw invocation-target unions, callable-value void, all-void and mixed match arms, ternaries,
coalescing and stored values against fresh pinned-PHAR captures.

Null subtraction also preserves void. `TUnion::remove_null()` follows PHPStan's
`TypeCombinator::removeNull()` explicit-null gate before subtracting null from the union.
Without an explicit null alternative, mixed and nullable template bounds remain unchanged.
The non-null narrowing backend no longer drops void as if it were null. The transport
fixture distinguishes coalescing pure void, `int|void`, `void|null` and named void-call values.

## Decimal-integer string accessories

`TString` retains PHPStan's positive and inverse decimal-integer-string accessory separately
from general numericness. Its constructors, literal query, intersections, containment, and
identity preserve that distinction and the existing literal, casing, and non-empty metadata.
PHPDoc reference resolution recognizes both accessory names before namespace resolution.

`TInteger::to_string_type()` ports `IntegerType::toString()` and
`IntegerRangeType::toString()` from PHPStan 2.3.1 at
`cb3b7e99489b2e3789ee2eec10501cb7a72580ea`. Finite intervals of width at most 128
expand inclusively without overflow. Other integer ranges retain the decimal accessory and
are non-falsy exactly when zero is excluded. General numeric strings, noncanonical numeric
formats, and the inverse accessory do not acquire this guarantee.

`pun-analyse/tests/decimal_string_producers.rs` checks the public analyser, scanner, scalar
type API, and independent native artifacts against fresh PHPStan captures. Pun converts the
stored accessory to its existing `Accessory::DecimalIntString` representation.

## Set difference preserves mixed exclusions

The shared `TUnion::difference()` path follows PHPStan 2.3.1's `TypeCombinator::remove()`
and `MixedType::tryRemove()` at `cb3b7e99489b2e3789ee2eec10501cb7a72580ea`.
Removing `mixed~U` intersects the source with `U`. Removing an ordinary type from mixed
delegates to `MixedType::subtract()`, retaining earlier exclusions and explicitness.
Error types retain their overriding subtraction behavior.

The existing public `mixed_subtraction` matrix adds 65 pinned type-API records, including
both operand orders, explicit mixed, repeated exclusions, error inputs, and explicitness.
Existing oracle records remain unchanged. Conditional formal holders use this shared
operation for their negative guards rather than adding producer-specific mixed handling.

## Callable possibility respects mixed exclusions

`can_atomic_be_callable()` follows PHPStan 2.3.1's `MixedType::isCallable()` for mixed
values: callability remains possible unless the excluded type contains every callable.
Bare objects and unknown named classes also remain potentially callable. Known final
non-invokable classes retain their negative result. Concrete callable signatures still
resolve before this possibility check.

The public `callable_visibility` regression distinguishes unknown mixed and object
invocations from an integer invocation and a negative `is_callable()` branch. It also
retains accessible, inaccessible and escaped callable signatures as independent controls.

## Opaque Closure argument compatibility

`ClosureType::accepts()` accepts bare named `Closure` objects without inventing a signature.
`CallableType::accepts()` uses their trivial acceptor, whose purity is only maybe: ordinary callable
targets accept them, while pure-callable targets do not. Named objects with callable intersections
retain the existing signature-comparison path. This follows PHPStan 2.3.1 at
`cb3b7e99489b2e3789ee2eec10501cb7a72580ea`.

`failed_method_callables` retains the bare-Closure producer results and compares compatibility,
known-signature, purity, refined-signature and invokable-object controls with the pinned capture.
The original native dynamic-literal callback test remains unchanged.

## Nonexistent parent and unnamed literal class strings

`TObject::NonexistentParent` represents PHPStan's `NonexistentParentClassType`. It displays as
`parent` but has no class name, child type, member, callable target, iterable parameter, or offset
capability. It remains truthy. Ordinary named `parent` objects retain their separate identity.
Generic `object` does not contain the sentinel: `object|parent` retains both members, and
`object&parent` is bottom. Mixed and never retain their usual containment and combination behavior.

PHPStan's `ObjectShapeType::isSuperTypeOf()` and `accepts()` accept the sentinel for empty
and optional-only shapes because it has no properties. Any required property rejects it.
The sentinel does not contain any object shape in reverse. The public union regression checks
both directions, including a shape with optional and required properties, against the pinned
type API capture in `crates/pun-phpstan/tests/fixtures/nonexistent-parent/`.

`TClassLikeString::LiteralAny` represents `class-string&literal-string` without a constant value
or object bound. It differs from both `Any` and value-bearing `Literal`. Literal-origin queries,
containment, union reduction, intersection, population, and displays preserve that distinction.
Object projection and `new<...>` yield generic object, not a fabricated `parent` class name.
Callable queries retain possibility without inventing a concrete signature.

The source contracts are PHPStan 2.3.1's
[`NonexistentParentClassType`](https://github.com/phpstan/phpstan-src/blob/cb3b7e99489b2e3789ee2eec10501cb7a72580ea/src/Type/NonexistentParentClassType.php),
`JustNullableTypeTrait`, `ObjectWithoutClassType`, `ClassStringType`, and `AccessoryLiteralStringType`.
Public type API controls in `ttype/union/tests.rs` and `ttype/display/tests.rs` check both operand
orders against pinned-PHAR observations. They include ordinary named `parent`, object, mixed,
never, class-string, literal-string, and known literal class-string controls.

Codex's cast module resolves callable targets only. Scalar conversions and the analyzer's `::class`
producer belong to analyzer consumers. PHPStan converts the sentinel to boolean true and rejects
its nonboolean scalar and array conversions. `getClassStringType()` yields plain class-string,
while `toClassConstantType()` adds literal-string provenance. Neither projection carries a name.

## Reflected method effects remain separate from source purity

`FunctionLikeMetadata::reflected_side_effects` carries an optional Yes, No or Maybe reflection
fact. Normal scanners and generated enum methods leave it absent. Pun's builtin prelude projects
the canonical PHPStan method metadata into this field before invocation analysis.
The field does not set source PHPDoc flags or change `declared_purity()`.

Serialized metadata preserves all three values. Older metadata without the field defaults to
absence, retaining the source declaration's existing policy. The public `builtin_method_effects`
regression checks scanner defaults, source purity, legacy deserialization and value round trips.
Its analyser control compares eleven complete PHPStan records, distinguishing known builtin
effects from possibly impure user methods. The source contract is PHPStan 2.3.1's
`NativeMethodReflection::hasSideEffects()` and `resources/functionMetadata.php` at
`cb3b7e99489b2e3789ee2eec10501cb7a72580ea`.

## Stub class PHPDoc replaces the original's

`ClassLikeMetadata::has_docblock` records whether a declaration has a docblock. In `apply_patch`, a patch with a
docblock replaces the original's `@mixin` tags, its template types when the patch declares any, and drops the
original's `@method` methods the patch does not declare again (`removed_pseudo_methods`; `apply_patches_pass` also
drops their function-likes). This is PHPStan 2.3.1's `StubPhpDocProvider::findClassPhpDoc()`: a stub class's PHPDoc
replaces the class's own. Larastan's stubs rely on it, for example the `Http` facade's `@mixin Factory`.

## Larastan's PHPDoc types and `$this` type arguments

`ttype/builder.rs` resolves PHPStan's `__benevolent<T>` to T with both ignore flags (Pun's benevolent union), and
the types of Larastan's TypeNodeResolverExtensions: `view-string` and `model-property<T>` are `string`,
`collection-of<TModel>` is `Illuminate\Database\Eloquent\Collection<int, TModel>`. A `$this` type argument
(`HasMany<Link, $this>`) stays `$this`, as PHPStan's ThisType does, instead of becoming `static`.

`ttype/expander.rs` `resolve_static_type()` follows `CalledOnTypeUnresolvedMethodPrototypeReflection`: on a `$this`
receiver a `$this` type stays `$this`, also in a final class; on a `static` receiver it is `static` (the object in a
final class); on an object receiver both are that object.
