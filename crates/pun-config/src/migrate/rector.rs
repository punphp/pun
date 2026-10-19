//! `pun migrate`, lint part: a rector.php (as Rector resolves it) becomes `[lint]` (context/plan/migrate.md §3 and
//! Build 4). Rector's prepared sets become `[lint] sets` one to one and its PHP sets the `modernize` set; then every
//! rule Rector runs is matched to its Pun rule through rules/mapping.toml, and [`compose`] writes the difference
//! between what the sets turn on and what Rector ran as `[lint.rules]` lines, so the pun.toml runs exactly Rector's
//! rules. A rule skipped everywhere is `"off"`, one skipped for some paths is off in a `[[lint.overrides]]`, and a
//! skipped path goes to `[lint] exclude`. What Pun can't run is named and skipped.

use super::Parts;
use crate::composer::Composer;
use crate::{Config, mapping, rules};
use anyhow::Result;
use serde::Deserialize;
use serde_json::{Map, Value};
use std::collections::{BTreeMap, HashSet};
use std::fmt::Write;
use std::path::Path;

/// A rector.php as Rector resolves it, as pun-cli's `rector_config.php` prints it.
#[derive(Debug, Default, Deserialize)]
pub struct RectorConfig {
    /// Every rule Rector runs (`RectorConfig::getMainRectorClasses()`), sets included, in registration order.
    pub rules: Vec<String>,
    /// The rules registered with a configuration.
    #[serde(default)]
    pub configured: Vec<String>,
    /// `withSkip()` rule entries: class => `null` (everywhere) or the paths it is skipped for.
    #[serde(default)]
    pub skip_rules: Map<String, Value>,
    /// `withSkip()` path entries (absolute, or patterns).
    #[serde(default)]
    pub skip_paths: Vec<String>,
    /// `withPaths()` (and `withRootFiles()`), absolute.
    #[serde(default)]
    pub paths: Vec<String>,
    /// The set files the config loaded.
    #[serde(default)]
    pub sets: Vec<String>,
    /// `withPhpSets()` or `withPhpLevel()`.
    #[serde(default)]
    pub php_sets: bool,
}

/// What a rector.php becomes before [`compose`] reconciles it with the sets.
#[derive(Debug, Default)]
pub struct Rector {
    pub sets: Vec<&'static str>,
    /// The Pun rules of the Rector rules that run everywhere, each with its Rector class's short name.
    pub want: Vec<(String, String)>,
    /// Rules off for some paths: (paths relative to the project, rule ID).
    pub overrides: Vec<(Vec<String>, String)>,
    /// Skipped paths, relative to the project.
    pub exclude: Vec<String>,
    /// `withPaths()`, relative to the project (`.` for the project itself).
    pub paths: Vec<String>,
    /// What Pun can't run: (Rector class short name, why).
    pub unsupported: Vec<(String, String)>,
}

/// Rector's prepared set files (by file name) and the Pun set each is.
const PREPARED: [(&str, &str); 7] = [
    ("dead-code.php", "dead-code"),
    ("code-quality.php", "code-quality"),
    ("coding-style.php", "coding-style"),
    ("type-declaration.php", "types"),
    ("privatization.php", "privatization"),
    ("naming.php", "naming"),
    ("early-return.php", "early-return"),
];

/// The `[upstream.<rector tool>]` entries by fully qualified class.
fn by_class() -> &'static BTreeMap<String, toml::Value> {
    static T: std::sync::LazyLock<BTreeMap<String, toml::Value>> = std::sync::LazyLock::new(|| {
        let mut out = BTreeMap::new();
        for (tool, s) in mapping::under("upstream") {
            if !(tool.starts_with("rector") || tool == "pest-plugin-rector") {
                continue;
            }
            for e in s.table().values() {
                if let Some(c) = e.get("class").and_then(toml::Value::as_str) {
                    out.insert(c.to_string(), e.clone());
                }
            }
        }
        out
    });
    &T
}

fn short(class: &str) -> String {
    class.rsplit('\\').next().unwrap().to_string()
}

/// `path` relative to `root`, without a trailing slash; `.` for the root itself. A path outside it stays as it is.
fn relative(root: &Path, path: &str) -> String {
    let p = Path::new(path);
    match p.strip_prefix(root) {
        Ok(r) if r.as_os_str().is_empty() => ".".into(),
        Ok(r) => r.display().to_string(),
        Err(_) => path.trim_end_matches('/').to_string(),
    }
}

