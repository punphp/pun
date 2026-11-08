//! Minimal port of `PhpCsFixer\FixerConfiguration\FixerConfigurationResolver`: option-name
//! validation plus typed getters with upstream defaults. Allowed values/types are checked by
//! each fixer's `configure` (it knows the upstream `setAllowedValues`/`setAllowedTypes`).
use anyhow::{Result, bail};
use serde_json::{Map, Value};

pub(crate) struct Resolved {
    fixer: &'static str,
    map: Map<String, Value>,
}

/// The variant paired with `value`, which `string`/`strings` already checked against the names.
fn variant<T: Copy>(choices: &[(&str, T)], value: &str) -> T {
    choices.iter().find(|&&(name, _)| name == value).unwrap().1
}

/// `Null` (rule enabled with `true`) means "all defaults". Unknown keys are rejected like upstream.
pub(crate) fn resolve(fixer: &'static str, options: &Value, defined: &[&str]) -> Result<Resolved> {
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
        def.sort_unstable();
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

/// A PHP array key: json_decode turns a canonical decimal-integer string key that fits an int into an int key.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum PhpKey<'a> {
    Int(i64),
    Str(&'a str),
}

impl std::fmt::Display for PhpKey<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PhpKey::Int(i) => write!(f, "{i}"),
            PhpKey::Str(s) => f.write_str(s),
        }
    }
}

impl PhpKey<'_> {
    /// `gettype($key).'#'.$key`, as upstream's option validators print a key.
    pub(crate) fn typed(&self) -> String {
        match self {
            PhpKey::Int(i) => format!("integer#{i}"),
            PhpKey::Str(s) => format!("string#{s}"),
        }
    }
}

