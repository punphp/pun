//! Rector's own fixtures for the ported rules, copied verbatim from rector-src 2.6.1 (`.oracle/rector`,
//! rules-tests/...) into tests/rector/<Rector class>/Fixture. Oracle/ holds extra inputs with what Rector 2.6.1
//! (`.oracle/rector-bin`, the rule alone) wrote for them; Pun/ holds the cases where Pun keeps the meaning but not
//! Rector's bytes, its expected output derived by hand. A fixture is `input -----\n expected`, or only the input when
//! the rule must not change it (Rector's FixtureSplitter). Each one must pass as
//! `format(pun lint --fix) == format(expected)` (lint.md Build 1), with format being the laravel preset. The text edits
//! also reproduce Rector's bytes before format, which is checked too.

use std::path::{Path, PathBuf};

use pun_format::Pipeline;
use pun_lint::Linter;
use pun_lint::rules;
use pun_lint::rules::NoDuplicateCaseBody;
use pun_lint::rules::NoElseAfterReturn;
use pun_lint::rules::NoIdenticalBranches;
use pun_lint::rules::NoMirrorAssignment;
use pun_lint::rules::NoRedundantPromotedAssign;
use pun_lint::rules::NoUnreachableCode;
use pun_lint::rules::PreferCompoundAssignment;
use pun_lint::rules::PreferEarlyReturn;
use pun_lint::rules::PreferEarlyReturnOfVariable;
use pun_lint::rules::PreferEarlyReturnOverPreparedValue;
use pun_lint::rules::PreferInArray;
use pun_lint::rules::PreferNotInArray;
use pun_lint::rules::{
    NoAndAssignment, NoConcatAssignNewItem, NoNegatedAnd, NoNegatedBooleanLiteral, NoNegatedGroup,
    NoSingleArgumentSprintf, NoSingleItemInArray, NoTautologyTernary, NoUselessArrayValues,
    PreferArrayAppend, PreferEmptyArrayCompare, PreferFuncNumArgs, PreferNullCoalesce,
};

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
    let pipeline = laravel();
    for (path, input, expected) in fixtures(dir) {
        let name = path.file_name().unwrap().to_string_lossy();
        let fixed = linter.fix(&path, &input).unwrap();
        assert_eq!(fixed.source, expected, "{name}: lint --fix");
        assert_eq!(
            format(&pipeline, &fixed.source),
            format(&pipeline, &expected),
            "{name}: format(lint --fix)"
        );
        if input != expected {
            // The rule did the work, not the formatter.
            assert_ne!(
                format(&pipeline, &input),
                format(&pipeline, &expected),
                "{name}"
            );
            assert!(!fixed.applied.is_empty(), "{name}");
        } else {
            assert!(fixed.applied.is_empty(), "{name}");
        }
    }
}

#[test]
fn combined_assign() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    run(
        &Linter::new(vec![Box::new(PreferCompoundAssignment)], 80400),
        "CombinedAssignRector/Fixture",
    );
}

#[test]
fn and_assigns_to_separate_lines() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(NoAndAssignment)], 80400);
    run(&linter, "AndAssignsToSeparateLinesRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "AndAssignsToSeparateLinesRector/Oracle");
    // Where Rector drops parentheses the meaning needs, or cannot run; the expected code is derived by hand.
    run(&linter, "AndAssignsToSeparateLinesRector/Pun");
}

#[test]
fn array_key_exists_ternary_then_value_to_coalescing() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(PreferNullCoalesce)], 80400);
    run(
        &linter,
        "ArrayKeyExistsTernaryThenValueToCoalescingRector/Fixture",
    );
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(
        &linter,
        "ArrayKeyExistsTernaryThenValueToCoalescingRector/Oracle",
    );
}

#[test]
fn change_array_push_to_array_assign() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(PreferArrayAppend)], 80400);
    run(&linter, "ChangeArrayPushToArrayAssignRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "ChangeArrayPushToArrayAssignRector/Oracle");
    // Where Rector drops parentheses the meaning needs, or cannot run; the expected code is derived by hand.
    run(&linter, "ChangeArrayPushToArrayAssignRector/Pun");
}

