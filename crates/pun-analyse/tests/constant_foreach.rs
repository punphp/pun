//! PHPStan's sequential constant-array foreach scopes through public analysis.

use std::path::Path;

#[test]
fn nested_conditional_and_continue_writes_satisfy_integer_parameters() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let mut project = pun_analyse::Project::new(
        vec![root.join("rules/data/Functions/data/bug-13643.php")],
        vec![],
    );
    project.level = 9;
    project.php_version = 80400;
    let errors = pun_analyse::run(&project).unwrap();
    assert!(errors.is_empty(), "{errors:#?}");
}

fn compare(file: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/constant-foreach");
    let mut project = pun_analyse::Project::new(vec![root.join(file)], vec![]);
    project.level = 9;
    project.php_version = 80400;
    let got: Vec<_> = pun_analyse::run(&project)
        .unwrap()
        .into_iter()
        .map(|e| serde_json::json!([e.line, e.identifier, e.message, e.tip, e.can_be_ignored]))
        .collect();
    let oracle: serde_json::Value = serde_json::from_str(include_str!(
        "fixtures/constant-foreach/expected-phpstan.json"
    ))
    .unwrap();
    assert!(oracle["errors"].as_array().unwrap().is_empty());
    let expected = oracle["files"]
        .as_object()
        .unwrap()
        .iter()
        .find(|(path, _)| Path::new(path).file_name().unwrap() == file)
        .unwrap()
        .1;
    let want: Vec<_> = expected["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            serde_json::json!([
                e["line"],
                e["identifier"],
                e["message"],
                e.get("tip").unwrap_or(&serde_json::Value::Null),
                e["ignorable"]
            ])
        })
        .collect();
    assert_eq!(got, want, "{file}");
}

#[test]
fn named_offsets_keep_already_integer_missing_and_noninteger_values_distinct() {
    compare("values.php");
}

#[test]
fn continue_edges_and_nested_unroll_bounds_match_phpstan() {
    compare("boundaries.php");
}

#[test]
fn ordinary_body_observations_do_not_include_speculative_iterations() {
    compare("observations.php");
}

#[test]
fn literal_selector_scope_refines_only_the_selected_offset() {
    compare("scopes.php");
}