/// The entries of a value that is a PHP array (a JSON list keyed 0..n, or an object), in order. Panics on any other
/// value, so call it on options `check_types` has accepted as arrays.
pub(crate) fn php_array(v: &Value) -> Vec<(PhpKey<'_>, &Value)> {
    match v {
        Value::Object(o) => o
            .iter()
            .map(|(k, v)| {
                let key = match k.parse::<i64>() {
                    Ok(i) if i.to_string() == *k => PhpKey::Int(i),
                    _ => PhpKey::Str(k),
                };
                (key, v)
            })
            .collect(),
        _ => v
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, v)| (PhpKey::Int(i as i64), v))
            .collect(),
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
    pub(crate) fn type_error(&self, key: &str, v: &Value, expected: &str) -> anyhow::Error {
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
    pub(crate) fn invalid(&self, key: &str, v: &Value, allowed: &[&str]) -> anyhow::Error {
        self.invalid_listed(key, v, allowed, false)
    }

    /// `setAllowedValues` failure for a nullable option: the list ends with `null`.
    pub(crate) fn invalid_or_null(&self, key: &str, v: &Value, allowed: &[&str]) -> anyhow::Error {
        self.invalid_listed(key, v, allowed, true)
    }

    fn invalid_listed(&self, key: &str, v: &Value, allowed: &[&str], null: bool) -> anyhow::Error {
        if allowed.is_empty() {
            return self.error(key, v, "is invalid");
        }
        let mut list: Vec<String> = allowed.iter().map(|a| format!("\"{a}\"")).collect();
        if null {
            list.push("null".into());
        }
        self.error(
            key,
            v,
            &format!("is invalid. Accepted values are: {}", list.join(", ")),
        )
    }

    /// `php_array` of an option `check_types` accepted as an array; `None` when it is not set.
    pub(crate) fn php_array(&self, key: &str) -> Option<Vec<(PhpKey<'_>, &Value)>> {
        self.map.get(key).map(php_array)
    }

    pub(crate) fn get(&self, key: &str) -> Option<&Value> {
        self.map.get(key)
    }

    pub(crate) fn bool(&self, key: &str, default: bool) -> Result<bool> {
        match self.map.get(key) {
            None => Ok(default),
            Some(Value::Bool(b)) => Ok(*b),
            Some(v) => Err(self.type_error(key, v, "\"bool\"")),
        }
    }

    /// A string restricted to `allowed` (`setAllowedValues`), or any string when `allowed` is empty (`setAllowedTypes(['string'])`).
    pub(crate) fn string(&self, key: &str, default: &str, allowed: &[&str]) -> Result<String> {
        match self.map.get(key) {
            None => Ok(default.to_string()),
            Some(Value::String(s)) if allowed.is_empty() || allowed.contains(&s.as_str()) => {
                Ok(s.clone())
            }
            Some(v) if allowed.is_empty() => Err(self.type_error(key, v, "\"string\"")),
            Some(v) => Err(self.invalid(key, v, allowed)),
        }
    }

    /// `string` for an option whose allowed values stand for enum variants: the variant paired with the value.
    /// `choices` keeps upstream's order of allowed values, which the error message lists.
    pub(crate) fn choice<T: Copy>(
        &self,
        key: &str,
        default: &str,
        choices: &[(&str, T)],
    ) -> Result<T> {
        let allowed: Vec<&str> = choices.iter().map(|&(name, _)| name).collect();
        let value = self.string(key, default, &allowed)?;
        Ok(variant(choices, &value))
    }

    /// `strings` restricted to `choices`: the variant of each configured value, in order.
    pub(crate) fn choices<T: Copy>(
        &self,
        key: &str,
        default: &[&str],
        choices: &[(&str, T)],
    ) -> Result<Vec<T>> {
        let allowed: Vec<&str> = choices.iter().map(|&(name, _)| name).collect();
        let values = self.strings(key, default, &allowed)?;
        Ok(values.iter().map(|v| variant(choices, v)).collect())
    }

    /// `bool|null` options (e.g. yoda_style's `equal`).
    pub(crate) fn opt_bool(&self, key: &str, default: Option<bool>) -> Result<Option<bool>> {
        match self.map.get(key) {
            None => Ok(default),
            Some(Value::Null) => Ok(None),
            Some(Value::Bool(b)) => Ok(Some(*b)),
            Some(v) => Err(self.type_error(key, v, "\"bool\" or \"null\"")),
        }
    }

    /// A `string[]`, each item in `allowed` (an `AllowedValueSubset`; empty = any). JSON objects count as PHP arrays.
    pub(crate) fn strings(
        &self,
        key: &str,
        default: &[&str],
        allowed: &[&str],
    ) -> Result<Vec<String>> {
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
    pub(crate) fn check_types(&self, key: &str, types: &[&str]) -> Result<()> {
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
            " one of the elements"
        } else {
            ""
        };
        Err(self.error(
            key,
            v,
            &format!(
                "is expected to be of type \"{expected}\", but{element} is of type \"{provided}\""
            ),
        ))
    }
}

fn verify_types(type_: &str, v: &Value, invalid: &mut Vec<&'static str>, level: usize) -> bool {
    let items: Option<Vec<&Value>> = match v {
        Value::Array(a) => Some(a.iter().collect()),
        Value::Object(o) => Some(o.values().collect()),
        _ => None,
    };
    if let (Some(inner), Some(items)) = (type_.strip_suffix("[]"), items) {
        #[expect(
            clippy::unnecessary_fold,
            reason = "no short-circuit: every element records its invalid type"
        )]
        return items.into_iter().fold(true, |valid, item| {
            verify_types(inner, item, invalid, level + 1) && valid
        });
    }
    let ok = match type_ {
        "int" => v.is_i64(),
        "string" => v.is_string(),
        "bool" => v.is_boolean(),
        "null" => v.is_null(),
        "array" => v.is_array() || v.is_object(),
        _ => false,
    };
    if !ok && (invalid.is_empty() || level > 0) && !invalid.contains(&debug_type(v)) {
        invalid.push(debug_type(v));
    }
    ok
}
