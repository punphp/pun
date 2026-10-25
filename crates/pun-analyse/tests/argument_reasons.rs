//! Public calls and declaration controls, captured from the unmodified PHPStan 2.3.1 phar.

use pun_analyse::{Project, run};
use serde_json::Value;
use std::path::Path;

fn check(level: u8, expected: &str, count: usize) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/argument-contracts");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join(expected)).unwrap()).unwrap();
    let mut project = Project::new(vec![root.join("reasons.php")], Vec::new());
    project.level = level;
    project.php_version = 80500;
    let relevant = |id: &str| {
        matches!(
            id,
            "argument.unresolvableType" | "parameter.unresolvableType"
        )
    };
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .filter(|e| relevant(&e.identifier))
        .map(|e| (e.line, e.identifier, e.message, e.tip))
        .collect();
    let mut want: Vec<_> = expected["errors"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| relevant(e["identifier"].as_str().unwrap()))
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
            )
        })
        .collect();
    assert_eq!(
        want.iter()
            .filter(|e| e.1 == "argument.unresolvableType")
            .count(),
        count
    );
    got.sort();
    want.sort();
    assert_eq!(got, want, "level {level}");
}

#[test]
fn shared_unresolvable_reasons_match_phpstan_for_functions_methods_and_attributes() {
    check(5, "expected-reasons-phpstan.json", 20);
    check(9, "expected-reasons-phpstan.json", 20);
}

#[test]
fn unresolvable_calls_start_at_level_five_and_do_not_duplicate_invalid_declarations() {
    check(4, "expected-reasons-level4-phpstan.json", 0);
}
