//! Missing destructuring offsets remain ErrorType; genuine mixed assignments still warn.

use std::path::Path;

use pun_analyse::{Project, run_mago};
use serde_json::Value;

#[test]
fn error_assignments_preserve_primary_errors_and_types_without_suppressing_mixed() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/assignment-error-values");
    // PHPStan 2.3.1, phpstan-src cb3b7e99489b2e3789ee2eec10501cb7a72580ea, level 8, PHP 8.5.
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    let messages = oracle["files"]["values.php"]["messages"]
        .as_array()
        .unwrap();
    let mut project = Project::new(vec![root.join("values.php")], Vec::new());
    project.level = 8;
    project.php_version = 80500;

    let raw = run_mago(&project, None).unwrap();
    let mixed_assignments: Vec<_> = raw
        .issues
        .iter()
        .filter(|issue| issue.code == "mixed-assignment")
        .map(|issue| i64::from(issue.line))
        .collect();
    // Only the final dump assigns genuine mixed. ErrorType and null assignments must not warn.
    let mixed_lines: Vec<_> = messages
        .iter()
        .filter(|message| message["message"] == "Dumped type: mixed")
        .map(|message| message["line"].as_i64().unwrap())
        .collect();
    assert_eq!(mixed_assignments, mixed_lines, "{:?}", raw.issues);

    let got_types: Vec<_> = raw
        .dumped
        .iter()
        .map(|dump| {
            let value = dump
                .type_id
                .strip_prefix("int(")
                .and_then(|value| value.strip_suffix(')'))
                .unwrap_or(&dump.type_id);
            (i64::from(dump.line), value)
        })
        .collect();
    let expected_types: Vec<_> = messages
        .iter()
        .filter_map(|message| {
            message["message"]
                .as_str()
                .unwrap()
                .strip_prefix("Dumped type: ")
                .map(|value| (message["line"].as_i64().unwrap(), value))
        })
        .collect();
    assert_eq!(got_types, expected_types);

    // Compare the complete native issue stream, not Pun's separate PHPStan-style consumer.
    // The inherited unit fixtures distinguish empty, keyed and list offset errors by these codes.
    let mut got: Vec<_> = raw
        .issues
        .iter()
        .map(|issue| (i64::from(issue.line), issue.code.as_str()))
        .collect();
    let mut expected: Vec<_> = messages
        .iter()
        .filter(|message| message["identifier"] == "offsetAccess.notFound")
        .map(|message| {
            let text = message["message"].as_str().unwrap();
            let code = if text.ends_with("on array{}.") {
                "mismatched-array-index"
            } else if text.starts_with("Offset '") {
                "undefined-string-array-index"
            } else {
                "undefined-int-array-index"
            };
            (message["line"].as_i64().unwrap(), code)
        })
        .collect();
    expected.extend(mixed_lines.iter().map(|&line| (line, "mixed-assignment")));
    got.sort();
    expected.sort();
    assert_eq!(got, expected);
}
