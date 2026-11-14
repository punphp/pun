//! TOML text for JSON values, as Pun writes its own files: `pun.toml`'s dependency sections (`pun migrate`) and
//! `pun.lock`. Keys and strings are bare or quoted as TOML needs; a string with a backslash (a namespace) is written
//! as a literal string, `'App\'`, so it reads as the PHP it names. JSON `null` has no TOML form: callers remove it
//! first (see [`strip_nulls`]).

use serde_json::{Map, Value};
use std::fmt::Write;

/// The longest inline value a [`section`] writes on one line; a longer array goes one item per line and a longer
/// table becomes its own section.
const WIDTH: usize = 100;

/// `k` as a TOML key: bare when it is only ASCII letters, digits, `-` and `_`.
pub fn key(k: &str) -> String {
    let bare = !k.is_empty()
        && k.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_');
    if bare { k.to_string() } else { string(k) }
}

/// `s` as a TOML string: literal (`'...'`) when it has a backslash and no `'` or control character, else basic.
pub fn string(s: &str) -> String {
    let control = |c: char| c.is_control() && c != '\t';
    if s.contains('\\') && !s.contains('\'') && !s.chars().any(control) {
        return format!("'{s}'");
    }
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => write!(out, "\\u{:04X}", c as u32).unwrap(),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// `v` on one line. Nulls inside are left out.
pub fn inline(v: &Value) -> String {
    match v {
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => match n.as_i64() {
            Some(i) => i.to_string(),
            // A whole float keeps its `.0`: TOML reads `1` as an integer.
            None => format!("{:?}", n.as_f64().unwrap_or_default()),
        },
        Value::String(s) => string(s),
        Value::Array(items) => {
            let items: Vec<String> = items.iter().filter(|v| !v.is_null()).map(inline).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Object(map) => {
            let pairs: Vec<String> = (map.iter())
                .filter(|(_, v)| !v.is_null())
                .map(|(k, v)| format!("{} = {}", key(k), inline(v)))
                .collect();
            if pairs.is_empty() {
                "{}".to_string()
            } else {
                format!("{{ {} }}", pairs.join(", "))
            }
        }
    }
}

/// `map` as the section `[path]` (`[[path]]` when `array`; the document's top level when `path` is empty),
/// pun.toml style: values that fit on a line are written inline, then longer tables as their own `[path.key]` sections
/// and lists of tables as `[[path.key]]`, in `map`'s order. TOML needs a table's own keys before its sub-tables, so
/// those come last.
pub fn section(out: &mut String, path: &str, map: &Map<String, Value>, array: bool) {
    if !path.is_empty() && !out.is_empty() && !out.ends_with("\n\n") {
        out.push('\n');
    }
    let (open, close) = if array { ("[[", "]]") } else { ("[", "]") };
    let header = format!("{open}{path}{close}\n");
    let start = out.len();
    let mut later = vec![];
    for (k, v) in map {
        let sub = match path {
            "" => key(k),
            _ => format!("{path}.{}", key(k)),
        };
        match v {
            Value::Null => {}
            Value::Object(m) if !m.is_empty() && !fits(k, v) => later.push((sub, v)),
            Value::Array(items) if tables(items) => later.push((sub, v)),
            Value::Array(items) if !fits(k, v) => {
                writeln!(out, "{} = [", key(k)).unwrap();
                for i in items.iter().filter(|v| !v.is_null()) {
                    writeln!(out, "    {},", inline(i)).unwrap();
                }
                out.push_str("]\n");
            }
            v => writeln!(out, "{} = {}", key(k), inline(v)).unwrap(),
        }
    }
    // `[a.b]` alone defines `a`: a table with nothing but sub-tables gets no `[a]` of its own.
    if !path.is_empty() && (out.len() > start || array || later.is_empty()) {
        out.insert_str(start, &header);
    }
    for (sub, v) in later {
        match v {
            Value::Object(m) => section(out, &sub, m, false),
            _ => {
                for m in v
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(Value::as_object)
                {
                    section(out, &sub, m, true);
                }
            }
        }
    }
}

/// `map`'s entries as dotted keys under `prefix` (`source.url = "..."`), one value per line, so every key keeps its
/// place in `map`'s order; pun.lock uses it, where the order is the order Composer writes. A non-empty table is
/// spread into its keys, an empty one is `{}`, and a list is written inline.
pub fn dotted(out: &mut String, prefix: &str, map: &Map<String, Value>) {
    for (k, v) in map {
        let name = match prefix {
            "" => key(k),
            _ => format!("{prefix}.{}", key(k)),
        };
        match v {
            Value::Null => {}
            Value::Object(m) if !m.is_empty() => dotted(out, &name, m),
            v => writeln!(out, "{name} = {}", inline(v)).unwrap(),
        }
    }
}

/// A non-empty list whose items are all tables: written as `[[path.key]]` sections.
fn tables(items: &[Value]) -> bool {
    !items.is_empty() && items.iter().all(Value::is_object)
}

/// Whether `k = v` fits on a line: a table only when none of its values is a table or a list of tables.
fn fits(k: &str, v: &Value) -> bool {
    let flat = match v {
        Value::Object(m) => !m.values().any(|v| match v {
            Value::Object(_) => true,
            Value::Array(items) => items.iter().any(Value::is_object),
            _ => false,
        }),
        _ => true,
    };
    flat && key(k).len() + 3 + inline(v).len() <= WIDTH
}

/// Removes every `null` in `v`, naming each as a JSON pointer under `at` in `removed` (TOML has no null).
pub fn strip_nulls(v: &mut Value, at: &str, removed: &mut Vec<String>) {
    match v {
        Value::Object(m) => {
            m.retain(|k, v| {
                let here = format!("{at}/{k}");
                if v.is_null() {
                    removed.push(here);
                    return false;
                }
                strip_nulls(v, &here, removed);
                true
            });
        }
        Value::Array(items) => {
            let mut i = 0;
            items.retain_mut(|v| {
                let here = format!("{at}/{i}");
                i += 1;
                if v.is_null() {
                    removed.push(here);
                    return false;
                }
                strip_nulls(v, &here, removed);
                true
            });
        }
        _ => {}
    }
}

/// `v` as JSON: what a section of pun.toml or pun.lock means in composer.json terms.
pub fn to_json(v: &toml::Value) -> Value {
    match v {
        toml::Value::String(s) => Value::String(s.clone()),
        toml::Value::Integer(i) => Value::from(*i),
        toml::Value::Float(f) => Value::from(*f),
        toml::Value::Boolean(b) => Value::Bool(*b),
        toml::Value::Datetime(d) => Value::String(d.to_string()),
        toml::Value::Array(items) => Value::Array(items.iter().map(to_json).collect()),
        toml::Value::Table(t) => {
            Value::Object(t.iter().map(|(k, v)| (k.clone(), to_json(v))).collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// What `section` writes, read back by the `toml` crate, as JSON.
    fn round_trip(path: &str, v: &Value) -> Value {
        let mut out = String::new();
        section(&mut out, path, v.as_object().unwrap(), false);
        let t: toml::Table = toml::from_str(&out).unwrap_or_else(|e| panic!("{e}\n{out}"));
        to_json(&toml::Value::Table(t))[path].clone()
    }

    #[test]
    fn keys_and_strings_are_quoted_as_toml_needs() {
        assert_eq!(key("psr-4"), "psr-4");
        assert_eq!(key("laravel/framework"), "\"laravel/framework\"");
        assert_eq!(key(""), "\"\"");
        assert_eq!(key("App\\"), "'App\\'");
        assert_eq!(string("it's \\"), "\"it's \\\\\"");
        assert_eq!(string("a\"b\nc\u{1}"), "\"a\\\"b\\nc\\u0001\"");
        assert_eq!(inline(&json!(1.0)), "1.0");
        assert_eq!(inline(&json!({})), "{}");
        assert_eq!(string("\r\t"), "\"\\r\\t\"");
        assert_eq!(inline(&Value::Null), "");
        // Nulls are left out at every level; a TOML datetime reads back as its text.
        let mut out = String::new();
        section(
            &mut out,
            "p",
            json!({"a": null, "b": 1}).as_object().unwrap(),
            false,
        );
        dotted(
            &mut out,
            "q",
            json!({"a": null, "b": 2}).as_object().unwrap(),
        );
        assert_eq!(out, "[p]\nb = 1\nq.b = 2\n");
        let t: toml::Table = toml::from_str("t = 1979-05-27T07:32:00Z").unwrap();
        assert_eq!(
            to_json(&toml::Value::Table(t)),
            json!({"t": "1979-05-27T07:32:00Z"})
        );
    }

    /// The renderer's output must mean the same JSON: a wrong quote, a table key after a sub-table, or a dropped
    /// value fails the parse or the comparison.
    #[test]
    fn sections_read_back_as_the_same_json() {
        let v = json!({
            "name": "acme/app",
            "keywords": ["a", "b"],
            "long": ["0123456789", "0123456789", "0123456789", "0123456789", "0123456789", "0123456789",
                     "0123456789", "0123456789", "0123456789"],
            "license": "MIT",
            "authors": [{"name": "Ann", "email": "ann@example.com"}, {"name": "Bo"}],
            "autoload": {
                "psr-4": {"App\\": "app/", "Database\\Factories\\": "database/factories/", "": "src/"},
                "files": ["app/helpers.php"],
            },
            "scripts": {
                "post-autoload-dump": [
                    "Illuminate\\Foundation\\ComposerScripts::postAutoloadDump",
                    "@php artisan package:discover --ansi",
                    "@php -r \"file_exists('.env') || copy('.env.example', '.env');\"",
                ],
                "test": "pest",
            },
            "extra": {"laravel": {"dont-discover": []}, "branch-alias": {"dev-main": "1.x-dev"}, "n": 3, "f": 1.5},
            "empty": {},
        });
        assert_eq!(round_trip("package", &v), v);
        // The top level: plain keys first, then the tables.
        let mut out = String::new();
        section(&mut out, "", v.as_object().unwrap(), false);
        let t: toml::Table = toml::from_str(&out).unwrap();
        assert_eq!(to_json(&toml::Value::Table(t)), v);
        assert!(out.starts_with("name = \"acme/app\"\n"));
    }

    /// Dotted keys keep every key where it was, at every depth: a table after a sub-table, too.
    #[test]
    fn dotted_keys_keep_the_order() {
        let v = json!({"name": "a/b", "source": {"type": "git", "url": "u"}, "require": {"php": "^8.4"},
            "extra": {"laravel": {"providers": ["P"], "dont-discover": []}, "x": {}}, "type": "library"});
        let mut out = String::new();
        dotted(&mut out, "", v.as_object().unwrap());
        let t: toml::Table = toml::from_str(&out).unwrap();
        let back = to_json(&toml::Value::Table(t));
        assert_eq!(back, v);
        let keys: Vec<&String> = back.as_object().unwrap().keys().collect();
        assert_eq!(keys, ["name", "source", "require", "extra", "type"]);
    }

    #[test]
    fn nulls_are_removed_and_named() {
        let mut v = json!({"a": null, "b": [1, null, {"c": null}]});
        let mut removed = vec![];
        strip_nulls(&mut v, "", &mut removed);
        assert_eq!(v, json!({"b": [1, {}]}));
        assert_eq!(removed, ["/a", "/b/1", "/b/2/c"]);
    }
}
