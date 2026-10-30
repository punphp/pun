//! Source-owned selection, unpacked name policy and constant identity through public Project execution.

use std::path::Path;

use pun_analyse::{Project, analyse_project};
use serde_json::{Value, json};

#[test]
fn complete_parameter_branch_contracts_match_phpstan() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/params-branch-contracts");
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/params-branch-contracts/expected.json"
    ))
    .unwrap();
    let mut failures = Vec::new();
    for name in ["named", "constants", "variants"] {
        for level in [0, 4, 5] {
            let mut project = Project::new(vec![root.join(format!("{name}.php"))], vec![]);
            project.level = level;
            project.php_version = 80500;
            let result = analyse_project(&project).unwrap();
            let mut actual: Vec<_> = result
                .errors
                .into_iter()
                .map(|error| {
                    json!([
                        Path::new(&error.file)
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap(),
                        error.line,
                        error.identifier,
                        error.message,
                        error.tip,
                        error.can_be_ignored,
                    ])
                })
                .collect();
            actual.sort_by_key(Value::to_string);
            let mode = format!("{name}/{level}");
            let expected = oracle["errors"][&mode].as_array().unwrap();
            println!(
                "{}",
                json!({"mode": mode, "actual": actual, "oracle": expected,
                "general": result.general})
            );
            if actual != *expected || !result.general.is_empty() {
                failures.push(mode);
            }
        }
    }
    assert!(failures.is_empty(), "{}", json!(failures));
}
