# Analyse engine spike (analyse.md Build 0)

Date: 2026-10-17. Mago 1.57.0 (mago-analyzer, mago-codex), PHPStan 2.2.7, Larastan 3.10.0. Code: `crates/pun-analyse/src/mago.rs`
(Mago's analyser over a `Project`), `crates/pun-analyse/src/laravel/` (the Larastan provider), `scripts/analyse-spike.py`
(the table and the Larastan oracle), examples `mago` and `mago_fixtures`.

## Verdict

**The engine decision holds for inference, and does not hold as written for reporting.**

- **Holds:** Mago's analyser runs inside pun-analyse on Pun's own codebase (the codex the 54 level-0 rules already
  use), and a Larastan provider plugs into Mago's plugin API (`PluginRegistry::register_expression_hook`) with no
  fork. The provider reproduces Larastan's model attribute types exactly on the test fixture (75 of 75 dumps for each
  `parseModelCastsMethod` value, accessors excepted, see below).
- **Does not hold as written:** analyse.md §7 says "where a Mago issue code is the same check as a PHPStan rule, Pun
  reports it under the Pun rule". The spike shows Mago's codes are broader than PHPStan's level-0 checks, so forwarding
  them reports far too much:
  - On Pinkary, PHPStan + Larastan report **0** errors at level 0 (and at `max`). Mago reports **421** issues in 80
    files, and **226** of them carry codes that mapping.toml sends to a level-0 Pun rule (`mixed-method-access` 106,
    `non-existent-method` 76, `mixed-property-access` 18, `possibly-invalid-argument` 7, `mixed-argument` 7, ...).
  - On PHPStan's own rule fixtures, the codes mapped to remaining level-0 rules fire where PHPStan reports nothing of
    that rule: `UnusedStatement`/`UnusedFunctionCall`/`UnusedMethodCall` 963 times, `UndefinedVariable` & co. 225,
    `NonExistentProperty` 181, `MethodAccessOnNull`/`MixedMethodAccess` 100, `InvalidTypeCast` 48, ...
  - So a Mago issue can only be evidence for a Pun rule, filtered to PHPStan's semantics for that rule and preset
    (`mixed-*` are PHPStan level 9-10, `possibly-*` level 7-8). Step 3 should write each remaining level-0 rule as a Pun
    check that reads Mago's inferred types (as §7's second bullet already says for the rest), and use a Mago issue
    directly only where the table says `yes` and the extras column is empty or explained.
- **Coverage:** of the 116 Pun rules behind the remaining level-0 identifiers, Mago's analyser already makes the
  same check for **11** (`yes`), part of it for **32** (`partial`), and none for **73** (`no`). By identifier: 40 yes, 43
  partial, 185 no, of 268. Step 3 therefore writes about 105 rules, not "about 100 minus Mago's 190 codes".
- **Larastan knowledge is the main gap on Pinkary, not inference.** The 76 `non-existent-method` issues are
  `auth()->id()`/`user()`/`check()`/`logout()` on `Illuminate\Contracts\Auth\Factory` (53; Larastan's `auth()` return type
  extension), `$this->dispatch()`/`redirectRoute()` inside Livewire traits (8; PHPStan checks a trait in its using
  class), `Http::get()->ok()` on Guzzle's `PromiseInterface` (7; Larastan's facade and Http extensions), and
  `Storage::url()`/`app()->isProduction()` (6; container and facade resolution), and 2 more on `Guard` and `Blade`. The 195 `mixed-*` issues mostly come
  from the same missing return types. Every one is analyse.md §8 work (helpers, facades, container), plus PHPStan's
  trait-in-context analysis, which Pun's level-0 rules already do (138dab1).
- **Stability:** Mago 1.57.0 trips its own debug assertion (`TUnion::from_vec()` with an empty Vec) on 10 of 1,270
  fixture cases, all in typehint fixtures (the impossible `int&mixed` of `intersection-types.php` among them). `run_mago()` catches the panic and
  skips the file (tested with `tests/fixtures/mago/debug-panic.php`); release builds don't assert.
- **Speed:** a debug build of the `mago` example analyses Pinkary (its five phpstan.neon paths, with vendor in the
  codex) in 12.7-15.2 s wall, 860 MB peak RSS. That is not a benchmark (debug build, single-threaded analysis); it only
  shows the whole project fits.

## Pinkary: Mago compared with PHPStan level 0

