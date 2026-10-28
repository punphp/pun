//! Complete PHPStan 2.3.1 diagnostics for independently configured property receiver eligibility.
//! Reproduce with fixtures/property-receiver-settings/capture.php and the pinned PHAR.

use std::path::Path;

use pun_analyse::{Project, run};
use pun_phpstan::{Array, Key, Value};

fn check(file: &str) {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/property-receiver-settings");
    let oracle: serde_json::Value = serde_json::from_str(include_str!(
        "fixtures/property-receiver-settings/expected.json"
    ))
    .unwrap();
    let mut failures = Vec::new();
    for level in [0, 5] {
        for (mode, this_only) in [
            ("default", None),
            ("this", Some(true)),
            ("all", Some(false)),
        ] {
            let case = format!("{file}:{level}-{mode}");
            let mut parameters = Array::new();
            parameters.set(Key::from("level"), Value::Int(i64::from(level)));
            if let Some(value) = this_only {
                parameters.set(Key::from("checkThisOnly"), Value::Bool(value));
            }
            let mut configured = Project::from_config(&root, &parameters);
            configured.analysed = vec![root.join(file)];
            let mut direct = Project::new(vec![root.join(file)], vec![]);
            direct.level = level;
            direct.check_this_only = this_only;
            let mut want: Vec<_> = oracle[&case]
                .as_array()
                .unwrap()
                .iter()
                .map(|e| {
                    (
                        e["file"].as_str().unwrap().to_string(),
                        e["line"].as_i64().unwrap(),
                        e["identifier"].as_str().unwrap().to_string(),
                        e["message"].as_str().unwrap().to_string(),
                        e["tip"].as_str().map(str::to_string),
                        e["ignorable"].as_bool().unwrap(),
                    )
                })
                .collect();
            want.sort();
            for (constructor, mut project) in
                [("from_config", configured), ("new_then_level", direct)]
            {
                project.php_version = 80500;
                let mut got: Vec<_> = run(&project)
                    .unwrap()
                    .into_iter()
                    .map(|e| {
                        (
                            e.file.replace(root.to_str().unwrap(), "$ROOT"),
                            e.line,
                            e.identifier,
                            e.message,
                            e.tip,
                            e.can_be_ignored,
                        )
                    })
                    .collect();
                got.sort();
                if got != want {
                    failures.push(format!(
                        "{case}/{constructor}\n  extra: {:?}\n  missing: {:?}",
                        got.iter().filter(|e| !want.contains(e)).collect::<Vec<_>>(),
                        want.iter().filter(|e| !got.contains(e)).collect::<Vec<_>>()
                    ));
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn this_copy_parenthesized_nullsafe_and_dynamic_class_receivers_follow_check_this_only() {
    check("receivers.php");
}

#[test]
fn named_classes_and_later_property_type_rules_keep_their_independent_boundaries() {
    check("controls.php");
}
