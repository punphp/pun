//! Branches of the dead-code rules that PHPStan's own rule fixtures miss: NotAnalysedTraitRule (it stays quiet when
//! only files are analysed, as the level fixtures are), the bleedingEdge toggles (reportPreciseLineForUnusedFunction-
//! Parameter, unusedLabel, then all of bleedingEdge.neon) and the private members of traits. Expected: PHPStan's run on tests/fixtures/dead_code for
//! each phpstan*.neon there, restricted to the identifiers Pun reports.

use std::path::Path;

use pun_analyse::{Project, analyse, reports, run};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

type Key = (String, i64, String, String, Option<String>);

fn check(neon: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/dead_code");
    let dir = root.to_string_lossy().into_owned();
    let options = Options {
        config: Some(format!("{dir}/{neon}.neon")),
        ..Options::for_project(&dir)
    };
    let config = resolve(&options).unwrap_or_else(|e| panic!("{e}"));
    let prefix = format!("{dir}/");
    let mut got: Vec<Key> = analyse(&root, &config)
        .unwrap()
        .into_iter()
        .map(|e| {
            // compared without a trait's ` (in context of …)`: PHPStan's JSON names some errors' files without it
            let file = e.file.strip_prefix(&prefix).unwrap_or(&e.file);
            let file = file
                .split(" (in context of ")
                .next()
                .unwrap_or(file)
                .to_string();
            (file, e.line, e.identifier, e.message, e.tip)
        })
        .collect();
    got.sort();
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join(format!("expected-{neon}.json"))).unwrap())
            .unwrap();
    let mut want: Vec<Key> = expected["errors"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| reports(e["identifier"].as_str().unwrap_or("")))
        .map(|e| {
            (
                e["file"]
                    .as_str()
                    .unwrap()
                    .split(" (in context of ")
                    .next()
                    .unwrap()
                    .to_string(),
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
            )
        })
        .collect();
    want.sort();
    assert_eq!(got, want);
}

/// PHPStan 2.3.1 --level=4: skip terminated statements, but keep labels and declarations live.
#[test]
fn unreachable_walk_preserves_labels_and_independent_declarations() {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/dead_code/unreachable-walk.php");
    let mut project = Project::new(vec![path], vec![]);
    project.level = 4;
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|error| {
            (
                error.line,
                error.identifier,
                error.message,
                error.tip,
                error.can_be_ignored,
            )
        })
        .collect();
    got.sort();
    let unreachable = "Unreachable statement - code above always terminates.";
    let mut want: Vec<_> = [
        (6, "deadCode.unreachable", unreachable),
        (12, "deadCode.unreachable", unreachable),
        (14, "expr.resultUnused", "Expression \"'reachable'\" on a separate line does not do anything."),
        (20, "deadCode.unreachable", unreachable),
        (20, "function.inner", "Inner named functions are not supported by PHPStan. Consider refactoring to an anonymous function, class method, or a top-level-defined function. See issue #165 (https://github.com/phpstan/phpstan/issues/165) for more details."),
        (21, "expr.resultUnused", "Expression \"'nested-reachable'\" on a separate line does not do anything."),
        (23, "deadCode.unreachable", unreachable),
        (30, "expr.resultUnused", "Expression \"'early-bound-reachable'\" on a separate line does not do anything."),
        (35, "expr.resultUnused", "Expression \"'class-reachable'\" on a separate line does not do anything."),
        (39, "deadCode.unreachable", unreachable),
    ]
    .into_iter()
    .map(|(line, identifier, message)| (line, identifier.to_owned(), message.to_owned(), None, true))
    .collect();
    want.sort();
    assert_eq!(got, want);
}

#[test]
fn default_toggles() {
    check("phpstan");
}

#[test]
fn bleeding_edge_toggles() {
    check("phpstan-bleeding");
}

#[test]
fn bleeding_edge() {
    check("phpstan-edge");
}

/// Metadata for a nonexistent parameter is not an omitted optional callback. Keep all caller diagnostics,
/// including the absence of unused-result errors, from PHPStan's original no-named-arguments fixture.
#[test]
fn callback_purity_requires_a_flagged_parameter() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let expected: Value = serde_json::from_slice(
        &std::fs::read(root.join("levels/5/Functions/CallUserFuncRuleTest.json")).unwrap(),
    )
    .unwrap();
    let case = expected["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["method"] == "testNoNamedArguments")
        .unwrap();
    let data = root.join("rules/data");
    let paths = case["files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|path| data.join(path.as_str().unwrap()))
        .collect();
    let mut project = Project::new(paths, Vec::new());
    project.level = 5;
    let prefix = format!("{}/", data.display());
    let mut got: Vec<Key> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| {
            (
                e.file.strip_prefix(&prefix).unwrap_or(&e.file).to_string(),
                e.line,
                e.identifier,
                e.message,
                e.tip,
            )
        })
        .collect();
    let mut want: Vec<Key> = case["errors"]
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
            )
        })
        .collect();
    got.sort();
    want.sort();
    assert_eq!(got, want);
}

/// Native continuation survives inference's reference bookkeeping, not semantic reference writes.
#[test]
fn native_continuation_through_project_inference_matches_phpstan() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/dead_code");
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/dead_code/native-inference.json")).unwrap();
    let mut project = Project::new(vec![root.join("native-inference.php")], vec![]);
    project.level = 4;
    project.php_version = 80500;
    let mut actual: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|error| {
            (
                error.line,
                error.identifier,
                error.message,
                error.tip,
                error.can_be_ignored,
            )
        })
        .collect();
    actual.sort();
    assert_eq!(serde_json::to_value(actual).unwrap(), expected["errors"]);
}
