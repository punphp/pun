//! Shared parameter checks against the PHPStan 2.3.1 phar, including attribute constructors.

use pun_analyse::{Project, reports, run};
use serde_json::Value;
use std::path::Path;

const IDS: &[&str] = &["argument.staticClosure", "argument.unresolvableType"];

fn check(level: u8, version: u32, files: &[&str], expected: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/argument-contracts");
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join(expected)).unwrap()).unwrap();
    let mut project = Project::new(files.iter().map(|f| root.join(f)).collect(), Vec::new());
    project.level = level;
    project.php_version = version;
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| {
            (
                Path::new(&e.file)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                e.line,
                e.identifier,
                e.message,
                e.tip,
            )
        })
        .collect();
    let mut want: Vec<_> = oracle["errors"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| {
            let id = e["identifier"].as_str().unwrap();
            reports(id) || IDS.contains(&id)
        })
        .map(|e| {
            (
                e["file"].as_str().unwrap().to_string(),
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
            )
        })
        .collect();
    got.sort();
    want.sort();
    assert_eq!(got, want, "level {level}");
}

#[test]
fn shared_closure_and_unresolvable_parameter_checks_match_phpstan() {
    check(
        5,
        80500,
        &["static.php", "unresolvable.php", "no-named.php"],
        "expected-phpstan.json",
    );
}

#[test]
fn shared_parameter_checks_start_at_level_five() {
    check(
        4,
        80500,
        &["static.php", "unresolvable.php", "no-named.php"],
        "expected-phpstan-level4.json",
    );
    check(5, 70400, &["legacy.php"], "expected-phpstan-php74.json");
}

#[test]
fn original_shared_parameter_fixtures_match_without_gaps() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let data = root.join("rules/data");
    let prefix = format!("{}/", data.display());
    let mut checked = 0;
    for level in [5, 9] {
        for (suite, method) in [
            (
                "Functions/CallToFunctionParametersRuleTest",
                "testParamClosureThis",
            ),
            ("Methods/CallMethodsRuleTest", "testUnresolvableParameter"),
        ] {
            let oracle: Value = serde_json::from_slice(
                &std::fs::read(root.join(format!("levels/{level}/{suite}.json"))).unwrap(),
            )
            .unwrap();
            for case in oracle["cases"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|c| c["method"] == method)
            {
                checked += 1;
                let paths = |key: &str| {
                    case[key]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|p| data.join(p.as_str().unwrap()))
                        .collect()
                };
                let mut project = Project::new(paths("files"), paths("scan"));
                project.level = level;
                let mut got: Vec<_> = run(&project)
                    .unwrap()
                    .into_iter()
                    .map(|e| {
                        (
                            e.file.strip_prefix(&prefix).unwrap().to_string(),
                            e.line,
                            e.identifier,
                            e.message,
                        )
                    })
                    .collect();
                let mut want: Vec<_> = case["errors"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|e| {
                        let id = e["identifier"].as_str().unwrap();
                        reports(id) || IDS.contains(&id)
                    })
                    .map(|e| {
                        (
                            e["file"].as_str().unwrap().to_string(),
                            e["line"].as_i64().unwrap(),
                            e["identifier"].as_str().unwrap().to_string(),
                            e["message"].as_str().unwrap().to_string(),
                        )
                    })
                    .collect();
                got.sort();
                want.sort();
                assert_eq!(got, want, "level {level}: {suite}::{method}");
            }
        }
    }
    assert_eq!(checked, 4);
}

/// Check every shared-argument diagnostic of the original caller fixture. Its independent unused-result errors
/// belong to the dead-code checker, not the callable parameter contract.
#[test]
fn original_first_class_callbacks_retain_no_named_arguments() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let oracle: Value = serde_json::from_slice(
        &std::fs::read(root.join("levels/5/Functions/CallUserFuncRuleTest.json")).unwrap(),
    )
    .unwrap();
    let case = oracle["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["method"] == "testNoNamedArguments")
        .unwrap();
    let files = case["files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| root.join("rules/data").join(p.as_str().unwrap()))
        .collect();
    let mut project = Project::new(files, Vec::new());
    project.level = 5;
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .filter(|e| e.identifier.starts_with("argument."))
        .map(|e| (e.line, e.identifier, e.message, e.tip))
        .collect();
    let mut want: Vec<_> = case["errors"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| e["identifier"].as_str().unwrap().starts_with("argument."))
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
            )
        })
        .collect();
    got.sort();
    want.sort();
    assert_eq!(want.len(), 8);
    assert_eq!(got, want);
}

#[test]
fn void_diagnostics_keep_raw_calls_while_argument_types_use_values() {
    // PHPStan 2.3.1, level N and phpVersion 80500 on void-values.php; retain complete diagnostics.
    for level in [1, 2, 5] {
        check(
            level,
            80500,
            &["void-values.php"],
            &format!("expected-void-values-level{level}.json"),
        );
    }
}
