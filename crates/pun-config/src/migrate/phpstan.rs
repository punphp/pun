//! `pun migrate`, analyse part: a phpstan.neon becomes `[analyse]` (context/plan/migrate.md §3 and Build 5). The config
//! is resolved the way `phpstan analyse` resolves it (pun-phpstan), except that baseline files read as empty: baselines
//! are never copied. The level becomes the analyse preset that holds it, plus `"off"` lines for the preset's rules above
//! the level; the built-in extensions (Larastan, strict-rules, deprecation-rules) count only when the config includes
//! them; `paths` and `excludePaths` carry over, and `ignoreErrors` become `[[analyse.ignore]]` tables under Pun rule IDs.
//! Every other include and every parameter set by hand is named and skipped.

use super::Parts;
use super::rector::resolve;
use crate::{AnalysePreset, mapping, rules};
use anyhow::Result;
use pun_phpstan::config::{Options, ProjectFiles, resolve_with};
use pun_phpstan::di::Files;
use pun_phpstan::{Array, Value};
use std::collections::BTreeSet;
use std::fmt::Write;
use std::path::Path;

/// What a phpstan.neon becomes.
#[derive(Debug, Default)]
pub struct Phpstan {
    /// PHPStan's level, 0 to 10 (`max` is 10).
    pub level: u8,
    /// `paths` and `excludePaths`, relative to the project.
    pub paths: Vec<String>,
    pub exclude: Vec<String>,
    /// `stubFiles`, relative to the project.
    pub stubs: Vec<String>,
    /// `[[analyse.ignore]]` tables, each with its header.
    pub ignores: Vec<String>,
    /// The baseline files the config includes (not copied).
    pub baselines: Vec<String>,
    /// What Pun can't take: (name, why).
    pub unsupported: Vec<(String, String)>,
    /// The built-in extensions the config includes, by package (`larastan/larastan`).
    pub extensions: BTreeSet<String>,
    /// The resolved parameters, for the rules a parameter turns on (`gated_by`).
    pub parameters: Array,
    /// Parameters the project changed from PHPStan's value at its level that rules read as options: (dotted name,
    /// value), for [`compose`] to write on those rules.
    pub changed: Vec<(String, Value)>,
}

/// The extensions Pun has built in, by package, and the mapping's `package` of their rule classes.
const BUILT_IN: [(&str, Option<&str>); 6] = [
    ("larastan/larastan", Some("larastan")),
    ("phpstan/phpstan-strict-rules", Some("phpstan-strict-rules")),
    (
        "phpstan/phpstan-deprecation-rules",
        Some("phpstan-deprecation-rules"),
    ),
    ("pestphp/pest-plugin-phpstan", None),
    ("phpstan/phpstan-phpunit", None),
    ("phpstan/extension-installer", None),
];

/// Parameters migrate reads, or that only steer PHPStan's own run (cache, parallelism, output).
const HANDLED: [&str; 19] = [
    "level",
    "stubFiles",
    "paths",
    "excludePaths",
    "ignoreErrors",
    "reportUnmatchedIgnoredErrors",
    "tmpDir",
    "parallel",
    "memoryLimitFile",
    "editorUrl",
    "editorUrlTitle",
    "errorFormat",
    "tips",
    "fileExtensions",
    "bootstrapFiles",
    "scanFiles",
    "scanDirectories",
    "resultCachePath",
    "resultCacheChecksProjectExtensionFilesDependencies",
];

/// Files as PHPStan reads them, with every baseline file empty; the baselines met are recorded. `virtual_file` is a
/// config only migrate writes (PHPStan's defaults for the project's level and packages).
struct WithoutBaselines {
    files: ProjectFiles,
    seen: std::cell::RefCell<BTreeSet<String>>,
    virtual_file: std::cell::RefCell<Option<(String, String)>>,
}

