//! Complete public diagnostics from PHPStan 2.3.1 with independently registered method-rule services.
//! Reproduce with fixtures/named-producer-gates/capture.php and the pinned PHPStan PHAR.

use std::path::Path;

use pun_analyse::{MethodRules, Project, run};
use serde_json::Value;

fn check(prefix: &str, mode: &str, filter_join_identifier: bool) {
    let fixture = if prefix == "ignore-identity-" {
        "named-join-ignore-identity"
    } else {
        "named-producer-gates"
    };
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);
    let files = match prefix {
        "boundaries-" => vec![root.join("boundaries.php")],
        "constructor-" => vec![root.join("constructor-only.php")],
        "ignore-identity-" => vec![root.join("calls.php")],
        _ => vec![root.join("contracts.php"), root.join("ignored.php")],
    };
    let mut project = Project::new(files, Vec::new());
    project.level = 5;
    project.php_version = 80500;
    project.check_non_stringable_dynamic_access = prefix == "boundaries-";
    project.method_rules = MethodRules {
        call_methods: mode != "no-calls",
        overriding_method: mode != "no-overrides",
        consistent_constructor: mode != "no-constructors",
        renamed_named_arguments: filter_join_identifier || mode != "no-join",
    };
    if filter_join_identifier {
        project.reported = Some(std::sync::Arc::new(|id| {
            id != "argument.parameterRenamedInSubtype"
        }));
    }
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
    let capture = if prefix == "ignore-identity-" {
        include_str!("fixtures/named-join-ignore-identity/expected.json")
    } else {
        include_str!("fixtures/named-producer-gates/expected.json")
    };
    let oracle: Value = serde_json::from_str(capture).unwrap();
    let case = format!("{prefix}{mode}");
    let mut want: Vec<_> = oracle[&case]
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
    got.sort();
    want.sort();
    assert_eq!(
        got, want,
        "{case}, filter_join_identifier={filter_join_identifier}"
    );
}

#[test]
fn registered_owners_emit_complete_normal_and_joined_diagnostics() {
    check("", "all", false);
    check("boundaries-", "all", false);
    check("constructor-", "all", false);
}

#[test]
fn disabled_call_rule_preserves_enabled_override_diagnostics_and_unmatched_ignores() {
    check("", "no-calls", false);
    check("boundaries-", "no-calls", false);
    check("constructor-", "no-calls", false);
}

#[test]
fn disabled_override_rule_preserves_consistent_constructor_collection_and_errors() {
    check("", "no-overrides", false);
    check("boundaries-", "no-overrides", false);
    check("constructor-", "no-overrides", false);
}

#[test]
fn disabled_constructor_rule_preserves_method_rename_collection_and_errors() {
    check("", "no-constructors", false);
    check("boundaries-", "no-constructors", false);
    check("constructor-", "no-constructors", false);
}

#[test]
fn disabled_join_drops_collector_records_without_consuming_unmatched_ignores() {
    check("", "no-join", false);
    check("boundaries-", "no-join", false);
    check("constructor-", "no-join", false);
}

#[test]
fn identifier_filtering_remains_separate_from_rule_registration() {
    check("", "no-join", true);
}

#[test]
fn joined_ignores_match_the_complete_identifier_in_either_order() {
    check("ignore-identity-", "all", false);
}

#[test]
fn disabled_join_preserves_exact_and_similar_unmatched_identifiers() {
    check("ignore-identity-", "no-join", false);
}

#[test]
fn identifier_filtering_preserves_exact_and_similar_unmatched_identifiers() {
    check("ignore-identity-", "no-join", true);
}
