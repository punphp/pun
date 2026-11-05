//! PHPStan 2.3.1's regex type tests: copies of tests/PHPStan/Analyser/nsrt's preg_match and preg_replace_callback
//! shape files in tests/fixtures/nsrt (MIT, see LICENSE there). PHPStan's FileAssertRule holds on every assertType
//! line there, so `phpstan.*` errors are wrong, and every other error must be PHPStan's own run on the directory
//! (expected-phpstan.json, from `scripts/export-analyse-expected.py crates/pun-analyse/tests/fixtures/nsrt`).
//!
//! Lines that still differ are listed in tests/fixtures/nsrt/gaps.txt as `<file>:<line>`, each with its reason, and
//! the list must be exact: a fixed line has to leave it.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use pun_analyse::{analyse, reports};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

type Found = BTreeSet<(String, String, Option<String>)>;

#[test]
fn phpstan_regex_type_tests() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/nsrt");
    let root = dir.to_string_lossy().into_owned();
    let config = resolve(&Options {
        config: Some(format!("{root}/phpstan.neon")),
        ..Options::for_project(&root)
    })
    .unwrap();
    let prefix = format!("{root}/");
    let mut got: BTreeMap<String, Found> = BTreeMap::new();
    for e in analyse(&dir, &config).unwrap() {
        let file = e.file.strip_prefix(&prefix).unwrap_or(&e.file);
        got.entry(format!("{file}:{}", e.line))
            .or_default()
            .insert((e.identifier, e.message, e.tip));
    }
    let expected: Value =
        serde_json::from_slice(&std::fs::read(dir.join("expected-phpstan.json")).unwrap()).unwrap();
    let mut want: BTreeMap<String, Found> = BTreeMap::new();
    for e in expected["errors"].as_array().unwrap() {
        let id = e["identifier"].as_str().unwrap();
        if reports(id) {
            want.entry(format!("{}:{}", e["file"].as_str().unwrap(), e["line"]))
                .or_default()
                .insert((
                    id.to_string(),
                    e["message"].as_str().unwrap().to_string(),
                    e["tip"].as_str().map(str::to_string),
                ));
        }
    }
    let gaps: BTreeSet<String> = std::fs::read_to_string(dir.join("gaps.txt"))
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(str::to_string)
        .collect();
    let lines: BTreeSet<&String> = got.keys().chain(want.keys()).collect();
    let empty = Found::new();
    let failing: BTreeMap<&String, String> = lines
        .into_iter()
        .filter_map(|line| {
            let (got, want) = (
                got.get(line).unwrap_or(&empty),
                want.get(line).unwrap_or(&empty),
            );
            (got != want).then(|| {
                let missing: Vec<_> = want.difference(got).collect();
                let extra: Vec<_> = got.difference(want).collect();
                (line, format!("  missing {missing:?}\n  extra   {extra:?}"))
            })
        })
        .collect();
    let fixed: Vec<_> = gaps.iter().filter(|g| !failing.contains_key(g)).collect();
    let new: Vec<_> = failing
        .iter()
        .filter(|(line, _)| !gaps.contains(**line))
        .collect();
    assert!(
        fixed.is_empty() && new.is_empty(),
        "fixed (remove from gaps.txt): {fixed:#?}\nnew failures:\n{}",
        new.iter()
            .map(|(line, diff)| format!("{line}\n{diff}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
