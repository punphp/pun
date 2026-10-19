//! Rector's own fixtures for lint-a's rules (dead-code, complexity, control-flow, operators, strings, arrays,
//! statements, comments, tags and refactor groups), laid out as in tests/rector.rs: Fixture/ verbatim from rector-src
//! 2.6.1, Oracle/ with what Rector 2.6.1 (`.oracle/rector-bin`, the rule alone) wrote, Pun/ derived by hand where Pun
//! keeps the meaning but not Rector's bytes. Each must pass as `format(pun lint --fix) == format(expected)`.

use std::path::{Path, PathBuf};

use pun_format::Pipeline;
use pun_lint::Linter;
use pun_lint::rules;

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
    run_formatter_too(linter, dir, &[]);
}

/// Like `run`, but the fixtures in `formatter_too` are ones the formatter alone also fixes (Pint drops a bare
/// `null;`), so only the rule's own output is checked for them.
fn run_formatter_too(linter: &Linter, dir: &str, formatter_too: &[&str]) {
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
        if input != expected && !formatter_too.contains(&name.as_ref()) {
            // The rule did the work, not the formatter.
            assert_ne!(
                format(&pipeline, &input),
                format(&pipeline, &expected),
                "{name}"
            );
            assert!(!fixed.applied.is_empty(), "{name}");
        } else if input == expected {
            assert!(fixed.applied.is_empty(), "{name}");
        }
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
fn logical_to_boolean() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    run(
        &only(rules::PreferBooleanAndOr, 80400),
        "LogicalToBooleanRector/Fixture",
    );
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone, PHP 8.4).
    run(
        &only(rules::PreferBooleanAndOr, 80400),
        "LogicalToBooleanRector/Oracle",
    );
}

#[test]
fn unwrap_future_compatible_if_php_version() {
    // Rector's tests run with PhpVersionProvider::provide() at 100000 (PHP 10.0).
    run(
        &only(rules::NoFuturePhpVersionCheck, 100000),
        "UnwrapFutureCompatibleIfPhpVersionRector/Fixture",
    );
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone, PHP 8.4).
    run(
        &only(rules::NoFuturePhpVersionCheck, 80400),
        "UnwrapFutureCompatibleIfPhpVersionRector/Oracle",
    );
}

#[test]
fn remove_php_version_id_check() {
    // config/configured_rule.php: PhpVersion::PHP_80.
    run(
        &only(rules::NoDeadPhpVersionCheck, 80000),
        "RemovePhpVersionIdCheckRector/Fixture",
    );
    // Pun's own cases where Pun keeps the meaning Rector loses (the module docs say how); expected derived by hand.
    run(
        &only(rules::NoDeadPhpVersionCheck, 80400),
        "RemovePhpVersionIdCheckRector/Pun",
    );
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone, PHP 8.4).
    run(
        &only(rules::NoDeadPhpVersionCheck, 80400),
        "RemovePhpVersionIdCheckRector/Oracle",
    );
}

#[test]
fn remove_unused_public_method_parameter() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    run(
        &only(rules::NoUnusedPublicMethodParam, 80400),
        "RemoveUnusedPublicMethodParameterRector/Fixture",
    );
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone, PHP 8.4).
    run(
        &only(rules::NoUnusedPublicMethodParam, 80400),
        "RemoveUnusedPublicMethodParameterRector/Oracle",
    );
}

#[test]
fn remove_unused_private_method_parameter() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    run(
        &only(rules::NoUnusedPrivateMethodParam, 80400),
        "RemoveUnusedPrivateMethodParameterRector/Fixture",
    );
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone, PHP 8.4).
    run(
        &only(rules::NoUnusedPrivateMethodParam, 80400),
        "RemoveUnusedPrivateMethodParameterRector/Oracle",
    );
}

#[test]
fn remove_unused_private_property() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    // on_arg's expected half lacks the input's final newline, which Rector's test runner trims away.
    run_with_whitespace_exceptions(
        &only(rules::NoUnusedPrivateProperty, 80400),
        "RemoveUnusedPrivatePropertyRector/Fixture",
        &["on_arg.php.inc"],
    );
    // Pun's own cases where Pun keeps the meaning Rector loses (the module docs say how); expected derived by hand.
    run(
        &only(rules::NoUnusedPrivateProperty, 80400),
        "RemoveUnusedPrivatePropertyRector/Pun",
    );
}

