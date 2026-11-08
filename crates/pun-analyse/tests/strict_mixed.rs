//! Public checked-copy controls from pinned PHPStan 2.3.1. No declaration or inference is made strict.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn selected(id: &str) -> bool {
    matches!(
        id,
        "method.nonObject"
            | "staticMethod.nonObject"
            | "property.nonObject"
            | "method.notFound"
            | "property.notFound"
            | "classConstant.notFound"
            | "staticProperty.nonObject"
            | "classConstant.nonObject"
            | "callable.nonCallable"
            | "foreach.nonIterable"
            | "clone.nonObject"
            | "preInc.type"
            | "method.nameNotString"
            | "property.nameNotString"
            | "classConstant.nameNotString"
            | "phpstan.dumpType"
            | "binaryOp.invalid"
            | "class.notFound"
            | "property.unresolvableType"
            | "argument.type"
    )
}

#[test]
fn checked_mixed_capabilities_templates_and_scope_metadata_match_phpstan() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/strict-mixed");
    let mut failures = Vec::new();
    for level in [2, 6, 7, 8, 9, 10] {
        let mut project = Project::new(vec![root.join("cases.php")], vec![]);
        project.level = level;
        project.php_version = 80400;
        project.options.set(
            pun_analyse::ProjectOptions::CHECK_NON_STRINGABLE_DYNAMIC_ACCESS,
            true,
        );
        let got: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .filter(|e| selected(&e.identifier))
            .map(|e| serde_json::json!([e.line, e.identifier, e.message, e.tip, e.can_be_ignored]))
            .collect();
        let capture: Value = serde_json::from_slice(
            &std::fs::read(root.join(format!("expected-{level}.json"))).unwrap(),
        )
        .unwrap();
        assert!(
            capture["errors"].as_array().unwrap().is_empty(),
            "oracle must have no run errors"
        );
        let want: Vec<_> = capture["files"]
            .as_object()
            .unwrap()
            .values()
            .flat_map(|f| f["messages"].as_array().unwrap())
            .filter(|e| selected(e["identifier"].as_str().unwrap()))
            .map(|e| {
                serde_json::json!([
                    e["line"],
                    e["identifier"],
                    e["message"],
                    e.get("tip").unwrap_or(&Value::Null),
                    e["ignorable"]
                ])
            })
            .collect();
        if got != want {
            failures.push(format!(
                "level {level}:\nreceived {}\nexpected {}",
                serde_json::to_string_pretty(&got).unwrap(),
                serde_json::to_string_pretty(&want).unwrap()
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
