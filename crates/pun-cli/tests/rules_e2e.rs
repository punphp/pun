//! Every rule end to end through the real binary: tests/e2e/rules.json (scripts/export-e2e-cases.py) holds, per
//! rule, a one-file project enabling only that rule (pint.json, or pun.toml for Pun/ rules) and what real Pint does
//! to it: the file it writes and `pint --test`'s stdout and exit code. Pun/ rules have no Pint; their expectation
//! is the first Mago-derived case of the fixer's own tests. `pun format` must write the same file and
//! `pun format --test` must print the same and exit the same.

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;

fn project(case: &Value, i: usize) -> PathBuf {
    let d = std::env::temp_dir().join(format!("pun-rules-e2e-{}-{i}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    if let Some(cfg) = case.get("pint.json") {
        std::fs::write(d.join("pint.json"), cfg.to_string()).unwrap();
    }
    if let Some(toml) = case.get("pun.toml") {
        std::fs::write(d.join("pun.toml"), toml.as_str().unwrap()).unwrap();
    }
    std::fs::write(
        d.join(case["file"].as_str().unwrap()),
        case["input"].as_str().unwrap(),
    )
    .unwrap();
    d
}

/// The binary with the environment Pint ran with when the expectations were recorded (`env -i PATH HOME`).
fn pun(dir: &Path, args: &[&str]) -> (i32, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_pun"))
        .args(args)
        .current_dir(dir)
        .env_clear()
        .env("PATH", std::env::var("PATH").unwrap_or_default())
        .env("HOME", std::env::var("HOME").unwrap_or_default())
        .output()
        .unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
    )
}

#[test]
fn every_rule_matches_pint() {
    let cases: Vec<Value> = serde_json::from_str(include_str!("e2e/rules.json")).unwrap();
    let node_modules = Path::new(env!("CARGO_MANIFEST_DIR")).join("../pun-fixers/node_modules");
    let mut failures = Vec::new();
    for (i, case) in cases.iter().enumerate() {
        let rule = case["rule"].as_str().unwrap();
        let dir = project(case, i);
        if rule == "Pint/laravel_blade" {
            if !node_modules.join("prettier").exists() {
                eprintln!(
                    "skipping {rule}: no prettier in crates/pun-fixers/node_modules (npm install there)"
                );
                continue;
            }
            #[cfg(unix)]
            std::os::unix::fs::symlink(
                node_modules.canonicalize().unwrap(),
                dir.join("node_modules"),
            )
            .unwrap();
        }
        let (code, stdout) = pun(&dir, &["format", "--test"]);
        if code != case["test_exit"].as_i64().unwrap() as i32 {
            failures.push(format!(
                "{rule}: `--test` exited {code}, Pint {}\n{stdout}",
                case["test_exit"]
            ));
        }
        if let Some(want) = case.get("test_stdout").and_then(Value::as_str) {
            if stdout != want {
                failures.push(format!(
                    "{rule}: `--test` output\n--- pint\n{want}\n--- pun\n{stdout}"
                ));
            }
        } else if !stdout.contains(rule) {
            failures.push(format!(
                "{rule}: `--test` output does not name the rule\n{stdout}"
            ));
        }
        let file = dir.join(case["file"].as_str().unwrap());
        assert_eq!(
            std::fs::read_to_string(&file).unwrap(),
            case["input"].as_str().unwrap(),
            "{rule}: --test wrote the file"
        );
        pun(&dir, &["format"]);
        let got = std::fs::read_to_string(&file).unwrap();
        if got != case["expected"].as_str().unwrap() {
            failures.push(format!(
                "{rule}: written file\n--- expected\n{}\n--- pun\n{got}",
                case["expected"].as_str().unwrap()
            ));
        }
        let _ = std::fs::remove_dir_all(&dir);
    }
    assert!(
        failures.is_empty(),
        "{} of {} rules differ:\n{}",
        failures.len(),
        cases.len(),
        failures.join("\n")
    );
}
