//! The live Never-valued sibling of PHPUnit's dead-tail `Pun/never8` consumer.

use std::path::{Path, PathBuf};

use pun_lint::Linter;
use pun_lint::rules::{PreferMergedWillReturnCallback, facts::with_sources};

fn sources(path: &Path, out: &mut Vec<(String, String)>) {
    let mut paths: Vec<_> = std::fs::read_dir(path)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    paths.sort();
    for path in paths {
        if path.is_dir() {
            sources(&path, out);
        } else if path.extension().is_some_and(|extension| extension == "php") {
            out.push((
                path.to_string_lossy().into_owned(),
                std::fs::read_to_string(path).unwrap(),
            ));
        }
    }
}

#[test]
fn asserted_impossible_repeated_call_does_not_declare_a_callback_return() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut extra = Vec::new();
    sources(&root.join("tests/phpunit-stubs"), &mut extra);
    sources(&root.join("tests/rector-phpunit-stubs"), &mut extra);
    let input = include_str!(
        "../../pun-analyse/tests/fixtures/repeated-assertion-expressions/lint-input.php.inc"
    );
    let expected = include_str!(
        "../../pun-analyse/tests/fixtures/repeated-assertion-expressions/lint-expected.php.inc"
    );
    let linter = Linter::new(vec![Box::new(PreferMergedWillReturnCallback)], 80400);
    let fixed = with_sources(extra.clone(), || linter.fix(Path::new("input.php"), input)).unwrap();
    assert_eq!(fixed.source, expected);
    assert!(!fixed.applied.is_empty());
    let repeated =
        with_sources(extra, || linter.fix(Path::new("input.php"), &fixed.source)).unwrap();
    assert_eq!(repeated.source, expected);
    assert!(repeated.applied.is_empty());
}
