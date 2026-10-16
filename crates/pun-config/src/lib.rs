//! Resolved formatter configuration: Pint preset + pint.json + pun.toml, gated on composer.json.

mod constraint;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::path::{Path, PathBuf};

pub use constraint::allows_below;

/// Presets from the pinned Pint (`resources/presets/*.php`), in declaration order. laravel comes from the
/// harness export (`rules_as_configured`); the others are plain rule maps exported the same way.
pub const PRESETS: [(&str, &str); 5] = [
    ("laravel", include_str!("../../../docs/presets/laravel.json")),
    ("per", include_str!("../presets/per.json")),
    ("psr12", include_str!("../presets/psr12.json")),
    ("symfony", include_str!("../presets/symfony.json")),
    ("empty", include_str!("../presets/empty.json")),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Engine {
    #[default]
    Compat,
    Pun,
}

/// One enabled fixer. `options` is `Null` when the rule is just `true`.
#[derive(Debug, Clone, Serialize)]
pub struct Rule {
    pub name: String,
    pub options: Value,
}

/// Finder options from pint.json, kept as Pint passes them to Symfony Finder.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Finder {
    #[serde(rename = "in")]
    pub in_dirs: Vec<String>,
    pub exclude: Vec<String>,
    #[serde(rename = "notName")]
    pub not_name: Vec<String>,
    #[serde(rename = "notPath")]
    pub not_path: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Config {
    pub root: PathBuf,
    pub preset: String,
    pub engine: Engine,
    /// Target PHP version emulated by the compat engine.
    pub php: String,
    /// Enabled rules in merge order (preset, then pint.json, then pun.toml). The pipeline sorts by priority.
    pub rules: Vec<Rule>,
    pub finder: Finder,
    /// `Pint/laravel_blade` is on: `*.blade.php` files are discovered.
    pub blade: bool,
    /// Files that contributed, in load order.
    pub sources: Vec<PathBuf>,
}

#[derive(Debug, Default, Clone)]
pub struct Options {
    /// `--config`: pint.json path (relative to root).
    pub config: Option<PathBuf>,
    pub no_config: bool,
    pub preset: Option<String>,
    pub engine: Option<Engine>,
    /// `--blade`.
    pub blade: bool,
    /// Invoked as `pint`: ignore pun.toml and .php-cs-fixer.php like Pint does.
    pub pint_mode: bool,
}

#[derive(Deserialize, Default)]
struct PunToml {
    #[serde(default)]
    format: PunFormat,
    #[serde(default)]
    compat: PunCompat,
}

#[derive(Deserialize, Default)]
struct PunFormat {
    engine: Option<Engine>,
    #[serde(default)]
    rules: Map<String, Value>,
}

#[derive(Deserialize, Default)]
struct PunCompat {
    php: Option<String>,
}

pub fn load(root: &Path, opts: &Options) -> Result<Config> {
    let mut sources = Vec::new();
    check_composer(root, &mut sources)?;

    // `--config=http://...` must reach read_pint_json unjoined (Path::join would make it `root/http:/...`).
    let pint_path = match &opts.config {
        Some(c) if c.to_string_lossy().contains("://") => c.clone(),
        c => root.join(c.clone().unwrap_or_else(|| "pint.json".into())),
    };
    let pint = if opts.no_config { Map::new() } else { read_pint_json(&pint_path, &mut sources)? };

    if !opts.pint_mode && !opts.no_config && opts.config.is_none() && !pint_path.exists() {
        for f in [".php-cs-fixer.php", ".php-cs-fixer.dist.php"] {
            if root.join(f).exists() {
                bail!("{f} found: evaluating PHP-CS-Fixer configs is not yet supported. Add a pint.json or pun.toml instead.");
            }
        }
    }

    let preset = opts
        .preset
        .clone()
        .or_else(|| pint.get("preset").and_then(Value::as_str).map(String::from))
        .unwrap_or_else(|| "laravel".into());
    let Some((_, preset_json)) = PRESETS.iter().find(|(n, _)| *n == preset) else {
        bail!("Preset not found.");
    };
    let mut rules: Map<String, Value> = serde_json::from_str(preset_json)?;
    if let Some(Value::Object(r)) = rules.remove("rules_as_configured") {
        rules = r;
    }

    // Pint: array_merge($preset, $pintJson['rules']) — overrides keep the preset's position.
    if let Some(r) = pint.get("rules").and_then(Value::as_object) {
        rules.extend(r.clone());
    }
    if opts.blade {
        rules.insert("Pint/laravel_blade".into(), Value::Bool(true));
    }

    let mut engine = Engine::Compat;
    let mut php = "8.4".to_string();
    let toml_path = root.join("pun.toml");
    if !opts.pint_mode && toml_path.exists() {
        let t: PunToml = toml::from_str(&std::fs::read_to_string(&toml_path)?)
            .with_context(|| format!("invalid {}", toml_path.display()))?;
        rules.extend(t.format.rules);
        engine = t.format.engine.unwrap_or(engine);
        php = t.compat.php.unwrap_or(php);
        sources.push(toml_path);
    }
    engine = opts.engine.unwrap_or(engine);

    let blade = rules.get("Pint/laravel_blade").is_some_and(|v| *v != Value::Bool(false));
    let rules = rules
        .into_iter()
        .filter(|(_, v)| *v != Value::Bool(false))
        .map(|(name, v)| Rule { name, options: if v == Value::Bool(true) { Value::Null } else { v } })
        .collect();

    let list = |k: &str| match pint.get(k) {
        Some(Value::Array(a)) => a.iter().filter_map(|v| v.as_str().map(String::from)).collect(),
        Some(Value::String(s)) => vec![s.clone()],
        _ => vec![],
    };
    let finder = Finder { in_dirs: list("in"), exclude: list("exclude"), not_name: list("notName"), not_path: list("notPath") };

    Ok(Config { root: root.to_path_buf(), preset, engine, php, rules, finder, blade, sources })
}

/// Pint's ConfigurationJsonRepository::get(): a missing file means "no config"; `extend` is one level deep.
fn read_pint_json(path: &Path, sources: &mut Vec<PathBuf>) -> Result<Map<String, Value>> {
    let s = path.to_string_lossy();
    if s.starts_with("http://") {
        bail!("Loading the configuration over plaintext HTTP is not allowed. Use HTTPS.");
    }
    if s.starts_with("https://") {
        bail!("Remote configuration files are not yet supported.");
    }
    if !path.exists() {
        return Ok(Map::new());
    }
    let invalid = || format!("The configuration file [{}] is not valid JSON.", path.display());
    let mut v: Value = serde_json::from_slice(&std::fs::read(path)?).with_context(invalid)?;
    if let Some(ext) = v.get("extend").and_then(Value::as_str) {
        let ext_path = path.parent().unwrap_or(Path::new(".")).join(ext);
        let base: Value = serde_json::from_slice(&std::fs::read(&ext_path).with_context(|| format!("cannot read {}", ext_path.display()))?)?;
        if base.get("extend").is_some() {
            bail!("Pint configuration cannot extend from more than 1 file.");
        }
        let mut merged = base;
        replace_recursive(&mut merged, v);
        v = merged;
        sources.push(ext_path);
    }
    sources.push(path.to_path_buf());
    match v {
        Value::Object(m) => Ok(m),
        _ => bail!(invalid()),
    }
}

/// PHP's array_replace_recursive on JSON values (lists merge by index).
fn replace_recursive(base: &mut Value, over: Value) {
    match (base, over) {
        (Value::Object(b), Value::Object(o)) => {
            for (k, v) in o {
                match b.get_mut(&k) {
                    Some(slot) => replace_recursive(slot, v),
                    None => {
                        b.insert(k, v);
                    }
                }
            }
        }
        (Value::Array(b), Value::Array(o)) => {
            for (i, v) in o.into_iter().enumerate() {
                match b.get_mut(i) {
                    Some(slot) => replace_recursive(slot, v),
                    None => b.push(v),
                }
            }
        }
        (b, o) => *b = o,
    }
}

/// Refuse projects whose `require.php` admits any PHP below 8.4.
fn check_composer(root: &Path, sources: &mut Vec<PathBuf>) -> Result<()> {
    let path = root.join("composer.json");
    let Ok(bytes) = std::fs::read(&path) else { return Ok(()) };
    let v: Value = serde_json::from_slice(&bytes).with_context(|| format!("invalid {}", path.display()))?;
    sources.push(path);
    if let Some(c) = v.pointer("/require/php").and_then(Value::as_str) {
        // PUN_IGNORE_PHP_CONSTRAINT=1: harness-only escape hatch so pre-8.4 corpora (laravel/framework...) can be diffed.
        if allows_below(c, (8, 4, 0)).map_err(anyhow::Error::msg)? && std::env::var_os("PUN_IGNORE_PHP_CONSTRAINT").is_none() {
            bail!(
                "composer.json requires PHP \"{c}\", which allows PHP < 8.4. Pun only supports projects on PHP 8.4+; raise the constraint (for example \"^8.4\") or keep using Pint."
            );
        }
    }
    Ok(())
}

impl Config {
    /// `pun config --show`, as a plain JSON/TOML-friendly tree (`true` for option-less rules).
    pub fn show(&self) -> Value {
        let rules: Map<String, Value> = self
            .rules
            .iter()
            .map(|r| (r.name.clone(), if r.options.is_null() { Value::Bool(true) } else { r.options.clone() }))
            .collect();
        serde_json::json!({
            "sources": self.sources,
            "format": { "engine": self.engine, "preset": self.preset, "rules": rules },
            "compat": { "php": self.php },
            "finder": self.finder,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir(files: &[(&str, &str)]) -> PathBuf {
        static N: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let n = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let d = std::env::temp_dir().join(format!("pun-config-{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        for (n, c) in files {
            std::fs::write(d.join(n), c).unwrap();
        }
        d
    }

    #[test]
    fn merges_like_pint() {
        let d = dir(&[
            ("composer.json", r#"{"require":{"php":"^8.4"}}"#),
            ("base.json", r#"{"rules":{"final_class":true},"exclude":["a","b"]}"#),
            ("pint.json", r#"{"extend":"base.json","rules":{"array_syntax":false,"concat_space":{"spacing":"one"},"Pint/laravel_blade":true},"exclude":["x"],"notPath":"y"}"#),
            ("pun.toml", "[format]\nengine = \"pun\"\n[format.rules]\nfinal_class = false\n[compat]\nphp = \"8.5\"\n"),
        ]);
        let c = load(&d, &Options::default()).unwrap();
        let names: Vec<_> = c.rules.iter().map(|r| r.name.as_str()).collect();
        assert!(!names.contains(&"array_syntax") && !names.contains(&"final_class"));
        let concat = names.iter().position(|n| *n == "concat_space").unwrap();
        assert!(concat < names.iter().position(|n| *n == "Pint/laravel_blade").unwrap(), "override keeps preset position");
        assert_eq!(c.rules[concat].options["spacing"], "one");
        assert!(c.blade);
        assert_eq!(c.finder.exclude, ["x", "b"]);
        assert_eq!(c.finder.not_path, ["y"]);
        assert_eq!((c.engine, c.php.as_str()), (Engine::Pun, "8.5"));
    }

    #[test]
    fn gates() {
        let d = dir(&[("composer.json", r#"{"require":{"php":"^8.2"}}"#)]);
        assert!(load(&d, &Options::default()).unwrap_err().to_string().contains("PHP < 8.4"));
        let d = dir(&[("composer.json", "{}"), (".php-cs-fixer.dist.php", "<?php")]);
        assert!(load(&d, &Options::default()).unwrap_err().to_string().contains("not yet supported"));
        assert!(load(&d, &Options { pint_mode: true, ..Default::default() }).is_ok());
    }

    fn err(d: &Path, o: Options) -> String {
        load(d, &o).unwrap_err().to_string()
    }

    /// Messages are Pint's (ConfigurationResolverFactory, ConfigurationJsonRepository).
    #[test]
    fn pint_errors() {
        let d = dir(&[("pint.json", r#"{"preset":"nope"}"#)]);
        assert_eq!(err(&d, Options::default()), "Preset not found.");
        // Real Pint: `pint --config=http://x/pint.json` prints this and exits 1.
        let o = |c: &str| Options { config: Some(c.into()), ..Default::default() };
        assert_eq!(err(&d, o("http://x/pint.json")), "Loading the configuration over plaintext HTTP is not allowed. Use HTTPS.");
        assert!(err(&d, o("https://x/pint.json")).contains("not yet supported"));
        let d = dir(&[("a.json", r#"{"extend":"b.json"}"#), ("pint.json", r#"{"extend":"a.json"}"#)]);
        assert_eq!(err(&d, Options::default()), "Pint configuration cannot extend from more than 1 file.");
        let d = dir(&[("pint.json", "1")]);
        assert!(err(&d, Options::default()).ends_with("pint.json] is not valid JSON."));
        let d = dir(&[("pint.json", "{")]);
        assert!(err(&d, Options::default()).ends_with("pint.json] is not valid JSON."));
    }

    #[test]
    fn options_and_show() {
        // Lists merge by index like array_replace_recursive: ["x"] over ["a","b"] is ["x","b"]; a longer override appends.
        let d = dir(&[
            ("base.json", r#"{"preset":"empty","notName":["a"]}"#),
            ("custom.json", r#"{"extend":"base.json","rules":{"concat_space":{"spacing":"one"},"single_quote":true},"notName":["x","y"],"in":"src"}"#),
        ]);
        let c = load(&d, &Options { config: Some("custom.json".into()), blade: true, engine: Some(Engine::Pun), ..Default::default() }).unwrap();
        assert_eq!(c.finder.not_name, ["x", "y"]);
        assert_eq!(c.finder.in_dirs, ["src"]);
        assert!(c.blade);
        assert_eq!(c.sources, [d.join("base.json"), d.join("custom.json")]);
        assert_eq!(
            c.show(),
            serde_json::json!({
                "sources": [d.join("base.json"), d.join("custom.json")],
                "format": {"engine": "pun", "preset": "empty", "rules": {"concat_space": {"spacing": "one"}, "single_quote": true, "Pint/laravel_blade": true}},
                "compat": {"php": "8.4"},
                "finder": {"in": ["src"], "exclude": [], "notName": ["x", "y"], "notPath": []},
            })
        );
        // --no-config ignores pint.json; --preset wins.
        let d = dir(&[("pint.json", r#"{"preset":"nope"}"#)]);
        let c = load(&d, &Options { no_config: true, preset: Some("psr12".into()), ..Default::default() }).unwrap();
        assert_eq!((c.preset.as_str(), c.sources.len()), ("psr12", 0));
    }
}
