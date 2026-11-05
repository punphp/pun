//! Complete public PHPStan diagnostics retain written trait-alias spelling independently of lookup casing.
use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(case: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/mixed-case-alias");
    let mut project = Project::new(vec![root.join(format!("{case}.php"))], Vec::new());
    project.level = 5;
    project.php_version = 80500;
    project.strict_rules.set(
        pun_analyse::StrictRules::MATCHING_INHERITED_METHOD_NAMES,
        case == "strict",
    );
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| {
            (
                e.line,
                e.identifier,
                e.message,
                e.tip,
                e.can_be_ignored,
                e.file.replace(root.to_str().unwrap(), "$ROOT"),
            )
        })
        .collect();
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/mixed-case-alias/expected.json")).unwrap();
    let mut want: Vec<_> = oracle[case]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
                e["ignorable"].as_bool().unwrap(),
                e["file"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    got.sort();
    want.sort();
    assert_eq!(got, want, "{case}");
}

#[test]
fn direct_alias_spelling_is_independent_of_call_and_override_casing() {
    check("direct");
}

#[test]
fn inherited_and_nested_aliases_keep_the_nearest_written_name() {
    check("inherited");
}

#[test]
fn visibility_only_adaptations_preserve_original_names_and_private_controls() {
    check("visibility");
}

#[test]
fn original_names_and_own_declarations_do_not_take_alias_spelling() {
    check("original");
}

#[test]
fn inherited_case_checks_use_the_reflected_alias_not_its_storage_name() {
    check("strict");
}
