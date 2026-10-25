//! Receiver semantics from PHPStan 2.3.1: require-extends is not nominal inheritance.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

#[test]
fn required_ancestors_preserve_receiver_identity_and_explicit_intersections() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/required-receivers");
    let mut project = Project::new(vec![root.join("methods.php")], vec![]);
    project.level = 3;
    project.php_version = 80400;
    let selected = |id: &str| {
        id.starts_with("method.")
            || id.starts_with("staticMethod.")
            || id == "argument.type"
            || id == "arguments.count"
            || id == "phpstan.dumpType"
    };
    let got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .filter(|e| selected(&e.identifier))
        .map(|e| (e.line, e.identifier, e.message, e.can_be_ignored))
        .collect();
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    let want: Vec<_> = oracle["files"]
        .as_object()
        .unwrap()
        .values()
        .flat_map(|f| f["messages"].as_array().unwrap())
        .filter(|e| selected(e["identifier"].as_str().unwrap()))
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["ignorable"].as_bool().unwrap(),
            )
        })
        .collect();
    assert_eq!(got, want);
}
