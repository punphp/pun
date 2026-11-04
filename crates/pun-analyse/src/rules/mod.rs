//! PHPStan's level-0 rules. One walk per analysed file; each `walk_in_*` hook is the `processNode()` of the rules
//! registered for that node type, in PHPStan's source order.

mod anonymous;
mod arrays;
mod assign;
mod attributes;
mod calls;
mod class_names;
mod classes;
mod by_ref_foreach;
mod constants;
mod date_interval;
mod date_time;
mod dead_code;
mod declaration_effects;
mod declaration_purity;
mod purity_points;
mod debug;
mod declarations;
mod declared;
mod deprecations;
mod dynamic_fetch;
mod doc;
mod driver;
mod enums;
mod function_callable;
mod function_checks;
mod functions;
mod generics;
mod hierarchy;
mod include_file;
mod infer;
mod internal;
mod internal_usage;
mod invalid_throws;
mod larastan;
mod members;
mod method_callable;
mod method_reflection;
mod missing_types;
mod named_argument;
mod names;
mod new_static_abstract;
mod overriding;
mod overriding_property;
mod params;
mod printf_helper;
mod properties;
mod readonly_init;
mod property_access;
mod property_hooks;
mod require_ancestors;
mod reflection;
mod restricted_functions;
mod scope;
mod selector;
mod set_hook;
mod signatures;
mod statements;
mod strict;
mod syntax;
mod through_static;
mod type_aliases;
mod types;
mod unused_trait;
mod write_only;
mod variables;
mod unset;
mod phpdoc_tags;
mod require_tags;
mod var_tags;
mod callable_tags;
mod class_tags;
mod operators;
mod conditional_return;
mod assert_tags;
mod phpdoc_types;
mod var_names;
mod variance;
mod bind_scope;
mod return_types;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::symbol::SymbolKind;
use mago_codex::ttype::union::TUnion;
use mago_database::file::{File, FileId};
use mago_names::ResolvedNames;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use pun_phpstan::ignore::Error;

use crate::codebase::Sources;
use scope::{Guard, Saved};
use statements::{Frame, FrameKind};

pub(crate) use doc::{doc_before, tags as doc_tags};
pub(crate) use driver::{Checks, check_file};
pub use larastan::Larastan;
pub(crate) use dead_code::conditions_in_traits;
pub(crate) use dead_code::impure_points;
pub(crate) use named_argument::resolve as renamed_named_arguments;
pub(crate) use larastan::finish as larastan_finish;
pub(crate) use reflection::builtins;
pub(crate) use unused_trait::unused_traits;

pub fn reports(identifier: &str) -> bool {
    IDENTIFIERS.contains(&identifier)
        || declaration_purity::IDENTIFIERS.contains(&identifier)
        || missing_types::IDENTIFIERS.contains(&identifier)
        || generics::IDENTIFIERS.contains(&identifier)
        || larastan::IDENTIFIERS.contains(&identifier)
        || types::IDENTIFIERS.iter().any(|ids| ids.contains(&identifier))
        || phpdoc_tags::IDENTIFIERS.contains(&identifier)
        || require_tags::IDENTIFIERS.contains(&identifier)
        || var_tags::IDENTIFIERS.contains(&identifier)
        || class_tags::IDENTIFIERS.contains(&identifier)
        || callable_tags::IDENTIFIERS.contains(&identifier)
        || operators::IDENTIFIERS.contains(&identifier)
        || conditional_return::IDENTIFIERS.contains(&identifier)
        || assert_tags::IDENTIFIERS.contains(&identifier)
        || phpdoc_types::IDENTIFIERS.contains(&identifier)
        || invalid_throws::IDENTIFIERS.contains(&identifier)
        || var_names::IDENTIFIERS.contains(&identifier)
        || variance::IDENTIFIERS.contains(&identifier)
        || date_interval::IDENTIFIERS.contains(&identifier)
        || date_time::IDENTIFIERS.contains(&identifier)
        || by_ref_foreach::IDENTIFIERS.contains(&identifier)
}

