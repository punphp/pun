//! Upstream fixtures for the lint 7 small rules, laid out as in tests/rector.rs: Fixture/ verbatim from rector-src
//! 2.8.4, Oracle/ with what Rector 2.8.4 (`.oracle/rector-bin`, the rule alone) wrote. Each must pass as
//! `pun lint --fix == expected` and `format(pun lint --fix) == format(expected)`.
//!
//! tests/lint7_report/<group>/<name>/*.php.inc is an input, `-----`, and what the upstream tool reported for it, one
//! problem a line: `line: message` as PHPStan 2.3.1 with phpstan-strict-rules 2.0.12 (level 0, only that strict rule
//! on, `--error-format=json`, filtered to the rule's identifier) reports them, sorted by line and message as PHPStan
//! sorts them.

use std::path::{Path, PathBuf};

use mago_syntax::cst::Node;
use pun_format::Pipeline;
use pun_lint::rules::*;
use pun_lint::{Linter, Rule};

fn fixtures(dir: &str) -> Vec<(PathBuf, String, String)> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/rector")
        .join(dir);
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    paths.sort();
    assert!(!paths.is_empty(), "no fixtures in {}", dir.display());
    paths
        .into_iter()
        .map(|path| {
            let content = std::fs::read_to_string(&path)
                .unwrap()
                .replace("\r\n", "\n");
            let (input, expected) = match content.split_once("-----\n") {
                Some((i, e)) => (i.to_string(), e.to_string()),
                None => (content.clone(), content),
            };
            (path, input, expected)
        })
        .collect()
}

fn format(pipeline: &Pipeline, code: &str) -> String {
    match pipeline
        .run(Path::new("fixture.php"), code.as_bytes())
        .unwrap()
    {
        Some(fixed) => String::from_utf8(fixed.code).unwrap(),
        None => code.to_string(),
    }
}

fn laravel() -> Pipeline {
    let rules = pun_config::rule_set(pun_config::preset_rules("laravel").unwrap()).unwrap();
    let pipeline = Pipeline::new(&rules, &pun_fixers::all).unwrap();
    assert!(pipeline.not_ported.is_empty(), "{:?}", pipeline.not_ported);
    pipeline
}

fn run(linter: &Linter, dir: &str) {
    run_with_sources(linter, dir, &[]);
}

/// [`run`] with the symbols of tests/rector/`sources`/*.php known, as Rector's tests autoload a fixture's Source/.
fn run_with_sources(linter: &Linter, dir: &str, sources: &[&str]) {
    let pipeline = laravel();
    let mut extra = Vec::new();
    for source in sources {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/rector")
            .join(source);
        extra.push((
            path.display().to_string(),
            std::fs::read_to_string(&path).unwrap(),
        ));
    }
    for (path, input, expected) in fixtures(dir) {
        let name = path.file_name().unwrap().to_string_lossy();
        let fixed = facts::with_sources(extra.clone(), || linter.fix(&path, &input)).unwrap();
        assert_eq!(fixed.source, expected, "{name}: lint --fix");
        assert_eq!(
            format(&pipeline, &fixed.source),
            format(&pipeline, &expected),
            "{name}: format(lint --fix)"
        );
        assert_eq!(input != expected, !fixed.applied.is_empty(), "{name}");
    }
}

fn only(rule: impl Rule + 'static) -> Linter {
    Linter::new(vec![Box::new(rule)], 80400)
}

#[test]
fn boolean_not_identical_to_not_identical() {
    let linter = only(NoNegatedComparison);
    run(&linter, "BooleanNotIdenticalToNotIdenticalRector/Fixture");
    run(&linter, "BooleanNotIdenticalToNotIdenticalRector/Oracle");
}

#[test]
fn ternary_empty_array_array_dim_fetch_to_coalesce() {
    let linter = only(PreferCoalesceOverEmptyTernary);
    run(
        &linter,
        "TernaryEmptyArrayArrayDimFetchToCoalesceRector/Fixture",
    );
    run(
        &linter,
        "TernaryEmptyArrayArrayDimFetchToCoalesceRector/Oracle",
    );
}

/// Checks the report cases of `rule` in tests/lint7_report/`dir`.
fn report(rule: impl Rule + 'static, dir: &str) {
    report_as(rule, dir, false);
}

