//! Shared reflected constructor/method contracts, captured from PHPStan 2.3.1 rather than Pun.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(level: u8) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/attribute-reflection");
    let mut project = Project::new(vec![root.join("calls.php")], Vec::new());
    project.level = level;
    project.php_version = 80500;
    let expected: Value = serde_json::from_slice(
        &std::fs::read(root.join(format!("expected-level{level}.json"))).unwrap(),
    )
    .unwrap();
    let relevant = |id: &str| {
        id.starts_with("argument.")
            || id.starts_with("attribute.")
            || matches!(
                id,
                "arguments.count" | "new.privateConstructor" | "method.private"
            )
    };
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .filter(|e| relevant(&e.identifier))
        .map(|e| (e.line, e.identifier, e.message, e.tip, e.can_be_ignored))
        .collect();
    let mut want: Vec<_> = expected["files"]
        .as_object()
        .unwrap()
        .values()
        .flat_map(|f| f["messages"].as_array().unwrap())
        .filter(|e| relevant(e["identifier"].as_str().unwrap()))
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
    got.sort();
    want.sort();
    assert_eq!(got, want, "level {level}");
}

#[test]
fn trait_constructor_reflection_retains_visibility_storage_and_context() {
    check(6);
}

#[test]
fn shared_constructor_checks_follow_argument_check_levels() {
    check(0);
    check(5);
}
