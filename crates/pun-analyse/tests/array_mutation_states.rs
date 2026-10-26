//! Array mutation state is checked against PHPStan 2.3.1, through the public analyser.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(project: Project, expected: &Value) {
    let got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message, e.tip))
        .collect();
    let mut want: Vec<_> = expected
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
            )
        })
        .collect();
    want.sort_by(|a, b| (a.0, &a.2).cmp(&(b.0, &b.2)));
    assert_eq!(got, want, "{:?}", project.analysed);
}

fn upstream(level: u8, rule: &str, method: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let oracle: Value = serde_json::from_slice(
        &std::fs::read(root.join(format!("levels/{level}/{rule}.json"))).unwrap(),
    )
    .unwrap();
    let case = oracle["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["method"] == method)
        .unwrap();
    let paths = |key: &str| {
        case[key]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| root.join("rules/data").join(p.as_str().unwrap()))
            .collect()
    };
    let mut project = Project::new(paths("files"), paths("scan"));
    project.level = level;
    check(project, &case["errors"]);
}

#[test]
fn upstream_foreach_isset_unset_does_not_guarantee_nonempty() {
    upstream(
        4,
        "Comparison/NumberComparisonOperatorsConstantConditionRuleTest",
        "testBug11146",
    );
}

#[test]
fn upstream_general_property_write_does_not_create_empty_string_offset() {
    upstream(
        3,
        "Properties/TypesAssignedToPropertiesRuleTest",
        "testBug5298",
    );
}

fn oracle(file: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/array-mutation-states");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    let mut project = Project::new(vec![root.join(file)], Vec::new());
    project.level = 4;
    check(project, &expected["files"][file]["messages"]);
}

#[test]
fn literal_and_general_writes_keep_distinct_offset_contracts() {
    oracle("writes.php");
}

#[test]
fn literal_general_and_union_unsets_keep_unrelated_offsets() {
    oracle("unsets.php");
}

#[test]
fn array_exit_states_depend_on_the_actual_iteration_sequence() {
    oracle("loops.php");
}

#[test]
fn bounded_replay_preserves_generic_and_control_flow_fallbacks() {
    oracle("bounds.php");
}
