//! Minimal port of `PhpCsFixer\FixerConfiguration\FixerConfigurationResolver`: option-name
//! validation plus typed getters with upstream defaults. Allowed values/types are checked by
//! each fixer's `configure` (it knows the upstream `setAllowedValues`/`setAllowedTypes`).
use anyhow::{Result, bail};
use serde_json::{Map, Value};

pub struct Resolved {
    fixer: &'static str,
    map: Map<String, Value>,
}

/// `Null` (rule enabled with `true`) and `[]` mean "all defaults". Unknown keys are rejected like upstream.
pub fn resolve(fixer: &'static str, options: &Value, defined: &[&str]) -> Result<Resolved> {
    let map = match options {
        Value::Null => Map::new(),
        Value::Array(a) if a.is_empty() => Map::new(),
        Value::Object(o) => o.clone(),
        _ => bail!("[{fixer}] Invalid configuration: expected an array of options."),
    };
    let mut unknown: Vec<&String> = map.keys().filter(|k| !defined.contains(&k.as_str())).collect();
    if !unknown.is_empty() {
        unknown.sort();
        let names = |v: Vec<&str>| v.iter().map(|s| format!("\"{s}\"")).collect::<Vec<_>>().join(", ");
        let mut def = defined.to_vec();
        def.sort();
        bail!(
            "[{fixer}] Invalid configuration: The option{} {} do{} not exist. Defined options are: {}.",
            if unknown.len() > 1 { "s" } else { "" },
            names(unknown.iter().map(|s| s.as_str()).collect()),
            if unknown.len() > 1 { "" } else { "es" },
            names(def)
        );
    }
    Ok(Resolved { fixer, map })
}

impl Resolved {
    fn invalid(&self, key: &str, v: &Value) -> anyhow::Error {
        anyhow::anyhow!("[{}] Invalid configuration: The option \"{key}\" with value {v} is invalid.", self.fixer)
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.map.get(key)
    }

    pub fn bool(&self, key: &str, default: bool) -> Result<bool> {
        match self.map.get(key) {
            None => Ok(default),
            Some(Value::Bool(b)) => Ok(*b),
            Some(v) => Err(self.invalid(key, v)),
        }
    }

    /// A string restricted to `allowed` (empty = any string).
    pub fn string(&self, key: &str, default: &str, allowed: &[&str]) -> Result<String> {
        match self.map.get(key) {
            None => Ok(default.to_string()),
            Some(Value::String(s)) if allowed.is_empty() || allowed.contains(&s.as_str()) => Ok(s.clone()),
            Some(v) => Err(self.invalid(key, v)),
        }
    }

    /// `bool|null` options (e.g. yoda_style's `equal`).
    pub fn opt_bool(&self, key: &str, default: Option<bool>) -> Result<Option<bool>> {
        match self.map.get(key) {
            None => Ok(default),
            Some(Value::Null) => Ok(None),
            Some(Value::Bool(b)) => Ok(Some(*b)),
            Some(v) => Err(self.invalid(key, v)),
        }
    }

    /// A list of strings, each restricted to `allowed` (empty = any). JSON objects count as PHP arrays (values taken).
    pub fn strings(&self, key: &str, default: &[&str], allowed: &[&str]) -> Result<Vec<String>> {
        let items: Vec<&Value> = match self.map.get(key) {
            None => return Ok(default.iter().map(|s| s.to_string()).collect()),
            Some(Value::Array(a)) => a.iter().collect(),
            Some(Value::Object(o)) => o.values().collect(),
            Some(v) => return Err(self.invalid(key, v)),
        };
        items
            .into_iter()
            .map(|v| match v {
                Value::String(s) if allowed.is_empty() || allowed.contains(&s.as_str()) => Ok(s.clone()),
                v => Err(self.invalid(key, v)),
            })
            .collect()
    }
}