/// [`report`], with `line:column: message` lines when `column` is set (Mago's `--reporting-format emacs`).
fn report_as(rule: impl Rule + 'static, dir: &str, column: bool) {
    let linter = only(rule);
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/lint7_report")
        .join(dir);
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    paths.sort();
    assert!(!paths.is_empty(), "no cases in {}", dir.display());
    for path in paths {
        let content = std::fs::read_to_string(&path).unwrap();
        let (input, expected) = content.split_once("-----\n").unwrap();
        let mut got: Vec<(usize, String)> = linter
            .check(&path, input)
            .unwrap()
            .into_iter()
            .filter(|p| p.fix.is_none())
            .map(|p| {
                let line_start = input[..p.span.start].rfind('\n').map_or(0, |i| i + 1);
                let line = input[..p.span.start].matches('\n').count() + 1;
                let at = if column {
                    format!(
                        "{line}:{}",
                        input[line_start..p.span.start].chars().count() + 1
                    )
                } else {
                    line.to_string()
                };
                (line, format!("{at}: {}", p.message))
            })
            .collect();
        got.sort();
        let got: String = got.iter().map(|(_, m)| format!("{m}\n")).collect();
        assert_eq!(got, expected, "{}", path.display());
    }
}

#[test]
fn overwrite_variables_with_for_loop_init() {
    report(NoForInitOverwrite, "control-flow/no-for-init-overwrite");
}

#[test]
fn disallowed_loose_comparison() {
    report(NoLooseEquality, "operators/no-loose-equality");
}

#[test]
fn no_debug_symbols() {
    // Mago's reports: the calls without a fix.
    report_as(NoDebugCalls, "security/no-debug-calls", true);
}

#[test]
fn remove_debug_expectations() {
    run(&only(NoDebugCalls), "RemoveDebugExpectationsRector/Oracle");
}

#[test]
fn sort_call_like_named_args() {
    let linter = only(RequireNamedArgumentOrder);
    let source = ["SortCallLikeNamedArgsRector/Source/MyAttribute.php"];
    run_with_sources(&linter, "SortCallLikeNamedArgsRector/Fixture", &source);
    run(&linter, "SortCallLikeNamedArgsRector/Oracle");
}

#[test]
fn sort_attribute_named_args() {
    let linter = only(RequireAttributeArgumentOrder);
    let source = ["SortAttributeNamedArgsRector/Source/MyAttribute.php"];
    run_with_sources(&linter, "SortAttributeNamedArgsRector/Fixture", &source);
    run(&linter, "SortAttributeNamedArgsRector/Oracle");
}

#[test]
fn overwrite_variables_with_foreach() {
    report(
        NoLoopVariableOverwrite,
        "control-flow/no-loop-variable-overwrite",
    );
}

/// A rule called on every node, not only the kinds of its `targets()`.
struct Everywhere(Box<dyn Rule>);

impl Rule for Everywhere {
    fn id(&self) -> &'static str {
        self.0.id()
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &pun_lint::Context<'_>,
        out: &mut Vec<pun_lint::Report>,
    ) {
        self.0.visit(node, ancestors, ctx, out);
    }
}

#[test]
fn remove_unused_private_class_constant() {
    let linter = only(NoUnusedPrivateConstant);
    let sources = [
        "RemoveUnusedPrivateClassConstantRector/Source/AbstractEnum.php",
        "RemoveUnusedPrivateClassConstantRector/Source/TraitUsingPrivateConstant.php",
    ];
    run_with_sources(
        &linter,
        "RemoveUnusedPrivateClassConstantRector/Fixture",
        &sources,
    );
    run(&linter, "RemoveUnusedPrivateClassConstantRector/Oracle");
}

#[test]
fn func_get_args_to_variadic_param() {
    let linter = only(PreferVariadicParam);
    run(&linter, "FuncGetArgsToVariadicParamRector/Fixture");
    run(&linter, "FuncGetArgsToVariadicParamRector/Oracle");
    // `VARIADIC_PARAM`: PHP 5.6.
    let path = Path::new("old.php");
    let code = "<?php function f() { $args = func_get_args(); }\n";
    let old = Linter::new(vec![Box::new(PreferVariadicParam)], 50500);
    assert_eq!(
        old.fix(path, code).unwrap().applied,
        [] as [pun_problem::Problem; 0]
    );
    let new = Linter::new(vec![Box::new(PreferVariadicParam)], 50600);
    assert_ne!(
        new.fix(path, code).unwrap().applied,
        [] as [pun_problem::Problem; 0]
    );
}

