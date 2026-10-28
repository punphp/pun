//! `lint/phpunit/prefer-yield-data-provider` over a project's codebase, which is scanned once, before the first fix
//! round: the second round finds the class no longer where the codebase has it and leaves it as the first round left
//! it. Without that, the stale codebase misses the call to `p11()` in the moved `testUsesP11()` and the second round
//! changes `p11()` too. The expected code is Rector's (tests/rector/YieldDataProviderRector/Oracle/
//! used_after_other.php.inc). Its own binary: the project codebase is process-wide.

use std::path::{Path, PathBuf};

use pun_lint::Linter;
use pun_lint::rules::PreferYieldDataProvider;
use pun_lint::rules::facts::{project_codebase, set_project_codebase};

/// The PHP files under `dir`, recursively: `(path, code)`.
fn sources(dir: &Path, out: &mut Vec<(String, String)>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path: PathBuf = entry.unwrap().path();
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

#[test]
fn a_later_round_leaves_the_changed_class_alone() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let fixture = std::fs::read_to_string(
        root.join("rector/YieldDataProviderRector/Oracle/used_after_other.php.inc"),
    )
    .unwrap();
    let (input, expected) = fixture.split_once("-----\n").unwrap();
    let mut files = Vec::new();
    sources(&root.join("phpunit-stubs"), &mut files);
    files.push(("/project/tests/BodyTest.php".into(), input.into()));
    set_project_codebase(Some(project_codebase(&files, 80400)));
    let linter = Linter::new(vec![Box::new(PreferYieldDataProvider)], 80400);
    let fixed = linter
        .fix(Path::new("/project/tests/BodyTest.php"), input)
        .unwrap();
    set_project_codebase(None);
    assert_eq!(fixed.source, expected);
    // the first round changed the class, the second found nothing to change
    assert_eq!(fixed.rounds, 1);
}
