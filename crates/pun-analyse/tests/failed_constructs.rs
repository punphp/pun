//! Failed construct inputs keep their primary diagnostics without a second type error.

use std::path::Path;

use pun_analyse::{Project, run_mago};
use serde_json::Value;

#[test]
fn failed_construct_reads_preserve_evaluation_and_independent_errors() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/failed-reads");
    let oracle: Value = serde_json::from_slice(
        &std::fs::read(root.join("expected-constructs-phpstan.json")).unwrap(),
    )
    .unwrap();
    let messages = oracle["files"]["constructs.php"]["messages"]
        .as_array()
        .unwrap();
    let mut project = Project::new(vec![root.join("constructs.php")], Vec::new());
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
                    | "possibly-invalid-argument"
                    | "null-argument"
                    | "possibly-null-argument"
                    | "false-argument"
                    | "possibly-false-argument"
                    | "less-specific-argument"
                    | "no-value"
            )
        })
        .map(|issue| (i64::from(issue.line), issue.code.as_str()))
        .collect();
    let mut want: Vec<_> = messages
        .iter()
        .filter(|error| {
            matches!(
                error["identifier"].as_str().unwrap(),
                "echo.nonString" | "print.nonString"
            )
        })
        .map(|error| (error["line"].as_i64().unwrap(), "invalid-argument"))
        .collect();
    // Preserve Mago's existing exit int|string contract, independently of PHPStan's exit checks.
    // Null, false, and unconstrained mixed do not satisfy that contract.
    want.extend([
        (35, "null-argument"),
        (40, "false-argument"),
        (45, "mixed-argument"),
    ]);
    got.sort();
    want.sort();
    assert_eq!(got, want, "raw issues: {:?}", raw.issues);

    let mut got: Vec<_> = raw
        .issues
        .iter()
        .filter(|issue| {
            matches!(
                issue.code.as_str(),
                "undefined-string-array-index" | "undefined-int-array-index" | "undefined-variable"
            )
        })
        .map(|issue| i64::from(issue.line))
        .collect();
    let mut want: Vec<_> = messages
        .iter()
        .filter(|error| {
            matches!(
                error["identifier"].as_str().unwrap(),
                "offsetAccess.notFound" | "variable.undefined"
            )
        })
        .map(|error| error["line"].as_i64().unwrap())
        .collect();
    got.sort();
    want.sort();
    assert_eq!(got, want);

    let got: Vec<_> = raw
        .issues
        .iter()
        .filter(|issue| issue.code == "impure-construct")
        .map(|issue| i64::from(issue.line))
        .collect();
    let want: Vec<_> = messages
        .iter()
        .filter(|error| error["identifier"] == "impure.echo")
        .map(|error| error["line"].as_i64().unwrap())
        .collect();
    assert_eq!(got, want);

    let got: Vec<_> = raw
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
}
