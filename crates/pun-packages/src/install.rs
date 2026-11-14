//! `pun install`: makes `vendor/` match pun.lock. Packages already installed at the locked reference are kept, the
//! rest are fetched in parallel, packages no longer locked are removed, and then `vendor/composer` (installed.json,
//! installed.php, the autoloader), `vendor/bin` and the supported plugins' files are written as Composer writes them.

use crate::autoload::{self, PlatformCheck};
use crate::bins;
use crate::fetch::{Fetcher, link_path, remove, unpack};
use crate::installed::{Installed, installed_json, installed_php, read_installed, root};
use crate::lock::{Lock, content_hash};
use crate::paths::normalize;
use crate::plugins;
use crate::version::is_platform;
use anyhow::{Context, Result, bail};
use pcre2::bytes::Regex;
use rayon::prelude::*;
use serde_json::{Map, Value};
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct Options {
    /// Leave out `packages-dev` (`--no-dev`).
    pub no_dev: bool,
    /// `-o`; also on with `config.optimize-autoloader`.
    pub optimize: bool,
    /// `-a`; also on with `config.classmap-authoritative`.
    pub authoritative: bool,
    /// `--apcu-autoloader`; also on with `config.apcu-autoloader`.
    pub apcu: bool,
    /// `--apcu-autoloader-prefix`, which turns APCu on; Composer takes the prefix from no config key.
    pub apcu_prefix: Option<String>,
    pub no_autoloader: bool,
    pub no_plugins: bool,
    /// `--ignore-platform-req` values; with `ignore_all_platform` they are all ignored.
    pub ignore_platform: Vec<String>,
    pub ignore_all_platform: bool,
}

/// What changed, for the report.
#[derive(Debug, Default)]
pub struct Report {
    /// (name, version) newly installed.
    pub installed: Vec<(String, String)>,
    /// (name, old version, new version).
    pub updated: Vec<(String, String, String)>,
    pub removed: Vec<String>,
    /// Packages already at the locked reference.
    pub kept: usize,
    /// Archives taken from the cache rather than downloaded.
    pub cached: usize,
    pub classes: usize,
    pub stale_lock: bool,
    pub warnings: Vec<String>,
    pub vendor: String,
    pub bin_dir: String,
}

fn config_str<'a>(config: &'a Map<String, Value>, key: &str) -> Option<&'a str> {
    config.get(key).and_then(Value::as_str)
}

fn config_bool(config: &Map<String, Value>, key: &str) -> bool {
    config.get(key) == Some(&Value::Bool(true))
}

/// A config path with `{$vendor-dir}` expanded, made absolute under `base`.
fn config_path(base: &str, value: &str, vendor: &str) -> String {
    let value = value.replace("{$vendor-dir}", vendor);
    normalize(&if value.starts_with('/') {
        value
    } else {
        format!("{base}/{value}")
    })
}

/// The vendor and bin directories, absolute: `config.vendor-dir` and `config.bin-dir` under `base` (canonical).
pub fn dirs(base: &str, manifest: &Map<String, Value>) -> (String, String) {
    let config = manifest.get("config").and_then(Value::as_object);
    let get = |k: &str| config.and_then(|c| config_str(c, k));
    let vendor = config_path(base, get("vendor-dir").unwrap_or("vendor"), "");
    let bin_dir = config_path(base, get("bin-dir").unwrap_or("{$vendor-dir}/bin"), &vendor);
    (vendor, bin_dir)
}

/// The installed reference and version recorded for each package in `installed.json`.
fn previous(vendor: &Path) -> Result<HashMap<String, Map<String, Value>>> {
    Ok((read_installed(vendor)?.into_iter())
        .filter_map(|p| Some((p.get("name")?.as_str()?.to_ascii_lowercase(), p)))
        .collect())
}

