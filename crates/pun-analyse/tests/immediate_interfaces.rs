//! Complete public PHPStan 2.3.1 expectations for ClassReflection::getImmediateInterfaces and
//! MethodPrototypeFinder, including named-argument producer keys and constructor signature prototypes.
use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(case: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/immediate-interfaces");
    let mut project = Project::new(vec![root.join(format!("{case}.php"))], Vec::new());
    project.level = 5;
    project.php_version = 80500;
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| {
            (
                e.line,
                e.identifier,
                e.message,
                e.tip,
                e.can_be_ignored,
                e.file.replace(root.to_str().unwrap(), "$ROOT"),
            )
        })
        .collect();
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/immediate-interfaces/expected.json")).unwrap();
    let mut want: Vec<_> = oracle[case]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
                e["ignorable"].as_bool().unwrap(),
                e["file"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    got.sort();
    want.sort();
    assert_eq!(got, want, "{case}");
}

#[test]
fn repeated_implements_selects_the_parent_method_not_the_repeated_contract() {
    check("repeated");
}

#[test]
fn transitive_interfaces_are_not_immediate_in_either_written_order() {
    check("transitive");
}

#[test]
fn sibling_interfaces_preserve_written_order_imports_and_anonymous_contexts() {
    check("ordering");
}

#[test]
fn constructor_signature_and_inheritance_prototypes_remain_distinct() {
    check("constructors");
}

#[test]
fn builtin_parents_interfaces_and_implicit_enum_interfaces_keep_their_roles() {
    check("builtins");
}
