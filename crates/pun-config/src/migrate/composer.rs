//! `pun migrate`, dependency part: composer.json becomes pun.toml's dependency sections ([`crate::manifest`]).
//! Constraints, autoload rules, scripts and `extra` are copied as they are. Credentials in `config` (`http-basic`,
//! `github-oauth` and the rest) do not belong in a committed file: they move to the project's `auth.json`, which
//! Composer reads too. A key composer.json's schema doesn't have, and a `null` (TOML has none), is named and skipped.

use crate::manifest::{PACKAGE_KEYS, SECTIONS};
use crate::toml_text;
use anyhow::{Result, bail};
use serde_json::{Map, Value};

/// `config` keys that hold credentials: Composer's auth.json keys.
pub const CREDENTIALS: [&str; 8] = [
    "http-basic",
    "bearer",
    "github-oauth",
    "gitlab-oauth",
    "gitlab-token",
    "bitbucket-oauth",
    "forgejo-token",
    "client-certificate",
];

#[derive(Debug)]
pub struct Dependencies {
    /// The sections, as pun.toml text.
    pub toml: String,
    /// The credentials taken out of `config`, as auth.json's top level; empty when there were none.
    pub auth: Map<String, Value>,
    /// What isn't carried over: (composer.json key, why).
    pub skipped: Vec<(String, String)>,
    /// The entries of `require` and `require-dev`.
    pub require: usize,
    pub require_dev: usize,
}

/// composer.json's text as pun.toml's dependency sections.
pub fn from_composer_json(text: &str) -> Result<Dependencies> {
    let mut json: Value = serde_json::from_str(text)
        .map_err(|e| anyhow::anyhow!("composer.json is not valid JSON: {e}"))?;
    let mut skipped = vec![];
    let mut nulls = vec![];
    toml_text::strip_nulls(&mut json, "", &mut nulls);
    for n in nulls {
        skipped.push((
            n[1..].replace('/', "."),
            "null has no TOML form".to_string(),
        ));
    }
    let Value::Object(mut root) = json else {
        bail!("composer.json must hold an object");
    };
    let mut auth = Map::new();
    if let Some(Value::Object(config)) = root.get_mut("config") {
        for k in CREDENTIALS {
            if let Some(v) = config.shift_remove(k) {
                auth.insert(k.to_string(), v);
            }
        }
    }
    if let Some(repos) = root.get_mut("repositories") {
        *repos = repository_list(repos);
    }
    let mut package = Map::new();
    let mut sections: Vec<(&str, Value)> = vec![];
    for (k, v) in root {
        if PACKAGE_KEYS.contains(&k.as_str()) {
            package.insert(k, v);
        } else if let Some(name) = SECTIONS.iter().find(|s| **s == k && **s != "package") {
            sections.push((name, v));
        } else if k != "$schema" {
            // `$schema` only points an editor at composer.json's JSON schema, so it is dropped unreported.
            skipped.push((k, "not a composer.json key".to_string()));
        }
    }
    let count = |name: &str| {
        (sections.iter())
            .find(|s| s.0 == name)
            .and_then(|s| s.1.as_object())
            .map_or(0, Map::len)
    };
    let (require, require_dev) = (count("require"), count("require-dev"));
    let mut toml = String::new();
    if !package.is_empty() {
        toml_text::section(&mut toml, "package", &package, false);
    }
    for name in SECTIONS {
        let Some((_, v)) = sections.iter().find(|s| s.0 == name) else {
            continue;
        };
        match v {
            Value::Object(m) if m.is_empty() => {}
            Value::Object(m) => toml_text::section(&mut toml, name, m, false),
            Value::Array(items) if name == "repositories" => {
                for m in items.iter().filter_map(Value::as_object) {
                    toml_text::section(&mut toml, name, m, true);
                }
            }
            // An empty `[]` (PHP's empty array) is an empty section.
            Value::Array(items) if items.is_empty() => {}
            _ => skipped.push((name.to_string(), "must be an object".to_string())),
        }
    }
    Ok(Dependencies {
        toml,
        auth,
        skipped,
        require,
        require_dev,
    })
}

