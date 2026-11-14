//! The text PHP itself writes for the files under `vendor/composer/`: `json_encode` with PHP's default flags and with
//! Composer's (`JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE`), `var_export` of nested arrays,
//! and `strtr` with several pairs. JSON goes through PHP arrays on the way: an empty object and an object keyed
//! `"0"`, `"1"`, ... come back as lists, as `json_decode($s, true)` then `json_encode` gives them.

use pun_tokens::php_compat::var_export_string;
use serde_json::Value;
use std::fmt::Write;

/// PHP's `md5()`: the digest as lowercase hex.
pub fn md5(bytes: &[u8]) -> String {
    use md5::Digest;
    md5::Md5::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// PHP's `sha1()`: the digest as lowercase hex.
pub fn sha1(bytes: &[u8]) -> String {
    use sha1::Digest;
    sha1::Sha1::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// `json_encode($v)` with PHP's default flags: one line, `/` and every non-ASCII character escaped.
pub fn json_compact(v: &Value) -> String {
    let mut out = String::new();
    json(&mut out, v, None, true);
    out
}

/// `json_encode($v, JSON_PRETTY_PRINT)`: four-space indented, `/` and non-ASCII escaped.
pub fn json_pretty_escaped(v: &Value) -> String {
    let mut out = String::new();
    json(&mut out, v, Some(0), true);
    out
}

/// `json_encode($v, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE)`, four-space indented.
pub fn json_pretty(v: &Value) -> String {
    let mut out = String::new();
    json(&mut out, v, Some(0), false);
    out
}

/// Whether PHP holds `m` as a list: its keys are `0`, `1`, ... in order (no keys at all included).
fn is_list(m: &serde_json::Map<String, Value>) -> bool {
    m.keys().enumerate().all(|(i, k)| *k == i.to_string())
}

fn json(out: &mut String, v: &Value, indent: Option<usize>, escape: bool) {
    let newline = |out: &mut String, level: usize| {
        if indent.is_some() {
            out.push('\n');
            out.push_str(&"    ".repeat(level));
        }
    };
    let level = indent.unwrap_or(0);
    let deeper = indent.map(|l| l + 1);
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => write!(out, "{b}").unwrap(),
        Value::Number(n) => match n.as_i64().map(|_| n.to_string()) {
            Some(i) => out.push_str(&i),
            None => out.push_str(&float(n.as_f64().unwrap_or_default())),
        },
        Value::String(s) => string(out, s, escape),
        Value::Array(items) => {
            if items.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                newline(out, level + 1);
                json(out, item, deeper, escape);
            }
            newline(out, level);
            out.push(']');
        }
        Value::Object(m) if is_list(m) => {
            json(
                out,
                &Value::Array(m.values().cloned().collect()),
                indent,
                escape,
            );
        }
        Value::Object(m) => {
            out.push('{');
            for (i, (k, item)) in m.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                newline(out, level + 1);
                string(out, k, escape);
                out.push_str(if indent.is_some() { ": " } else { ":" });
                json(out, item, deeper, escape);
            }
            newline(out, level);
            out.push('}');
        }
    }
}

/// A float as `json_encode` writes it (`serialize_precision = -1`): the shortest repr, a whole number without `.0`.
fn float(f: f64) -> String {
    if f.fract() == 0.0 && f.abs() < 1e15 {
        format!("{f:.0}")
    } else {
        format!("{f}")
    }
}

