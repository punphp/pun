//! PHPStan 2.3.1's DumpTypeRule and DumpPhpDocTypeRule at level 0. The oracle fixture includes the deferred debug
//! rules' diagnostics so their calls also check that this slice does not accidentally implement them.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

const DUMPS: &[&str] = &["phpstan.dumpType", "phpstan.dumpPhpDocType"];

fn check(file: &str) {
    type Diagnostic = (i64, String, String, Option<String>, bool);
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/debug");
    let errors = run(&Project::new(vec![root.join(file)], Vec::new())).unwrap();
    assert!(
        errors.iter().all(
            |e| !e.identifier.starts_with("phpstan.") || DUMPS.contains(&e.identifier.as_str())
        ),
        "deferred assertion/native/scope rules must not report: {errors:?}"
    );
    let selected = |id: &str| {
        DUMPS.contains(&id) || matches!(id, "arguments.count" | "ignore.unmatchedIdentifier")
    };
    let mut got: Vec<Diagnostic> = errors
        .into_iter()
        .filter(|e| selected(&e.identifier))
        .map(|e| (e.line, e.identifier, e.message, e.tip, e.can_be_ignored))
        .collect();
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    let mut want: Vec<Diagnostic> = expected
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| e["file"] == file && selected(e["identifier"].as_str().unwrap()))
        .map(|e| {
            (
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
    assert_eq!(got, want, "{file}");
}

#[test]
fn named_calls_and_syntactic_arguments() {
    check("calls.php");
}

#[test]
fn phpdoc_aware_types_and_distinct_serialization() {
    check("types.php");
}

#[test]
fn unrelated_calls_and_deferred_rules_do_not_dump() {
    check("negative.php");
}

#[test]
fn dumps_cannot_be_locally_ignored() {
    check("ignored.php");
}

/// Neither analyser visits the argument after `return`: absent inference must not become a fabricated mixed dump.
#[test]
fn missing_expression_artifacts_do_not_fabricate_mixed() {
    check("unreachable.php");
}
