//! Full PHPStan 2.3.1 records for extended method identity, not just method-existence checks.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(level: u8) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/extended-mixins");
    let mut project = Project::new(vec![root.join("methods.php")], Vec::new());
    project.level = level;
    project.php_version = 80500;
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| {
            (
                e.file.replace(root.to_str().unwrap(), "$ROOT"),
                e.line,
                e.identifier,
                e.message,
                e.tip,
                e.can_be_ignored,
            )
        })
        .collect();
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/extended-mixins/expected.json")).unwrap();
    let mut want: Vec<_> = oracle[level.to_string()]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["file"].as_str().unwrap().to_string(),
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
fn extended_method_identity_at_level_five() {
    check(5);
}

#[test]
fn extended_method_identity_at_level_eight() {
    check(8);
}
