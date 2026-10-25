//! PipeOperatorRule through the public analyser, using PHPStan 2.3.1 oracle diagnostics.

use std::path::Path;

use pun_analyse::{IDENTIFIERS, Project, run};
use serde_json::Value;

fn check(file: &str, level: u8) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/pipe-rule");
    let oracle: Value = serde_json::from_slice(
        &std::fs::read(root.join(format!("expected-level-{level}.json"))).unwrap(),
    )
    .unwrap();
    let mut project = Project::new(vec![root.join(file)], Vec::new());
    project.level = level;
    project.php_version = 80500;
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .filter(|e| e.identifier == "pipe.byRef")
        .map(|e| (e.line, e.message, e.tip, e.can_be_ignored))
        .collect();
    let mut want: Vec<_> = oracle[file]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
                e["ignorable"].as_bool().unwrap(),
            )
        })
        .collect();
    got.sort();
    want.sort();
    assert_eq!(got, want, "{file}, level {level}");
}

#[test]
fn upstream_pipe_rule() {
    for level in [0, 2] {
        check("upstream.php", level);
    }
}

#[test]
fn upstream_chained_pipes() {
    for level in [0, 2] {
        check("upstream-chain.php", level);
    }
}

#[test]
fn callable_shapes_variadics_and_rhs_lines() {
    check("callbacks.php", 2);
}

#[test]
fn invokable_objects_methods_and_visibility() {
    check("objects.php", 2);
}

#[test]
fn level_zero_and_one_check_this_only() {
    assert!(IDENTIFIERS.contains(&"pipe.byRef"));
    for level in [0, 1] {
        check("objects.php", level);
        check("callbacks.php", level);
    }
}

#[test]
fn union_filtering_nullability_and_unknown_classes() {
    for level in [2, 7, 8, 9, 10] {
        check("named-unions.php", level);
    }
}

#[test]
fn unnamed_callable_union_signatures() {
    for level in [2, 7, 8, 9, 10] {
        check("unions.php", level);
        check("callbacks.php", level);
    }
}
