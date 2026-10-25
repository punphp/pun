//! Finite dynamic ClassConstantRule selections. Expectations are PHPStan 2.3.1's public analyse output,
//! including the upstream dynamic-access regression, without a gap exemption.

use std::path::Path;

use pun_analyse::{IDENTIFIERS, Project, analyse, run};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

type Key = (String, i64, String, String, Option<String>, bool);

fn keys(errors: Vec<pun_analyse::Error>, root: &Path) -> Vec<Key> {
    let prefix = format!("{}/", root.display());
    errors
        .into_iter()
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
        .collect()
}

fn expected(errors: &Value) -> Vec<Key> {
    let mut want: Vec<_> = errors
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| IDENTIFIERS.contains(&e["identifier"].as_str().unwrap_or("")))
        .map(|e| {
            (
                e["file"].as_str().unwrap().to_string(),
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
                e["ignorable"].as_bool().unwrap_or(true),
            )
        })
        .collect();
    // Public analyse output is ordered by file, line and message, independently of the rule walk order.
    want.sort_by(|a, b| (&a.0, a.1, &a.3).cmp(&(&b.0, b.1, &b.3)));
    want
}

fn check(neon: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/class_constants");
    let config = resolve(&Options {
        config: Some(
            root.join(format!("{neon}.neon"))
                .to_string_lossy()
                .into_owned(),
        ),
        ..Options::for_project(&root.to_string_lossy())
    })
    .unwrap();
    let want: Value =
        serde_json::from_slice(&std::fs::read(root.join(format!("expected-{neon}.json"))).unwrap())
            .unwrap();
    assert_eq!(
        keys(analyse(&root, &config).unwrap(), &root),
        expected(&want["errors"])
    );
}

#[test]
fn finite_names_visibility_guards_and_receivers() {
    check("phpstan");
}

#[test]
fn constant_checks_are_not_replaced_by_the_php_version_error() {
    check("phpstan-php82");
}

#[test]
fn upstream_dynamic_access() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rules");
    let data = root.join("data");
    let test: Value = serde_json::from_slice(
        &std::fs::read(root.join("Classes/ClassConstantRuleTest.json")).unwrap(),
    )
    .unwrap();
    let case = test["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["method"] == "testDynamicAccess")
        .unwrap();
    let files = case["files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| data.join(p.as_str().unwrap()))
        .collect();
    assert_eq!(
        keys(run(&Project::new(files, Vec::new())).unwrap(), &data),
        expected(&case["errors"])
    );
}
