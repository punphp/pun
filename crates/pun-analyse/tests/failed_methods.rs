//! Failed method results and ordinary unknown-class diagnostics, captured from PHPStan 2.3.1.

use std::path::Path;

use pun_analyse::{Project, analyse, run_mago};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

fn check(file: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/failed-methods");
    let dir = root.to_string_lossy().into_owned();
    let config = resolve(&Options {
        config: Some(format!("{dir}/phpstan.neon")),
        ..Options::for_project(&dir)
    })
    .unwrap();
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    assert_eq!(oracle["errors"], serde_json::json!([]));
    let mut want: Vec<_> = oracle["files"][file]["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|error| {
            (
                error["line"].as_i64().unwrap(),
                error["identifier"].as_str().unwrap().to_owned(),
                error["message"].as_str().unwrap().to_owned(),
                error["tip"].as_str().map(str::to_owned),
                error["ignorable"].as_bool().unwrap(),
            )
        })
        .collect();
    let mut got: Vec<_> = analyse(&root, &config)
        .unwrap()
        .into_iter()
        .filter(|error| Path::new(&error.file).file_name().unwrap() == file)
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
    want.sort();
    let missing: Vec<_> = want.iter().filter(|e| !got.contains(e)).collect();
    let extra: Vec<_> = got.iter().filter(|e| !want.contains(e)).collect();
    assert_eq!(got, want, "{file}: missing {missing:#?}\nextra {extra:#?}");
}

#[test]
fn ordinary_failed_method_results_and_unknown_receivers() {
    check("matrix.php");
}

#[test]
fn raw_relative_initializer_member_calls_keep_their_unknown_identities() {
    check("relative.php");
}

#[test]
fn failed_results_preserve_dynamic_nullable_union_magic_and_never_boundaries() {
    check("boundaries.php");
}

/// Compare the engine's `never` directly. Pun's separate `*NEVER*` display is not changed by this port.
#[test]
fn a_reflected_never_return_is_not_a_failed_lookup() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/failed-methods");
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    let mut project = Project::new(vec![root.join("never.php")], Vec::new());
    project.php_version = 80400;
    let got: Vec<_> = run_mago(&project, None)
        .unwrap()
        .dumped
        .into_iter()
        .map(|dump| (i64::from(dump.line), dump.type_id))
        .collect();
    let want: Vec<_> = oracle["files"]["never.php"]["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|dump| {
            (
                dump["line"].as_i64().unwrap(),
                dump["message"]
                    .as_str()
                    .unwrap()
                    .strip_prefix("Dumped type: ")
                    .unwrap()
                    .to_owned(),
            )
        })
        .collect();
    assert_eq!(got, want);
}
