//! Reflected declaring-class arguments, with complete independent PHPStan 2.3.1 records.

use std::path::Path;

use pun_analyse::{Project, run};
use pun_phpstan::{Array, Key, Value as Parameter};
use serde_json::{Value, json};

#[test]
fn generic_declaring_property_descriptions_and_writes_match_phpstan() {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/generic-property-description");
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/generic-property-description/expected.json"
    ))
    .unwrap();
    let mut failures = Vec::new();
    for (case, messages) in oracle.as_object().unwrap() {
        let (file, level) = case.split_once(':').unwrap();
        let level: u8 = level.parse().unwrap();
        let mut parameters = Array::new();
        parameters.set(Key::from("level"), Parameter::Int(i64::from(level)));
        parameters.set(Key::from("phpVersion"), Parameter::Int(80500));
        let mut configured = Project::from_config(&root, &parameters);
        configured.analysed = vec![root.join(file)];
        let mut direct = Project::new(vec![root.join(file)], vec![]);
        direct.level = level;
        let mut want: Vec<_> = messages
            .as_array()
            .unwrap()
            .iter()
            .map(|e| {
                json!([
                    e["file"],
                    e["line"],
                    e["identifier"],
                    e["message"],
                    e.get("tip").unwrap_or(&Value::Null),
                    e["ignorable"]
                ])
            })
            .collect();
        want.sort_by_cached_key(Value::to_string);
        for (constructor, mut project) in [("from_config", configured), ("new", direct)] {
            project.php_version = 80500;
            let mut got: Vec<_> = run(&project)
                .unwrap()
                .into_iter()
                .map(|e| {
                    json!([
                        e.file.replace(root.to_str().unwrap(), "$ROOT"),
                        e.line,
                        e.identifier,
                        e.message,
                        e.tip,
                        e.can_be_ignored
                    ])
                })
                .collect();
            got.sort_by_cached_key(Value::to_string);
            println!(
                "{case}/{constructor}: {}",
                json!({"got": got, "want": want})
            );
            if got != want {
                failures.push(format!("{case}/{constructor}"));
            }
        }
    }
    assert!(failures.is_empty(), "mismatches: {}", failures.join(", "));
}
