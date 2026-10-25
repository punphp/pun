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
    project.check_maybe_undefined_variables = true;
    let relevant = |id: &str| matches!(id, "variable.undefined" | "phpstan.dumpType");
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .filter(|e| relevant(&e.identifier))
        .map(|e| (e.line, e.identifier, e.message))
        .collect();
    let mut want: Vec<_> = expected["errors"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| relevant(e["identifier"].as_str().unwrap()))
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
