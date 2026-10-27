//! PHPStan distinguishes a never-valued expression from evaluation that cannot continue.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

#[test]
fn never_values_and_call_termination_match_phpstan() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/never-calls");
    let capture: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    let messages = capture["files"]
        .as_object()
        .unwrap()
        .values()
        .next()
        .unwrap()["messages"]
        .as_array()
        .unwrap();
    let want: Vec<_> = messages
        .iter()
        .map(|error| {
            (
                error["line"].as_i64().unwrap(),
                error["identifier"].as_str().unwrap().to_owned(),
                error["message"].as_str().unwrap().to_owned(),
                error["tip"].as_str().map(str::to_owned),
            )
        })
        .collect();
    let mut project = Project::new(vec![root.join("cases.php")], Vec::new());
    project.php_version = 80400;
    project.level = 5;
    let got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|error| (error.line, error.identifier, error.message, error.tip))
        .collect();
    let missing: Vec<_> = want.iter().filter(|error| !got.contains(error)).collect();
    let extra: Vec<_> = got.iter().filter(|error| !want.contains(error)).collect();
    assert!(
        missing.is_empty() && extra.is_empty(),
        "missing: {missing:#?}\nextra: {extra:#?}"
    );
    assert_eq!(got.len(), want.len());
}