#[test]
fn negated_ands_to_positive_ors() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(NoNegatedAnd)], 80400);
    run(&linter, "NegatedAndsToPositiveOrsRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "NegatedAndsToPositiveOrsRector/Oracle");
    // Where Rector drops parentheses the meaning needs, or cannot run; the expected code is derived by hand.
    run(&linter, "NegatedAndsToPositiveOrsRector/Pun");
}

#[test]
fn new_array_item_concat_assign_to_assign() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(NoConcatAssignNewItem)], 80400);
    run(&linter, "NewArrayItemConcatAssignToAssignRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "NewArrayItemConcatAssignToAssignRector/Oracle");
}

#[test]
fn replace_constant_boolean_not() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(NoNegatedBooleanLiteral)], 80400);
    run(&linter, "ReplaceConstantBooleanNotRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "ReplaceConstantBooleanNotRector/Oracle");
}

#[test]
fn simplify_de_morgan_binary() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(NoNegatedGroup)], 80400);
    run(&linter, "SimplifyDeMorganBinaryRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "SimplifyDeMorganBinaryRector/Oracle");
    // Where Rector drops parentheses the meaning needs, or cannot run; the expected code is derived by hand.
    run(&linter, "SimplifyDeMorganBinaryRector/Pun");
}

#[test]
fn simplify_empty_array_check() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(PreferEmptyArrayCompare)], 80400);
    run(&linter, "SimplifyEmptyArrayCheckRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "SimplifyEmptyArrayCheckRector/Oracle");
}

#[test]
fn simplify_func_get_args_count() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(PreferFuncNumArgs)], 80400);
    run(&linter, "SimplifyFuncGetArgsCountRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "SimplifyFuncGetArgsCountRector/Oracle");
}

#[test]
fn simplify_in_array_values() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(NoUselessArrayValues)], 80400);
    run(&linter, "SimplifyInArrayValuesRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "SimplifyInArrayValuesRector/Oracle");
    // Where Rector drops parentheses the meaning needs, or cannot run; the expected code is derived by hand.
    run(&linter, "SimplifyInArrayValuesRector/Pun");
}

#[test]
fn simplify_tautology_ternary() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(NoTautologyTernary)], 80400);
    run(&linter, "SimplifyTautologyTernaryRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "SimplifyTautologyTernaryRector/Oracle");
}

#[test]
fn single_in_array_to_compare() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(NoSingleItemInArray)], 80400);
    run(&linter, "SingleInArrayToCompareRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "SingleInArrayToCompareRector/Oracle");
    // Where Rector drops parentheses the meaning needs, or cannot run; the expected code is derived by hand.
    run(&linter, "SingleInArrayToCompareRector/Pun");
}

#[test]
fn unwrap_sprintf_one_argument() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(NoSingleArgumentSprintf)], 80400);
    run(&linter, "UnwrapSprintfOneArgumentRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "UnwrapSprintfOneArgumentRector/Oracle");
    // Where Rector drops parentheses the meaning needs, or cannot run; the expected code is derived by hand.
    run(&linter, "UnwrapSprintfOneArgumentRector/Pun");
}

#[test]
fn every_rule_id_is_a_lint_rule_of_the_mapping() {
    // crates/pun-config/rules/mapping.toml lists every lint rule as `"<group>/<name>" = { ... }` under [rules.lint].
    let mapping = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../pun-config/rules/mapping.toml"),
    )
    .unwrap();
    let lint = &mapping[mapping.find("[rules.lint]").unwrap()..];
    let lint = &lint[..lint[1..].find("\n[").unwrap()];
    for rule in pun_lint::rules::all() {
        let key = format!("\n\"{}\" = {{", rule.id().strip_prefix("lint/").unwrap());
        assert!(lint.contains(&key), "{} is not in [rules.lint]", rule.id());
    }
}

