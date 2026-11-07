//! Which reported errors PHPStan 2.3.5 drops: `ignoreErrors` (`IgnoredErrorHelper` / `IgnoredErrorHelperResult`, with
//! `count`, `reportUnmatched` and the unmatched-pattern errors) and the `@phpstan-ignore*` comments
//! (`LocalIgnoresProcessor`, `reportIgnoresWithoutComments`, the unmatched-line errors). Comment parsing is in
//! `comments`. Everything is a pure function of the errors and the resolved config, except that `path` entries are
//! looked up on disk the way PHPStan does (`is_file`, `FileExcluder`).

use crate::comments::{Identifier, LinesToIgnore};
use crate::neon::json_string;
use crate::php::{FileExcluder, absolutize_path, is_match, normalize_path, preg, preg_quote};
use crate::value::{Array, Key, Value};
use indexmap::IndexMap;
use pcre2::bytes::Regex;
use std::collections::HashMap;

/// `PHPStan\Analyser\Error`, with the fields ignoring and the baseline formatters look at.
#[derive(Clone, Debug, PartialEq)]
pub struct Error {
    pub message: String,
    /// `getFile()`: the file description (a trait's errors carry the using class's file here).
    pub file: String,
    pub line: Option<i64>,
    pub can_be_ignored: bool,
    /// `getFilePath()`: the file the error is in.
    pub file_path: String,
    pub trait_file_path: Option<String>,
    pub tip: Option<String>,
    pub identifier: Option<String>,
}

impl Error {
    pub fn new(message: impl Into<String>, file: impl Into<String>, line: Option<i64>) -> Error {
        let file = file.into();
        Error {
            message: message.into(),
            file_path: file.clone(),
            file,
            line,
            can_be_ignored: true,
            trait_file_path: None,
            tip: None,
            identifier: None,
        }
    }

    fn non_ignorable(mut self, identifier: &str) -> Error {
        self.can_be_ignored = false;
        self.identifier = Some(identifier.into());
        self
    }
}

/// `isset($a[$k])`
fn isset<'a>(v: &'a Value, k: &str) -> Option<&'a Value> {
    v.as_array()?.get_str(k).filter(|v| **v != Value::Null)
}

fn str_of(v: Option<&Value>) -> Option<String> {
    v.and_then(Value::to_php_string)
}

/// `Nette\Utils\Json::encode()` of an `ignoreErrors` entry (schema-checked: strings, ints, bools, lists, maps).
fn json(v: &Value) -> String {
    match v {
        Value::Null => "null".into(),
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Array(a) if a.is_list() => {
            format!("[{}]", a.values().map(json).collect::<Vec<_>>().join(","))
        }
        Value::Array(a) => format!(
            "{{{}}}",
            a.iter()
                .map(|(k, v)| format!("{}:{}", json_string(&k.as_string()), json(v)))
                .collect::<Vec<_>>()
                .join(",")
        ),
        other => json_string(&other.to_php_string().unwrap_or_default()),
    }
}

/// `IgnoredError::getIgnoredErrorLabel()`
fn label(e: &Value) -> &'static str {
    if isset(e, "message").is_none() && isset(e, "rawMessage").is_some() {
        "Ignored error"
    } else {
        "Ignored error pattern"
    }
}

/// `IgnoredError::stringifyPattern()`
fn stringify(e: &Value) -> String {
    let Value::Array(_) = e else {
        return e.to_php_string().unwrap_or_default();
    };
    let mut message = match (isset(e, "message"), isset(e, "rawMessage")) {
        (Some(m), _) => m.to_php_string().unwrap_or_default(),
        (None, Some(r)) => format!("\"{}\"", r.to_php_string().unwrap_or_default()),
        _ => String::new(),
    };
    // (`identifiers` never reaches here: initialize() expanded it to `identifier`.) Pun's `rule` shows as the
    // identifier does when there is none.
    if let Some(ids) = str_of(isset(e, "identifier").or_else(|| isset(e, "rule"))) {
        message = if message.is_empty() {
            ids
        } else {
            format!("{message} ({ids})")
        };
    }
    if let Some(p) = isset(e, "path") {
        return format!(
            "{message} in path {}",
            p.to_php_string().unwrap_or_default()
        );
    }
    if let Some(Value::Array(paths)) = isset(e, "paths") {
        let list: Vec<String> = paths.values().filter_map(Value::to_php_string).collect();
        let s = if list.len() == 1 {
            " in path "
        } else {
            " in paths: "
        };
        return format!("{message}{s}{}", list.join(", "));
    }
    message
}

