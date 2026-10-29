//! Complete PHPStan 2.3.1 diagnostics for CallMethodsRule's per-name filtered scopes, including the
//! FunctionCallParametersCheck collector's unpacked argument dependencies and evaluation positions.
//! Reproduce with fixtures/filtered-named-arguments/capture.php and the pinned PHAR.

use std::path::Path;

use pun_analyse::{Error, Project, run};
use serde_json::Value;

#[test]
fn original_correlated_receiver_and_unpack_scopes_at_level_five() {
    check("calls", 5);
}

#[test]
fn original_collectors_remain_disabled_at_level_four() {
    check("calls", 4);
}

#[test]
fn derived_dependencies_and_argument_positions_at_level_five() {
    check("boundaries", 5);
}

#[test]
fn derived_dependencies_and_argument_positions_at_level_four() {
    check("boundaries", 4);
}

#[test]
fn nullsafe_chains_preserve_genuine_nullable_results() {
    for level in [4, 5, 8] {
        check("nullable-chain", level);
    }
}

fn check(file: &str, level: u8) {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/filtered-named-arguments");
    let mut project = Project::new(vec![root.join(format!("{file}.php"))], vec![]);
    project.level = level;
    project.php_version = 80500;
    let mut got = run(&project).unwrap();
    for error in &mut got {
        error.file = error.file.replace(root.to_str().unwrap(), "$ROOT");
    }
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/filtered-named-arguments/expected.json"
    ))
    .unwrap();
    let mode = format!("{file}-{level}");
    let mut want: Vec<_> = oracle[&mode]
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
        })
        .collect();
    got.sort();
    want.sort();
    assert!(
        got == want,
        "{mode}\nextra: {:#?}\nmissing: {:#?}",
        got.iter()
            .filter(|error| !want.contains(error))
            .collect::<Vec<_>>(),
        want.iter()
            .filter(|error| !got.contains(error))
            .collect::<Vec<_>>()
    );
}