#[test]
fn call_user_func_array_to_variadic() {
    let linter = only(PreferSpread);
    let sources = ["CallUserFuncArrayToVariadicRector/Source/Redirector.php"];
    run_with_sources(
        &linter,
        "CallUserFuncArrayToVariadicRector/Fixture",
        &sources,
    );
    run(&linter, "CallUserFuncArrayToVariadicRector/Oracle");
}

#[test]
fn illegal_constructor_static_call() {
    // unknown_parents.php.inc: PHPStan scanned a file declaring `Vendor\Base` and `Vendor\Deep extends Base`, which
    // Pun does not see.
    report(
        NoStaticConstructorCall,
        "classes/no-static-constructor-call",
    );
}

#[test]
fn remove_null_tag_value_node() {
    let linter = only(NoNullTagValue);
    run(&linter, "RemoveNullTagValueNodeRector/Fixture");
    run(&linter, "RemoveNullTagValueNodeRector/Oracle");
}

#[test]
fn strict_function_calls() {
    report(
        RequireStrictFunctionCall,
        "functions/require-strict-function-call",
    );
}

#[test]
fn catch_exception_name_matching_type() {
    let linter = only(PreferCatchNameFromType);
    run(&linter, "CatchExceptionNameMatchingTypeRector/Fixture");
    run(&linter, "CatchExceptionNameMatchingTypeRector/Oracle");
}

#[test]
fn remove_parent_call_without_parent() {
    let linter = only(NoParentCallWithoutParent);
    let sources = [
        "RemoveParentCallWithoutParentRector/Source/SkipBrokenHierarchyParent.php",
        "RemoveParentCallWithoutParentRector/Source/SomeParentMethod.php",
        "RemoveParentCallWithoutParentRector/Source/SomeParentMethodMagicCall.php",
    ];
    run_with_sources(
        &linter,
        "RemoveParentCallWithoutParentRector/Fixture",
        &sources,
    );
    run(&linter, "RemoveParentCallWithoutParentRector/Oracle");
}

#[test]
fn mago_no_request_all() {
    // Mago 1.57.0 with `integrations = ["laravel"]`, `--reporting-format emacs`.
    report_as(NoRequestAll, "laravel/no-request-all", true);
}

#[test]
fn mago_prefer_casts_method() {
    report_as(PreferCastsMethod, "laravel/prefer-casts-method", true);
}

#[test]
fn mago_prefer_anonymous_migration() {
    report_as(
        PreferAnonymousMigration,
        "laravel/prefer-anonymous-migration",
        true,
    );
}

#[test]
fn simplify_useless_variable() {
    let linter = only(NoUselessVariable);
    run(&linter, "SimplifyUselessVariableRector/Fixture");
    run(&linter, "SimplifyUselessVariableRector/Oracle");
}

#[test]
fn remove_null_arg_on_null_default_param() {
    run_with_sources(
        &only(NoNullDefaultArgument),
        "RemoveNullArgOnNullDefaultParamRector/Fixture",
        &["RemoveNullArgOnNullDefaultParamRector/Source/SomeExternalClass.php"],
    );
    run(
        &only(NoNullDefaultArgument),
        "RemoveNullArgOnNullDefaultParamRector/Oracle",
    );
}

#[test]
fn remove_null_named_arg_on_null_default_param() {
    run_with_sources(
        &only(NoNullDefaultNamedArgument),
        "RemoveNullNamedArgOnNullDefaultParamRector/Fixture",
        &["RemoveNullNamedArgOnNullDefaultParamRector/Source/SomeExternalClass.php"],
    );
    run(
        &only(NoNullDefaultNamedArgument),
        "RemoveNullNamedArgOnNullDefaultParamRector/Oracle",
    );
}

#[test]
fn remove_unused_constructor_param() {
    let linter = only(NoUnusedConstructorParam);
    run_with_sources(
        &linter,
        "RemoveUnusedConstructorParamRector/Fixture",
        &["RemoveUnusedConstructorParamRector/Source/InterfaceWithConstruct.php"],
    );
    run(&linter, "RemoveUnusedConstructorParamRector/Oracle");
}

#[test]
fn use_class_keyword_for_class_name_resolution() {
    let linter = only(PreferClassKeyword);
    run_with_sources(
        &linter,
        "UseClassKeywordForClassNameResolutionRector/Fixture",
        &["UseClassKeywordForClassNameResolutionRector/Source/ConstantReferenceClass.php"],
    );
    run(
        &linter,
        "UseClassKeywordForClassNameResolutionRector/Oracle",
    );
    run(&linter, "UseClassKeywordForClassNameResolutionRector/Pun");
}

