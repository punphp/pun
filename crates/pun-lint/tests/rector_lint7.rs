//! Upstream fixtures for the lint 7 small rules, laid out as in tests/rector.rs: Fixture/ verbatim from rector-src
//! 2.6.1, Oracle/ with what Rector 2.6.1 (`.oracle/rector-bin`, the rule alone) wrote. Each must pass as
//! `pun lint --fix == expected` and `format(pun lint --fix) == format(expected)`.

use std::path::{Path, PathBuf};

use pun_format::Pipeline;
use pun_lint::rules::*;
use pun_lint::{Linter, Rule};

fn fixtures(dir: &str) -> Vec<(PathBuf, String, String)> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/rector")
        .join(dir);
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
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

fn run(linter: &Linter, dir: &str) {
    let pipeline = laravel();
    for (path, input, expected) in fixtures(dir) {
        let name = path.file_name().unwrap().to_string_lossy();
        let fixed = linter.fix(&path, &input).unwrap();
        assert_eq!(fixed.source, expected, "{name}: lint --fix");
        assert_eq!(
            format(&pipeline, &fixed.source),
            format(&pipeline, &expected),
            "{name}: format(lint --fix)"
        );
        assert_eq!(input != expected, !fixed.applied.is_empty(), "{name}");
    }
}

fn only(rule: impl Rule + 'static) -> Linter {
    Linter::new(vec![Box::new(rule)], 80400)
}

#[test]
fn boolean_not_identical_to_not_identical() {
    let linter = only(NoNegatedComparison);
    run(&linter, "BooleanNotIdenticalToNotIdenticalRector/Fixture");
    run(&linter, "BooleanNotIdenticalToNotIdenticalRector/Oracle");
}

#[test]
fn ternary_empty_array_array_dim_fetch_to_coalesce() {
    let linter = only(PreferCoalesceOverEmptyTernary);
    run(&linter, "TernaryEmptyArrayArrayDimFetchToCoalesceRector/Fixture");
    run(&linter, "TernaryEmptyArrayArrayDimFetchToCoalesceRector/Oracle");
}
