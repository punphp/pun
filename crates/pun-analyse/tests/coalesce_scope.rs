//! Assignment certainty and optional coalesce writes through the public analyser, against PHPStan 2.3.1.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

#[test]
fn coalesce_assignment_scopes_match_phpstan() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/coalesce-scope");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    let mut project = Project::new(vec![root.join("scope.php")], Vec::new());
    project.level = 1;
    project.options.set(
        pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES,
        true,
    );
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message))
        .collect();
    let mut want: Vec<_> = expected["errors"]
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
    got.sort();
    want.sort();
    assert_eq!(got, want);
}

#[test]
fn false_isset_offset_probes_do_not_invent_nullable_entries() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/coalesce-scope");
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/coalesce-scope/offset-probes-phpstan.json"
    ))
    .unwrap();
    assert!(oracle["errors"].as_array().unwrap().is_empty());
    let expected = oracle["files"]
        .as_object()
        .unwrap()
        .values()
        .next()
        .unwrap();
    let want: Vec<_> = expected["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            serde_json::json!([
                e["line"],
                e["identifier"],
                e["message"],
                e.get("tip").unwrap_or(&Value::Null),
                e["ignorable"]
            ])
        })
        .collect();
    let mut project = Project::new(vec![root.join("offset-probes.php")], Vec::new());
    project.level = 4;
    project.php_version = 80500;
    let got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| serde_json::json!([e.line, e.identifier, e.message, e.tip, e.can_be_ignored]))
        .collect();
    assert_eq!(got, want);
}
