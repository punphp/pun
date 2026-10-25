//! InvalidThrowsPhpDocValueRule, pinned PHPStan 2.3.1. Expectations are generated from phpstan.neon, not Pun.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

const ID: &str = "throws.notThrowable";

fn project(file: &str, level: u8) -> Project {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/invalid-throws");
    let mut project = Project::new(vec![root.join(file)], Vec::new());
    project.level = level;
    project.php_version = 80400;
    project
}

fn check(file: &str) {
    let errors = run(&project(file, 2)).unwrap();
    let mut got: Vec<_> = errors
        .into_iter()
        .filter(|e| e.identifier == ID)
        .map(|e| (e.line, e.message, e.tip, e.can_be_ignored))
        .collect();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/invalid-throws");
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    let mut want: Vec<_> = oracle
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| e["file"] == file)
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
                e["ignorable"].as_bool().unwrap(),
            )
        })
        .collect();
    got.sort();
    want.sort();
    assert_eq!(got, want, "{file}");
}

#[test]
fn upstream_scalar_object_void_and_conditional_branches() {
    check("incompatible-throws.php");
}

#[test]
fn only_own_method_tags_are_validated_not_inherited_tags() {
    check("merge-inherited-throws.php");
}

#[test]
fn property_hook_virtual_node() {
    check("invalid-throws-property-hook.php");
}

#[test]
fn hook_templates_shadow_class_templates_and_apply_to_body_statements() {
    check("hook-templates.php");
}

#[test]
fn required_extends_interfaces_unions_and_intersections() {
    check("throws-with-require.php");
}

#[test]
fn tag_precedence_templates_names_and_negative_controls() {
    check("controls.php");
}

#[test]
fn statement_and_virtual_declaration_boundaries() {
    check("boundaries.php");
}

#[test]
fn invalid_throws_errors_are_locally_ignorable() {
    check("ignored.php");
}

#[test]
fn level_one_does_not_enable_the_rule() {
    assert!(
        run(&project("controls.php", 1))
            .unwrap()
            .iter()
            .all(|e| e.identifier != ID)
    );
}
