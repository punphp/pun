//! `pun migrate`, format part: pint.json (read like Pint: `extend`, preset, RuleSet checks) or the rules of a
//! `.php-cs-fixer.php` become a pun.toml under Pun rule IDs. Risky fixers go to `[lint.rules]` at `"error"`, so
//! `pun check --fix` applies them as the project's old tool did; the rest is written as a delta to the preset, with
//! options under their Pun names (`options::to_pun`). Pint's `in` becomes `[files] paths`; its other finder keys stay
//! format-only under `[format]`.

use crate::{Rule, fixer_entries, preset_rules, rule_of, rule_set};
use anyhow::{Context, Result, bail};
use serde_json::{Map, Value};
use std::fmt::Write;
use std::path::Path;

pub struct Migration {
    pub toml: String,
    /// Rules Pun can't run, with the reason: the migration must not go ahead with these.
    pub unsupported: Vec<String>,
}

/// pint.json at `path` (a missing file is an empty config, as in Pint), with `--preset` winning over its preset.
pub fn from_pint(path: &Path, preset: Option<&str>) -> Result<Migration> {
    let pint = read_pint_json(path)?;
    let preset = preset
        .or_else(|| pint.get("preset").and_then(Value::as_str))
        .unwrap_or("laravel");
    let mut rules = preset_rules(preset).context("Preset not found.")?;
    // Pint: array_merge($preset, $pintJson['rules']), overrides keep the preset's position.
    if let Some(r) = pint.get("rules").and_then(Value::as_object) {
        rules.extend(r.clone());
    }
    // RuleSet::__construct: only true, false or an options array.
    for (name, v) in &rules {
        if !matches!(v, Value::Bool(_) | Value::Array(_) | Value::Object(_)) {
            let (what, kind) = if name.starts_with('@') {
                ("Set must be enabled (true) or disabled (false).", "set")
            } else {
                (
                    "Rule must be enabled (true), disabled (false) or configured (non-empty, assoc array).",
                    "rule",
                )
            };
            let null = if v.is_null() {
                format!(" To disable the {kind}, use \"FALSE\" instead of \"NULL\".")
            } else {
                String::new()
            };
            bail!("[{name}] {what} Other values are not allowed.{null}");
        }
    }
    let (mut paths, mut finder) = (String::new(), String::new());
    for (json, key) in [
        ("in", "paths"),
        ("exclude", "exclude"),
        ("notName", "not-name"),
        ("notPath", "not-path"),
    ] {
        let list: Vec<&str> = match pint.get(json) {
            Some(Value::String(s)) => vec![s],
            Some(Value::Array(a)) => a.iter().filter_map(Value::as_str).collect(),
            _ => continue,
        };
        let out = if json == "in" {
            &mut paths
        } else {
            &mut finder
        };
        writeln!(out, "{key} = {}", toml::Value::try_from(list).unwrap()).unwrap();
    }
    write(preset, rule_set(rules)?, &paths, &finder)
}

/// A `.php-cs-fixer.php`'s rules as its ConfigurationResolver resolves them, over preset `empty`.
pub fn from_rules(rules: Map<String, Value>) -> Result<Migration> {
    write("empty", rule_set(rules)?, "", "")
}

