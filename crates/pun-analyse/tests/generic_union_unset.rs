//! Complete PHPStan records for generic-union unset values, not declaration-effect substitutes.

use pun_analyse::{Project, analyse_project};
use serde_json::{Value, json};

#[test]
fn generic_union_unset_matches_complete_phpstan_records() {
    let root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/generic-union-unset");
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/generic-union-unset/expected.json")).unwrap();
    let mut failures = Vec::new();
    for (file, levels) in [
        ("virtual", [0, 4].as_slice()),
        ("values", [0].as_slice()),
        ("scopes", [0].as_slice()),
        ("objects", [0].as_slice()),
        ("members", [0].as_slice()),
    ] {
        for &level in levels {
            let mut project = Project::new(vec![root.join(format!("{file}.php"))], Vec::new());
            project.php_version = 80500;
            project.level = level;
            project.options.set(
                pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES,
                level > 0,
            );
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
            let levels = &expected[file];
            let records = if levels.is_array() {
                &levels[usize::from(level)]
            } else {
                &levels[level.to_string()]
            };
            let mut want = records.as_array().unwrap().clone();
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