fn is_baseline(path: &str) -> bool {
    let name = path.rsplit('/').next().unwrap_or(path);
    name.to_ascii_lowercase().contains("baseline")
}

impl Files for WithoutBaselines {
    fn read(&self, path: &str) -> Option<Vec<u8>> {
        if let Some((p, text)) = &*self.virtual_file.borrow()
            && p == path
        {
            return Some(text.clone().into_bytes());
        }
        if is_baseline(path) && self.files.is_file(path) {
            self.seen.borrow_mut().insert(path.to_string());
            return Some(vec![]);
        }
        self.files.read(path)
    }

    fn is_file(&self, path: &str) -> bool {
        (self.virtual_file.borrow().as_ref()).is_some_and(|(p, _)| p == path)
            || self.files.is_file(path)
    }
}

fn relative(root: &Path, path: &str) -> String {
    let root = root.display().to_string();
    match path.strip_prefix(&root) {
        Some("") => ".".into(),
        Some(r) if r.starts_with('/') => r[1..].trim_end_matches('/').to_string(),
        _ => path.to_string(),
    }
}

fn strings(v: Option<&Value>) -> Vec<String> {
    match v {
        Some(Value::Str(s) | Value::OptionalPath(s)) => vec![s.clone()],
        Some(Value::Array(a)) => a.values().flat_map(|v| strings(Some(v))).collect(),
        _ => vec![],
    }
}

