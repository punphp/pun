//! What the fixture tests of the `types` and `laravel` groups share: tests/rector/<Rector class>/{Fixture,Oracle,Pun}
//! run through the rules, compared before and after the laravel preset's format.

use std::path::{Path, PathBuf};

use pun_format::Pipeline;
use pun_lint::rules::facts::with_sources;
use pun_lint::{Linter, Rule};

/// The files under `dir`, its subdirectories too (Rector's `yieldFilesFromDirectory`).
fn files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            files(&path, out);
        } else {
            out.push(path);
        }
    }
}

/// The fixtures of one directory, sorted: `(path, input, expected)`.
fn fixtures(dir: &Path) -> Vec<(PathBuf, String, String)> {
    let mut paths = Vec::new();
    files(dir, &mut paths);
    paths.sort();
    assert!(!paths.is_empty(), "no fixtures in {}", dir.display());
    paths
        .into_iter()
        .map(|path| {
            let content = std::fs::read_to_string(&path)
                .unwrap()
                .replace("\r\n", "\n");
            let (input, expected) = match content.split_once("-----\n") {
                Some((i, e)) => (i.to_string(), e.to_string()),
                None => (content.clone(), content),
            };
            (path, input, expected)
        })
        .collect()
}

/// The PHP files under `dir` (a fixture's `Source/`), recursively: `(path, code)`.
fn sources(dir: &Path, out: &mut Vec<(String, String)>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries.map(|e| e.unwrap().path()).collect();
    paths.sort();
    for path in paths {
        if path.is_dir() {
            sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "php") {
            out.push((
                path.to_string_lossy().into_owned(),
                std::fs::read_to_string(&path).unwrap(),
            ));
        }
    }
}

fn format(pipeline: &Pipeline, code: &str) -> String {
    match pipeline
        .run(Path::new("fixture.php"), code.as_bytes())
        .unwrap()
    {
        Some(fixed) => String::from_utf8(fixed.code).unwrap(),
        None => code.to_string(),
    }
}

fn laravel() -> Pipeline {
    let rules = pun_config::rule_set(pun_config::preset_rules("laravel").unwrap()).unwrap();
    let pipeline = Pipeline::new(&rules, &pun_fixers::all).unwrap();
    assert!(pipeline.not_ported.is_empty(), "{:?}", pipeline.not_ported);
    pipeline
}

/// Run every fixture of tests/rector/<class>/{Fixture,Oracle,Pun} (the directories that exist; Fixture must) through
/// `rules` at PHP 8.4, Rector's default version in the fixtures' configs, with the classes of <class>/Source known.
#[allow(
    dead_code,
    reason = "each test target uses a different part of this shared module"
)]
pub(crate) fn run_class(rules: Vec<Box<dyn Rule>>, class: &str) {
    run_class_at(rules, class, 80400);
}

/// As [`run_class`], at the PHP version `php_version`.
pub(crate) fn run_class_at(rules: Vec<Box<dyn Rule>>, class: &str, php_version: u32) {
    run_class_full(rules, class, php_version, &[], &[], true);
}

/// As [`run_class_at`]; the fixtures named in `preset_too` (`Fixture/in_for.php.inc`) make a change the laravel
/// preset's format makes as well, so only the rule's own fix shows that the rule did it.
#[allow(
    dead_code,
    reason = "each test target uses a different part of this shared module"
)]
pub(crate) fn run_class_with(
    rules: Vec<Box<dyn Rule>>,
    class: &str,
    php_version: u32,
    preset_too: &[&str],
) {
    run_class_full(rules, class, php_version, &[], preset_too, true);
}

/// As [`run_class_at`], with the classes of the directories `stubs` (under tests/) known too.
#[allow(
    dead_code,
    reason = "each test target uses a different part of this shared module"
)]
pub(crate) fn run_class_stubs(
    rules: Vec<Box<dyn Rule>>,
    class: &str,
    php_version: u32,
    stubs: &[&str],
) {
    run_class_full(rules, class, php_version, stubs, &[], true);
}

/// As [`run_class_at`], with the classes under `stubs` (a package's stub directories) known beside <class>/Source.
#[allow(
    dead_code,
    reason = "each test target uses a different part of this shared module"
)]
pub(crate) fn run_class_paths(
    rules: Vec<Box<dyn Rule>>,
    class: &str,
    php_version: u32,
    stubs: &[&Path],
) {
    check_dirs(class, true);
    run_dirs(
        rules,
        class,
        &["Fixture", "Oracle", "Pun"],
        php_version,
        stubs,
    );
}

/// As [`run_class_at`] for a Rector class without upstream fixtures: tests/rector/<class>/{Oracle,Pun} only.
#[allow(
    dead_code,
    reason = "each test target uses a different part of this shared module"
)]
pub(crate) fn run_oracle_class(rules: Vec<Box<dyn Rule>>, class: &str, php_version: u32) {
    run_class_full(rules, class, php_version, &[], &[], false);
}

fn run_class_full(
    rules: Vec<Box<dyn Rule>>,
    class: &str,
    php_version: u32,
    stubs: &[&str],
    preset_too: &[&str],
    fixture: bool,
) {
    check_dirs(class, fixture);
    let tests = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let stubs: Vec<PathBuf> = stubs.iter().map(|d| tests.join(d)).collect();
    let stubs: Vec<&Path> = stubs.iter().map(PathBuf::as_path).collect();
    run_dirs_full(
        rules,
        class,
        &["Fixture", "Oracle", "Pun"],
        php_version,
        &stubs,
        preset_too,
    );
}