/// `repositories` as a list. Composer also takes an object keyed by labels; the labels mean nothing except a
/// `false` entry, which disables that repository (`"packagist.org": false`) and stays as `{ label = false }`.
fn repository_list(repos: &Value) -> Value {
    match repos {
        Value::Object(m) => Value::Array(
            m.iter()
                .map(|(label, v)| match v {
                    Value::Bool(false) => {
                        Value::Object(Map::from_iter([(label.clone(), Value::Bool(false))]))
                    }
                    v => v.clone(),
                })
                .collect(),
        ),
        v => v.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Laravel's skeleton composer.json (laravel/laravel 13.x), hand-converted.
    const LARAVEL: &str = r#"{
    "$schema": "https://getcomposer.org/schema.json",
    "name": "laravel/laravel",
    "type": "project",
    "description": "The skeleton application for the Laravel framework.",
    "keywords": ["laravel", "framework"],
    "license": "MIT",
    "require": {
        "php": "^8.4",
        "laravel/framework": "^13.0",
        "laravel/tinker": "^3.0"
    },
    "require-dev": {
        "fakerphp/faker": "^1.23",
        "pestphp/pest": "^4.0"
    },
    "autoload": {
        "psr-4": {
            "App\\": "app/",
            "Database\\Factories\\": "database/factories/",
            "Database\\Seeders\\": "database/seeders/"
        }
    },
    "autoload-dev": {
        "psr-4": {
            "Tests\\": "tests/"
        }
    },
    "scripts": {
        "post-autoload-dump": [
            "Illuminate\\Foundation\\ComposerScripts::postAutoloadDump",
            "@php artisan package:discover --ansi"
        ],
        "test": [
            "@php artisan config:clear --ansi",
            "@php artisan test"
        ]
    },
    "extra": {
        "laravel": {
            "dont-discover": []
        }
    },
    "config": {
        "optimize-autoloader": true,
        "preferred-install": "dist",
        "sort-packages": true,
        "allow-plugins": {
            "pestphp/pest-plugin": true,
            "php-http/discovery": true
        },
        "github-oauth": {"github.com": "token"}
    },
    "minimum-stability": "stable",
    "prefer-stable": true
}"#;

    const LARAVEL_TOML: &str = r#"[package]
name = "laravel/laravel"
type = "project"
description = "The skeleton application for the Laravel framework."
keywords = ["laravel", "framework"]
license = "MIT"
minimum-stability = "stable"
prefer-stable = true

[require]
php = "^8.4"
"laravel/framework" = "^13.0"
"laravel/tinker" = "^3.0"

[require-dev]
"fakerphp/faker" = "^1.23"
"pestphp/pest" = "^4.0"

[autoload.psr-4]
'App\' = "app/"
'Database\Factories\' = "database/factories/"
'Database\Seeders\' = "database/seeders/"

[autoload-dev]
psr-4 = { 'Tests\' = "tests/" }

[scripts]
post-autoload-dump = [
    'Illuminate\Foundation\ComposerScripts::postAutoloadDump',
    "@php artisan package:discover --ansi",
]
test = ["@php artisan config:clear --ansi", "@php artisan test"]

[config]
optimize-autoloader = true
preferred-install = "dist"
sort-packages = true
allow-plugins = { "pestphp/pest-plugin" = true, "php-http/discovery" = true }

[extra]
laravel = { dont-discover = [] }
"#;

    #[test]
    fn laravel_skeleton() {
        let d = from_composer_json(LARAVEL).unwrap();
        assert_eq!(d.toml, LARAVEL_TOML);
        assert_eq!(
            Value::Object(d.auth),
            json!({"github-oauth": {"github.com": "token"}})
        );
        assert!(d.skipped.is_empty(), "{:?}", d.skipped);
        assert_eq!((d.require, d.require_dev), (3, 2));
        // pun.toml's sections read back as the composer.json they came from, without what was skipped or moved.
        let table: toml::Table = toml::from_str(&d.toml).unwrap();
        let m = crate::manifest::parse(std::path::Path::new("pun.toml"), &table)
            .unwrap()
            .unwrap();
        let mut want: Value = serde_json::from_str(LARAVEL).unwrap();
        let want = want.as_object_mut().unwrap();
        want.shift_remove("$schema");
        want["config"]
            .as_object_mut()
            .unwrap()
            .shift_remove("github-oauth");
        assert_eq!(Value::Object(m.json), Value::Object(want.clone()));
    }

    #[test]
    fn repositories_nulls_and_wrong_shapes() {
        let d = from_composer_json(
            r#"{"$schema": "https://getcomposer.org/schema.json", "repositories": {"local": {"type": "path", "url": "packages/*"}, "packagist.org": false},
                "version": null, "require": [], "extra": "x", "nonsense": 1}"#,
        )
        .unwrap();
        assert_eq!(
            d.toml,
            "[[repositories]]\ntype = \"path\"\nurl = \"packages/*\"\n\n[[repositories]]\n\"packagist.org\" = false\n"
        );
        assert_eq!(
            d.skipped,
            [
                ("version".to_string(), "null has no TOML form".to_string()),
                (
                    "nonsense".to_string(),
                    "not a composer.json key".to_string()
                ),
                ("extra".to_string(), "must be an object".to_string()),
            ]
        );
        // A list of repositories stays a list.
        let d = from_composer_json(r#"{"repositories": [{"type": "vcs", "url": "u"}]}"#).unwrap();
        assert_eq!(d.toml, "[[repositories]]\ntype = \"vcs\"\nurl = \"u\"\n");
        assert!(from_composer_json("[]").is_err());
        assert!(
            from_composer_json("{")
                .unwrap_err()
                .to_string()
                .starts_with("composer.json is not valid JSON")
        );
    }
}
