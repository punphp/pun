//! Public rule expectations from PHPStan 2.3.1, including its upstream union-key fixtures.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(file: &Path, errors: &Value) {
    let got: Vec<_> = run(&Project::new(vec![file.to_path_buf()], Vec::new()))
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message, e.tip, e.can_be_ignored))
        .collect();
    let mut want: Vec<_> = errors
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
                e["ignorable"].as_bool().unwrap_or(true),
            )
        })
        .collect();
    want.sort_by(|a, b| (a.0, &a.2).cmp(&(b.0, &b.2)));
    assert_eq!(got, want, "{}", file.display());
}

#[test]
fn upstream_duplicate_keys_and_array_regressions() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rules");
    let expected: Value = serde_json::from_slice(
        &std::fs::read(root.join("Arrays/DuplicateKeysInLiteralArraysRuleTest.json")).unwrap(),
    )
    .unwrap();
    for case in expected["cases"].as_array().unwrap() {
        let file = case["files"][0].as_str().unwrap();
        check(&root.join("data").join(file), &case["errors"]);
    }
}

#[test]
fn inferred_keys_and_pending_union_promotion() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/duplicate-array-keys");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    check(
        &root.join("keys.php"),
        &expected["files"]["keys.php"]["messages"],
    );
}
