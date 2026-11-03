//! LarastanStubFilesExtension: `stubs/common` and every `stubs/<version>` directory whose version is at most the
//! installed Laravel's, a later directory's stub replacing the one at the same relative path.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The stub files Larastan installed in `root`'s vendor directory adds, in no particular order.
pub(crate) fn files(root: &Path) -> Vec<PathBuf> {
    let stubs = root.join("vendor/larastan/larastan/stubs");
    let laravel = laravel_version(root);
    let mut versions: Vec<String> = std::fs::read_dir(&stubs)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|n| n.starts_with(|c: char| c.is_ascii_digit()))
        // without LARAVEL_VERSION (no app to boot), no versioned directory applies
        .filter(|n| laravel.as_deref().is_some_and(|l| compare(n, l).is_le()))
        .collect();
    versions.sort_by(|a, b| compare(a, b));
    let mut found = BTreeMap::new();
    for dir in std::iter::once("common".to_string()).chain(versions) {
        let dir = stubs.join(dir);
        collect(&dir, &dir, &mut found);
    }
    found.into_values().collect()
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
fn laravel_version(root: &Path) -> Option<String> {
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
fn compare(a: &str, b: &str) -> std::cmp::Ordering {
    let parts = |v: &str| -> Vec<u64> { v.split('.').map(|p| p.parse().unwrap_or(0)).collect() };
    parts(a).cmp(&parts(b))
}
