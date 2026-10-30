//! Independent FunctionCallParametersCheck switches and RuleLevelHelper receiver eligibility.
//! Complete PHPStan 2.3.1 public expectations: fixtures/argument-settings/capture.php.

use std::path::Path;

use pun_analyse::{Project, run};
use pun_phpstan::{Array, Key, Value};

#[test]
fn public_argument_settings_matrix() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/argument-settings");
    let expected: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/argument-settings/expected.json")).unwrap();
    let mut failures = Vec::new();
    for (case, want) in expected.as_object().unwrap() {
        let parts: Vec<_> = case.split('-').collect();
        let level: u8 = parts[0].parse().unwrap();
        let mut params = Array::new();
        params.set(Key::from("level"), Value::Int(i64::from(level)));
        if parts[1] != "default" {
            for (name, value) in [
                "checkFunctionArgumentTypes",
                "checkArgumentsPassedByReference",
            ]
            .into_iter()
            .zip(parts[1].bytes())
            {
                params.set(Key::from(name), Value::Bool(value == b'T'));
            }
        }
        if parts[2] != "default" {
            params.set(Key::from("checkThisOnly"), Value::Bool(parts[2] == "this"));
        }
        let mut projects = vec![("from_config", Project::from_config(&root, &params))];
        if parts[1] == "default" && parts[2] == "default" {
            let mut project = Project::new(vec![root.join("matrix.php")], vec![]);
            // Defaults must resolve when running, not when Project::new() was called at level 0.
            project.level = level;
            projects.push(("new_then_level", project));
        }
        let mut want: Vec<_> = want
            .as_array()
            .unwrap()
            .iter()
            .map(|e| {
                (
                    e["file"].as_str().unwrap().to_string(),
                    e["line"].as_i64().unwrap(),
                    e["identifier"].as_str().unwrap().to_string(),
                    e["message"].as_str().unwrap().to_string(),
                    e["tip"].as_str().map(str::to_string),
                    e["ignorable"].as_bool().unwrap(),
                )
            })
            .collect();
        want.sort();
        for (constructor, mut project) in projects {
            project.analysed = vec![root.join("matrix.php")];
            project.php_version = 80500;
            let mut got: Vec<_> = run(&project)
                .unwrap()
                .into_iter()
                .map(|e| {
                    (
                        e.file.replace(root.to_str().unwrap(), "$ROOT"),
                        e.line,
                        e.identifier,
                        e.message,
                        e.tip,
                        e.can_be_ignored,
                    )
                })
                .collect();
            got.sort();
            if got != want {
                failures.push(format!(
                    "{case}/{constructor}\n  extra: {:?}\n  missing: {:?}",
                    got.iter().filter(|e| !want.contains(e)).collect::<Vec<_>>(),
                    want.iter().filter(|e| !got.contains(e)).collect::<Vec<_>>()
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} / {} cases failed:\n{}",
        failures.len(),
        expected.as_object().unwrap().len(),
        failures.join("\n")
    );
}