/// An entry's `message`, `rawMessage`, `identifier` and (Pun's) `rule`.
type Fields = (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
);

/// `IgnoredErrorHelper::initialize()`'s result over the resolved `ignoreErrors` and `reportUnmatchedIgnoredErrors`.
pub struct IgnoredErrors {
    /// `getErrors()`: config entries PHPStan rejects here (reported before analysis).
    pub errors: Vec<String>,
    /// Entries as matched (`ignoreErrorEntry['ignoreError']`) and as reported when unmatched (normalized `path`,
    /// `realPath`).
    entries: Vec<(Value, Value)>,
    by_file: HashMap<String, Vec<usize>>,
    other: Vec<usize>,
    report_unmatched: bool,
    /// Each entry's message pattern, compiled (`None` inside where it does not compile).
    patterns: Vec<Option<Option<Regex>>>,
    /// The Pun rule of an identifier, for entries with Pun's `rule` key (`[[analyse.ignore]] rule`).
    rule_of: fn(&str) -> Option<&'static str>,
    /// IgnoreErrorsConfigFixer::getEntriesByPath(): the config files' entries by normalized path, or by `""`.
    config_entries: HashMap<String, Vec<ConfigEntry>>,
    current_working_directory: String,
}

/// An `ignoreErrors` entry of a config file, with its values as loaded.
struct ConfigEntry {
    file: String,
    values: Array,
    line: i64,
}

/// IgnoreErrorsConfigFixer::ENTRY_KEYS
const ENTRY_KEYS: [&str; 10] = [
    "message",
    "messages",
    "rawMessage",
    "rawMessages",
    "identifier",
    "identifiers",
    "path",
    "paths",
    "count",
    "reportUnmatched",
];

/// `IgnoredErrorHelperResult::process()`'s result.
#[derive(Debug, Default, PartialEq)]
pub struct Processed {
    /// The remaining errors, then `ignore.count` / `ignore.unmatched` errors.
    pub errors: Vec<Error>,
    /// The ignored errors with the entry that ignored them.
    pub ignored: Vec<(Error, Value)>,
    /// Not file-specific errors (unmatched patterns without a file, errors that cannot be ignored).
    pub string_errors: Vec<String>,
}

fn expand(entry: &Value, list: &str, one: &str, out: &mut Vec<Value>) -> bool {
    let Some(Value::Array(items)) = isset(entry, list) else {
        return isset(entry, list).is_some();
    };
    for item in items.values() {
        let mut e = entry.as_array().cloned().unwrap_or_default();
        e.remove(&Key::from(list));
        e.set(one.into(), item.clone());
        out.push(Value::Array(e));
    }
    true
}

