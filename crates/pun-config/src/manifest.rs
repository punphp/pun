//! pun.toml's dependency sections: the project's package metadata, requirements, autoload rules, scripts and
//! repositories, everything composer.json holds (context/plan/packages.md §3). Each composer.json key has a section of
//! the same name, except the root package's own fields, which share `[package]`. `pun install` reads them through
//! [`Manifest::json`], in composer.json's shape, so the installer works in Composer's terms.

use crate::toml_text;
use anyhow::{Context, Result, bail};
use serde_json::{Map, Value};
use std::path::{Path, PathBuf};

/// The sections that hold the dependencies, in the order `pun migrate` writes them.
pub const SECTIONS: [&str; 15] = [
    "package",
    "require",
    "require-dev",
    "conflict",
    "provide",
    "replace",
    "suggest",
    "autoload",
    "autoload-dev",
    "scripts",
    "scripts-descriptions",
    "scripts-aliases",
    "config",
    "repositories",
    "extra",
];

/// The composer.json keys under `[package]`: the root package's own fields and its stability settings.
pub const PACKAGE_KEYS: [&str; 22] = [
    "name",
    "description",
    "version",
    "type",
    "keywords",
    "homepage",
    "readme",
    "time",
    "license",
    "authors",
    "support",
    "funding",
    "minimum-stability",
    "prefer-stable",
    "bin",
    "archive",
    "abandoned",
    "non-feature-branches",
    "target-dir",
    "include-path",
    "default-branch",
    "php-ext",
];

/// The sections whose entries are package names with version constraints (`suggest`'s are descriptions).
const LINKS: [&str; 6] = [
    "require",
    "require-dev",
    "conflict",
    "provide",
    "replace",
    "suggest",
];

/// The dependency sections of one pun.toml.
#[derive(Debug, Clone, PartialEq)]
pub struct Manifest {
    pub path: PathBuf,
    /// What composer.json would hold: `[package]`'s keys at the top, then each other section under its name.
    pub json: Map<String, Value>,
}

impl Manifest {
    /// `require` (or `require-dev`) as (package, constraint), in pun.toml's order.
    pub fn links(&self, section: &str) -> impl Iterator<Item = (&str, &str)> {
        (self
            .json
            .get(section)
            .and_then(Value::as_object)
            .into_iter()
            .flatten())
        .filter_map(|(k, v)| Some((k.as_str(), v.as_str()?)))
    }
}

/// The dependency sections of `root/pun.toml`; `None` when there is no pun.toml or it has none.
pub fn read(root: &Path) -> Result<Option<Manifest>> {
    let path = root.join("pun.toml");
    let text = match std::fs::read_to_string(&path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        r => r.with_context(|| format!("can't read {}", path.display()))?,
    };
    let table: toml::Table =
        toml::from_str(&text).with_context(|| format!("invalid {}", path.display()))?;
    parse(&path, &table)
}