/// `config` (a phpstan.neon in `root`) as pun.toml's `[analyse]`. PHPStan's own config errors stop the migration with
/// PHPStan's message.
pub fn from_phpstan(root: &Path, config: &Path) -> Result<Phpstan> {
    let cwd = root.display().to_string();
    let mut o = Options::for_project(&cwd);
    o.config = Some(config.display().to_string());
    let phar = std::fs::canonicalize(&o.phar)
        .map_or_else(|_| o.phar.clone(), |p| p.to_string_lossy().into_owned());
    o.phar = phar.clone();
    let files = WithoutBaselines {
        files: ProjectFiles::new(&phar),
        seen: Default::default(),
        virtual_file: Default::default(),
    };
    let resolved = resolve_with(&o, &files).map_err(|e| anyhow::anyhow!(e))?;
    let p = &resolved.parameters;
    let mut out = Phpstan {
        baselines: (files.seen.borrow().iter())
            .map(|b| relative(root, b))
            .collect(),
        ..Phpstan::default()
    };
    // PHPStan itself rejects any other level (no `config.level<n>.neon`).
    out.level = match p.get_str("level").and_then(Value::to_php_string).as_deref() {
        Some("max") => 10,
        l => l.and_then(|l| l.parse().ok()).unwrap_or(0),
    };
    out.paths = strings(p.get_str("paths"))
        .iter()
        .map(|s| relative(root, s))
        .collect();
    let exclude = (p.get_str("excludePaths").and_then(Value::as_array))
        .cloned()
        .unwrap_or_default();
    out.exclude = strings(exclude.get_str("analyseAndScan"))
        .into_iter()
        .chain(strings(exclude.get_str("analyse")))
        .map(|s| relative(root, &s))
        .collect();
    let includes = includes(root, config);
    for inc in &includes {
        if is_baseline(inc) {
            continue;
        }
        let rel = relative(root, inc);
        match package(&rel) {
            Some(pkg) if pkg == "phpstan/phpstan" => {
                out.unsupported
                    .extend((inc.ends_with("/bleedingEdge.neon")).then(|| {
                        (
                            rel,
                            "Pun checks as PHPStan does without bleeding edge".into(),
                        )
                    }))
            }
            Some(pkg) if BUILT_IN.iter().any(|b| b.0 == pkg) => {
                out.extensions.insert(pkg);
            }
            Some(pkg) => out
                .unsupported
                .push((pkg, "a PHPStan extension Pun doesn't have built in".into())),
            None => out.unsupported.push((
                rel,
                "the project's own PHPStan config (services, rules, parameters)".into(),
            )),
        }
    }
    for pkg in installer_extensions(root) {
        if BUILT_IN.iter().any(|b| b.0 == pkg) {
            out.extensions.insert(pkg);
        } else if !out.unsupported.iter().any(|u| u.0 == pkg) {
            out.unsupported
                .push((pkg, "a PHPStan extension Pun doesn't have built in".into()));
        }
    }
    // What PHPStan resolves for the project's level and packages alone: a parameter the project sets to that value
    // changes nothing.
    let packages: Vec<&String> = (includes.iter())
        .filter(|i| package(&relative(root, i)).is_some())
        .collect();
    let mut text = String::from("includes:\n");
    for inc in &packages {
        writeln!(text, "    - '{inc}'").unwrap();
    }
    writeln!(text, "parameters:\n    level: {}", out.level).unwrap();
    let path = format!("{cwd}/.pun-migrate-defaults.neon");
    *files.virtual_file.borrow_mut() = Some((path.clone(), text));
    o.config = Some(path);
    let defaults = resolve_with(&o, &files)
        .map_err(|e| anyhow::anyhow!(e))?
        .parameters;
    let gates = gate_parameters();
    let table = mapping::section(&["phpstan-parameters"]).table();
    for key in own_parameters(root, config, &includes) {
        if HANDLED.contains(&key.as_str()) {
            continue;
        }
        let mut leaves = vec![];
        differences(&key, p.get_str(&key), defaults.get_str(&key), &mut leaves);
        for (name, value) in leaves {
            let entry = table.get(&name);
            let level = entry.is_some_and(|e| e.get("level").is_some());
            let why = match name.as_str() {
                _ if gates.contains(&name) => continue,
                _ if entry.is_some() && !level => {
                    out.changed.push((name, value));
                    continue;
                }
                _ if level => "the level sets it; Pun's analyse presets decide it",
                "phpVersion" => {
                    "Pun reads the PHP version from composer.json; [php] version overrides it"
                }
                _ => "no pun.toml setting for this parameter yet",
            };
            out.unsupported
                .push((format!("parameter {name}"), why.into()));
        }
    }
    out.stubs = (strings(p.get_str("stubFiles")).iter())
        .filter(|s| !s.starts_with("phar://") && !s.contains("/vendor/"))
        .map(|s| relative(root, s))
        .collect();
    out.parameters = p.clone();
    let ignore = (p.get_str("ignoreErrors").and_then(Value::as_array))
        .cloned()
        .unwrap_or_default();
    for e in ignore.values() {
        ignore_entry(root, e, &mut out);
    }
    Ok(out)
}

/// Every file `config` includes, depth first, as absolute paths; `%rootDir%`-relative PHPStan files included. PHPStan
/// rejects a file included twice, so each appears once.
fn includes(root: &Path, config: &Path) -> Vec<String> {
    let mut out = vec![];
    let mut todo = vec![config.display().to_string()];
    while let Some(file) = todo.pop() {
        let dir = Path::new(&file).parent().unwrap_or(root).to_path_buf();
        for inc in strings(neon(&file).get_str("includes")) {
            // As PHPStan normalizes it (`../vendor/...` from a subdirectory is a package's); an absolute include
            // replaces `dir`.
            let path = pun_phpstan::php::normalize_path(&dir.join(inc).display().to_string());
            out.push(path.clone());
            // A package's own includes belong to it.
            if package(&relative(root, &path)).is_none() {
                todo.push(path);
            }
        }
    }
    out
}

/// A neon file PHPStan has already read (so it exists and parses) as its top-level array.
fn neon(file: &str) -> Array {
    (std::fs::read(file).ok())
        .and_then(|b| pun_phpstan::neon::decode_bytes(&b).ok())
        .and_then(|v| v.as_array().cloned())
        .unwrap_or_default()
}

