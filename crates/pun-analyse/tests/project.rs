//! `analyse()` on a project from its phpstan.neon: which files are analysed or only scanned (paths, excludePaths,
//! fileExtensions, scanFiles, scanDirectories, Composer autoload of the root package and of vendor/composer/
//! installed.json), phpVersion, ignoreErrors and the ignore flags. Expected: PHPStan's own run on the fixture
//! (`scripts/analyse-corpus.py fixture`), restricted to the identifiers Pun reports.

use std::path::Path;

use pun_analyse::{IDENTIFIERS, Project, analyse, run};
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
        .unwrap()
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
                e["line"].as_i64().unwrap_or(0),
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

/// The class-member declaration rules' PHP-version branches (hooks, final and interface properties, asymmetric
/// visibility of static properties, promoted and final promoted properties), `PHP_VERSION_ID` narrowing in
/// FinalPrivateMethodRule, PHPDoc tags and `@immutable` parents (in another analysed file, in a scanned one, missing),
/// virtual properties overriding a parent's, and enum case values the codex does not type.
#[test]
fn declaration_rules_by_php_version() {
    check("phpstan-versions-70400");
    check("phpstan-versions-80000");
    check("phpstan-versions-80400");
}

/// The rules bleedingEdge turns on through `featureToggles` (NewStaticInAbstractClassStaticMethodRule's own test).
#[test]
fn feature_toggles() {
    check("phpstan-bleeding");
}

/// A file that exists but can't be read stops the run with its path, as it stops PHPStan's ("Could not read file:
/// <path>" for a chmod 000 file); a directory stands in for one, since root reads any file. A file that is gone is
/// skipped.
#[test]
fn unreadable_file_stops_the_run() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/project");
    let e = run(&Project::new(vec![dir.clone()], Vec::new())).unwrap_err();
    assert_eq!(
        e.to_string(),
        format!("Could not read file: {}", dir.display())
    );
    assert!(std::error::Error::source(&e).is_some());
    let gone = dir.join("gone.php");
    assert_eq!(run(&Project::new(vec![gone], Vec::new())).unwrap(), []);
}

/// Without `level` (config.neon's null) the rules run at level 0, where PHPStan stops with "No rules detected".
#[test]
fn level_is_zero_when_unset() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/project");
    let options = Options {
        config: Some(
            dir.join("phpstan-nolevel.neon")
                .to_string_lossy()
                .into_owned(),
        ),
        ..Options::for_project(&dir.to_string_lossy())
    };
    let config = resolve(&options).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(Project::from_config(&dir, &config.parameters).level, 0);
}