impl IgnoredErrors {
    pub fn new(ignore_errors: &Array, report_unmatched: bool) -> IgnoredErrors {
        let mut errors = Vec::new();
        let mut expanded = Vec::new();
        for entry in ignore_errors.values() {
            if let Value::Array(_) = entry {
                let keys = [
                    "message",
                    "messages",
                    "rawMessage",
                    "rawMessages",
                    "identifier",
                    "identifiers",
                    "rule",
                ];
                if keys.iter().all(|k| isset(entry, k).is_none()) {
                    errors.push(format!(
                        "Ignored error {} is missing a message or an identifier.",
                        json(entry)
                    ));
                    continue;
                }
                if !expand(entry, "messages", "message", &mut expanded)
                    && !expand(entry, "rawMessages", "rawMessage", &mut expanded)
                    && !expand(entry, "identifiers", "identifier", &mut expanded)
                {
                    expanded.push(entry.clone());
                }
            } else {
                expanded.push(entry.clone());
            }
        }

        // Entries with the same path and message/identifier add up their counts.
        let mut uniqued: Vec<Value> = Vec::new();
        let mut keyed: HashMap<String, usize> = HashMap::new();
        for entry in expanded {
            let fields = ["message", "rawMessage", "identifier", "rule"];
            let Some(path) =
                isset(&entry, "path").filter(|_| fields.iter().any(|k| isset(&entry, k).is_some()))
            else {
                uniqued.push(entry);
                continue;
            };
            let mut key = path.to_php_string().unwrap_or_default();
            for f in fields {
                if let Some(v) = str_of(isset(&entry, f)) {
                    key = format!("{key}\n{v}");
                }
            }
            let Some(&at) = keyed.get(&key) else {
                keyed.insert(key, uniqued.len());
                uniqued.push(entry);
                continue;
            };
            let prev = &uniqued[at];
            let flag = |e: &Value| {
                isset(e, "reportUnmatched")
                    .cloned()
                    .unwrap_or(Value::Bool(report_unmatched))
            };
            let mut report = flag(prev);
            if report == Value::Bool(false) {
                report = flag(&entry);
            }
            let count = |e: &Value| match isset(e, "count") {
                Some(Value::Int(c)) => *c,
                _ => 1,
            };
            let mut merged = Array::new();
            for f in ["message", "rawMessage"] {
                merged.set(f.into(), isset(&entry, f).cloned().unwrap_or(Value::Null));
            }
            merged.set("path".into(), path.clone());
            merged.set(
                "identifier".into(),
                isset(&entry, "identifier").cloned().unwrap_or(Value::Null),
            );
            if let Some(rule) = isset(&entry, "rule") {
                merged.set("rule".into(), rule.clone());
            }
            merged.set("count".into(), Value::Int(count(prev) + count(&entry)));
            merged.set("reportUnmatched".into(), report);
            uniqued[at] = Value::Array(merged);
        }

        let mut by_file: HashMap<String, Vec<usize>> = HashMap::new();
        let mut other = Vec::new();
        let mut entries = Vec::new();
        for (i, entry) in uniqued.into_iter().enumerate() {
            // (initialize() re-checks for a message here; the expansion above always leaves one.)
            let mut reported = entry.clone();
            match str_of(isset(&entry, "path")) {
                Some(path) if std::path::Path::new(&path).is_file() => {
                    let normalized = normalize_path(&path);
                    if let Value::Array(a) = &mut reported {
                        a.set("path".into(), Value::str(&normalized));
                        a.set("realPath".into(), Value::str(&normalized));
                    }
                    by_file.entry(normalized).or_default().push(i);
                }
                _ => other.push(i),
            }
            entries.push((entry, reported));
        }
        let patterns = entries
            .iter()
            .map(|(entry, _)| {
                let message = match entry {
                    Value::Array(_) => str_of(isset(entry, "message")),
                    _ => entry.to_php_string(),
                };
                // Literal `\r\n` / `\r` (backslash sequences, not newlines) become `\n`, as PHPStan does.
                message.map(|pattern| {
                    let pattern = pattern
                        .replace(&preg_quote("\\r\\n", None), &preg_quote("\\n", None))
                        .replace(&preg_quote("\\r", None), &preg_quote("\\n", None));
                    preg(&pattern).ok()
                })
            })
            .collect();
        IgnoredErrors {
            errors,
            entries,
            by_file,
            other,
            report_unmatched,
            patterns,
            rule_of: |_| None,
            config_entries: HashMap::new(),
            current_working_directory: String::new(),
        }
    }

