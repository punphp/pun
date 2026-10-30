//! Effectful operand and loop joins must invalidate only affected method specifications.

use std::path::Path;

use pun_analyse::{Project, analyse_project};
use serde_json::{Value, json};

#[test]
fn method_specification_edges_match_complete_phpstan_records() {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/method-specification-edges");
    let expected: Vec<Value> = serde_json::from_str(include_str!(
        "fixtures/method-specification-edges/expected.json"
    ))
    .unwrap();
    let mut project = Project::new(vec![root.join("input.php")], Vec::new());
    project.php_version = 80500;
    project.level = 4;
    let analysis = analyse_project(&project).unwrap();
    let mut actual: Vec<_> = analysis
        .errors
        .into_iter()
        .map(|error| {
            json!([
                "input.php",
                error.line,
                error.identifier,
                error.message,
                error.tip,
                error.can_be_ignored,
            ])
        })
        .collect();
    actual.sort_by_cached_key(Value::to_string);
    let mut expected = expected;
    expected.sort_by_cached_key(Value::to_string);
    println!(
        "method-specification-edges={}",
        json!({"actual":actual,"oracle":expected})
    );
    assert_eq!(actual, expected);
    assert!(analysis.general.is_empty(), "{:?}", analysis.general);
}
