//! FunctionCallParametersCheck::describeParameter(), compared with PHPStan 2.3.1.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(file: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/argument-names");
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    let mut project = Project::new(vec![root.join(file)], Vec::new());
    project.level = 7;
    project.php_version = 80500;
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message, e.tip, e.can_be_ignored))
        .collect();
    let mut want: Vec<_> = oracle[file]
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
    got.sort();
    want.sort();
    assert!(!want.is_empty());
    assert_eq!(got, want, "{file}");
}

#[test]
fn positional_and_named_type_errors_only_append_nonempty_parameter_names() {
    check("values.php");
}

#[test]
fn positional_named_and_pipe_reference_errors_only_append_nonempty_parameter_names() {
    check("references.php");
}