#[test]
fn remove_dead_catch() {
    let linter = only(NoDeadCatch);
    run_with_sources(
        &linter,
        "RemoveDeadCatchRector/Fixture",
        &["RemoveDeadCatchRector/Source/ShouldNotHappenException.php"],
    );
    run(&linter, "RemoveDeadCatchRector/Oracle");
}

#[test]
fn merge_date_time_property_type_declaration() {
    let linter = only(AddDatetimePropertyType);
    run(
        &linter,
        "MergeDateTimePropertyTypeDeclarationRector/Fixture",
    );
    run(&linter, "MergeDateTimePropertyTypeDeclarationRector/Oracle");
}

#[test]
fn make_inherited_method_visibility_same_as_parent() {
    let linter = only(RequireParentVisibility);
    let dir = "MakeInheritedMethodVisibilitySameAsParentRector";
    let sources: Vec<String> = [
        "Father",
        "GrandFather",
        "NoMethodAnnotationHere",
        "ParentWithProtectedMethod",
        "ParentWithPublicConstructor",
        "SomeInterface",
    ]
    .iter()
    .map(|s| format!("{dir}/Source/{s}.php"))
    .collect();
    let sources: Vec<&str> = sources.iter().map(String::as_str).collect();
    run_with_sources(&linter, &format!("{dir}/Fixture"), &sources);
    run(&linter, &format!("{dir}/Oracle"));
}

#[test]
fn remove_unused_promoted_property() {
    let linter = only(NoUnusedPromotedProperty);
    run_with_sources(
        &linter,
        "RemoveUnusedPromotedPropertyRector/Fixture",
        &["RemoveUnusedPromotedPropertyRector/Source/SomeInterfaceWithConstruct.php"],
    );
    run(&linter, "RemoveUnusedPromotedPropertyRector/Oracle");
    run(&linter, "RemoveUnusedPromotedPropertyRector/Pun");
}

