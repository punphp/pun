//! Public inference and reachability expectations from PHPStan 2.3.1 (fixtures/array-rand/phpstan.neon).

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(file: &str) {
    check_expected(file, 5, "expected-phpstan.json");
}

fn check_expected(file: &str, level: u8, expected: &str) {
    check_version(file, level, expected, 80500);
}

fn check_version(file: &str, level: u8, expected: &str, php_version: u32) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/array-rand");
    let mut project = Project::new(vec![root.join(file)], Vec::new());
    project.level = level;
    project.php_version = php_version;
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
fn generic_union_and_non_empty_picked_keys() {
    check_expected("full.php", 0, "expected-full.json");
}

#[test]
fn random_bounds_coercions_and_nested_calls() {
    check_expected("random.php", 0, "expected-full.json");
}

#[test]
fn invalid_random_bounds_and_controls_match_across_php_versions() {
    for php_version in [70400, 80500] {
        check_version(
            "random-versions.php",
            5,
            "expected-random-versions.json",
            php_version,
        );
    }
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
        for (method, file) in [
            ("testArrayRand", "array_rand.php"),
            ("testBug9803", "bug-9803.php"),
        ] {
            let case = expected["cases"]
                .as_array()
                .unwrap()
                .iter()
                .find(|case| case["method"] == method)
                .unwrap();
            let mut project = Project::new(
                vec![root.join(format!("rules/data/Functions/data/{file}"))],
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
            assert_eq!(got, want, "upstream {method}, level {level}");
        }
    }
}
