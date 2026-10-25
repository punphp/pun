//! PHPStan 2.3.1 expectations for declaration metadata at resolved method-call source edges.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(file: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/method-origins");
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    let mut got: Vec<_> = run(&Project::new(vec![root.join(file)], Vec::new()))
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message, e.can_be_ignored))
        .collect();
    let mut want: Vec<_> = oracle[file]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["ignorable"].as_bool().unwrap(),
            )
        })
        .collect();
    got.sort();
    want.sort();
    assert!(!want.is_empty());
    assert_eq!(got, want, "{file}");
}

#[test]
fn direct_static_and_inherited_returns_follow_the_resolved_declaration() {
    check("returns.php");
}

#[test]
fn generic_receivers_and_contextual_callbacks_keep_their_own_metadata() {
    check("generics.php");
}

#[test]
fn dynamic_unpacked_unresolved_and_mutated_values_do_not_borrow_origins() {
    check("controls.php");
}