    /// IgnoreErrorsConfigFixer::getEntriesByPath() over the container parameters' `allConfigFiles`, so an unmatched
    /// entry without a file of its own is reported in the config file it is in (`findEntryLocation()`). The files
    /// in the PHAR and in installed Composer packages are left out.
    pub fn with_config_files(mut self, parameters: &Array) -> IgnoredErrors {
        let string = |k: &str| str_of(parameters.get_str(k)).unwrap_or_default();
        let strings = |k: &str| -> Vec<String> {
            parameters
                .get_str(k)
                .and_then(Value::as_array)
                .map(|a| a.values().filter_map(Value::to_php_string).collect())
                .unwrap_or_default()
        };
        let cwd = string("currentWorkingDirectory");
        // IgnoreErrorsConfigFixer::getParameters()
        let mut env = Array::new();
        for (k, v) in std::env::vars() {
            env.set(Key::from_string(k), Value::Str(v));
        }
        let mut known = Array::new();
        known.set("rootDir".into(), Value::Str(string("rootDir")));
        known.set("currentWorkingDirectory".into(), Value::Str(cwd.clone()));
        known.set("env".into(), Value::Array(env));
        let installed = InstalledPackages::of(&strings("composerAutoloaderProjectPaths"));
        let relative = [
            "[parameters][ignoreErrors][][path]".to_string(),
            "[parameters][ignoreErrors][][paths][]".to_string(),
        ];
        for file in strings("allConfigFiles") {
            if file.starts_with("phar://") || installed.has_file(&file) {
                continue;
            }
            // ponytail: entries in a .php config file are not looked for (findPhpEntries() needs PHP's parser)
            if file.ends_with(".php") {
                continue;
            }
            let Some(items) = std::fs::read(&file).ok().and_then(|c| crate::neon::ignore_errors_items(&c)) else {
                continue;
            };
            for (raw, line) in items {
                if let Value::Array(a) = &raw
                    && a.keys().any(|k| !matches!(k, Key::Str(k) if ENTRY_KEYS.contains(&k.as_str())))
                {
                    continue;
                }
                let mut list = Array::new();
                list.push(raw);
                let mut wrapped = Array::new();
                wrapped.set("ignoreErrors".into(), Value::Array(list));
                let mut root = Array::new();
                root.set("parameters".into(), Value::Array(wrapped));
                let values = crate::di::process(root, "", &file, &relative, &known)
                    .ok()
                    .and_then(|p| p.get_str("parameters")?.as_array()?.get_str("ignoreErrors")?.as_array()?.values().next().cloned())
                    .and_then(|v| crate::di::expand(&v, &known, None).ok());
                let values = match values {
                    Some(Value::Array(a)) => a,
                    Some(v @ Value::Str(_)) => {
                        let mut a = Array::new();
                        a.set("message".into(), v);
                        a
                    }
                    _ => continue,
                };
                if values.is_empty() || values.get_str("count").is_some_and(|c| !matches!(c, Value::Int(_))) {
                    continue;
                }
                let key = match values.get_str("path") {
                    None if values.get_str("paths").is_none() => String::new(),
                    Some(Value::Str(p)) => normalize_path(&absolutize_path(&cwd, p)),
                    _ => continue,
                };
                self.config_entries.entry(key).or_default().push(ConfigEntry {
                    file: file.clone(),
                    values,
                    line: line as i64,
                });
            }
        }
        self.current_working_directory = cwd;
        self
    }

    /// IgnoreErrorsConfigFixer::findEntryLocation(): the config file and line of the first entry `ignore` was made
    /// of.
    fn entry_location(&self, ignore: &Value) -> Option<(String, i64)> {
        let e = match ignore {
            Value::Array(a) => a.clone(),
            other => {
                let mut a = Array::new();
                a.set("message".into(), other.clone());
                a
            }
        };
        let e = Value::Array(e);
        let path = str_of(isset(&e, "realPath").or_else(|| isset(&e, "path")));
        if path.is_none() && isset(&e, "paths").is_some() {
            return None;
        }
        let key = path.map_or_else(String::new, |p| normalize_path(&absolutize_path(&self.current_working_directory, &p)));
        let entry = self.config_entries.get(&key)?.iter().find(|entry| {
            [("message", "messages"), ("rawMessage", "rawMessages"), ("identifier", "identifiers")]
                .iter()
                .all(|(field, list_field)| {
                    let Some(expected) = isset(&e, field) else {
                        return !entry.values.contains(&Key::from(*field)) && !entry.values.contains(&Key::from(*list_field));
                    };
                    if entry.values.get_str(field).filter(|v| **v != Value::Null) == Some(expected) {
                        return true;
                    }
                    matches!(entry.values.get_str(list_field), Some(Value::Array(list)) if list.values().any(|v| v == expected))
                })
        })?;
        Some((entry.file.clone(), entry.line))
    }

    /// Pun's `rule` key: an entry with one ignores only errors whose identifier `rule_of` maps to that rule.
    pub fn with_rules(self, rule_of: fn(&str) -> Option<&'static str>) -> IgnoredErrors {
        IgnoredErrors { rule_of, ..self }
    }

    /// `IgnoredError::shouldIgnore()`
    fn should_ignore(
        &self,
        error: &Error,
        i: usize,
        fields: &Fields,
        path: Option<String>,
    ) -> bool {
        let (_, raw, identifier, rule) = fields.clone();
        if identifier.is_some() && error.identifier != identifier {
            return false;
        }
        if rule.is_some_and(|r| error.identifier.as_deref().and_then(self.rule_of) != Some(&r)) {
            return false;
        }
        if let Some(re) = &self.patterns[i] {
            let message = error.message.replace("\\r\\n", "\\n").replace("\\r", "\\n");
            if !re.as_ref().is_some_and(|re| is_match(re, &message)) {
                return false;
            }
        }
        if raw.is_some_and(|raw| raw != error.message) {
            return false;
        }
        if let Some(path) = path {
            let excluder = FileExcluder::new([path.as_str()]);
            return excluder.is_excluded(&error.file_path)
                || error
                    .trait_file_path
                    .as_deref()
                    .is_some_and(|t| excluder.is_excluded(t));
        }
        true
    }

