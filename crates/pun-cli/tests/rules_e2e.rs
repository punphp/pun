//! Every rule end to end through the real binary: tests/e2e/rules.json (scripts/export-e2e-cases.py) holds, per
//! rule, a one-file project enabling only that rule (pint.json, or pun.toml for Pun/ rules) and what real Pint does
//! to it: the file it writes and `pint --test`'s stdout and exit code. Pun/ rules have no Pint; their expectation
//! is the first Mago-derived case of the fixer's own tests. After `pun migrate` (risky fixers become lint rules),
//! `pun check --fix` must write the same file, and `pun check` must exit the same and name the same rule (in Pun's
//! own output, under its Pun ID) or the same syntax error.

mod common;
#[path = "common/prettier.rs"]
mod prettier;

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;

fn project(case: &Value, i: usize) -> PathBuf {
    let d = common::scratch("rules-e2e", &i.to_string());
    if let Some(cfg) = case.get("pint.json") {
        std::fs::write(d.join("pint.json"), cfg.to_string()).unwrap();
        assert_eq!(pun(&d, &["migrate"]).0, 0, "{cfg}");
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
    let out = common::clean_env(&mut Command::new(common::PUN), &["PATH", "HOME"])
        .args(args)
        .current_dir(dir)
        // The cache under the run's own directory, not the user's ~/.cache/pun (one file per case otherwise).
        .env("XDG_CACHE_HOME", dir.join(".cache"))
        .env("PUN_AGENT", "0")
        // Pun's own output: a UTF-8 locale gets ✖ rather than its ASCII fallback.
        .env("LANG", "en_US.UTF-8")
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
    let node_modules = prettier::node_modules();
    let mut failures = Vec::new();
    for (i, case) in cases.iter().enumerate() {
        let rule = case["rule"].as_str().unwrap();
        let dir = project(case, i);
        if rule == "Pint/laravel_blade" {
            let Some(modules) = &node_modules else {
                eprintln!(
                    "skipping {rule}: no prettier in crates/pun-fixers/node_modules (bun install there)"
                );
                continue;
            };
            #[cfg(unix)]
            std::os::unix::fs::symlink(modules, dir.join("node_modules")).unwrap();
        }
        let (code, stdout) = pun(&dir, &["check"]);
        if code != case["test_exit"].as_i64().unwrap() as i32 {
            failures.push(format!(
                "{rule}: `check` exited {code}, `pint --test` {}\n{stdout}",
                case["test_exit"]
            ));
        }
        // Pun's own output (output.md §1): a fixable file is a ✖ file line naming the rule's Pun ID; Pint's `!` (a
        // syntax error) is a problem block at the file.
        let pint_error =
            (case.get("test_stdout").and_then(Value::as_str)).is_some_and(|s| s.contains("\n  ! "));
        // `pun migrate` folds the deprecated visibility_required into modifier_keywords.
        let fixer = if rule == "visibility_required" {
            "modifier_keywords"
        } else {
            rule
        };
        let id = pun_config::rule_of(fixer).unwrap_or(rule);
        let named = if pint_error {
            stdout.contains(" ✖ syntax-error  ")
                && stdout.contains(&format!("╭─ {}:", case["file"].as_str().unwrap()))
        } else {
            let line = format!(" ✖ {} ", case["file"].as_str().unwrap());
            (stdout.lines()).any(|l| l.starts_with(&line) && l.split([' ', ',']).any(|w| w == id))
        };
        if !named {
            failures.push(format!(
                "{rule}: `check` output does not name `{id}`\n{stdout}"
            ));
        }
        let file = dir.join(case["file"].as_str().unwrap());
        assert_eq!(
            std::fs::read_to_string(&file).unwrap(),
            case["input"].as_str().unwrap(),
            "{rule}: --test wrote the file"
        );
        pun(&dir, &["check", "--fix"]);
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
