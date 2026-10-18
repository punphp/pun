//! `--generate-baseline`: PHPStan's `BaselineNeonErrorFormatter` and `BaselinePhpErrorFormatter`, byte for byte.
//! Reading a baseline back is ordinary config loading (`di::load_file`: NEON, or the PHP file's literal array).

use crate::di::escape;
use crate::ignore::Error;
use crate::neon;
use crate::php::{parent_relative_path, preg_quote};
use crate::value::{Array, Key, Value};
use std::collections::BTreeMap;

/// One `ignoreErrors` entry: message (raw or as the `#^...$#` regex), identifier, count, path relative to the baseline.
struct Entry {
    message: Value,
    identifier: Option<Value>,
    count: i64,
    path: Value,
}

/// PHP array keys: `ksort(SORT_STRING)` order, but numeric strings come back as ints.
fn php_key(s: &str) -> Value {
    match Key::from_string(s.to_string()) {
        Key::Int(i) => Value::Int(i),
        Key::Str(s) => Value::Str(s),
    }
}

fn entries(baseline_dir: &str, errors: &[Error], raw: bool, path_prefix: &str) -> Vec<Entry> {
    // file -> message -> (total, identifier -> count); BTreeMap of String is strcmp order, as ksort(SORT_STRING).
    type ByMessage<'a> = BTreeMap<&'a str, (i64, BTreeMap<&'a str, i64>)>;
    let mut files: BTreeMap<String, ByMessage> = BTreeMap::new();
    for e in errors.iter().filter(|e| e.can_be_ignored) {
        let file = format!(
            "{path_prefix}{}",
            parent_relative_path(baseline_dir, &e.file_path)
        );
        let (total, ids) = files
            .entry(file)
            .or_default()
            .entry(&e.message)
            .or_default();
        *total += 1;
        if let Some(id) = &e.identifier {
            *ids.entry(id).or_default() += 1;
        }
    }
    let mut out = Vec::new();
    for (file, messages) in files {
        for (message, (total, ids)) in messages {
            // ponytail: a numeric message without rawMessage is a TypeError in PHPStan (preg_quote(int)); quoted here.
            let message = if raw {
                php_key(message)
            } else {
                Value::Str(format!("#^{}$#", preg_quote(message, Some('#'))))
            };
            let message = escape(message);
            let path = escape(php_key(&file));
            let counts: Vec<(Option<Value>, i64)> = if ids.is_empty() {
                vec![(None, total)]
            } else {
                ids.into_iter()
                    .map(|(id, n)| (Some(escape(php_key(id))), n))
                    .collect()
            };
            for (identifier, count) in counts {
                out.push(Entry {
                    message: message.clone(),
                    identifier,
                    count,
                    path: path.clone(),
                });
            }
        }
    }
    out
}

/// The `phpstan-baseline.neon` content. `existing` is the current baseline file's content (`""` when there is none):
/// its trailing newlines are kept.
pub fn neon(baseline_dir: &str, errors: &[Error], existing: &str, raw: bool) -> String {
    let key = if raw { "rawMessage" } else { "message" };
    let list = entries(baseline_dir, errors, raw, "").into_iter().map(|e| {
        let mut a = Array::new();
        a.set(key.into(), e.message);
        if let Some(id) = e.identifier {
            a.set("identifier".into(), id);
        }
        a.set("count".into(), Value::Int(e.count));
        a.set("path".into(), e.path);
        Value::Array(a)
    });
    let mut ignore = Array::new();
    ignore.set("ignoreErrors".into(), Value::Array(Array::list(list)));
    let mut root = Array::new();
    root.set("parameters".into(), Value::Array(ignore));
    let encoded = neon::encode(&Value::Array(root), true, "\t");
    if existing.is_empty() {
        return encoded[..encoded.len() - 1].to_string();
    }
    let kept = existing.len() - existing.trim_end_matches('\n').len();
    format!("{}{}", &encoded[..encoded.len() - 2], "\n".repeat(kept))
}

/// The `phpstan-baseline.php` content. `has_errors` is whether the analysis reported any error at all (including
/// non-ignorable and not file-specific ones).
pub fn php(baseline_dir: &str, errors: &[Error], has_errors: bool, raw: bool) -> String {
    let mut php = String::from("<?php declare(strict_types = 1);\n\n");
    if !has_errors {
        return php + "return [];\n";
    }
    php.push_str("$ignoreErrors = [];\n");
    let key = Value::str(if raw { "rawMessage" } else { "message" }).var_export();
    for e in entries(baseline_dir, errors, raw, "/") {
        php.push_str(&format!(
            "$ignoreErrors[] = [\n\t{key} => {},\n",
            e.message.var_export()
        ));
        if let Some(id) = e.identifier {
            php.push_str(&format!("\t'identifier' => {},\n", id.var_export()));
        }
        php.push_str(&format!(
            "\t'count' => {},\n\t'path' => __DIR__ . {},\n];\n",
            e.count,
            e.path.var_export()
        ));
    }
    php + "\nreturn ['parameters' => ['ignoreErrors' => $ignoreErrors]];\n"
}
