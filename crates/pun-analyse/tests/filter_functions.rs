//! Public PHPStan captures: dynamic providers must reach assignments and downstream argument checks.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(file: &str, oracle: &str, level: u8) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/filter-functions");
    let capture: Value =
        serde_json::from_slice(&std::fs::read(root.join(oracle)).unwrap()).unwrap();
    let want: Vec<_> = capture["files"][file]["messages"]
        .as_array()
        .unwrap()
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
    assert!(!want.is_empty());
    let mut project = Project::new(vec![root.join(file)], Vec::new());
    project.php_version = 80400;
    project.level = level;
    let got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|error| (error.line, error.identifier, error.message, error.tip))
        .collect();
    let missing: Vec<_> = want.iter().filter(|error| !got.contains(error)).collect();
    let extra: Vec<_> = got.iter().filter(|error| !want.contains(error)).collect();
    assert!(
        missing.is_empty() && extra.is_empty(),
        "{file}\nmissing: {missing:#?}\nextra: {extra:#?}"
    );
    assert_eq!(got.len(), want.len());
}

#[test]
fn filter_flags_values_and_ranges() {
    check("cases.php", "expected-phpstan.json", 0);
}

#[test]
fn unknown_states_are_implicit_but_explicit_inputs_and_failures_are_not_suppressed() {
    check("flow.php", "expected-flow.json", 9);
}

#[test]
fn untouched_upstream_filter_regressions_at_level_nine() {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rules/data/Functions/data");
    for name in ["bug-6261.php", "bug-11863.php"] {
        let mut project = Project::new(vec![root.join(name)], Vec::new());
        project.level = 9;
        assert_eq!(run(&project).unwrap(), Vec::new(), "{name}");
    }
}