pub const IDENTIFIERS: &[&str] = &[
    // the throw point rules switched on by `exceptions.check.*` (MissingCheckedExceptionIn*ThrowsRule,
    // MethodThrowTypeCovarianceRule)
    "missingType.checkedException",
    "throws.notCovariant",
    "throws.notCovariantWithImplicitVoid",
    "throws.shouldBeVoid",
    "argument.parameterRenamedInSubtype",
    "include.fileNotFound",
    "includeOnce.fileNotFound",
    "require.fileNotFound",
    "requireOnce.fileNotFound",
    "variable.undefined",
    "variable.nameNotString",
    "unset.variable",
    "unset.offset",
    "unset.readOnlyProperty",
    "unset.readOnlyPropertyByPhpDoc",
    "unset.hookedProperty",
    "unset.possiblyHookedProperty",
    "phpstan.dumpType",
    "phpstan.dumpPhpDocType",
    "phpstan.type",
    "phpstan.nativeType",
    "phpstan.superType",
    "phpstan.variable",
    "phpstan.unknownExpectation",
    "pipe.byRef",
    "typeAlias.circular",
    "typeAlias.duplicate",
    "typeAlias.invalidName",
    "typeAlias.invalidType",
    "typeAlias.notFound",
    "typeAlias.trait",
    "typeAlias.unresolvableType",
    "generics.notGeneric",
    "argument.unused",
    "callable.inVoidCast",
    "callable.resultDiscarded",
    "closure.unusedUse",
    "constructor.unusedParameter",
    "constructor.unusedParameterFlow",
    "closure.unusedUseFlow",
    "function.unusedParameter",
    "function.unusedParameterFlow",
    "method.unusedParameter",
    "method.unusedParameterFlow",
    "finally.exitPoint",
    "function.inVoidCast",
    "function.resultDiscarded",
    "function.uselessReturnValue",
    "method.inVoidCast",
    "method.resultDiscarded",
    "staticMethod.inVoidCast",
    "staticMethod.resultDiscarded",
    "function.notFound",
    "function.nameCase",
    "argument.printf",
    "argument.sprintf",
    "argument.sscanf",
    "argument.fscanf",
    "argument.vprintf",
    "argument.vsprintf",
    "class.notFound",
    "new.static",
    "new.nonObject",
    "new.trait",
    "new.enum",
    "new.interface",
    "new.abstract",
    "new.noConstructor",
    "new.privateConstructor",
    "new.protectedConstructor",
    "arguments.count",
    "function.void",
    "method.void",
    "staticMethod.void",
    "method.notFound",
    "method.private",
    "method.protected",
    "staticMethod.notFound",
    "staticMethod.nameCase",
    "method.staticCall",
    "staticMethod.private",
    "staticMethod.protected",
    "method.callToAbstract",
    "staticMethod.callToAbstract",
    "outOfClass.self",
    "outOfClass.static",
    "outOfClass.parent",
    "class.noParent",
    "classConstant.notFound",
    "classConstant.onTrait",
    "classConstant.private",
    "classConstant.protected",
    "classConstant.nonObject",
    "classConstant.notSupported",
    "classConstant.dynamicString",
    "catch.notThrowable",
    "parameter.trait",
    "return.trait",
    "interface.notFound",
    "class.extendsInterface",
    "class.extendsTrait",
    "class.extendsEnum",
    "interfaceExtends.class",
    "interfaceExtends.trait",
    "interfaceExtends.enum",
    "classImplements.class",
    "classImplements.trait",
    "classImplements.enum",
    "enumImplements.class",
    "enumImplements.trait",
    "enumImplements.enum",
    "parameter.duplicate",
    "parameter.variadicNotLast",
    "parameter.superglobal",
    "parameter.this",
    "closure.useDuplicate",
    "closure.useSuperGlobal",
    "closure.useThis",
    "function.inner",
    "magicConstant.outOfClass",
    "magicConstant.outOfFunction",
    "magicConstant.outOfNamespace",
    "magicConstant.outOfTrait",
    "declareStrictTypes.notFirst",
    "declareStrictTypes.value",
    "goto.labelUndefined",
    "label.unused",
    "trait.unused",
    "constructor.missingParentCall",
    "propertyGetHook.noRead",
    "propertySetHook.noAssign",
    "staticMethod.dynamicCall",
    "cast.useless",
    "switch.type",
    "method.nameCase",
    "constant.notFound",
    "class.missingExtends",
    "class.missingImplements",
    "property.uninitialized",
    "filterVar.nullOnFailureAndThrowOnFailure",
    "argument.missing",
    "argument.unknown",
    "argument.duplicate",
    "argument.positionalAfterNamed",
    "argument.unpackAfterNamed",
    "argument.nonUnpackAfterUnpacked",
    "argument.namedNotSupported",
    "constructor.returnType",
    "cast.deprecated",
    "cast.unset",
    "cast.void",
    "cast.voidNotSupported",
    "backtick.deprecated",
    "break.outOfLoop",
    "continue.outOfLoop",
    "offsetAccess.noDim",
    "nullsafe.assign",
    "nullsafe.byRef",
    "assign.invalidExpr",
    "assign.this",
    "global.this",
    "static.this",
    "class.nameInUse",
    "interface.nameInUse",
    "trait.nameInUse",
    "enum.nameInUse",
    "use.nameInUse",
    "unionType.mixed",
    "unionType.never",
    "unionType.void",
    "nullableType.mixed",
    "nullableType.never",
    "nullableType.void",
    "ignore.unmatchedLine",
    "ignore.unmatchedIdentifier",
    "ignore.parseError",
    "ignore.noComment",
    "ignore.unmatched",
    "ignore.count",
    "ignore.allLineErrors",
    "phpstan.parse",
    "enum.constructor",
    "enum.destructor",
    "enum.magicMethod",
    "enum.methodRedeclaration",
    "enum.backingType",
    "enum.serializable",
    "enum.caseWithValue",
    "enum.missingCase",
    "enum.caseType",
    "enum.duplicateValue",
    "enum.caseOutsideOfEnum",
    "class.duplicateConstant",
    "class.duplicateProperty",
    "class.duplicateMethod",
    "class.duplicateEnumCase",
    "interface.duplicateConstant",
    "interface.duplicateProperty",
    "interface.duplicateMethod",
    "interface.duplicateEnumCase",
    "enum.duplicateConstant",
    "enum.duplicateProperty",
    "enum.duplicateMethod",
    "enum.duplicateEnumCase",
    "method.abstractPrivate",
    "method.finalPrivate",
    "classConstant.finalPrivate",
    "property.inInterface",
    "property.nonHookedInInterface",
    "property.nonPublicInInterface",
    "property.readOnlyInInterface",
    "property.hookedStatic",
    "property.abstractInInterface",
    "property.finalInInterface",
    "property.abstractFinalHook",
    "property.hookBodyInInterface",
    "property.final",
    "property.abstractNonHooked",
    "property.abstractWithoutAbstractHook",
    "property.abstract",
    "property.hookWithoutBody",
    "property.abstractPrivate",
    "property.finalPrivate",
    "property.notFound",
    "property.private",
    "staticProperty.nonStaticAccess",
    "staticProperty.notFound",
    "staticProperty.private",
    "staticProperty.protected",
    "property.staticAccess",
    "property.finalPrivateHook",
    "property.abstractFinal",
    "property.hookReadOnly",
    "property.staticAsymmetricVisibility",
    "property.virtualDefault",
    "property.hooksNotSupported",
    "property.readOnlyNoNativeType",
    "property.readOnlyDefaultValue",
    "property.readOnlyStatic",
    "property.readOnlyByPhpDocDefaultValue",
    "property.promotedNotSupported",
    "property.invalidPromoted",
    "attribute.notFound",
    "attribute.notAttribute",
    "attribute.abstract",
    "attribute.target",
    "attribute.nonRepeatable",
    "attribute.noConstructor",
    "attribute.constructorNotPublic",
    "attribute.interface",
    "attribute.enum",
    "attribute.trait",
    "class.deprecatedAttribute",
    "interface.deprecatedAttribute",
    "enum.deprecatedAttribute",
    "trait.deprecatedAttribute",
    "class.allowDynamicPropertiesReadonly",
    "enum.allowDynamicProperties",
    "interface.allowDynamicProperties",
    "trait.allowDynamicProperties",
    "property.overrideAttribute",
    "constant.attributesNotSupported",
    "class.extendsFinal",
    "class.extendsFinalByPhpDoc",
    "class.readOnly",
    "class.nonReadOnly",
    "trait.notFound",
    "traitUse.class",
    "traitUse.enum",
    "traitUse.interface",
    "interface.traitUse",
    "instanceof.trait",
    "method.abstract",
    "method.nonAbstract",
    "consistentConstructor.private",
    "sealed.onTrait",
    "class.disallowedSubtype",
    "interface.disallowedSubtype",
    "enum.disallowedSubtype",
    "method.visibility",
    "trait.duplicateConstant",
    "trait.duplicateProperty",
    "trait.duplicateMethod",
    "staticClassAccess.privateProperty",
    "staticClassAccess.privateMethod",
    "staticClassAccess.privateConstant",
    "method.internal",
    "method.internalClass",
    "method.internalInterface",
    "method.internalTrait",
    "method.internalEnum",
    "staticMethod.internal",
    "staticMethod.internalClass",
    "staticMethod.internalInterface",
    "staticMethod.internalTrait",
    "staticMethod.internalEnum",
    "function.internal",
    "property.callableType",
    "class.serializable",
    "parameter.requiredAfterOptional",
    "parameter.implicitlyNullable",
    "parameter.unresolvableNativeType",
    "return.unresolvableNativeType",
    "property.unresolvableNativeType",
    "parameter.noParent",
    "return.noParent",
    "parameter.unionTypeNotSupported",
    "return.unionTypeNotSupported",
    "return.neverTypeNotSupported",
    "method.templateTypeNotInParameter",
    "selfOut.trait",
    "property.trait",
    "class.nameCase",
    "interface.nameCase",
    "trait.nameCase",
    "enum.nameCase",
    "class.prefixed",
    "classConstant.class",
    "classConstant.inTrait",
    "classConstant.finalNotSupported",
    "classConstant.nativeTypeNotSupported",
    "classConstant.final",
    "classConstant.nonFinal",
    "classConstant.visibility",
    "classConstant.nativeType",
    "classConstant.missingNativeType",
    "classConstant.type",
    "classConstant.value",
    "catch.nonCapturingNotSupported",
    "throw.notSupported",
    "callable.notSupported",
    "array.duplicateKey",
    "property.assignByRef",
    "method.parentMethodFinal",
    "method.parentMethodFinalByPhpDoc",
    "method.abstractOverridingNonAbstract",
    "method.override",
    "method.nonStatic",
    "method.static",
    "method.tentativeReturnType",
    "method.childReturnType",
    "method.childParameterType",
    "method.impure",
    "method.impureOverridePureUnlessCallable",
    "method.impureOverridePureUnlessParameterPassed",
    "parameter.missing",
    "parameter.byRef",
    "parameter.notByRef",
    "parameter.notOptional",
    "parameter.notVariadic",
    "property.override",
    "property.nonStatic",
    "property.static",
    "property.readWrite",
    "property.readOnly",
    "property.notReadable",
    "property.notWritable",
    "property.abstractOverridingNonAbstractHook",
    "property.visibility",
    "property.parentPropertyFinal",
    "property.parentPropertyFinalByPhpDoc",
    "property.missingNativeType",
    "property.nativeType",
    "property.extraNativeType",
    "propertySetHook.nativeParameterType",
    "new.staticInAbstractClassStaticMethod",
    "property.uninitializedReadonly",
    "assign.readOnlyProperty",
    "property.uninitializedReadonlyByPhpDoc",
    "assign.readOnlyPropertyByPhpDoc",
    "property.writeOnly",
    "classConstant.dynamicFetch",
    "classConstant.nameType",
    "classConstant.internal",
    "classConstant.internalClass",
    "classConstant.internalInterface",
    "classConstant.internalTrait",
    "classConstant.internalEnum",
    "return.missing",
    "return.never",
    "regexp.pattern",
    "preInc.expr",
    "postInc.expr",
    "preDec.expr",
    "postDec.expr",
    "assign.propertyReadOnly",
    "property.internal",
    "property.internalClass",
    "property.internalInterface",
    "property.internalTrait",
    "property.internalEnum",
    "staticProperty.internal",
    "staticProperty.internalClass",
    "staticProperty.internalInterface",
    "staticProperty.internalTrait",
    "staticProperty.internalEnum",
    "traitUse.internalClass",
    "traitUse.internalInterface",
    "traitUse.internalTrait",
    "traitUse.internalEnum",
    "traitUse.deprecatedClass",
    "traitUse.deprecatedInterface",
    "traitUse.deprecatedTrait",
    "traitUse.deprecatedEnum",
    "staticProperty.deprecatedClass",
    "staticProperty.deprecatedInterface",
    "staticProperty.deprecatedTrait",
    "staticProperty.deprecatedEnum",
    "attribute.internalClass",
    "attribute.internalInterface",
    "attribute.internalTrait",
    "attribute.internalEnum",
    "attribute.deprecated",
    "attribute.deprecatedClass",
    "attribute.deprecatedInterface",
    "attribute.deprecatedTrait",
    "attribute.deprecatedEnum",
    "catch.internalClass",
    "catch.internalInterface",
    "catch.internalTrait",
    "catch.internalEnum",
    "catch.deprecatedClass",
    "catch.deprecatedInterface",
    "catch.deprecatedTrait",
    "catch.deprecatedEnum",
    "classConstant.deprecatedClass",
    "classConstant.deprecatedInterface",
    "classConstant.deprecatedTrait",
    "classConstant.deprecatedEnum",
    "instanceof.internalClass",
    "instanceof.internalInterface",
    "instanceof.internalTrait",
    "instanceof.internalEnum",
    "instanceof.deprecatedClass",
    "instanceof.deprecatedInterface",
    "instanceof.deprecatedTrait",
    "instanceof.deprecatedEnum",
    "property.deprecatedClass",
    "property.deprecatedInterface",
    "property.deprecatedTrait",
    "property.deprecatedEnum",
    "parameter.internalClass",
    "parameter.internalInterface",
    "parameter.internalTrait",
    "parameter.internalEnum",
    "parameter.deprecatedClass",
    "parameter.deprecatedInterface",
    "parameter.deprecatedTrait",
    "parameter.deprecatedEnum",
    "return.internalClass",
    "return.internalInterface",
    "return.internalTrait",
    "return.internalEnum",
    "return.deprecatedClass",
    "return.deprecatedInterface",
    "return.deprecatedTrait",
    "return.deprecatedEnum",
    "new.internalClass",
    "new.internalInterface",
    "new.internalTrait",
    "new.internalEnum",
    "new.deprecatedClass",
    "new.deprecatedInterface",
    "new.deprecatedTrait",
    "new.deprecatedEnum",
    "staticMethod.deprecatedClass",
    "staticMethod.deprecatedInterface",
    "staticMethod.deprecatedTrait",
    "staticMethod.deprecatedEnum",
    "class.extendsInternalClass",
    "class.extendsInternalInterface",
    "class.extendsInternalTrait",
    "class.extendsInternalEnum",
    "class.extendsDeprecatedClass",
    "class.extendsDeprecatedInterface",
    "class.extendsDeprecatedTrait",
    "class.extendsDeprecatedEnum",
    "class.implementsInternalClass",
    "class.implementsInternalInterface",
    "class.implementsInternalTrait",
    "class.implementsInternalEnum",
    "class.implementsDeprecatedClass",
    "class.implementsDeprecatedInterface",
    "class.implementsDeprecatedTrait",
    "class.implementsDeprecatedEnum",
    "interface.extendsInternalClass",
    "interface.extendsInternalInterface",
    "interface.extendsInternalTrait",
    "interface.extendsInternalEnum",
    "interface.extendsDeprecatedClass",
    "interface.extendsDeprecatedInterface",
    "interface.extendsDeprecatedTrait",
    "interface.extendsDeprecatedEnum",
    "enum.implementsInternalClass",
    "enum.implementsInternalInterface",
    "enum.implementsInternalTrait",
    "enum.implementsInternalEnum",
    "enum.implementsDeprecatedClass",
    "enum.implementsDeprecatedInterface",
    "enum.implementsDeprecatedTrait",
    "enum.implementsDeprecatedEnum",
    "method.deprecated",
    "method.deprecatedClass",
    "method.deprecatedInterface",
    "method.deprecatedTrait",
    "method.deprecatedEnum",
    "staticMethod.deprecated",
    "property.deprecated",
    "staticProperty.deprecated",
    "classConstant.deprecated",
    "function.deprecated",
    "class.toStringDeprecated",
    "constant.deprecated",
    // analyse dead-code (an-dead)
    "deadCode.unreachable",
    "classConstant.unused",
    "method.unused",
    "property.unused",
    "property.neverRead",
    "property.neverWritten",
    "property.onlyRead",
    "property.onlyWritten",
    "expr.resultUnused",
    "logicalXor.resultUnused",
    "logicalAnd.resultUnused",
    "logicalOr.resultUnused",
    "booleanAnd.resultUnused",
    "booleanOr.resultUnused",
    "ternary.resultUnused",
    "function.resultUnused",
    "method.resultUnused",
    "staticMethod.resultUnused",
    "new.resultUnused",
    "if.alwaysTrue",
    "if.alwaysFalse",
    "elseif.alwaysTrue",
    "elseif.alwaysFalse",
    "ternary.alwaysTrue",
    "ternary.alwaysFalse",
    "doWhile.alwaysTrue",
    "doWhile.alwaysFalse",
    "booleanNot.alwaysTrue",
    "booleanNot.alwaysFalse",
    "while.alwaysTrue",
    "while.alwaysFalse",
    "booleanAnd.alwaysTrue",
    "booleanAnd.alwaysFalse",
    "booleanAnd.leftAlwaysTrue",
    "booleanAnd.leftAlwaysFalse",
    "booleanAnd.rightAlwaysTrue",
    "booleanAnd.rightAlwaysFalse",
    "booleanOr.alwaysTrue",
    "booleanOr.alwaysFalse",
    "booleanOr.leftAlwaysTrue",
    "booleanOr.leftAlwaysFalse",
    "booleanOr.rightAlwaysTrue",
    "booleanOr.rightAlwaysFalse",
    "logicalAnd.alwaysTrue",
    "logicalAnd.alwaysFalse",
    "logicalAnd.leftAlwaysTrue",
    "logicalAnd.leftAlwaysFalse",
    "logicalAnd.rightAlwaysTrue",
    "logicalAnd.rightAlwaysFalse",
    "logicalOr.alwaysTrue",
    "logicalOr.alwaysFalse",
    "logicalOr.leftAlwaysTrue",
    "logicalOr.leftAlwaysFalse",
    "logicalOr.rightAlwaysTrue",
    "logicalOr.rightAlwaysFalse",
    "logicalXor.leftAlwaysTrue",
    "logicalXor.leftAlwaysFalse",
    "logicalXor.rightAlwaysTrue",
    "logicalXor.rightAlwaysFalse",
    "identical.alwaysTrue",
    "identical.alwaysFalse",
    "notIdentical.alwaysTrue",
    "notIdentical.alwaysFalse",
    "equal.alwaysTrue",
    "equal.alwaysFalse",
    "notEqual.alwaysTrue",
    "notEqual.alwaysFalse",
    "greater.alwaysTrue",
    "greater.alwaysFalse",
    "greaterOrEqual.alwaysTrue",
    "greaterOrEqual.alwaysFalse",
    "smaller.alwaysTrue",
    "smaller.alwaysFalse",
    "smallerOrEqual.alwaysTrue",
    "smallerOrEqual.alwaysFalse",
    "instanceof.alwaysTrue",
    "instanceof.alwaysFalse",
    "function.alreadyNarrowedType",
    "function.impossibleType",
    "method.alreadyNarrowedType",
    "method.impossibleType",
    "staticMethod.alreadyNarrowedType",
    "staticMethod.impossibleType",
    "match.alwaysFalse",
    "match.alwaysTrue",
    "match.unhandled",
    "instanceof.invalidExprType",
    "nullCoalesce.variable",
    "nullCoalesce.offset",
    "nullCoalesce.property",
    "nullCoalesce.expr",
    "nullsafe.neverNull",
    "nullCoalesce.unnecessary",
    "nullCoalesce.initializedProperty",
    "isset.variable",
    "isset.offset",
    "isset.property",
    "isset.expr",
    "isset.initializedProperty",
    "empty.variable",
    "empty.offset",
    "empty.property",
    "empty.expr",
    "empty.initializedProperty",
    "switch.duplicateCase",
    "switch.alwaysFalse",
    "switch.alwaysTrue",
    "function.impossibleHaystackValue",
    "variable.unused",
    "assign.redundant",
    "assign.unused",
    "assign.overwritten",
    "assign.unusedFlow",
    "preInc.unused",
    "preInc.overwritten",
    "preInc.unusedFlow",
    "postInc.unused",
    "postInc.overwritten",
    "postInc.unusedFlow",
    "preDec.unused",
    "preDec.overwritten",
    "preDec.unusedFlow",
    "postDec.unused",
    "postDec.overwritten",
    "postDec.unusedFlow",
    "foreach.unusedValue",
    "foreach.valueOverwritten",
    "foreach.unusedValueFlow",
    "foreach.unusedKey",
    "foreach.keyOverwritten",
    "foreach.unusedKeyFlow",
    "catch.unusedVariableFlow",
    "array.unusedOffset",
    "array.offsetOverwritten",
    "array.unusedOffsetFlow",
    "sortArray.empty",
    "sortArray.list",
    "sortArray.noop",
];

pub const DISCOVERING_SYMBOLS_TIP: &str =
    "Learn more at https://phpstan.org/user-guide/discovering-symbols";

/// The class-like a node is in (`$scope->getClassReflection()`).
struct ClassScope {
    name: Option<String>,
    /// Guards the constructor ends with, which PHPStan remembers in the other methods.
    constructor_guards: Vec<Guard>,
}

/// The identifier of `self`, `static` or `parent` (`e`) used outside of a class.
fn out_of_class(e: &Expression) -> &'static str {
    match e {
        Expression::Self_(_) => "outOfClass.self",
        Expression::Static(_) => "outOfClass.static",
        _ => "outOfClass.parent",
    }
}

/// How messages name a member the scope cannot access.
fn restricted(v: mago_codex::visibility::Visibility) -> &'static str {
    if v.is_private() {
        "private"
    } else {
        "protected"
    }
}

/// A trait walked in the context of a class using it (see [`driver::check_file()`]).
struct TraitContext {
    /// The class's file and name.
    class_file: String,
    class: String,
    trait_name: String,
    /// The trait's methods `use T { foo as bar; }` renames, by lowercase name.
    renames: HashMap<String, String>,
}

