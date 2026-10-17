//! Which files are analysed and which are only scanned for symbols, as PHPStan decides it: `paths` minus
//! `excludePaths`, plus `scanFiles`, `scanDirectories` and what Composer autoloads (the project's composer.json and
//! vendor/composer/installed.json, as PHPStan's ComposerJsonAndInstalledJsonSourceLocatorMaker reads them).

use std::path::{Path, PathBuf};

use pun_phpstan::Array;
use pun_phpstan::Value;
use pun_phpstan::ignore::IgnoredErrors;
use pun_phpstan::php::FileExcluder;
use serde_json::Value as Json;

use crate::{display, php_files};

pub struct Project {
    /// Reported on.
    pub analysed: Vec<PathBuf>,
    /// Symbols only.
    pub scanned: Vec<PathBuf>,
    /// `PHP_VERSION_ID`.
    pub php_version: u32,
    /// `reportUnmatchedIgnoredErrors`.
    pub report_unmatched_ignores: bool,
    /// `reportIgnoresWithoutComments`.
    pub report_ignores_without_comments: bool,
    /// `ignoreErrors` (with the baseline's entries).
    pub ignore_errors: IgnoredErrors,
}

/// PHP 8.5.11, the oracle's runtime, which PHPStan uses when `phpVersion` is null.
pub const RUNTIME_PHP_VERSION: u32 = 80511;

fn strings<'a>(params: &'a Array, key: &str) -> Vec<&'a str> {
    params
        .get_str(key)
        .and_then(Value::as_array)
        .map(|a| a.values().filter_map(Value::as_str).collect())
        .unwrap_or_default()
}

impl Project {
    pub fn from_config(root: &Path, params: &Array) -> Project {
        let extensions: Vec<String> = strings(params, "fileExtensions")
            .into_iter()
            .map(str::to_string)
            .collect();
        let excludes = params.get_str("excludePaths").and_then(Value::as_array);
        let group = |k: &str| {
            excludes
                .and_then(|e| e.get_str(k))
                .and_then(Value::as_array)
                .map(|a| a.values().filter_map(Value::as_str).collect::<Vec<_>>())
                .unwrap_or_default()
        };
        let both = FileExcluder::new(group("analyseAndScan"));
        let analyse_only = FileExcluder::new(group("analyse"));
        let mut analysed = Vec::new();
        let mut scanned = Vec::new();
        for path in strings(params, "paths") {
            let mut found = Vec::new();
            let p = Path::new(path);
            if p.is_file() {
                found.push(p.to_path_buf());
            } else {
                php_files(p, &extensions, &mut found);
            }
            for f in found {
                let name = display(&f);
                if both.is_excluded(&name) {
                    continue;
                }
                if analyse_only.is_excluded(&name) {
                    scanned.push(f);
                } else {
                    analysed.push(f);
                }
            }
        }
        scanned.extend(strings(params, "scanFiles").into_iter().map(PathBuf::from));
        for dir in strings(params, "scanDirectories") {
            php_files(Path::new(dir), &extensions, &mut scanned);
        }
        scanned.extend(composer_files(root, &extensions));
        scanned.retain(|f| !both.is_excluded(&display(f)));
        let php_version = params
            .get_str("phpVersion")
            .and_then(|v| match v {
                Value::Int(i) => u32::try_from(*i).ok(),
                Value::Array(a) => match a.get_str("min") {
                    Some(Value::Int(i)) => u32::try_from(*i).ok(),
                    _ => None,
                },
                _ => None,
            })
            .unwrap_or(RUNTIME_PHP_VERSION);
        let flag = |k: &str| params.get_str(k) == Some(&Value::Bool(true));
        let report_unmatched_ignores = flag("reportUnmatchedIgnoredErrors");
        let empty = Array::new();
        let ignore_errors = params
            .get_str("ignoreErrors")
            .and_then(Value::as_array)
            .unwrap_or(&empty);
        Project {
            analysed,
            scanned,
            php_version,
            report_unmatched_ignores,
            report_ignores_without_comments: flag("reportIgnoresWithoutComments"),
            ignore_errors: IgnoredErrors::new(ignore_errors, report_unmatched_ignores),
        }
    }

    /// PHPStan's defaults (no `ignoreErrors`, unmatched ignores reported) over the given files.
    pub fn new(analysed: Vec<PathBuf>, scanned: Vec<PathBuf>) -> Project {
        Project {
            analysed,
            scanned,
            php_version: RUNTIME_PHP_VERSION,
            report_unmatched_ignores: true,
            report_ignores_without_comments: false,
            ignore_errors: IgnoredErrors::new(&Array::new(), true),
        }
    }
}

/// Files Composer can autoload: psr-4 and psr-0 directories, classmap entries and `files`, of the root package
/// (with autoload-dev) and every installed package.
fn composer_files(root: &Path, extensions: &[String]) -> Vec<PathBuf> {
    let read =
        |p: PathBuf| -> Option<Json> { serde_json::from_slice(&std::fs::read(p).ok()?).ok() };
    let mut out = Vec::new();
    let mut add = |base: &Path, autoload: &Json| {
        for kind in ["psr-4", "psr-0"] {
            for dirs in autoload[kind]
                .as_object()
                .into_iter()
                .flat_map(|m| m.values())
            {
                let dirs = match dirs {
                    Json::Array(a) => a.iter().filter_map(Json::as_str).collect(),
                    d => d.as_str().into_iter().collect::<Vec<_>>(),
                };
                for d in dirs {
                    php_files(&base.join(d), extensions, &mut out);
                }
            }
        }
        for entry in autoload["classmap"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Json::as_str)
        {
            let p = base.join(entry);
            if p.is_dir() {
                php_files(&p, extensions, &mut out);
            } else {
                out.push(p);
            }
        }
        for f in autoload["files"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Json::as_str)
        {
            out.push(base.join(f));
        }
    };
    if let Some(composer) = read(root.join("composer.json")) {
        add(root, &composer["autoload"]);
        add(root, &composer["autoload-dev"]);
    }
    let installed_dir = root.join("vendor/composer");
    if let Some(installed) = read(installed_dir.join("installed.json")) {
        let packages = installed.get("packages").unwrap_or(&installed);
        for p in packages.as_array().into_iter().flatten() {
            if let Some(path) = p["install-path"].as_str() {
                add(&installed_dir.join(path), &p["autoload"]);
            }
        }
    }
    // Composer's own classes, in every generated classmap (PHPStan finds it through the project's autoloader)
    // ponytail: other classes only the runtime autoloader knows (autoload_classmap.php) aren't scanned
    for class in ["InstalledVersions.php", "ClassLoader.php"] {
        let path = installed_dir.join(class);
        if path.is_file() {
            out.push(path);
        }
    }
    out
}
