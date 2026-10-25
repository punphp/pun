//! PHPStan 2.3.1 source metadata must not replace inferred array values or borrow unrelated callable members.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

#[test]
fn declarations_copies_reassignments_and_asymmetric_refinements() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/array-metadata");
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    let got: Vec<_> = run(&Project::new(vec![root.join("types.php")], Vec::new()))
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message, e.tip, e.can_be_ignored))
        .collect();
    let mut want: Vec<_> = oracle["files"]["types.php"]["messages"]
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
