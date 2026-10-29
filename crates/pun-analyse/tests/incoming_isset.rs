//! Incoming isset scopes must preserve definition certainty before and after loop and array joins.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

#[test]
fn native_isset_property_reflection_matches_phpstan() {
    check_property_reflection(
        "properties.php",
        include_str!("fixtures/incoming-isset/properties-phpstan.json"),
    );
}

#[test]
fn property_selector_expressions_and_excluded_subtypes_match_phpstan() {
    check_property_reflection(
        "property-selectors.php",
        include_str!("fixtures/incoming-isset/property-selectors-phpstan.json"),
    );
}

fn check_property_reflection(file: &str, capture: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/incoming-isset");
    let oracle: Value = serde_json::from_str(capture).unwrap();
    let mut failures = Vec::new();
    for level in [1, 4, 5] {
        let mut project = Project::new(vec![root.join(file)], Vec::new());
        project.level = level;
        project.php_version = 80500;
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
        let mut want: Vec<_> = oracle[level.to_string()]
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
                "level {level}: {} actual / {} expected complete records\nextra: {:?}\nmissing: {:?}",
                got.len(),
                want.len(),
                got.iter().filter(|e| !want.contains(e)).collect::<Vec<_>>(),
                want.iter().filter(|e| !got.contains(e)).collect::<Vec<_>>()
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn exported_terminating_isset_edge_makes_the_foreach_value_absent() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let mut project = Project::new(
        vec![fixtures.join("rules/data/DeadCode/data/unreachable.php")],
        Vec::new(),
    );
    project.level = 4;
    project.check_maybe_undefined_variables = true;
    let got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .filter(|error| error.identifier == "variable.undefined")
        .map(|error| (error.line, error.message))
        .collect();
    // PHPStan 2.3.1's fresh level-4 oracle, also saved in UnreachableStatementRuleTest::testRule.
    assert_eq!(got, [(74, "Undefined variable: $a".to_string())]);
}

#[test]
fn exported_counterfactual_coalesce_rhs_makes_the_value_absent() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let mut project = Project::new(
        vec![fixtures.join("rules/data/Comparison/data/bug-12087c.php")],
        Vec::new(),
    );
    project.level = 4;
    project.check_maybe_undefined_variables = true;
    let got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .filter(|error| error.identifier == "variable.undefined")
        .map(|error| (error.line, error.message))
        .collect();
    // PHPStan 2.3.1 analyzes the unreachable RHS in the counterfactual !isset scope.
    assert_eq!(got, [(23, "Undefined variable: $value".to_string())]);
}

#[test]
fn exported_addition_reads_are_certain_at_levels_five_and_nine() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    for level in [5, 9] {
        let expected: Value = serde_json::from_slice(
            &std::fs::read(fixtures.join(format!(
                "levels/{level}/Functions/CallToFunctionParametersRuleTest.json"
            )))
            .unwrap(),
        )
        .unwrap();
        let case = expected["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|case| case["method"] == "testFunctionWithNumericParameterThatCreatedByAddition")
            .unwrap();
        let paths = |key: &str| {
            case[key]
                .as_array()
                .unwrap()
                .iter()
                .map(|file| fixtures.join("rules/data").join(file.as_str().unwrap()))
                .collect()
        };
        let mut project = Project::new(paths("files"), paths("scan"));
        project.level = level;
        project.check_class_case_sensitivity = true;
        project.check_maybe_undefined_variables = true;
        let got: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .filter(|error| error.identifier == "variable.undefined")
            .collect();
        assert!(
            case["errors"]
                .as_array()
                .unwrap()
                .iter()
                .all(|error| error["identifier"] != "variable.undefined")
        );
        assert!(got.is_empty(), "level {level}: {got:?}");
    }
}

#[test]
fn incoming_isset_certainty_matches_phpstan() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/incoming-isset");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    for level in [4, 5, 9] {
        let mut project = Project::new(vec![root.join("flow.php")], Vec::new());
        project.level = level;
        project.php_version = 80400;
        project.check_maybe_undefined_variables = true;
        let mut got: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .filter(|error| {
                matches!(
                    error.identifier.as_str(),
                    "variable.undefined" | "phpstan.dumpType" | "isset.variable"
                )
            })
            .map(|error| (error.line, error.identifier, error.message))
            .collect();
        let mut want: Vec<_> = expected["files"]["flow.php"]["messages"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|error| {
                matches!(
                    error["identifier"].as_str(),
                    Some("variable.undefined" | "phpstan.dumpType" | "isset.variable")
                )
            })
            .map(|error| {
                (
                    error["line"].as_i64().unwrap(),
                    error["identifier"].as_str().unwrap().to_string(),
                    error["message"].as_str().unwrap().to_string(),
                )
            })
            .collect();
        got.sort();
        want.sort();
        assert_eq!(got, want, "level {level}");
    }
}

/// PHPStan 2.3.1, invocation.neon at each level: basic variables start at 1, advanced fetches at 4.
/// The upstream trait fixture includes error/no-error disagreement, agreement and class-independent checks.
#[test]
fn isset_invocation_contract_matches_phpstan() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/incoming-isset/invocation-phpstan.json"
    ))
    .unwrap();
    let prefix = format!("{}/", fixtures.display());
    for level in [0, 1, 4, 9] {
        let mut project = Project::new(
            vec![
                fixtures.join("incoming-isset/invocation.php"),
                fixtures.join("rules/data/Variables/data/isset-in-trait.php"),
            ],
            Vec::new(),
        );
        project.level = level;
        project.php_version = 80400;
        project.check_maybe_undefined_variables = true;
        let mut got: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .filter(|e| {
                e.file.ends_with("invocation.php")
                    || e.identifier.starts_with("isset.")
                    || e.identifier == "nullsafe.neverNull"
            })
            .map(|e| {
                serde_json::json!([
                    e.file.strip_prefix(&prefix).unwrap(),
                    e.line,
                    e.identifier,
                    e.message,
                    e.tip,
                    e.can_be_ignored
                ])
            })
            .collect();
        let capture = &expected[level.to_string()];
        assert!(capture["errors"].as_array().unwrap().is_empty());
        let mut want: Vec<_> = capture["files"]
            .as_object()
            .unwrap()
            .iter()
            .flat_map(|(file, result)| {
                result["messages"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(move |e| {
                        file.ends_with("invocation.php")
                            || e["identifier"].as_str().unwrap().starts_with("isset.")
                            || e["identifier"] == "nullsafe.neverNull"
                    })
                    .map(move |e| {
                        serde_json::json!([
                            file,
                            e["line"],
                            e["identifier"],
                            e["message"],
                            e.get("tip").unwrap_or(&Value::Null),
                            e["ignorable"]
                        ])
                    })
            })
            .collect();
        got.sort_by_cached_key(Value::to_string);
        want.sort_by_cached_key(Value::to_string);
        assert_eq!(got, want, "level {level}");
    }
}
