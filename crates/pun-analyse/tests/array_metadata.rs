//! PHPStan 2.3.1 source metadata must not replace inferred array values or borrow unrelated callable members.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(file: &str, expected: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/array-metadata");
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join(expected)).unwrap()).unwrap();
    let got: Vec<_> = run(&Project::new(vec![root.join(file)], Vec::new()))
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message, e.tip, e.can_be_ignored))
        .collect();
    let mut want: Vec<_> = oracle["files"][file]["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
                e["ignorable"].as_bool().unwrap(),
            )
        })
        .collect();
    want.sort_by(|a, b| (a.0, &a.2).cmp(&(b.0, &b.2)));
    assert_eq!(got, want);
}

#[test]
fn declarations_copies_reassignments_and_asymmetric_refinements() {
    check("types.php", "expected-phpstan.json");
}

#[test]
fn mutated_shapes_keep_surviving_key_and_union_display_order() {
    check("mutated-order.php", "mutated-order.json");
}

#[test]
fn introduced_parent_keys_follow_write_back_order() {
    check("introduced-order.php", "introduced-order.json");
}
