//! php.rs against real PHP and PHPStan 2.3.1 (tests/fixtures/php.json, scripts/export-php-helpers-fixtures.php).

use crate::common::fixture;
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

#[test]
fn php_config_files() {
    let dir = format!("{}/tests/fixtures/phpfile", env!("CARGO_MANIFEST_DIR"));
    let want = &fixture("php.json")["phpfile"];
    let files = pun_phpstan::config::ProjectFiles::new("");
    let mut names: Vec<String> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    for name in names {
        let php = std::fs::read_to_string(format!("{dir}/{name}")).unwrap();
        let got = pun_phpstan::phpfile::load_config(&php, "{dir}", &files);
        if name.starts_with("ok-") {
            let got = crate::common::tagged(&pun_phpstan::Value::Array(got.unwrap()));
            assert_eq!(got, want[&name], "{name}");
        } else {
            let message = match name.as_str() {
                "unsupported-no-open-tag.php" => Some("unexpected \"no php\\n\" on line 1"),
                "unsupported-push-scalar.php" => Some("$x is not an array"),
                "unsupported-scalar.php" => Some("the config does not return an array"),
                "unsupported-undefined.php" => Some("undefined $nope"),
                "unsupported-unknown-class.php" => {
                    Some("App\\Foo is neither a PHP nor a PHPStan class")
                }
                "unsupported-unterminated.php" => Some("unterminated string"),
                _ => None,
            };
            let got = got.unwrap_err().to_string();
            if let Some(message) = message {
                assert_eq!(got, message, "{name}");
            }
        }
    }
    for (not_generated, message) in [
        ("<?php class X {}", "no EXTENSIONS constant"),
        ("<?php const EXTENSIONS;", "no EXTENSIONS value"),
        ("<?php const EXTENSIONS = 5;", "EXTENSIONS is not an array"),
    ] {
        let got = pun_phpstan::phpfile::generated_config_extensions(not_generated, "/", &files);
        assert_eq!(got.unwrap_err().to_string(), message);
    }
}

#[test]
fn php_string_cast() {
    let f = &fixture("php.json")["string_cast"];
    let pun_phpstan::Value::Array(items) = pun_phpstan::neon::decode(s(&f["neon"])).unwrap() else {
        panic!()
    };
    let got: Vec<J> = items.values().map(|v| json!(v.to_php_string())).collect();
    assert_eq!(J::Array(got), f["casts"]);
}
