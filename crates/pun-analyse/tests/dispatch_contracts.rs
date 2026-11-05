//! Public dispatch boundaries, with complete independent PHPStan 2.3.1 diagnostic records.

use std::path::Path;

use pun_analyse::{Project, analyse_project};
use pun_phpstan::{Array, Key, Value as Parameter};
use serde_json::{Value, json};

fn check(group: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/dispatch-contracts");
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/dispatch-contracts/expected.json")).unwrap();
    let mut failures = Vec::new();
    let cases: Vec<_> = oracle
        .as_array()
        .unwrap()
        .iter()
        .filter(|case| case["group"] == group)
        .collect();
    assert_eq!(
        cases.len(),
        if group == "nullsafe" { 3 } else { 2 },
        "the capture retains each configuration"
    );
    for case in cases {
        let level = case["level"].as_u64().unwrap() as u8;
        let names = case["names"].as_bool().unwrap();
        let calls = case["calls"].as_bool().unwrap_or(true);
        let paths = |key: &str| {
            case[key]
                .as_array()
                .unwrap()
                .iter()
                .map(|file| root.join(file.as_str().unwrap()))
                .collect::<Vec<_>>()
        };
        let mut parameters = Array::new();
        parameters.set(Key::from("level"), Parameter::Int(i64::from(level)));
        parameters.set(Key::from("phpVersion"), Parameter::Int(80500));
        let mut toggles = Array::new();
        toggles.set(
            Key::from("checkNonStringableDynamicAccess"),
            Parameter::Bool(names),
        );
        parameters.set(Key::from("featureToggles"), Parameter::Array(toggles));
        let mut configured = Project::from_config(&root, &parameters);
        configured.analysed = paths("paths");
        configured.scanned = paths("scan");
        configured.method_rules.call_methods = calls;
        let mut direct = Project::new(paths("paths"), paths("scan"));
        direct.level = level;
        direct.php_version = 80500;
        direct.options.set(pun_analyse::ProjectOptions::CHECK_NON_STRINGABLE_DYNAMIC_ACCESS, names);
        direct.method_rules.call_methods = calls;
        let mut want: Vec<_> = case["messages"]
            .as_array()
            .unwrap()
            .iter()
            .map(|message| {
                json!([
                    message["file"],
                    message["line"],
                    message["identifier"],
                    message["message"],
                    message.get("tip").unwrap_or(&Value::Null),
                    message["ignorable"]
                ])
            })
            .collect();
        want.sort_by_cached_key(Value::to_string);
        for (constructor, project) in [("from_config", configured), ("new", direct)] {
            let result = analyse_project(&project).unwrap();
            assert_eq!(
                json!(result.general),
                case["general"],
                "{group}/{constructor}"
            );
            let mut got: Vec<_> = result
                .errors
                .into_iter()
                .map(|error| {
                    json!([
                        error.file.replace(root.to_str().unwrap(), "$ROOT"),
                        error.line,
                        error.identifier,
                        error.message,
                        error.tip,
                        error.can_be_ignored
                    ])
                })
                .collect();
            got.sort_by_cached_key(Value::to_string);
            println!(
                "{}/{constructor}: {}",
                case["id"],
                json!({"got": got, "want": want})
            );
            if got != want {
                failures.push(format!("{}/{constructor}", case["id"]));
            }
        }
    }
    assert!(failures.is_empty(), "mismatches: {}", failures.join(", "));
}

#[test]
fn closure_values_and_discovered_invoke_keep_pipe_controls() {
    check("callable-values");
}

#[test]
fn equal_and_distinct_guard_identities_remain_separate() {
    check("guards");
}

#[test]
fn anonymous_attributes_walk_arguments_and_readonly_modifiers() {
    check("anonymous-values");
}

#[test]
fn nullsafe_dynamic_names_respect_their_feature_toggle() {
    check("nullsafe");
}

#[test]
fn retained_scanned_trait_sources_are_not_admitted_to_host_analysis() {
    check("retained");
}