/// tests/rector/<class> has a Fixture directory exactly when upstream has fixtures, and an Oracle one otherwise.
fn check_dirs(class: &str, fixture: bool) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/rector")
        .join(class);
    // A class with upstream fixtures may also be run on fresh Rector cases alone (its Fixture directory left out
    // where Pun reads the fixtures' stubs differently); one without never has a Fixture directory.
    assert!(
        root.join("Fixture").is_dir() == fixture || (fixture && root.join("Oracle").is_dir()),
        "{class}: a Fixture directory where upstream has fixtures, and only there"
    );
    assert!(
        root.join("Oracle").is_dir() || fixture,
        "{class}: no Oracle directory"
    );
}

/// The cases of the directories `subs` of tests/rector/<class> that exist (a configurable rule runs each
/// configuration on its own directories), with the classes under `stubs` known beside <class>/Source.
#[allow(
    dead_code,
    reason = "each test target uses a different part of this shared module"
)]
pub(crate) fn run_dirs(
    rules: Vec<Box<dyn Rule>>,
    class: &str,
    subs: &[&str],
    php_version: u32,
    stubs: &[&Path],
) {
    run_dirs_full(rules, class, subs, php_version, stubs, &[]);
}

fn run_dirs_full(
    rules: Vec<Box<dyn Rule>>,
    class: &str,
    subs: &[&str],
    php_version: u32,
    stubs: &[&Path],
    preset_too: &[&str],
) {
    let linter = Linter::new(rules, php_version);
    let pipeline = laravel();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/rector")
        .join(class);
    for &sub in subs {
        let dir = root.join(sub);
        if !dir.is_dir() {
            continue;
        }
        for (path, input, expected) in fixtures(&dir) {
            let name = format!("{sub}/{}", path.file_name().unwrap().to_string_lossy());
            let mut extra = Vec::new();
            for dir in stubs {
                sources(dir, &mut extra);
            }
            sources(&root.join("Source"), &mut extra);
            // Classes Rector's own test run autoloads from its dev dependencies (vendor/), as minimal stubs.
            sources(&root.join("Stubs"), &mut extra);
            let fixed = with_sources(extra, || linter.fix(&path, &input)).unwrap();
            // Rector's test falls back to `assertStringMatchesFormat`, whose `$` also matches before a final newline,
            // so an expected half without the file's last newline still passes.
            let expected = match fixed.source.strip_suffix('\n') {
                Some(without) if without == expected => fixed.source.clone(),
                _ => expected,
            };
            assert_eq!(fixed.source, expected, "{class} {name}: lint --fix");
            assert_eq!(
                format(&pipeline, &fixed.source),
                format(&pipeline, &expected),
                "{class} {name}: format(lint --fix)"
            );
            if input != expected {
                // The rule did the work, not the formatter.
                assert!(
                    preset_too.contains(&name.as_str())
                        || FORMAT_OVERLAP.contains(&format!("{class}/{name}").as_str())
                        || format(&pipeline, &input) != format(&pipeline, &expected),
                    "{class} {name}"
                );
                assert!(!fixed.applied.is_empty(), "{class} {name}");
            } else {
                assert!(fixed.applied.is_empty(), "{class} {name}");
            }
        }
    }
}

/// Fixtures whose expected half the laravel format also reaches on its own; the rule's text edits must still give
/// the expected bytes and apply. FixClassCaseSensitivityVarDocblockRector/Fixture/{inline_var,property_var}: Pint's
/// `fully_qualified_strict_types` fixes the case of an imported class in a `@var` too.
/// RenameClassRector/FixtureAutoImportNames/auto_import_conflict_name_all_fqcn_same_namespace: it imports the global
/// `\SomeShort` of a `@return` too. RenameClassRector/Oracle/namespaces_{f2,f3,f5}: Pint's `no_unused_imports` removes
/// the unused import too. RenameClassRector/OracleAutoImportNames/doc_only_comment: Pint's
/// `fully_qualified_strict_types` imports the class of the lone `@var` too. RenameClassRector/OracleAutoImportNames/
/// doc_on_{only_use,use,used_use}: Pint's `fully_qualified_strict_types` imports the `@var` class too (and `no_unused_imports`
/// removes the unused import). RenameClassRector/OracleAutoImportNames/doc_on_second_namespace: Pint's
/// `no_unused_imports` removes the unused import too.
const FORMAT_OVERLAP: &[&str] = &[
    "RenameClassRector/FixtureAutoImportNames/auto_import_conflict_name_all_fqcn_same_namespace.php.inc",
    "RenameClassRector/OracleAutoImportNames/doc_only_comment.php.inc",
    "RenameClassRector/OracleAutoImportNames/doc_on_only_use.php.inc",
    "RenameClassRector/OracleAutoImportNames/doc_on_use.php.inc",
    "RenameClassRector/OracleAutoImportNames/doc_on_used_use.php.inc",
    "RenameClassRector/OracleAutoImportNames/doc_on_second_namespace.php.inc",
    "RenameClassRector/Oracle/namespaces_f2.php.inc",
    "RenameClassRector/Oracle/namespaces_f3.php.inc",
    "RenameClassRector/Oracle/namespaces_f5.php.inc",
    "FixClassCaseSensitivityVarDocblockRector/Fixture/inline_var.php.inc",
    "FixClassCaseSensitivityVarDocblockRector/Fixture/property_var.php.inc",
];
