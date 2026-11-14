//! pun.lock: the exact packages a project installs, next to pun.toml. It is TOML holding what composer.lock holds:
//! each package's metadata as its repository published it (version, dist, requirements, autoload rules, ...), in
//! the same order, so `vendor/composer/installed.json` can be written from it byte for byte. `content-hash` is the
//! md5 Composer computes over the parts of the manifest that decide the resolution, so a pun.lock says whether it
//! still matches pun.toml.

use crate::php::json_compact;
use anyhow::{Context, Result, bail};
use pun_config::toml_text;
use serde_json::{Map, Value};
use std::fmt::Write;
use std::path::Path;

/// pun.lock's format version.
pub const VERSION: i64 = 1;

const HEADER: &str =
    "# pun.lock: the exact packages `pun install` installs. Written by Pun; do not edit by hand.\n";

/// The top-level keys after `content-hash`, in the order pun.lock writes them.
const SETTINGS: [&str; 9] = [
    "minimum-stability",
    "stability-flags",
    "prefer-stable",
    "prefer-lowest",
    "platform",
    "platform-dev",
    "platform-overrides",
    "plugin-api-version",
    "aliases",
];

#[derive(Debug, Clone, PartialEq)]
pub struct Lock {
    pub content_hash: String,
    /// [`SETTINGS`] that are set, in that order.
    pub settings: Map<String, Value>,
    /// Each package as composer.lock's entry for it.
    pub packages: Vec<Map<String, Value>>,
    pub packages_dev: Vec<Map<String, Value>>,
}

/// `md5(json_encode(...))` of the manifest's `name`, `version`, links, stability settings, repositories, `extra`,
/// `config.platform` and `config.policy`, with the top-level keys sorted (Composer's `Locker::getContentHash`).
pub fn content_hash(manifest: &Map<String, Value>) -> String {
    const RELEVANT: [&str; 11] = [
        "name",
        "version",
        "require",
        "require-dev",
        "conflict",
        "replace",
        "provide",
        "minimum-stability",
        "prefer-stable",
        "repositories",
        "extra",
    ];
    let mut relevant: Vec<(String, Value)> = (manifest.iter())
        .filter(|(k, _)| RELEVANT.contains(&k.as_str()))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    let mut config = Map::new();
    for key in ["platform", "policy"] {
        if let Some(v) = manifest.get("config").and_then(|c| c.get(key)) {
            config.insert(key.to_string(), v.clone());
        }
    }
    if !config.is_empty() {
        relevant.push(("config".into(), Value::Object(config)));
    }
    relevant.sort_by(|a, b| a.0.cmp(&b.0));
    let json = json_compact(&Value::Object(relevant.into_iter().collect()));
    crate::php::md5(json.as_bytes())
}

impl Lock {
    /// composer.lock's text as a pun.lock. Its `content-hash` is Composer's, so it is fresh for pun.toml exactly when
    /// composer.lock was fresh for composer.json (dependency sections migrate unchanged).
    pub fn from_composer_lock(text: &str) -> Result<Lock> {
        let mut json: Value =
            serde_json::from_str(text).context("composer.lock is not valid JSON")?;
        toml_text::strip_nulls(&mut json, "", &mut vec![]);
        let Value::Object(mut root) = json else {
            bail!("composer.lock must hold an object");
        };
        let content_hash = match root.get("content-hash") {
            Some(Value::String(h)) => h.clone(),
            _ => bail!("composer.lock has no content-hash"),
        };
        let mut packages = |key: &str| -> Result<Vec<Map<String, Value>>> {
            match root.shift_remove(key) {
                None => Ok(vec![]),
                Some(Value::Array(items)) => (items.into_iter())
                    .map(|p| match p {
                        Value::Object(m) if m.get("name").is_some_and(Value::is_string) => Ok(m),
                        _ => bail!(
                            "composer.lock: every entry of {key} must be a package with a name"
                        ),
                    })
                    .collect(),
                Some(_) => bail!("composer.lock: {key} must be a list"),
            }
        };
        let (packages, packages_dev) = (packages("packages")?, packages("packages-dev")?);
        let settings = (SETTINGS.iter())
            .filter_map(|k| Some((k.to_string(), root.get(*k)?.clone())))
            .collect();
        Ok(Lock {
            content_hash,
            settings,
            packages,
            packages_dev,
        })
    }

    /// pun.lock's text: the header comment, `version`, `content-hash` and the settings, then one `[[packages]]` (or
    /// `[[packages-dev]]`) table per package with dotted keys in composer.lock's order.
    pub fn to_toml(&self) -> String {
        let mut out = String::from(HEADER);
        writeln!(out, "version = {VERSION}").unwrap();
        writeln!(
            out,
            "content-hash = {}",
            toml_text::string(&self.content_hash)
        )
        .unwrap();
        toml_text::dotted(&mut out, "", &self.settings);
        for (table, list) in [
            ("packages", &self.packages),
            ("packages-dev", &self.packages_dev),
        ] {
            for p in list {
                write!(out, "\n[[{table}]]\n").unwrap();
                toml_text::dotted(&mut out, "", p);
            }
        }
        out
    }

