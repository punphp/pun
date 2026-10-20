//! Branches of FunctionDefinitionCheck and ExistingClassesInPropertiesRule that PHPStan's own rule fixtures miss:
//! other `phpVersion`s, template bounds and defaults against native types, PHPDoc tag prefixes, `@phpstan-self-out`
//! names, `parent` without a parent, implicitly nullable and required-after-optional variants, which intersections
//! are unresolvable, and property hooks. Expected: PHPStan's run on tests/fixtures/signatures for each
//! phpstan*.neon there, restricted to the identifiers Pun reports.

use std::path::Path;

use pun_analyse::{IDENTIFIERS, analyse};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

type Key = (String, i64, String, String, Option<String>);

fn check(neon: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/signatures");
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

#[test]
fn php_8_5() {
    check("phpstan");
}

/// Native union types need PHP 8.0; required parameters after optional ones are deprecated from 8.0 on.
#[test]
fn php_7_4() {
    check("phpstan-php74");
}

/// A null default makes a parameter optional only from PHP 8.1 on; intersections are checked from 8.1 on.
#[test]
fn php_8_0() {
    check("phpstan-php80");
}

/// `?int $x = null` is optional from 8.1, `int|null $x = null` from 8.3; arrow functions return `never` from 8.2.
#[test]
fn php_8_1() {
    check("phpstan-php81");
}

/// Implicitly nullable parameters are deprecated from PHP 8.4.
#[test]
fn php_8_3() {
    check("phpstan-php83");
}
