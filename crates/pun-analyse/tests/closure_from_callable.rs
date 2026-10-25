//! Closure::fromCallable contracts from actual PHPStan 2.3.1 runs, through the public analyser.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(file: &str, level: u8) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/closure-from-callable");
    let mut project = Project::new(vec![root.join(file)], Vec::new());
    project.level = level;
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message, e.tip, e.can_be_ignored))
        .collect();
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    let mut want: Vec<_> = expected[file]
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
            )
        })
        .collect();
    got.sort();
    want.sort();
    assert!(!want.is_empty());
    assert_eq!(got, want, "{file}, level {level}");
}

#[test]
fn strings_arrays_invokables_and_closures() {
    check("forms.php", 0);
}

#[test]
fn parameters_generics_assignments_and_invocation() {
    check("signatures.php", 5);
}

#[test]
fn all_resolved_alternatives_or_the_declared_closure() {
    check("unions.php", 5);
}

#[test]
fn inherited_targets_visibility_staticness_and_invalid_arguments() {
    check("controls.php", 5);
}
