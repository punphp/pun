//! php.rs against real PHP and PHPStan 2.2.7 (tests/fixtures/php.json, scripts/export-php-helpers-fixtures.php).

mod common;

use common::fixture;
use pun_phpstan::php;
use serde_json::{Value as J, json};

fn s(j: &J) -> &str {
    j.as_str().unwrap()
}

#[test]
fn php_helpers() {
    let f = fixture("php.json");
    let mut failures = Vec::new();
    let mut check = |what: &str, input: &J, want: J, got: J| {
        if want != got {
            failures.push(format!("{what} {input}: want {want}, got {got}"));
        }
    };
    for c in f["regex"].as_array().unwrap() {
        // ponytail (php.rs): the D modifier is not emulated.
        if s(&c["pattern"]).ends_with("/D") {
            continue;
        }
        let got = match php::preg(s(&c["pattern"])) {
            Ok(re) => json!({ "matched": php::is_match(&re, s(&c["subject"])) }),
            Err(e) => json!({ "error": e }),
        };
        let want = match c.get("error") {
            Some(e) => json!({ "error": e }),
            None => json!({ "matched": c["matched"] }),
        };
        check("regex", c, want, got);
    }
    for c in f["quote"].as_array().unwrap() {
        let d = c["delimiter"].as_str().map(|d| d.chars().next().unwrap());
        check(
            "preg_quote",
            c,
            c["quoted"].clone(),
            json!(php::preg_quote(s(&c["s"]), d)),
        );
    }
    for c in f["normalize"].as_array().unwrap() {
        check(
            "normalizePath",
            c,
            c["normalized"].clone(),
            json!(php::normalize_path(s(&c["path"]))),
        );
    }
    for c in f["absolutize"].as_array().unwrap() {
        check(
            "absolutizePath",
            c,
            c["absolute"].clone(),
            json!(php::absolutize_path("/cwd/project", s(&c["path"]))),
        );
    }
    for c in f["relative"].as_array().unwrap() {
        let got = php::parent_relative_path(s(&c["parent"]), s(&c["file"]));
        check("getRelativePath", c, c["relative"].clone(), json!(got));
    }
    for c in f["fnmatch"].as_array().unwrap() {
        check(
            "fnmatch",
            c,
            c["matched"].clone(),
            json!(php::fnmatch(s(&c["pattern"]), s(&c["string"]))),
        );
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Literal-array PHP configs (baselines) as `require` returns them; anything beyond literals is refused.
#[test]
fn php_config_files() {
    let dir = format!("{}/tests/fixtures/phpfile", env!("CARGO_MANIFEST_DIR"));
    let want = &fixture("php.json")["phpfile"];
    let mut names: Vec<String> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    for name in names {
        let php = std::fs::read_to_string(format!("{dir}/{name}")).unwrap();
        let got = pun_phpstan::phpfile::load_config(&php, "{dir}");
        if name.starts_with("ok-") {
            let got = common::tagged(&pun_phpstan::Value::Array(got.unwrap()));
            assert_eq!(got, want[&name], "{name}");
        } else {
            assert!(got.is_err(), "{name}: {got:?}");
        }
    }
    for not_generated in [
        "<?php class X {}",
        "<?php const EXTENSIONS;",
        "<?php const EXTENSIONS = 5;",
    ] {
        assert!(pun_phpstan::phpfile::generated_config_extensions(not_generated, "/").is_err());
    }
}
