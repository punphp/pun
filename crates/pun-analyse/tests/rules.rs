//! PHPStan's own rule tests, run through `phpstan analyse --level 0` by scripts/export-rule-fixtures.php: every
//! case's errors with an identifier Pun reports must be exactly PHPStan's. Cases that still differ are listed in
//! tests/fixtures/rules/gaps.txt, and the list must be exact (a fixed case has to leave it).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use pun_analyse::{IDENTIFIERS, Project, run};
use serde_json::Value;

type Key = (String, i64, String, String, Option<String>);

fn fixtures(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in std::fs::read_dir(dir).unwrap().flatten() {
        let p = e.path();
        if p.is_dir() && p.file_name().unwrap() != "data" {
            fixtures(&p, out);
        } else if p.extension().is_some_and(|x| x == "json") {
            out.push(p);
        }
    }
}

#[test]
fn phpstan_rule_fixtures() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rules");
    let data = root.join("data");
    let mut files = Vec::new();
    fixtures(&root, &mut files);
    files.sort();
    // PUN_RULES_FILTER=Functions/ runs only the fixtures (and checks only the gaps) whose name contains it
    let filter = std::env::var("PUN_RULES_FILTER").unwrap_or_default();
    files.retain(|f| f.to_string_lossy().contains(&filter));
    let gaps: BTreeSet<String> = std::fs::read_to_string(root.join("gaps.txt"))
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#') && l.contains(&filter))
        .map(str::to_string)
        .collect();
    let prefix = format!("{}/", data.display());
    let mut failing = BTreeSet::new();
    let mut details = Vec::new();
    // identifier => (cases where it occurs, cases where it matches)
    let mut per_rule: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    for f in files {
        let test: Value = serde_json::from_str(&std::fs::read_to_string(&f).unwrap()).unwrap();
        for case in test["cases"].as_array().unwrap() {
            let paths = |k: &str| -> Vec<PathBuf> {
                case[k]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|p| data.join(p.as_str().unwrap()))
                    .collect()
            };
            let want: BTreeSet<Key> = case["errors"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|e| {
                    e["identifier"]
                        .as_str()
                        .is_some_and(|i| IDENTIFIERS.contains(&i))
                })
                .map(|e| {
                    (
                        e["file"].as_str().unwrap().to_string(),
                        e["line"].as_i64().unwrap(),
                        e["identifier"].as_str().unwrap().to_string(),
                        e["message"].as_str().unwrap().to_string(),
                        e["tip"].as_str().map(str::to_string),
                    )
                })
                .collect();
            let got: BTreeSet<Key> = run(&Project::new(paths("files"), paths("scan")))
                .unwrap()
                .into_iter()
                .map(|e| {
                    let file = e.file.strip_prefix(&prefix).unwrap_or(&e.file).to_string();
                    (file, e.line, e.identifier, e.message, e.tip)
                })
                .collect();
            let name = format!(
                "{}::{}",
                test["test"].as_str().unwrap(),
                case["method"].as_str().unwrap()
            );
            let ids: BTreeSet<&String> = want.iter().chain(&got).map(|k| &k.2).collect();
            for id in ids {
                let of = |s: &BTreeSet<Key>| {
                    s.iter().filter(|k| &k.2 == id).cloned().collect::<Vec<_>>()
                };
                let entry = per_rule.entry(id.clone()).or_default();
                entry.0 += 1;
                entry.1 += usize::from(of(&want) == of(&got));
            }
            if want != got {
                details.push(format!(
                    "{name}\n  missing {:?}\n  extra   {:?}",
                    want.difference(&got).collect::<Vec<_>>(),
                    got.difference(&want).collect::<Vec<_>>()
                ));
                failing.insert(name);
            }
        }
    }
    for (id, (cases, ok)) in &per_rule {
        eprintln!("{id:40} {ok:4}/{cases:<4} cases match");
    }
    let fixed: Vec<_> = gaps.difference(&failing).collect();
    let new: Vec<_> = details
        .iter()
        .filter(|d| !gaps.contains(d.lines().next().unwrap()))
        .collect();
    assert!(
        fixed.is_empty() && new.is_empty(),
        "gaps.txt lists {} passing case(s): {fixed:?}\n{} new failure(s):\n{}",
        fixed.len(),
        new.len(),
        new.iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
