//! ExistingClassInInstanceOfRule descriptions, captured from PHPStan 2.3.1.

use std::path::Path;

use pun_analyse::{Project, run};
use pun_phpstan::{Array, Key, Value as Parameter};
use serde_json::{Value, json};

#[test]
fn receiver_descriptions_match_complete_phpstan_records() {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/instanceof-trait-descriptions");
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/instanceof-trait-descriptions/expected-phpstan.json"
    ))
    .unwrap();
    let mut failures = Vec::new();
    for level in [0, 2] {
        let mut parameters = Array::new();
        parameters.set(Key::from("level"), Parameter::Int(level));
        parameters.set(Key::from("phpVersion"), Parameter::Int(80500));
        parameters.set(
            Key::from("checkClassCaseSensitivity"),
            Parameter::Bool(level >= 2),
        );
        let file = root.join("receivers.php");
        let mut configured = Project::from_config(&root, &parameters);
        configured.analysed = vec![file.clone()];
        let mut direct = Project::new(vec![file], Vec::new());
        direct.level = level as u8;
        direct.php_version = 80500;
        direct.options.set(pun_analyse::ProjectOptions::CHECK_CLASS_CASE_SENSITIVITY, level >= 2);
        let mut want: Vec<_> = oracle[level.to_string()]
            .as_array()
            .unwrap()
            .iter()
            .map(|record| {
                json!([
                    record["file"],
                    record["line"],
                    record["identifier"],
                    record["message"],
                    record["tip"],
                    record["ignorable"],
                ])
            })
            .collect();
        want.sort_by_key(Value::to_string);
        for (constructor, project) in [("from_config", configured), ("new", direct)] {
            let mut got: Vec<_> = run(&project)
                .unwrap()
                .into_iter()
                .map(|record| {
                    json!([
                        record.file.replace(root.to_str().unwrap(), "$ROOT"),
                        record.line,
                        record.identifier,
                        record.message,
                        record.tip,
                        record.can_be_ignored,
                    ])
                })
                .collect();
            got.sort_by_key(Value::to_string);
            if got != want {
                failures.push(json!({
                    "level": level,
                    "constructor": constructor,
                    "got": got,
                    "expected": want,
                }));
            }
        }
    }
    assert!(failures.is_empty(), "{}", json!(failures));
}