    /// pun.lock's text read back.
    pub fn parse(text: &str) -> Result<Lock> {
        let table: toml::Table = toml::from_str(text)?;
        let mut json = toml_text::to_json(&toml::Value::Table(table));
        let mut root = json.as_object_mut().map(std::mem::take).unwrap_or_default();
        match root.get("version").and_then(Value::as_i64) {
            Some(VERSION) => {}
            Some(v) => bail!(
                "pun.lock has format version {v}; this Pun reads version {VERSION}, so update Pun"
            ),
            None => bail!("pun.lock has no version"),
        }
        let Some(Value::String(content_hash)) = root.shift_remove("content-hash") else {
            bail!("pun.lock has no content-hash");
        };
        let mut packages = |key: &str| -> Result<Vec<Map<String, Value>>> {
            match root.shift_remove(key) {
                None => Ok(vec![]),
                Some(Value::Array(items)) => (items.into_iter())
                    .map(|p| match p {
                        Value::Object(m) if m.get("name").is_some_and(Value::is_string) => Ok(m),
                        _ => bail!("pun.lock: every [[{key}]] needs a name"),
                    })
                    .collect(),
                Some(_) => bail!("pun.lock: {key} must be [[{key}]] tables"),
            }
        };
        let (packages, packages_dev) = (packages("packages")?, packages("packages-dev")?);
        let settings = (SETTINGS.iter())
            .filter_map(|k| Some((k.to_string(), root.get(*k)?.clone())))
            .collect();
        Ok(Lock {
            content_hash,
            settings,
            packages,
            packages_dev,
        })
    }

