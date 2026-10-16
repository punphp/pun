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

/// PHP-CS-Fixer rule sets pun can expand (`RuleSets::getSetDefinition($name)->getRules()` of the pinned version).
/// Other sets stay unexpanded and are reported as not ported.
const SETS: &str = include_str!("../presets/sets.json");

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
    /// The merged rule map before set expansion and `false` filtering: what Pint's Config gets (delegate mode).
    pub configured_rules: Map<String, Value>,
    /// `.php-cs-fixer(.dist).php` that replaces pint.json (`pun format` without pint.json): run by delegate mode.
    pub php_cs_fixer: Option<PathBuf>,
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
#[serde(deny_unknown_fields)]
struct PunToml {
    #[serde(default)]
    format: PunFormat,
    #[serde(default)]
    compat: PunCompat,
}

/// `[format]`: everything pint.json holds, so pun.toml can replace it. Keys set here win over pint.json's.
#[derive(Deserialize, Default)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
struct PunFormat {
    engine: Option<Engine>,
    preset: Option<String>,
    paths: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
    not_name: Option<Vec<String>>,
    not_path: Option<Vec<String>>,
    #[serde(default)]
    rules: Map<String, Value>,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
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

    let php_cs_fixer = [".php-cs-fixer.php", ".php-cs-fixer.dist.php"]
        .iter()
        .map(|f| root.join(f))
        .find(|f| f.exists())
        .filter(|_| !opts.pint_mode && !opts.no_config && opts.config.is_none() && !pint_path.exists() && !root.join("pun.toml").exists());

    let toml_path = root.join("pun.toml");
    let toml = if !opts.pint_mode && toml_path.exists() {
        let t: PunToml = toml::from_str(&std::fs::read_to_string(&toml_path)?)
            .with_context(|| format!("invalid {}", toml_path.display()))?;
        sources.push(toml_path);
        t
    } else {
        PunToml::default()
    };

    let preset = opts
        .preset
        .clone()
        .or_else(|| toml.format.preset.clone())
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

    let PunToml { format: f, compat } = toml;
    rules.extend(f.rules);
    let engine = opts.engine.or(f.engine).unwrap_or(Engine::Compat);
    let php = compat.php.unwrap_or_else(|| "8.4".into());

    // RuleSet::__construct: only true, false or an options array.
    for (name, v) in &rules {
        if !matches!(v, Value::Bool(_) | Value::Array(_) | Value::Object(_)) {
            let (what, kind) = if name.starts_with('@') {
                ("Set must be enabled (true) or disabled (false).", "set")
            } else {
                ("Rule must be enabled (true), disabled (false) or configured (non-empty, assoc array).", "rule")
            };
            let null = if v.is_null() { format!(" To disable the {kind}, use \"FALSE\" instead of \"NULL\".") } else { String::new() };
            bail!("[{name}] {what} Other values are not allowed.{null}");
        }
    }

    let configured_rules = rules.clone();
    let rules = resolve_set(rules);
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
    let finder = Finder {
        in_dirs: f.paths.unwrap_or_else(|| list("in")),
        exclude: f.exclude.unwrap_or_else(|| list("exclude")),
        not_name: f.not_name.unwrap_or_else(|| list("notName")),
        not_path: f.not_path.unwrap_or_else(|| list("notPath")),
    };