    /// `IgnoredErrorHelperResult::process()`: `only_files` is true when only file-specific errors are reported (e.g.
    /// the analysed paths are a subset), `analysed_files` the analysed paths.
    pub fn process(
        &self,
        errors: Vec<Error>,
        only_files: bool,
        analysed_files: &[String],
        has_internal_errors: bool,
    ) -> Processed {
        let mut unmatched: Vec<Option<Value>> =
            self.entries.iter().map(|(_, r)| Some(r.clone())).collect();
        let mut out = Processed::default();
        // entry index => (times matched, its `count`)
        let mut real_counts: HashMap<usize, (i64, i64)> = HashMap::new();
        let mut matched_at: HashMap<usize, (String, Option<i64>)> = HashMap::new();

        // Returns whether the error stays reported.
        let mut process_one = |error: &Error, i: usize, string_errors: &mut Vec<String>| -> bool {
            let ignore = &self.entries[i].0;
            let fields: Fields = (
                str_of(isset(ignore, "message")),
                str_of(isset(ignore, "rawMessage")),
                str_of(isset(ignore, "identifier")),
                str_of(isset(ignore, "rule")),
            );
            let mut ignored = false;
            if let Value::Array(_) = ignore {
                if let Some(path) = str_of(isset(ignore, "path")) {
                    ignored = self.should_ignore(error, i, &fields, Some(path));
                    if ignored {
                        if let Some(Value::Int(count)) = isset(ignore, "count") {
                            let (real, _) = real_counts.entry(i).or_insert((0, *count));
                            *real += 1;
                            matched_at
                                .entry(i)
                                .or_insert_with(|| (error.file.clone(), error.line));
                            if *real > *count {
                                ignored = false;
                            }
                        } else {
                            unmatched[i] = None;
                        }
                    }
                } else if let Some(Value::Array(paths)) = isset(ignore, "paths") {
                    for (j, path) in paths.iter() {
                        ignored = self.should_ignore(error, i, &fields, path.to_php_string());
                        if !ignored {
                            continue;
                        }
                        if let Some(Value::Array(u)) = &mut unmatched[i]
                            && let Some(Value::Array(left)) = u.get_mut(&Key::from("paths"))
                        {
                            left.remove(j);
                            if left.is_empty() {
                                unmatched[i] = None;
                            }
                        }
                        break;
                    }
                } else {
                    ignored = self.should_ignore(error, i, &fields, None);
                    if ignored {
                        unmatched[i] = None;
                    }
                }
            } else {
                let fields = (ignore.to_php_string(), None, None, None);
                ignored = self.should_ignore(error, i, &fields, None);
                if ignored {
                    unmatched[i] = None;
                }
            }
            if ignored && !error.can_be_ignored {
                string_errors.push(format!(
                    "Error message \"{}\" cannot be ignored, use excludePaths instead.",
                    error.message
                ));
                return true;
            }
            !ignored
        };

        'errors: for error in errors {
            let mut candidates: Vec<usize> = Vec::new();
            if let Some(ids) = self.by_file.get(&normalize_path(&error.file_path)) {
                candidates.extend(ids);
            }
            if let Some(ids) = error
                .trait_file_path
                .as_deref()
                .and_then(|t| self.by_file.get(&normalize_path(t)))
            {
                candidates.extend(ids);
            }
            candidates.extend(&self.other);
            for i in candidates {
                if !process_one(&error, i, &mut out.string_errors) {
                    out.ignored.push((error.clone(), self.entries[i].0.clone()));
                    continue 'errors;
                }
            }
            out.errors.push(error);
        }

