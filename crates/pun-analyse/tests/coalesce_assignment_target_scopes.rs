//! Coalesce's assignment scope applies to target chains, not index or right-hand-side reads.

use std::path::Path;

use pun_analyse::{Project, analyse_project};
use serde_json::{Value, json};

#[test]
fn coalesce_target_scopes_match_complete_phpstan_records() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/coalesce-assignment-target-scopes");
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/coalesce-assignment-target-scopes/expected.json"
    ))
    .unwrap();
    let mut project = Project::new(vec![root.join("input.php")], Vec::new());
    project.php_version = 80500;
    let result = analyse_project(&project).unwrap();
    let mut actual: Vec<_> = result
        .errors
        .into_iter()
        .map(|e| {
            json!([
                "input.php",
                e.line,
                e.identifier,
                e.message,
                e.tip,
                e.can_be_ignored
            ])
        })
        .collect();
    let mut expected = oracle["records"].as_array().unwrap().clone();
    actual.sort_by_cached_key(Value::to_string);
    expected.sort_by_cached_key(Value::to_string);
    println!(
        "coalesce-target-records={}",
        json!({"actual":actual,"oracle":expected,"general":result.general})
    );
    assert!(result.general.is_empty(), "{:?}", result.general);
    assert_eq!(actual, expected);
}
