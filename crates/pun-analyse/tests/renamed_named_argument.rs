//! PHPStan 2.3.1 MethodCallWithPossiblyRenamedNamedArgumentRule, including the actual emissions in
//! FunctionCallParametersCheck and MethodParameterComparisonHelper (the Collector classes are placeholders).
//! Oracle: from the repository cwd, strip agent-format environment variables and run
//! `php <fixture>/oracle.php <oracle>/phpstan.phar`. The generator maps cases.json's analysed/scanned files to
//! absolute paths/scanFiles and runs `analyse --debug --error-format=json --no-progress` at each case's level.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

const ID: &str = "argument.parameterRenamedInSubtype";

fn check(mode: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/renamed-named-argument");
    let cases: Value =
        serde_json::from_slice(&std::fs::read(root.join("cases.json")).unwrap()).unwrap();
    let case = &cases[mode];
    let files = |key: &str| {
        case[key]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| root.join(f.as_str().unwrap()))
            .collect()
    };
    let mut project = Project::new(files("analysed"), files("scanned"));
    project.level = case["level"].as_u64().unwrap() as u8;
    project.php_version = 80500;
    project.report_ignores_without_comments = case["explainIgnores"].as_bool().unwrap_or(false);
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .filter(|e| {
            e.identifier == ID
                || e.identifier.starts_with("ignore.")
                || (case["dumpTypes"].as_bool().unwrap_or(false)
                    && e.identifier == "phpstan.dumpType")
        })
        .map(|e| {
            (
                e.file.replace(root.to_str().unwrap(), "$ROOT"),
                e.line,
                e.identifier,
                e.message,
                e.tip,
                e.can_be_ignored,
            )
        })
        .collect();
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    let mut want: Vec<_> = oracle[mode]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["file"].as_str().unwrap().to_string(),
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
                e["ignorable"].as_bool().unwrap(),
            )
        })
        .collect();
    got.sort();
    want.sort();
    assert_eq!(got, want, "{mode}");
}

#[test]
fn renamed_parameters_are_joined_across_analysed_files() {
    check("all");
}

#[test]
fn scanned_files_do_not_contribute_collectors() {
    check("calls-only");
    check("overrides-only");
}

#[test]
fn parameter_checks_gate_collection() {
    check("level-zero");
    check("level-four");
}

#[test]
fn visibility_finality_prototypes_and_named_argument_controls() {
    check("controls");
}

#[test]
fn dynamic_union_intersection_and_nullable_receivers() {
    check("receivers-five");
}

#[test]
fn strict_union_and_nullable_receivers() {
    check("receivers-eight");
}

#[test]
fn traits_collect_in_the_analysed_using_class() {
    check("traits");
    check("traits-scanned");
    check("trait-users-scanned");
}

#[test]
fn ignores_apply_to_joined_errors_not_collector_payloads() {
    check("ignores");
}

#[test]
fn comment_requirements_are_preserved_after_the_join() {
    check("explained-ignores");
}

#[test]
fn reflection_and_unpacked_argument_contracts() {
    check("contracts");
}

#[test]
fn template_intersection_and_unsealed_array_contracts() {
    check("refinements");
}

#[test]
fn written_union_order_safe_calls_reference_mutation_and_narrowing() {
    check("source-contracts");
}