/// Like `run`, but for the fixtures named in `whitespace_only`, where Rector's printer writes different blanks than
/// Pun's text edits (each entry says why): there the raw output must equal the expected one once whitespace is
/// ignored, and format must make them equal.
fn run_with_whitespace_exceptions(linter: &Linter, dir: &str, whitespace_only: &[&str]) {
    let pipeline = laravel();
    let squeeze = |s: &str| s.split_whitespace().collect::<String>();
    for (path, input, expected) in fixtures(dir) {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let fixed = linter.fix(&path, &input).unwrap();
        if whitespace_only.contains(&name.as_str()) {
            assert_ne!(fixed.source, expected, "{name}: no longer an exception");
            assert_eq!(
                squeeze(&fixed.source),
                squeeze(&expected),
                "{name}: lint --fix"
            );
        } else {
            assert_eq!(fixed.source, expected, "{name}: lint --fix");
        }
        assert_eq!(
            format(&pipeline, &fixed.source),
            format(&pipeline, &expected),
            "{name}: format(lint --fix)"
        );
        assert_eq!(input != expected, !fixed.applied.is_empty(), "{name}");
    }
}

fn only(rule: impl pun_lint::Rule + 'static, php_version: u32) -> Linter {
    Linter::new(vec![Box::new(rule)], php_version)
}

#[test]
fn remove_and_true() {
    run(
        &only(rules::NoAndTrue, 80400),
        "RemoveAndTrueRector/Fixture",
    );
}

#[test]
fn remove_concat_autocast() {
    run(
        &only(rules::NoCastInConcat, 80400),
        "RemoveConcatAutocastRector/Fixture",
    );
}

#[test]
fn remove_dead_continue() {
    run(
        &only(rules::NoUselessContinue, 80400),
        "RemoveDeadContinueRector/Fixture",
    );
}

#[test]
fn remove_dead_if_foreach_for() {
    run(
        &only(rules::NoEmptyBlock, 80400),
        "RemoveDeadIfForeachForRector/Fixture",
    );
}

#[test]
fn remove_dead_try_catch() {
    run(
        &only(rules::NoUselessCatch, 80400),
        "RemoveDeadTryCatchRector/Fixture",
    );
}

#[test]
fn remove_double_self_assign() {
    run(
        &only(rules::NoDoubleSelfAssignment, 80400),
        "RemoveDoubleSelfAssignRector/Fixture",
    );
}

#[test]
fn remove_duplicated_array_key() {
    run(
        &only(rules::NoDuplicateArrayKey, 80400),
        "RemoveDuplicatedArrayKeyRector/Fixture",
    );
}

#[test]
fn remove_final_from_const() {
    // config/configured_rule.php: phpVersion(PhpVersionFeature::FINAL_CLASS_CONSTANTS).
    run(
        &only(rules::NoRedundantFinalConst, 80100),
        "RemoveFinalFromConstRector/Fixture",
    );
}

#[test]
fn remove_null_property_initialization() {
    run(
        &only(rules::NoNullPropertyInit, 80400),
        "RemoveNullPropertyInitializationRector/Fixture",
    );
}

#[test]
fn remove_null_property_initialization_skips_typed_properties() {
    // config/typed_properties.php: phpVersion(PhpVersionFeature::TYPED_PROPERTIES).
    run(
        &only(rules::NoNullPropertyInit, 70400),
        "RemoveNullPropertyInitializationRector/FixtureTypedProperties",
    );
}

#[test]
fn remove_unused_closure_variable_use() {
    // The laravel preset's `lambda_not_used_import` makes the same change, so `run`'s check that format alone does
    // not is left out.
    run_with_whitespace_exceptions(
        &only(rules::NoUnusedClosureVariable, 80400),
        "RemoveUnusedClosureVariableUseRector/Fixture",
        &[],
    );
}

