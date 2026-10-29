//! IssetCheck's virtual markers come from real incoming constructor/helper scopes.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

#[test]
fn property_initialization_consumers_match_complete_phpstan_records() {
    check_property_initialization_consumers(
        "property-initialization-consumers.php",
        include_str!(
            "fixtures/property-initialization-bridge/property-initialization-consumers-phpstan.json"
        ),
    );
}

#[test]
fn called_method_summaries_reach_public_property_consumers() {
    check_property_initialization_consumers(
        "called-methods-consumers.php",
        include_str!(
            "fixtures/property-initialization-bridge/called-methods-consumers-phpstan.json"
        ),
    );
}

#[test]
fn called_method_compound_consumers_match_complete_phpstan_records() {
    check_property_initialization_consumers(
        "called-methods-compound.php",
        include_str!(
            "fixtures/property-initialization-bridge/called-methods-compound-phpstan.json"
        ),
    );
}

fn check_property_initialization_consumers(file: &str, capture: &str) {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/property-initialization-bridge");
    let expected: Value = serde_json::from_str(capture).unwrap();
    let mut failures = Vec::new();
    for (case, records) in expected.as_object().unwrap() {
        let (level, mode) = case.split_once('-').unwrap();
        let mut project = Project::new(vec![root.join(file)], Vec::new());
        project.level = level.parse().unwrap();
        project.php_version = 80500;
        if mode != "default" {
            project.check_advanced_isset = Some(mode == "on");
        }
        let mut got: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .map(|e| {
                serde_json::json!([
                    e.file.replace(root.to_str().unwrap(), "$ROOT"),
                    e.line,
                    e.identifier,
                    e.message,
                    e.tip,
                    e.can_be_ignored
                ])
            })
            .collect();
        let mut want: Vec<_> = records
            .as_array()
            .unwrap()
            .iter()
            .map(|e| {
                serde_json::json!([
                    e["file"],
                    e["line"],
                    e["identifier"],
                    e["message"],
                    e.get("tip").unwrap_or(&Value::Null),
                    e["ignorable"]
                ])
            })
            .collect();
        got.sort_by_cached_key(Value::to_string);
        want.sort_by_cached_key(Value::to_string);
        if got != want {
            failures.push(format!(
                "{case}: {} actual / {} expected complete records\nextra: {:?}\nmissing: {:?}",
                got.len(),
                want.len(),
                got.iter().filter(|e| !want.contains(e)).collect::<Vec<_>>(),
                want.iter().filter(|e| !got.contains(e)).collect::<Vec<_>>()
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
