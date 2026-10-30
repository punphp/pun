//! PHPStan 2.3.1 captures made before the correction: callable constraints belong to values.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(input: &Path, capture: &Path) {
    let oracle: Value = serde_json::from_slice(&std::fs::read(capture).unwrap()).unwrap();
    let mut project = Project::new(vec![input.to_owned()], Vec::new());
    project.level = 9;
    project.php_version = 80500;
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message, e.tip, e.can_be_ignored))
        .collect();
    let mut want: Vec<_> = oracle["files"]
        .as_object()
        .unwrap()
        .values()
        .flat_map(|file| file["messages"].as_array().unwrap())
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_owned(),
                e["message"].as_str().unwrap().to_owned(),
                e["tip"].as_str().map(str::to_owned),
                e["ignorable"].as_bool().unwrap(),
            )
        })
        .collect();
    got.sort();
    want.sort();
    assert!(!want.is_empty());
    assert_eq!(got, want, "{}", input.display());
}

#[test]
fn original_callable_after_empty_branches() {
    let tests = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    check(
        &tests.join("fixtures/rules/data/Functions/data/callable-or-closure-problem.php"),
        &tests.join("fixtures/callable-branches/original.json"),
    );
}

#[test]
fn narrowing_projections_copies_mutations_and_replacement_negatives() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/callable-branches");
    check(&root.join("matrix.php"), &root.join("matrix.json"));
}

#[test]
fn concrete_list_array_and_closure_projections() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/callable-branches");
    check(
        &root.join("projections.php"),
        &root.join("projections.json"),
    );
}

#[test]
fn count_extension_retains_phpdoc_certainty() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/callable-branches");
    check(
        &root.join("count-certainty.php"),
        &root.join("count-certainty.json"),
    );
}

#[test]
fn array_sizes_follow_sequential_and_conditional_appends() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/callable-branches");
    check(
        &root.join("count-joins.php"),
        &root.join("count-joins.json"),
    );
}
