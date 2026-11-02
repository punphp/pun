//! PHPStan's own rule tests of levels 1 to 9, run through `phpstan analyse --level N` by
//! `scripts/export-rule-fixtures.php --level=N` into tests/fixtures/levels/N: every case's errors with an
//! identifier Pun reports at that level must be exactly PHPStan's. Cases that still differ are listed in
//! tests/fixtures/levels/gaps.txt, and the list must be exact (a fixed case has to leave it). The partial
//! declaration-purity port uses diagnostic-specific void-pure-gaps.txt instead of case-wide exceptions.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use pun_analyse::{Project, run};
use serde_json::Value;

type Key = (String, i64, String, String, Option<String>);

/// These newly ported contracts must also match inside fixtures with older, unrelated inference gaps.
const EXACT: &[&str] = &[
    "variable.undefined",
    "variable.nameNotString",
    "argument.named",
    "argument.parameterRenamedInSubtype",
    "void.pure",
];

/// The identifiers of the rules ported above level 0, and of level-0 rules whose errors the level fixtures hold.
const PORTED: &[&str] = &[
    "phpstan.dumpType",
    "phpstan.dumpPhpDocType",
    "pipe.byRef",
    "classConstant.nativeType",
    "classConstant.missingNativeType",
    "classConstant.type",
    "method.nameCase",
    "staticMethod.nameCase",
    "catch.alreadyCaught",
    "catch.neverThrown",
    "throws.unusedType",
    "missingType.checkedException",
    "throws.notCovariant",
    "throws.notCovariantWithImplicitVoid",
    "throws.shouldBeVoid",
    "postDec.type",
    "preDec.type",
    "postInc.type",
    "preInc.type",
    "throws.void",
    "throws.notThrowable",
    "nullsafe.neverNull",
    "property.unusedType",
    "return.unusedType",
    "binaryOp.invalid",
    "assignOp.invalid",
    "offsetAssign.dimType",
    "offsetAssign.valueType",
    "assign.propertyType",
    "property.readOnlyAssignOutOfClass",
    "property.readOnlyAssignNotOnThis",
    "property.readOnlyAssignNotInConstructor",
    "property.readOnlyAssignByRef",
    "property.readOnlyByPhpDocAssignOutOfClass",
    "property.readOnlyByPhpDocAssignNotOnThis",
    "property.readOnlyByPhpDocAssignNotInConstructor",
    "property.readOnlyByPhpDocAssignByRef",
    "classConstant.value",
    "classConstant.phpDocType",
    "assign.propertyReadOnly",
    "property.defaultValue",
    "generator.outOfFunction",
    "generator.returnType",
    "generator.keyType",
    "generator.valueType",
    "generator.nonIterable",
    "generator.sendType",
    "generator.void",
    "throw.notThrowable",
    "clone.nonObject",
    "foreach.nonIterable",
    "arrayUnpacking.nonIterable",
    "arrayUnpacking.stringOffset",
    "offsetAccess.invalidOffset",
    "foreach.emptyArray",
    "echo.nonString",
    "print.nonString",
    "encapsedStringPart.nonString",
    "cast.int",
    "cast.bool",
    "cast.double",
    "cast.string",
    "unaryOp.invalid",
    "equal.invalid",
    "notEqual.invalid",
    "greater.invalid",
    "greaterOrEqual.invalid",
    "smaller.invalid",
    "smallerOrEqual.invalid",
    "spaceship.invalid",
    "parameter.defaultValue",
    "match.void",
    "return.type",
    "return.empty",
    "return.void",
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
    "finally.exitPoint",
    "function.inVoidCast",
    "function.resultDiscarded",
    "function.uselessReturnValue",
    "method.inVoidCast",
    "method.resultDiscarded",
    "staticMethod.inVoidCast",
    "staticMethod.resultDiscarded",
    "missingType.parameter",
    "missingType.return",
    "missingType.property",
    "missingType.iterableValue",
    "missingType.generics",
    "generics.existingClass",
    "generics.existingTypeAlias",
    "generics.traitBound",
    "generics.notSupportedBound",
    "generics.requiredTypeAfterOptional",
    "generics.templateDefaultOutOfBounds",
    "generics.notGeneric",
    "generics.lessTypes",
    "generics.moreTypes",
    "generics.notSubtype",
    "generics.notCompatible",
    "generics.noParent",
    "generics.wrongParent",
    "generics.trait",
    "method.shadowTemplate",
    "methodTag.shadowTemplate",
    "enum.generic",
    "method.parentMethodFinal",
    "method.parentMethodFinalByPhpDoc",
    "method.abstractOverridingNonAbstract",
    "method.override",
    "method.nonStatic",
    "method.static",
    "method.tentativeReturnType",
    "method.childReturnType",
    "method.childParameterType",
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
    "classConstant.internal",
    "classConstant.internalClass",
    "classConstant.internalInterface",
    "classConstant.internalTrait",
    "classConstant.internalEnum",
    "function.internal",
    "phpDoc.parseError",
    "phpDoc.phpstanTag",
    "requireExtends.duplicate",
    "requireExtends.nonObject",
    "requireExtends.interface",
    "requireExtends.trait",
    "requireExtends.enum",
    "requireExtends.finalClass",
    "requireExtends.onClass",
    "requireExtends.onEnum",
    "requireImplements.nonObject",
    "requireImplements.class",
    "requireImplements.trait",
    "requireImplements.enum",
    "requireImplements.onClass",
    "requireImplements.onEnum",
    "requireImplements.onInterface",
    "sealed.onEnum",
    "sealed.nonObject",
    "methodTag.trait",
    "methodTag.unresolvableType",
    "propertyTag.trait",
    "propertyTag.unresolvableType",
    "mixin.nonObject",
    "mixin.unresolvableType",
    "mixin.trait",
    "varTag.unresolvableType",
    "varTag.trait",
    "conditionalType.subjectNotFound",
    "conditionalType.alwaysTrue",
    "conditionalType.alwaysFalse",
    "assert.unknownExpr",
    "assert.unresolvableType",
    "assert.impossibleType",
    "assert.alreadyNarrowedType",
    "assert.trait",
    "parameter.unresolvableType",
    "parameter.phpDocType",
    "paramClosureThis.nonClosure",
    "return.unresolvableType",
    "return.phpDocType",
    "varTag.differentVariable",
    "varTag.misplaced",
    "varTag.multipleTags",
    "varTag.noVariable",
    "varTag.variableNotFound",
    "callable.shadowTemplate",
    "generics.callSiteVarianceConflict",
    "generics.callSiteVarianceRedundant",
    "generics.callSiteVarianceNotAllowed",
    "generics.variance",
    "function.variance",
    "method.variance",
    "parameter.trait",
    "return.trait",
    "property.unresolvableType",
    "property.phpDocType",
    "classConstant.unresolvableType",
    "paramImmediatelyInvokedCallable.nonCallable",
    "paramLaterInvokedCallable.nonCallable",
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
    "label.unused",
    // the bleeding-edge feature-toggle rules (off in the exported cases, which run without bleedingEdge)
    "assign.byRefForeachExpr",
    "constant.value",
    "constant.defineValue",
    "new.dateInterval",
    "trait.unused",
    "propertyGetHook.noRead",
    "propertySetHook.noAssign",
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
    "nullCoalesce.initializedProperty",
    "constant.notFound",
    "class.missingExtends",
    "class.missingImplements",
    // analyse types, levels 2-9 (an-types2)
    "argument.type",
    "argument.invalidPregQuote",
    "callable.nonCallable",
    "callable.inaccessibleMethod",
    "callable.void",
    "argument.byRef",
    "argument.unpackNonIterable",
    "argument.templateType",
    "argument.named",
    "method.nonObject",
    "method.notFound",
    "staticMethod.nonObject",
    "staticMethod.notFound",
    "arrayFilter.empty",
    "arrayFilter.same",
    "arrayFilter.alwaysEmpty",
    "arrayValues.empty",
    "arrayValues.list",
    "property.notFound",
    "property.nonObject",
    "property.private",
    "property.protected",
    "property.staticAccess",
    "staticProperty.nonStaticAccess",
    "staticProperty.nonObject",
    "staticProperty.notFound",
    "staticProperty.private",
    "staticProperty.protected",
    "method.private",
    "method.protected",
    "staticMethod.private",
    "staticMethod.protected",
    "method.staticCall",
    // analyse param-out and self-out (batch-paramout)
    "paramOut.type",
    "parameterByRef.type",
    "paramOut.unusedType",
    "paramOut.nestedUnusedType",
    "paramOut.tooWideBool",
    "parameterByRef.unusedType",
    "parameterByRef.nestedUnusedType",
    "parameterByRef.tooWideBool",
    "selfOut.type",
    "selfOut.static",
    "selfOut.unresolvableType",
];

