//! Scope-aware callable results, captured from PHPStan 2.3.1 with the adjacent phpstan.neon.

use std::path::Path;

use pun_analyse::{Project, run, run_mago};
use serde_json::Value;

#[test]
fn lexical_method_access_and_unknown_callees_preserve_complete_results() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/callable-visibility");
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/callable-visibility/expected.json")).unwrap();
    assert_eq!(oracle["errors"], serde_json::json!([]));
    let messages = oracle["files"]["visibility.php"]["messages"]
        .as_array()
        .unwrap();
    let mut project = Project::new(vec![root.join("visibility.php")], vec![]);
    project.level = 2;
    project.php_version = 80500;
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|error| {
            (
                error.line,
                error.identifier,
                error.message,
                error.tip,
                error.can_be_ignored,
            )
        })
        .collect();
    let mut want: Vec<_> = messages
        .iter()
        .map(|error| {
            (
                error["line"].as_i64().unwrap(),
                error["identifier"].as_str().unwrap().to_owned(),
                error["message"].as_str().unwrap().to_owned(),
                error["tip"].as_str().map(str::to_owned),
                error["ignorable"].as_bool().unwrap(),
            )
        })
        .collect();
    got.sort();
    want.sort();
    assert_eq!(got, want);

    // Check the producer independently of the PHPStan-style diagnostic consumer.
    let mut got: Vec<_> = run_mago(&project, None)
        .unwrap()
        .dumped
        .into_iter()
        .map(|dump| (i64::from(dump.line), dump.type_id))
        .collect();
    let mut want: Vec<_> = messages
        .iter()
        .filter_map(|error| {
            error["message"]
                .as_str()
                .unwrap()
                .strip_prefix("Dumped type: ")
                .map(|ty| (error["line"].as_i64().unwrap(), ty.to_owned()))
        })
        .collect();
    got.sort();
    want.sort();
    assert_eq!(got, want);
}
