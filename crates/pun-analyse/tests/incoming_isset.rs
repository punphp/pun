//! Incoming isset scopes must preserve definition certainty before and after loop and array joins.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

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
                    "variable.undefined" | "phpstan.dumpType"
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
                    Some("variable.undefined" | "phpstan.dumpType")
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