    /// `root/pun.lock`, or `None` when there is none.
    pub fn read(root: &Path) -> Result<Option<Lock>> {
        let path = root.join("pun.lock");
        match std::fs::read_to_string(&path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e).with_context(|| format!("can't read {}", path.display())),
            Ok(text) => Lock::parse(&text)
                .with_context(|| format!("invalid {}", path.display()))
                .map(Some),
        }
    }

    /// Every package, the dev ones last.
    pub fn all(&self) -> impl Iterator<Item = &Map<String, Value>> {
        self.packages.iter().chain(&self.packages_dev)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// A composer.lock as Composer 2.11.2 writes it, trimmed to two packages.
    const COMPOSER_LOCK: &str = r#"{
    "_readme": ["This file locks the dependencies of your project to a known state"],
    "content-hash": "0f2b0c8f6a1d4e1b9c3a7d5e8f601234",
    "packages": [
        {
            "name": "psr/log",
            "version": "3.0.2",
            "source": {"type": "git", "url": "https://github.com/php-fig/log.git", "reference": "f16e1d5863e37f8d8c2a01719f5b34baa2b714d3"},
            "dist": {"type": "zip", "url": "https://api.github.com/repos/php-fig/log/zipball/f16e1d5863e37f8d8c2a01719f5b34baa2b714d3", "reference": "f16e1d5863e37f8d8c2a01719f5b34baa2b714d3", "shasum": ""},
            "require": {"php": ">=8.0.0"},
            "type": "library",
            "extra": {"branch-alias": {"dev-master": "3.x-dev"}},
            "autoload": {"psr-4": {"Psr\\Log\\": "src"}},
            "notification-url": "https://packagist.org/downloads/",
            "license": ["MIT"],
            "authors": [{"name": "PHP-FIG", "homepage": "https://www.php-fig.org/"}],
            "description": "Common interface for logging libraries",
            "keywords": ["log", "psr", "psr-3"],
            "support": {"source": "https://github.com/php-fig/log/tree/3.0.2"},
            "time": "2024-09-11T13:17:53+00:00"
        }
    ],
    "packages-dev": [
        {"name": "acme/tool", "version": "dev-main", "dist": {"type": "path", "url": "packages/tool", "reference": null}, "type": "metapackage", "extra": {}}
    ],
    "aliases": [],
    "minimum-stability": "stable",
    "stability-flags": {"acme/tool": 20},
    "prefer-stable": true,
    "prefer-lowest": false,
    "platform": {"php": "^8.4"},
    "platform-dev": {},
    "plugin-api-version": "2.9.0"
}"#;

    #[test]
    fn composer_lock_round_trips_through_pun_lock() {
        let lock = Lock::from_composer_lock(COMPOSER_LOCK).unwrap();
        assert_eq!(lock.content_hash, "0f2b0c8f6a1d4e1b9c3a7d5e8f601234");
        assert_eq!((lock.packages.len(), lock.packages_dev.len()), (1, 1));
        let text = lock.to_toml();
        assert!(text.starts_with(HEADER), "{text}");
        assert!(text.contains(
            "\n[[packages]]\nname = \"psr/log\"\nversion = \"3.0.2\"\nsource.type = \"git\"\n"
        ));
        let back = Lock::parse(&text).unwrap();
        assert_eq!(back, lock);
        // composer.lock's own order, key by key, including the `null` reference left out.
        let want: Value = serde_json::from_str(COMPOSER_LOCK).unwrap();
        assert_eq!(Value::Object(back.packages[0].clone()), want["packages"][0]);
        assert_eq!(
            Value::Object(back.packages_dev[0].clone()),
            json!({"name": "acme/tool", "version": "dev-main", "dist": {"type": "path", "url": "packages/tool"}, "type": "metapackage", "extra": {}})
        );
        assert_eq!(back.all().count(), 2);
    }

    /// The hash Composer 2.11.2 wrote into laravel/laravel's composer.lock for this composer.json's relevant keys
    /// (`php -r 'echo Composer\Package\Locker::getContentHash($json);'`).
    #[test]
    fn content_hash_matches_composer() {
        let manifest = json!({
            "name": "acme/app",
            "description": "left out",
            "require": {"php": "^8.4", "laravel/framework": "^13.0"},
            "autoload": {"psr-4": {"App\\": "app/"}},
            "extra": {"laravel": {"dont-discover": []}},
            "config": {"sort-packages": true, "platform": {"php": "8.4.1"}},
            "minimum-stability": "stable",
            "prefer-stable": true
        });
        assert_eq!(
            content_hash(manifest.as_object().unwrap()),
            "1f6b30e94de0732cc8351f3d4a2ecc8c"
        );
    }

    #[test]
    fn bad_locks_are_errors() {
        let err = |t: &str| Lock::parse(t).unwrap_err().to_string();
        assert_eq!(err("content-hash = \"x\"\n"), "pun.lock has no version");
        assert_eq!(
            err("version = 9\n"),
            "pun.lock has format version 9; this Pun reads version 1, so update Pun"
        );
        assert_eq!(err("version = 1\n"), "pun.lock has no content-hash");
        assert_eq!(
            err("version = 1\ncontent-hash = \"x\"\n[[packages]]\nversion = \"1\"\n"),
            "pun.lock: every [[packages]] needs a name"
        );
        assert_eq!(
            err("version = 1\ncontent-hash = \"x\"\npackages = 1\n"),
            "pun.lock: packages must be [[packages]] tables"
        );
        let lerr = |t: &str| Lock::from_composer_lock(t).unwrap_err().to_string();
        assert_eq!(lerr("[]"), "composer.lock must hold an object");
        assert_eq!(lerr("{}"), "composer.lock has no content-hash");
        assert_eq!(
            lerr(r#"{"content-hash":"x","packages":{}}"#),
            "composer.lock: packages must be a list"
        );
        assert_eq!(
            lerr(r#"{"content-hash":"x","packages":[1]}"#),
            "composer.lock: every entry of packages must be a package with a name"
        );
        assert_eq!(lerr("{"), "composer.lock is not valid JSON");
        let empty = Lock::parse("version = 1\ncontent-hash = \"x\"\n").unwrap();
        assert_eq!(
            empty,
            Lock::from_composer_lock(r#"{"content-hash":"x"}"#).unwrap()
        );
        assert!(empty.packages.is_empty() && empty.packages_dev.is_empty());
    }

    #[test]
    fn reads_pun_lock_from_the_project() {
        let d = std::env::temp_dir().join(format!("pun-lock-read-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        assert!(Lock::read(&d).unwrap().is_none());
        std::fs::write(d.join("pun.lock"), "version = 1\ncontent-hash = \"abc\"\n").unwrap();
        assert_eq!(Lock::read(&d).unwrap().unwrap().content_hash, "abc");
        std::fs::write(d.join("pun.lock"), "version = 1\n").unwrap();
        let err = Lock::read(&d).unwrap_err();
        assert_eq!(
            err.to_string(),
            format!("invalid {}", d.join("pun.lock").display())
        );
        assert_eq!(err.root_cause().to_string(), "pun.lock has no content-hash");
        std::fs::remove_file(d.join("pun.lock")).unwrap();
        std::fs::create_dir(d.join("pun.lock")).unwrap();
        let err = Lock::read(&d).unwrap_err().to_string();
        assert_eq!(err, format!("can't read {}", d.join("pun.lock").display()));
        std::fs::remove_dir_all(&d).unwrap();
    }
}