        let times = |n: i64| if n == 1 { "time" } else { "times" };
        let count_error = |i: usize, message: String| {
            let (file, line) = matched_at.get(&i).cloned().unwrap_or_default();
            Error::new(message, file, line).non_ignorable("ignore.count")
        };
        for (i, u) in unmatched.iter().enumerate() {
            let (Some(u), Some(&(real, count))) = (u, real_counts.get(&i)) else {
                continue;
            };
            if real > count {
                out.errors.push(count_error(
                    i,
                    format!(
                        "{} {} is expected to occur {count} {}, but occurred {real} {}.",
                        label(u),
                        stringify(u),
                        times(count),
                        times(real)
                    ),
                ));
            }
        }
        if has_internal_errors {
            return out;
        }
        for (i, u) in unmatched.iter().enumerate() {
            let Some(u) = u else { continue };
            let report = isset(u, "reportUnmatched")
                .cloned()
                .unwrap_or(Value::Bool(self.report_unmatched));
            if report == Value::Bool(false) {
                continue;
            }
            let real_path = str_of(isset(u, "realPath"));
            let not_matched = || {
                format!(
                    "{} {} was not matched in reported errors.",
                    label(u),
                    stringify(u)
                )
            };
            match real_counts.get(&i) {
                Some(&(real, count)) if real_path.is_some() || !only_files => {
                    if real < count {
                        out.errors.push(count_error(
                            i,
                            format!(
                                "{} {} is expected to occur {count} times, but occurred only {real} {}.",
                                label(u),
                                stringify(u),
                                times(real)
                            ),
                        ));
                    }
                }
                _ => match real_path {
                    Some(p) => {
                        if analysed_files.contains(&p) && !only_files {
                            out.errors.push(
                                Error::new(not_matched(), p, None)
                                    .non_ignorable("ignore.unmatched"),
                            );
                        }
                    }
                    // without a file of its own, the error is reported in the config file the entry is in
                    None if !only_files => match self.entry_location(u) {
                        Some((file, line)) => out.errors.push(
                            Error::new(not_matched(), file, Some(line)).non_ignorable("ignore.unmatched"),
                        ),
                        None => out.string_errors.push(not_matched()),
                    },
                    None => {}
                },
            }
        }
        out
    }
}

/// `LocalIgnoresProcessor::process()` for one analysed file: `lines` / `unmatched` are that file's `linesToIgnore` /
/// `unmatchedLineIgnores` (errors whose `getFile()` is another file, e.g. trait contexts, are not ignored here).
/// Returns (errors still reported, locally ignored errors).
pub fn local_ignores(
    file: &str,
    errors: Vec<Error>,
    lines: &mut LinesToIgnore,
    unmatched: &mut LinesToIgnore,
) -> (Vec<Error>, Vec<Error>) {
    let mut kept = Vec::new();
    let mut ignored = Vec::new();
    for error in errors {
        let entry = match error.line {
            Some(line) if error.can_be_ignored && error.file == file => {
                lines.get(&line).map(|ids| (line, ids.clone()))
            }
            _ => None,
        };
        let Some((line, ids)) = entry else {
            kept.push(error);
            continue;
        };
        let Some(mut ids) = ids else {
            unmatched.shift_remove(&line);
            ignored.push(error);
            continue;
        };
        let Some(id) = &error.identifier else {
            kept.push(error);
            continue;
        };
        let Some(pos) = ids.iter().position(|i| i.name == *id) else {
            kept.push(error);
            continue;
        };
        ids.remove(pos);
        if ids.is_empty() {
            lines.shift_remove(&line);
        } else {
            lines.insert(line, Some(ids));
        }
        if let Some(Some(left)) = unmatched.get_mut(&line)
            && let Some(j) = left.iter().position(|i| i.name == *id)
        {
            left.remove(j);
            if left.is_empty() {
                unmatched.shift_remove(&line);
            }
        }
        ignored.push(error);
    }
    (kept, ignored)
}

/// `reportIgnoresWithoutComments: true` (FileAnalyser): errors for comment-less ignores, which are then dropped from
/// `lines` and `unmatched`.
pub fn ignores_without_comments(
    file: &str,
    lines: &mut LinesToIgnore,
    unmatched: &mut LinesToIgnore,
) -> Vec<Error> {
    let mut errors = Vec::new();
    for (line, ids) in lines.clone() {
        let error = |message: String, tip: &str, identifier: &str| Error {
            tip: Some(tip.into()),
            ..Error::new(message, file, Some(line)).non_ignorable(identifier)
        };
        let Some(ids) = ids else {
            errors.push(error(
                "Ignoring all errors on a line is not allowed.".into(),
                "Use @phpstan-ignore with an identifier and a comment (in parentheses).",
                "ignore.allLineErrors",
            ));
            lines.shift_remove(&line);
            unmatched.shift_remove(&line);
            continue;
        };
        for Identifier { name, comment } in ids {
            if comment.is_none() {
                errors.push(error(
                    format!("Ignore with identifier {name} has no comment."),
                    "Explain why this ignore is necessary in parentheses after the identifier.",
                    "ignore.noComment",
                ));
                lines.shift_remove(&line);
                unmatched.shift_remove(&line);
            }
        }
    }
    errors
}

