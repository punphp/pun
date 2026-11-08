//! Complete PHPStan 2.3.1 records for tagged array-shape union reduction and offset filtering.
//! Reproduce with fixtures/shape-discrimination/capture.php and the pinned PHAR.

use std::path::Path;

use pun_analyse::{Error, Project, run};
use serde_json::Value;

#[test]
fn shape_discrimination_preserves_correlations_without_preventing_upstream_merges() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/shape-discrimination");
    let mut project = Project::new(vec![root.join("matrix.php")], vec![]);
    project.level = 5;
    project.php_version = 80500;
    let mut got = run(&project).unwrap();
    for error in &mut got {
        error.file = error.file.replace(root.to_str().unwrap(), "$ROOT");
    }
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/shape-discrimination/expected.json")).unwrap();
    let mut want: Vec<_> = oracle
        .as_array()
        .unwrap()
        .iter()
        .map(|error| Error {
            file: error["file"].as_str().unwrap().into(),
            line: error["line"].as_i64().unwrap(),
            identifier: error["identifier"].as_str().unwrap().into(),
            message: error["message"].as_str().unwrap().into(),
            tip: error["tip"].as_str().map(str::to_owned),
            can_be_ignored: error["ignorable"].as_bool().unwrap(),
            fixable: error["fixable"].as_bool().unwrap_or(false),
        })
        .collect();
    got.sort();
    want.sort();
    assert!(
        got == want,
        "extra: {:#?}\nmissing: {:#?}",
        got.iter()
            .filter(|error| !want.contains(error))
            .collect::<Vec<_>>(),
        want.iter()
            .filter(|error| !got.contains(error))
            .collect::<Vec<_>>()
    );
}
