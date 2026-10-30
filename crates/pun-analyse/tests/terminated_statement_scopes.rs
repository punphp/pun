//! Reachability belongs to the source walk, independently of UnreachableStatementRule's level.

use std::path::Path;

use pun_analyse::{Project, analyse_project};
use serde_json::{Value, json};

#[test]
fn terminated_statements_match_complete_phpstan_records_at_each_level() {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/terminated-statement-scopes");
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/terminated-statement-scopes/expected.json"
    ))
    .unwrap();
    let mut failures = Vec::new();
    for level in 0..=4 {
        let mut project = Project::new(vec![root.join("input.php")], Vec::new());
        project.php_version = 80500;
        project.level = level;
        project.check_maybe_undefined_variables = level >= 1;
        let analysis = analyse_project(&project).unwrap();
        let mut actual: Vec<_> = analysis
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
        let mut oracle = expected["levels"][level as usize]
            .as_array()
            .unwrap()
            .clone();
        actual.sort_by_cached_key(Value::to_string);
        oracle.sort_by_cached_key(Value::to_string);
        println!(
            "terminated-statement-records={}",
            json!({"level":level,"actual":actual,"oracle":oracle,"general":analysis.general})
        );
        if actual != oracle || !analysis.general.is_empty() {
            failures.push(
                json!({"level":level,"actual":actual,"oracle":oracle,"general":analysis.general}),
            );
        }
    }
    assert!(failures.is_empty(), "{}", json!(failures));
}
