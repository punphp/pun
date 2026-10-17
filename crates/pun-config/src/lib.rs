//! Resolved configuration from pun.toml (context/plan/config.md §3): `[php]`, `[files]`, `[format]` (a Pint preset
//! plus rules), `[lint]` (sets, groups, rules, overrides) and `[analyse]` (preset, paths, rules), all under Pun rule IDs
//! (rules/mapping.toml) and gated on composer.json and composer.lock. pint.json and `.php-cs-fixer.php` are only read
//! by `pun migrate`.

pub mod composer;
mod constraint;
mod explain;
pub mod migrate;
pub mod options;
mod schema;

pub use explain::{MAPPING_URL, explain};
pub use schema::{
    ANALYSE_PRESETS, Analyse, AnalyseRule, Files, LINT_SETS, Level, Lint, LintRule, LintSetting,
    Override,
};

use anyhow::{Context, Result, bail};
use composer::Composer;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

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

/// The format presets pun.toml takes (`per` and `symfony` stay in [`PRESETS`] for `pun migrate` and `--preset`).
pub const FORMAT_PRESETS: [&str; 3] = ["laravel", "psr12", "empty"];

/// One enabled fixer (upstream name). `options` is `Null` when the rule is just `true`.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Rule {
    pub name: String,
    pub options: Value,
}

/// `pun format`'s finder options, kept as Pint passes them to Symfony Finder: `in` is `[files] paths`, `exclude`
/// is `[files] exclude` plus `[format] exclude`.
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
    /// The format preset.
    pub preset: String,
    /// Target PHP version: `[php] version`, else the lowest composer.json admits, else 8.4.
    pub php: String,
    /// `pun format`'s fixers: the preset's and `[format.rules]`, minus lint rules. The pipeline sorts by priority.
    pub rules: Vec<Rule>,
    pub finder: Finder,
    /// `Pint/laravel_blade` is on: `*.blade.php` files are discovered.
    pub blade: bool,
    /// `[files]`, shared by every command.
    pub files: Files,
    pub lint: Lint,
    pub analyse: Analyse,
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
    files: schema::FilesToml,
    #[serde(default)]
    format: PunFormat,
    #[serde(default)]
    lint: schema::LintToml,
    #[serde(default)]
    analyse: schema::AnalyseToml,
    /// The old lint + analyse tier: an error naming its replacements.
    preset: Option<toml::Value>,
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
    /// Moved to `[files] paths`: an error.
    paths: Option<toml::Value>,
    #[serde(default)]
    exclude: Vec<String>,
    #[serde(default)]
    not_name: Vec<String>,
    #[serde(default)]
    not_path: Vec<String>,
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

/// The Pun rule ID of a fixer (`no_alias_functions` -> `lint/style/no-alias-function`).
pub fn rule_of(fixer: &str) -> Option<&'static str> {
    fixer_entries().get(fixer)?.get("pun")?.as_str()
}

/// The fixer behind a Pun rule ID; the `<command>/` part may be left out (`format/imports/ordered` or
/// `imports/ordered`).
pub fn fixer_of(command: &str, rule: &str) -> Option<&'static str> {
    let id = match rule.strip_prefix(command).and_then(|r| r.strip_prefix('/')) {
        Some(_) => rule.to_string(),
        None => format!("{command}/{rule}"),
    };
    fixer_entries()
        .iter()
        .find(|(_, e)| e.get("pun").and_then(toml::Value::as_str) == Some(&id))
        .map(|(k, _)| k.as_str())
}

pub fn load(root: &Path, opts: &Options) -> Result<Config> {
    let composer = composer::detect(root)?;
    check_composer(&composer)?;
    let mut sources = composer.sources.clone();
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
    parse(root, &path, &text, opts, &composer, sources)
}

/// pun.toml text that isn't written yet (`pun migrate`) as `load` resolves it, without composer.json and
/// composer.lock.
pub fn from_toml(root: &Path, text: &str) -> Result<Config> {
    parse(
        root,
        &root.join("pun.toml"),
        text,
        &Options::default(),
        &Composer::default(),
        vec![],
    )
}

