//! Constant-name extraction through the public analyser, from PHPStan 2.3.1's actual level-0 diagnostics.
//! PHP 8.4, with checkMaybeUndefinedVariables off and on; source cb3b7e99489b2e3789ee2eec10501cb7a72580ea.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(fixture: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/variable-names");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join(format!("{fixture}.json"))).unwrap())
            .unwrap();
    let mut failures = Vec::new();
    for (mode, maybe) in [("default", false), ("maybe", true)] {
        let mut params = pun_phpstan::Array::new();
        params.set("level".into(), pun_phpstan::Value::Int(0));
        params.set("phpVersion".into(), pun_phpstan::Value::Int(80400));
        params.set(
            "checkMaybeUndefinedVariables".into(),
            pun_phpstan::Value::Bool(maybe),
        );
        let mut project = Project::from_config(&root, &params);
        project.analysed = vec![root.join(format!("{fixture}.php"))];
        let got: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .filter(|e| e.identifier.starts_with("variable."))
            .map(|e| (e.line, e.identifier, e.message, e.tip, e.can_be_ignored))
            .collect();
        let mut want: Vec<_> = expected[mode]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| {
                (
                    e["line"].as_i64().unwrap(),
                    e["identifier"].as_str().unwrap().to_string(),
                    e["message"].as_str().unwrap().to_string(),
                    e["tip"].as_str().map(str::to_string),
                    e["ignorable"].as_bool().unwrap(),
                )
            })
            .collect();
        want.sort_by(|a, b| (a.0, &a.2).cmp(&(b.0, &b.2)));
        if got != want {
            failures.push(format!(
                "{fixture}/{mode}\nactual: {got:?}\nexpected: {want:?}"
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

#[test]
fn literal_bounded_templates_and_copies_keep_their_constant_names() {
    check("templates");
}

#[test]
fn late_types_and_nonconstant_unions_do_not_invent_names() {
    check("late");
}

#[test]
fn template_union_source_keeps_each_distinct_constant_type() {
    // Keep the real oracle regression active: conversion currently combines the union before
    // applying argument strategy, erasing its template member. The rule cannot recover that source.
    check("template-union-source");
}