/// The `class.notFound` messages of the level-2 Generics and PHPDoc tag rules (other rules report the identifier at every level).
const GENERICS_NOT_FOUND: &[&str] = &[
    "PHPDoc tag @template ",
    "PHPDoc tag @method template ",
    "PHPDoc tag @extends has invalid type",
    "PHPDoc tag @implements has invalid type",
    "PHPDoc tag @use has invalid type",
    "PHPDoc tag @phpstan-require-extends contains unknown class",
    "PHPDoc tag @phpstan-require-implements contains unknown class",
    "PHPDoc tag @phpstan-sealed contains unknown class",
    "PHPDoc tag @mixin contains unknown class",
    "PHPDoc tag @method for method ",
    "PHPDoc tag @property",
    "PHPDoc tag @var for variable ",
    "PHPDoc tag @var contains unknown class",
    "Throwing object of an unknown class ",
    "Cloning object of an unknown class ",
    "Iterating over an object of an unknown class ",
    "PHPDoc tag @phpstan-assert",
    "Invoking callable on an unknown class ",
    // FunctionDefinitionCheck's (native types from level 0, PHPDoc ones from level 2)
    "Parameter $",
    "Function ",
    "Method ",
    "Anonymous function has invalid return type",
    // ExistingClassesInPropertiesRule's and the property hooks' (PHPDoc types from level 2)
    "Property ",
    "Get hook for property ",
    "Set hook for property ",
];

