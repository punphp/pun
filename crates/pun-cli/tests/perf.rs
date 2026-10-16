//! Speed features with timing-free, hand-derived expectations: `PUN_PROFILE` stage counts.
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

fn project(name: &str, files: &[(&str, &str)]) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let d = std::env::temp_dir().join(format!("pun-perf-{name}-{}-{}", std::process::id(), N.fetch_add(1, Ordering::Relaxed)));
    let _ = std::fs::remove_dir_all(&d);
    for (f, c) in [("composer.json", r#"{"require":{"php":"^8.4"}}"#), ("pint.json", r#"{"preset":"empty","rules":{"no_trailing_whitespace":true}}"#)]
        .iter()
        .chain(files)
    {
        let p = d.join(f);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, c).unwrap();
    }
    d.canonicalize().unwrap()
}

fn pun(dir: &Path, env: &[(&str, &str)], args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_pun")).current_dir(dir).envs(env.iter().copied()).arg("format").args(args).output().unwrap()
}

#[test]
fn profile_reports_stage_counts() {
    let dir = project("profile", &[("a.php", "<?php\n$a = 1;   \n"), ("b.php", "<?php\n$b = 2;\n")]);
    let out = pun(&dir, &[("PUN_PROFILE", "1")], &["--test"]);
    assert_eq!(out.status.code(), Some(1));
    let err = String::from_utf8(out.stderr).unwrap();
    let count = |stage: &str| -> Option<u64> {
        err.lines().find(|l| l.ends_with(&format!(" {stage}"))).map(|l| l.split_whitespace().nth(1).unwrap().parse().unwrap())
    };
    // Both files are linted, tokenized and handed to the one fixer; only a.php changes, so one re-lint.
    assert_eq!((count("(lint)"), count("(tokenize)"), count("no_trailing_whitespace")), (Some(2), Some(2), Some(2)));
    assert_eq!(count("(lint after fix)"), Some(1));
    // Off by default.
    let quiet = pun(&dir, &[], &["--test"]);
    assert!(!String::from_utf8(quiet.stderr).unwrap().contains("(tokenize)"));
}