#[test]
fn rules_ignore_nodes_outside_their_targets() {
    // `targets()` only spares the engine work: called on every node, each rule reports what it reports on its own.
    let cases: Vec<(fn() -> Box<dyn Rule>, &[&str])> = vec![
        (
            || Box::new(NoNegatedComparison),
            &["rector/BooleanNotIdenticalToNotIdenticalRector"],
        ),
        (
            || Box::new(PreferCoalesceOverEmptyTernary),
            &["rector/TernaryEmptyArrayArrayDimFetchToCoalesceRector"],
        ),
        (
            || Box::new(NoForInitOverwrite),
            &["lint7_report/control-flow/no-for-init-overwrite"],
        ),
        (
            || Box::new(NoLoopVariableOverwrite),
            &["lint7_report/control-flow/no-loop-variable-overwrite"],
        ),
        (
            || Box::new(NoLooseEquality),
            &["lint7_report/operators/no-loose-equality"],
        ),
        (
            || Box::new(NoDebugCalls),
            &[
                "lint7_report/security/no-debug-calls",
                "rector/RemoveDebugExpectationsRector",
            ],
        ),
        (
            || Box::new(RequireNamedArgumentOrder),
            &["rector/SortCallLikeNamedArgsRector"],
        ),
        (
            || Box::new(RequireAttributeArgumentOrder),
            &["rector/SortAttributeNamedArgsRector"],
        ),
        (
            || Box::new(NoUnusedPrivateConstant),
            &["rector/RemoveUnusedPrivateClassConstantRector"],
        ),
        (
            || Box::new(PreferVariadicParam),
            &["rector/FuncGetArgsToVariadicParamRector"],
        ),
        (
            || Box::new(PreferSpread),
            &["rector/CallUserFuncArrayToVariadicRector"],
        ),
        (
            || Box::new(NoStaticConstructorCall),
            &["lint7_report/classes/no-static-constructor-call"],
        ),
        (
            || Box::new(NoNullTagValue),
            &["rector/RemoveNullTagValueNodeRector"],
        ),
        (
            || Box::new(RequireStrictFunctionCall),
            &["lint7_report/functions/require-strict-function-call"],
        ),
        (
            || Box::new(PreferCatchNameFromType),
            &["rector/CatchExceptionNameMatchingTypeRector"],
        ),
        (
            || Box::new(NoParentCallWithoutParent),
            &["rector/RemoveParentCallWithoutParentRector"],
        ),
        (
            || Box::new(NoRequestAll),
            &["lint7_report/laravel/no-request-all"],
        ),
        (
            || Box::new(PreferCastsMethod),
            &["lint7_report/laravel/prefer-casts-method"],
        ),
        (
            || Box::new(PreferAnonymousMigration),
            &["lint7_report/laravel/prefer-anonymous-migration"],
        ),
        (
            || Box::new(NoUselessVariable),
            &["rector/SimplifyUselessVariableRector"],
        ),
        (
            || Box::new(NoNullDefaultArgument),
            &["rector/RemoveNullArgOnNullDefaultParamRector"],
        ),
        (
            || Box::new(NoNullDefaultNamedArgument),
            &["rector/RemoveNullNamedArgOnNullDefaultParamRector"],
        ),
        (
            || Box::new(NoUnusedConstructorParam),
            &["rector/RemoveUnusedConstructorParamRector"],
        ),
        (
            || Box::new(PreferClassKeyword),
            &["rector/UseClassKeywordForClassNameResolutionRector"],
        ),
        (|| Box::new(NoDeadCatch), &["rector/RemoveDeadCatchRector"]),
        (
            || Box::new(RequireParentVisibility),
            &["rector/MakeInheritedMethodVisibilitySameAsParentRector"],
        ),
        (
            || Box::new(NoUnusedPromotedProperty),
            &["rector/RemoveUnusedPromotedPropertyRector"],
        ),
        (
            || Box::new(AddDatetimePropertyType),
            &["rector/MergeDateTimePropertyTypeDeclarationRector"],
        ),
        (
            || Box::new(PreferResolveHelper),
            &["rector/AppToResolveRector"],
        ),
        (
            || Box::new(PreferDateFacade),
            &["rector/CarbonToDateFacadeRector"],
        ),
        (
            || Box::new(PreferRequestFacade),
            &["rector/RequestVariablesToRequestFacadeRector"],
        ),
        (
            || Box::new(PreferDefaultArgument),
            &["rector/ApplyDefaultInsteadOfNullCoalesceRector"],
        ),
        (
            || Box::new(PreferExplicitJsonCall),
            &["rector/JsonCallToExplicitJsonCallRector"],
        ),
        (
            || Box::new(PreferTravelTo),
            &["rector/CarbonSetTestNowToTravelToRector"],
        ),
        (
            || Box::new(PreferNullsafe::default()),
            &["rector/OptionalToNullsafeOperatorRector"],
        ),
        (
            || Box::new(NoEmptyMethod),
            &["rector/RemoveEmptyClassMethodRector/Fixture"],
        ),
        (
            || Box::new(NoUnusedPrivateMethod),
            &["rector/RemoveUnusedPrivateMethodRector/Fixture"],
        ),
        (
            || Box::new(NoUselessOverride),
            &["rector/RemoveParentDelegatingClassMethodRector/Fixture"],
        ),
        (
            || Box::new(NoRedundantDefaultArgument),
            &["rector/RemoveArgumentFromDefaultParentCallRector/Fixture"],
        ),
        (
            || Box::new(PreferFillableAttribute),
            &["rector/FillablePropertyToFillableAttributeRector"],
        ),
        (
            || Box::new(PreferCollectedByAttribute),
            &["rector/CollectedByPropertyToCollectedByAttributeRector"],
        ),
        (
            || Box::new(PreferTableAttribute),
            &["rector/TablePropertyToTableAttributeRector"],
        ),
        (
            || Box::new(PreferScopeAttribute),
            &["rector/ScopeNamedClassMethodToScopeAttributedClassMethodRector"],
        ),
    ];
    for (rule, dirs) in cases {
        let targeted = Linter::new(vec![rule()], 80400);
        let everywhere = Linter::new(vec![Box::new(Everywhere(rule()))], 80400);
        for dir in dirs {
            let root = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join(dir);
            let mut files = Vec::new();
            for sub in std::fs::read_dir(&root).unwrap() {
                let sub = sub.unwrap().path();
                if sub.is_dir() {
                    files.extend(std::fs::read_dir(sub).unwrap().map(|e| e.unwrap().path()));
                } else {
                    files.push(sub);
                }
            }
            assert!(!files.is_empty(), "{dir}");
            for path in files {
                let content = std::fs::read_to_string(&path).unwrap();
                let input = content.split("-----\n").next().unwrap();
                assert_eq!(
                    everywhere.check(&path, input).unwrap(),
                    targeted.check(&path, input).unwrap(),
                    "{}",
                    path.display()
                );
            }
        }
    }
}