/// `$scope->getFunction()`: the function or method a node is in (closures don't change it).
pub struct FunctionScope {
    pub name: String,
    pub kind: FunctionScopeKind,
    /// `isDeprecated()`, when the deprecation rules run.
    pub deprecated: bool,
    /// Where its declaration starts.
    pub start: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FunctionScopeKind {
    Function,
    Method { is_static: bool },
}

pub struct Checker<'a, 'arena> {
    file: &'a File,
    /// `MutatingScope::fileHasCompilerHaltStatementCalls()` for this file's parsed program.
    has_compiler_halt: bool,
    /// The display names of the file's anonymous classes, by their codex name (see [`anonymous::names()`]).
    anonymous: HashMap<Vec<u8>, String>,
    property_access: property_access::State,
    relative_path: &'a crate::relative_path::RelativePath,
    /// Mago's inferred types of the expressions rules ask about (see [`typed_expressions()`]).
    types: HashMap<Span, TUnion>,
    /// ExpressionResult keep-void answers, read only behind Scope's explicit-null gate.
    keep_void_types: HashMap<Span, TUnion>,
    /// Independent native answers from the same evaluations; absence never substitutes the full type.
    native_types: HashMap<Span, TUnion>,
    /// The processed nullsafe receiver before its non-null plain twin, not the ensured operand view.
    nullsafe_method_receiver_types: HashMap<Span, TUnion>,
    /// A property's new value after an offset write, by the root fetch's span (AssignHandler's PropertyAssignNode).
    property_offset_writes: HashMap<Span, TUnion>,
    /// PHPStan VariableWritesNode results for completed closure bodies.
    variable_writes: HashMap<Span, mago_analyzer::variable_flow::VariableWritesNode>,
    /// Requested PropertyInitializationExpr certainties immediately before real IssetCheck operands.
    property_initializations: HashMap<Span, mago_analyzer::property_initialization::PropertyInitializationCertainty>,
    /// Resolved calls' own termination, independent of their provider-selected result type.
    call_terminations: HashMap<(u32, u32), bool>,
    /// Operand-position answers for each method name, including its expression conversion dependencies.
    named_call_scopes: HashMap<Span, Vec<infer::NamedCallScope>>,
    /// `$scope->hasVariableType()` where the wanted direct variables are read (see [`infer::analyse()`]).
    defined: HashMap<Span, pun_phpstan::types::Tri>,
    /// The running inference blocks observed by DefinedVariableRule and CompactVariablesRule.
    variables: variables::Observations,
    /// This inference run's source provenance for PHPDoc metadata Mago's types cannot retain.
    origins: infer::Origins,
    names: &'a ResolvedNames<'arena>,
    codebase: &'a CodebaseMetadata,
    classes: Vec<ClassScope>,
    /// Guards in effect; `marks` are where enclosing blocks started (guards an early exit leaves last to the
    /// block's end).
    guards: Vec<Guard>,
    marks: Vec<usize>,
    vars: HashMap<Vec<u8>, Vec<Guard>>,
    saved: Vec<Saved>,
    functions: Vec<FunctionScope>,
    /// The functions and methods the walk is in (var_tags.rs reads their templates).
    var_tag_functions: Vec<mago_span::Span>,
    /// Inside the arguments of class_exists() & co. (`inFunctionCallsStack`).
    in_exists_call: usize,
    /// Spans of expressions PHPStan processes in a top-level ExpressionContext (`isInFirstLevelStatement()`):
    /// expression statements, `for` initialisations and increments, arrow function and match arm bodies.
    top_level: HashSet<(u32, u32)>,
    /// `f(...)` on the right of `|>`: analysed as the call `f($x)`, not as a first-class callable.
    piped: HashSet<Span>,
    /// Closures whose `$this` is rebound: `(function () {...})->call($o)` (MethodCallHandler) and `Closure::bind()`
    /// (StaticCallHandler), and inline `@param-closure-this` arguments (ArgumentsHandler). `->bindTo()` keeps the
    /// scope's `$this` (oracle-checked).
    rebound: HashMap<Span, bind_scope::Binding>,
    /// `Error::file`: the path, or `T.php (in context of class C)` inside a trait analysed for class C.
    label: String,
    /// Set when walking a trait in a class's context (`$scope->isInTrait()`).
    trait_context: Option<TraitContext>,
    /// Inside the context walk's trait (in a context walk, nothing else reports).
    in_target: bool,
    /// Each `use T;` in a class body, to analyse T in the class's context.
    trait_uses: Vec<driver::TraitUse>,
    /// The trait method being walked under its alias (span, alias).
    renamed: Option<(Span, String)>,
    php_version: u32,
    /// PHPStan's reflection of built-ins at `php_version`.
    builtins: &'static pun_phpstan::symbols::Symbols,
    /// Line ranges of the traits declared in this file (their ignores are never unmatched here).
    trait_lines: Vec<(i64, i64)>,
    /// Scope boundaries: what `$scope->getFunction()` & co. see, and the goto labels of each.
    frames: Vec<Frame>,
    /// `$scope->getNamespace()` per namespace statement entered (`None`: the global one).
    namespaces: Vec<Option<String>>,
    /// Magic constants that are a parameter's default (MagicConstantParamDefaultVisitor).
    magic_defaults: HashSet<Span>,
    /// DeclarePositionVisitor: no statement seen yet, and the `declare`s that came after one.
    first_statement: bool,
    declares_not_first: Vec<Span>,
    /// The class-likes being declared (InClassNode's ClassReflection), innermost last.
    decls: Vec<declarations::Decl>,
    /// `isImmutable()` of the codebase's classes, by lowercase name.
    immutable: HashMap<String, bool>,
    /// A symbol's file, and the analysed files' sources (other files are read when a rule needs them).
    files: &'a HashMap<FileId, PathBuf>,
    sources: &'a Sources,
    /// [`driver::Checks`]'s stub declarations: a patched declaration's span to its stub's, whose PHPDoc PHPStan reads.
    patched_docs: &'a HashMap<Span, Span>,
    /// [`driver::Checks`]'s classes, built once per run.
    class_cache: &'a declared::ClassCache,
    syntax: syntax::State,
    assign: assign::State,
    signatures: signatures::State,
    class_names: class_names::State,
    arrays: arrays::State,
    /// `throw` expressions that are an expression statement (StandaloneThrowExprVisitor).
    standalone_throws: HashSet<Span>,
    /// [`Checks`]'s `check_class_case_sensitivity`.
    case_sensitivity: bool,
    /// [`Checks`]'s independent class-name case options.
    check_internal_class_case_sensitivity: bool,
    check_imported_class_name_case: bool,
    /// [`Checks`]'s independent `checkFunctionNameCase` switch.
    check_function_name_case: bool,
    /// [`driver::Checks`]'s level and `checkGenericIterableClasses`.
    level: u8,
    /// FunctionCallParametersCheck's independent switches and RuleLevelHelper's receiver eligibility.
    check_argument_types: bool,
    check_arguments_passed_by_reference: bool,
    check_this_only: bool,
    /// IssetCheck's independent advanced offset, property and leaf checks.
    check_advanced_isset: bool,
    /// RuleLevelHelper's `checkExplicitMixed` and `checkImplicitMixed`.
    check_explicit_mixed: bool,
    check_implicit_mixed: bool,
    /// IssetCheck's independent PHPDoc certainty; other rules keep their existing type selection.
    treat_php_doc_types_as_certain: bool,
    generic_iterable_classes: bool,
    /// [`driver::Checks`]'s `featureToggles.newStaticInAbstractClassStaticMethod`.
    new_static_in_abstract_class_static_method: bool,
    /// [`driver::Checks`]'s Larastan switches, when the config includes Larastan.
    larastan: Option<&'a Larastan>,
    larastan_state: larastan::State,
    /// [`driver::Checks`]'s `@internal` method names, when the internal usage rules run.
    internal: Option<&'a internal::Names>,
    /// [`driver::Checks`]'s `featureToggles.internalTag`.
    internal_tag: bool,
    precise_unused_parameter_line: bool,
    /// UnreachableStatementNode positions and statement traversal after termination.
    unreachable: dead_code::UnreachableStatements,
    /// LastConditionVisitor's attributes and more the constant condition rules read.
    conditions: dead_code::ConditionFacts,
    /// [`driver::Checks`]'s level-2 RestrictedInternalMethodUsageExtension and RestrictedInternalPropertyUsageExtension.
    internal_members: bool,
    /// [`driver::Checks`]'s phpstan-deprecation-rules switch and `featureToggles.bleedingEdge`.
    deprecations: bool,
    bleeding_edge: bool,
    /// UnusedLabelRule: level 4 with `featureToggles.unusedLabel`.
    unused_label: bool,
    /// SwitchConditionRule: `featureToggles.switchConditionAlwaysFalse`.
    switch_condition: bool,
    /// SortWithoutEffectRule: `featureToggles.sortWithoutEffect`.
    sort_without_effect: bool,
    /// NonStringableDynamicAccessCheck: `featureToggles.checkNonStringableDynamicAccess`.
    non_stringable_names: bool,
    cli_arguments_variables_registered: bool,
    check_maybe_undefined_variables: bool,
    pollute_scope_with_loop_initial_assignments: bool,
    pollute_scope_with_always_iterable_foreach: bool,
    pollute_scope_with_block: bool,
    /// MethodSignatureRule's purity checks: `featureToggles.reportMethodPurityOverride`.
    method_purity_override: bool,
    /// UninitializedPropertyRule: `checkUninitializedProperties`.
    uninitialized_properties: bool,
    /// MissingTypehintCheck's `checkMissingCallableSignature`.
    missing_callable_signature: bool,
    /// The phpstan-strict-rules switches.
    strict: crate::StrictRules,
    /// [`driver::Checks`]'s `exceptions.*` parameters.
    exceptions: &'a crate::Exceptions,
    /// TooWideTypeCheck's switches.
    too_wide: crate::TooWide,
    include_settings: &'a include_file::Settings,
    include_state: include_file::State,
    /// How many `Closure::bind()` calls the walk is inside (`isInClosureBind()`).
    closure_bind: usize,
    /// [`driver::Checks`]'s feature-toggled rules.
    toggled: &'a driver::Toggled,
    /// AssignToByRefExprFromForeachRule's statement entries (empty unless it runs).
    by_ref_foreach: by_ref_foreach::Entries,
    /// Registrations of the owners of named-argument call and rename records.
    method_rules: crate::MethodRules,
    errors: Vec<Error>,
}

/// A file's contents, borrowed from the run's sources or shared with its read cache.
pub(crate) enum Contents<'a> {
    Borrowed(&'a [u8]),
    Shared(std::sync::Arc<[u8]>),
}

impl Default for Contents<'_> {
    fn default() -> Self {
        Contents::Borrowed(&[])
    }
}

impl std::ops::Deref for Contents<'_> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        match self {
            Contents::Borrowed(b) => b,
            Contents::Shared(b) => b,
        }
    }
}

