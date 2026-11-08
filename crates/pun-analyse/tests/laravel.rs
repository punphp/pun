//! Pun's Laravel plugin for Mago's analyser against Larastan: the type of each model attribute that
//! tests/fixtures/laravel/app/Probe.php passes to `\PHPStan\dumpType()`. The fixture is Pinkary's Link model and the
//! Pinkary migrations its table and its foreign keys come from, plus models and a migration that reach every branch
//! of Larastan's SchemaAggregator and ModelPropertyHelper.
//!
//! Expected: tests/fixtures/laravel/expected.json, PHPStan 2.3.5 + Larastan 3.14.0's own dumps on a copy of Pinkary
//! (`scripts/export-laravel-fixture.py`), for `parseModelCastsMethod` false (Larastan's default) and true. Mago's type
//! is read back with pun-phpstan's type-string resolver and printed with PHPStan's `describe()`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use pun_analyse::{Laravel, Project, run_mago};
use pun_phpstan::resolve::resolve_str;
use pun_phpstan::types::Level;
use serde_json::Value;

fn php_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in std::fs::read_dir(dir).unwrap().flatten() {
        let p = e.path();
        if p.is_dir() {
            php_files(&p, out);
        } else if p.extension().is_some_and(|x| x == "php") {
            out.push(p);
        }
    }
}

/// Mago's type notation as a PHPDoc type: its literal strings `string('a')` are `'a'`, and Mago has no benevolent
/// unions, so the `array<array-key, mixed>` it gives the `array` and `json` casts is Larastan's
/// `array<(int|string), mixed>` with a benevolent key, which PHPStan prints as `array`.
fn phpdoc(mago: &str) -> String {
    if mago == "array<array-key, mixed>" {
        return "array".into();
    }
    mago.split('|')
        .map(
            |t| match t.strip_prefix("string(").and_then(|t| t.strip_suffix(')')) {
                Some(literal) => literal,
                None => t,
            },
        )
        .collect::<Vec<_>>()
        .join("|")
}

/// Probe.php's line => the type Pun gives, in PHPStan's notation.
fn dumped(parse_casts_method: bool) -> BTreeMap<u64, String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/laravel");
    let mut analysed = Vec::new();
    php_files(&root.join("app"), &mut analysed);
    analysed.sort();
    let project = Project::new(analysed, vec![root.join("stubs/laravel.php")]);
    let run = run_mago(
        &project,
        Some(Laravel {
            root: &root,
            parse_casts_method,
        }),
    )
    .unwrap();
    run.dumped
        .into_iter()
        .map(|d| {
            (
                u64::from(d.line),
                resolve_str(&phpdoc(&d.type_id), None).describe(Level::Precise),
            )
        })
        .collect()
}

/// Larastan's dumps.
fn expected(key: &str) -> BTreeMap<u64, String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/laravel");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    expected[key]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| (e[0].as_u64().unwrap(), e[1].as_str().unwrap().to_string()))
        .collect()
}

/// Probe lines whose type still differs from Larastan's, each with its reason. The comparison fails when one matches.
/// 78: Larastan 3.14 dumps the cover migration's `remember_token` as *ERROR* where 3.10 and Pun give `string`; its
/// SchemaAggregator rewrite (literal SQL statements, resolveName(), arrow-function blueprints, a nullable
/// rememberToken()) is not ported, and which part gives the error type is not identified.
const GAPS: &[u64] = &[78];

fn compare(mut got: BTreeMap<u64, String>, mut want: BTreeMap<u64, String>) {
    for line in GAPS {
        assert_ne!(got.remove(line), want.remove(line), "line {line} matches Larastan now: remove it from GAPS");
    }
    assert_eq!(got, want);
}

#[test]
fn model_attributes_from_migrations() {
    compare(dumped(false), expected("parseModelCastsMethod=false"));
}

#[test]
fn casts_method_when_parse_model_casts_method() {
    compare(dumped(true), expected("parseModelCastsMethod=true"));
}