Setup: a copy of corpus/pinkary in the scratch directory with `composer install` done (never the corpus checkout).
PHPStan: `php vendor/phpstan/phpstan/phpstan.phar analyse` with Pinkary's phpstan.neon (Larastan, bleedingEdge) at
`level: 0` and at its own `level: max`. Pun: `examples/analyse` (the 54 ported identifiers) and `examples/mago`
(Mago's analyser, with and without `--laravel`).

| Tool | Errors |
| --- | ---: |
| PHPStan + Larastan, level max | 0 |
| PHPStan + Larastan, level 0 | 8, all `ignore.unmatchedLine`/`ignore.unmatchedIdentifier` (Pinkary's `@phpstan-ignore` comments target level-max errors) |
| Pun level-0 rules | the same 8, plus 1 `class.notFound` at app/Filament/Resources/QuestionResource.php:105 (the known Pinkary diff) |
| Mago's analyser | 421 issues (226 under codes mapped to level-0 Pun rules); PHPStan reports none of them |
| Mago's analyser + Pun's Laravel plugin | the same 421: every Pinkary model has an `@property` tag for every column, and PHPStan's reflection (and so the plugin) puts declared and `@property` properties before Larastan's extension |

## The Larastan provider: model attributes from migrations

`crates/pun-analyse/src/laravel/` ports Larastan's `ModelPropertyExtension`/`ModelPropertyHelper`,
`SchemaAggregator` (every `Schema::create`/`table`/`rename`/`drop` and column method it knows, `foreignIdFor`,
morphs, `after()` groups, `if`/`else` branches) and `ModelCastHelper::getReadableType()`. Without PHP, the model
instance Larastan queries is answered from the source: `$table` (or `Str::snake(Str::pluralStudly(...))`),
`$primaryKey`, `$keyType`, `$incrementing`, `$timestamps`, `$casts`, `casts()` (with `parseModelCastsMethod`),
`SoftDeletes`, and `Date::use()`.

It is one `ExpressionHook`: after Mago types `$model->column` (through `Model::__get`, so `mixed`), the hook replaces
the type with the column's. Mago has no property-type provider for magic properties, so the hook is the only fit in
1.57's plugin API, and it was enough.

Test: `crates/pun-analyse/tests/laravel.rs`. The fixture is Pinkary's `Link` model, an untagged copy of it and of
`PollVote`, and the Pinkary migrations their tables come from, plus models and two migrations that reach every
branch. Expected: PHPStan 2.2.7 + Larastan 3.10.0's own `dumpType()` output on the fixture copied into the Pinkary
copy (`scripts/analyse-spike.py oracle`), compared after reading Mago's type back through pun-phpstan's type parser
and PHPStan's `describe()`. Result: all 75 dumps match for both `parseModelCastsMethod` values, except two accessor
attributes (`get…Attribute()`, `Attribute` methods: Larastan's `ModelAccessorExtension`, not part of this spike),
which the test lists explicitly.

Not covered by the spike: accessors, relations, `$appends`, `AsArrayObject`/`AsCollection`/`Castable` casts, the
schema dump in `database/schema/*.sql`, and Doctrine's irregular plurals (the plural rules are the regular ones).

## Remaining level-0 rules: what Mago covers

How the table is made (`scripts/analyse-spike.py table`): the remaining identifiers are the `[upstream.phpstan]`
entries of `crates/pun-config/rules/mapping.toml` with `level = "0"` that `crates/pun-analyse/src/rules/mod.rs`
`IDENTIFIERS` doesn't list (PHPStan's own `phpstanApi.*`/`phpParser.*` checks left out). Mago's analyser runs on
every case of PHPStan's rule fixtures (`tests/fixtures/rules`, 142 test classes, 1,270 cases). A PHPStan error counts
as found when Mago reports, in the same case and file, an issue whose code maps to the same Pun rule on the same line
or up to 10 lines away (PHPStan reports some checks on the class line), or an issue with a code reviewed by hand as the
same check that mapping.toml sends elsewhere ("as unmapped `code`"). Verdict per identifier: `yes` every fixture
error found; `partial` some found, or no fixture case but a Mago code maps to the rule; `no` otherwise. The last
column counts Mago issues under that rule with no PHPStan error of the rule within 10 lines.

Notes:
- Many `yes` and `partial` verdicts come only through codes mapping.toml doesn't map to those rules (`invalid-method-access`,
  `non-existent-property`, `invalid-property-write`, `incompatible-static-modifier`, ...). mapping.toml should add those
  codes as variants (a follow-up for pun-config's owner).
- `format.encoding` and `format.no-closing-tag` are level-0 PHPStan identifiers that mapping.toml sends to format
  rules; they are listed for completeness and need no analyse rule.
- `partial` with "no fixture case" means the export has no case for that identifier, so Mago's code mapping is the
  only evidence; those are unverified.
- Some `no` rules are checks Mago makes outside its analyser (mago-semantics or the linter, e.g. a parameter after a
  variadic). This spike measured mago-analyzer only.

| Pun rule | PHPStan identifier | Mago covers | Evidence | Mago codes mapped to the rule |
| --- | --- | --- | --- | --- |
| argument-by-ref | `argument.byRef` | partial | no fixture case; a Mago code maps to the rule | InvalidPassByReference (20 more where PHPStan reports none) |
| argument-type | `argument.type` | partial | no fixture case; a Mago code maps to the rule | FalseArgument, InvalidArgument, LessSpecificArgument, MixedArgument, NullArgument, PossiblyFalseArgument, PossiblyInvalidArgument, PossiblyNullArgument (1000 more where PHPStan reports none) |
| break-outside-loop | `%s.outOfLoop` | yes | 13/13 found | InvalidBreak, InvalidContinue |
| call-to-abstract-method | `%s.callToAbstract` | no | 0/8 found | - |
| class-constant-on-non-object | `classConstant.nonObject` | no | no fixture case | - |
| conflicting-flag-arguments | `argument.exclusiveConstants` | no | no fixture case | - |
|  | `filterVar.nullOnFailureAndThrowOnFailure` | no | 0/3 found |  |
| constant-type | `classConstant.type` | no | no fixture case | - |
|  | `classConstant.value` | no | 0/3 found |  |
| constructor-with-return-type | `constructor.returnType` | no | 0/3 found | - |
| deprecated-attribute | `attribute.deprecated` | no | no fixture case | - |
| deprecated-backtick | `backtick.deprecated` | no | 0/3 found | - |
| deprecated-cast | `cast.deprecated` | no | 0/4 found | - |
| disallowed-subtype | `%s.disallowedSubtype` | no | 0/5 found | - |
| duplicate-array-key | `array.duplicateKey` | partial | 14/18 found (14 within 10 lines) | DuplicateArrayKey (2 more where PHPStan reports none) |
| duplicate-closure-use | `closure.useDuplicate` | no | 0/3 found | DuplicateClosureUseVariable |
| duplicate-declaration | `%s.duplicateConstant` | no | 0/5 found | DuplicateDefinition (8 more where PHPStan reports none) |
|  | `%s.duplicateEnumCase` | no | 0/2 found |  |
|  | `%s.duplicateMethod` | no | 0/5 found |  |
|  | `%s.duplicateProperty` | no | 0/8 found |  |
| duplicate-enum-value | `enum.duplicateValue` | yes | 4/4 found (4 within 10 lines) | DuplicateEnumCaseValue |
| duplicate-named-argument | `argument.duplicate` | partial | 3/5 found | DuplicateNamedArgument, NamedArgumentOverridesPositional |
| duplicate-parameter | `parameter.duplicate` | no | 0/5 found | - |
| implicitly-nullable-parameter | `parameter.implicitlyNullable` | no | 0/47 found | - |
| inaccessible-constructor | `new.%sConstructor` | yes | 5/5 found (5 as unmapped `invalid-method-access`) | - |
| inaccessible-property | `property.notReadable` | no | 0/1 found | - |
|  | `property.notWritable` | no | 0/2 found |  |
|  | `property.private` | yes | 4/4 found (4 as unmapped `non-existent-property`) |  |
|  | `property.readWrite` | yes | 2/2 found (2 as unmapped `incompatible-readonly-modifier`) |  |
|  | `property.writeOnly` | no | 0/6 found |  |
|  | `staticProperty.private` | yes | 1/1 found (1 as unmapped `non-existent-property`) |  |
| incompatible-override | `classConstant.missingNativeType` | no | 0/1 found | IncompatibleConstantOverride, IncompatibleConstantType, IncompatibleConstantVisibility, IncompatibleParameterCount, IncompatibleParameterType, IncompatiblePropertyOverride, IncompatiblePropertyReadonly, IncompatiblePropertyStatic, IncompatiblePropertyType, IncompatiblePropertyVisibility, IncompatibleReadonlyModifier, IncompatibleReturnType, IncompatibleStaticModifier, IncompatibleVisibility (32 more where PHPStan reports none) |
|  | `classConstant.nativeType` | no | 0/2 found |  |
|  | `method.childParameterType` | partial | 19/21 found (2 within 10 lines) |  |
|  | `method.childReturnType` | partial | 19/23 found (1 within 10 lines) |  |
|  | `method.visibility` | partial | 7/11 found |  |
|  | `parameter.byRef` | yes | 2/2 found (2 within 10 lines) |  |
|  | `parameter.missing` | partial | 6/13 found |  |
|  | `parameter.notByRef` | yes | 2/2 found (2 within 10 lines) |  |
|  | `parameter.notOptional` | yes | 10/10 found |  |
|  | `parameter.notVariadic` | partial | 2/4 found (1 within 10 lines) |  |
|  | `parameter.variadic` | partial | no fixture case; a Mago code maps to the rule |  |
|  | `property.extraNativeType` | yes | 1/1 found |  |
|  | `property.missingNativeType` | yes | 1/1 found |  |
|  | `property.nativeType` | partial | 1/4 found |  |
|  | `property.readOnly` | partial | 2/3 found |  |
|  | `property.visibility` | yes | 5/5 found (5 as unmapped `incompatible-property-access`) |  |
| invalid-argument | `argument.%s` | no | 0/40 found | - |
| invalid-argument-order | `argument.nonUnpackAfterUnpacked` | no | 0/1 found | - |
|  | `argument.positionalAfterNamed` | no | 0/5 found |  |
|  | `argument.unpackAfterNamed` | no | 0/1 found |  |
| invalid-assignment-target | `assign.invalidExpr` | yes | 2/2 found (2 as unmapped `invalid-assignment`) | - |
| invalid-attribute | `attribute.%s` | partial | 2/5 found (2 within 10 lines) | AbstractClassUsedAsAttribute, AttributeNotRepeatable, ClassNotMarkedAsAttribute, InvalidAttributeTarget, NonClassUsedAsAttribute (1 more where PHPStan reports none) |
|  | `attribute.abstract` | partial | 3/6 found (2 within 10 lines) |  |
|  | `attribute.constructorNotPublic` | no | 0/4 found |  |
|  | `attribute.noConstructor` | yes | 2/2 found (2 within 10 lines) |  |
|  | `attribute.nonRepeatable` | yes | 1/1 found |  |
|  | `attribute.notAttribute` | yes | 4/4 found |  |
|  | `attribute.target` | partial | 13/31 found |  |
|  | `trait.allowDynamicProperties` | no | 0/1 found |  |
| invalid-bitmask-argument | `argument.bitmaskNotAllowed` | no | no fixture case | - |
| invalid-cast | `cast.unset` | no | 0/2 found | InvalidTypeCast (48 more where PHPStan reports none) |
| invalid-class-constant | `classConstant.%s` | no | 0/6 found | - |
|  | `classConstant.class` | no | 0/2 found |  |
|  | `classConstant.dynamicString` | no | no fixture case |  |
|  | `classConstant.final` | partial | 2/4 found (2 as unmapped `override-final-constant`) |  |
|  | `classConstant.finalPrivate` | no | 0/1 found |  |
|  | `classConstant.nameNotString` | no | no fixture case |  |
|  | `classConstant.nameType` | no | no fixture case |  |
|  | `classConstant.nonFinal` | no | 0/1 found |  |
|  | `classConstant.visibility` | no | 0/11 found |  |
| invalid-closure-use | `closure.useSuperGlobal` | no | 0/18 found | - |
| invalid-constant-argument | `argument.invalidConstant` | no | no fixture case | - |
| invalid-declare-strict-types | `declareStrictTypes.notFirst` | no | 0/3 found | - |
|  | `declareStrictTypes.value` | no | 0/2 found |  |
| invalid-deprecated-attribute | `%s.deprecatedAttribute` | no | 0/3 found | - |
|  | `trait.deprecatedAttribute` | no | no fixture case |  |
| invalid-enum | `enum.backingType` | no | 0/2 found | InvalidEnumCaseValue |
|  | `enum.caseOutsideOfEnum` | no | 0/3 found |  |
|  | `enum.caseType` | yes | 2/2 found |  |
|  | `enum.caseWithValue` | yes | 7/7 found |  |
|  | `enum.constructor` | no | 0/1 found |  |
|  | `enum.destructor` | no | 0/1 found |  |
|  | `enum.magicMethod` | no | 0/4 found |  |
|  | `enum.methodRedeclaration` | no | 0/12 found |  |
| invalid-expression | `%s.expr` | no | 0/2 found | - |
| invalid-inheritance | `class.extendsFinal` | yes | 4/4 found | ExtendFinalClass, InvalidExtend, InvalidImplement, InvalidTraitUse, OverrideFinalConstant, OverrideFinalMethod, OverrideFinalProperty (29 more where PHPStan reports none) |
|  | `method.parentMethodFinal` | yes | 6/6 found |  |
|  | `property.parentPropertyFinal` | yes | 4/4 found (2 within 10 lines) |  |
|  | `traitUse.class` | partial | no fixture case; a Mago code maps to the rule |  |
|  | `traitUse.enum` | yes | 2/2 found |  |
|  | `traitUse.interface` | yes | 1/1 found |  |
| invalid-method | `method.%s` | no | no fixture case | - |
| invalid-method-declaration | `method.abstract` | partial | 106/113 found (106 as unmapped `unimplemented-abstract-method`) | - |
|  | `method.abstractOverridingNonAbstract` | no | 0/5 found |  |
|  | `method.abstractPrivate` | no | 0/3 found |  |
|  | `method.finalPrivate` | no | 0/3 found |  |
|  | `method.nonAbstract` | no | 0/2 found |  |
| invalid-nullable-type | `nullableType.%s` | no | 0/5 found | - |
| invalid-nullsafe | `nullsafe.assign` | no | 0/5 found | - |
|  | `nullsafe.byRef` | no | 0/7 found |  |
| invalid-offset | `offsetAccess.noDim` | yes | 16/16 found (16 as unmapped `array-append-in-read-context`) | InvalidArrayIndex, MixedArrayIndex, NullArrayIndex, PossiblyNullArrayIndex (4 more where PHPStan reports none) |
| invalid-override-attribute | `method.override` | no | 0/3 found | InvalidOverrideAttribute |
|  | `property.override` | no | 0/26 found |  |
| invalid-parameter-name | `parameter.superglobal` | no | 0/9 found | - |
| invalid-pipe | `pipe.byRef` | no | no fixture case | - |
| invalid-property | `property.%s` | no | no fixture case | - |
|  | `property.nameNotString` | no | no fixture case |  |
| invalid-property-assignment | `assign.property%s` | yes | 1/1 found (1 as unmapped `invalid-property-write`) | - |
|  | `property.assignByRef` | no | 0/7 found |  |
| invalid-property-declaration | `property.abstract` | no | 0/2 found | - |
|  | `property.abstractFinal` | no | 0/1 found |  |
|  | `property.abstractFinalHook` | no | 0/2 found |  |
|  | `property.abstractInInterface` | no | 0/1 found |  |
|  | `property.abstractNonHooked` | no | 0/2 found |  |
|  | `property.abstractOverridingNonAbstractHook` | no | 0/4 found |  |
|  | `property.abstractPrivate` | no | 0/1 found |  |
|  | `property.abstractWithoutAbstractHook` | no | 0/2 found |  |
|  | `property.callableType` | no | 0/4 found |  |
|  | `property.final` | no | no fixture case |  |
|  | `property.finalInInterface` | no | 0/3 found |  |
|  | `property.finalPrivate` | no | 0/2 found |  |
|  | `property.finalPrivateHook` | no | 0/1 found |  |
|  | `property.hookBodyInInterface` | no | 0/2 found |  |
|  | `property.hookReadOnly` | no | 0/5 found |  |
|  | `property.hookWithoutBody` | no | 0/8 found |  |
|  | `property.hookedStatic` | no | 0/5 found |  |
|  | `property.inInterface` | no | no fixture case |  |
|  | `property.invalidPromoted` | no | 0/15 found |  |
|  | `property.nonHookedInInterface` | no | 0/5 found |  |
|  | `property.nonPublicInInterface` | no | 0/2 found |  |
|  | `property.readOnlyByPhpDocDefaultValue` | no | 0/5 found |  |
|  | `property.readOnlyDefaultValue` | no | 0/5 found |  |
|  | `property.readOnlyInInterface` | no | 0/3 found |  |
|  | `property.readOnlyNoNativeType` | no | 0/5 found |  |
|  | `property.readOnlyStatic` | no | 0/4 found |  |
|  | `property.virtualDefault` | no | 0/1 found |  |
| invalid-property-hook | `propertySetHook.nativeParameterType` | partial | 5/6 found (5 as unmapped `incompatible-property-hook-parameter-type`) | - |
|  | `propertySetHook.parameterType` | no | no fixture case |  |
| invalid-regex | `regexp.pattern` | no | 0/29 found | - |
| invalid-sealed-tag | `sealed.onTrait` | no | 0/1 found | - |
| invalid-static-call | `staticMethod.%s` | no | 0/8 found | - |
| invalid-static-property | `staticProperty.%s` | no | 0/3 found | - |
| invalid-this | `assign.this` | partial | 29/32 found (2 within 10 lines) | AssignmentToThis (1 more where PHPStan reports none) |
|  | `closure.useThis` | no | 0/2 found |  |
|  | `global.this` | no | 0/3 found |  |
|  | `parameter.this` | no | 0/1 found |  |
|  | `static.this` | no | 0/3 found |  |
| invalid-type | `%s.type` | no | no fixture case | - |
| invalid-type-alias | `typeAlias.circular` | no | 0/15 found | - |
|  | `typeAlias.duplicate` | no | 0/9 found |  |
|  | `typeAlias.invalidName` | no | 0/6 found |  |
|  | `typeAlias.invalidType` | no | 0/3 found |  |
| invalid-union-type | `unionType.%s` | no | 0/5 found | - |
| invalid-unset | `unset.offset` | partial | 3/4 found (2 within 10 lines) | InvalidUnset (19 more where PHPStan reports none) |
|  | `unset.variable` | partial | 1/3 found (1 within 10 lines) |  |
| invalid-void-cast | `cast.void` | no | 0/3 found | - |
| magic-constant-out-of-context | `magicConstant.outOfClass` | no | 0/9 found | - |
|  | `magicConstant.outOfFunction` | no | 0/6 found |  |
|  | `magicConstant.outOfNamespace` | no | 0/5 found |  |
|  | `magicConstant.outOfTrait` | no | 0/14 found |  |
| method-call-on-non-object | `method.nonObject` | partial | no fixture case; a Mago code maps to the rule | MethodAccessOnNull, MixedMethodAccess, PossibleMethodAccessOnNull (100 more where PHPStan reports none) |
| missing-argument | `argument.missing` | no | 0/16 found | - |
| missing-enum-case | `enum.missingCase` | no | 0/2 found | - |
| missing-override-attribute | `method.missingOverride` | partial | no fixture case; a Mago code maps to the rule | MissingOverrideAttribute |
| missing-property-override-attribute | `property.missingOverride` | no | no fixture case | - |
| missing-return | `return.missing` | partial | 278/321 found (115 within 10 lines) | MissingReturnStatement (56 more where PHPStan reports none) |
| missing-serialize-methods | `class.serializable` | no | 0/2 found | - |
|  | `enum.serializable` | no | 0/2 found |  |
| missing-tentative-return-type | `method.tentativeReturnType` | no | 0/36 found | - |
| name-already-in-use | `%s.nameInUse` | no | 0/5 found | NameAlreadyInUse |
|  | `use.nameInUse` | no | 0/3 found |  |
| name-case-mismatch | `function.nameCase` | partial | no fixture case; a Mago code maps to the rule | IncorrectClassLikeCasing, IncorrectFunctionCasing |
|  | `method.nameCase` | partial | no fixture case; a Mago code maps to the rule |  |
|  | `staticMethod.nameCase` | partial | no fixture case; a Mago code maps to the rule |  |
| named-argument-not-allowed | `argument.named` | partial | no fixture case; a Mago code maps to the rule | NamedArgumentNotAllowed (2 more where PHPStan reports none) |
| new-on-non-object | `new.nonObject` | no | no fixture case | - |
| not-callable | `callable.nonCallable` | partial | no fixture case; a Mago code maps to the rule | InvalidCallable (11 more where PHPStan reports none) |
| php-version-too-low | `argument.namedNotSupported` | no | 0/1 found | - |
|  | `callable.notSupported` | no | 0/2 found |  |
|  | `cast.voidNotSupported` | no | no fixture case |  |
|  | `catch.nonCapturingNotSupported` | no | no fixture case |  |
|  | `classConstant.dynamicFetch` | no | no fixture case |  |
|  | `classConstant.finalNotSupported` | no | no fixture case |  |
|  | `classConstant.inTrait` | no | no fixture case |  |
|  | `classConstant.nativeTypeNotSupported` | no | no fixture case |  |
|  | `classConstant.notSupported` | no | no fixture case |  |
|  | `constant.attributesNotSupported` | no | no fixture case |  |
|  | `constant.attributesRuleCannotRun` | no | no fixture case |  |
|  | `parameter.unionTypeNotSupported` | no | no fixture case |  |
|  | `property.hooksNotSupported` | no | no fixture case |  |
|  | `property.overrideAttribute` | no | no fixture case |  |
|  | `property.promotedNotSupported` | no | no fixture case |  |
|  | `property.readOnlyNotSupported` | no | no fixture case |  |
|  | `property.staticAsymmetricVisibility` | no | no fixture case |  |
|  | `return.neverTypeNotSupported` | no | no fixture case |  |
|  | `return.unionTypeNotSupported` | no | no fixture case |  |
|  | `throw.notSupported` | no | no fixture case |  |
| phpdoc-final-violation | `class.extendsFinalByPhpDoc` | yes | 4/4 found (4 as unmapped `extend-final-class`) | - |
|  | `method.parentMethodFinalByPhpDoc` | no | 0/3 found |  |
|  | `property.parentPropertyFinalByPhpDoc` | no | 0/2 found |  |
| phpdoc-type-mismatch | `property.phpDocType` | partial | no fixture case; a Mago code maps to the rule | DocblockTypeMismatch (27 more where PHPStan reports none) |
| private-consistent-constructor | `consistentConstructor.private` | no | 0/1 found | - |
| property-fetch-on-non-object | `property.nonObject` | partial | no fixture case; a Mago code maps to the rule | InvalidPropertyAccess, MixedPropertyAccess, NullPropertyAccess, PossiblyNullPropertyAccess (138 more where PHPStan reports none) |
| readonly-class-mismatch | `class.nonReadOnly` | yes | 2/2 found (2 as unmapped `invalid-extend`) | - |
|  | `class.readOnly` | yes | 2/2 found (2 as unmapped `invalid-extend`) |  |
| readonly-property-write | `assign.propertyReadOnly` | yes | 9/9 found (9 as unmapped `invalid-property-write`) | - |
|  | `assign.readOnlyProperty` | partial | 10/12 found (10 as unmapped `invalid-property-write`) |  |
|  | `assign.readOnlyPropertyByPhpDoc` | partial | 4/6 found (4 as unmapped `invalid-property-write`) |  |
| renamed-parameter-in-subtype | `argument.parameterRenamedInSubtype` | partial | no fixture case; a Mago code maps to the rule | IncompatibleParameterName (14 more where PHPStan reports none) |
| required-after-optional | `parameter.requiredAfterOptional` | no | 0/193 found | - |
| return-in-never-function | `return.never` | no | 0/6 found | NeverReturn (3 more where PHPStan reports none) |
| self-outside-class | `outOfClass.%s` | partial | no fixture case; a Mago code maps to the rule | ParentOutsideClassScope, SelfOutsideClassScope, StaticOutsideClassScope (12 more where PHPStan reports none) |
| static-access-mismatch | `method.nonStatic` | yes | 3/3 found (3 as unmapped `incompatible-static-modifier`) | InvalidStaticMethodCall, InvalidStaticPropertyAccess (1 more where PHPStan reports none) |
|  | `method.static` | yes | 3/3 found (3 as unmapped `incompatible-static-modifier`) |  |
|  | `property.nonStatic` | yes | 2/2 found (2 as unmapped `incompatible-static-modifier`) |  |
|  | `property.static` | yes | 2/2 found (2 as unmapped `incompatible-static-modifier`) |  |
|  | `property.staticAccess` | partial | 10/11 found (2 within 10 lines) |  |
|  | `staticProperty.nonStaticAccess` | no | 0/8 found |  |
| static-call-on-non-object | `staticMethod.nonObject` | no | no fixture case | - |
| static-closure-not-allowed | `argument.staticClosure` | no | no fixture case | - |
| static-property-assign-type | `assign.staticProperty%s` | no | 0/3 found | - |
| static-property-on-non-object | `staticProperty.nonObject` | no | no fixture case | - |
| trait-in-interface | `interface.traitUse` | no | 0/1 found | - |
| trait-used-as-type | `attribute.trait` | no | 0/3 found | TraitInstantiation (1 more where PHPStan reports none) |
|  | `instanceof.trait` | no | 0/1 found |  |
|  | `property.trait` | no | 0/1 found |  |
|  | `selfOut.trait` | no | 0/1 found |  |
|  | `typeAlias.trait` | no | 0/1 found |  |
| unanalysed-inner-function | `function.inner` | no | 0/3 found | - |
| unbound-template-type | `method.templateTypeNotInParameter` | no | 0/10 found | - |
| undefined-attribute | `attribute.notFound` | yes | 14/14 found | NonExistentAttributeClass |
| undefined-constant | `constant.notFound` | no | 0/3 found | NonExistentConstant (5 more where PHPStan reports none) |
| undefined-goto-label | `goto.labelUndefined` | no | 0/7 found | - |
| undefined-named-argument | `argument.unknown` | yes | 14/14 found (1 within 10 lines) | InvalidNamedArgument (1 more where PHPStan reports none) |
| undefined-parent | `parameter.noParent` | no | 0/2 found | - |
|  | `return.noParent` | no | 0/2 found |  |
| undefined-property | `property.notFound` | yes | 27/27 found (4 within 10 lines) | NonExistentProperty, PossiblyNonExistentProperty (181 more where PHPStan reports none) |
| undefined-static-property | `staticProperty.notFound` | partial | 42/43 found (42 as unmapped `non-existent-property`) | - |
| undefined-trait | `trait.notFound` | yes | 2/2 found (2 as unmapped `non-existent-class-like`) | - |
| undefined-type-alias | `typeAlias.notFound` | no | 0/3 found | - |
| undefined-variable | `variable.undefined` | partial | 111/117 found (10 within 10 lines) | PossiblyUndefinedVariable, ReferenceToUndefinedVariable, UndefinedVariable, UndefinedVariableInClosureUse (225 more where PHPStan reports none) |
| uninitialized-property | `property.uninitializedReadonly` | no | 0/53 found | MissingConstructor, UninitializedProperty |
|  | `property.uninitializedReadonlyByPhpDoc` | no | 0/19 found |  |
| unpack-non-iterable | `argument.unpackNonIterable` | no | no fixture case | - |
| unresolvable-native-type | `parameter.unresolvableNativeType` | no | 0/17 found | - |
|  | `property.unresolvableNativeType` | no | 0/4 found |  |
|  | `return.unresolvableNativeType` | no | 0/16 found |  |
| unresolvable-template-type | `argument.templateType` | no | no fixture case | - |
| unresolvable-type | `%s.unresolvableReturnType` | no | no fixture case | - |
|  | `argument.unresolvableType` | no | no fixture case |  |
|  | `typeAlias.unresolvableType` | no | 0/2 found |  |
| unsafe-new-static | `new.staticInAbstractClassStaticMethod` | partial | no fixture case; a Mago code maps to the rule | UnsafeInstantiation (51 more where PHPStan reports none) |
| unset-hooked-property | `unset.hookedProperty` | yes | 11/11 found (11 as unmapped `invalid-unset`) | - |
|  | `unset.possiblyHookedProperty` | yes | 8/8 found (8 as unmapped `invalid-unset`) |  |
| unused-argument | `argument.unused` | no | 0/1 found | - |
| unused-result | `callable.resultDiscarded` | yes | 4/4 found | UnusedFunctionCall, UnusedMethodCall, UnusedStatement (963 more where PHPStan reports none) |
|  | `function.resultDiscarded` | partial | 2/4 found |  |
|  | `method.resultDiscarded` | partial | no fixture case; a Mago code maps to the rule |  |
|  | `staticMethod.resultDiscarded` | partial | 2/4 found |  |
| useless-void-cast | `callable.inVoidCast` | no | 0/1 found | - |
|  | `function.inVoidCast` | no | 0/3 found |  |
|  | `method.inVoidCast` | no | no fixture case |  |
|  | `staticMethod.inVoidCast` | no | 0/3 found |  |
| variadic-not-last | `parameter.variadicNotLast` | no | 0/3 found | - |
| void-result-used | `%s.void` | no | 0/42 found | - |
| format.encoding | `whitespace.bom` | no | no fixture case | - |
| format.no-closing-tag | `whitespace.fileEnd` | no | no fixture case | - |

Totals: 116 rules (11 yes, 32 partial, 73 no); 268 identifiers (40 yes, 43 partial, 185 no).
