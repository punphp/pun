//! Expectations captured from the public PHPStan 2.3.1 analyser, not from Pun's inference.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(file: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/strlen");
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
    assert_ne!(
        want,
        [] as [(
            i64,
            std::string::String,
            std::string::String,
            std::option::Option<std::string::String>
        ); 0]
    );
    assert_eq!(got, want, "{file}");
}

#[test]
fn scalar_values_and_binary_byte_lengths() {
    check("values.php");
}

#[test]
fn scalar_extraction_queries_and_integer_range_boundaries() {
    check("domains.php");
}

#[test]
fn template_parent_extraction_and_queries() {
    check("templates.php");
}

#[test]
fn assigned_and_nested_calls_keep_the_precise_return() {
    check("flow.php");
}
