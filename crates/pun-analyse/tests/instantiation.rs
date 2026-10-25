//! InstantiationRule::getClassNames(): PHPStan 2.3.1's dynamic class names, reflection constraints and erased
//! unknown objects. Expectations are fresh oracle runs, plus the two upstream regressions this fixes.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use pun_analyse::{IDENTIFIERS, Project, analyse, run};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

type Key = (String, i64, String, String, Option<String>);

fn keys(errors: Vec<pun_analyse::Error>, root: &Path) -> BTreeSet<Key> {
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
            )
        })
        .collect()
}

fn expected(errors: &Value) -> BTreeSet<Key> {
    errors
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
            )
        })
        .collect()
}

#[test]
fn dynamic_class_reflection() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/instantiation");
    let options = Options {
        config: Some(root.join("phpstan.neon").to_string_lossy().into_owned()),
        ..Options::for_project(&root.to_string_lossy())
    };
    let config = resolve(&options).unwrap();
    let want: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    assert_eq!(
        keys(analyse(&root, &config).unwrap(), &root),
        expected(&want["errors"])
    );
}

/// No gap exemption: both upstream cases must match, including their other rules' existing diagnostics.
#[test]
fn unknown_object_operands() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rules");
    let data = root.join("data");
    let cases: Value = serde_json::from_slice(
        &std::fs::read(root.join("Classes/InstantiationRuleTest.json")).unwrap(),
    )
    .unwrap();
    for case in cases["cases"].as_array().unwrap().iter().filter(|c| {
        matches!(
            c["method"].as_str(),
            Some("testInstantiation" | "testBug4471")
        )
    }) {
        let paths = |key: &str| -> Vec<PathBuf> {
            case[key]
                .as_array()
                .unwrap()
                .iter()
                .map(|p| data.join(p.as_str().unwrap()))
                .collect()
        };
        assert_eq!(
            keys(
                run(&Project::new(paths("files"), paths("scan"))).unwrap(),
                &data
            ),
            expected(&case["errors"]),
            "{}",
            case["method"]
        );
    }
}
