//! Complete offset-unset effects after the shared producer has updated the selected value.

use pun_analyse::{Project, analyse_project};
use serde_json::{Value, json};

#[test]
fn offset_unset_declaration_effects_match_complete_phpstan_records() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/offset-unset-declaration-effects");
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/offset-unset-declaration-effects/expected.json"
    ))
    .unwrap();
    let mut failures = Vec::new();
    for file in ["values", "objects", "unknown"] {
        for level in [0, 4] {
            let mut project = Project::new(vec![root.join(format!("{file}.php"))], Vec::new());
            project.php_version = 80500;
            project.level = level;
            project.check_maybe_undefined_variables = level > 0;
            let analysis = analyse_project(&project).unwrap();
            assert!(analysis.general.is_empty(), "{:?}", analysis.general);
            let mut actual: Vec<_> = analysis
                .errors
                .into_iter()
                .map(|error| {
                    json!([
                        std::path::Path::new(&error.file)
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
            actual.sort_by_cached_key(Value::to_string);
            let mut want = expected[file][level.to_string()]
                .as_array()
                .unwrap()
                .clone();
            want.sort_by_cached_key(Value::to_string);
            if actual != want {
                failures.push(json!({"file": file, "level": level,
                    "missing": want.iter().filter(|record| !actual.contains(record)).collect::<Vec<_>>(),
                    "extra": actual.iter().filter(|record| !want.contains(record)).collect::<Vec<_>>(),
                }));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}
