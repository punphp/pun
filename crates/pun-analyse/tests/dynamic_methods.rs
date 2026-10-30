//! Complete PHPStan 2.3.1 diagnostics for constant dynamic CallMethodsRule operands.
use std::path::Path;

use pun_analyse::{Project, run};
use pun_phpstan::{Array, Key, Value as Parameter};
use serde_json::Value;

fn check(mode: &str, level: u8) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/dynamic-methods");
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/dynamic-methods/expected.json")).unwrap();
    let key = format!("{mode}-{level}");
    let (file, mode) = mode
        .split_once('-')
        .filter(|(file, _)| matches!(*file, "boundaries" | "guards" | "nullsafe"))
        .unwrap_or(("calls", mode));
    let mut parameters = Array::new();
    parameters.set(Key::from("level"), Parameter::Int(i64::from(level)));
    match mode {
        "types-only" => parameters.set(
            Key::from("checkArgumentsPassedByReference"),
            Parameter::Bool(false),
        ),
        "references-only" => parameters.set(
            Key::from("checkFunctionArgumentTypes"),
            Parameter::Bool(false),
        ),
        "neither" => {
            parameters.set(
                Key::from("checkFunctionArgumentTypes"),
                Parameter::Bool(false),
            );
            parameters.set(
                Key::from("checkArgumentsPassedByReference"),
                Parameter::Bool(false),
            );
        }
        "this-only" => parameters.set(Key::from("checkThisOnly"), Parameter::Bool(true)),
        "all-receivers" => parameters.set(Key::from("checkThisOnly"), Parameter::Bool(false)),
        "case" => parameters.set(Key::from("checkFunctionNameCase"), Parameter::Bool(true)),
        _ => {}
    }
    let mut project = Project::from_config(&root, &parameters);
    project.analysed = vec![root.join(format!("{file}.php"))];
    project.php_version = 80500;
    project.method_rules.call_methods = mode != "no-calls";
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
fn constant_names_use_correlated_receivers_and_arguments() {
    for level in [0, 1, 2, 4, 5, 7] {
        check("default", level);
    }
}

#[test]
fn independent_argument_switches_preserve_count_and_void_checks() {
    for mode in ["types-only", "references-only", "neither"] {
        check(mode, 5);
    }
}

#[test]
fn receiver_and_registration_switches_remain_independent() {
    check("this-only", 5);
    check("all-receivers", 0);
    check("no-calls", 5);
}

#[test]
fn filtered_operands_preserve_selector_spans_overloads_and_reference_returns() {
    for level in [2, 5, 7] {
        check("boundaries-default", level);
    }
    check("boundaries-case", 5);
}

#[test]
fn dynamic_method_exists_uses_original_selector_and_invalidates_on_writes() {
    for level in [2, 5, 9] {
        check("guards-default", level);
    }
}

#[test]
fn nullsafe_void_checks_use_plain_calls_without_erasing_declared_null() {
    for level in [2, 5, 9] {
        check("nullsafe-default", level);
    }
}
