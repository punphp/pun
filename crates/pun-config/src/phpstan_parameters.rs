//! `[analyse]` as the PHPStan parameters pun-analyse reads: the level the enabled rules add up to, the parameters
//! that gate a rule class (on when its rule is), the options `pun migrate` writes from changed parameters
//! (mapping.toml `[phpstan-parameters]`), `stubFiles` and `ignoreErrors` from `stubs` and `[[analyse.ignore]]`, and
//! Larastan when Laravel is locked at a version it supports.

use crate::composer::Version;
use crate::{Analyse, mapping};
use pun_phpstan::php::normalize_path;
use pun_phpstan::{Array, Key, Value};
use std::collections::HashSet;
use std::path::Path;

/// The PHPStan level each type switch stands for (mapping.toml `[upstream.phpstan-parameters]`).
const SWITCH_LEVELS: [(&str, u8); 4] = [
    ("union-types", 7),
    ("nullables", 8),
    ("explicit-mixed", 9),
    ("implicit-mixed", 10),
];

/// Whether Larastan (3.x, the version Pun ports) installs beside a locked Laravel: `^11.44.2 || ^12.4.1 || ^13`. A
/// branch (`dev-main`) can't be told apart, so it counts.
pub(crate) fn larastan_supports(laravel: Option<Version>) -> bool {
    laravel.is_none_or(|v| match v.major {
        11 => v.minor >= 44 && (v.minor > 44 || v.patch >= 2),
        12 => v.minor > 4 || (v.minor == 4 && v.patch >= 1),
        13 => true,
        _ => false,
    })
}

impl Analyse {
    fn on(&self) -> HashSet<&str> {
        self.rules.iter().map(|r| r.id.as_str()).collect()
    }

    /// The PHPStan level of this config: the highest level of a core rule class (`[upstream.phpstan-classes]`) whose
    /// rule is on, or of a type switch.
    pub fn level(&self) -> u8 {
        let on = self.on();
        let classes = mapping::section(&["upstream", "phpstan-classes"]).table();
        let rules = (classes.values())
            .filter(|e| e.get("package").and_then(toml::Value::as_str) == Some("phpstan"))
            .filter(|e| {
                (e.get("pun").and_then(toml::Value::as_str)).is_some_and(|p| on.contains(p))
            })
            .filter_map(|e| e.get("level")?.as_integer());
        let switches = (SWITCH_LEVELS.iter())
            .filter(|(s, _)| self.switches.contains(s))
            .map(|(_, l)| i64::from(*l));
        rules.chain(switches).max().unwrap_or(0) as u8
    }

    /// The PHPStan parameters pun-analyse's `Project::from_config` reads, besides paths and the PHP version.
    pub fn parameters(&self, root: &Path) -> Array {
        let on = self.on();
        let mut p = Array::new();
        let level = self.level();
        p.set(Key::from("level"), Value::Int(i64::from(level)));
        // Pun is a CLI binary, with Project::new's argc/argv registration; never read php.ini by running PHP.
        p.set(
            Key::from("cliArgumentsVariablesRegistered"),
            Value::Bool(true),
        );
        // config.level1.neon turns it on; an explicit `maybe` option below overrides the level's default.
        p.set(
            Key::from("checkMaybeUndefinedVariables"),
            Value::Bool(level >= 1),
        );
        // config.level2.neon turns it on
        p.set(
            Key::from("checkClassCaseSensitivity"),
            Value::Bool(level >= 2),
        );
        p.set(Key::from("reportUnmatchedIgnoredErrors"), Value::Bool(true));
        let stubs = self.stubs.iter().map(|s| Value::str(absolute(root, s)));
        p.set(Key::from("stubFiles"), Value::Array(Array::list(stubs)));
        let ignores = self.ignore.iter().map(|i| Value::Array(ignore(root, i)));
        p.set(
            Key::from("ignoreErrors"),
            Value::Array(Array::list(ignores)),
        );
        // A parameter gating a rule class is on when one of the rules it gates is.
        let classes = mapping::section(&["upstream", "phpstan-classes"]).table();
        for e in classes.values() {
            let (Some(gate), Some(pun)) = (
                e.get("gated_by").and_then(toml::Value::as_str),
                e.get("pun").and_then(toml::Value::as_str),
            ) else {
                continue;
            };
            if e.get("package").and_then(toml::Value::as_str) == Some("larastan") && !self.larastan
            {
                continue;
            }
            for part in gate.split(" + ") {
                let (name, note) = part.split_once(" (").unwrap_or((part, ""));
                let name = match (name, note.strip_prefix("featureToggles.")) {
                    ("bleedingEdge", Some(t)) => {
                        format!("featureToggles.{}", t.trim_end_matches(')'))
                    }
                    ("bleedingEdge", None) => "featureToggles.bleedingEdge".into(),
                    _ => name.to_string(),
                };
                if get(&p, &name) != Some(&Value::Bool(true)) {
                    set(&mut p, &name, Value::Bool(on.contains(pun)));
                }
            }
        }
        // phpstan-deprecation-rules' rules.neon sets it: included when one of its rules is on.
        let deprecations = classes.values().any(|e| {
            e.get("package").and_then(toml::Value::as_str) == Some("phpstan-deprecation-rules")
                && (e.get("pun").and_then(toml::Value::as_str)).is_some_and(|p| on.contains(p))
        });
        p.set(
            Key::from("deprecationRulesInstalled"),
            Value::Bool(deprecations),
        );
        // An option `pun migrate` wrote from a changed parameter, on a rule that runs.
        for (name, e) in mapping::section(&["phpstan-parameters"]).table() {
            let (Some(option), Some(rules)) = (
                e.get("option").and_then(toml::Value::as_str),
                e.get("rules").and_then(toml::Value::as_array),
            ) else {
                continue;
            };
            let value = (self.rules.iter())
                .filter(|r| rules.iter().any(|x| x.as_str() == Some(r.id.as_str())))
                .find_map(|r| r.options.get(option));
            if let Some(v) = value {
                set(&mut p, name, json(v));
            }
        }
        p
    }
}

