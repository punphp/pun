//! Expectations are PHPStan 2.3.1's actual JSON output (see fixtures/array-slice/phpstan.neon).

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(file: &str) {
    check_expected(file, "expected-phpstan.json");
}

fn check_expected(file: &str, expected: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/array-slice");
    let got: Vec<_> = run(&Project::new(vec![root.join(file)], Vec::new()))
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message))
        .collect();
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join(expected)).unwrap()).unwrap();
    let mut want: Vec<_> = expected["files"][file]["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    want.sort_by(|a, b| (a.0, &a.2).cmp(&(b.0, &b.2)));
    assert!(!want.is_empty());
    assert_eq!(got, want, "{file}");
}

#[test]
fn general_arrays_and_accessories() {
    check("general.php");
}

#[test]
fn literal_offsets_lengths_and_shapes() {
    check("shapes.php");
}

#[test]
fn invalid_inputs_and_insufficient_arity() {
    check("controls.php");
}

#[test]
fn unrefined_inner_calls_keep_their_array_return() {
    check("composition.php");
}

#[test]
fn mutated_shape_order_and_empty_slice_controls() {
    check_expected("mutated-order.php", "mutated-order.json");
}

#[test]
fn call_methods_bug_12880() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rules");
    let oracle: Value = serde_json::from_slice(
        &std::fs::read(root.join("Methods/CallMethodsRuleTest.json")).unwrap(),
    )
    .unwrap();
    let case = oracle["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["method"] == "testBug12880")
        .unwrap();
    let got: Vec<_> = run(&Project::new(
        vec![root.join("data/Methods/data/bug-12880.php")],
        Vec::new(),
    ))
    .unwrap()
    .into_iter()
    .map(|e| (e.line, e.identifier, e.message))
    .collect();
    let want: Vec<_> = case["errors"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    assert_eq!(got, want);
}
