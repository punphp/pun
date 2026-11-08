//! Complete PHPStan 2.3.5 records for compound method reflections before call-variant selection.
//! Reproduce with fixtures/named-method-reflections/capture.php and the pinned PHPStan PHAR.

use std::path::Path;

use pun_analyse::{Error, Project, run};
use serde_json::Value;

#[test]
fn intersection_declaring_method_uses_every_original_variant() {
    check("intersection", 5);
}

#[test]
fn union_binding_uses_the_first_declaring_class_not_combined_builtin_provenance() {
    check("union", 5);
}

#[test]
fn intersection_collectors_remain_disabled_at_level_four() {
    check("intersection", 4);
}

#[test]
fn union_collectors_remain_disabled_at_level_four() {
    check("union", 4);
}

#[test]
fn ordinary_compound_binding_and_mixin_providers() {
    for level in [4, 5, 8] {
        check("structural", level);
    }
}

#[test]
fn compound_structure_precedes_independent_argument_switches() {
    for switches in ["TT", "TF", "FT", "FF"] {
        check_with("structural", 5, Some(switches));
    }
}

fn check(file: &str, level: u8) {
    check_with(file, level, None);
}

fn check_with(file: &str, level: u8, switches: Option<&str>) {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/named-method-reflections");
    let mut project = Project::new(vec![root.join(format!("{file}.php"))], vec![]);
    project.level = level;
    project.php_version = 80500;
    if let Some(switches) = switches {
        project.check_function_argument_types = Some(switches.as_bytes()[0] == b'T');
        project.check_arguments_passed_by_reference = Some(switches.as_bytes()[1] == b'T');
    }
    let mut got = run(&project).unwrap();
    for error in &mut got {
        error.file = error.file.replace(root.to_str().unwrap(), "$ROOT");
    }
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/named-method-reflections/expected.json"
    ))
    .unwrap();
    let mode = format!(
        "{file}-{level}{}",
        switches.map(|s| format!("-{s}")).unwrap_or_default()
    );
    let mut want: Vec<_> = oracle[&mode]
        .as_array()
        .unwrap()
        .iter()
        .map(|error| Error {
            file: error["file"].as_str().unwrap().into(),
            line: error["line"].as_i64().unwrap(),
            identifier: error["identifier"].as_str().unwrap().into(),
            message: error["message"].as_str().unwrap().into(),
            tip: error["tip"].as_str().map(str::to_owned),
            can_be_ignored: error["ignorable"].as_bool().unwrap(),
            fixable: error["fixable"].as_bool().unwrap_or(false),
        })
        .collect();
    got.sort();
    want.sort();
    assert!(
        got == want,
        "{mode}\nextra: {:#?}\nmissing: {:#?}",
        got.iter()
            .filter(|error| !want.contains(error))
            .collect::<Vec<_>>(),
        want.iter()
            .filter(|error| !got.contains(error))
            .collect::<Vec<_>>()
    );
}
