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
`crates/pun-analyse/tests/native_intersections.rs` checks seven native signatures before Pun's
masked-source recovery. Its fixtures capture both native reflection and parser types from PHPStan 2.3.1,
plus complete diagnostics that include a missing-function control after the impossible signatures.
