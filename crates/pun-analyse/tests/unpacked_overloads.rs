//! Complete PHPStan 2.3.1 diagnostics for unpack-aware ParametersAcceptorSelector overloads.
use std::path::Path;

use pun_analyse::{Project, run};
use pun_phpstan::{Array, Key, Value as Parameter};
use serde_json::Value;

fn check(mode: &str, level: u8) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/unpacked-overloads");
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/unpacked-overloads/expected.json")).unwrap();
    let key = format!("{mode}-{level}");
    let mut parameters = Array::new();
    parameters.set(Key::from("level"), Parameter::Int(i64::from(level)));
    if ["references-only", "neither"].contains(&mode) {
        parameters.set(
            Key::from("checkFunctionArgumentTypes"),
            Parameter::Bool(false),
        );
    }
    if ["types-only", "neither"].contains(&mode) {
        parameters.set(
            Key::from("checkArgumentsPassedByReference"),
            Parameter::Bool(false),
        );
    }
    let mut project = Project::from_config(&root, &parameters);
    project.analysed = vec![root.join("calls.php")];
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
    let mut want: Vec<_> = oracle[&key]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["file"].as_str().unwrap().to_string(),
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_owned),
                e["ignorable"].as_bool().unwrap(),
            )
        })
        .collect();
    got.sort();
    want.sort();
    assert_eq!(got, want, "{key}");
}

#[test]
fn unpacked_overloads_use_types_before_selecting_variants() {
    for level in [0, 1, 2, 5, 7] {
        check("default", level);
    }
}

#[test]
fn overload_selection_is_independent_of_argument_check_switches() {
    for mode in ["types-only", "references-only", "neither"] {
        check(mode, 5);
    }
}
