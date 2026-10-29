//! Full PHPStan 2.3.1 records for extended method identity, not just method-existence checks.

use std::path::{Path, PathBuf};

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(file: &str, expected: &str, level: u8) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/extended-mixins");
    check_files(
        &root,
        vec![root.join(format!("{file}.php"))],
        expected,
        level,
    );
}

fn check_files(root: &Path, files: Vec<PathBuf>, expected: &str, level: u8) {
    let mut project = Project::new(files, Vec::new());
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
    let oracle: Value = serde_json::from_str(expected).unwrap();
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
    check(
        "methods",
        include_str!("fixtures/extended-mixins/expected.json"),
        5,
    );
}

#[test]
fn extended_method_identity_at_level_eight() {
    check(
        "methods",
        include_str!("fixtures/extended-mixins/expected.json"),
        8,
    );
}

#[test]
fn required_types_use_extended_method_reflection_at_level_four() {
    check(
        "requirements",
        include_str!("fixtures/extended-mixins/expected-requirements.json"),
        4,
    );
}

#[test]
fn required_types_use_extended_method_reflection_at_level_five() {
    check(
        "requirements",
        include_str!("fixtures/extended-mixins/expected-requirements.json"),
        5,
    );
}

#[test]
fn required_types_use_extended_method_reflection_at_level_eight() {
    check(
        "requirements",
        include_str!("fixtures/extended-mixins/expected-requirements.json"),
        8,
    );
}

#[test]
fn ordinary_generic_unions_do_not_activate_leaf_only_argument_semantics() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    for level in [5, 9] {
        check_files(
            &root,
            [
                "rules/data/Methods/data/bug-7593.php",
                "rules/data/Methods/data/bug-7600.php",
                "extended-mixins/generic-boundary.php",
            ]
            .map(|file| root.join(file))
            .to_vec(),
            include_str!("fixtures/extended-mixins/expected-generic-boundary.json"),
            level,
        );
    }
}