/// `vendor/<vendor>/<package>/...` as `<vendor>/<package>`.
fn package(rel: &str) -> Option<String> {
    let rest = rel.strip_prefix("vendor/")?;
    let mut parts = rest.splitn(3, '/');
    Some(format!("{}/{}", parts.next()?, parts.next()?))
}

/// The packages phpstan/extension-installer includes by itself (its GeneratedConfig::EXTENSIONS keys).
fn installer_extensions(root: &Path) -> Vec<String> {
    let path = root.join("vendor/phpstan/extension-installer/src/GeneratedConfig.php");
    let Ok(text) = std::fs::read_to_string(path) else {
        return vec![];
    };
    let after = text.split_once("EXTENSIONS").map_or("", |x| x.1);
    let mut out = vec![];
    for line in after.lines().skip(1) {
        let t = line.trim();
        if t.starts_with("const ") || t.starts_with("public const") {
            break;
        }
        if let Some(rest) = t.strip_prefix('\'')
            && let Some((name, after)) = rest.split_once('\'')
            && after.trim_start().starts_with("=>")
            && name.matches('/').count() == 1
            && !name.contains(' ')
        {
            out.push(name.to_string());
        }
    }
    out
}

/// The parameters the project's own files set (not PHPStan's, a package's or a baseline's).
fn own_parameters(root: &Path, config: &Path, includes: &[String]) -> BTreeSet<String> {
    let own = std::iter::once(config.display().to_string()).chain(
        (includes.iter())
            .filter(|i| package(&relative(root, i)).is_none() && !is_baseline(i))
            .cloned(),
    );
    own.flat_map(|file| {
        (neon(&file).get_str("parameters").and_then(Value::as_array))
            .map(|p| p.keys().map(|k| k.as_string()).collect::<Vec<_>>())
            .unwrap_or_default()
    })
    .collect()
}

/// The parameters a `gated_by` condition of `[upstream.phpstan-classes]` reads: their effect is the rules [`compose`]
/// turns on and off.
fn gate_parameters() -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for e in mapping::section(&["upstream", "phpstan-classes"])
        .table()
        .values()
    {
        for part in (e.get("gated_by").and_then(toml::Value::as_str))
            .into_iter()
            .flat_map(|g| g.split(" + "))
        {
            let (name, note) = part.split_once(" (").unwrap_or((part, ""));
            out.insert(name.to_string());
            out.extend(
                note.strip_prefix("featureToggles.")
                    .map(|t| format!("featureToggles.{}", t.trim_end_matches(')'))),
            );
        }
    }
    out
}

/// The leaves under `name` where the project's value `a` differs from PHPStan's `b`, by dotted name: string-keyed
/// tables are compared key by key, anything else as a whole.
fn differences(name: &str, a: Option<&Value>, b: Option<&Value>, out: &mut Vec<(String, Value)>) {
    let table = |v: Option<&Value>| {
        v.and_then(Value::as_array)
            .filter(|t| !t.is_list())
            .cloned()
    };
    match (table(a), table(b)) {
        (Some(x), Some(y)) => {
            let keys: BTreeSet<String> = x.keys().chain(y.keys()).map(|k| k.as_string()).collect();
            for k in keys {
                differences(&format!("{name}.{k}"), x.get_str(&k), y.get_str(&k), out);
            }
        }
        _ if a != b => out.push((name.to_string(), a.cloned().unwrap_or_default())),
        _ => {}
    }
}