fn version(p: &Map<String, Value>) -> String {
    p.get("version")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

fn reference(p: &Map<String, Value>) -> Option<String> {
    let get = |k: &str| {
        p.get(k)?
            .get("reference")?
            .as_str()
            .filter(|r| !r.is_empty())
            .map(String::from)
    };
    if p.contains_key("dist") {
        get("dist").or_else(|| get("source"))
    } else {
        get("source")
    }
}

/// Installs `lock` into `base` (the project directory), for the manifest `manifest` (composer.json's shape).
/// `events` runs the root's scripts for `pre-autoload-dump` and `post-autoload-dump`.
pub fn install(
    base: &Path,
    manifest: &Map<String, Value>,
    lock: &Lock,
    fetcher: &Fetcher,
    o: &Options,
    events: &mut dyn FnMut(&str) -> Result<()>,
) -> Result<Report> {
    let base_dir =
        std::fs::canonicalize(base).with_context(|| format!("can't open {}", base.display()))?;
    let base_str = base_dir.to_string_lossy().into_owned();
    let config = manifest
        .get("config")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let (vendor, bin_dir) = dirs(&base_str, manifest);
    let mut report = Report {
        vendor: vendor.clone(),
        bin_dir: bin_dir.clone(),
        stale_lock: content_hash(manifest) != lock.content_hash,
        ..Report::default()
    };

    let mut packages: Vec<Installed> = (lock.packages.iter().map(|p| (p, false)))
        .chain(
            lock.packages_dev
                .iter()
                .filter(|_| !o.no_dev)
                .map(|p| (p, true)),
        )
        .map(|(p, dev)| {
            let name = p
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_ascii_lowercase();
            let meta = p.get("type").and_then(Value::as_str) == Some("metapackage");
            Installed {
                package: p.clone(),
                path: (!meta).then(|| format!("{vendor}/{name}")),
                dev,
            }
        })
        .collect();
    static NAME: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
        Regex::new(r"^[a-z0-9](?:[_.-]?[a-z0-9]++)*+/[a-z0-9](?:(?:[_.]|-{1,2})?[a-z0-9]++)*+$")
            .unwrap()
    });
    for p in &packages {
        let name = p.name();
        if !NAME.is_match(name.as_bytes()).unwrap_or(false) {
            bail!("pun.lock has a package named `{name}`, which is not a valid package name");
        }
    }

    // Decide what to fetch; remove what is no longer locked.
    let vendor_path = Path::new(&vendor);
    let before = previous(vendor_path)?;
    let mut todo: Vec<usize> = vec![];
    for (i, p) in packages.iter().enumerate() {
        let name = p.name();
        let old = before.get(&name);
        let present = p.path.as_ref().is_none_or(|path| Path::new(path).exists());
        match old {
            Some(old)
                if present
                    && reference(old) == reference(&p.package)
                    && version(old) == version(&p.package) =>
            {
                report.kept += 1;
            }
            Some(old) => {
                report
                    .updated
                    .push((name, version(old), version(&p.package)));
                todo.push(i);
            }
            None => {
                report.installed.push((name, version(&p.package)));
                todo.push(i);
            }
        }
    }
    let locked: Vec<String> = packages.iter().map(Installed::name).collect();
    for (name, old) in &before {
        let bins = old
            .get("bin")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str);
        let gone = !locked.contains(name);
        let changed = todo.iter().any(|&i| packages[i].name() == *name);
        if gone || changed {
            for bin in bins {
                let _ = std::fs::remove_file(
                    Path::new(&bin_dir).join(bin.rsplit('/').next().unwrap_or(bin)),
                );
            }
        }
        if gone {
            if let Some(dir) = old.get("install-path").and_then(Value::as_str) {
                let dir = normalize(&format!("{vendor}/composer/{dir}"));
                if dir.starts_with(&format!("{vendor}/")) {
                    remove(Path::new(&dir))?;
                }
            }
            report.removed.push(name.clone());
        }
    }

    // Fetch and unpack. Downloads mostly wait on the network, so they get 48 threads (Composer's
    // `COMPOSER_MAX_PARALLEL_HTTP` sets another number), never fewer than the cores; each archive's entries are
    // unpacked in parallel on the same pool.
    let cores = std::thread::available_parallelism().map_or(8, usize::from);
    let threads = download_threads(std::env::var("COMPOSER_MAX_PARALLEL_HTTP").ok(), cores);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()?;
    let results: Vec<Result<bool>> = pool.install(|| {
        (todo.par_iter())
            .map(|&i| {
                let p = &packages[i];
                let Some(path) = &p.path else {
                    return Ok(false);
                };
                place(fetcher, &p.package, base, Path::new(path))
                    .with_context(|| format!("installing {}", p.name()))
            })
            .collect()
    });
    let mut errors = vec![];
    for r in results {
        match r {
            Ok(cached) => report.cached += usize::from(cached),
            Err(e) => errors.push(format!("{e:#}")),
        }
    }
    if !errors.is_empty() {
        bail!("{}", errors.join("\n"));
    }

    // vendor/composer: what is installed.
    let repo = format!("{vendor}/composer");
    std::fs::create_dir_all(&repo)?;
    let dev_mode = !o.no_dev;
    write_if_changed(
        &format!("{repo}/installed.json"),
        installed_json(&packages, &vendor, dev_mode).as_bytes(),
    )?;
    let root_pkg = root(manifest, &base_dir);
    write_if_changed(
        &format!("{repo}/installed.php"),
        installed_php(&root_pkg, &packages, &vendor, &base_str, dev_mode).as_bytes(),
    )?;
    write_if_changed(
        &format!("{repo}/InstalledVersions.php"),
        include_bytes!("../assets/InstalledVersions.php"),
    )?;

    // vendor/bin, in Composer's install order: a name already taken is skipped.
    let mut claimed: HashSet<String> = HashSet::new();
    for i in install_order(&packages) {
        let p = &packages[i];
        let Some(path) = &p.path else { continue };
        let names: Vec<String> = (p
            .package
            .get("bin")
            .and_then(Value::as_array)
            .into_iter()
            .flatten())
        .filter_map(|b| b.as_str().map(String::from))
        .collect();
        let (proxies, warnings) = bins::proxies(&p.name(), path, &names, &bin_dir, &vendor);
        report.warnings.extend(warnings);
        for proxy in proxies {
            if !claimed.insert(proxy.name.clone()) {
                report.warnings.push(format!(
                    "Skipped installation of bin {} for package {}: name conflicts with an existing file",
                    proxy.bin,
                    p.name()
                ));
                continue;
            }
            bins::write(&proxy, Path::new(&bin_dir))?;
        }
    }

    // The autoloader.
    packages.sort_by_key(Installed::name);
    if !o.no_autoloader {
        let check = match config.get("platform-check") {
            Some(Value::Bool(false)) => PlatformCheck::Off,
            Some(Value::Bool(true)) => PlatformCheck::Full,
            _ => PlatformCheck::PhpOnly,
        };
        let autoload_file = format!("{vendor}/autoload.php");
        let existing = std::fs::read_to_string(&autoload_file)
            .ok()
            .and_then(|text| {
                let re = Regex::new(r"ComposerAutoloaderInit([^:\s]+)::").ok()?;
                let caps = re.captures(text.as_bytes()).ok()??;
                Some(String::from_utf8_lossy(&caps[1]).into_owned())
            });
        let suffix = config_str(&config, "autoloader-suffix")
            .map(String::from)
            .or(existing)
            .unwrap_or_else(|| lock.content_hash.clone());
        let apcu = (o.apcu || o.apcu_prefix.is_some() || config_bool(&config, "apcu-autoloader"))
            .then(|| o.apcu_prefix.clone());
        let options = autoload::Options {
            optimize: o.optimize || config_bool(&config, "optimize-autoloader"),
            authoritative: o.authoritative || config_bool(&config, "classmap-authoritative"),
            apcu,
            dev_mode,
            platform_check: check,
            ignore_platform: (!o.ignore_all_platform).then(|| o.ignore_platform.clone()),
            use_include_path: config_bool(&config, "use-include-path"),
            prepend: config.get("prepend-autoloader") != Some(&Value::Bool(false)),
            suffix,
        };
        events("pre-autoload-dump")?;
        let dumped = autoload::dump(&base_str, &vendor, manifest, &packages, &options)?;
        report.classes = dumped.classes;
        report.warnings.extend(dumped.warnings);
        for (rel, body) in dumped.files {
            let path = format!("{base_str}/{rel}");
            match body {
                Some(bytes) => write_if_changed(&path, &bytes)?,
                None => {
                    let _ = std::fs::remove_file(&path);
                }
            }
        }
        if !o.no_plugins {
            let refs: Vec<&Installed> = packages.iter().collect();
            let out = plugins::after_dump(manifest, &refs);
            for (rel, bytes) in out.files {
                write_if_changed(&format!("{vendor}/{rel}"), &bytes)?;
            }
            for name in out.unsupported {
                report.warnings.push(format!("pun does not run the Composer plugin {name} yet, so the files it writes on install are missing"));
            }
        }
        events("post-autoload-dump")?;
    }
    Ok(report)
}

