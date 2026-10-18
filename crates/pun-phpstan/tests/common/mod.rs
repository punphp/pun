#![allow(dead_code, reason = "each test binary uses a subset")]

use pun_phpstan::{Array, Entity, Key, Value};
use serde_json::{Value as J, json};

/// Tagged JSON written by scripts/export-neon-fixtures.php and scripts/phpstan-config.php.
pub fn tagged(v: &Value) -> J {
    match v {
        Value::Null => J::Null,
        Value::Bool(b) => json!(b),
        Value::Int(i) => json!(i),
        Value::Float(f) => json!({ "float": pun_phpstan::value::float_var_export(*f) }),
        Value::Str(s) => json!(s),
        Value::Array(a) => json!({ "array": items(a) }),
        Value::Entity(e) => {
            json!({ "entity": tagged(&e.value), "attributes": tagged(&Value::Array(e.attributes.clone())) })
        }
        Value::DateTime(raw) => {
            json!({ "datetime": pun_phpstan::value::datetime_format(raw, "Y-m-d H:i:s.u P") })
        }
        Value::OptionalPath(p) => {
            json!({ "object": "PHPStan\\DependencyInjection\\Neon\\OptionalPath", "props": { "array": [["path", p]] } })
        }
    }
}

fn items(a: &Array) -> Vec<J> {
    a.iter()
        .map(|(k, v)| {
            let k = match k {
                Key::Int(i) => json!(i),
                Key::Str(s) => json!(s),
            };
            json!([k, tagged(v)])
        })
        .collect()
}

/// Inverse of `tagged` (for encoder inputs).
pub fn untagged(j: &J) -> Value {
    match j {
        J::Null => Value::Null,
        J::Bool(b) => Value::Bool(*b),
        J::Number(n) => Value::Int(n.as_i64().expect("int")),
        J::String(s) => Value::Str(s.clone()),
        J::Object(o) if o.contains_key("float") => {
            Value::Float(parse_php_float(o["float"].as_str().unwrap()))
        }
        J::Object(o) if o.contains_key("array") => Value::Array(array(&o["array"])),
        J::Object(o) if o.contains_key("entity") => Value::Entity(Box::new(Entity {
            value: untagged(&o["entity"]),
            attributes: match untagged(&o["attributes"]) {
                Value::Array(a) => a,
                _ => Array::new(),
            },
        })),
        J::Object(o) if o.contains_key("object") => Value::Array(array(&o["props"]["array"])),
        J::Object(o) if o.contains_key("datetime") => {
            Value::DateTime(o["datetime"].as_str().unwrap().to_string())
        }
        other => panic!("untagged: {other}"),
    }
}

fn array(items: &J) -> Array {
    let mut a = Array::new();
    for it in items.as_array().unwrap() {
        let k = match &it[0] {
            J::Number(n) => Key::Int(n.as_i64().unwrap()),
            J::String(s) => Key::Str(s.clone()),
            other => panic!("key {other}"),
        };
        a.set(k, untagged(&it[1]));
    }
    a
}

fn parse_php_float(s: &str) -> f64 {
    match s {
        "INF" => f64::INFINITY,
        "-INF" => f64::NEG_INFINITY,
        "NAN" => f64::NAN,
        s => s.parse().unwrap(),
    }
}

pub fn fixture(name: &str) -> J {
    let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    serde_json::from_str(&std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}")))
        .unwrap()
}

/// `dumpValue()` of scripts/phpstan-config.php: lists as arrays, maps as objects, `{"@float": ...}`, `{"@class": ...}`.
pub fn dumped(v: &Value) -> J {
    match v {
        Value::Float(f) => json!({ "@float": pun_phpstan::value::float_var_export(*f) }),
        Value::Array(a) if a.is_list() => J::Array(a.values().map(dumped).collect()),
        Value::Array(a) => J::Object(a.iter().map(|(k, v)| (k.as_string(), dumped(v))).collect()),
        Value::Entity(e) => json!({
            "@class": "Nette\\DI\\Definitions\\Statement",
            "entity": dumped(&e.value),
            "arguments": dumped(&Value::Array(e.attributes.clone())),
        }),
        Value::OptionalPath(p) => {
            json!({ "@class": "PHPStan\\DependencyInjection\\Neon\\OptionalPath", "path": p })
        }
        other => tagged(other),
    }
}

/// Replace path prefixes with the exporter's placeholders.
pub fn placeholders(j: J, map: &[(&str, &str)]) -> J {
    match j {
        J::String(mut s) => {
            for (from, to) in map {
                s = s.replace(from, to);
            }
            J::String(s)
        }
        J::Array(a) => J::Array(a.into_iter().map(|x| placeholders(x, map)).collect()),
        J::Object(o) => J::Object(
            o.into_iter()
                .map(|(k, v)| (k, placeholders(v, map)))
                .collect(),
        ),
        j => j,
    }
}