impl<'a, 'arena> Checker<'a, 'arena> {
    /// A file's contents: this one, an analysed one, or one read from disk.
    fn file_contents(&self, id: FileId) -> Option<Contents<'a>> {
        if id == self.file.id {
            return Some(Contents::Borrowed(&self.file.contents[..]));
        }
        let path = self.files.get(&id)?;
        match self.sources.get(path) {
            Some(f) => Some(Contents::Borrowed(&f.contents[..])),
            None => self.sources.read(path).map(Contents::Shared),
        }
    }

    /// `class.noParent`: `{action} but C does not extend any class.`, for a parent:: or `new parent` in class `scope`.
    fn report_no_parent(&mut self, at: impl HasSpan, scope: &ClassLikeMetadata, action: &str) {
        let display = self.display(scope);
        self.report(
            at,
            "class.noParent",
            format!("{action} but {display} does not extend any class."),
            None,
        );
    }

    /// The name of the function or method a node is in, empty outside one.
    fn function_name(&self) -> String {
        self.functions
            .last()
            .map(|f| f.name.clone())
            .unwrap_or_default()
    }

    /// The trait walked in a class's context, while inside it (`$scope->isInTrait()`).
    fn in_trait(&self) -> Option<&str> {
        self.trait_context
            .as_ref()
            .filter(|_| self.in_target)
            .map(|t| t.trait_name.as_str())
    }

    fn file_name(&self) -> String {
        String::from_utf8_lossy(&self.file.name).into_owned()
    }

    fn report(&mut self, at: impl HasSpan, identifier: &str, message: String, tip: Option<&str>) {
        let line = self.line_of(at.span());
        self.report_at_line(line, identifier, message, tip);
    }

    fn line_of(&self, at: Span) -> i64 {
        i64::from(self.file.line_number(at.start.offset)) + 1
    }

    fn report_at_line(&mut self, line: i64, identifier: &str, message: String, tip: Option<&str>) {
        debug_assert!(reports(identifier), "{identifier} is not in IDENTIFIERS");
        if self.trait_context.is_some() && !self.in_target {
            return;
        }
        self.errors.push(Error {
            identifier: Some(identifier.to_string()),
            tip: tip.map(str::to_string),
            file_path: self
                .trait_context
                .as_ref()
                .map_or_else(|| self.file_name(), |t| t.class_file.clone()),
            trait_file_path: self.trait_context.as_ref().map(|_| self.file_name()),
            ..Error::new(message, self.label.clone(), Some(line))
        });
    }

    /// A `nonIgnorable()` error: no `@phpstan-ignore` or `ignoreErrors` entry removes it.
    fn report_non_ignorable(
        &mut self,
        at: impl HasSpan,
        identifier: &str,
        message: String,
        tip: Option<&str>,
    ) {
        let count = self.errors.len();
        self.report(at, identifier, message, tip);
        if let Some(e) = self.errors.get_mut(count) {
            e.can_be_ignored = false;
        }
    }

    /// `(string) $node->name` after php-parser's NameResolver: an unqualified function or constant name stays as
    /// written (it falls back to the global one at runtime), everything else is the resolved name.
    fn written_or_resolved(&self, id: &Identifier<'arena>) -> String {
        let text = match id {
            Identifier::Local(l) if !self.names.is_imported(&l.span) => l.value,
            _ => self.names.get(&id.span()),
        };
        String::from_utf8_lossy(text).into_owned()
    }

    /// `Closure::bind(...)`.
    fn is_closure_bind(&self, call: &StaticMethodCall<'arena>) -> bool {
        matches!((call.class, &call.method), (Expression::Identifier(id), ClassLikeMemberSelector::Identifier(m))
            if self.resolved(id).eq_ignore_ascii_case("Closure") && m.value.eq_ignore_ascii_case(b"bind"))
    }

    /// A rebound closure literal and the class of its bound `$this` (see [`Checker::bind_class()`]).
    fn rebind(&mut self, e: &Expression, class: Option<String>, defines_this: bool) {
        let e = types::flow::unparen(e);
        if matches!(e, Expression::Closure(_) | Expression::ArrowFunction(_)) {
            self.rebound.insert(e.span(), bind_scope::Binding { class, defines_this });
        }
    }

    fn trait_uses_of(&mut self, class: &str, members: &[ClassLikeMember<'arena>]) {
        for m in members {
            let ClassLikeMember::TraitUse(u) = m else {
                continue;
            };
            let adaptations = match &u.specification {
                TraitUseSpecification::Concrete(s) => s.adaptations.as_slice(),
                TraitUseSpecification::Abstract(_) => &[],
            };
            for id in u.trait_names.iter() {
                let trait_name = self.resolved(id);
                let mut renames = HashMap::new();
                for a in adaptations {
                    let TraitUseAdaptation::Alias(a) = a else {
                        continue;
                    };
                    let Some(alias) = &a.alias else {
                        continue;
                    };
                    let method = match &a.method_reference {
                        TraitUseMethodReference::Identifier(m) => m,
                        // Mago resolves only the `use` list's names: the one written the same is this trait
                        TraitUseMethodReference::Absolute(r) => {
                            if !self
                                .written(r.trait_name.span())
                                .eq_ignore_ascii_case(&self.written(id.span()))
                            {
                                continue;
                            }
                            &r.method_name
                        }
                    };
                    renames.insert(
                        String::from_utf8_lossy(method.value).to_ascii_lowercase(),
                        String::from_utf8_lossy(alias.value).into_owned(),
                    );
                }
                self.trait_uses.push(driver::TraitUse {
                    class: class.to_string(),
                    trait_name,
                    renames,
                    php_versions: self.guards.iter().filter(|g| matches!(g, Guard::PhpVersion(..))).cloned().collect(),
                });
            }
        }
    }

    /// processStmtNode() of a trait method in a context walk: skipped when the class's method of that name (after
    /// `as` renames) is not this trait's, as when the class or another trait shadows it.
    fn shadowed(&self, class: &str, name: &str, trait_span: Span) -> bool {
        let this_trait = |f: &mago_codex::metadata::function_like::FunctionLikeMetadata| {
            f.span.file_id == trait_span.file_id
                && trait_span.start.offset <= f.span.start.offset
                && f.span.end.offset <= trait_span.end.offset
        };
        self.codebase.get_class_like(class.as_bytes()).is_some_and(|k| {
            // BetterReflection's own cases(), and a backed enum's from() and tryFrom(), replace an enum's others
            let lower = name.to_ascii_lowercase();
            (k.kind == SymbolKind::Enum
                && (lower == "cases" || (k.enum_type.is_some() && (lower == "from" || lower == "tryfrom"))))
                || members::method(self, k, name.as_bytes())
                    .is_some_and(|m| !matches!(m, members::Method::User { f, .. } if this_trait(f)))
        })
    }

    fn written(&self, span: Span) -> String {
        String::from_utf8_lossy(&self.file.contents[span.to_range_usize()]).into_owned()
    }

    fn resolved_span(&self, span: Span) -> String {
        String::from_utf8_lossy(self.names.get(&span)).into_owned()
    }

    fn resolved(&self, id: &Identifier<'arena>) -> String {
        String::from_utf8_lossy(self.names.get(&id.span())).into_owned()
    }

    fn class_scope(&self) -> Option<&'a ClassLikeMetadata> {
        let name = self.classes.last()?.name.as_deref()?;
        self.codebase.get_class_like(name.as_bytes())
    }

    /// The class-like a member is walked in and its display name (trait bodies are walked only in a class's context).
    fn member_scope(&self) -> (&'a ClassLikeMetadata, String) {
        let class = self
            .class_scope()
            .expect("a member is walked inside its class-like");
        let display = self
            .decls
            .last()
            .and_then(|d| d.display.clone())
            .expect("a class-like's declaration has its display name");
        (class, display)
    }

    /// In an anonymous class's body (not in a trait it uses).
    fn in_anonymous_class(&self) -> bool {
        self.trait_context.is_none()
            && self
                .classes
                .last()
                .and_then(|k| k.name.as_deref())
                .is_some_and(|n| n.starts_with("{anonymous-class:"))
    }

    fn enter_class(&mut self, name: Option<String>, members: &[ClassLikeMember<'arena>]) {
        let constructor_guards = members
            .iter()
            .find_map(|m| match m {
                ClassLikeMember::Method(m) if m.name.value.eq_ignore_ascii_case(b"__construct") => {
                    match &m.body {
                        MethodBody::Concrete(b) => Some(self.exit_guards(b.statements.iter())),
                        MethodBody::Abstract(_) => None,
                    }
                }
                _ => None,
            })
            .unwrap_or_default();
        self.classes.push(ClassScope {
            name,
            constructor_guards,
        });
    }

    fn mark_top_level(&mut self, e: &Expression) {
        let e = types::flow::unparen(e);
        let span = e.span();
        self.top_level.insert((span.start.offset, span.end.offset));
        // PHPStan passes the statement's context on to a ternary's branches, the right side of `&&` / `||` and
        // the operand of `@`
        match e {
            Expression::Conditional(c) => {
                if let Some(then) = c.then {
                    self.mark_top_level(then);
                }
                self.mark_top_level(c.r#else);
            }
            Expression::Binary(b)
                if matches!(
                    b.operator,
                    BinaryOperator::And(_)
                        | BinaryOperator::LowAnd(_)
                        | BinaryOperator::Or(_)
                        | BinaryOperator::LowOr(_)
                ) =>
            {
                self.mark_top_level(b.rhs);
            }
            Expression::UnaryPrefix(u)
                if matches!(u.operator, UnaryPrefixOperator::ErrorControl(_)) =>
            {
                self.mark_top_level(u.operand);
            }
            _ => {}
        }
    }

    fn is_top_level(&self, span: Span) -> bool {
        self.top_level
            .contains(&(span.start.offset, span.end.offset))
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Checker<'_, 'arena> {
    fn walk_expression(&mut self, e: &'ast Expression<'arena>, context: &mut ()) {
        assign::enter_expression(self, e);
        mago_syntax::walker::walk_expression_mut(self, e, context);
        assign::exit_expression(self, e);
    }

    fn walk_in_class(&mut self, class: &'ast Class<'arena>, _: &mut ()) {
        let current = format!("Class {}", self.resolved_span(class.name.span));
        names::class_hierarchy(
            self,
            class.span(),
            &current,
            class.extends.as_ref(),
            class.implements.as_ref(),
            false,
            class.modifiers.contains_readonly(),
        );
        if self.trait_context.is_none() {
            let name = self.resolved_span(class.name.span);
            self.trait_uses_of(&name, class.members.as_slice());
        }
        self.enter_class(
            Some(self.resolved_span(class.name.span)),
            class.members.as_slice(),
        );
        self.push_frame(FrameKind::Class);
        var_names::misplaced(self, class.span(), "a class");
        let decl = self.decl_of(
            class.span(),
            Some(class.name.span),
            SymbolKind::Class,
            class.modifiers.as_slice(),
            class.extends.as_ref(),
        );
        self.decls.push(decl);
        declarations::duplicates(self, class.members.as_slice());
        let name = self.resolved_span(class.name.span);
        attributes::class_like(
            self,
            class.span(),
            class.attribute_lists.as_slice(),
            SymbolKind::Class,
            class.modifiers.contains_readonly(),
            &name,
        );
        if let Some(k) = self.codebase.get_class_like(name.as_bytes()) {
            hierarchy::in_class(self, class.span(), k);
            require_ancestors::check(self, class.span(), k);
            hierarchy::allowed_subtypes(self, class.span(), k);
            require_tags::class_like(self, class.span(), k);
            class_tags::class_like(self, class.span(), k);
            dead_code::unused_private(self, k, class.attribute_lists.as_slice(), class.members.as_slice());
            readonly_init::class(self, k, class.members.as_slice());
            types::too_wide_properties(self, k, class.members.as_slice());
        }
        type_aliases::class_like(self, class.span());
        invalid_throws::node(self, class.span());
        missing_types::class(self, class);
        generics::class(self, class);
        larastan::class(self, class);
        constants::read_only_class(self, class.span(), class.modifiers.as_slice(), false);
    }
    fn walk_out_class(&mut self, class: &'ast Class<'arena>, _: &mut ()) {
        phpdoc_tags::nop(self, class.right_brace);
        self.classes.pop();
        self.decls.pop();
        self.pop_frame();
    }
    fn walk_in_interface(&mut self, i: &'ast Interface<'arena>, _: &mut ()) {
        let current = format!("Interface {}", self.resolved_span(i.name.span));
        names::class_hierarchy(
            self,
            i.span(),
            &current,
            i.extends.as_ref(),
            None,
            true,
            false,
        );
        self.enter_class(Some(self.resolved_span(i.name.span)), i.members.as_slice());
        self.push_frame(FrameKind::Class);
        var_names::misplaced(self, i.span(), "an interface");
        let decl = self.decl_of(
            i.span(),
            Some(i.name.span),
            SymbolKind::Interface,
            &[],
            None,
        );
        self.decls.push(decl);
        declarations::duplicates(self, i.members.as_slice());
        let name = self.resolved_span(i.name.span);
        attributes::class_like(
            self,
            i.span(),
            i.attribute_lists.as_slice(),
            SymbolKind::Interface,
            false,
            &name,
        );
        if let Some(k) = self.codebase.get_class_like(name.as_bytes()) {
            hierarchy::allowed_subtypes(self, i.span(), k);
            require_tags::class_like(self, i.span(), k);
            class_tags::class_like(self, i.span(), k);
        }
        type_aliases::class_like(self, i.span());
        invalid_throws::node(self, i.span());
        missing_types::interface(self, i);
        generics::interface(self, i);
    }
    fn walk_out_interface(&mut self, i: &'ast Interface<'arena>, _: &mut ()) {
        phpdoc_tags::nop(self, i.right_brace);
        self.classes.pop();
        self.decls.pop();
        self.pop_frame();
    }
    fn walk_in_enum(&mut self, e: &'ast Enum<'arena>, _: &mut ()) {
        let current = format!("Enum {}", self.resolved_span(e.name.span));
        names::class_hierarchy(
            self,
            e.span(),
            &current,
            None,
            e.implements.as_ref(),
            false,
            false,
        );
        if self.trait_context.is_none() {
            let name = self.resolved_span(e.name.span);
            self.trait_uses_of(&name, e.members.as_slice());
        }
        self.enter_class(Some(self.resolved_span(e.name.span)), e.members.as_slice());
        self.push_frame(FrameKind::Class);
        var_names::misplaced(self, e.span(), "an enum");
        let decl = self.decl_of(e.span(), Some(e.name.span), SymbolKind::Enum, &[], None);
        self.decls.push(decl);
        enums::sanity(self, e);
        declarations::duplicates(self, e.members.as_slice());
        let name = self.resolved_span(e.name.span);
        attributes::class_like(
            self,
            e.span(),
            e.attribute_lists.as_slice(),
            SymbolKind::Enum,
            false,
            &name,
        );
        if let Some(k) = self.codebase.get_class_like(name.as_bytes()) {
            hierarchy::in_class(self, e.span(), k);
            require_ancestors::check(self, e.span(), k);
            hierarchy::allowed_subtypes(self, e.span(), k);
            require_tags::class_like(self, e.span(), k);
            class_tags::class_like(self, e.span(), k);
            dead_code::unused_private(self, k, e.attribute_lists.as_slice(), e.members.as_slice());
        }
        type_aliases::class_like(self, e.span());
        invalid_throws::node(self, e.span());
        missing_types::enum_(self, e);
        generics::enum_(self, e);
    }
    fn walk_out_enum(&mut self, e: &'ast Enum<'arena>, _: &mut ()) {
        phpdoc_tags::nop(self, e.right_brace);
        self.classes.pop();
        self.decls.pop();
        self.pop_frame();
    }
    /// A trait is analysed only in the context of each class using it (see [`in_trait_context()`]).
    fn walk_trait(&mut self, t: &'ast Trait<'arena>, c: &mut ()) {
        // Sources also retains scanned global declarations for initializer resolution. Retention is
        // not admission to ordinary trait-body analysis.
        if self.file.file_type != mago_database::file::FileType::Host {
            return;
        }
        let start = i64::from(self.file.line_number(t.span().start.offset)) + 1;
        let end = i64::from(self.file.line_number(t.span().end.offset)) + 1;
        self.trait_lines.push((start, end));
        let Some(class) = self.trait_context.as_ref().map(|t| t.class.clone()) else {
            attributes::trait_declaration(self, t);
            hierarchy::sealed_trait(self, t);
            let name = self.resolved_span(t.name.span);
            require_tags::trait_(self, t.span(), &name);
            class_tags::trait_definition(self, t.span(), &name);
            type_aliases::trait_definition(self, t.span(), &name);
            missing_types::trait_(self, t);
            generics::trait_(self, t);
            return;
        };
        let name = self.resolved_span(t.name.span);
        let target = self
            .trait_context
            .as_ref()
            .is_some_and(|t| t.trait_name.eq_ignore_ascii_case(&name));
        // the class's walk put it in the codebase
        let Some(decl) = target.then(|| self.decl_of_class(&class)).flatten() else {
            return;
        };
        self.trait_uses_of(&class, t.members.as_slice());
        let saved = self.classes.len();
        self.enter_class(None, t.members.as_slice());
        self.classes[saved].name = Some(class.clone());
        self.decls.push(decl);
        self.in_target = true;
        self.types.extend(crate::codebase::trait_initializer_types(
            self.codebase, self.file, t.span(), mago_word::word(&class), self.php_version,
        ));
        let display = self
            .codebase
            .get_class_like(name.as_bytes())
            .map_or_else(|| name.clone(), |k| self.display(k));
        declarations::trait_duplicates(self, t.members.as_slice(), &display);
        attributes::trait_in_context(self, t);
        type_aliases::trait_use(self, t.span(), &name, &class);
        class_tags::trait_use(self, t.span(), &name, &class);
        invalid_throws::node(self, t.span());
        for member in t.members.iter() {
            if let ClassLikeMember::Method(m) = member {
                let original = String::from_utf8_lossy(m.name.value).into_owned();
                let name = self
                    .trait_context
                    .as_ref()
                    .and_then(|t| t.renames.get(&original.to_ascii_lowercase()))
                    .cloned()
                    .unwrap_or(original);
                if self.shadowed(&class, &name, t.span()) {
                    continue;
                }
                self.renamed = Some((m.span(), name));
            }
            self.walk_class_like_member(member, c);
        }
        self.in_target = false;
        self.classes.truncate(saved);
        self.decls.pop();
    }
    // the constructor arguments are evaluated in the enclosing scope, before the class is entered
    fn walk_anonymous_class(&mut self, a: &'ast AnonymousClass<'arena>, c: &mut ()) {
        if let Some(arguments) = &a.argument_list {
            self.walk_partial_argument_list(arguments, c);
        }
        self.walk_in_anonymous_class(a, c);
        for attribute_list in &a.attribute_lists {
            self.walk_attribute_list(attribute_list, c);
        }
        for modifier in &a.modifiers {
            self.walk_modifier(modifier, c);
        }
        if let Some(extends) = &a.extends {
            self.walk_extends(extends, c);
        }
        if let Some(implements) = &a.implements {
            self.walk_implements(implements, c);
        }
        for member in &a.members {
            self.walk_class_like_member(member, c);
        }
        self.walk_out_anonymous_class(a, c);
    }
    fn walk_in_anonymous_class(&mut self, a: &'ast AnonymousClass<'arena>, _: &mut ()) {
        names::class_hierarchy(
            self,
            a.span(),
            "Anonymous class",
            a.extends.as_ref(),
            a.implements.as_ref(),
            false,
            a.modifiers.contains_readonly(),
        );
        let name = String::from_utf8_lossy(
            mago_codex::get_anonymous_class_name(self.file, a.span()).as_bytes(),
        )
        .into_owned();
        classes::anonymous(self, a, &name);
        if self.trait_context.is_none() {
            self.trait_uses_of(&name, a.members.as_slice());
        }
        self.enter_class(Some(name.clone()), a.members.as_slice());
        self.push_frame(FrameKind::Class);
        let mut decl = self.decl_of(
            a.span(),
            None,
            SymbolKind::Class,
            a.modifiers.as_slice(),
            a.extends.as_ref(),
        );
        let display = self.anonymous[name.as_bytes()].clone();
        decl.display = Some(display.clone());
        self.decls.push(decl);
        declarations::duplicates(self, a.members.as_slice());
        attributes::class_like(
            self,
            a.span(),
            a.attribute_lists.as_slice(),
            SymbolKind::Class,
            a.modifiers.contains_readonly(),
            &display,
        );
        if let Some(k) = self.codebase.get_class_like(name.as_bytes()) {
            hierarchy::in_class(self, a.span(), k);
            require_ancestors::check(self, a.span(), k);
            hierarchy::allowed_subtypes(self, a.span(), k);
            require_tags::class_like(self, a.span(), k);
            class_tags::class_like(self, a.span(), k);
            dead_code::unused_private(self, k, a.attribute_lists.as_slice(), a.members.as_slice());
            readonly_init::class(self, k, a.members.as_slice());
            types::too_wide_properties(self, k, a.members.as_slice());
        }
        generics::anonymous(self, a, &name, &display);
        invalid_throws::anonymous(self, a);
        constants::read_only_class(self, a.span(), a.modifiers.as_slice(), true);
    }

    fn walk_in_function(&mut self, f: &'ast Function<'arena>, _: &mut ()) {
        dead_code::impure_function(self, f);
        dead_code::function_parameters(self, f);
        declaration_purity::function(self, f);
        callable_tags::function_like(self, f.span(), &f.parameter_list);
        conditional_return::function(self, f);
        assert_tags::function(self, f);
        phpdoc_types::function(self, f);
        invalid_throws::function(self, f);
        self.enter_this(false);
        functions::inner_function(self, f);
        self.larastan_enter_function(&f.parameter_list);
        missing_types::function(self, f);
        types::function(self, f);
        types::function_defaults(self, f);
        types::function_returns(self, f);
        types::too_wide_function(self, f);
        types::param_out_function(self, f);
        types::throws_void_function(self, f);
        types::throws_function(self, f);
        generics::function(self, f);
        variance::function(self, f);
        functions::function_like(self, f, &f.parameter_list);
        self.push_frame(FrameKind::Named);
        self.frames.last_mut().expect("pushed").defined = Some(var_names::defined(self, var_names::params(&f.parameter_list), &f.body));
        var_names::misplaced(self, f.span(), "a function");
        let name = self.resolved_span(f.name.span);
        names::typehints(
            self,
            names::FunctionKind::Function(&name),
            &f.parameter_list,
            f.return_type_hint.as_ref(),
        );
        signatures::check(
            self,
            names::FunctionKind::Function(&name),
            f.span(),
            &f.parameter_list,
            f.return_type_hint.as_ref(),
        );
        if self.level >= 2 {
            let types = declared::with_classes(self, || declared::function_phpdoc(self, f));
            let kind = names::FunctionKind::Function(&name);
            signatures::phpdoc_classes(self, kind, f.span(), &f.parameter_list, f.return_type_hint.as_ref(), &types);
        }
        let docblock = self
            .codebase
            .get_function(name.as_bytes())
            .and_then(|f| f.return_type_metadata.as_ref())
            .filter(|t| t.from_docblock)
            .map(|t| &t.type_union);
        names::no_discard(
            self,
            names::FunctionKind::Function(&name),
            f.span(),
            f.attribute_lists.as_slice(),
            f.return_type_hint.as_ref(),
            false,
            docblock,
        );
        attributes::check(
            self,
            f.attribute_lists.as_slice(),
            attributes::TARGET_FUNCTION,
            "function",
        );
        declarations::promoted(
            self,
            f.span(),
            declarations::FunctionLike::Other,
            &f.parameter_list,
        );
        syntax::function_like_types(self, Some(&f.parameter_list), f.return_type_hint.as_ref());
        self.syntax.by_ref.push(f.ampersand.is_some());
        self.enter_function(Vec::new());
        self.var_tag_functions.push(f.span());
        let deprecated = self.entered_deprecated("", false, f.span());
        self.functions.push(FunctionScope {
            name: String::from_utf8_lossy(f.name.value).into_owned(),
            kind: FunctionScopeKind::Function,
            deprecated,
            start: f.span().start.offset,
        });
    }
    fn walk_out_function(&mut self, _: &'ast Function<'arena>, _: &mut ()) {
        self.var_tag_functions.pop();
        self.leave_this();
        self.larastan_leave_function();
        self.syntax.by_ref.pop();
        self.pop_frame();
        self.leave_function();
        self.functions.pop();
    }
    fn walk_in_method(&mut self, m: &'ast Method<'arena>, _: &mut ()) {
        dead_code::impure_method(self, m);
        declaration_purity::method(self, m);
        phpdoc_tags::node(self, m.span());
        callable_tags::function_like(self, m.span(), &m.parameter_list);
        conditional_return::method(self, m);
        assert_tags::method(self, m);
        phpdoc_types::method(self, m);
        invalid_throws::method(self, m);
        phpdoc_types::promoted(self, m);
        self.enter_this(!m.modifiers.contains_static());
        functions::function_like(self, m, &m.parameter_list);
        statements::constructor_return_type(self, m);
        larastan::method(self, m);
        self.larastan_enter_function(&m.parameter_list);
        self.push_frame(FrameKind::Named);
        if let MethodBody::Concrete(body) = &m.body {
            self.frames.last_mut().expect("pushed").defined = Some(var_names::defined(self, var_names::params(&m.parameter_list), body));
        }
        var_names::misplaced(self, m.span(), "a method");
        for p in m
            .parameter_list
            .parameters
            .iter()
            .filter(|p| p.is_promoted_property())
        {
            names::property_type(self, p.span(), p.hint.as_ref(), &[p.variable.name]);
            signatures::property(self, p.span(), p.hint.as_ref(), &[p.variable.name]);
            if let Some(hooks) = &p.hooks {
                signatures::hooks(self, p.variable.name, hooks);
                signatures::hook_phpdoc_classes(self, p.variable.name, hooks, p.span());
                types::get_hooks(self, p.variable.name, p.hint.as_ref(), p.span(), hooks);
                types::hook_returns(self, p.variable.name, p.hint.as_ref(), p.span(), hooks);
                types::throws_void_hooks(self, p.variable.name, hooks);
                types::throws_hooks(self, p.variable.name, hooks);
                set_hook::hooks(self, p.variable.name, p.hint.as_ref(), hooks);
                set_hook::missing_types(self, p.variable.name, p.hint.as_ref(), hooks, p.span());
            }
        }
        // a trait method under its `as` alias
        let method = match &self.renamed {
            Some((span, alias)) if *span == m.span() => alias.clone(),
            _ => String::from_utf8_lossy(m.name.value).into_owned(),
        };
        declarations::method(self, m, &method);
        hierarchy::method(self, m, &method);
        overriding::method(self, m, &method);
        overriding::inherited_case(self, m, &method);
        strict::constructor(self, m);
        hierarchy::interface_method_visibility(self, m);
        dead_code::method_parameters(self, m, &method);
        missing_types::method(self, m);
        types::method(self, m);
        types::method_defaults(self, m);
        types::method_returns(self, m);
        types::too_wide_method(self, m);
        types::param_out_method(self, m);
        types::self_out(self, m);
        types::throws_void_method(self, m);
        types::throws_method(self, m);
        types::throws_covariance(self, m);
        generics::method(self, m, &method);
        variance::method(self, m, &method);
        declarations::promoted(
            self,
            m.span(),
            declarations::FunctionLike::Method {
                name: &method,
                original: m.name.value,
                body: matches!(m.body, MethodBody::Concrete(_)),
            },
            &m.parameter_list,
        );
        if method.eq_ignore_ascii_case("__construct")
            || m.name.value.eq_ignore_ascii_case(b"__construct")
        {
            properties::promoted(self, &m.parameter_list);
            variance::promoted(self, m);
            for p in m
                .parameter_list
                .parameters
                .iter()
                .filter(|p| p.is_promoted_property())
            {
                hierarchy::callable_property(self, p.span(), p.variable.name, p.hint.as_ref());
            }
        }
        // trait bodies are walked only in a class's context, so a method always has its class-like
        let class = self
            .class_scope()
            .expect("a method is walked inside its class-like");
        let docblock = self
            .codebase
            .get_method(class.name.as_bytes(), method.as_bytes())
            .and_then(|f| f.return_type_metadata.as_ref())
            .filter(|t| t.from_docblock)
            .map(|t| &t.type_union);
        let class = self.display(class);
        let kind = names::FunctionKind::Method(&class, &method);
        names::typehints(self, kind, &m.parameter_list, m.return_type_hint.as_ref());
        signatures::check(
            self,
            kind,
            m.span(),
            &m.parameter_list,
            m.return_type_hint.as_ref(),
        );
        if self.level >= 2
            && let Some(context) = self.class_scope()
            && let Some(declaring) = missing_types::declaring(self, m.span())
        {
            let types = declared::with_classes(self, || declared::method_phpdoc(self, m, declaring, context));
            signatures::phpdoc_classes(self, kind, m.span(), &m.parameter_list, m.return_type_hint.as_ref(), &types);
        }
        // PhpMethodFromParserNodeReflection's void methods
        let forced_void = m.name.value.eq_ignore_ascii_case(b"__construct")
            || [
                "__construct",
                "__destruct",
                "__unset",
                "__wakeup",
                "__clone",
                "__set",
                "__unserialize",
            ]
            .iter()
            .any(|n| method.eq_ignore_ascii_case(n));
        names::no_discard(
            self,
            kind,
            m.span(),
            m.attribute_lists.as_slice(),
            m.return_type_hint.as_ref(),
            forced_void,
            docblock,
        );
        syntax::function_like_types(self, Some(&m.parameter_list), m.return_type_hint.as_ref());
        if m.name.value.eq_ignore_ascii_case(b"__construct") {
            for p in m
                .parameter_list
                .parameters
                .iter()
                .filter(|p| p.is_promoted_property())
            {
                if let Some(hint) = &p.hint {
                    syntax::complex_type(self, hint);
                }
            }
        }
        self.syntax.by_ref.push(m.ampersand.is_some());
        attributes::check(
            self,
            m.attribute_lists.as_slice(),
            attributes::TARGET_METHOD,
            "method",
        );
        // a static method can run before any instance exists
        let guards =
            if m.name.value.eq_ignore_ascii_case(b"__construct") || m.modifiers.contains_static() {
                Vec::new()
            } else {
                self.classes
                    .last()
                    .map(|c| c.constructor_guards.clone())
                    .unwrap_or_default()
            };
        self.enter_function(guards);
        self.var_tag_functions.push(m.span());
        let deprecated = self.entered_deprecated(&method, true, m.span());
        self.functions.push(FunctionScope {
            deprecated,
            start: m.span().start.offset,
            name: method,
            kind: FunctionScopeKind::Method {
                is_static: m.modifiers.contains_static(),
            },
        });
    }
    fn walk_out_method(&mut self, _: &'ast Method<'arena>, _: &mut ()) {
        self.var_tag_functions.pop();
        self.leave_this();
        self.larastan_leave_function();
        self.syntax.by_ref.pop();
        self.pop_frame();
        self.leave_function();
        self.functions.pop();
    }

    fn walk_in_block(&mut self, _: &'ast Block<'arena>, _: &mut ()) {
        self.marks.push(self.guards.len());
    }
    fn walk_out_block(&mut self, b: &'ast Block<'arena>, _: &mut ()) {
        phpdoc_tags::nop(self, b.right_brace);
        self.end_guard_block();
    }

    fn walk_if(&mut self, r#if: &'ast If<'arena>, c: &mut ()) {
        dead_code::if_statement(self, r#if);
        if self.strict.booleans_in_conditions {
            operators::condition(self, r#if.span(), r#if.condition, "an if condition", "if.condNotBoolean");
            for (at, cond) in operators::else_if_conditions(r#if) {
                operators::condition(self, at, cond, "an elseif condition", "elseif.condNotBoolean");
            }
        }
        self.walk_expression(r#if.condition, c);
        // (condition, statements) per branch; `None` is the `else`
        let mut branches: Vec<(Option<&Expression<'arena>>, Vec<&Statement<'arena>>)> = Vec::new();
        let has_else = match &r#if.body {
            IfBody::Statement(b) => {
                branches.push((Some(r#if.condition), vec![b.statement]));
                branches.extend(
                    b.else_if_clauses
                        .iter()
                        .map(|e| (Some(e.condition), vec![e.statement])),
                );
                if let Some(e) = &b.else_clause {
                    branches.push((None, vec![e.statement]));
                }
                b.else_if_clauses
                    .iter()
                    .for_each(|e| phpdoc_tags::clause(self, e.span()));
                b.else_clause
                    .iter()
                    .for_each(|e| phpdoc_tags::clause(self, e.span()));
                b.else_clause.is_some()
            }
            IfBody::ColonDelimited(b) => {
                branches.push((Some(r#if.condition), b.statements.iter().collect()));
                branches.extend(
                    b.else_if_clauses
                        .iter()
                        .map(|e| (Some(e.condition), e.statements.iter().collect())),
                );
                if let Some(e) = &b.else_clause {
                    branches.push((None, e.statements.iter().collect()));
                }
                b.else_if_clauses
                    .iter()
                    .for_each(|e| phpdoc_tags::clause(self, e.span()));
                b.else_clause
                    .iter()
                    .for_each(|e| phpdoc_tags::clause(self, e.span()));
                b.else_clause.is_some()
            }
        };
        let mut previous_false = Vec::new();
        let mut survivors: Vec<Vec<Guard>> = Vec::new();
        let mut expressions = Vec::new();
        let mut assignment_paths = Vec::new();
        let mut remaining_assignment_path = true;
        for (i, (cond, statements)) in branches.into_iter().enumerate() {
            let mut entry = previous_false.clone();
            let mut known = None;
            if let Some(cond) = cond {
                if i > 0 {
                    let mut assignments = self.assign.expressions.clone();
                    self.with_guards(previous_false.clone(), |s| {
                        s.walk_expression(cond, &mut ());
                        assignments = s.assign.expressions.clone();
                    });
                    self.assign.expressions = assignments;
                }
                // IfHandler prices the condition on the running falsey scope, before entering its branch.
                known = match types::flow::unparen(cond) {
                    Expression::Literal(Literal::True(_)) => Some(true),
                    Expression::Literal(Literal::False(_) | Literal::Null(_)) => Some(false),
                    _ => self.type_of(cond).and_then(|t| {
                        if t.is_always_truthy() { Some(true) } else { t.is_always_falsy().then_some(false) }
                    }),
                };
                entry.extend(self.truthy(cond));
                previous_false.extend(self.falsy(cond));
            }
            let (terminates, outgoing, assignments) = self.walk_branch(entry.clone(), &statements);
            if !terminates {
                expressions.push(outgoing);
                survivors.push(entry);
                if remaining_assignment_path && known != Some(false) {
                    assignment_paths.push(assignments);
                }
            }
            remaining_assignment_path &= known != Some(true);
        }
        if !has_else {
            let mut outgoing = self.expression_guards();
            outgoing.extend(previous_false.iter().filter(|g| scope::definite_expression(g)).cloned());
            expressions.push(outgoing);
            survivors.push(previous_false);
            if remaining_assignment_path {
                assignment_paths.push(self.assign.expressions.clone());
            }
        }
        // after the `if`: what holds in every branch that falls through
        if let Some((first, rest)) = survivors.split_first() {
            let after: Vec<Guard> = first
                .iter()
                .filter(|g| !scope::definite_expression(g) && rest.iter().all(|r| r.contains(g)))
                .cloned()
                .collect();
            self.guards.extend(after);
        }
        let assignments = match assignment_paths.len() {
            0 => self.assign.expressions.clone(),
            1 => assignment_paths.pop().expect("one surviving scope"),
            _ => HashSet::new(),
        };
        self.join_expression_guards(&expressions);
        self.assign.expressions = assignments;
    }

    fn walk_conditional(&mut self, cond: &'ast Conditional<'arena>, c: &mut ()) {
        dead_code::ternary(self, cond);
        // an elvis `?:` is not checked
        if self.strict.booleans_in_conditions && cond.then.is_some() {
            let place = "a ternary operator condition";
            operators::condition(self, cond.span(), cond.condition, place, "ternary.condNotBoolean");
        }
        self.walk_expression(cond.condition, c);
        let t = self.truthy(cond.condition);
        let yes = if let Some(then) = cond.then {
            self.guarded_expression(t, then)
        } else {
            self.expression_guards().into_iter().chain(t.into_iter().filter(scope::definite_expression)).collect()
        };
        let f = self.falsy(cond.condition);
        let no = self.guarded_expression(f, cond.r#else);
        self.join_expression_guards(&[yes, no]);
    }

    fn walk_binary(&mut self, b: &'ast Binary<'arena>, c: &mut ()) {
        types::binary(self, b);
        types::binary_operation(self, b);
        dead_code::binary(self, b);
        if b.operator.is_null_coalesce() {
            dead_code::null_coalesce(self, b.lhs, b.rhs, "??");
        }
        operators::binary(self, b);
        if matches!(b.operator, BinaryOperator::Instanceof(_)) {
            names::instanceof(self, b.lhs, b.rhs);
        }
        self.walk_expression(b.lhs, c);
        let extra = match b.operator {
            BinaryOperator::And(_) | BinaryOperator::LowAnd(_) => Some(self.truthy(b.lhs)),
            BinaryOperator::Or(_) | BinaryOperator::LowOr(_) => Some(self.falsy(b.lhs)),
            _ => None,
        };
        if let Some(extra) = extra {
            let skipped = self.expression_guards();
            let evaluated = self.guarded_expression(extra, b.rhs);
            self.join_expression_guards(&[skipped, evaluated]);
        } else {
            self.walk_expression(b.rhs, c);
        }
    }

    fn walk_match(&mut self, m: &'ast Match<'arena>, c: &mut ()) {
        types::void_match(self, m);
        dead_code::match_expression(self, m);
        self.walk_expression(m.expression, c);
        let on_true = matches!(m.expression, Expression::Literal(Literal::True(_)));
        let assignments = self.assign.expressions.clone();
        for arm in m.arms.iter() {
            self.assign.expressions = assignments.clone();
            match arm {
                MatchArm::Expression(arm) => {
                    for cond in arm.conditions.iter() {
                        self.walk_expression(cond, c);
                    }
                    let extra = match (
                        on_true,
                        arm.conditions.iter().collect::<Vec<_>>().as_slice(),
                    ) {
                        (true, [one]) => self.truthy(one),
                        _ => Vec::new(),
                    };
                    self.mark_top_level(arm.expression);
                    self.with_guards(extra, |s| s.walk_expression(arm.expression, &mut ()));
                }
                MatchArm::Default(arm) => {
                    self.mark_top_level(arm.expression);
                    self.walk_expression(arm.expression, c);
                }
            }
        }
        // Multiple result scopes go through mergeWith, which clears the assignment-expression table.
        if m.arms.len() > 1 {
            self.assign.expressions.clear();
        }
    }

    fn walk_assignment(&mut self, a: &'ast Assignment<'arena>, context: &mut ()) {
        self.walk_in_assignment(a, context);
        if assign::is_list_target(a.lhs) {
            // KIND_LIST applies its targets after the RHS, keeping every item's key before its value.
            self.walk_expression(a.rhs, context);
            self.walk_expression(a.lhs, context);
        } else {
            self.walk_expression(a.lhs, context);
            self.walk_expression(a.rhs, context);
        }
        self.walk_assignment_operator(&a.operator, context);
        self.walk_out_assignment(a, context);
    }

    fn walk_in_assignment(&mut self, a: &'ast Assignment<'arena>, _: &mut ()) {
        if matches!(a.operator, AssignmentOperator::Coalesce(_)) {
            dead_code::null_coalesce(self, a.lhs, a.rhs, "??=");
        }
        property_access::assign(self, a);
        assign::assignment(self, a);
        assign::property_assign_ref(self, a);
        types::assignment(self, a);
        types::property_assignment(self, a);
        types::offset_assignment(self, a);
        types::assign_operation(self, a);
        operators::assignment(self, a);
        if let AssignmentOperator::Assign(_) = a.operator {
            arrays::destructuring(self, a.lhs, true);
            if matches!(types::flow::unparen(a.lhs), Expression::Array(_) | Expression::List(_)) {
                let value = self.phpstan_type(a.rhs);
                types::destructuring(self, a.span(), a.lhs, value, Some(a.rhs.span()));
            }
        }
        if let Expression::Variable(Variable::Direct(v)) = a.lhs {
            let guards = self.truthy(a.rhs).into_iter().filter(scope::remembered_guard).collect::<Vec<_>>();
            if guards.is_empty() {
                self.vars.remove(v.name);
            } else {
                self.vars.insert(v.name.to_vec(), guards);
            }
        }
    }
    fn walk_out_assignment(&mut self, a: &'ast Assignment<'arena>, _: &mut ()) {
        self.assigned_expression(a);
        property_access::assigned(self, a);
        include_file::assigned(self, a.lhs);
    }
    fn walk_in_construct(&mut self, construct: &'ast Construct<'arena>, _: &mut ()) {
        include_file::construct(self, construct);
        match construct {
            Construct::Isset(i) => dead_code::isset(self, i),
            Construct::Empty(e) => dead_code::empty(self, e),
            _ => {}
        }
    }
    fn walk_in_property_access(&mut self, p: &'ast PropertyAccess<'arena>, _: &mut ()) {
        property_access::read(self, p);
        write_only::read(self, p.span(), p.object, &p.property);
        internal::property_fetch(self, p);
    }
    fn walk_in_null_safe_property_access(&mut self, p: &'ast NullSafePropertyAccess<'arena>, _: &mut ()) {
        property_access::null_safe_read(self, p);
        write_only::null_safe(self, p);
        types::nullsafe_property(self, p);
    }
    fn walk_in_static_property_access(
        &mut self,
        p: &'ast StaticPropertyAccess<'arena>,
        _: &mut (),
    ) {
        self.variables.property_selectors.insert(p.property.span());
        property_access::static_read(self, p);
        through_static::property(self, p);
        internal::static_property_fetch(self, p);
    }
    fn walk_out_anonymous_class(&mut self, a: &'ast AnonymousClass<'arena>, _: &mut ()) {
        phpdoc_tags::nop(self, a.right_brace);
        self.classes.pop();
        self.pop_frame();
        self.decls.pop();
    }

    fn walk_in_variable(&mut self, variable: &'ast Variable<'arena>, _: &mut ()) {
        variables::variable(self, variable);
    }

    fn walk_in_closure_use_clause_variable(&mut self, capture: &'ast ClosureUseClauseVariable<'arena>, _: &mut ()) {
        variables::capture(self, capture);
    }

    fn walk_in_function_call(&mut self, call: &'ast FunctionCall<'arena>, _: &mut ()) {
        variables::compact(self, call);
        self.bind_function_arguments(call);
        dead_code::function_check(self, call);
        debug::function_call(self, call);
        larastan::function_call(self, call);
        types::function_call(self, call);
        types::castable_call(self, call);
        types::define_value(self, call);
        function_checks::function_call(self, call);
        restricted_functions::function_call(self, call);
        deprecations::ini_option(self, call);
        calls::function_call(self, call);
        types::callable_call(self, call);
        types::call_user_func(self, call);
        dead_code::function_call(self, call);
        dead_code::sort_call(self, call);
        dead_code::haystack_call(self, call);
        assign::arguments(self, assign::Callee::Function(call));
        assign::function_virtual_assign(self, call);
        self.in_exists_call += usize::from(calls::is_class_exists_family(self, call));
        self.guards.extend(self.define_guard(call));
    }
    fn walk_out_function_call(&mut self, call: &'ast FunctionCall<'arena>, _: &mut ()) {
        self.in_exists_call -= usize::from(calls::is_class_exists_family(self, call));
        let references = assign::by_ref_arguments(self, call);
        self.invalidate_call_references(&call.argument_list, references);
    }

    fn walk_in_instantiation(&mut self, new: &'ast Instantiation<'arena>, _: &mut ()) {
        new_static_abstract::instantiation(self, new);
        if statements::instantiation_callable(self, new) {
            return;
        }
        classes::instantiation(self, new);
        date_interval::instantiation(self, new);
        date_time::instantiation(self, new);
        assign::arguments(self, assign::Callee::New(new));
    }

    fn walk_in_null_safe_method_call(
        &mut self,
        call: &'ast NullSafeMethodCall<'arena>,
        _: &mut (),
    ) {
        self.bind_method_arguments(call.object, &call.method, &call.argument_list, false);
        if self.method_rules.call_methods {
            named_argument::call(self, call.span(), call.object, &call.argument_list, true);
        }
        internal::method_call(self, call.span(), call.object, &call.method);
        strict::dynamic_call(self, call.span(), call.object, &call.method);
        if self.method_rules.call_methods {
            types::method_call(self, call.span(), call.object, &call.method, &call.argument_list, true);
            if let Some(name) = types::dynamic_names::member(&call.method) {
                types::dynamic_names::check(self, name, false, "Method", call.object, true, "method.nameNotString");
            }
        }
        types::nullsafe_method(self, call);
    }
    fn walk_in_method_call(&mut self, call: &'ast MethodCall<'arena>, _: &mut ()) {
        self.bind_method_arguments(call.object, &call.method, &call.argument_list, false);
        if self.method_rules.call_methods {
            named_argument::call(self, call.span(), call.object, &call.argument_list, false);
        }
        dead_code::method_check(self, call);
        internal::method_call(self, call.span(), call.object, &call.method);
        strict::dynamic_call(self, call.span(), call.object, &call.method);
        larastan::method_call(self, call);
        if let ClassLikeMemberSelector::Identifier(m) = &call.method
            && m.value.eq_ignore_ascii_case(b"call")
        {
            // MutatingScope::enterClosureCall() defines `$this` as the first argument's type
            let class = self.call_class(&call.argument_list);
            let defines_this = !call.argument_list.arguments.is_empty()
                && match types::flow::unparen(call.object) {
                    Expression::Closure(f) => f.r#static.is_none(),
                    Expression::ArrowFunction(f) => f.r#static.is_none(),
                    _ => false,
                };
            self.rebind(call.object, class, defines_this);
        }
        // A static closure has no `$this`, even when its parameter carries a binding annotation. The undefined
        // variable check reports it; there is no receiver on which to check methods.
        if self.method_rules.call_methods && self.this_defined() {
            calls::method_call(self, call);
        }
        if self.method_rules.call_methods {
            types::method_call(self, call.span(), call.object, &call.method, &call.argument_list, false);
            if let Some(name) = types::dynamic_names::member(&call.method) {
                types::dynamic_names::check(self, name, false, "Method", call.object, false, "method.nameNotString");
            }
        }
        assign::arguments(self, assign::Callee::Method(call));
    }
    fn walk_out_method_call(&mut self, call: &'ast MethodCall<'arena>, _: &mut ()) {
        self.invalidate_called_receiver(call.object, &call.method);
        let references = assign::by_ref_call_arguments(self, assign::Callee::Method(call), &call.argument_list);
        self.invalidate_call_references(&call.argument_list, references);
    }
    fn walk_in_pipe(&mut self, p: &'ast Pipe<'arena>, _: &mut ()) {
        self.piped.insert(p.callable.span());
        types::pipe_by_ref(self, p);
        calls::pipe(self, p);
        types::callable_pipe(self, p);
        function_checks::pipe(self, p);
        restricted_functions::pipe(self, p);
        types::pipe_operation(self, p);
    }
    fn walk_in_function_partial_application(
        &mut self,
        f: &'ast FunctionPartialApplication<'arena>,
        _: &mut (),
    ) {
        let whole = f.function.span().join(f.argument_list.span());
        if !statements::first_class_callable(self, whole, &f.argument_list) {
            function_callable::check(self, f);
            restricted_functions::callable(self, f);
        }
    }
    fn walk_in_method_partial_application(
        &mut self,
        m: &'ast MethodPartialApplication<'arena>,
        _: &mut (),
    ) {
        if !statements::first_class_callable(self, m.span(), &m.argument_list) {
            method_callable::method(self, m);
        }
        internal::method_callable(self, m);
        if internal::is_callable(&m.argument_list) {
            strict::dynamic_call(self, m.span(), m.object, &m.method);
        }
    }
    fn walk_in_static_method_partial_application(
        &mut self,
        m: &'ast StaticMethodPartialApplication<'arena>,
        _: &mut (),
    ) {
        if !statements::first_class_callable(self, m.span(), &m.argument_list) {
            method_callable::static_method(self, m);
        }
        internal::static_method_callable(self, m);
    }
    fn walk_in_static_method_call(&mut self, call: &'ast StaticMethodCall<'arena>, _: &mut ()) {
        self.bind_method_arguments(call.class, &call.method, &call.argument_list, true);
        dead_code::static_method_check(self, call);
        through_static::method(self, call);
        internal::static_method_call(self, call);
        larastan::static_method_call(self, call);
        if self.is_closure_bind(call) {
            self.closure_bind += 1;
            if let Some(Argument::Positional(a)) = call.argument_list.arguments.first() {
                let class = self.bind_class(&call.argument_list);
                self.rebind(a.value, class, false);
            }
        }
        calls::static_method_call(self, call);
        types::static_call(self, call);
        if let Some(name) = types::dynamic_names::member(&call.method) {
            types::dynamic_names::check(self, name, false, "Method", call.class, false, "staticMethod.nameNotString");
        }
        assign::arguments(self, assign::Callee::StaticMethod(call));
    }
    fn walk_out_static_method_call(&mut self, call: &'ast StaticMethodCall<'arena>, _: &mut ()) {
        self.invalidate_static_call(call);
        let references = assign::by_ref_call_arguments(self, assign::Callee::StaticMethod(call), &call.argument_list);
        self.invalidate_call_references(&call.argument_list, references);
        if self.is_closure_bind(call) {
            self.closure_bind -= 1;
        }
    }
    // a rebound closure is checked in its bound `$this`'s class (none when Pun cannot tell it)
    fn walk_out_closure(&mut self, f: &'ast Closure<'arena>, _: &mut ()) {
        self.assign.leave_scope();
        self.leave_this();
        self.larastan_leave_function();
        self.syntax.parents.pop();
        self.syntax.by_ref.pop();
        self.pop_frame();
        if self.rebound.contains_key(&f.span()) {
            self.classes.pop();
        }
    }
    fn walk_out_arrow_function(&mut self, f: &'ast ArrowFunction<'arena>, _: &mut ()) {
        self.assign.leave_scope();
        self.leave_this();
        self.pop_frame();
        if self.rebound.contains_key(&f.span()) {
            self.classes.pop();
        }
    }

    fn walk_in_statement_expression(&mut self, s: &'ast ExpressionStatement<'arena>, _: &mut ()) {
        self.mark_top_level(s.expression);
        statements::standalone_throw(self, s);
        dead_code::statement(self, s);
    }
    fn walk_in_clone(&mut self, k: &'ast Clone<'arena>, _: &mut ()) {
        types::clone(self, k);
    }
    fn walk_in_array_access(&mut self, a: &'ast ArrayAccess<'arena>, _: &mut ()) {
        types::array_access(self, a);
        types::dim_fetch(self, a.span(), a.array, Some(a.index));
    }
    fn walk_in_key_value_array_element(&mut self, kv: &'ast KeyValueArrayElement<'arena>, _: &mut ()) {
        types::array_item(self, kv);
    }
    fn walk_in_throw(&mut self, t: &'ast Throw<'arena>, _: &mut ()) {
        statements::throw_expression(self, t);
        types::throw(self, t);
    }
    fn walk_in_array(&mut self, a: &'ast Array<'arena>, _: &mut ()) {
        arrays::literal_array(self, a.span(), &a.elements);
        types::array(self, a.elements.as_slice());
    }
    fn walk_in_legacy_array(&mut self, a: &'ast LegacyArray<'arena>, _: &mut ()) {
        arrays::literal_array(self, a.span(), &a.elements);
        types::array(self, a.elements.as_slice());
    }
    fn walk_in_for(&mut self, f: &'ast For<'arena>, _: &mut ()) {
        self.syntax.parents.push(syntax::Parent::Loop);
        for e in f.initializations.iter().chain(f.increments.iter()) {
            self.mark_top_level(e);
        }
    }
    fn walk_in_trait_use(&mut self, u: &'ast mago_syntax::cst::TraitUse<'arena>, _: &mut ()) {
        phpdoc_tags::clause(self, u.span());
        hierarchy::trait_use(self, u);
        generics::trait_use(self, u);
    }
    fn walk_in_use(&mut self, u: &'ast Use<'arena>, _: &mut ()) {
        names::use_functions(self, u);
        names::use_classes(self, u);
        signatures::use_statement(self, u);
        class_names::use_statement(self, u);
    }
    fn walk_in_property(&mut self, p: &'ast Property<'arena>, _: &mut ()) {
        phpdoc_tags::node(self, p.span());
        invalid_throws::node(self, p.span());
        let names: Vec<&[u8]> = p.variables().iter().map(|v| v.name).collect();
        names::property_type(self, p.span(), p.hint(), &names);
        signatures::property(self, p.span(), p.hint(), &names);
        if let Property::Hooked(h) = p {
            signatures::hooks(self, h.item.variable().name, &h.hook_list);
            declaration_purity::hooks(self, h.item.variable().name, &h.hook_list);
            signatures::hook_phpdoc_classes(self, h.item.variable().name, &h.hook_list, p.span());
            set_hook::hooks(self, h.item.variable().name, p.hint(), &h.hook_list);
            set_hook::missing_types(self, h.item.variable().name, p.hint(), &h.hook_list, p.span());
            types::get_hooks(self, h.item.variable().name, p.hint(), p.span(), &h.hook_list);
            types::hook_returns(self, h.item.variable().name, p.hint(), p.span(), &h.hook_list);
            types::throws_void_hooks(self, h.item.variable().name, &h.hook_list);
            types::throws_hooks(self, h.item.variable().name, &h.hook_list);
            for hook in h.hook_list.hooks.iter() {
                phpdoc_types::hook(self, hook, h.item.variable().name, p.hint());
            }
        }
        properties::property(self, p);
        missing_types::property(self, p);
        types::property_default(self, p);
        phpdoc_types::property(self, p);
        larastan::property(self, p);
        for v in p.variables() {
            hierarchy::callable_property(self, v.span(), v.name, p.hint());
        }
        // one ClassPropertyNode per property of the declaration
        if let Some(hint) = p.hint() {
            for _ in &names {
                syntax::complex_type(self, hint);
            }
        }
        let lists = match p {
            Property::Plain(p) => p.attribute_lists.as_slice(),
            Property::Hooked(h) => h.attribute_lists.as_slice(),
        };
        for v in p.variables() {
            attributes::property(self, v.span(), lists, false);
        }
    }
    fn walk_in_class_like_constant(&mut self, k: &'ast ClassLikeConstant<'arena>, _: &mut ()) {
        phpdoc_tags::node(self, k.span());
        invalid_throws::node(self, k.span());
        declarations::constant(self, k);
        constants::class_constant(self, k);
        missing_types::class_constant(self, k);
        phpdoc_types::class_constant(self, k);
        types::class_constant(self, k);
        attributes::check(
            self,
            k.attribute_lists.as_slice(),
            attributes::TARGET_CLASS_CONSTANT,
            "class constant",
        );
    }
    fn walk_in_enum_case(&mut self, case: &'ast EnumCase<'arena>, _: &mut ()) {
        phpdoc_tags::clause(self, case.span());
        enums::case_outside(self, case);
        attributes::check(
            self,
            case.attribute_lists.as_slice(),
            attributes::TARGET_CLASS_CONSTANT,
            "class constant",
        );
    }
    fn walk_in_constant(&mut self, k: &'ast Constant<'arena>, _: &mut ()) {
        attributes::constant(self, k);
        types::global_constant(self, k);
    }
    fn walk_in_closure(&mut self, f: &'ast Closure<'arena>, _: &mut ()) {
        self.assign.enter_scope();
        strict::closure(self, f);
        if self.rebound.get(&f.span()).is_some_and(|b| b.defines_this) {
            self.enter_this(true);
        } else {
            self.enter_closure_this(f.r#static.is_some());
        }
        functions::function_like(self, f, &f.parameter_list);
        functions::closure_use(self, f);
        self.larastan_enter_closure(f);
        dead_code::closure(self, f);
        types::closure(self, f);
        types::anonymous_defaults(self, &f.parameter_list);
        types::closure_returns(self, f);
        types::too_wide_closure(self, f);
        syntax::function_like_types(self, Some(&f.parameter_list), f.return_type_hint.as_ref());
        self.syntax.parents.push(syntax::Parent::Closure);
        self.syntax.by_ref.push(f.ampersand.is_some());
        self.push_frame(FrameKind::Anonymous);
        self.frames.last_mut().expect("pushed").defined =
            Some(var_names::defined(
                self,
                var_names::params(&f.parameter_list)
                    .chain(f.use_clause.iter().flat_map(|u| u.variables.iter()).map(|u| u.variable.name)),
                &f.body,
            ));
        if let Some(binding) = self.rebound.get(&f.span()) {
            self.classes.push(ClassScope {
                name: binding.class.clone(),
                constructor_guards: Vec::new(),
            });
        }
        names::typehints(
            self,
            names::FunctionKind::Anonymous,
            &f.parameter_list,
            f.return_type_hint.as_ref(),
        );
        signatures::check(
            self,
            names::FunctionKind::Anonymous,
            f.span(),
            &f.parameter_list,
            f.return_type_hint.as_ref(),
        );
        names::no_discard(
            self,
            names::FunctionKind::Anonymous,
            f.span(),
            f.attribute_lists.as_slice(),
            f.return_type_hint.as_ref(),
            false,
            None,
        );
        attributes::check(
            self,
            f.attribute_lists.as_slice(),
            attributes::TARGET_FUNCTION,
            "function",
        );
        declarations::promoted(
            self,
            f.span(),
            declarations::FunctionLike::Other,
            &f.parameter_list,
        );
    }
    fn walk_in_class_constant_access(&mut self, a: &'ast ClassConstantAccess<'arena>, _: &mut ()) {
        through_static::constant(self, a);
        names::class_constant(self, a);
        if let Some(name) = types::dynamic_names::constant(&a.constant) {
            types::dynamic_names::check(self, name, false, "Class constant", a.class, false, "classConstant.nameNotString");
        }
        dynamic_fetch::class_constant(self, a);
        internal_usage::class_constant(self, a);
    }
    fn walk_in_try(&mut self, t: &'ast Try<'arena>, _: &mut ()) {
        dead_code::try_statement(self, t);
        types::dead_catches(self, t);
        types::unthrown_catches(self, t);
    }
    fn walk_in_try_catch_clause(&mut self, clause: &'ast TryCatchClause<'arena>, _: &mut ()) {
        phpdoc_tags::clause(self, clause.span());
        statements::noncapturing_catch(self, clause);
        names::catch(self, clause);
        assign::catch(self, clause);
    }
    fn walk_in_arrow_function(&mut self, f: &'ast ArrowFunction<'arena>, _: &mut ()) {
        self.assign.enter_scope();
        types::anonymous_defaults(self, &f.parameter_list);
        types::arrow_function_returns(self, f);
        types::too_wide_arrow_function(self, f);
        if self.rebound.get(&f.span()).is_some_and(|b| b.defines_this) {
            self.enter_this(true);
        } else {
            self.enter_closure_this(f.r#static.is_some());
        }
        functions::function_like(self, f, &f.parameter_list);
        syntax::function_like_types(self, Some(&f.parameter_list), f.return_type_hint.as_ref());
        syntax::arrow_function(self, f);
        signatures::arrow_never(self, f);
        self.push_frame(FrameKind::Anonymous);
        if let Some(binding) = self.rebound.get(&f.span()) {
            self.classes.push(ClassScope {
                name: binding.class.clone(),
                constructor_guards: Vec::new(),
            });
        }
        names::typehints(
            self,
            names::FunctionKind::Anonymous,
            &f.parameter_list,
            f.return_type_hint.as_ref(),
        );
        signatures::check(
            self,
            names::FunctionKind::Anonymous,
            f.span(),
            &f.parameter_list,
            f.return_type_hint.as_ref(),
        );
        names::no_discard(
            self,
            names::FunctionKind::Anonymous,
            f.span(),
            f.attribute_lists.as_slice(),
            f.return_type_hint.as_ref(),
            false,
            None,
        );
        attributes::check(
            self,
            f.attribute_lists.as_slice(),
            attributes::TARGET_FUNCTION,
            "function",
        );
        declarations::promoted(
            self,
            f.span(),
            declarations::FunctionLike::Other,
            &f.parameter_list,
        );
        self.mark_top_level(f.expression);
    }

    fn walk_in_program(&mut self, program: &'ast Program<'arena>, _: &mut ()) {
        self.has_compiler_halt = names::file_has_compiler_halt_statement_calls(program);
        syntax::used_names(self, program);
        self.unreachable = dead_code::unreachable_statements(self, program);
        self.conditions = dead_code::condition_facts(self, program);
        if self.toggled.by_ref_foreach {
            self.by_ref_foreach = by_ref_foreach::entries(program);
        }
        self.push_frame(FrameKind::Top);
        types::yields_outside(self, program);
    }
    fn walk_out_program(&mut self, program: &'ast Program<'arena>, _: &mut ()) {
        let end = mago_span::Position::new(program.source_text.len() as u32);
        phpdoc_tags::nop(self, Span::new(program.file_id, end, end));
        self.pop_frame();
    }
    fn walk_in_namespace(&mut self, n: &'ast Namespace<'arena>, _: &mut ()) {
        let name = n.name.as_ref().map(|id| self.resolved(id));
        signatures::enter_namespace(self, name.as_deref());
        class_names::enter_namespace(self);
        self.namespaces.push(name);
    }
    fn walk_out_namespace(&mut self, _: &'ast Namespace<'arena>, _: &mut ()) {
        signatures::leave_namespace(self);
        self.namespaces.pop();
    }
    fn walk_in_property_hook(&mut self, h: &'ast PropertyHook<'arena>, _: &mut ()) {
        phpdoc_tags::node(self, h.span());
        invalid_throws::node(self, h.span());
        self.enter_this(true);
        attributes::property_hook(self, h);
        if let Some(params) = &h.parameter_list {
            functions::function_like(self, h, params);
            declarations::promoted(self, h.span(), declarations::FunctionLike::Other, params);
        }
        syntax::function_like_types(self, h.parameter_list.as_ref(), None);
        self.syntax.by_ref.push(h.ampersand.is_some());
        self.push_frame(FrameKind::Named);
        if let PropertyHookBody::Concrete(PropertyHookConcreteBody::Block(body)) = &h.body {
            let names: Vec<&[u8]> = match &h.parameter_list {
                Some(list) => var_names::params(list).collect(),
                None if h.name.value == b"set" => vec![b"$value"],
                None => Vec::new(),
            };
            self.frames.last_mut().expect("pushed").defined = Some(var_names::defined(self, names, body));
        }
    }
    fn walk_out_property_hook(&mut self, _: &'ast PropertyHook<'arena>, _: &mut ()) {
        self.leave_this();
        self.syntax.by_ref.pop();
        self.pop_frame();
    }
    fn walk_in_function_like_parameter(
        &mut self,
        p: &'ast FunctionLikeParameter<'arena>,
        _: &mut (),
    ) {
        functions::parameter(self, p);
        statements::parameter_default(self, p);
        attributes::parameter(self, p);
        if !p.modifiers.is_empty() {
            attributes::property(self, p.span(), p.attribute_lists.as_slice(), true);
        }
        self.push_frame(FrameKind::Param);
    }
    fn walk_out_function_like_parameter(
        &mut self,
        _: &'ast FunctionLikeParameter<'arena>,
        _: &mut (),
    ) {
        self.pop_frame();
    }
    fn walk_statement(&mut self, s: &'ast Statement<'arena>, context: &mut ()) {
        if self.unreachable.reported.contains(&s.span()) {
            self.report(s, "deadCode.unreachable", "Unreachable statement - code above always terminates.".into(), None);
        }
        // The UnreachableStatementNode is a Stmt with the original's doc comment. The phpdoc tag value rules and
        // WrongVariableNameInVarTagRule skip virtual nodes.
        if self.unreachable.nodes.contains(&s.span()) {
            invalid_throws::node(self, s.span());
            var_tags::node(self, s.span());
        }
        if !self.unreachable.skipped.contains(&s.span()) {
            mago_syntax::walker::walk_statement_mut(self, s, context);
        }
    }
    fn walk_in_statement(&mut self, s: &'ast Statement<'arena>, _: &mut ()) {
        invalid_throws::statement(self, s);
        phpdoc_tags::statement(self, s);
        statements::statement_position(self, s);
    }
    fn walk_in_declare(&mut self, d: &'ast Declare<'arena>, _: &mut ()) {
        statements::declare(self, d);
    }
    fn walk_in_goto(&mut self, g: &'ast Goto<'arena>, _: &mut ()) {
        self.goto(g);
    }
    fn walk_in_label(&mut self, l: &'ast Label<'arena>, _: &mut ()) {
        self.label(l);
    }
    fn walk_in_constant_access(&mut self, a: &'ast ConstantAccess<'arena>, _: &mut ()) {
        deprecations::constant_fetch(self, a.span(), &a.name);
        names::constant_fetch(self, &a.name);
    }
    fn walk_in_magic_constant(&mut self, m: &'ast MagicConstant<'arena>, _: &mut ()) {
        statements::magic_constant(self, m);
    }
    fn walk_in_echo(&mut self, e: &'ast Echo<'arena>, _: &mut ()) {
        types::echo(self, e);
    }
    fn walk_in_print_construct(&mut self, p: &'ast PrintConstruct<'arena>, _: &mut ()) {
        types::print(self, p);
    }
    fn walk_in_interpolated_string(&mut self, s: &'ast InterpolatedString<'arena>, _: &mut ()) {
        types::interpolated(self, s.parts.as_slice());
    }
    fn walk_in_document_string(&mut self, s: &'ast DocumentString<'arena>, _: &mut ()) {
        types::interpolated(self, s.parts.as_slice());
    }
    fn walk_in_unary_prefix(&mut self, u: &'ast UnaryPrefix<'arena>, _: &mut ()) {
        dead_code::boolean_not(self, u);
        statements::cast(self, u);
        strict::cast(self, u);
        types::prefix(self, u);
        types::unary(self, u);
        operators::unary_prefix(self, u);
        internal::string_cast(self, u);
        if matches!(
            u.operator,
            UnaryPrefixOperator::PreIncrement(_) | UnaryPrefixOperator::PreDecrement(_)
        ) {
            assign::increment(self, u.operand);
            self.incremented(u.operand);
            self.invalidate_expression(u.operand);
        }
        if u.operator.is_increment_or_decrement() {
            types::property_increment(self, u.operand, u.operator.span(), u.span());
        }
    }
    fn walk_in_unary_postfix(&mut self, u: &'ast UnaryPostfix<'arena>, _: &mut ()) {
        types::postfix(self, u);
        operators::unary_postfix(self, u);
        assign::increment(self, u.operand);
        self.incremented(u.operand);
        self.invalidate_expression(u.operand);
        types::property_increment(self, u.operand, u.operator.span(), u.span());
    }
    fn walk_out_for(&mut self, _: &'ast For<'arena>, _: &mut ()) {
        self.syntax.parents.pop();
    }
    fn walk_in_foreach(&mut self, f: &'ast Foreach<'arena>, _: &mut ()) {
        self.syntax.parents.push(syntax::Parent::Loop);
        assign::foreach(self, f);
        types::foreach(self, f);
        let (key, value) = match &f.target {
            ForeachTarget::Value(t) => (None, t.value),
            ForeachTarget::KeyValue(t) => (Some(t.key), t.value),
        };
        for target in key.into_iter().chain([value]) {
            self.invalidate_expression(target);
            arrays::destructuring(self, target, true);
        }
        if matches!(types::flow::unparen(value), Expression::Array(_) | Expression::List(_)) {
            let iterated = self.phpstan_type(f.expression).map(|t| declared::with_classes(self, || t.iterable_value_type()));
            types::destructuring(self, value.span(), value, iterated, None);
        }
    }
    fn walk_out_foreach(&mut self, _: &'ast Foreach<'arena>, _: &mut ()) {
        self.syntax.parents.pop();
    }
    fn walk_while(&mut self, w: &'ast While<'arena>, c: &mut ()) {
        dead_code::while_loop(self, w);
        if self.strict.booleans_in_loop_conditions {
            operators::condition(self, w.span(), w.condition, "a while condition", "while.condNotBoolean");
        }
        self.syntax.parents.push(syntax::Parent::Loop);
        self.walk_expression(w.condition, c);
        // the body starts where the condition held
        let statements: Vec<&Statement<'arena>> = match &w.body {
            WhileBody::Statement(s) => vec![s],
            WhileBody::ColonDelimited(b) => b.statements.iter().collect(),
        };
        let guards = self.truthy(w.condition);
        self.walk_branch(guards, &statements);
        self.syntax.parents.pop();
    }
    fn walk_in_do_while(&mut self, d: &'ast DoWhile<'arena>, _: &mut ()) {
        dead_code::do_while(self, d);
        if self.strict.booleans_in_loop_conditions {
            operators::condition(self, d.span(), d.condition, "a do-while condition", "doWhile.condNotBoolean");
        }
        self.syntax.parents.push(syntax::Parent::Loop);
    }
    fn walk_out_do_while(&mut self, _: &'ast DoWhile<'arena>, _: &mut ()) {
        self.syntax.parents.pop();
    }
    fn walk_in_switch(&mut self, s: &'ast Switch<'arena>, _: &mut ()) {
        dead_code::switch_statement(self, s);
        strict::switch(self, s);
        self.syntax.parents.push(syntax::Parent::Loop);
    }
    fn walk_out_switch(&mut self, _: &'ast Switch<'arena>, _: &mut ()) {
        self.syntax.parents.pop();
    }
    fn walk_in_break(&mut self, b: &'ast Break<'arena>, _: &mut ()) {
        syntax::continue_break(self, b, b.level, "break");
    }
    fn walk_in_continue(&mut self, k: &'ast Continue<'arena>, _: &mut ()) {
        syntax::continue_break(self, k, k.level, "continue");
    }
    fn walk_in_global(&mut self, g: &'ast Global<'arena>, _: &mut ()) {
        syntax::global(self, g);
    }
    fn walk_in_static(&mut self, s: &'ast Static<'arena>, _: &mut ()) {
        syntax::r#static(self, s);
    }
    fn walk_in_return(&mut self, r: &'ast Return<'arena>, _: &mut ()) {
        syntax::r#return(self, r);
    }
    fn walk_in_unset(&mut self, u: &'ast Unset<'arena>, _: &mut ()) {
        assign::unset(self, u);
        types::property_unset(self, u);
        unset::check(self, u);
        for value in u.values.iter() {
            self.invalidate_expression(value);
        }
    }
    fn walk_in_array_append(&mut self, a: &'ast ArrayAppend<'arena>, _: &mut ()) {
        assign::array_append(self, a);
        types::dim_fetch(self, a.span(), a.array, None);
    }
    fn walk_in_shell_execute_string(&mut self, s: &'ast ShellExecuteString<'arena>, _: &mut ()) {
        statements::backtick(self, s);
    }
}