    Ok(Config { root: root.to_path_buf(), preset, engine, php, rules, finder, blade, sources, configured_rules, php_cs_fixer })
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

/// RuleSet::resolveSet: sets expand in place (array_merge: a rule already present keeps its position, takes the
/// set's value); a later rule or set overrides an earlier one.
fn resolve_set(rules: Map<String, Value>) -> Map<String, Value> {
    let mut out = Map::new();
    for (name, value) in rules {
        // Set values are already checked to be bools.
        match resolve_subset(&name, value == Value::Bool(true)) {
            Some(set) => out.extend(set),
            None => drop(out.insert(name, value)),
        }
    }
    out
}

/// RuleSet::resolveSubset: a set's rules with nested sets merged in; `false` disables every rule of the set.
fn resolve_subset(name: &str, enabled: bool) -> Option<Map<String, Value>> {
    let mut sets: std::collections::BTreeMap<String, Map<String, Value>> = serde_json::from_str(SETS).unwrap();
    let def = sets.remove(name)?;
    let mut rules = def.clone();
    for (k, v) in def {
        if k.starts_with('@') {
            rules.shift_remove(&k);
            rules.extend(resolve_subset(&k, enabled).unwrap());
        } else {
            rules.insert(k, if enabled { v } else { Value::Bool(false) });
        }
    }
    Some(rules)
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

/// `pun config --init`: pint.json rewritten as a standalone pun.toml (same preset, rule overrides and finder keys).
pub fn init_toml(root: &Path) -> Result<String> {
    let pint = read_pint_json(&root.join("pint.json"), &mut Vec::new())?;
    let mut format = toml::Table::new();
    format.insert("preset".into(), pint.get("preset").and_then(Value::as_str).unwrap_or("laravel").into());
    for (json, key) in [("in", "paths"), ("exclude", "exclude"), ("notName", "not-name"), ("notPath", "not-path")] {
        match pint.get(json) {
            Some(Value::String(s)) => drop(format.insert(key.into(), vec![s.clone()].into())),
            Some(v @ Value::Array(_)) => drop(format.insert(key.into(), toml::Value::try_from(v)?)),
            _ => {}
        }
    }
    if let Some(r) = pint.get("rules") {
        format.insert("rules".into(), toml::Value::try_from(r)?);
    }
    let mut doc = toml::Table::new();
    doc.insert("format".into(), format.into());
    Ok(toml::to_string(&doc)?)
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
        // A PHP-CS-Fixer config replaces pint.json for `pun format` (php-cs-fixer's lookup order); `pint`, pint.json,
        // pun.toml and --config keep Pint's behaviour of ignoring it.
        let d = dir(&[("composer.json", "{}"), (".php-cs-fixer.dist.php", "<?php")]);
        assert_eq!(load(&d, &Options::default()).unwrap().php_cs_fixer, Some(d.join(".php-cs-fixer.dist.php")));
        std::fs::write(d.join(".php-cs-fixer.php"), "<?php").unwrap();
        assert_eq!(load(&d, &Options::default()).unwrap().php_cs_fixer, Some(d.join(".php-cs-fixer.php")));
        assert_eq!(load(&d, &Options { pint_mode: true, ..Default::default() }).unwrap().php_cs_fixer, None);
        assert_eq!(load(&d, &Options { config: Some("x.json".into()), ..Default::default() }).unwrap().php_cs_fixer, None);
        std::fs::write(d.join("pun.toml"), "").unwrap();
        assert_eq!(load(&d, &Options::default()).unwrap().php_cs_fixer, None);
        std::fs::remove_file(d.join("pun.toml")).unwrap();
        std::fs::write(d.join("pint.json"), r#"{"preset":"psr12","rules":{"braces_position":false,"single_quote":true}}"#).unwrap();
        let c = load(&d, &Options::default()).unwrap();
        assert_eq!(c.php_cs_fixer, None);
        // Delegate mode gets the map Pint builds: sets unexpanded, `false` kept, in merge order.
        assert_eq!(
            serde_json::Value::Object(c.configured_rules),
            serde_json::json!({"@PSR12": true, "no_unused_imports": true, "braces_position": false, "single_quote": true})
        );
        assert!(!c.rules.iter().any(|r| r.name == "braces_position" || r.name.starts_with('@')));
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

    #[test]
    fn pun_toml_replaces_pint_json() {
        // pun.toml keys win key by key; pint.json keys pun.toml doesn't set still apply.
        let d = dir(&[
            ("pint.json", r#"{"preset":"psr12","exclude":["a"],"notName":["n.php"],"rules":{"single_quote":true}}"#),
            ("pun.toml", "[format]\npreset = \"empty\"\npaths = [\"src\"]\nexclude = [\"vendor2\"]\nnot-path = [\"p\"]\n[format.rules]\nfinal_class = true\n"),
        ]);
        let c = load(&d, &Options::default()).unwrap();
        assert_eq!(c.preset, "empty");
        assert_eq!(c.rules.iter().map(|r| r.name.as_str()).collect::<Vec<_>>(), ["single_quote", "final_class"]);
        assert_eq!((c.finder.in_dirs, c.finder.exclude, c.finder.not_name, c.finder.not_path), (vec!["src".to_string()], vec!["vendor2".to_string()], vec!["n.php".to_string()], vec!["p".to_string()]));
        // --preset still beats pun.toml; pint mode ignores pun.toml entirely.
        assert_eq!(load(&d, &Options { preset: Some("laravel".into()), ..Default::default() }).unwrap().preset, "laravel");
        assert_eq!(load(&d, &Options { pint_mode: true, ..Default::default() }).unwrap().preset, "psr12");
        let d = dir(&[("pun.toml", "[format]\nexlude = [\"typo\"]\n")]);
        assert!(format!("{:#}", load(&d, &Options::default()).unwrap_err()).contains("unknown field `exlude`"));
    }

    #[test]
    fn init_toml_round_trips() {
        // The generated pun.toml alone must resolve to exactly what pint.json resolved to.
        let pint = r#"{"preset":"psr12","in":"app","exclude":["a","b"],"notPath":"y","rules":{"concat_space":{"spacing":"one"},"array_syntax":false,"final_class":true}}"#;
        let from_pint = load(&dir(&[("pint.json", pint)]), &Options::default()).unwrap();
        let toml = init_toml(&dir(&[("pint.json", pint)])).unwrap();
        let from_toml = load(&dir(&[("pun.toml", &toml)]), &Options::default()).unwrap();
        assert_eq!(from_toml.show()["format"], from_pint.show()["format"]);
        assert_eq!(from_toml.show()["finder"], from_pint.show()["finder"]);
        assert_eq!(init_toml(&dir(&[])).unwrap(), "[format]\npreset = \"laravel\"\n");
    }

    #[test]
    fn rule_values_like_pint() {
        // Messages from real Pint 1.29.3 (PHP-CS-Fixer RuleSet) on the same pint.json.
        let d = dir(&[("pint.json", r#"{"rules":{"final_class":null}}"#)]);
        assert_eq!(err(&d, Options::default()), "[final_class] Rule must be enabled (true), disabled (false) or configured (non-empty, assoc array). Other values are not allowed. To disable the rule, use \"FALSE\" instead of \"NULL\".");
        let d = dir(&[("pint.json", r#"{"rules":{"@PSR12":1}}"#)]);
        assert_eq!(err(&d, Options::default()), "[@PSR12] Set must be enabled (true) or disabled (false). Other values are not allowed.");
        let d = dir(&[("pun.toml", "[format.rules]\nfinal_class = \"yes\"\n")]);
        assert!(err(&d, Options::default()).starts_with("[final_class] Rule must be enabled"));
    }

    /// Rule sets resolve like PHP-CS-Fixer's RuleSet (tests/rule_sets.json: `(new RuleSet($rules))->getRules()` of the
    /// pinned version): same rules, options and order. The first case is Pint's psr12 preset.
    #[test]
    fn rule_sets_like_php_cs_fixer() {
        let cases: Vec<Value> = serde_json::from_str(include_str!("../tests/rule_sets.json")).unwrap();
        let resolved = |c: &Config| -> Vec<(String, Value)> { c.rules.iter().map(|r| (r.name.clone(), r.options.clone())).collect() };
        let expected = |case: &Value| -> Vec<(String, Value)> {
            let m = case["expected"].as_object().unwrap();
            m.iter().map(|(k, v)| (k.clone(), if *v == Value::Bool(true) { Value::Null } else { v.clone() })).collect()
        };
        let psr12 = load(&dir(&[]), &Options { preset: Some("psr12".into()), ..Default::default() }).unwrap();
        assert_eq!(resolved(&psr12), expected(&cases[0]));
        for case in &cases {
            let pint = serde_json::json!({"preset": "empty", "rules": case["rules"]}).to_string();
            assert_eq!(resolved(&load(&dir(&[("pint.json", &pint)]), &Options::default()).unwrap()), expected(case));
        }
        // Sets pun has no definition for stay as one rule, so they are reported as not ported.
        let c = load(&dir(&[("pint.json", r#"{"preset":"empty","rules":{"@PER-CS":true}}"#)]), &Options::default()).unwrap();
        assert_eq!(resolved(&c), [("@PER-CS".to_string(), Value::Null)]);
    }
}
