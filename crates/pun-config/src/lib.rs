//! Resolved configuration from pun.toml: a Pint preset plus `[format]` / `[lint]` rules under Pun rule IDs
//! (rules/mapping.toml), gated on composer.json. pint.json and `.php-cs-fixer.php` are only read by `pun migrate`.

mod constraint;
pub mod migrate;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

pub use constraint::allows_below;

/// Presets from the pinned Pint (`resources/presets/*.php`), in declaration order. laravel comes from the
/// harness export (`rules_as_configured`); the others are plain rule maps exported the same way.
pub const PRESETS: [(&str, &str); 5] = [
    (
        "laravel",
        include_str!("../../../docs/presets/laravel.json"),
    ),
    ("per", include_str!("../presets/per.json")),
    ("psr12", include_str!("../presets/psr12.json")),
    ("symfony", include_str!("../presets/symfony.json")),
    ("empty", include_str!("../presets/empty.json")),
];

/// PHP-CS-Fixer rule sets pun can expand (`RuleSets::getSetDefinition($name)->getRules()` of the pinned version).
/// Other sets stay unexpanded and are reported as not ported.
const SETS: &str = include_str!("../presets/sets.json");

/// Upstream rule -> Pun rule mapping (context/plan/rules.md).
const MAPPING: &str = include_str!("../rules/mapping.toml");

/// What every command prints when the project has no pun.toml.
pub const NO_CONFIG: &str = "No pun.toml found. Run `pun migrate`.";

/// One enabled fixer (upstream name). `options` is `Null` when the rule is just `true`.
#[derive(Debug, Clone, Serialize)]
pub struct Rule {
    pub name: String,
    pub options: Value,
}

/// Finder options, kept as Pint passes them to Symfony Finder.
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
    /// Target PHP version (`[php] version`).
    pub php: String,
    /// `pun format`'s fixers: the preset's and `[format.rules]`, minus lint rules. The pipeline sorts by priority.
    pub rules: Vec<Rule>,
    /// `[lint.rules]` backed by a fixer, and whether `--fix` applies it (`fix = "apply"`).
    pub lint: Vec<(Rule, bool)>,
    pub finder: Finder,
    /// `Pint/laravel_blade` is on: `*.blade.php` files are discovered.
    pub blade: bool,
    /// Files that contributed, in load order.
    pub sources: Vec<PathBuf>,
}

