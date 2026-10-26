//! PHPDoc union construction through the public analyser, using real PHPStan 2.3.1 expectations.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(file: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/phpdoc-unions");
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    let want: Vec<_> = oracle["errors"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|error| error["file"] == file)
        .map(|error| {
            (
                error["line"].as_i64().unwrap(),
                error["identifier"].as_str().unwrap().to_owned(),
                error["message"].as_str().unwrap().to_owned(),
                error["tip"].as_str().map(str::to_owned),
            )
        })
        .collect();
    let got: Vec<_> = run(&Project::new(vec![root.join(file)], Vec::new()))
        .unwrap()
        .into_iter()
        .map(|error| (error.line, error.identifier, error.message, error.tip))
        .collect();
    assert!(!want.is_empty());
    assert_eq!(got, want, "{file}");
}

#[test]
fn flat_129_arm_union_uses_the_normal_test_stack() {
    check("flat.php");
}

#[test]
fn null_order_nesting_and_template_controls() {
    check("controls.php");
}
