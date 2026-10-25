//! Public inference and reachability expectations from PHPStan 2.3.1 (fixtures/array-rand/phpstan.neon).

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(file: &str) {
    check_expected(file, 5, "expected-phpstan.json");
}

fn check_expected(file: &str, level: u8, expected: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/array-rand");
    let mut project = Project::new(vec![root.join(file)], Vec::new());
    project.level = level;
    let got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message, e.tip))
        .collect();
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join(expected)).unwrap()).unwrap();
    let mut want: Vec<_> = oracle["files"][file]["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
            )
        })
        .collect();
    want.sort_by(|a, b| (a.0, &a.2).cmp(&(b.0, &b.2)));
    assert!(!want.is_empty());
    assert_eq!(got, want, "{file}");
}

#[test]
fn keys_cardinality_and_empty_arrays() {
    check("types.php");
}

#[test]
fn empty_array_calls_do_not_replace_explicit_never_termination() {
    check("reachability.php");
}

#[test]
fn generic_bottom_keys_are_not_the_constant_empty_fallback() {
    check_expected("generics.php", 9, "expected-generics.json");
}

#[test]
fn upstream_array_rand_at_strict_levels() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    for level in [5, 9] {
        let expected: Value = serde_json::from_slice(
            &std::fs::read(root.join(format!(
                "levels/{level}/Functions/CallToFunctionParametersRuleTest.json"
            )))
            .unwrap(),
        )
        .unwrap();
        let case = expected["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|case| case["method"] == "testArrayRand")
            .unwrap();
        let mut project = Project::new(
            vec![root.join("rules/data/Functions/data/array_rand.php")],
            Vec::new(),
        );
        project.level = level;
        let got: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .map(|e| (e.line, e.identifier, e.message, e.tip))
            .collect();
        let mut want: Vec<_> = case["errors"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| {
                (
                    e["line"].as_i64().unwrap(),
                    e["identifier"].as_str().unwrap().to_string(),
                    e["message"].as_str().unwrap().to_string(),
                    e["tip"].as_str().map(str::to_string),
                )
            })
            .collect();
        want.sort_by(|a, b| (a.0, &a.2).cmp(&(b.0, &b.2)));
        assert_eq!(got, want, "upstream testArrayRand, level {level}");
    }
}