/// The `parameter.notFound` messages of the ported rules (the PHPDoc tag rules that report it are not all ported).
const PARAMETER_NOT_FOUND: &[&str] = &[
    "PHPDoc tag @param-immediately-invoked-callable references unknown parameter",
    "PHPDoc tag @param-later-invoked-callable references unknown parameter",
    "Conditional return type references unknown parameter",
    "Assert references unknown parameter",
    "PHPDoc tag @param references unknown parameter",
    "PHPDoc tag @param-out references unknown parameter",
    "PHPDoc tag @param-closure-this references unknown parameter",
];

fn fixtures(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in std::fs::read_dir(dir).unwrap().flatten() {
        let p = e.path();
        if p.is_dir() {
            fixtures(&p, out);
        } else if p.extension().is_some_and(|x| x == "json") {
            out.push(p);
        }
    }
}

fn key(e: &Value) -> Key {
    (
        e["file"].as_str().unwrap().to_string(),
        e["line"].as_i64().unwrap(),
        e["identifier"].as_str().unwrap().to_string(),
        e["message"].as_str().unwrap().to_string(),
        e["tip"].as_str().map(str::to_string),
    )
}

#[test]
fn phpstan_level_fixtures() {
    let tests = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let root = tests.join("levels");
    let data = tests.join("rules/data");
    // PUN_LEVELS_FILTER=Properties/ selects fixture paths; only executed cases can retire a gap.
    let filter = std::env::var("PUN_LEVELS_FILTER").unwrap_or_default();
    // Half-open case bounds split large fixtures without rewriting their oracle records.
    let case_start = std::env::var("PUN_LEVELS_CASE_START").map_or(0, |v| v.parse().unwrap());
    let case_end = std::env::var("PUN_LEVELS_CASE_END").map_or(usize::MAX, |v| v.parse().unwrap());
    let filtered = !filter.is_empty() || case_start != 0 || case_end != usize::MAX;
    let gaps: BTreeSet<String> = std::fs::read_to_string(root.join("gaps.txt"))
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(str::to_string)
        .collect();
    // Declaration purity is partial. Only these exact oracle-positive locations may be missing;
    // older case-wide gaps must not swallow a regression in its supported declarations.
    let purity_gaps: BTreeSet<(String, String, i64)> =
        std::fs::read_to_string(root.join("void-pure-gaps.txt"))
            .unwrap()
            .lines()
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .map(|l| {
                let (case, location) = l.split_once(" | ").unwrap();
                let (file, line) = location.rsplit_once(':').unwrap();
                (case.to_string(), file.to_string(), line.parse().unwrap())
            })
            .collect();
    let mut missing_purity = BTreeSet::new();
    let prefix = format!("{}/", data.display());
    let mut failing = BTreeSet::new();
    let mut tested = BTreeSet::new();
    let mut exact_failures = BTreeSet::new();
    let mut details = Vec::new();
    let mut levels: Vec<_> = std::fs::read_dir(&root)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    levels.sort();
    for dir in levels {
        let level: u8 = dir.file_name().unwrap().to_str().unwrap().parse().unwrap();
        let mut files = Vec::new();
        fixtures(&dir, &mut files);
        files.sort();
        for f in files
            .into_iter()
            .filter(|f| f.to_string_lossy().contains(&filter))
        {
            let test: Value = serde_json::from_str(&std::fs::read_to_string(&f).unwrap()).unwrap();
            for case in test["cases"]
                .as_array()
                .unwrap()
                .iter()
                .skip(case_start)
                .take(case_end.saturating_sub(case_start))
            {
                let paths = |k: &str| -> Vec<PathBuf> {
                    case[k]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|p| data.join(p.as_str().unwrap()))
                        .collect()
                };
                let ported = |id: &str, message: &str| {
                    PORTED.contains(&id)
                        || EXACT.contains(&id)
                        || (id == "class.notFound"
                            && GENERICS_NOT_FOUND.iter().any(|p| message.starts_with(p)))
                        || (id == "parameter.notFound"
                            && PARAMETER_NOT_FOUND.iter().any(|p| message.starts_with(p)))
                        // (ValueAssignedToClassConstantRule's errors share the identifier and are not ported)
                        || (id == "classConstant.phpDocType" && message.contains(" native type "))
                };
                let mut want: BTreeSet<Key> = case["errors"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|e| {
                        e["identifier"]
                            .as_str()
                            .is_some_and(|id| ported(id, e["message"].as_str().unwrap()))
                    })
                    .map(key)
                    .collect();
                let mut project = Project::new(paths("files"), paths("scan"));
                project.level = level;
                project.check_class_case_sensitivity = level >= 2;
                // The pinned exports used PHP's register_argc_argv=0, independently of this host's INI.
                project.cli_arguments_variables_registered = false;
                project.check_maybe_undefined_variables = level >= 1;
                let got: BTreeSet<Key> = run(&project)
                    .unwrap()
                    .into_iter()
                    .filter(|e| ported(&e.identifier, &e.message))
                    .map(|e| {
                        let file = e.file.strip_prefix(&prefix).unwrap_or(&e.file).to_string();
                        (file, e.line, e.identifier, e.message, e.tip)
                    })
                    .collect();
                let name = format!(
                    "{level} {}::{}",
                    test["test"].as_str().unwrap(),
                    case["method"].as_str().unwrap()
                );
                tested.insert(name.clone());
                for e in want.difference(&got).filter(|e| e.2 == "void.pure") {
                    let gap = (name.clone(), e.0.clone(), e.1);
                    if purity_gaps.contains(&gap) {
                        missing_purity.insert(gap);
                    }
                }
                want.retain(|e| {
                    e.2 != "void.pure" || !purity_gaps.contains(&(name.clone(), e.0.clone(), e.1))
                });
                if want != got {
                    if want
                        .symmetric_difference(&got)
                        .any(|e| EXACT.contains(&e.2.as_str()))
                    {
                        exact_failures.insert(name.clone());
                    }
                    details.push(format!(
                        "{name}\n  missing {:#?}\n  extra   {:#?}",
                        want.difference(&got).collect::<Vec<_>>(),
                        got.difference(&want).collect::<Vec<_>>()
                    ));
                    failing.insert(name);
                }
            }
        }
    }
    let fixed: Vec<_> = gaps
        .iter()
        .filter(|name| !filtered || tested.contains(*name))
        .filter(|name| !failing.contains(*name))
        .collect();
    let new: Vec<_> = details
        .iter()
        .filter(|d| {
            let name = d.lines().next().unwrap();
            !gaps.contains(name) || exact_failures.contains(name)
        })
        .collect();
    let fixed_purity: Vec<_> = purity_gaps
        .iter()
        .filter(|(case, _, _)| !filtered || tested.contains(case))
        .filter(|gap| !missing_purity.contains(*gap))
        .collect();
    assert!(
        fixed.is_empty() && new.is_empty() && fixed_purity.is_empty(),
        "gaps.txt lists {} passing case(s): {fixed:?}\nvoid-pure-gaps.txt lists resolved/absent locations: {fixed_purity:?}\n{} new failure(s):\n{}",
        fixed.len(),
        new.len(),
        new.iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    );
    eprintln!("checked {} PHPStan level fixture cases", tested.len());
}
