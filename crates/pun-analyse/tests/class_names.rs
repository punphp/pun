//! ClassNameCheck (ClassCaseSensitivityCheck and ClassForbiddenNameCheck) at every caller Pun has, on names PHPStan's
//! own rule fixtures miss: `use` aliases, PHPStan's stubbed core attributes, a prefix without a namespace, the
//! `__callStatic()` drop, and the `checkClassCaseSensitivity` callers with the parameter on. Expected: PHPStan's runs on
//! tests/fixtures/class_names with phpstan.neon (level 0) and phpstan-level2.neon (level 2).

use std::path::Path;

use pun_analyse::{IDENTIFIERS, Project, analyse, run};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

type Key = (String, i64, String, String, Option<String>);

const CLASS_NAME_CHECK: &[&str] = &[
    "class.nameCase",
    "interface.nameCase",
    "trait.nameCase",
    "enum.nameCase",
    "class.prefixed",
];

fn root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/class_names")
}

fn keys(errors: Vec<pun_analyse::Error>, keep: &dyn Fn(&str) -> bool) -> Vec<Key> {
    let prefix = format!("{}/", root().display());
    let mut got: Vec<Key> = errors
        .into_iter()
        .filter(|e| keep(&e.identifier))
        .map(|e| {
            let file = e.file.strip_prefix(&prefix).unwrap_or(&e.file).to_string();
            (file, e.line, e.identifier, e.message, e.tip)
        })
        .collect();
    got.sort();
    got
}

fn expected(neon: &str, keep: &dyn Fn(&str) -> bool) -> Vec<Key> {
    let expected: Value = serde_json::from_slice(
        &std::fs::read(root().join(format!("expected-{neon}.json"))).unwrap(),
    )
    .unwrap();
    let mut want: Vec<Key> = expected["errors"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| keep(e["identifier"].as_str().unwrap_or("")))
        .map(|e| {
            (
                e["file"].as_str().unwrap().to_string(),
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
            )
        })
        .collect();
    want.sort();
    want
}

fn config(neon: &str) -> pun_phpstan::config::Resolved {
    let dir = root().to_string_lossy().into_owned();
    let options = Options {
        config: Some(format!("{dir}/{neon}.neon")),
        ..Options::for_project(&dir)
    };
    resolve(&options).unwrap_or_else(|e| panic!("{e}"))
}

/// Level 0: the case checks of FunctionDefinitionCheck, ExistingClassInInstanceOfRule, CaughtExceptionExistenceRule and
/// ExistingClassesInPropertiesRule are off; the prefix check and the other callers' case checks are on.
#[test]
fn level_0() {
    let keep = |i: &str| IDENTIFIERS.contains(&i);
    let got = keys(analyse(&root(), &config("phpstan")).unwrap(), &keep);
    assert_eq!(got, expected("phpstan", &keep));
}

/// `checkClassCaseSensitivity` on (level 2) turns those callers' case checks on.
#[test]
fn check_class_case_sensitivity() {
    let keep = |i: &str| CLASS_NAME_CHECK.contains(&i);
    let project = Project {
        check_class_case_sensitivity: true,
        ..Project::from_config(&root(), &config("phpstan-level2").parameters)
    };
    let got = keys(run(&project).unwrap(), &keep);
    let mut want = expected("phpstan-level2", &keep);
    // `Magic::missing()` keeps its case error at level 2 only because reportMagicMethods is on there too
    want.retain(|k| !k.3.starts_with("Class ClassNames\\Magic "));
    assert_eq!(got, want);
}

/// Captured from PHPStan 2.3.1, restricted to ClassNameCheck identifiers. The same inputs exercise attributes,
/// instantiation, inheritance, trait use, signatures, instanceof, catches, constants, static calls and callables.
fn case_options(neon: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/class-case-options");
    let dir = root.to_string_lossy().into_owned();
    let config = resolve(&Options {
        config: Some(format!("{dir}/phpstan-{neon}.neon")),
        ..Options::for_project(&dir)
    })
    .unwrap();
    let expected: Value = serde_json::from_slice(
        &std::fs::read(root.join(format!("expected-{neon}.json"))).unwrap(),
    )
    .unwrap();
    assert!(expected["general"].as_array().unwrap().is_empty());
    let prefix = format!("{dir}/");
    let mut got: Vec<_> = analyse(&root, &config)
        .unwrap()
        .into_iter()
        .filter(|e| CLASS_NAME_CHECK.contains(&e.identifier.as_str()))
        .map(|e| {
            (
                e.file.strip_prefix(&prefix).unwrap_or(&e.file).to_string(),
                e.line,
                e.identifier,
                e.message,
                e.tip,
                e.can_be_ignored,
            )
        })
        .collect();
    let mut want: Vec<_> = expected["errors"]
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
    got.sort();
    want.sort();
    assert_eq!(got, want, "{neon}");
}

#[test]
fn case_options_default_and_explicit_off() {
    case_options("default");
    case_options("off");
}

#[test]
fn case_options_imported_names() {
    case_options("imported");
}

#[test]
fn case_options_internal_names() {
    case_options("internal");
}

#[test]
fn case_options_both() {
    case_options("both");
}

#[test]
fn case_options_bleeding_edge_and_override() {
    case_options("edge");
    case_options("edge-off");
}