#[test]
fn remove_unused_foreach_key() {
    run(
        &only(rules::NoUnusedForeachKey, 80400),
        "RemoveUnusedForeachKeyRector/Fixture",
    );
}

#[test]
fn remove_useless_return_expr_in_construct() {
    run(
        &only(rules::NoReturnInConstructor, 80400),
        "RemoveUselessReturnExprInConstructRector/Fixture",
    );
}

#[test]
fn remove_unreachable_statement() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(NoUnreachableCode)], 80400);
    run(&linter, "RemoveUnreachableStatementRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "RemoveUnreachableStatementRector/Oracle");
    // Where Rector reprints the whole file; the expected code is derived by hand.
    run(&linter, "RemoveUnreachableStatementRector/Pun");
}

#[test]
fn simplify_mirror_assign() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(NoMirrorAssignment)], 80400);
    run(&linter, "SimplifyMirrorAssignRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "SimplifyMirrorAssignRector/Oracle");
}

#[test]
fn remove_useless_assign_from_property_promotion() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(NoRedundantPromotedAssign)], 80400);
    run(
        &linter,
        "RemoveUselessAssignFromPropertyPromotionRector/Fixture",
    );
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(
        &linter,
        "RemoveUselessAssignFromPropertyPromotionRector/Oracle",
    );
}

#[test]
fn remove_duplicated_case_in_switch() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(NoDuplicateCaseBody)], 80400);
    run(&linter, "RemoveDuplicatedCaseInSwitchRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "RemoveDuplicatedCaseInSwitchRector/Oracle");
}

#[test]
fn simplify_if_else_with_same_content() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(NoIdenticalBranches)], 80400);
    run(&linter, "SimplifyIfElseWithSameContentRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone). Rector reprints a
    // single-statement body its own way, so only the formatted code is compared.
    run_formatted(&linter, "SimplifyIfElseWithSameContentRector/OracleFormat");
}

#[test]
fn remove_always_else() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(NoElseAfterReturn)], 80400);
    run(&linter, "RemoveAlwaysElseRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "RemoveAlwaysElseRector/Oracle");
    // Rector reprints a colon `if` or a single-statement body its own way, so only the formatted code is compared.
    run_formatted(&linter, "RemoveAlwaysElseRector/OracleFormat");
}

/// [`run`] for cases where Rector reprints more than Pun changes (a colon `if` with braces, the parent of a
/// single-statement body): only `format(lint --fix) == format(expected)` holds, the bytes before format differ.
fn run_formatted(linter: &Linter, dir: &str) {
    let pipeline = laravel();
    for (path, input, expected) in fixtures(dir) {
        let name = path.file_name().unwrap().to_string_lossy();
        let fixed = linter.fix(&path, &input).unwrap();
        assert_eq!(
            format(&pipeline, &fixed.source),
            format(&pipeline, &expected),
            "{name}: format(lint --fix)"
        );
        assert_eq!(fixed.applied.is_empty(), input == expected, "{name}");
    }
}

#[test]
fn change_if_else_value_assign_to_early_return() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(PreferEarlyReturn)], 80400);
    run(
        &linter,
        "ChangeIfElseValueAssignToEarlyReturnRector/Fixture",
    );
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "ChangeIfElseValueAssignToEarlyReturnRector/Oracle");
}

#[test]
fn prepared_value_to_early_return() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(PreferEarlyReturnOverPreparedValue)], 80400);
    run(&linter, "PreparedValueToEarlyReturnRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "PreparedValueToEarlyReturnRector/Oracle");
}

#[test]
fn return_early_if_variable() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(PreferEarlyReturnOfVariable)], 80400);
    run(&linter, "ReturnEarlyIfVariableRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "ReturnEarlyIfVariableRector/Oracle");
}

#[test]
fn repeated_or_equal_to_in_array() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(PreferInArray)], 80400);
    run(&linter, "RepeatedOrEqualToInArrayRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "RepeatedOrEqualToInArrayRector/Oracle");
}