#[derive(Debug, Default, Clone)]
pub struct Options {
    /// `--config`: pun.toml path (relative to root).
    pub config: Option<PathBuf>,
    pub no_config: bool,
    pub preset: Option<String>,
    pub blade: bool,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct PunToml {
    #[serde(default)]
    php: PunPhp,
    #[serde(default)]
    format: PunFormat,
    #[serde(default)]
    lint: PunLint,
    /// `[analyse]` is `pun analyse`'s.
    #[serde(default, rename = "analyse")]
    _analyse: Option<toml::Table>,
    /// The old lint + analyse tier, replaced in Step 1 (context/plan/config.md §5). Checked here; nothing applies tiers yet.
    #[serde(default, rename = "preset")]
    _tier: Option<Tier>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Tier {
    Essential,
    Recommended,
    Strict,
    Pedantic,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct PunPhp {
    version: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
struct PunFormat {
    preset: Option<String>,
    paths: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
    not_name: Option<Vec<String>>,
    not_path: Option<Vec<String>>,
    #[serde(default)]
    rules: toml::Table,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct PunLint {
    #[serde(default)]
    rules: toml::Table,
}

/// `[upstream.php-cs-fixer]`, `[upstream.pint]` and `[upstream.pun]` of rules/mapping.toml: fixer => entry. Only
/// these sections are parsed: every `pun format` run needs them, the rest of the file is ten times bigger.
pub fn fixer_entries() -> &'static toml::Table {
    static T: LazyLock<toml::Table> = LazyLock::new(|| {
        let mut out = toml::Table::new();
        for tool in ["php-cs-fixer", "pint", "pun"] {
            let head = format!("[upstream.{tool}]\n");
            let body = MAPPING.split_once(&head).expect("mapping section").1;
            let body = body.split("\n[").next().unwrap();
            let entries = toml::from_str::<toml::Table>(body).expect("mapping.toml");
            out.extend(entries);
        }
        out
    });
    &T
}

/// The Pun rule ID of a fixer (`no_alias_functions` -> `lint.no-alias-functions`).
pub fn rule_of(fixer: &str) -> Option<&'static str> {
    fixer_entries().get(fixer)?.get("pun")?.as_str()
}

/// The fixer behind a Pun rule ID; `command.` may be left out (`format.ordered-imports` or `ordered-imports`).
pub fn fixer_of(command: &str, rule: &str) -> Option<&'static str> {
    let id = format!(
        "{command}.{}",
        rule.strip_prefix(&format!("{command}.")).unwrap_or(rule)
    );
    fixer_entries()
        .iter()
        .find(|(_, e)| e.get("pun").and_then(toml::Value::as_str) == Some(&id))
        .map(|(k, _)| k.as_str())
}

pub fn load(root: &Path, opts: &Options) -> Result<Config> {
    let mut sources = Vec::new();
    check_composer(root, &mut sources)?;
    let path = root.join(opts.config.as_deref().unwrap_or(Path::new("pun.toml")));
    // `--no-config` (Pint's flag) is an explicit choice like `--preset`: the built-in preset, no file.
    let text = if opts.no_config || (opts.config.is_none() && !path.exists()) {
        if !opts.no_config && opts.preset.is_none() {
            bail!(NO_CONFIG);
        }
        String::new()
    } else {
        let t = std::fs::read_to_string(&path)
            .with_context(|| format!("can't read {}", path.display()))?;
        sources.push(path.clone());
        t
    };
    parse(root, &path, &text, opts, sources)
}

/// pun.toml text that isn't written yet (`pun migrate`) as `load` resolves it, without the composer.json gate.
pub fn from_toml(root: &Path, text: &str) -> Result<Config> {
    parse(
        root,
        &root.join("pun.toml"),
        text,
        &Options::default(),
        vec![],
    )
}

fn parse(
    root: &Path,
    path: &Path,
    text: &str,
    opts: &Options,
    sources: Vec<PathBuf>,
) -> Result<Config> {
    let toml: PunToml =
        toml::from_str(text).with_context(|| format!("invalid {}", path.display()))?;
    let PunToml {
        php,
        format: f,
        lint,
        ..
    } = toml;

    let preset = opts
        .preset
        .clone()
        .or(f.preset)
        .unwrap_or_else(|| "laravel".into());
    let mut rules = resolve_set(preset_rules(&preset).context("Preset not found.")?)?;
    for (name, v) in f.rules {
        let fixer = fixer(path, "format", &name)?;
        rules.insert(fixer.into(), options(&name, v)?);
    }
    if opts.blade {
        rules.insert("Pint/laravel_blade".into(), Value::Bool(true));
    }
    // Risky fixers are lint rules: a preset's run only under `[lint.rules]`.
    rules.retain(|name, _| !rule_of(name).is_some_and(|id| id.starts_with("lint.")));
    let rules = enabled(rules);
    let blade = rules.iter().any(|r| r.name == "Pint/laravel_blade");

    let mut lint_rules = vec![];
    for (name, v) in lint.rules {
        let fixer = fixer(path, "lint", &name)?;
        let (apply, v) = match v {
            toml::Value::String(s) => match s.as_str() {
                "off" => continue,
                "error" | "warn" | "info" => (false, Value::Bool(true)),
                _ => bail!(
                    "{}: `{name}` = \"{s}\": expected \"error\", \"warn\", \"info\" or \"off\"",
                    path.display()
                ),
            },
            toml::Value::Table(mut t) => {
                t.remove("severity");
                let apply = match t.remove("fix") {
                    None => false,
                    Some(fix) => match fix.as_str() {
                        Some("apply") => true,
                        Some("safe-only" | "off") => false,
                        _ => bail!(
                            "{}: `{name}`: fix must be \"apply\", \"safe-only\" or \"off\"",
                            path.display()
                        ),
                    },
                };
                // `{ fix = "apply" }` alone is the rule without options.
                let v = if t.is_empty() {
                    Value::Bool(true)
                } else {
                    options(&name, toml::Value::Table(t))?
                };
                (apply, v)
            }
            v => (false, options(&name, v)?),
        };
        if v != Value::Bool(false) {
            let mut r = enabled(Map::from_iter([(fixer.to_string(), v)]));
            lint_rules.push((r.remove(0), apply));
        }
    }

    Ok(Config {
        root: root.to_path_buf(),
        preset,
        php: php.version.unwrap_or_else(|| "8.4".into()),
        rules,
        lint: lint_rules,
        finder: Finder {
            in_dirs: f.paths.unwrap_or_default(),
            exclude: f.exclude.unwrap_or_default(),
            not_name: f.not_name.unwrap_or_default(),
            not_path: f.not_path.unwrap_or_default(),
        },
        blade,
        sources,
    })
}

/// The fixer for `name` under `[<command>.rules]`.
fn fixer(path: &Path, command: &str, name: &str) -> Result<&'static str> {
    let bare = name.strip_prefix(&format!("{command}.")).unwrap_or(name);
    if let Some(f) = fixer_of(command, bare).or_else(|| fixer_of(command, alias(command, bare)?)) {
        return Ok(f);
    }
    // An upstream fixer name (`array_syntax`) or a rule of the other command: name the ID to use instead.
    let other = if command == "format" {
        "lint"
    } else {
        "format"
    };
    let id = rule_of(&bare.replace('-', "_")).or_else(|| rule_of(fixer_of(other, bare)?));
    match id {
        Some(id) => bail!("{}: `{name}` is `{id}` in pun.toml", path.display()),
        None => bail!(
            "{}: `{name}` is not a {command} rule Pun can run (see crates/pun-config/rules/mapping.toml)",
            path.display()
        ),
    }
}

/// A Pun rule's metadata from `[rules.<command>]` of rules/mapping.toml.
#[derive(Debug)]
pub struct RuleMeta {
    /// `safe`, `unsafe`, `mixed` or `none` (analyse rules have no fix).
    pub fix: &'static str,
    /// Lowest preset tier that enables it (lint/analyse); `None` for format rules and opt-in rules.
    pub tier: Option<&'static str>,
    pub summary: Option<&'static str>,
    /// Earlier Pun names, without the `command.` prefix.
    pub aliases: Vec<&'static str>,
}

/// Every `[rules.format]`, `[rules.lint]` and `[rules.analyse]` entry by rule ID. Parsed on first use: a plain
/// `pun format` run never needs it.
fn rules() -> &'static Vec<(String, toml::Value)> {
    static T: LazyLock<Vec<(String, toml::Value)>> = LazyLock::new(|| {
        ["format", "lint", "analyse"]
            .iter()
            .flat_map(|c| {
                let body = MAPPING
                    .split_once(&format!("[rules.{c}]\n"))
                    .expect("mapping section")
                    .1;
                let rules =
                    toml::from_str::<toml::Table>(body.split("\n[").next().unwrap()).unwrap();
                rules
                    .into_iter()
                    .map(move |(rule, e)| (format!("{c}.{rule}"), e))
            })
            .collect()
    });
    &T
}

