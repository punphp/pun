//! Larastan's resolved PHPDoc array-shape offsets through the public Project API.

use std::path::Path;

use pun_analyse::{Larastan, Project, analyse_project};
use serde_json::{Value, json};

#[test]
fn config_shape_offsets_match_larastan() {
    compare("Calls");
}

#[test]
fn config_shape_caller_zero_matches_larastan() {
    compare("CallerZero");
}

#[test]
fn config_shape_caller_one_matches_larastan() {
    compare("CallerOne");
}

#[test]
fn config_shape_outside_class_matches_larastan() {
    compare("CallerOutside");
}

#[test]
fn config_shape_scope_contexts_match_larastan() {
    compare("ScopeCases");
}

#[test]
fn config_shape_global_scope_matches_larastan() {
    compare("GlobalScope");
}

#[test]
fn config_shape_cached_scope_matches_larastan() {
    compare("CacheContexts");
}

#[test]
fn config_shape_cache_is_complete_keyed_and_keeps_missing_first_type() {
    compare("CacheAsymmetric");
}

fn compare(case: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/config-shape-offsets");
    let stubs = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/larastan/stubs.php");
    let mut project = Project::new(
        vec![root.join(format!("{case}.php"))],
        vec![stubs, root.join("ConfigFacade.php")],
    );
    let mut larastan = Larastan::new(&root);
    larastan
        .switches
        .set(pun_analyse::LarastanSwitches::CHECK_CONFIG_TYPES, true);
    project.larastan = Some(larastan);
    project.php_version = 80500;
    let result = analyse_project(&project).unwrap();
    let mut actual: Vec<_> = result
        .errors
        .into_iter()
        .map(|e| {
            json!([
                Path::new(&e.file).file_name().unwrap().to_str().unwrap(),
                e.line,
                e.identifier,
                e.message,
                e.tip,
                e.can_be_ignored
            ])
        })
        .collect();
    actual.sort_by_key(Value::to_string);
    let oracle: Value = serde_json::from_str(
        &std::fs::read_to_string(root.join(format!("expected-{case}.json"))).unwrap(),
    )
    .unwrap();
    println!(
        "{}",
        json!({"actual": actual, "oracle": oracle["errors"], "general": result.general})
    );
    assert!(result.general.is_empty(), "{:?}", result.general);
    assert_eq!(actual, *oracle["errors"].as_array().unwrap());
}
