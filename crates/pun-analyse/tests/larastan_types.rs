//! Larastan 3.10.0's type tests, run through PHPStan 2.3.1 with Larastan by scripts/export-larastan-types.php:
//! tests/Type's assertion files (TypeInferenceTestCase) and tests/Integration's analysed files (IntegrationTest), each
//! test class's files at level 8 with its own config (FileAssertRule runs at every level). Pun analyses the same files in
//! tests/fixtures/larastan-types/root, a copy of the Larastan checkout with the vendor files Pun reads, and must report
//! PHPStan's errors:
//!
//! - every assertion line's `phpstan.*` errors (PHPStan's own assertions hold, so it has none);
//! - every Type file's other errors and every Integration file's errors, with an identifier Pun reports.
//!
//! Cases that still differ are listed in tests/fixtures/larastan-types/gaps.txt, one per line (`<file>:<line>` for an
//! assertion line, `<file>` for a file's other errors), and the list must be exact: a fixed case has to leave it.
//! `PUN_LARASTAN_TYPES_REPORT=1` prints every failing case's difference.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use pun_analyse::{Project, reports, run};
use pun_phpstan::config::{Options, resolve};
use pun_phpstan::{Key, Value as Parameter};
use serde_json::Value;

type Found = BTreeSet<(i64, String, String)>;

/// The identifiers of FileAssertRule.
fn is_assertion(identifier: &str) -> bool {
    matches!(
        identifier,
        "phpstan.type"
            | "phpstan.nativeType"
            | "phpstan.superType"
            | "phpstan.variable"
            | "phpstan.unknownExpectation"
    )
}

fn found<'e>(errors: impl Iterator<Item = (i64, &'e str, &'e str)>) -> Found {
    errors
        .filter(|(_, id, _)| reports(id))
        .map(|(line, id, message)| (line, id.to_string(), message.to_string()))
        .collect()
}

fn json_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in std::fs::read_dir(dir).unwrap().flatten() {
        let p = e.path();
        if p.is_dir() && p.file_name().unwrap() != "root" {
            json_files(&p, out);
        } else if p.extension().is_some_and(|x| x == "json") {
            out.push(p);
        }
    }
}

/// Pun on one test class's files, with its config at level 8: the errors by file. Files that declare the same name go
/// to separate runs (in one run, one declaration would be every file's).
fn analyse(root: &Path, test: &Value) -> BTreeMap<String, Found> {
    let configs = test["configs"].as_array().unwrap();
    assert_eq!(configs.len(), 1, "{}", test["test"]);
    let config = resolve(&Options {
        config: Some(root.join(configs[0].as_str().unwrap()).to_string_lossy().into_owned()),
        ..Options::for_project(&root.to_string_lossy())
    })
    .unwrap();
    let mut params = config.parameters.clone();
    params.set(Key::from("level"), Parameter::Int(8));
    let mut project = Project::from_config(root, &params);
    // first fit: each file in the first run that declares none of its names
    let mut runs: Vec<(Vec<PathBuf>, BTreeSet<&str>)> = Vec::new();
    for case in test["cases"].as_array().unwrap() {
        let file = root.join(case["file"].as_str().unwrap());
        let names: BTreeSet<&str> = case["declares"].as_array().unwrap().iter().map(|n| n.as_str().unwrap()).collect();
        match runs.iter_mut().find(|(_, declared)| declared.is_disjoint(&names)) {
            Some((files, declared)) => {
                files.push(file);
                declared.extend(names);
            }
            None => runs.push((vec![file], names)),
        }
    }
    let prefix = format!("{}/", root.display());
    let mut by_file: BTreeMap<String, Found> = BTreeMap::new();
    for (files, _) in runs {
        project.analysed = files;
        for e in run(&project).unwrap() {
            let file = e.file.strip_prefix(&prefix).unwrap_or(&e.file).to_string();
            by_file
                .entry(file)
                .or_default()
                .extend(found(std::iter::once((e.line, e.identifier.as_str(), e.message.as_str()))));
        }
    }
    by_file
}

#[test]
fn larastan_type_tests() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/larastan-types");
    // PUN_LARASTAN_TYPES_ROOT: the full Larastan checkout the fixture was cut from, which must give the same result
    let root = std::env::var_os("PUN_LARASTAN_TYPES_ROOT").map_or_else(|| dir.join("root"), PathBuf::from);
    let gaps: BTreeSet<String> = std::fs::read_to_string(dir.join("gaps.txt"))
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(str::to_string)
        .collect();
    let mut tests = Vec::new();
    json_files(&dir, &mut tests);
    tests.sort();
    let mut failing = BTreeMap::new();
    // (matched, total) assertions, Type files' other errors, Integration files
    let mut counts = [(0, 0); 3];
    for path in tests {
        let test: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        let got = analyse(&root, &test);
        for case in test["cases"].as_array().unwrap() {
            let file = case["file"].as_str().unwrap();
            let want = found(case["errors"].as_array().unwrap().iter().map(|e| {
                (
                    e["line"].as_i64().unwrap_or(0),
                    e["identifier"].as_str().unwrap_or(""),
                    e["message"].as_str().unwrap(),
                )
            }));
            let got = got.get(file).cloned().unwrap_or_default();
            let mut compare = |name: String, want: Found, got: Found, count: &mut (usize, usize), n: usize| {
                count.1 += n;
                if want == got {
                    count.0 += n;
                } else {
                    let missing: Vec<_> = want.difference(&got).collect();
                    let extra: Vec<_> = got.difference(&want).collect();
                    failing.insert(name, format!("  missing {missing:?}\n  extra   {extra:?}"));
                }
            };
            let Some(asserts) = case["asserts"].as_array() else {
                compare(file.to_string(), want, got, &mut counts[2], 1);
                continue;
            };
            let mut lines: BTreeMap<i64, usize> = BTreeMap::new();
            for line in asserts {
                *lines.entry(line.as_i64().unwrap()).or_default() += 1;
            }
            let at = |errors: &Found, line: i64| -> Found {
                errors.iter().filter(|(l, id, _)| *l == line && is_assertion(id)).cloned().collect()
            };
            for (&line, &n) in &lines {
                compare(format!("{file}:{line}"), at(&want, line), at(&got, line), &mut counts[0], n);
            }
            // assertion errors off the gathered lines count with the file's other errors
            let other = |errors: &Found| -> Found {
                errors.iter().filter(|(l, id, _)| !is_assertion(id) || !lines.contains_key(l)).cloned().collect()
            };
            compare(file.to_string(), other(&want), other(&got), &mut counts[1], 1);
        }
    }
    let [asserts, others, integration] = counts;
    eprintln!(
        "assertions {}/{}, Type files' other errors {}/{}, Integration files {}/{}",
        asserts.0, asserts.1, others.0, others.1, integration.0, integration.1
    );
    if std::env::var_os("PUN_LARASTAN_TYPES_REPORT").is_some() {
        for (name, diff) in &failing {
            eprintln!("{name}\n{diff}");
        }
    }
    assert!(asserts.1 >= 1390, "only {} assertions", asserts.1);
    let failing_names: BTreeSet<String> = failing.keys().cloned().collect();
    let fixed: Vec<_> = gaps.difference(&failing_names).collect();
    let new: Vec<_> = failing.iter().filter(|(name, _)| !gaps.contains(*name)).collect();
    assert!(
        fixed.is_empty() && new.is_empty(),
        "fixed (remove from gaps.txt): {fixed:#?}\nnew failures:\n{}",
        new.iter().map(|(name, diff)| format!("{name}\n{diff}")).collect::<Vec<_>>().join("\n")
    );
}
