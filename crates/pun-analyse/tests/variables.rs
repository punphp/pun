//! DefinedVariableRule and CompactVariablesRule through the public analyser. The expectations are PHPStan
//! 2.3.1's exported level-0 diagnostics, not Mago's native undefined-variable issues.
//! The custom oracle runs use PHP's `-d phpstan.restarted=1 -d register_argc_argv=0|1` and PHPStan's `--debug`:
//! the turbo restart otherwise drops the INI setting that supplies cliArgumentsVariablesRegistered.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

#[test]
fn variable_contexts_and_configuration_match_phpstan() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("variables.json")).unwrap()).unwrap();
    let mut failures = Vec::new();
    for (name, level, cli, maybe, non_stringable) in [
        ("level0", 0, true, false, false),
        ("edge", 2, false, true, true),
        ("maybe", 0, true, true, false),
        ("quiet", 2, false, false, true),
    ] {
        let mut params = pun_phpstan::Array::new();
        params.set("level".into(), pun_phpstan::Value::Int(level));
        params.set(
            "cliArgumentsVariablesRegistered".into(),
            pun_phpstan::Value::Bool(cli),
        );
        params.set(
            "checkMaybeUndefinedVariables".into(),
            pun_phpstan::Value::Bool(maybe),
        );
        let mut project = Project::from_config(&root, &params);
        project.analysed = vec![root.join("variables.php")];
        project.check_non_stringable_dynamic_access = non_stringable;
        let mut got: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .filter(|e| e.identifier.starts_with("variable."))
            .map(|e| (e.line, e.identifier, e.message))
            .collect();
        let mut want: Vec<_> = expected[name]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| {
                (
                    e[0].as_i64().unwrap(),
                    e[1].as_str().unwrap().to_string(),
                    e[2].as_str().unwrap().to_string(),
                )
            })
            .collect();
        got.sort();
        want.sort();
        if got != want {
            failures.push(format!("{name}\n got {got:?}\nwant {want:?}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn undefined_variables_match_phpstan() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rules");
    let mut failures = Vec::new();
    for rule in ["CompactVariablesRuleTest", "DefinedVariableRuleTest"] {
        let test: Value = serde_json::from_slice(
            &std::fs::read(root.join(format!("Variables/{rule}.json"))).unwrap(),
        )
        .unwrap();
        for case in test["cases"].as_array().unwrap() {
            let files = case["files"]
                .as_array()
                .unwrap()
                .iter()
                .map(|p| root.join("data").join(p.as_str().unwrap()))
                .collect();
            let mut got: Vec<_> = run(&Project::new(files, Vec::new()))
                .unwrap()
                .into_iter()
                .filter(|e| e.identifier.starts_with("variable."))
                .map(|e| (e.line, e.identifier, e.message))
                .collect();
            let mut want: Vec<_> = case["errors"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|e| e["identifier"].as_str().unwrap().starts_with("variable."))
                .map(|e| {
                    (
                        e["line"].as_i64().unwrap(),
                        e["identifier"].as_str().unwrap().to_string(),
                        e["message"].as_str().unwrap().to_string(),
                    )
                })
                .collect();
            got.sort();
            want.sort();
            if got != want {
                failures.push(format!(
                    "{rule}::{}\n got {got:?}\nwant {want:?}",
                    case["method"]
                ));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
