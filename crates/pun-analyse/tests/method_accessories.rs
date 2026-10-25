//! PHPStan 2.3.1's MethodExistsTypeSpecifyingExtension and HasMethodType (cb3b7e99).

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

#[test]
fn literal_method_guards_preserve_types_case_and_branch_scope() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/debug");
    let errors = run(&Project::new(
        vec![root.join("method-accessories.php")],
        vec![],
    ))
    .unwrap();
    let got: Vec<_> = errors
        .iter()
        .filter(|e| e.identifier.starts_with("phpstan.dump"))
        .map(|e| {
            (
                e.line,
                e.identifier.as_str(),
                e.message.as_str(),
                e.can_be_ignored,
            )
        })
        .collect();
    let oracle: Value = serde_json::from_slice(
        &std::fs::read(root.join("method-accessories-phpstan.json")).unwrap(),
    )
    .unwrap();
    let want: Vec<_> = oracle["files"]
        .as_object()
        .unwrap()
        .values()
        .flat_map(|f| f["messages"].as_array().unwrap())
        .filter(|e| {
            e["identifier"]
                .as_str()
                .unwrap()
                .starts_with("phpstan.dump")
        })
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap(),
                e["message"].as_str().unwrap(),
                e["ignorable"].as_bool().unwrap(),
            )
        })
        .collect();
    assert_eq!(got, want);
}

#[test]
fn bug_1267_static_class_string_retains_literal_method_accessory() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rules/data/Methods/data");
    let errors = run(&Project::new(vec![root.join("bug-1267.php")], vec![])).unwrap();
    let dumps: Vec<_> = errors
        .iter()
        .filter(|e| e.identifier == "phpstan.dumpType")
        .map(|e| (e.line, e.message.as_str()))
        .collect();
    assert_eq!(
        dumps,
        [(
            19,
            "Dumped type: class-string<static(Bug1267\\ObjectType)>&hasMethod(get_connections)"
        )]
    );
}

#[test]
fn method_guard_identity_keeps_other_guard_kinds_distinct() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/debug");
    let mut project = Project::new(vec![root.join("method-guard-identity.php")], vec![]);
    project.level = 2;
    let errors = run(&project).unwrap();
    let got: Vec<_> = errors
        .iter()
        .map(|e| {
            (
                e.line,
                e.identifier.as_str(),
                e.message.as_str(),
                e.tip.as_deref(),
            )
        })
        .collect();
    let oracle: Value = serde_json::from_slice(
        &std::fs::read(root.join("method-guard-identity-phpstan.json")).unwrap(),
    )
    .unwrap();
    let want: Vec<_> = oracle["files"]["method-guard-identity.php"]["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap(),
                e["message"].as_str().unwrap(),
                e["tip"].as_str(),
            )
        })
        .collect();
    assert_eq!(got, want);
}
