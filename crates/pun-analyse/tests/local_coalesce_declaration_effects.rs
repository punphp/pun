//! Local ??= declaration effects through both public Project constructors.

use std::path::Path;

use pun_analyse::{Project, run};
use pun_phpstan::{Array, Key, Value as Parameter};
use serde_json::{Value, json};

#[test]
fn local_coalesce_effects_match_complete_phpstan_records() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/local-coalesce-declaration-effects");
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/local-coalesce-declaration-effects/expected.json"
    ))
    .unwrap();
    let mut failures = Vec::new();
    for level in [0, 1, 2, 4] {
        let file = root.join("effects.php");
        let mut parameters = Array::new();
        parameters.set(Key::from("level"), Parameter::Int(level));
        parameters.set(Key::from("phpVersion"), Parameter::Int(80500));
        let mut configured = Project::from_config(&root, &parameters);
        configured.analysed = vec![file.clone()];
        let direct = Project::new(vec![file], Vec::new());
        for (constructor, mut project) in [("from_config", configured), ("new", direct)] {
            project.level = level as u8;
            project.php_version = 80500;
            project.options.set(pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES, level > 0);
            let mut actual: Vec<_> = run(&project)
                .unwrap()
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
            actual.sort_by_cached_key(Value::to_string);
            let mut want = expected[level.to_string()].as_array().unwrap().clone();
            want.sort_by_cached_key(Value::to_string);
            if actual != want {
                failures.push(json!({
                    "level": level, "constructor": constructor,
                    "extra": actual.iter().filter(|record| !want.contains(record)).collect::<Vec<_>>(),
                    "missing": want.iter().filter(|record| !actual.contains(record)).collect::<Vec<_>>(),
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
