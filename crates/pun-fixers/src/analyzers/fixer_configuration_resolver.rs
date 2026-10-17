//! Minimal port of `PhpCsFixer\FixerConfiguration\FixerConfigurationResolver`: option-name
//! validation plus typed getters with upstream defaults. Allowed values/types are checked by
//! each fixer's `configure` (it knows the upstream `setAllowedValues`/`setAllowedTypes`).
use anyhow::{Result, bail};
use serde_json::{Map, Value};

pub struct Resolved {
    fixer: &'static str,
    map: Map<String, Value>,
}

/// `Null` (rule enabled with `true`) means "all defaults". Unknown keys are rejected like upstream.
pub fn resolve(fixer: &'static str, options: &Value, defined: &[&str]) -> Result<Resolved> {
    // RuleSet / FixerFactory::useRuleSet checks, done before configure() upstream.
    let map = match options {
        Value::Null => Map::new(),
        Value::Object(o) if !o.is_empty() => o.clone(),
        Value::Array(a) if !a.is_empty() => a
            .iter()
            .enumerate()
            .map(|(i, v)| (i.to_string(), v.clone()))
            .collect(),
        Value::Array(_) | Value::Object(_) => {
            bail!("[{fixer}] Configuration must be an array and may not be empty.")
        }
        _ => bail!(
            "[{fixer}] Rule must be enabled (true), disabled (false) or configured (non-empty, assoc array). Other values are not allowed."
        ),
    };
    let mut unknown: Vec<&String> = map
        .keys()
        .filter(|k| !defined.contains(&k.as_str()))
        .collect();
    if !unknown.is_empty() {
        unknown.sort();
        let names = |v: Vec<&str>| {
            v.iter()
                .map(|s| format!("\"{s}\""))
                .collect::<Vec<_>>()
                .join(", ")
        };
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

/// Symfony OptionsResolver's `formatValue`: strings quoted, arrays as `array`.
fn format_value(v: &Value) -> String {
    match v {
        Value::String(s) => format!("\"{s}\""),
        Value::Array(_) | Value::Object(_) => "array".into(),
        // json_decode makes a number beyond the int range a float.
        Value::Number(n) if !n.is_i64() => {
            pun_tokens::php_compat::float_to_string(n.as_f64().unwrap())
        }
        v => v.to_string(),
    }
}

/// PHP `get_debug_type` of a decoded JSON value.
fn debug_type(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(n) if !n.is_i64() => "float",
        Value::Number(_) => "int",
        Value::String(_) => "string",
        Value::Array(_) | Value::Object(_) => "array",
    }
}

impl Resolved {
    fn error(&self, key: &str, v: &Value, problem: &str) -> anyhow::Error {
        anyhow::anyhow!(
            "[{}] Invalid configuration: The option \"{key}\" with value {} {problem}.",
            self.fixer,
            format_value(v)
        )
    }

    /// `setAllowedTypes` failure, e.g. `"bool" or "null"`.
    fn type_error(&self, key: &str, v: &Value, expected: &str) -> anyhow::Error {
        self.error(
            key,
            v,
            &format!(
                "is expected to be of type {expected}, but is of type \"{}\"",
                debug_type(v)
            ),
        )
    }

    /// `setAllowedValues` failure: the accepted values are listed unless the check is a closure (`allowed` empty).
    fn invalid(&self, key: &str, v: &Value, allowed: &[&str]) -> anyhow::Error {
        if allowed.is_empty() {
            return self.error(key, v, "is invalid");
        }
        let list = allowed
            .iter()
            .map(|a| format!("\"{a}\""))
            .collect::<Vec<_>>()
            .join(", ");
        self.error(key, v, &format!("is invalid. Accepted values are: {list}"))
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.map.get(key)
    }

    pub fn bool(&self, key: &str, default: bool) -> Result<bool> {
        match self.map.get(key) {
            None => Ok(default),
            Some(Value::Bool(b)) => Ok(*b),
            Some(v) => Err(self.type_error(key, v, "\"bool\"")),
        }
    }

    /// A string restricted to `allowed` (`setAllowedValues`), or any string when `allowed` is empty (`setAllowedTypes(['string'])`).
    pub fn string(&self, key: &str, default: &str, allowed: &[&str]) -> Result<String> {
        match self.map.get(key) {
            None => Ok(default.to_string()),
            Some(Value::String(s)) if allowed.is_empty() || allowed.contains(&s.as_str()) => {
                Ok(s.clone())
            }
            Some(v) if allowed.is_empty() => Err(self.type_error(key, v, "\"string\"")),
            Some(v) => Err(self.invalid(key, v, allowed)),
        }
    }

    /// `bool|null` options (e.g. yoda_style's `equal`).
    pub fn opt_bool(&self, key: &str, default: Option<bool>) -> Result<Option<bool>> {
        match self.map.get(key) {
            None => Ok(default),
            Some(Value::Null) => Ok(None),
            Some(Value::Bool(b)) => Ok(Some(*b)),
            Some(v) => Err(self.type_error(key, v, "\"bool\" or \"null\"")),
        }
    }

    /// A `string[]`, each item in `allowed` (an `AllowedValueSubset`; empty = any). JSON objects count as PHP arrays.
    pub fn strings(&self, key: &str, default: &[&str], allowed: &[&str]) -> Result<Vec<String>> {
        let v = match self.map.get(key) {
            None => return Ok(default.iter().map(|s| s.to_string()).collect()),
            Some(v) => v,
        };
        let items: Vec<&Value> = match v {
            Value::Array(a) => a.iter().collect(),
            Value::Object(o) => o.values().collect(),
            v => return Err(self.type_error(key, v, "\"string[]\"")),
        };
        if let Some(bad) = items.iter().find(|i| !i.is_string()) {
            return Err(self.error(key, v, &format!("is expected to be of type \"string[]\", but one of the elements is of type \"{}\"", debug_type(bad))));
        }
        let items: Vec<String> = items
            .iter()
            .map(|i| i.as_str().unwrap().to_string())
            .collect();
        if !allowed.is_empty() && items.iter().any(|i| !allowed.contains(&i.as_str())) {
            return Err(self.invalid(key, v, &[]));
        }
        Ok(items)
    }

    /// Symfony OptionsResolver's `setAllowedTypes` check (`verifyTypes`) for scalar/array types such as
    /// `["int", "int[]"]` or `["string[][]"]`; Ok when the option is absent or matches one of `types`.
    pub fn check_types(&self, key: &str, types: &[&str]) -> Result<()> {
        let Some(v) = self.map.get(key) else {
            return Ok(());
        };
        let mut invalid = vec![];
        if types.iter().any(|t| verify_types(t, v, &mut invalid, 0)) {
            return Ok(());
        }
        let (expected, provided) = (types.join("\" or \""), invalid.join("|"));
        let element = if (v.is_array() || v.is_object()) && types.iter().any(|t| t.ends_with("[]"))
        {
            "one of the elements"
        } else {
            ""
        };
        Err(if element.is_empty() {
            self.error(
                key,
                v,
                &format!("is expected to be of type \"{expected}\", but is of type \"{provided}\""),
            )
        } else {
            self.error(key, v, &format!("is expected to be of type \"{expected}\", but {element} is of type \"{provided}\""))
        })
    }
}

fn verify_types(type_: &str, v: &Value, invalid: &mut Vec<&'static str>, level: usize) -> bool {
    let items: Option<Vec<&Value>> = match v {
        Value::Array(a) => Some(a.iter().collect()),
        Value::Object(o) => Some(o.values().collect()),
        _ => None,
    };
    if let (Some(inner), Some(items)) = (type_.strip_suffix("[]"), items) {
        // No short-circuit: every element records its invalid type.
        #[allow(clippy::unnecessary_fold)]
        return items.into_iter().fold(true, |valid, item| {
            verify_types(inner, item, invalid, level + 1) && valid
        });
    }
    let ok = match type_ {
        "int" => v.is_i64(),
        "string" => v.is_string(),
        "bool" => v.is_boolean(),
        "null" => v.is_null(),
        _ => false,
    };
    if !ok && (invalid.is_empty() || level > 0) && !invalid.contains(&debug_type(v)) {
        invalid.push(debug_type(v));
    }
    ok
}
