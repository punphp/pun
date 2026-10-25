//! OverridingConstantRule: PHPStan 2.3.1's diagnostics, including non-ignorable native errors.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn overriding(identifier: &str) -> bool {
    matches!(
        identifier,
        "classConstant.final"
            | "classConstant.visibility"
            | "classConstant.nativeType"
            | "classConstant.missingNativeType"
            | "classConstant.type"
    )
}

fn check(level: u8) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/constant-overrides");
    let expected: Value = serde_json::from_slice(
        &std::fs::read(root.join(format!("expected-level{level}.json"))).unwrap(),
    )
    .unwrap();
    let files = [
        "overriding-constant.php",
        "overriding-constant-native-types.php",
        "overriding-final-constant.php",
        "controls.php",
        "ignored.php",
    ];
    let mut project = Project::new(
        files.iter().map(|f| root.join(f)).collect(),
        vec![root.join("parents.php")],
    );
    project.level = level;
    project.php_version = 80400;
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .filter(|e| overriding(&e.identifier))
        .map(|e| {
            (
                Path::new(&e.file)
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
                e.line,
                e.identifier,
                e.message,
                e.tip,
                e.can_be_ignored,
            )
        })
        .collect();
    let mut want: Vec<_> = expected
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
fn native_and_effective_phpdoc_covariance_matches_phpstan() {
    check(3);
}

#[test]
fn level_zero_preserves_final_and_visibility_only() {
    check(0);
}

#[test]
fn level_two_does_not_check_override_types() {
    check(2);
}
