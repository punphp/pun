//! Complete PHPStan records for converter inputs, through both public Project constructors.

use std::path::Path;

use pun_analyse::{Project, analyse_project};
use pun_phpstan::{Array, Key, Value as Parameter};
use serde_json::{Value, json};

#[test]
fn callback_dependencies_ignore_extra_positional_and_unknown_named_arguments() {
    let file = "callbacks.php";
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/converter-remainder");
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/converter-remainder/expected.json")).unwrap();
    let mut failures = Vec::new();
    for level in [0, 5] {
        let mut parameters = Array::new();
        parameters.set(Key::from("level"), Parameter::Int(level));
        parameters.set(Key::from("phpVersion"), Parameter::Int(80500));
        let mut configured = Project::from_config(&root, &parameters);
        configured.analysed = vec![root.join(file)];
        let mut direct = Project::new(configured.analysed.clone(), Vec::new());
        direct.level = level as u8;
        direct.php_version = 80500;
        let mut expected = oracle["records"][format!("{file}/{level}")]
            .as_array()
            .unwrap()
            .clone();
        expected.sort_by_cached_key(Value::to_string);
        for (constructor, project) in [("new", direct), ("from_config", configured)] {
            let result = analyse_project(&project).unwrap();
            assert!(result.general.is_empty(), "{:?}", result.general);
            let mut actual = result
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
                .collect::<Vec<_>>();
            actual.sort_by_cached_key(Value::to_string);
            println!(
                "records={}",
                json!({"file": file, "level": level, "constructor": constructor, "actual": actual, "oracle": expected})
            );
            if actual != expected {
                failures.push(format!("{file}/{level}/{constructor}"));
            }
        }
    }
    assert!(failures.is_empty(), "{failures:?}");
}