fn parse(
    root: &Path,
    path: &Path,
    text: &str,
    opts: &Options,
    composer: &Composer,
    sources: Vec<PathBuf>,
) -> Result<Config> {
    let toml: PunToml =
        toml::from_str(text).with_context(|| format!("invalid {}", path.display()))?;
    let PunToml {
        php,
        files,
        format: f,
        lint,
        analyse,
        preset: tier,
    } = toml;
    let at = path.display();
    if let Some(tier) = tier {
        bail!(
            "{at}: the top-level `preset = {tier}` was removed: use `[lint] sets` for lint rules, `[analyse] preset` for analyse and `[format] preset` for format"
        );
    }
    if f.paths.is_some() {
        bail!("{at}: [format] paths moved to [files] paths, which every command shares");
    }
    match f.preset.as_deref() {
        Some(p @ ("per" | "symfony")) => bail!(
            "{at}: [format] preset = \"{p}\" was removed: use preset = \"empty\" with its rules under [format.rules] (`pun migrate` expands a pint.json that used it)"
        ),
        Some(p) if !FORMAT_PRESETS.contains(&p) => {
            bail!("{at}: [format] preset = \"{p}\": expected `laravel`, `psr12` or `empty`")
        }
        _ => {}
    }
    let php = match php.version {
        Some(v) if schema::version(&v).is_none() || v.split('.').count() != 2 => bail!(
            "{at}: [php] version = \"{v}\": expected a PHP version as major.minor, such as \"8.4\""
        ),
        Some(v) => v,
        None => composer.php_version().unwrap_or_else(|| "8.4".into()),
    };
    let files = Files::from(files);
    let gates = schema::Gates {
        php: schema::version(&php).unwrap(),
        composer,
    };
    let lint = schema::lint(path, lint, &files, &gates)?;
    let analyse = schema::analyse(path, analyse, &files, &gates)?;

    let preset = opts
        .preset
        .clone()
        .or(f.preset)
        .unwrap_or_else(|| "laravel".into());
    let mut rules = resolve_set(preset_rules(&preset).context("Preset not found.")?)?;
    for (name, v) in f.rules {
        let id = schema::rule_id(path, "format", &name)?;
        // Every format rule is a PHP-CS-Fixer or Pint fixer (rules/mapping.toml).
        let fixer = fixer_of("format", &id).unwrap();
        let v = options::to_fixer(&name, &id, v).map_err(|e| anyhow::anyhow!("{at}: {e}"))?;
        rules.insert(fixer.into(), v);
    }
    if opts.blade {
        rules.insert("Pint/laravel_blade".into(), Value::Bool(true));
    }
    // Risky fixers are lint rules: a preset's run only under `[lint]`.
    rules.retain(|name, _| !rule_of(name).is_some_and(|id| id.starts_with("lint/")));
    let rules = enabled(rules);
    let blade = rules.iter().any(|r| r.name == "Pint/laravel_blade");

    Ok(Config {
        root: root.to_path_buf(),
        preset,
        php,
        rules,
        finder: Finder {
            in_dirs: files.paths.clone(),
            exclude: [&files.exclude[..], &f.exclude[..]].concat(),
            not_name: f.not_name,
            not_path: f.not_path,
        },
        blade,
        files,
        lint,
        analyse,
        sources,
    })
}

/// The Pun rule ID behind a name from before the `<command>/<group>/<name>` IDs: an upstream rule (`array_syntax`,
/// `Pint/laravel_blade`, `StrictInArray`, `method.notFound`, Mago's `no-empty`) under any casing or separators (the
/// earlier auto-derived names were kebab-cased upstream names: `final-class`, `simplify-mirror-assign`), and the old
/// `format.` / `lint.` / `analyse.` names (`format.array-syntax`, `no-alias-function`, `undefined-method`).
fn renamed(name: &str) -> Option<&'static str> {
    let bare = ["format.", "lint.", "analyse."]
        .iter()
        .find_map(|p| name.strip_prefix(p))
        .unwrap_or(name);
    let norm = |s: &str| -> String {
        s.chars()
            .filter(char::is_ascii_alphanumeric)
            .map(|c| c.to_ascii_lowercase())
            .collect()
    };
    let key = norm(bare);
    if let Some((_, id)) = upstream().iter().find(|(k, _)| norm(k) == key) {
        return Some(id);
    }
    // The old name is the rule's name, its group and name joined (`undefined-method`), or its name with the group
    // word in it (`no-redundant-pest-uses` is `pest/no-redundant-uses`).
    let mut hits = rules().iter().map(|(id, _)| id.as_str()).filter(|id| {
        let mut parts = id.splitn(3, '/').skip(1);
        let (group, rule) = (parts.next().unwrap(), parts.next().unwrap());
        [
            bare.to_string(),
            bare.replacen(&format!("{group}-"), "", 1),
            bare.replacen(&format!("-{group}"), "", 1),
        ]
        .contains(&rule.to_string())
            || format!("{group}-{rule}") == bare
    });
    hits.next().filter(|_| hits.next().is_none())
}

/// Every `[upstream.<tool>]` entry that maps to a rule: upstream key => Pun rule ID. Only an error needs it.
fn upstream() -> &'static Vec<(String, &'static str)> {
    static T: LazyLock<Vec<(String, &'static str)>> = LazyLock::new(|| {
        let mut out = vec![];
        for part in MAPPING.split("\n[upstream.").skip(1) {
            let body = part.split_once("]\n").unwrap().1;
            for (k, e) in toml::from_str::<toml::Table>(body).unwrap() {
                if let Some(id) = e.get("pun").and_then(toml::Value::as_str) {
                    let id = rules()
                        .iter()
                        .find(|(r, _)| r == id)
                        .map(|(r, _)| r.as_str());
                    out.extend(id.map(|id| (k, id)));
                }
            }
        }
        out
    });
    &T
}