#[test]
fn repeated_and_not_equal_to_not_in_array() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = Linter::new(vec![Box::new(PreferNotInArray)], 80400);
    run(&linter, "RepeatedAndNotEqualToNotInArrayRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "RepeatedAndNotEqualToNotInArrayRector/Oracle");
}

#[test]
fn simplify_if_not_null_return() {
    let linter = only(rules::NoRedundantNullCheck, 80400);
    run(&linter, "SimplifyIfNotNullReturnRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "SimplifyIfNotNullReturnRector/Oracle");
}

#[test]
fn simplify_conditions() {
    let linter = only(rules::PreferSimpleCondition, 80400);
    run(&linter, "SimplifyConditionsRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "SimplifyConditionsRector/Oracle");
    // Where Rector cannot run, stops with an error or escapes bytes; the expected code is derived by hand.
    run(&linter, "SimplifyConditionsRector/Pun");
}

#[test]
fn simplify_array_search() {
    let linter = only(rules::PreferInArrayOverSearch, 80400);
    run(&linter, "SimplifyArraySearchRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "SimplifyArraySearchRector/Oracle");
}

#[test]
fn ternary_condition_variable_assignment() {
    let linter = only(rules::NoAssignInTernaryBranch, 80400);
    run(&linter, "TernaryConditionVariableAssignmentRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "TernaryConditionVariableAssignmentRector/Oracle");
    // Where Rector drops parentheses the meaning needs, or cannot run; the expected code is derived by hand.
    run(&linter, "TernaryConditionVariableAssignmentRector/Pun");
}

#[test]
fn singular_switch_to_if() {
    let linter = only(rules::NoSingleCaseSwitch, 80400);
    run(&linter, "SingularSwitchToIfRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "SingularSwitchToIfRector/Oracle");
    // Where Rector drops parentheses the meaning needs, or cannot run; the expected code is derived by hand.
    run(&linter, "SingularSwitchToIfRector/Pun");
}

#[test]
fn ternary_implode_to_implode() {
    let linter = only(rules::NoImplodeTernary, 80400);
    run(&linter, "TernaryImplodeToImplodeRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "TernaryImplodeToImplodeRector/Oracle");
}

#[test]
fn new_static_to_new_self() {
    // The laravel preset's `self_static_accessor` makes the same change, so `run`'s check that format alone does not
    // is left out.
    let linter = only(rules::PreferNewSelf, 80400);
    run_with_whitespace_exceptions(&linter, "NewStaticToNewSelfRector/Fixture", &[]);
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run_with_whitespace_exceptions(&linter, "NewStaticToNewSelfRector/Oracle", &[]);
}

#[test]
fn array_merge_of_non_arrays_to_simple_array() {
    let linter = only(rules::PreferArrayLiteral, 80400);
    run(&linter, "ArrayMergeOfNonArraysToSimpleArrayRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "ArrayMergeOfNonArraysToSimpleArrayRector/Oracle");
    // Where Rector cannot run, stops with an error or escapes bytes; the expected code is derived by hand.
    run(&linter, "ArrayMergeOfNonArraysToSimpleArrayRector/Pun");
}

#[test]
fn remove_readonly_property_visibility_on_readonly_class() {
    let linter = only(rules::NoRedundantReadonlyProperty, 80400);
    run(
        &linter,
        "RemoveReadonlyPropertyVisibilityOnReadonlyClassRector/Fixture",
    );
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(
        &linter,
        "RemoveReadonlyPropertyVisibilityOnReadonlyClassRector/Oracle",
    );
}

#[test]
fn absolutize_require_and_include_path() {
    let linter = only(rules::PreferAbsoluteInclude, 80400);
    run(&linter, "AbsolutizeRequireAndIncludePathRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run(&linter, "AbsolutizeRequireAndIncludePathRector/Oracle");
    // Where Rector cannot run, stops with an error or escapes bytes; the expected code is derived by hand.
    run(&linter, "AbsolutizeRequireAndIncludePathRector/Pun");
}