/// Pint's ConfigurationJsonRepository::get(): a missing file means "no config"; `extend` is one level deep.
fn read_pint_json(path: &Path) -> Result<Map<String, Value>> {
    if !path.exists() {
        return Ok(Map::new());
    }
    let invalid = || {
        format!(
            "The configuration file [{}] is not valid JSON.",
            path.display()
        )
    };
    let mut v: Value = serde_json::from_slice(&std::fs::read(path)?).with_context(invalid)?;
    if let Some(ext) = v.get("extend").and_then(Value::as_str) {
        let ext_path = path.parent().unwrap().join(ext);
        // file_get_contents(realpath(...)): a missing file is realpath's `false`.
        let raw =
            std::fs::read(&ext_path).map_err(|_| anyhow::anyhow!("Path must not be empty"))?;
        // json_decode then array_replace_recursive($base, ...): anything but an array is a TypeError.
        let base: Value = serde_json::from_slice(&raw).unwrap_or(Value::Null);
        let given = match &base {
            Value::Object(_) | Value::Array(_) => None,
            Value::Null => Some("null"),
            Value::Bool(true) => Some("true"),
            Value::Bool(false) => Some("false"),
            Value::Number(n) if n.is_i64() => Some("int"),
            Value::Number(_) => Some("float"),
            Value::String(_) => Some("string"),
        };
        if let Some(t) = given {
            bail!(
                "array_replace_recursive(): Argument #1 ($array) must be of type array, {t} given"
            );
        }
        if base.get("extend").is_some() {
            bail!("Pint configuration cannot extend from more than 1 file.");
        }
        let mut merged = base;
        replace_recursive(&mut merged, v);
        v = merged;
    }
    match v {
        Value::Object(m) => Ok(m),
        // json_decode(..., true) makes a list a PHP array with only int keys, so no setting is found.
        Value::Array(_) => Ok(Map::new()),
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

/// pun.toml for `rules` (what the old config ran, resolved) over `preset`: `[format.rules]` holds what differs from
/// the preset, `[lint.rules]` every risky fixer. A preset pun.toml doesn't take (`per`, `symfony`) is written as
/// `empty` plus every rule it ran.
fn write(preset: &str, mut rules: Vec<Rule>, paths: &str, finder: &str) -> Result<Migration> {
    let preset = if crate::FORMAT_PRESETS.contains(&preset) {
        preset
    } else {
        "empty"
    };
    // Pun runs the deprecated visibility_required as modifier_keywords (as upstream's proxy does). Both on: one
    // modifier_keywords for the union of their `elements` (no options = all elements).
    if let Some(i) = rules.iter().position(|r| r.name == "visibility_required") {
        let vr = rules.remove(i).options;
        match rules.iter_mut().find(|r| r.name == "modifier_keywords") {
            None => rules.push(Rule {
                name: "modifier_keywords".into(),
                options: vr,
            }),
            Some(mk) if vr.is_null() || mk.options.is_null() => mk.options = Value::Null,
            Some(mk) => {
                let mut all = mk.options["elements"]
                    .as_array()
                    .cloned()
                    .unwrap_or_default();
                for e in vr["elements"].as_array().into_iter().flatten() {
                    if !all.contains(e) {
                        all.push(e.clone());
                    }
                }
                mk.options = serde_json::json!({ "elements": all });
            }
        }
    }
    let base = rule_set(preset_rules(preset).unwrap())?;
    let (mut format, mut lints, mut unsupported) = (String::new(), String::new(), vec![]);
    for r in &rules {
        let id = rule_of(&r.name);
        let lint = id.is_some_and(|id| id.starts_with("lint/"));
        // The preset brings it (rules Pun can't run, such as sets it can't expand, included).
        if !lint
            && base
                .iter()
                .any(|b| b.name == r.name && b.options == r.options)
        {
            continue;
        }
        let Some(id) = id else {
            let entry = fixer_entries().get(&r.name);
            let why = entry
                .and_then(|e| e.get("drop").or(e.get("unsupported")))
                .and_then(toml::Value::as_str)
                .unwrap_or(
                    "not a PHP-CS-Fixer or Pint rule Pun knows (a custom fixer or rule set)",
                );
            unsupported.push(format!("{}: {why}", r.name));
            continue;
        };
        let name = &id[id.find('/').unwrap() + 1..];
        let v = match (lint, value(&r.options)) {
            (true, Value::Bool(true)) => Value::from("error"),
            (true, v) => v,
            (false, _) => {
                crate::options::to_pun(&r.name, id, &r.options).map_err(anyhow::Error::msg)?
            }
        };
        let Some(v) = toml_value(&v) else {
            unsupported.push(format!(
                "{}: an option is null, which TOML can't hold",
                r.name
            ));
            continue;
        };
        writeln!(
            if lint { &mut lints } else { &mut format },
            "\"{name}\" = {v}"
        )
        .unwrap();
    }
    for b in &base {
        if let Some(name) = rule_of(&b.name).and_then(|id| id.strip_prefix("format/"))
            && !rules.iter().any(|r| r.name == b.name)
        {
            writeln!(format, "\"{name}\" = false").unwrap();
        }
    }
    let mut toml = String::new();
    if !paths.is_empty() {
        toml += &format!("[files]\n{paths}\n");
    }
    toml += &format!("[format]\npreset = \"{preset}\"\n{finder}");
    if !format.is_empty() {
        toml += &format!("\n[format.rules]\n{format}");
    }
    if !lints.is_empty() {
        toml += &format!("\n[lint.rules]\n{lints}");
    }
    Ok(Migration { toml, unsupported })
}

/// A rule's options as pun.toml writes them: `true` for none, top-level option keys kebab-cased.
pub fn value(options: &Value) -> Value {
    match options {
        Value::Object(m) => Value::Object(
            m.iter()
                .map(|(k, v)| (k.replace('_', "-"), v.clone()))
                .collect(),
        ),
        _ => Value::Bool(true),
    }
}

/// `v` as an inline TOML value; `None` when it holds a null.
fn toml_value(v: &Value) -> Option<String> {
    serde_json::from_value::<toml::Value>(v.clone())
        .ok()
        .map(|t| t.to_string())
}
