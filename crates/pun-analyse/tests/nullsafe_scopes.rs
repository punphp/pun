//! Complete PHPStan 2.3.1 public diagnostics, independent of observation transport.

use std::path::Path;

use pun_analyse::{Project, run};

#[test]
fn nullsafe_scope_devices_keep_the_original_public_diagnostics() {
    check(
        "nullsafe-scopes.php",
        include_str!("fixtures/call-scope-observations/nullsafe-scopes-expected.json"),
    );
}

#[test]
fn nullsafe_nested_devices_keep_void_and_undefined_diagnostics() {
    check(
        "nullsafe-scope-edges.php",
        include_str!("fixtures/call-scope-observations/nullsafe-scope-edges-expected.json"),
    );
}

#[test]
fn nullsafe_results_preserve_the_definitely_null_receiver_error() {
    check(
        "nullsafe-scope-results.php",
        include_str!("fixtures/call-scope-observations/nullsafe-scope-results-expected.json"),
    );
}

fn check(input: &str, expected: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/call-scope-observations");
    let expected: serde_json::Value = serde_json::from_str(expected).unwrap();
    let mut project = Project::new(vec![root.join(input)], Vec::new());
    project.php_version = 80500;
    project.check_this_only = Some(false);
    project.check_function_argument_types = Some(true);
    let mut actual: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|error| {
            (
                error.line,
                error.identifier,
                error.message,
                error.tip,
                error.can_be_ignored,
            )
        })
        .collect();
    actual.sort();
    assert_eq!(serde_json::to_value(actual).unwrap(), expected["errors"]);
}