/// Download threads: `COMPOSER_MAX_PARALLEL_HTTP` when it is a positive number, else 48, and never fewer than
/// `cores`.
fn download_threads(env: Option<String>, cores: usize) -> usize {
    let http = env.and_then(|v| v.parse::<usize>().ok()).filter(|n| *n > 0);
    http.unwrap_or(48).max(cores)
}

/// The names a package answers to: its own, and what it provides and replaces (`getNames`).
fn names(p: &Installed) -> Vec<String> {
    let mut out = vec![p.name()];
    for kind in ["provide", "replace"] {
        out.extend(link_targets(p, kind));
    }
    out
}

fn link_targets(p: &Installed, kind: &str) -> Vec<String> {
    (p.package
        .get(kind)
        .and_then(Value::as_object)
        .into_iter()
        .flatten())
    .map(|(target, _)| target.to_ascii_lowercase())
    .collect()
}

/// The order Composer's `Transaction::calculateOperations` installs `packages` in: depth first from the packages no
/// other one requires, taken in name order, each after what it requires; then `movePluginsToFront`.
fn install_order(packages: &[Installed]) -> Vec<usize> {
    let mut by_name: Vec<usize> = (0..packages.len()).collect();
    by_name.sort_by_key(|&i| std::cmp::Reverse(packages[i].name()));
    let mut providers: HashMap<String, Vec<usize>> = HashMap::new();
    for &i in &by_name {
        for name in names(&packages[i]) {
            providers.entry(name).or_default().push(i);
        }
    }
    let required = |i: usize| -> Vec<usize> {
        (link_targets(&packages[i], "require").iter())
            .flat_map(|t| providers.get(t).into_iter().flatten().copied())
            .collect()
    };
    let mut roots: Vec<usize> = by_name.clone();
    for &i in &by_name {
        if roots.contains(&i) {
            let required = required(i);
            roots.retain(|r| *r == i || !required.contains(r));
        }
    }
    let (mut visited, mut done) = (vec![false; packages.len()], vec![false; packages.len()]);
    let mut order = vec![];
    let mut stack = roots;
    while let Some(i) = stack.pop() {
        if done[i] {
            continue;
        }
        if visited[i] {
            done[i] = true;
            order.push(i);
        } else {
            visited[i] = true;
            stack.push(i);
            stack.extend(required(i));
        }
    }
    // Download-modifying plugins, then the other plugins, each with what they require, go first; a plugin that
    // requires no package goes before the plugins that do.
    let (mut front, mut rest): ([Vec<usize>; 4], Vec<usize>) = (Default::default(), vec![]);
    let mut wanted: [Vec<String>; 2] = Default::default();
    for &i in order.iter().rev() {
        let p = &packages[i];
        let kind = p.package.get("type").and_then(Value::as_str);
        let modifies = kind == Some("composer-plugin")
            && (p.package.get("extra")).and_then(|e| e.get("plugin-modifies-downloads"))
                == Some(&Value::Bool(true));
        let plugin = matches!(kind, Some("composer-plugin" | "composer-installer"));
        let names = names(p);
        let group = [modifies, plugin]
            .iter()
            .zip(&wanted)
            .position(|(is, want)| *is || names.iter().any(|n| want.contains(n)));
        let Some(g) = group else {
            rest.insert(0, i);
            continue;
        };
        let requires: Vec<String> = (link_targets(p, "require").into_iter())
            .filter(|r| !is_platform(r))
            .collect();
        if [modifies, plugin][g] && requires.is_empty() {
            front[2 * g].insert(0, i);
        } else {
            wanted[g].extend(requires);
            front[2 * g + 1].insert(0, i);
        }
    }
    front.into_iter().flatten().chain(rest).collect()
}

