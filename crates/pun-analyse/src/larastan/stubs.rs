//! LarastanStubFilesExtension: `stubs/common` and every `stubs/<version>` directory whose version is at most the
//! installed Laravel's, a later directory's stub replacing the one at the same relative path.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Larastan 3.14.0's `stubs` directory, for a Laravel app without Larastan in its vendor directory: Pun applies
/// Larastan's behaviour to every Laravel app, and without these stubs `$this` in an `Artisan::command()` closure is
/// an undefined variable.
static BUNDLED: include_dir::Dir =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/data/larastan-stubs");

/// The stub files Larastan adds, in no particular order: the `stubs` directory of Larastan installed in `root`'s
/// vendor directory, or of `root` itself when it is Larastan's checkout (StubFilesExtension's `dirname(__DIR__, 2)`).
/// Without one, the bundled stubs under the path an install would have; [`bundled`] has their contents.
pub(crate) fn files(root: &Path) -> Vec<PathBuf> {
    let stubs = package(root).join("stubs");
    let names: Vec<String> = match std::fs::read_dir(&stubs) {
        Ok(entries) => (entries.flatten())
            .filter(|e| e.path().is_dir())
            .filter_map(|e| e.file_name().into_string().ok())
            .collect(),
        Err(_) => (BUNDLED.dirs())
            .map(|d| d.path().to_string_lossy().into_owned())
            .collect(),
    };
    let laravel = laravel_version(root);
    let mut versions: Vec<String> = names
        .into_iter()
        .filter(|n| n.starts_with(|c: char| c.is_ascii_digit()))
        // without LARAVEL_VERSION (no app to boot), no versioned directory applies
        .filter(|n| laravel.as_deref().is_some_and(|l| compare(n, l).is_le()))
        .collect();
    versions.sort_by(|a, b| compare(a, b));
    let mut found = BTreeMap::new();
    for name in std::iter::once("common".to_string()).chain(versions) {
        let dir = stubs.join(&name);
        if stubs.is_dir() {
            collect(&dir, &dir, &mut found);
        } else if let Some(bundled) = BUNDLED.get_dir(&name) {
            collect_bundled(bundled, &name, &stubs, &mut found);
        }
    }
    found.into_values().collect()
}

/// The contents of a bundled stub, by the path [`files`] gave it.
pub(crate) fn bundled(path: &Path) -> Option<&'static [u8]> {
    let mut parts = path.components().rev();
    let mut relative = PathBuf::new();
    for part in parts.by_ref() {
        if part.as_os_str() == "stubs" {
            break;
        }
        relative = Path::new(part.as_os_str()).join(relative);
    }
    let below_larastan =
        parts.next()?.as_os_str() == "larastan" && parts.next()?.as_os_str() == "larastan";
    below_larastan
        .then(|| BUNDLED.get_file(&relative))
        .flatten()
        .map(include_dir::File::contents)
}

/// The bundled `*.stub` files below `dir`, by their path relative to the directory `name`.
fn collect_bundled(
    dir: &include_dir::Dir,
    name: &str,
    stubs: &Path,
    found: &mut BTreeMap<PathBuf, PathBuf>,
) {
    for file in dir
        .files()
        .filter(|f| f.path().extension().is_some_and(|x| x == "stub"))
    {
        let relative = file
            .path()
            .strip_prefix(name)
            .unwrap_or(file.path())
            .to_path_buf();
        found.insert(relative, stubs.join(file.path()));
    }
    for sub in dir.dirs() {
        collect_bundled(sub, name, stubs, found);
    }
}

/// Larastan's own directory (the extension's `dirname(__DIR__)`): its vendor install, or `root` when the project is
/// Larastan's checkout.
fn package(root: &Path) -> PathBuf {
    let installed = root.join("vendor/larastan/larastan");
    let own = std::fs::read_to_string(root.join("composer.json"))
        .ok()
        .and_then(|c| serde_json::from_str::<serde_json::Value>(&c).ok())
        .is_some_and(|c| c["name"] == "larastan/larastan");
    if own && !installed.is_dir() {
        root.to_path_buf()
    } else {
        installed
    }
}

/// The `*.stub` files below `dir`, by their path relative to `base`.
fn collect(base: &Path, dir: &Path, found: &mut BTreeMap<PathBuf, PathBuf>) {
    for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let p = e.path();
        if p.is_dir() {
            collect(base, &p, found);
        } else if p.extension().is_some_and(|x| x == "stub") {
            let relative = p.strip_prefix(base).unwrap_or(&p).to_path_buf();
            found.insert(relative, p);
        }
    }
}

/// LARAVEL_VERSION, which Larastan's bootstrap defines as the booted app's `version()`: the framework's
/// `Application::VERSION`.
pub(crate) fn laravel_version(root: &Path) -> Option<String> {
    let source = std::fs::read_to_string(
        root.join("vendor/laravel/framework/src/Illuminate/Foundation/Application.php"),
    )
    .ok()?;
    let rest = &source[source.find("const VERSION = ")? + "const VERSION = ".len()..];
    let quote = rest.chars().next()?;
    let rest = &rest[1..];
    Some(rest[..rest.find(quote)?].to_string())
}

/// `version_compare()` of two dotted numeric versions.
pub(crate) fn compare(a: &str, b: &str) -> std::cmp::Ordering {
    let parts = |v: &str| -> Vec<u64> { v.split('.').map(|p| p.parse().unwrap_or(0)).collect() };
    parts(a).cmp(&parts(b))
}
