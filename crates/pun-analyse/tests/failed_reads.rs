//! Failed offset arms are ErrorType, not null; successful arms and real null values survive.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

#[test]
fn failed_reads_match_phpstan() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/failed-reads");
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    let mut project = Project::new(vec![root.join("values.php")], Vec::new());
    project.php_version = 80400;
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|error| {
            (
                error.line,
                error.identifier,
                error.message,
                error.can_be_ignored,
            )
        })
        .collect();
    let mut want: Vec<_> = oracle["files"]["values.php"]["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|error| {
            (
                error["line"].as_i64().unwrap(),
                error["identifier"].as_str().unwrap().to_string(),
                error["message"].as_str().unwrap().to_string(),
                error["ignorable"].as_bool().unwrap(),
            )
        })
        .collect();
    got.sort();
    want.sort();
    assert_eq!(got, want);
}
