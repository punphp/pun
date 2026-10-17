//! `analyse()` on a project from its phpstan.neon: which files are analysed or only scanned (paths, excludePaths,
//! fileExtensions, scanFiles, scanDirectories, Composer autoload of the root package and of vendor/composer/
//! installed.json), phpVersion, ignoreErrors and the ignore flags. Expected: PHPStan's own run on the fixture
//! (`scripts/analyse-corpus.py fixture`), restricted to the identifiers Pun reports.

use std::path::Path;

use pun_analyse::{IDENTIFIERS, analyse};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

fn check(neon: &str) {
    check_in("project", neon);
}

fn check_in(fixture: &str, neon: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);
    let dir = root.to_string_lossy().into_owned();
    let options = Options {
        config: Some(format!("{dir}/{neon}.neon")),
        ..Options::for_project(&dir)
    };
    let config = resolve(&options).unwrap_or_else(|e| panic!("{e}"));
    let prefix = format!("{dir}/");
    let got: Vec<_> = analyse(&root, &config)
        .into_iter()
        .map(|e| {
            let file = e.file.strip_prefix(&prefix).unwrap_or(&e.file).to_string();
            (file, e.line, e.identifier, e.message, e.tip)
        })
        .collect();
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join(format!("expected-{neon}.json"))).unwrap())
            .unwrap();
    let mut want: Vec<_> = expected["errors"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| IDENTIFIERS.contains(&e["identifier"].as_str().unwrap_or("")))
        .map(|e| {
            (
                e["file"].as_str().unwrap().to_string(),
                e["line"].as_u64().unwrap_or(0) as u32,
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().replace("{root}", &dir),
                e["tip"].as_str().map(str::to_string),
            )
        })
        .collect();
    want.sort();
    assert_eq!(got, want);
}

#[test]
fn project_from_phpstan_neon() {
    check("phpstan");
}

#[test]
fn php_version_range_and_ignore_flags() {
    check("phpstan-min");
}

/// Classes of Composer's psr-4 directories, which Pun scans once an analysed or scanned file names them: through
/// another psr-4 file only, a namespace alias in another case, a group use, a name relative to an imported namespace or
/// to the file's own namespace, a class string with `\\`, the second of two directories, and a file excludePaths
/// names (PHPStan's PSR locators ignore it). A psr-4 directory that does not exist holds nothing.
#[test]
fn autoloaded_classes_found_on_demand() {
    check("phpstan-autoload");
}

/// A symlinked directory and a symlinked file under `paths` are walked and analysed, as PHPStan's finder follows
/// links (expected: its run on tests/fixtures/symlinks).
#[test]
fn symlinks_are_followed() {
    check_in("symlinks", "phpstan");
}