/// The metadata of a Pun rule ID (`lint.no-self-assignment`).
pub fn rule(id: &str) -> Option<RuleMeta> {
    let (_, e) = rules().iter().find(|(r, _)| r == id)?;
    let str = |k| e.get(k).and_then(toml::Value::as_str);
    Some(RuleMeta {
        fix: str("fix").unwrap_or("none"),
        tier: str("tier").filter(|t| *t != "none"),
        summary: str("summary"),
        aliases: e
            .get("aliases")
            .and_then(toml::Value::as_array)
            .map_or_else(Vec::new, |a| {
                a.iter().filter_map(toml::Value::as_str).collect()
            }),
    })
}

/// The current name of an earlier Pun rule name (`aliases` in `[rules.<command>]` of mapping.toml).
fn alias(command: &str, name: &str) -> Option<&'static str> {
    let prefix = format!("{command}.");
    rules().iter().find_map(|(id, _)| {
        let r = id.strip_prefix(&prefix)?;
        rule(id)?.aliases.contains(&name).then_some(r)
    })
}

/// A rule's value in pun.toml as PHP-CS-Fixer takes it: `true`/`false`, or options with snake_case keys.
fn options(rule: &str, v: toml::Value) -> Result<Value> {
    Ok(match v {
        toml::Value::Boolean(b) => Value::Bool(b),
        toml::Value::Table(t) => Value::Object(
            t.into_iter()
                .map(|(k, v)| (k.replace('-', "_"), serde_json::to_value(v).unwrap()))
                .collect(),
        ),
        _ => bail!("`{rule}` must be true, false or a table of options"),
    })
}