fn string(out: &mut String, s: &str, escape: bool) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '/' if escape => out.push_str("\\/"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => write!(out, "\\u{:04x}", c as u32).unwrap(),
            // Escaped even with JSON_UNESCAPED_UNICODE, unless JSON_UNESCAPED_LINE_TERMINATORS.
            '\u{2028}' | '\u{2029}' => write!(out, "\\u{:04x}", c as u32).unwrap(),
            c if escape && !c.is_ascii() => {
                let mut units = [0u16; 2];
                for u in c.encode_utf16(&mut units) {
                    write!(out, "\\u{u:04x}").unwrap();
                }
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

/// A PHP array key or value as `var_export` gives it.
#[derive(Debug, Clone, PartialEq)]
pub enum Php {
    Str(String),
    Int(i64),
    /// (key, value) pairs in order; a key is a string or an integer.
    Array(Vec<(Key, Php)>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Key {
    Str(String),
    Int(i64),
}

impl Key {
    /// `$a[$k]`'s key: PHP turns a canonical decimal string (`"7"`, not `"07"`) into an integer.
    pub fn of(k: &str) -> Key {
        match k.parse::<i64>() {
            Ok(i) if i.to_string() == k => Key::Int(i),
            _ => Key::Str(k.to_string()),
        }
    }
}

/// `var_export($v, true)`: nested arrays as `array (` with two spaces per level, `'key' => ` with the nested
/// `array (` on its own line.
pub fn var_export(v: &Php) -> String {
    let mut out = String::new();
    export(&mut out, v, 0);
    out
}

/// `var_export` of a string, as text.
pub fn export_str(s: &str) -> String {
    String::from_utf8(var_export_string(s.as_bytes())).expect("var_export keeps UTF-8 valid")
}

fn export(out: &mut String, v: &Php, level: usize) {
    match v {
        Php::Str(s) => out.push_str(&export_str(s)),
        Php::Int(i) => write!(out, "{i}").unwrap(),
        Php::Array(pairs) => {
            out.push_str("array (\n");
            let pad = "  ".repeat(level + 1);
            for (k, item) in pairs {
                out.push_str(&pad);
                match k {
                    Key::Str(s) => out.push_str(&export_str(s)),
                    Key::Int(i) => write!(out, "{i}").unwrap(),
                }
                out.push_str(" => ");
                if matches!(item, Php::Array(_)) {
                    out.push('\n');
                    out.push_str(&pad);
                }
                export(out, item, level + 1);
                out.push_str(",\n");
            }
            out.push_str(&"  ".repeat(level));
            out.push(')');
        }
    }
}

/// `strtr($s, $pairs)`: at each position the longest matching key is replaced, and the result is not searched
/// again.
pub fn strtr(s: &str, pairs: &[(&str, &str)]) -> String {
    let mut pairs: Vec<&(&str, &str)> = pairs.iter().filter(|p| !p.0.is_empty()).collect();
    pairs.sort_by_key(|p| std::cmp::Reverse(p.0.len()));
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    'outer: while i < s.len() {
        for (from, to) in &pairs {
            if s[i..].starts_with(from) {
                out.push_str(to);
                i += from.len();
                continue 'outer;
            }
        }
        let c = s[i..].chars().next().unwrap();
        out.push(c);
        i += c.len_utf8();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Expected values from PHP 8.5: `json_encode(json_decode($s, true))` and the same with Composer's flags.
    #[test]
    fn json_matches_php() {
        let v: Value = serde_json::from_str(
            r#"{"a/b":"é/𝄞","l":{"0":"x","1":"y"},"e":{},"n":[1,1.5,2.0,true,null],"c":"\u0001\b\u2028\"\f\n\r\t"}"#,
        )
        .unwrap();
        assert_eq!(
            json_compact(&v),
            r#"{"a\/b":"\u00e9\/\ud834\udd1e","l":["x","y"],"e":[],"n":[1,1.5,2,true,null],"c":"\u0001\b\u2028\"\f\n\r\t"}"#
        );
        assert_eq!(
            json_pretty(&v),
            "{\n    \"a/b\": \"é/𝄞\",\n    \"l\": [\n        \"x\",\n        \"y\"\n    ],\n    \"e\": [],\n    \"n\": [\n        1,\n        1.5,\n        2,\n        true,\n        null\n    ],\n    \"c\": \"\\u0001\\b\\u2028\\\"\\f\\n\\r\\t\"\n}"
        );
        assert_eq!(
            json_pretty(&json!({"k": {"x": 1}})),
            "{\n    \"k\": {\n        \"x\": 1\n    }\n}"
        );
        assert_eq!(
            json_pretty_escaped(&json!(["A\\B", "a/é"])),
            "[\n    \"A\\\\B\",\n    \"a\\/\\u00e9\"\n]"
        );
        // php -r 'echo md5("abc"), sha1("abc");'
        assert_eq!(md5(b"abc"), "900150983cd24fb0d6963f7d28e17f72");
        assert_eq!(sha1(b"abc"), "a9993e364706816aba3e25717850c26c9cd0d89d");
    }

    /// From PHP 8.5's `var_export(['v' => ['voku\\' => 5], 0 => 'a\'b'], true)`.
    #[test]
    fn var_export_matches_php() {
        let v = Php::Array(vec![
            (
                Key::of("v"),
                Php::Array(vec![(Key::Str("voku\\".into()), Php::Int(5))]),
            ),
            (Key::of("0"), Php::Str("a'b".into())),
            (Key::of("07"), Php::Array(vec![])),
        ]);
        assert_eq!(
            var_export(&v),
            "array (\n  'v' => \n  array (\n    'voku\\\\' => 5,\n  ),\n  0 => 'a\\'b',\n  '07' => \n  array (\n  ),\n)"
        );
    }

    /// From PHP 8.5's `strtr`: the longer key wins at a position, and replaced text is not searched again.
    #[test]
    fn strtr_matches_php() {
        assert_eq!(
            strtr(
                "/a/b/c",
                &[("/a", "X"), ("/a/b", "Y"), ("Y", "Z"), ("", "E")]
            ),
            "Y/c"
        );
        assert_eq!(strtr("é/a", &[("/a", "!")]), "é!");
    }
}