/// One `ignoreErrors` entry as `[[analyse.ignore]]` tables: one per message and identifier it lists.
fn ignore_entry(root: &Path, e: &Value, out: &mut Phpstan) {
    // PHPStan's schema allows a string (a message) or an array.
    let empty = Array::new();
    let a = e.as_array().unwrap_or(&empty);
    let mut messages: Vec<(&str, String)> = vec![];
    messages.extend(e.as_str().map(|m| ("message", m.to_string())));
    for (key, kind) in [
        ("message", "message"),
        ("messages", "message"),
        ("rawMessage", "raw-message"),
        ("rawMessages", "raw-message"),
    ] {
        messages.extend(strings(a.get_str(key)).into_iter().map(|m| (kind, m)));
    }
    let identifiers: Vec<String> = strings(a.get_str("identifier"))
        .into_iter()
        .chain(strings(a.get_str("identifiers")))
        .collect();
    let paths: Vec<String> = strings(a.get_str("path"))
        .into_iter()
        .chain(strings(a.get_str("paths")))
        .map(|p| relative(root, &p))
        .collect();
    let count = a.get_str("count").and_then(Value::to_php_string);
    let messages = if messages.is_empty() {
        vec![None]
    } else {
        messages.into_iter().map(Some).collect()
    };
    let identifiers: Vec<Option<String>> = if identifiers.is_empty() {
        vec![None]
    } else {
        identifiers.into_iter().map(Some).collect()
    };
    let q = |s: &str| toml::Value::String(s.to_string()).to_string();
    for id in &identifiers {
        let mut rule = None;
        if let Some(id) = id {
            let entry = crate::identifiers::entry(id);
            let pun = entry
                .and_then(|e| e.get("pun"))
                .and_then(toml::Value::as_str);
            match pun {
                Some(p) if p.starts_with("analyse/") => {
                    let narrow = entry.unwrap().get("variant").is_some();
                    rule = Some((p[8..].to_string(), narrow.then(|| id.clone())));
                }
                Some(p) => {
                    out.unsupported.push((
                        format!("ignoreErrors {id}"),
                        format!(
                            "`{id}` is the {} rule `{p}` in Pun; ignore it there",
                            p.split('/').next().unwrap()
                        ),
                    ));
                    continue;
                }
                None => {
                    let why = (entry.and_then(|e| e.get("unsupported").or(e.get("drop"))))
                        .and_then(toml::Value::as_str)
                        .map_or(
                            format!("`{id}` is not a PHPStan identifier Pun knows"),
                            |w| format!("`{id}`: {w}"),
                        );
                    out.unsupported.push((format!("ignoreErrors {id}"), why));
                    continue;
                }
            }
        }
        for m in &messages {
            let mut t = String::from("[[analyse.ignore]]\n");
            if let Some((r, identifier)) = &rule {
                writeln!(t, "rule = {}", q(r)).unwrap();
                if let Some(i) = identifier {
                    writeln!(t, "identifier = {}", q(i)).unwrap();
                }
            }
            if let Some((kind, text)) = m {
                writeln!(t, "{kind} = {}", q(text)).unwrap();
            }
            if !paths.is_empty() {
                let list = toml::Value::try_from(&paths).unwrap();
                writeln!(t, "paths = {list}").unwrap();
            }
            if let Some(c) = &count {
                writeln!(t, "count = {c}").unwrap();
            }
            out.ignores.push(t);
        }
    }
}

/// Whether a `gated_by` condition of `[upstream.phpstan-classes]` holds for the resolved `parameters`: each part
/// (joined by ` + `) is a parameter path that must be true, `bleedingEdge` (its `featureToggles.<x>` when named), or a
/// Larastan parameter with its default in parentheses.
fn gate(condition: &str, parameters: &Array) -> bool {
    condition.split(" + ").all(|part| {
        let (name, note) = match part.split_once(" (") {
            Some((n, rest)) => (n, rest.trim_end_matches(')')),
            None => (part, ""),
        };
        let path = match (name, note.strip_prefix("featureToggles.")) {
            ("bleedingEdge", Some(toggle)) => format!("featureToggles.{toggle}"),
            ("bleedingEdge", None) => "featureToggles.bleedingEdge".into(),
            _ => name.to_string(),
        };
        let mut v = Some(&Value::Array(parameters.clone()));
        for k in path.split('.') {
            v = v.and_then(Value::as_array).and_then(|a| a.get_str(k));
        }
        match v {
            Some(v) => !matches!(v, Value::Null | Value::Bool(false) | Value::Int(0)),
            None => note.starts_with("default true"),
        }
    })
}

