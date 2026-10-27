//! Global initializer values must reach normal inference, not only duplicate-key recovery.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

#[test]
fn forward_and_scanned_initializers_reach_full_types() {
    check("forward.php", Some("scanned.php"), "expected-phpstan.json");
}

#[test]
fn cycles_do_not_become_invented_literal_values() {
    check("cycles.php", None, "expected-cycles.json");
}

#[test]
fn global_initializer_copies_keep_scope_and_mixed_exclusions() {
    check("global.php", None, "expected-global.json");
}

#[test]
fn compiler_halt_offset_requires_a_parsed_statement() {
    let inputs: Value = serde_json::from_str(include_str!(
        "fixtures/constant-initializers/compiler-halt-inputs.json"
    ))
    .unwrap();
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/constant-initializers/expected-compiler-halt.json"
    ))
    .unwrap();
    let root = std::env::temp_dir().join(format!("pun-compiler-halt-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let mut mismatches = Vec::new();
    for (name, source) in inputs.as_object().unwrap() {
        let file = root.join(format!("{name}.php"));
        std::fs::write(&file, source.as_str().unwrap()).unwrap();
        let mut project = Project::new(vec![file], Vec::new());
        project.level = 1;
        project.php_version = 80500;
        let actual: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .map(|error| {
                serde_json::json!({
                    "line": error.line, "identifier": error.identifier,
                    "message": error.message, "tip": error.tip,
                    "ignorable": error.can_be_ignored,
                })
            })
            .collect();
        let actual = Value::Array(actual);
        if actual != expected[name]["errors"] {
            mismatches.push(format!(
                "{name}: got {actual}, want {}",
                expected[name]["errors"]
            ));
        }
    }
    std::fs::remove_dir_all(root).unwrap();
    assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
}

#[test]
fn loaded_vendor_initializers_do_not_scan_unrelated_autoload_files() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/constant-initializers");
    let mut extensions = pun_phpstan::Array::new();
    extensions.push(pun_phpstan::Value::str("php"));
    let mut params = pun_phpstan::Array::new();
    params.set(
        "fileExtensions".into(),
        pun_phpstan::Value::Array(extensions),
    );
    let mut project = Project::from_config(&root, &params);
    project.analysed = vec![root.join("autoload.php")];
    project.php_version = 80400;
    compare(project, "autoload.php", "expected-autoload.json");
}

fn check(file: &str, scanned: Option<&str>, expected: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/constant-initializers");
    let mut project = Project::new(
        vec![root.join(file)],
        scanned.map(|p| root.join(p)).into_iter().collect(),
    );
    project.php_version = 80400;
    compare(project, file, expected);
}

fn compare(project: Project, file: &str, expected: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/constant-initializers");
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join(expected)).unwrap()).unwrap();
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message, e.can_be_ignored))
        .collect();
    let mut want: Vec<_> = oracle["files"][file]["messages"]
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
    assert_eq!(got, want);
}
