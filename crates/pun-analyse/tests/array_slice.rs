//! Expectations are PHPStan 2.3.1's actual JSON output (see fixtures/array-slice/phpstan.neon).

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(file: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/array-slice");
    let got: Vec<_> = run(&Project::new(vec![root.join(file)], Vec::new()))
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message))
        .collect();
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
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
    want.sort();
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
