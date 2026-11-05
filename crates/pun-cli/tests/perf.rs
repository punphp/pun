//! Speed features with timing-free, hand-derived expectations: `PUN_PROFILE` stage counts.
mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn project(name: &str, files: &[(&str, &str)]) -> PathBuf {
    let base = [
        ("composer.json", r#"{"require":{"php":"^8.4"}}"#),
        (
            "pun.toml",
            "[format]\npreset = \"empty\"\n[format.rules]\n\"whitespace/no-trailing\" = true\n",
        ),
    ];
    common::project("perf", name, &[&base[..], files].concat())
}

fn pun(dir: &Path, env: &[(&str, &str)], args: &[&str]) -> Output {
    common::clean_env(&mut Command::new(common::PUN), &["PATH", "HOME"])
        .current_dir(dir)
        .env("PUN_AGENT", "0")
        .envs(env.iter().copied())
        .arg("format")
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn profile_reports_stage_counts() {
    let dir = project(
        "profile",
        &[
            ("a.php", "<?php\n$a = 1;   \n"),
            ("b.php", "<?php\n$b = 2;\n"),
        ],
    );
    let out = pun(&dir, &[("PUN_PROFILE", "1")], &["--test"]);
    assert_eq!(out.status.code(), Some(1));
    let err = String::from_utf8(out.stderr).unwrap();
    let count = |stage: &str| -> Option<u64> {
        err.lines()
            .find(|l| l.ends_with(&format!(" {stage}")))
            .map(|l| l.split_whitespace().nth(1).unwrap().parse().unwrap())
    };
    // Both files are linted, tokenized and handed to the one fixer; only a.php changes, so one re-lint.
    assert_eq!(
        (
            count("(lint)"),
            count("(tokenize)"),
            count("no_trailing_whitespace")
        ),
        (Some(2), Some(2), Some(2))
    );
    assert_eq!(count("(lint after fix)"), Some(1));
    let quiet = pun(&dir, &[], &["--test"]);
    assert!(
        !String::from_utf8(quiet.stderr)
            .unwrap()
            .contains("(tokenize)")
    );
}