/// A path of pun.toml (relative to the project) as PHPStan resolves a config's relative path; a `*` pattern stays.
fn absolute(root: &Path, path: &str) -> String {
    match path.starts_with('*') {
        true => path.to_string(),
        false => normalize_path(&root.join(path).to_string_lossy()),
    }
}

/// One `[[analyse.ignore]]` as an `ignoreErrors` entry: one path is `path` (a file entry, reported as one), several
/// are `paths`; `rule` is Pun's own key.
fn ignore(root: &Path, i: &crate::Ignore) -> Array {
    let mut e = Array::new();
    let mut put = |k: &str, v: Option<&String>| {
        if let Some(v) = v {
            e.set(Key::from(k), Value::str(v.as_str()));
        }
    };
    put("message", i.message.as_ref());
    put("rawMessage", i.raw_message.as_ref());
    put("identifier", i.identifier.as_ref());
    put("rule", i.rule.as_ref());
    let paths: Vec<String> = i.paths.iter().map(|p| absolute(root, p)).collect();
    match paths.as_slice() {
        [] => {}
        [one] => e.set(Key::from("path"), Value::str(one.as_str())),
        many => e.set(
            Key::from("paths"),
            Value::Array(Array::list(many.iter().map(|p| Value::str(p.as_str())))),
        ),
    }
    if let Some(c) = i.count {
        e.set(Key::from("count"), Value::Int(i64::from(c)));
    }
    e
}

/// A rule option (JSON from pun.toml) as a parameter value.
// ponytail: every [phpstan-parameters] option is a bool, a string or a list of strings; a number reads as null
fn json(v: &serde_json::Value) -> Value {
    match v {
        serde_json::Value::String(s) => Value::str(s.as_str()),
        serde_json::Value::Array(a) => Value::Array(Array::list(a.iter().map(json))),
        v => v.as_bool().map_or(Value::Null, Value::Bool),
    }
}

fn get<'a>(p: &'a Array, dotted: &str) -> Option<&'a Value> {
    let mut parts = dotted.split('.');
    let mut v = p.get_str(parts.next()?)?;
    for k in parts {
        v = v.as_array()?.get_str(k)?;
    }
    Some(v)
}

/// Sets `a.b.c`, making the tables on the way.
fn set(p: &mut Array, dotted: &str, value: Value) {
    match dotted.split_once('.') {
        None => p.set(Key::from(dotted), value),
        Some((head, rest)) => {
            let mut inner = match p.get_str(head) {
                Some(Value::Array(a)) => a.clone(),
                _ => Array::new(),
            };
            set(&mut inner, rest, value);
            p.set(Key::from(head), Value::Array(inner));
        }
    }
}
