//! Rector's own fixtures for the rules of lint-b's groups (style, classes, functions, phpdoc, security and the PHP
//! upgrade groups), laid out as tests/rector.rs describes: tests/rector/<Rector class>/Fixture copied verbatim from
//! rector-src 2.6.1 (`.oracle/rector`, rules-tests/...), Oracle/ with what Rector 2.6.1 (`.oracle/rector-bin`, the
//! rule alone, `withPhpVersion(80400)` unless the test says otherwise) wrote, and Pun/ for the cases where Pun keeps
//! the meaning but not Rector's bytes (expected derived by hand). Each must pass as
//! `format(pun lint --fix) == format(expected)` with the laravel preset; the raw bytes are checked too.

use std::path::{Path, PathBuf};

use pun_format::Pipeline;
use pun_lint::{Linter, Rule, rules};

/// The fixtures of `tests/rector/<rector>/{Fixture,Oracle,Pun}`, or of the one directory `rector` names.
fn fixtures(rector: &str) -> Vec<(PathBuf, String, String)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/rector")
        .join(rector);
    let dirs: Vec<PathBuf> = match rector.contains('/') {
        true => vec![root.clone()],
        false => ["Fixture", "Oracle", "Pun"]
            .iter()
            .map(|d| root.join(d))
            .filter(|d| d.is_dir())
            .collect(),
    };
    let mut paths: Vec<PathBuf> = dirs
        .iter()
        .flat_map(|d| std::fs::read_dir(d).unwrap())
        .map(|e| e.unwrap().path())
        .collect();
    paths.sort();
    assert!(!paths.is_empty(), "no fixtures in {}", root.display());
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
    Pipeline::new(&rules, &pun_fixers::all).unwrap()
}

/// Every fixture of `dir` with `rule` alone at `php_version`: the raw fix equals the expected code, so do the
/// formatted ones, and the rule applied a fix exactly when the fixture expects a change.
fn run(rule: impl Rule + 'static, php_version: u32, dir: &str) {
    let linter = Linter::new(vec![Box::new(rule)], php_version);
    let pipeline = laravel();
    for (path, input, mut expected) in fixtures(dir) {
        let name = path
            .strip_prefix(env!("CARGO_MANIFEST_DIR"))
            .unwrap()
            .display()
            .to_string();
        // Rector's test runner trims both halves; the expected half often lacks the input's final newline.
        if input.ends_with('\n') && !expected.ends_with('\n') {
            expected.push('\n');
        }
        let fixed = linter.fix(&path, &input).unwrap();
        assert_eq!(fixed.source, expected, "{name}: lint --fix");
        assert_eq!(
            format(&pipeline, &fixed.source),
            format(&pipeline, &expected),
            "{name}: format(lint --fix)"
        );
        assert_eq!(fixed.applied.is_empty(), input == expected, "{name}");
    }
}

#[test]
fn combined_assign_oracle() {
    run(
        rules::PreferCompoundAssignment,
        80400,
        "CombinedAssignRector/Oracle",
    );
}

#[test]
fn privatize_local_getter_to_property() {
    run(
        rules::PreferPropertyOverLocalGetter,
        80400,
        "PrivatizeLocalGetterToPropertyRector/Fixture",
    );
    run(
        rules::PreferPropertyOverLocalGetter,
        80400,
        "PrivatizeLocalGetterToPropertyRector/Oracle",
    );
}

#[test]
fn add_escape_argument() {
    run(
        rules::AddCsvEscapeArgument,
        80400,
        "AddEscapeArgumentRector",
    );
}

#[test]
fn rounding_mode_enum() {
    run(rules::PreferRoundModeEnum, 80400, "RoundingModeEnumRector");
}

#[test]
fn remove_finfo_buffer_context_arg() {
    run(
        rules::NoFinfoBufferContext,
        80500,
        "RemoveFinfoBufferContextArgRector",
    );
}

#[test]
fn null_debug_info_return() {
    // Rector's tests run with PhpVersionProvider::provide() at 100000 (PHP 10.0).
    run(rules::NoNullDebugInfo, 100000, "NullDebugInfoReturnRector");
}

#[test]
fn chr_arg_modulo() {
    run(rules::NoOutOfRangeChr, 80500, "ChrArgModuloRector");
}

#[test]
fn ord_single_byte() {
    run(rules::PreferOrdFirstByte, 80500, "OrdSingleByteRector");
}

#[test]
fn array_first_last() {
    run(rules::PreferArrayFirstLast, 80500, "ArrayFirstLastRector");
}

#[test]
fn min_max_to_clamp() {
    run(rules::PreferClamp, 80600, "MinMaxToClampRector");
}

#[test]
fn foreach_to_array_all() {
    run(rules::PreferArrayAll, 80400, "ForeachToArrayAllRector");
}

#[test]
fn foreach_to_array_any() {
    run(rules::PreferArrayAny, 80400, "ForeachToArrayAnyRector");
}

#[test]
fn foreach_to_array_find() {
    run(rules::PreferArrayFind, 80400, "ForeachToArrayFindRector");
}

#[test]
fn foreach_to_array_find_key() {
    run(
        rules::PreferArrayFindKey,
        80400,
        "ForeachToArrayFindKeyRector",
    );
}