/// Puts one package's files at `target`; returns whether its archive came from the cache.
fn place(
    fetcher: &Fetcher,
    package: &Map<String, Value>,
    base: &Path,
    target: &Path,
) -> Result<bool> {
    let dist = package.get("dist").and_then(Value::as_object);
    match dist.and_then(|d| d.get("type")).and_then(Value::as_str) {
        Some("path") => {
            let url = dist
                .and_then(|d| d.get("url"))
                .and_then(Value::as_str)
                .context("path dist without a url")?;
            link_path(url, base, target)?;
            Ok(false)
        }
        Some(kind) => {
            let (bytes, cached) = fetcher.archive(package)?;
            unpack(kind, &bytes, target)?;
            Ok(cached)
        }
        None => {
            let kind = package
                .get("source")
                .and_then(|s| s.get("type"))
                .and_then(Value::as_str)
                .unwrap_or("none");
            bail!("it has no dist archive, and installing from {kind} sources is not supported yet")
        }
    }
}

fn write_if_changed(path: &str, bytes: &[u8]) -> Result<()> {
    if std::fs::read(path).ok().as_deref() == Some(bytes) {
        return Ok(());
    }
    std::fs::create_dir_all(Path::new(path).parent().unwrap_or(Path::new("/")))?;
    std::fs::write(path, bytes).with_context(|| format!("can't write {path}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Composer 2.11.2's order for the same packages (`composer install --dry-run` on a project of path packages):
    /// the download-modifying plugin first, then the plugin and installer with what they require, then the rest
    /// depth first, `virtual/thing` reaching e/prov through its `provide`, and the n/cyc ↔ o/cyc cycle entered at
    /// o/cyc, the one that sorts last.
    /// The reference that tells whether an installed package is current: the dist's, else the source's.
    #[test]
    fn references() {
        let r = |v: Value| reference(v.as_object().unwrap());
        assert_eq!(
            r(json!({"dist": {"reference": "d"}, "source": {"reference": "s"}})).as_deref(),
            Some("d")
        );
        assert_eq!(
            r(json!({"dist": {"reference": ""}, "source": {"reference": "s"}})).as_deref(),
            Some("s")
        );
        assert_eq!(
            r(json!({"source": {"reference": "s"}})).as_deref(),
            Some("s")
        );
        assert_eq!(r(json!({"dist": {"url": "u"}})), None);
    }

    #[test]
    fn download_threads_follow_composer_max_parallel_http() {
        assert_eq!(download_threads(None, 8), 48);
        assert_eq!(download_threads(Some("12".into()), 8), 12);
        assert_eq!(download_threads(Some("2".into()), 8), 8);
        assert_eq!(download_threads(Some("0".into()), 8), 48);
        assert_eq!(download_threads(Some("many".into()), 8), 48);
    }

    #[test]
    fn install_order_matches_composer() {
        let packages: Vec<Installed> = [
            json!({"name": "a/dep", "require": {"php": ">=8"}}),
            json!({"name": "b/dl", "type": "composer-plugin", "require": {"composer-plugin-api": "^2"}, "extra": {"plugin-modifies-downloads": true}}),
            json!({"name": "c/lib", "require": {"d/base": "*", "virtual/thing": "*"}}),
            json!({"name": "d/base"}),
            json!({"name": "e/prov", "provide": {"virtual/thing": "1.0"}}),
            json!({"name": "m/app", "require": {"c/lib": "*"}}),
            json!({"name": "n/cyc", "require": {"o/cyc": "*"}}),
            json!({"name": "o/cyc", "require": {"n/cyc": "*"}}),
            json!({"name": "x/helper"}),
            json!({"name": "y/inst", "type": "composer-installer", "require": {"x/helper": "*"}}),
            json!({"name": "z/plugin", "type": "composer-plugin", "require": {"composer-plugin-api": "^2", "a/dep": "*"}}),
        ]
        .into_iter()
        .map(|p| Installed {
            package: p.as_object().unwrap().clone(),
            path: None,
            dev: false,
        })
        .collect();
        let order: Vec<String> = (install_order(&packages).into_iter())
            .map(|i| packages[i].name())
            .collect();
        assert_eq!(
            order,
            [
                "b/dl", "x/helper", "y/inst", "a/dep", "z/plugin", "d/base", "e/prov", "c/lib",
                "m/app", "o/cyc", "n/cyc"
            ]
        );
    }
}
