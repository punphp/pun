//! Rector's own fixtures for the two ported rules, copied verbatim from rector-src 2.6.1 (`.oracle/rector`,
//! rules-tests/...) into tests/rector/<Rector class>/. A fixture is `input -----\n expected`, or only the input when
//! the rule must not change it (Rector's FixtureSplitter). Each one must pass as
//! `format(pun lint --fix) == format(expected)` (lint.md Build 1), with format being the laravel preset. The text edits
//! also reproduce Rector's bytes before format, which is checked too.

use std::path::{Path, PathBuf};

use pun_format::Pipeline;
use pun_lint::Linter;
use pun_lint::rules::{NoUnusedCatchVariable, PreferCompoundAssignment};

fn fixtures(dir: &str) -> Vec<(PathBuf, String, String)> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/rector")
        .join(dir);
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    paths.sort();
    assert!(!paths.is_empty(), "no fixtures in {}", dir.display());
    paths
        .into_iter()
        .map(|path| {
            let content = std::fs::read_to_string(&path)
                .unwrap()
                .replace("\r\n", "\n");
            let (input, expected) = match content.split_once("-----\n") {
                Some((i, e)) => (i.to_string(), e.to_string()),
                None => (content.clone(), content),
            };
            (path, input, expected)
        })
        .collect()
}

fn format(pipeline: &Pipeline, code: &str) -> String {
    match pipeline
        .run(Path::new("fixture.php"), code.as_bytes())
        .unwrap()
    {
        Some(fixed) => String::from_utf8(fixed.code).unwrap(),
        None => code.to_string(),
    }
}

fn laravel() -> Pipeline {
    let rules = pun_config::rule_set(pun_config::preset_rules("laravel").unwrap()).unwrap();
    let pipeline = Pipeline::new(&rules, &pun_fixers::all).unwrap();
    assert!(pipeline.not_ported.is_empty(), "{:?}", pipeline.not_ported);
    pipeline
}

fn run(linter: &Linter, dir: &str) {
    let pipeline = laravel();
    for (path, input, expected) in fixtures(dir) {
        let name = path.file_name().unwrap().to_string_lossy();
        let fixed = linter.fix(&path, &input).unwrap();
        assert_eq!(fixed.source, expected, "{name}: lint --fix");
        assert_eq!(
            format(&pipeline, &fixed.source),
            format(&pipeline, &expected),
            "{name}: format(lint --fix)"
        );
        if input != expected {
            // The rule did the work, not the formatter.
            assert_ne!(
                format(&pipeline, &input),
                format(&pipeline, &expected),
                "{name}"
            );
            assert!(!fixed.applied.is_empty(), "{name}");
        } else {
            assert!(fixed.applied.is_empty(), "{name}");
        }
    }
}

#[test]
fn remove_unused_variable_in_catch() {
    // config/configured_rule.php: phpVersion(PhpVersion::PHP_80).
    run(
        &Linter::new(vec![Box::new(NoUnusedCatchVariable)], 80000),
        "RemoveUnusedVariableInCatchRector/Fixture",
    );
}

#[test]
fn remove_unused_variable_in_catch_skipped_below_php_80() {
    // config/configured_rule_php_version_74.php: phpVersion(PhpVersion::PHP_74).
    run(
        &Linter::new(vec![Box::new(NoUnusedCatchVariable)], 70400),
        "RemoveUnusedVariableInCatchRector/FixtureSkipped",
    );
}

#[test]
fn combined_assign() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version.
    run(
        &Linter::new(vec![Box::new(PreferCompoundAssignment)], 80400),
        "CombinedAssignRector/Fixture",
    );
}