#[test]
fn remove_default_value_from_assigned_property() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    run(
        &only(rules::NoRedundantPropertyDefault, 80400),
        "RemoveDefaultValueFromAssignedPropertyRector/Fixture",
    );
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone, PHP 8.4).
    run(
        &only(rules::NoRedundantPropertyDefault, 80400),
        "RemoveDefaultValueFromAssignedPropertyRector/Oracle",
    );
}

#[test]
fn inline_constructor_default_to_property() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    run(
        &only(rules::PreferPropertyDefault, 80400),
        "InlineConstructorDefaultToPropertyRector/Fixture",
    );
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone, PHP 8.4).
    run(
        &only(rules::PreferPropertyDefault, 80400),
        "InlineConstructorDefaultToPropertyRector/Oracle",
    );
}

#[test]
fn foreach_to_in_array() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    run(
        &only(rules::PreferInArrayOverForeach, 80400),
        "ForeachToInArrayRector/Fixture",
    );
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone, PHP 8.4).
    run(
        &only(rules::PreferInArrayOverForeach, 80400),
        "ForeachToInArrayRector/Oracle",
    );
}

#[test]
fn consecutive_null_compare_returns_to_null_coalesce_queue() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    run(
        &only(rules::PreferCoalesceChain, 80400),
        "ConsecutiveNullCompareReturnsToNullCoalesceQueueRector/Fixture",
    );
    // Pun's own cases where Pun keeps the meaning Rector loses (the module docs say how); expected derived by hand.
    run(
        &only(rules::PreferCoalesceChain, 80400),
        "ConsecutiveNullCompareReturnsToNullCoalesceQueueRector/Pun",
    );
}

#[test]
fn simplify_foreach_to_coalescing() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    run(
        &only(rules::PreferCoalesceOverForeach, 80400),
        "SimplifyForeachToCoalescingRector/Fixture",
    );
    // Pun's own cases; the expected code is the oracle's (.oracle/rector-bin, this rule alone, PHP 8.4).
    run(
        &only(rules::PreferCoalesceOverForeach, 80400),
        "SimplifyForeachToCoalescingRector/Oracle",
    );
}

#[test]
fn remove_dead_condition_above_return() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = only(rules::NoUselessCondition, 80400);
    run(&linter, "RemoveDeadConditionAboveReturnRector/Fixture");
    run(&linter, "RemoveDeadConditionAboveReturnRector/Oracle");
}

#[test]
fn remove_condition_exact_return() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = only(rules::NoConditionExactReturn, 80400);
    run(&linter, "RemoveConditionExactReturnRector/Fixture");
    run(&linter, "RemoveConditionExactReturnRector/Oracle");
}

#[test]
fn remove_double_assign() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = only(rules::NoOverwrittenAssignment, 80400);
    run(&linter, "RemoveDoubleAssignRector/Fixture");
    run(&linter, "RemoveDoubleAssignRector/Oracle");
}

#[test]
fn remove_next_same_value_condition() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    let linter = only(rules::NoRedundantCondition, 80400);
    run(&linter, "RemoveNextSameValueConditionRector/Fixture");
    run(&linter, "RemoveNextSameValueConditionRector/Oracle");
}

#[test]
fn remove_dead_loop() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    run(
        &only(rules::NoEmptyLoop, 80400),
        "RemoveDeadLoopRector/Fixture",
    );
}

#[test]
fn remove_dead_stmt() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version. Pun/ keeps the kept parts in source
    // order where Rector writes the first part last.
    let linter = only(rules::NoUnusedExpression, 80400);
    run_formatter_too(
        &linter,
        "RemoveDeadStmtRector/Fixture",
        &["constant_fetch.php.inc", "scalar.php.inc"],
    );
    run(&linter, "RemoveDeadStmtRector/FixtureRemovedComments");
    run(&linter, "RemoveDeadStmtRector/Pun");
}

#[test]
fn remove_sole_value_sprintf() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    run(
        &only(rules::NoSoleValueSprintf, 80400),
        "RemoveSoleValueSprintfRector/Fixture",
    );
}

#[test]
fn compact_to_variables() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    run(
        &only(rules::NoCompact, 80400),
        "CompactToVariablesRector/Fixture",
    );
}

#[test]
fn is_a_with_string_with_third_argument() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    run(
        &only(rules::RequireIsAAllowString, 80400),
        "IsAWithStringWithThirdArgumentRector/Fixture",
    );
}