/// The Pun rule of every PHPStan rule class `phpstan` runs: its level is at most the config's, its package is
/// PHPStan's or an included extension's, and its `gated_by` parameter is on. Also the IDs of every rule some class
/// maps to, run or not.
fn wanted(phpstan: &Phpstan) -> (BTreeSet<String>, BTreeSet<String>) {
    let (mut run, mut mapped) = (BTreeSet::new(), BTreeSet::new());
    for e in mapping::section(&["upstream", "phpstan-classes"])
        .table()
        .values()
    {
        let Some(id) = e.get("pun").and_then(toml::Value::as_str) else {
            continue;
        };
        mapped.insert(id.to_string());
        let level = e.get("level").and_then(toml::Value::as_integer);
        let package = e
            .get("package")
            .and_then(toml::Value::as_str)
            .unwrap_or("phpstan");
        let included = package == "phpstan"
            || (BUILT_IN.iter()).any(|b| b.1 == Some(package) && phpstan.extensions.contains(b.0));
        let gated = (e.get("gated_by").and_then(toml::Value::as_str))
            .is_none_or(|g| gate(g, &phpstan.parameters));
        if included && gated && level.is_none_or(|l| l <= i64::from(phpstan.level)) {
            run.insert(id.to_string());
        }
    }
    (run, mapped)
}

/// What [`compose`] wrote for the analyse rules.
#[derive(Debug, PartialEq)]
pub struct Analysed {
    pub preset: AnalysePreset,
    pub on: usize,
    pub off: usize,
    /// strict-rules checks that are lint rules in Pun, turned on in `[lint.rules]`.
    pub lint: usize,
    /// Changed parameters written as rule options.
    pub options: usize,
    /// PHPStan checks Pun runs elsewhere or not yet: (rule, why).
    pub unsupported: Vec<(String, String)>,
}

/// `parts` with `phpstan`'s `[analyse]`: the preset holding its level, then `"on"` and `"off"` lines so that the
/// rules that run are exactly the ones PHPStan ran at that level with those extensions and parameters (resolved
/// against the project's composer.json and composer.lock, as the Laravel rules turn on with Laravel). A check that is
/// a lint rule in Pun (strict-rules' `empty()` ban) is turned on in `[lint.rules]`.
pub fn compose(root: &Path, parts: &mut Parts, phpstan: &Phpstan) -> Result<Analysed> {
    let preset = AnalysePreset::of_level(phpstan.level as usize);
    writeln!(parts.analyse, "preset = \"{preset}\"").unwrap();
    let list = |v: &[String]| toml::Value::try_from(v).unwrap().to_string();
    if !phpstan.paths.is_empty() {
        writeln!(parts.analyse, "paths = {}", list(&phpstan.paths)).unwrap();
    }
    if !phpstan.exclude.is_empty() {
        writeln!(parts.analyse, "exclude = {}", list(&phpstan.exclude)).unwrap();
    }
    if !phpstan.stubs.is_empty() {
        writeln!(parts.analyse, "stubs = {}", list(&phpstan.stubs)).unwrap();
    }
    let cfg = resolve(root, &parts.render())?;
    let on: BTreeSet<&str> = cfg.analyse.rules.iter().map(|r| r.id.as_str()).collect();
    let (run, mapped) = wanted(phpstan);
    let mut done = Analysed {
        preset,
        on: 0,
        off: 0,
        lint: 0,
        options: 0,
        unsupported: vec![],
    };
    // Larastan's rules are the `laravel` group: off as a group when the config doesn't include Larastan.
    let larastan = phpstan.extensions.contains("larastan/larastan");
    let laravel = cfg.analyse.ecosystems.contains(&crate::Ecosystem::Laravel);
    if !larastan && laravel {
        parts
            .analyse_rules
            .push(("laravel".into(), "\"off\"".into()));
        done.off += 1;
    }
    for (id, _) in rules().iter().filter(|(id, _)| id.starts_with("analyse/")) {
        if !mapped.contains(id) || (id.starts_with("analyse/laravel/") && !(larastan && laravel)) {
            continue;
        }
        match (on.contains(id.as_str()), run.contains(id)) {
            (true, false) => {
                parts
                    .analyse_rules
                    .push((id[8..].to_string(), "\"off\"".into()));
                done.off += 1;
            }
            (false, true) => {
                parts
                    .analyse_rules
                    .push((id[8..].to_string(), "\"on\"".into()));
                done.on += 1;
            }
            _ => {}
        }
    }
    // A format rule the format config already runs (PHPStan's BOM check is `encoding`) needs nothing.
    let formats = |id: &str| {
        cfg.rules
            .iter()
            .any(|r| crate::rule_of(&r.name) == Some(id))
    };
    for id in run
        .iter()
        .filter(|id| !id.starts_with("analyse/") && !formats(id))
    {
        let lint = id.starts_with("lint/");
        let turned_on = lint && super::require_lint(parts, id, "\"error\"");
        done.lint += usize::from(turned_on);
        let why = match lint {
            true => "a PHPStan check that is this lint rule in Pun, not supported yet",
            false => {
                "a PHPStan check that is this format rule in Pun; turn it on in [format.rules]"
            }
        };
        done.unsupported
            .extend((!turned_on).then(|| (id.clone(), why.to_string())));
    }
    options(root, parts, phpstan, &mut done)?;
    parts.analyse_ignores = phpstan.ignores.join("\n");
    Ok(done)
}

