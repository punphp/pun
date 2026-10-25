//! Public rule expectations from PHPStan 2.3.1, including its upstream union-key fixtures.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(project: Project, errors: &Value) {
    let got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message, e.tip, e.can_be_ignored))
        .collect();
    let mut want: Vec<_> = errors
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
                e["ignorable"].as_bool().unwrap_or(true),
            )
        })
        .collect();
    want.sort_by(|a, b| (a.0, &a.2).cmp(&(b.0, &b.2)));
    assert_eq!(
        got, want,
        "{:?}; PHP {}; bleeding edge {}",
        project.analysed, project.php_version, project.bleeding_edge
    );
}

#[test]
fn upstream_duplicate_keys_and_array_regressions() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rules");
    let expected: Value = serde_json::from_slice(
        &std::fs::read(root.join("Arrays/DuplicateKeysInLiteralArraysRuleTest.json")).unwrap(),
    )
    .unwrap();
    for case in expected["cases"].as_array().unwrap() {
        let file = case["files"][0].as_str().unwrap();
        check(
            Project::new(vec![root.join("data").join(file)], Vec::new()),
            &case["errors"],
        );
    }
}

#[test]
fn inferred_keys_and_pending_union_promotion() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/duplicate-array-keys");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    check(
        Project::new(vec![root.join("keys.php")], Vec::new()),
        &expected["files"]["keys.php"]["messages"],
    );
}

#[test]
fn float_casts_constants_and_overridable_receivers() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/duplicate-array-keys");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-scalars.json")).unwrap())
            .unwrap();
    let mut project = Project::new(vec![root.join("scalars.php")], Vec::new());
    project.php_version = 80400;
    check(project, &expected["files"]["scalars.php"]["messages"]);
}

#[test]
fn compound_constant_initializers_use_the_declaring_scope() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/duplicate-array-keys");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-initializers.json")).unwrap())
            .unwrap();
    let mut project = Project::new(
        vec![root.join("initializers.php")],
        vec![root.join("initializers-source.php")],
    );
    project.php_version = 80400;
    check(project, &expected["files"]["initializers.php"]["messages"]);
}

#[test]
fn platform_integer_copies_preserve_bitness_and_actual_narrowing() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/duplicate-array-keys");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-platform.json")).unwrap())
            .unwrap();
    let mut project = Project::new(vec![root.join("platform.php")], Vec::new());
    project.php_version = 80400;
    check(project, &expected["files"]["platform.php"]["messages"]);
}

fn unpacking(config: &str, php_version: u32, bleeding_edge: bool) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/duplicate-array-keys");
    let expected: Value = serde_json::from_slice(
        &std::fs::read(root.join(format!("expected-{config}.json"))).unwrap(),
    )
    .unwrap();
    let mut project = Project::new(vec![root.join("unpacking.php")], Vec::new());
    project.php_version = php_version;
    project.bleeding_edge = bleeding_edge;
    check(project, &expected["files"]["unpacking.php"]["messages"]);
}

#[test]
fn unpacking_preserves_default_unknown_sealedness() {
    unpacking("unpacking", 80400, false);
}

#[test]
fn unpacking_sealed_shapes_with_bleeding_edge() {
    unpacking("unpacking-bleeding", 80400, true);
}

#[test]
fn unpacking_before_php81_renumbers_string_keys() {
    unpacking("unpacking-php80", 80000, true);
}
