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
fn expression_class_constant_php_versions() {
    check("receivers-php74");
}

#[test]
fn inferred_expression_receivers_at_each_rule_level() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/class_constants");
    for level in [0, 1, 2, 7, 8, 9, 10] {
        let config = resolve(&Options {
            config: Some(root.join("receivers.neon").to_string_lossy().into_owned()),
            level: Some(level.to_string()),
            ..Options::for_project(&root.to_string_lossy())
        })
        .unwrap();
        let want: Value = serde_json::from_slice(
            &std::fs::read(root.join(format!("expected-receivers-{level}.json"))).unwrap(),
        )
        .unwrap();
        let relevant = |e: &Key| {
            [
                "classConstant.notFound",
                "classConstant.private",
                "classConstant.protected",
                "classConstant.nonObject",
                "classConstant.dynamicString",
                "classConstant.notSupported",
                "class.notFound",
            ]
            .contains(&e.2.as_str())
        };
        let mut project = Project::from_config(&root, &config.parameters);
        project.level = level;
        let got: Vec<_> = keys(run(&project).unwrap(), &root)
            .into_iter()
            .filter(relevant)
            .collect();
        let want: Vec<_> = expected(&want["errors"])
            .into_iter()
            .filter(relevant)
            .collect();
        assert_eq!(got, want, "level {level}");
    }
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

#[test]
fn expression_type_guards_and_invalidation_at_each_rule_level() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/class_constants");
    let mut differences = Vec::new();
    for level in [0, 1, 2, 7, 8, 9, 10] {
        let config = resolve(&Options {
            config: Some(root.join("expression-guards.neon").to_string_lossy().into_owned()),
            level: Some(level.to_string()),
            ..Options::for_project(&root.to_string_lossy())
        })
        .unwrap();
        let want: Value = serde_json::from_slice(
            &std::fs::read(root.join(format!("expected-expression-guards-{level}.json"))).unwrap(),
        )
        .unwrap();
        let relevant = |e: &Key| e.2.starts_with("classConstant.") || e.2 == "class.notFound";
        let mut project = Project::from_config(&root, &config.parameters);
        project.level = level;
        let got: Vec<_> = keys(run(&project).unwrap(), &root).into_iter().filter(relevant).collect();
        let want: Vec<_> = expected(&want["errors"]).into_iter().filter(relevant).collect();
        if got != want {
            let mut missing = want;
            let mut extra = Vec::new();
            for error in got {
                if let Some(i) = missing.iter().position(|e| *e == error) {
                    missing.remove(i);
                } else {
                    extra.push(error);
                }
            }
            differences.push(format!("level {level}: extra {extra:#?}, missing {missing:#?}"));
        }
    }
    assert!(differences.is_empty(), "{}", differences.join("\n"));
}