/// Each changed parameter as an option (`[phpstan-parameters]` of mapping.toml names it) of every rule that reads it
/// and runs: the rule's line becomes a table of its options.
fn options(root: &Path, parts: &mut Parts, phpstan: &Phpstan, done: &mut Analysed) -> Result<()> {
    let cfg = resolve(root, &parts.render())?;
    let running: BTreeSet<&str> = cfg.analyse.rules.iter().map(|r| &r.id[8..]).collect();
    let table = mapping::section(&["phpstan-parameters"]).table();
    let mut by_rule: Vec<(String, toml::Table)> = vec![];
    for (name, value) in &phpstan.changed {
        let value = toml_value(value);
        let entry = &table[name.as_str()];
        let option = entry["option"].as_str().unwrap();
        let rules = entry["rules"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(toml::Value::as_str);
        let mut applied = false;
        for rule in rules.map(|r| &r[8..]).filter(|r| running.contains(r)) {
            let at = match by_rule.iter().position(|(r, _)| r == rule) {
                Some(at) => at,
                None => {
                    by_rule.push((rule.to_string(), toml::Table::new()));
                    by_rule.len() - 1
                }
            };
            by_rule[at].1.insert(option.to_string(), value.clone());
            applied = true;
        }
        done.options += usize::from(applied);
    }
    for (rule, options) in by_rule {
        let line = toml::Value::Table(options).to_string();
        parts.analyse_rules.retain(|(k, _)| *k != rule);
        parts.analyse_rules.push((rule, line));
    }
    Ok(())
}

/// A parameter value as TOML. PHPStan's and Larastan's schemas (conf/parametersSchema.neon, extension.neon) declare
/// every parameter of `[phpstan-parameters]` that isn't the level's as `bool()` or `listOf(string())`, and `resolve`
/// has checked the value against them.
fn toml_value(v: &Value) -> toml::Value {
    match v {
        Value::Bool(b) => toml::Value::Boolean(*b),
        Value::Array(a) => toml::Value::Array(a.values().map(toml_value).collect()),
        v => toml::Value::String(v.as_str().unwrap_or_default().to_string()),
    }
}
