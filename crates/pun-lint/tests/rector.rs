//! Rector's own fixtures for the two ported rules, copied verbatim from rector-src 2.6.1 (`.oracle/rector`,
//! rules-tests/...) into tests/rector/<Rector class>/. A fixture is `input -----\n expected`, or only the input when
//! the rule must not change it (Rector's FixtureSplitter). Each one must pass as
//! `format(pun lint --fix) == format(expected)` (lint.md Build 1), with format being the laravel preset. The text edits
//! also reproduce Rector's bytes before format, which is checked too.

use std::path::{Path, PathBuf};

use pun_format::Pipeline;
use pun_lint::Linter;
use pun_lint::rules::{
    NoAndAssignment, NoConcatAssignNewItem, NoDoubleNegation, NoNegatedAnd,
    NoNegatedBooleanLiteral, NoNegatedGroup, NoSingleArgumentSprintf, NoSingleItemInArray,
    NoTautologyTernary, NoUselessArrayValues, PreferArrayAppend, PreferEmptyArrayCompare,
    PreferFuncNumArgs, PreferNullCoalesce,
};
use pun_lint::rules::{NoUnusedCatchVariable, PreferCompoundAssignment};

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
fn remove_unused_variable_in_catch() {
    // config/configured_rule.php: phpVersion(PhpVersion::PHP_80).
    run(
        &Linter::new(vec![Box::new(NoUnusedCatchVariable)], 80000),
        "RemoveUnusedVariableInCatchRector/Fixture",
    );
}

#[test]
fn remove_unused_variable_in_catch_skipped_below_php_80() {
    // config/configured_rule_php_version_74.php: phpVersion(PhpVersion::PHP_74).
    run(
        &Linter::new(vec![Box::new(NoUnusedCatchVariable)], 70400),
        "RemoveUnusedVariableInCatchRector/FixtureSkipped",
    );
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
fn replace_multiple_boolean_not() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    // Pint's laravel preset has no_short_bool_cast, which also turns `!!$value` into `(bool) $value`.
    let linter = Linter::new(vec![Box::new(NoDoubleNegation)], 80400);
    run_format_agrees(&linter, "ReplaceMultipleBooleanNotRector/Fixture");
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone).
    run_format_agrees(&linter, "ReplaceMultipleBooleanNotRector/Oracle");
    // Where Rector drops parentheses the meaning needs, or cannot run; the expected code is derived by hand.
    run_format_agrees(&linter, "ReplaceMultipleBooleanNotRector/Pun");
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

/// [`run`] for a rule whose fix the laravel preset's format makes too: the formatted input may already equal the
/// expected code, so only the rule's own bytes and its applied fixes tell that the rule did the work.
fn run_format_agrees(linter: &Linter, dir: &str) {
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
        assert_eq!(fixed.applied.is_empty(), input == expected, "{name}");
    }
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
        // The spike's sample rule, out of scope since the mapping audit (PLAN.md Track B removes it).
        if rule.id() == "lint/php-8.0/no-unused-catch-variable" {
            continue;
        }
        let key = format!("\n\"{}\" = {{", rule.id().strip_prefix("lint/").unwrap());
        assert!(lint.contains(&key), "{} is not in [rules.lint]", rule.id());
    }
}