/// The dependency sections of a parsed pun.toml at `path`, checked: link sections map names to strings, the others
/// are tables (`[[repositories]]` a list of them), and `[package]` holds only [`PACKAGE_KEYS`].
pub fn parse(path: &Path, table: &toml::Table) -> Result<Option<Manifest>> {
    if !SECTIONS.iter().any(|s| table.contains_key(*s)) {
        return Ok(None);
    }
    let mut json = Map::new();
    for name in SECTIONS {
        let Some(v) = table.get(name) else { continue };
        let v = toml_text::to_json(v);
        match name {
            "repositories" => {
                if !v.as_array().is_some_and(|a| a.iter().all(Value::is_object)) {
                    bail!("pun.toml: repositories must be a list of tables ([[repositories]])");
                }
            }
            _ if !v.is_object() => bail!("pun.toml: {name} must be a table ([{name}])"),
            _ => {}
        }
        if LINKS.contains(&name) {
            for (k, c) in v.as_object().into_iter().flatten() {
                if !c.is_string() {
                    let what = if name == "suggest" {
                        "a description"
                    } else {
                        "a version constraint"
                    };
                    bail!(
                        "pun.toml: [{name}] {} must be {what} string",
                        toml_text::key(k)
                    );
                }
            }
        }
        if name == "package" {
            for (k, v) in v.as_object().into_iter().flatten() {
                if !PACKAGE_KEYS.contains(&k.as_str()) {
                    bail!(
                        "pun.toml: [package] has no key {}; it takes {}",
                        toml_text::key(k),
                        PACKAGE_KEYS.join(", ")
                    );
                }
                json.insert(k.clone(), v.clone());
            }
        } else {
            json.insert(name.to_string(), v);
        }
    }
    Ok(Some(Manifest {
        path: path.to_path_buf(),
        json,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn manifest(text: &str) -> Result<Option<Manifest>> {
        parse(Path::new("pun.toml"), &toml::from_str(text).unwrap())
    }

    #[test]
    fn sections_become_composer_json() {
        let m = manifest(
            "[package]\nname = \"acme/app\"\nprefer-stable = true\n\n[require]\nphp = \"^8.4\"\n\"laravel/framework\" = \"^13.0\"\n\n[autoload.psr-4]\n'App\\' = \"app/\"\n\n[[repositories]]\ntype = \"path\"\nurl = \"packages/*\"\n\n[format]\npreset = \"laravel\"\n",
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            Value::Object(m.json.clone()),
            json!({
                "name": "acme/app",
                "prefer-stable": true,
                "require": {"php": "^8.4", "laravel/framework": "^13.0"},
                "autoload": {"psr-4": {"App\\": "app/"}},
                "repositories": [{"type": "path", "url": "packages/*"}],
            })
        );
        let links: Vec<_> = m.links("require").collect();
        assert_eq!(links, [("php", "^8.4"), ("laravel/framework", "^13.0")]);
        assert_eq!(m.links("require-dev").count(), 0);
    }

    #[test]
    fn a_pun_toml_without_dependencies_has_no_manifest() {
        assert_eq!(manifest("[format]\npreset = \"laravel\"\n").unwrap(), None);
    }

    #[test]
    fn reads_the_project_pun_toml() {
        let d = std::env::temp_dir().join(format!("pun-manifest-read-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        let path = d.join("pun.toml");
        assert_eq!(read(&d).unwrap(), None);
        std::fs::write(&path, "[require]\nphp = \"^8.4\"\n").unwrap();
        assert_eq!(
            read(&d)
                .unwrap()
                .unwrap()
                .links("require")
                .collect::<Vec<_>>(),
            [("php", "^8.4")]
        );
        std::fs::write(&path, "[require\n").unwrap();
        assert_eq!(
            read(&d).unwrap_err().to_string(),
            format!("invalid {}", path.display())
        );
        std::fs::remove_file(&path).unwrap();
        std::fs::create_dir(&path).unwrap();
        assert_eq!(
            read(&d).unwrap_err().to_string(),
            format!("can't read {}", path.display())
        );
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn wrong_shapes_are_errors() {
        let err = |t: &str| manifest(t).unwrap_err().to_string();
        assert_eq!(
            err("[require]\n\"a/b\" = 1\n"),
            "pun.toml: [require] \"a/b\" must be a version constraint string"
        );
        assert_eq!(
            err("[suggest]\n\"a/b\" = true\n"),
            "pun.toml: [suggest] \"a/b\" must be a description string"
        );
        assert_eq!(
            err("require = 1\n"),
            "pun.toml: require must be a table ([require])"
        );
        assert_eq!(
            err("[repositories]\ntype = \"vcs\"\n"),
            "pun.toml: repositories must be a list of tables ([[repositories]])"
        );
        assert!(
            err("[package]\nnmae = \"x\"\n")
                .starts_with("pun.toml: [package] has no key nmae; it takes name, ")
        );
    }
}