/// `reportUnmatchedIgnoredErrors` (AnalyserResultFinalizer): errors for the ignore comments nothing matched.
pub fn unmatched_line_errors(file: &str, unmatched: &LinesToIgnore) -> Vec<Error> {
    let mut errors = Vec::new();
    for (&line, ids) in unmatched {
        let error = |message: String, identifier: &str| {
            Error::new(message, file, Some(line)).non_ignorable(identifier)
        };
        match ids {
            None => errors.push(error(
                format!("No error to ignore is reported on line {line}."),
                "ignore.unmatchedLine",
            )),
            Some(ids) => errors.extend(ids.iter().map(|id| {
                error(
                    format!(
                        "No error with identifier {} is reported on line {line}.",
                        id.name
                    ),
                    "ignore.unmatchedIdentifier",
                )
            })),
        }
    }
    errors
}

/// `IgnoreParseErrorRule`: the `@phpstan-ignore` comments that did not parse.
pub fn parse_errors(file: &str, errors: &IndexMap<i64, Vec<String>>) -> Vec<Error> {
    errors
        .iter()
        .flat_map(|(&line, messages)| {
            messages.iter().map(move |m| {
                Error::new(
                    format!("Parse error in @phpstan-ignore: {m}"),
                    file,
                    Some(line),
                )
                .non_ignorable("ignore.parseError")
            })
        })
        .collect()
}

/// PackageDependencyResolver::isFileOfInstalledPackage() over the projects' `vendor/composer/installed.json` (the
/// install paths Composer also writes to installed.php).
struct InstalledPackages {
    vendor_dirs: Vec<String>,
    /// Install paths, longest first, and whether the package came from a path repository.
    packages: Vec<(String, bool)>,
}

impl InstalledPackages {
    fn of(project_paths: &[String]) -> InstalledPackages {
        let mut vendor_dirs = Vec::new();
        let mut packages = Vec::new();
        let json = |p: &str| -> Option<serde_json::Value> { serde_json::from_slice(&std::fs::read(p).ok()?).ok() };
        for root in project_paths {
            // ComposerHelper::getComposerConfig() / getVendorDirFromComposerConfig()
            let Some(composer) = json(&format!("{root}/composer.json")) else {
                continue;
            };
            let vendor = composer["config"]["vendor-dir"].as_str().unwrap_or("vendor").trim_matches('/').to_string();
            let vendor = format!("{root}/{vendor}");
            vendor_dirs.push(normalize_path(&vendor));
            let installed = json(&format!("{vendor}/composer/installed.json"));
            let list = installed.as_ref().and_then(|i| i.get("packages").unwrap_or(i).as_array().cloned()).unwrap_or_default();
            for p in list {
                let Some(path) = p["install-path"].as_str() else {
                    continue;
                };
                let path_package = p["dist"]["type"].as_str() == Some("path");
                let at = normalize_path(&format!("{vendor}/composer/{path}"));
                if let Ok(real) = std::fs::canonicalize(&at) {
                    let real = normalize_path(&real.to_string_lossy());
                    if real != at {
                        packages.push((real, path_package));
                    }
                }
                packages.push((at, path_package));
            }
        }
        packages.sort_by_key(|(p, _)| std::cmp::Reverse(p.len()));
        InstalledPackages { vendor_dirs, packages }
    }

    fn has_file(&self, file: &str) -> bool {
        let file = normalize_path(file);
        if !self.vendor_dirs.iter().any(|v| file.starts_with(v.as_str())) {
            return false;
        }
        // a file of no package, or of one that is not the project's own code
        self.packages
            .iter()
            .find(|(p, _)| file.starts_with(&format!("{p}/")))
            .is_none_or(|(_, path_package)| !path_package)
    }
}