/// What `r` becomes: its sets, the Pun rules of what it runs, its skips and paths, and what Pun can't run.
pub fn from_rector(root: &Path, r: &RectorConfig) -> Rector {
    let mut out = Rector::default();
    for set in &r.sets {
        let file = set.rsplit('/').next().unwrap();
        if let Some((_, pun)) = PREPARED.iter().find(|(f, _)| *f == file)
            && set.contains("/rector/rector/")
            && !out.sets.contains(pun)
        {
            out.sets.push(pun);
        }
    }
    if r.php_sets {
        out.sets.push("modernize");
    }
    let mut seen = HashSet::new();
    for class in &r.rules {
        let skip = r.skip_rules.get(class);
        if skip.is_some_and(Value::is_null) {
            continue;
        }
        let entry = by_class().get(class);
        let pun = entry
            .and_then(|e| e.get("pun"))
            .and_then(toml::Value::as_str);
        let why = match (entry, pun) {
            (None, _) => Some("not a Rector rule Pun knows (a custom rule or another package's)"),
            (Some(e), None) => e
                .get("unsupported")
                .or(e.get("drop"))
                .and_then(toml::Value::as_str),
            (Some(_), Some(_)) if r.configured.contains(class) => {
                Some("configured; its configuration can't be migrated yet")
            }
            _ => None,
        };
        if let Some(why) = why {
            out.unsupported.push((short(class), why.to_string()));
            continue;
        }
        let id = pun.unwrap().to_string();
        if let Some(paths) = skip.and_then(Value::as_array) {
            let paths = (paths.iter().filter_map(Value::as_str))
                .map(|p| relative(root, p))
                .collect();
            out.overrides.push((paths, id.clone()));
        }
        if seen.insert(id.clone()) {
            out.want.push((id, short(class)));
        }
    }
    out.exclude = r.skip_paths.iter().map(|p| relative(root, p)).collect();
    out.paths = r.paths.iter().map(|p| relative(root, p)).collect();
    out
}

/// What [`compose`] wrote for the Rector rules: lines turning rules on and off, and the rules Pun runs through the
/// sets alone.
#[derive(Debug, Default, PartialEq)]
pub struct Reconciled {
    pub on: usize,
    pub off: usize,
    /// Rector rules whose Pun rule is not ported yet: the sets turn them on once it is.
    pub waiting: usize,
    /// Rector rules no set covers whose Pun rule is not ported yet: (class short name, why).
    pub unsupported: Vec<(String, String)>,
}

/// `parts` with `rector`'s `[lint]` (sets, exclude, overrides) added and `[lint.rules]` reconciled: what the sets and
/// the lines already there turn on (resolved against the project's composer.json and composer.lock, so ecosystem and
/// upgrade rules count only when they run) against what Rector ran. A rule Rector ran that nothing turns on gets
/// `"error"` (or is named when Pun can't run it yet); one the sets turn on that Rector didn't run gets `"off"`.
pub fn compose(root: &Path, parts: &mut Parts, rector: &Rector) -> Result<Reconciled> {
    let quoted = |v: &[String]| toml::Value::try_from(v).unwrap().to_string();
    if !rector.sets.is_empty() {
        let sets: Vec<String> = rector.sets.iter().map(|s| s.to_string()).collect();
        writeln!(parts.lint, "sets = {}", quoted(&sets)).unwrap();
    }
    if !rector.exclude.is_empty() {
        writeln!(parts.lint, "exclude = {}", quoted(&rector.exclude)).unwrap();
    }
    let mut out = Reconciled::default();
    let cfg = resolve(root, &parts.render())?;
    let on: Vec<&str> = cfg.lint.rules.iter().map(|r| r.id.as_str()).collect();
    let given: HashSet<String> = (parts.lint_rules.iter())
        .map(|(k, _)| format!("lint/{k}"))
        .collect();
    let want: HashSet<&str> = rector.want.iter().map(|w| w.0.as_str()).collect();
    for (id, class) in &rector.want {
        if given.contains(id) {
            continue;
        }
        if on.contains(&id.as_str()) {
            out.waiting += usize::from(!ported(id));
        } else if super::require_lint(parts, id) {
            out.on += 1;
        } else {
            let why = "not supported by Pun yet".to_string();
            out.unsupported.push((class.clone(), why));
        }
    }
    for id in on {
        if !want.contains(id) && !given.contains(id) {
            parts
                .lint_rules
                .push((id[5..].to_string(), "\"off\"".into()));
            out.off += 1;
        }
    }
    let mut by_paths: Vec<(&Vec<String>, Vec<&str>)> = vec![];
    for (paths, id) in &rector.overrides {
        match by_paths.iter_mut().find(|(p, _)| *p == paths) {
            Some((_, ids)) => ids.push(&id[5..]),
            None => by_paths.push((paths, vec![&id[5..]])),
        }
    }
    for (paths, ids) in by_paths {
        let rules: Vec<String> = ids.iter().map(|id| format!("\"{id}\" = \"off\"")).collect();
        write!(
            parts.overrides,
            "{}[[lint.overrides]]\nfiles = {}\nrules = {{ {} }}\n",
            if parts.overrides.is_empty() { "" } else { "\n" },
            quoted(paths),
            rules.join(", ")
        )
        .unwrap();
    }
    Ok(out)
}

/// Whether mapping.toml marks a rule as ported.
pub(super) fn ported(id: &str) -> bool {
    (rules().iter().find(|(r, _)| r == id))
        .and_then(|(_, e)| e.get("ported"))
        .and_then(toml::Value::as_bool)
        == Some(true)
}

/// pun.toml text as `pun_config::load` resolves it in `root`: gated on its composer.json and composer.lock.
pub(super) fn resolve(root: &Path, text: &str) -> Result<Config> {
    let composer: Composer = crate::composer::detect(root)?;
    crate::parse(
        root,
        &root.join("pun.toml"),
        text,
        &crate::Options::default(),
        &composer,
        vec![],
    )
}