/// A preset's `getRules()` (sets unresolved), or `None` for an unknown preset.
pub fn preset_rules(preset: &str) -> Option<Map<String, Value>> {
    let (_, json) = PRESETS.iter().find(|(n, _)| *n == preset)?;
    let mut rules: Map<String, Value> = serde_json::from_str(json).unwrap();
    if let Some(Value::Object(r)) = rules.remove("rules_as_configured") {
        rules = r;
    }
    Some(rules)
}

/// `new RuleSet($rules)`: sets resolved, disabled rules dropped, `true` as no options.
pub fn rule_set(rules: Map<String, Value>) -> Result<Vec<Rule>> {
    Ok(enabled(resolve_set(rules)?))
}

fn enabled(rules: Map<String, Value>) -> Vec<Rule> {
    rules
        .into_iter()
        .filter(|(_, v)| *v != Value::Bool(false))
        .map(|(name, v)| Rule {
            name,
            options: if v == Value::Bool(true) {
                Value::Null
            } else {
                v
            },
        })
        .collect()
}

/// RuleSet::resolveSet: sets expand in place (array_merge: a rule already present keeps its position, takes the
/// set's value); a later rule or set overrides an earlier one.
fn resolve_set(rules: Map<String, Value>) -> Result<Map<String, Value>> {
    let sets: std::collections::BTreeMap<String, Map<String, Value>> =
        serde_json::from_str(SETS).unwrap();
    let mut out = Map::new();
    for (name, value) in rules {
        if name.starts_with('@') && !value.is_boolean() {
            bail!("Nested rule set \"{name}\" configuration must be a boolean.");
        }
        match resolve_subset(&sets, &name, value == Value::Bool(true)) {
            Some(set) => out.extend(set),
            None => drop(out.insert(name, value)),
        }
    }
    Ok(out)
}

/// RuleSet::resolveSubset: a set's rules with nested sets merged in; `false` disables every rule of the set.
fn resolve_subset(
    sets: &std::collections::BTreeMap<String, Map<String, Value>>,
    name: &str,
    enabled: bool,
) -> Option<Map<String, Value>> {
    let def = sets.get(name)?;
    let mut rules = def.clone();
    for (k, v) in def {
        if k.starts_with('@') {
            rules.shift_remove(k);
            rules.extend(resolve_subset(sets, k, enabled).unwrap());
        } else {
            rules.insert(
                k.clone(),
                if enabled {
                    v.clone()
                } else {
                    Value::Bool(false)
                },
            );
        }
    }
    Some(rules)
}

/// Refuse projects whose `require.php` admits any PHP below 8.4.
fn check_composer(root: &Path, sources: &mut Vec<PathBuf>) -> Result<()> {
    let path = root.join("composer.json");
    let Ok(bytes) = std::fs::read(&path) else {
        return Ok(());
    };
    let v: Value =
        serde_json::from_slice(&bytes).with_context(|| format!("invalid {}", path.display()))?;
    sources.push(path);
    let Some(c) = v.pointer("/require/php").and_then(Value::as_str) else {
        return Ok(());
    };
    // PUN_IGNORE_PHP_CONSTRAINT=1: harness-only escape hatch so pre-8.4 corpora (laravel/framework...) can be diffed.
    if allows_below(c, (8, 4, 0)).map_err(anyhow::Error::msg)?
        && std::env::var_os("PUN_IGNORE_PHP_CONSTRAINT").is_none()
    {
        bail!(
            "composer.json requires PHP \"{c}\", which allows PHP < 8.4. Pun only supports projects on PHP 8.4+; raise the constraint (for example \"^8.4\") or keep using Pint."
        );
    }
    Ok(())
}

impl Config {
    /// `pun config --show`: what runs, under Pun rule IDs (`true` for option-less rules).
    pub fn show(&self) -> Value {
        let id = |r: &Rule| rule_of(&r.name).unwrap_or(&r.name).to_string();
        let format: Map<String, Value> = (self.rules.iter())
            .map(|r| (id(r), migrate::value(&r.options, false)))
            .collect();
        let lint: Map<String, Value> = (self.lint.iter())
            .map(|(r, apply)| (id(r), migrate::value(&r.options, *apply)))
            .collect();
        serde_json::json!({
            "sources": self.sources,
            "php": { "version": self.php },
            "format": { "preset": self.preset, "rules": format },
            "lint": { "rules": lint },
            "finder": self.finder,
        })
    }
}

#[cfg(test)]
mod tests;