/// A Pun rule's metadata from `[rules.<command>]` of rules/mapping.toml.
#[derive(Debug)]
pub struct RuleMeta {
    /// `safe`, `unsafe`, `mixed` or `none` (analyse rules have no fix).
    pub fix: &'static str,
    pub summary: Option<&'static str>,
}

/// Every `[rules.format]`, `[rules.lint]` and `[rules.analyse]` entry by rule ID (`lint/style/no-alias-function`).
/// Parsed on first use: a plain `pun format` run never needs it.
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
                    .map(move |(rule, e)| (format!("{c}/{rule}"), e))
            })
            .collect()
    });
    &T
}

/// The metadata of a Pun rule ID (`lint/dead-code/no-self-assignment`).
pub fn rule(id: &str) -> Option<RuleMeta> {
    let (_, e) = rules().iter().find(|(r, _)| r == id)?;
    let str = |k| e.get(k).and_then(toml::Value::as_str);
    Some(RuleMeta {
        fix: str("fix").unwrap_or("none"),
        summary: str("summary"),
    })
}

/// A lint or analyse rule's options as PHP-CS-Fixer takes them: `true`/`false`, or options with snake_case keys.
fn options(path: &Path, rule: &str, v: toml::Value) -> Result<Value> {
    Ok(match v {
        toml::Value::Boolean(b) => Value::Bool(b),
        toml::Value::Table(t) => Value::Object(
            t.into_iter()
                .map(|(k, v)| (k.replace('-', "_"), serde_json::to_value(v).unwrap()))
                .collect(),
        ),
        _ => bail!(
            "{}: `{rule}` must be true, false or a table of options",
            path.display()
        ),
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
fn check_composer(composer: &Composer) -> Result<()> {
    let Some(php) = &composer.php else {
        return Ok(());
    };
    // PUN_IGNORE_PHP_CONSTRAINT=1: harness-only escape hatch so pre-8.4 corpora (laravel/framework...) can be diffed.
    if php
        .lowest
        .is_some_and(|l| l < composer::Version::from((8, 4, 0)))
        && std::env::var_os("PUN_IGNORE_PHP_CONSTRAINT").is_none()
    {
        bail!(
            "composer.json requires PHP \"{}\", which allows PHP < 8.4. Pun only supports projects on PHP 8.4+; raise the constraint (for example \"^8.4\") or keep using Pint.",
            php.constraint
        );
    }
    Ok(())
}

impl Config {
    /// `pun config`: the resolved pun.toml, in its own sections and keys. Every rule that runs is listed under
    /// `[<command>.rules]` as pun.toml writes it (`<group>/<name>`; a rule with no Pun ID keeps its upstream name), each
    /// command's `exclude` includes `[files] exclude`, and `ecosystems` and `switches` show what composer.lock and the
    /// analyse preset turned on.
    pub fn show(&self) -> Value {
        let short = |id: &str| id.split_once('/').unwrap().1.to_string();
        let format: Map<String, Value> = (self.rules.iter())
            .map(|r| {
                let full = rule_of(&r.name).unwrap_or(&r.name);
                // Options came through `to_fixer` or from a preset, so they translate back.
                let v = options::to_pun(&r.name, full, &r.options).unwrap();
                (rule_of(&r.name).map_or(r.name.clone(), short), v)
            })
            .collect();
        let lint: Map<String, Value> = (self.lint.rules.iter())
            .map(|r| (short(&r.id), r.value()))
            .collect();
        let overrides: Vec<Value> = (self.lint.overrides.iter())
            .map(|o| {
                let rules: Map<String, Value> =
                    o.rules.iter().map(|r| (short(&r.id), r.value())).collect();
                serde_json::json!({ "files": o.files, "rules": rules })
            })
            .collect();
        let analyse: Map<String, Value> = (self.analyse.rules.iter())
            .map(|r| {
                let v = match &r.options {
                    Value::Null => Value::from("on"),
                    o => o.clone(),
                };
                (short(&r.id), v)
            })
            .collect();
        let a = &self.analyse;
        let mut analyse = serde_json::json!({
            "preset": a.preset, "paths": a.paths, "exclude": a.exclude, "switches": a.switches,
            "ecosystems": a.ecosystems, "rules": analyse,
        });
        // TOML has no null: no baseline is no key.
        if let Some(b) = &a.baseline {
            analyse["baseline"] = b.as_str().into();
        }
        let f = &self.finder;
        serde_json::json!({
            "php": { "version": self.php },
            "files": self.files,
            "format": {
                "preset": self.preset, "exclude": f.exclude, "not-name": f.not_name, "not-path": f.not_path,
                "rules": format,
            },
            "lint": {
                "sets": self.lint.sets, "exclude": self.lint.exclude, "ecosystems": self.lint.ecosystems,
                "rules": lint, "overrides": overrides,
            },
            "analyse": analyse,
        })
    }
}

#[cfg(test)]
mod tests;
