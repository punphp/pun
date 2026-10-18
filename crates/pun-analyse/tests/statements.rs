//! Branches of the declaration, keyword, assignment and type rules that PHPStan's own rule fixtures miss (other
//! `phpVersion`s, declare values, trait aliases, interface constructors, by-reference arguments, destructuring
//! targets, use statements, promoted and hooked property types). Expected: PHPStan's run on tests/fixtures/statements for each
//! phpstan*.neon there, restricted to the identifiers Pun reports.

use std::path::Path;

use pun_analyse::{IDENTIFIERS, Project, analyse, run};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

type Key = (String, u32, String, String, Option<String>);

fn check(neon: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/statements");
    let dir = root.to_string_lossy().into_owned();
    let options = Options {
        config: Some(format!("{dir}/{neon}.neon")),
        ..Options::for_project(&dir)
    };
    let config = resolve(&options).unwrap_or_else(|e| panic!("{e}"));
    let prefix = format!("{dir}/");
    let mut got: Vec<Key> = analyse(&root, &config)
        .into_iter()
        .map(|e| {
            let file = e.file.strip_prefix(&prefix).unwrap_or(&e.file).to_string();
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
        .filter(|e| IDENTIFIERS.contains(&e["identifier"].as_str().unwrap_or("")))
        .map(|e| {
            (
                e["file"].as_str().unwrap().to_string(),
                e["line"].as_u64().unwrap() as u32,
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
            )
        })
        .collect();
    want.sort();
    assert_eq!(got, want);
}

#[test]
fn php_8_5() {
    check("phpstan");
}

/// The casts and the backtick are allowed before PHP 8.5, the (unset) cast before 8.0.
#[test]
fn php_7_4() {
    check("phpstan-php74");
}

/// VoidCastRuleTest::testSupport below PHP 8.5 (the oracle's PHP 8.4 cannot parse `(void)`, so the expectation is
/// the upstream test's).
#[test]
fn void_cast_before_php_8_5() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/rules/data/Cast/data/void-cast-support.php");
    let mut project = Project::new(vec![file], Vec::new());
    project.php_version = 80400;
    let got: Vec<_> = run(&project)
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message))
        .collect();
    assert_eq!(
        got,
        [(
            10,
            "cast.voidNotSupported".to_string(),
            "The (void) cast is supported only on PHP 8.5 and later.".to_string()
        )]
    );
}
