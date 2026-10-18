//! What the fixture tests of the `types` and `laravel` groups share: tests/rector/<Rector class>/{Fixture,Oracle,Pun}
//! run through the rules, compared before and after the laravel preset's format.

use std::path::{Path, PathBuf};

use pun_format::Pipeline;
use pun_lint::{Linter, Rule};

/// The fixtures of one directory, sorted: `(path, input, expected)`.
fn fixtures(dir: &Path) -> Vec<(PathBuf, String, String)> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
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
/// `rules` at PHP 8.4, Rector's default version in the fixtures' configs.
pub fn run_class(rules: Vec<Box<dyn Rule>>, class: &str) {
    run_class_at(rules, class, 80400);
}

/// As [`run_class`], at the PHP version `php_version`.
pub fn run_class_at(rules: Vec<Box<dyn Rule>>, class: &str, php_version: u32) {
    let linter = Linter::new(rules, php_version);
    let pipeline = laravel();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/rector")
        .join(class);
    assert!(
        root.join("Fixture").is_dir(),
        "{class}: no Fixture directory"
    );
    for sub in ["Fixture", "Oracle", "Pun"] {
        let dir = root.join(sub);
        if !dir.is_dir() {
            continue;
        }
        for (path, input, expected) in fixtures(&dir) {
            let name = format!("{sub}/{}", path.file_name().unwrap().to_string_lossy());
            let fixed = linter.fix(&path, &input).unwrap();
            assert_eq!(fixed.source, expected, "{class} {name}: lint --fix");
            assert_eq!(
                format(&pipeline, &fixed.source),
                format(&pipeline, &expected),
                "{class} {name}: format(lint --fix)"
            );
            if input != expected {
                // The rule did the work, not the formatter.
                assert_ne!(
                    format(&pipeline, &input),
                    format(&pipeline, &expected),
                    "{class} {name}"
                );
                assert!(!fixed.applied.is_empty(), "{class} {name}");
            } else {
                assert!(fixed.applied.is_empty(), "{class} {name}");
            }
        }
    }
}
