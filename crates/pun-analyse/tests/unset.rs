//! UnsetRule's public contract from PHPStan 2.3.1's rule fixtures and actual oracle output.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(project: Project, expected: &Value) -> Result<(), String> {
    let got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message, e.tip, e.can_be_ignored))
        .collect();
    let mut want: Vec<_> = expected
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
                e["ignorable"].as_bool().unwrap_or(true),
            )
        })
        .collect();
    want.sort_by(|a, b| (a.0, &a.2).cmp(&(b.0, &b.2)));
    if got == want {
        Ok(())
    } else {
        Err(format!(
            "{:?}; PHP {}\nactual: {got:?}\nexpected: {want:?}",
            project.analysed, project.php_version
        ))
    }
}

#[test]
fn upstream_unset_rule() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rules");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("Variables/UnsetRuleTest.json")).unwrap())
            .unwrap();
    let mut failures = Vec::new();
    for case in expected["cases"].as_array().unwrap() {
        let paths = |key: &str| {
            case[key]
                .as_array()
                .unwrap()
                .iter()
                .map(|p| root.join("data").join(p.as_str().unwrap()))
                .collect()
        };
        if let Err(error) = check(Project::new(paths("files"), paths("scan")), &case["errors"]) {
            failures.push(format!("{}: {error}", case["method"]));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

fn oracle(name: &str, expected_name: &str, php_version: u32) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/unset");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join(expected_name)).unwrap()).unwrap();
    let messages = expected["files"]
        .as_object()
        .unwrap()
        .values()
        .next()
        .unwrap()["messages"]
        .clone();
    let mut project = Project::new(vec![root.join(name)], Vec::new());
    project.php_version = php_version;
    check(project, &messages).unwrap();
}

#[test]
fn recursive_offsets_and_entry_certainty() {
    oracle("offsets.php", "offsets.json", 80400);
}

#[test]
fn native_properties_and_reflection_controls() {
    oracle("properties.php", "properties.json", 80400);
}

#[test]
fn key_casts_and_unsealed_offsets() {
    oracle("keys.php", "keys.json", 80400);
}

#[test]
fn possibly_hooked_properties_depend_on_php_version() {
    oracle("version.php", "version.json", 80400);
    oracle("version.php", "version-82.json", 80200);
}
