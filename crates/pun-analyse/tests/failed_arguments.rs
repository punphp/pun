//! ErrorType arguments do not cause a second type diagnostic in the public maintained analyzer.

use std::path::Path;

use pun_analyse::{Project, run, run_mago};
use serde_json::Value;

#[test]
fn failed_reads_keep_argument_diagnostics_evaluation_and_reference_outputs() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/failed-reads");
    let oracle: Value = serde_json::from_slice(
        &std::fs::read(root.join("expected-arguments-phpstan.json")).unwrap(),
    )
    .unwrap();
    let messages = oracle["files"]["arguments.php"]["messages"]
        .as_array()
        .unwrap();
    let mut project = Project::new(vec![root.join("arguments.php")], Vec::new());
    project.php_version = 80400;
    project.level = 9;
    let raw = run_mago(&project, None).unwrap();
    let mut got: Vec<_> = raw
        .issues
        .iter()
        .filter(|issue| {
            matches!(
                issue.code.as_str(),
                "mixed-argument"
                    | "invalid-argument"
                    | "null-argument"
                    | "possibly-null-argument"
                    | "false-argument"
                    | "possibly-false-argument"
                    | "less-specific-argument"
                    | "less-specific-nested-argument-type"
            )
        })
        .map(|issue| i64::from(issue.line))
        .collect();
    let mut want: Vec<_> = messages
        .iter()
        .filter(|error| error["identifier"] == "argument.type")
        .map(|error| error["line"].as_i64().unwrap())
        .collect();
    got.sort_unstable();
    want.sort_unstable();
    assert_eq!(got, want, "raw argument issues: {:?}", raw.issues);
    let got: Vec<_> = raw
        .issues
        .iter()
        .filter(|issue| issue.code == "invalid-pass-by-reference")
        .map(|issue| i64::from(issue.line))
        .collect();
    let want: Vec<_> = messages
        .iter()
        .filter(|error| error["identifier"] == "argument.byRef")
        .map(|error| error["line"].as_i64().unwrap())
        .collect();
    assert_eq!(got, want);
    let got: Vec<_> = raw
        .dumped
        .iter()
        .map(|dump| {
            // Adapt only the tools' literal-integer display syntax, not their inferred values.
            let value = dump
                .type_id
                .strip_prefix("int(")
                .and_then(|value| value.strip_suffix(')'))
                .unwrap_or(&dump.type_id);
            (i64::from(dump.line), value)
        })
        .collect();
    let want: Vec<_> = messages
        .iter()
        .filter_map(|error| {
            error["message"]
                .as_str()
                .unwrap()
                .strip_prefix("Dumped type: ")
                .map(|value| (error["line"].as_i64().unwrap(), value))
        })
        .collect();
    assert_eq!(got, want);

    // Binding and reference checks are separate from the engine's argument type verification.
    let binding = |id: &str| {
        matches!(
            id,
            "arguments.count" | "argument.missing" | "argument.unknown" | "argument.byRef"
        )
    };
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .filter(|error| binding(&error.identifier))
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
    let mut want: Vec<_> = messages
        .iter()
        .filter(|error| binding(error["identifier"].as_str().unwrap()))
        .map(|error| {
            (
                error["line"].as_i64().unwrap(),
                error["identifier"].as_str().unwrap().to_string(),
                error["message"].as_str().unwrap().to_string(),
                error["tip"].as_str().map(str::to_string),
                error["ignorable"].as_bool().unwrap(),
            )
        })
        .collect();
    got.sort();
    want.sort();
    assert_eq!(got, want);
}
