//! Speed features with timing-free, hand-derived expectations: `PUN_PROFILE` stage counts.
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

fn project(name: &str, files: &[(&str, &str)]) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let d = std::env::temp_dir().join(format!(
        "pun-perf-{name}-{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&d);
    for (f, c) in [
        ("composer.json", r#"{"require":{"php":"^8.4"}}"#),
        (
            "pun.toml",
            "[format]\npreset = \"empty\"\n[format.rules]\n\"whitespace/no-trailing\" = true\n",
        ),
    ]
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
    Command::new(env!("CARGO_BIN_EXE_pun"))
        .current_dir(dir)
        .env("PUN_AGENT", "0")
        // The cache under the project, not the user's ~/.cache/pun.
        .env("XDG_CACHE_HOME", dir.join(".cache"))
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

/// The `cache:` stat line `--explain` prints: (hits, files).
fn cache_stat(out: &Output) -> Option<(usize, usize)> {
    let err = String::from_utf8_lossy(&out.stderr);
    let l = err.lines().find_map(|l| l.strip_prefix("cache: "))?;
    let w: Vec<&str> = l.split(' ').collect();
    Some((w[0].parse().unwrap(), w[2].parse().unwrap()))
}

#[test]
fn cache_skips_clean_unchanged_files() {
    let files = [
        ("clean.php", "<?php\n$a = 1;\n"),
        ("dirty.php", "<?php\n$b = 2;   \n"),
        ("broken.php", "<?php\nfunction (\n"),
    ];
    let dir = project("cache", &files);
    let cache = dir.join("c/pun.json");
    let run = |extra: &[&str]| {
        let args: Vec<&str> = ["--test", "--explain", "--cache-file", "c/pun.json"]
            .iter()
            .chain(extra)
            .copied()
            .collect();
        pun(&dir, &[], &args)
    };
    // First run: nothing cached. Second: only clean.php was clean (dirty.php needs a fix, broken.php fails to lint).
    let first = run(&[]);
    assert_eq!(
        (cache_stat(&first), first.status.code()),
        (Some((0, 3)), Some(1))
    );
    let second = run(&[]);
    // The report but for the run's time, the summary's last word.
    let report = |out: &Output| {
        let text = String::from_utf8_lossy(&out.stdout).into_owned();
        text.trim_end()
            .rsplit_once(' ')
            .map(|(head, _)| head.to_string())
    };
    assert_eq!(
        (cache_stat(&second), report(&second)),
        (Some((1, 3)), report(&first)),
        "same report, cached or not"
    );
    // Changed content is checked again.
    std::fs::write(dir.join("clean.php"), "<?php\n$a = 3;\n").unwrap();
    assert_eq!(cache_stat(&run(&[])), Some((0, 3)));
    assert_eq!(cache_stat(&run(&[])), Some((1, 3)));
    // A fixed file is not stored as clean (fixers are single-pass); the next run checks it, then it is.
    pun(&dir, &[], &["--cache-file", "c/pun.json"]);
    assert_eq!(cache_stat(&run(&[])), Some((1, 3)));
    assert_eq!(cache_stat(&run(&[])), Some((2, 3)));
    // Any rule/option change invalidates everything.
    std::fs::write(
        dir.join("pun.toml"),
        "[format]\npreset = \"empty\"\n[format.rules]\n\"whitespace/no-trailing\" = true\n\"strings/single-quote\" = true\n",
    )
    .unwrap();
    assert_eq!(cache_stat(&run(&[])), Some((0, 3)));
    assert_eq!(cache_stat(&run(&[])), Some((2, 3)));
    // A corrupt cache file is ignored and rewritten.
    std::fs::write(&cache, "{not json").unwrap();
    assert_eq!(cache_stat(&run(&[])), Some((0, 3)));
    assert_eq!(cache_stat(&run(&[])), Some((2, 3)));
    // --no-cache neither reads nor writes it.
    std::fs::remove_file(&cache).unwrap();
    assert_eq!(cache_stat(&run(&["--no-cache"])), None);
    assert!(!cache.exists());
    // An unwritable location only costs the speed-up.
    std::fs::write(dir.join("c/file"), "").unwrap();
    let out = pun(
        &dir,
        &[],
        &["--test", "--explain", "--cache-file", "c/file/pun.json"],
    );
    assert_eq!(
        (cache_stat(&out), out.status.code()),
        (Some((0, 3)), Some(1))
    );
    assert_eq!(
        cache_stat(&pun(
            &dir,
            &[],
            &["--test", "--explain", "--cache-file", "c/file/pun.json"]
        )),
        Some((0, 3))
    );
}

#[test]
fn cache_default_location() {
    let dir = project("cache-home", &[("a.php", "<?php\n$a = 1;\n")]);
    let home = dir.join("home");
    let files_in = |d: &Path| std::fs::read_dir(d).map(|r| r.count()).unwrap_or(0);
    // XDG_CACHE_HOME wins; without it ~/.cache; with neither there is no cache.
    pun(
        &dir,
        &[("XDG_CACHE_HOME", home.join("xdg").to_str().unwrap())],
        &["--test"],
    );
    assert_eq!(files_in(&home.join("xdg/pun")), 1);
    let out = pun(
        &dir,
        &[("XDG_CACHE_HOME", ""), ("HOME", home.to_str().unwrap())],
        &["--test", "--explain"],
    );
    assert_eq!(
        (cache_stat(&out), files_in(&home.join(".cache/pun"))),
        (Some((0, 1)), 1)
    );
    let out = pun(
        &dir,
        &[("XDG_CACHE_HOME", ""), ("HOME", home.to_str().unwrap())],
        &["--test", "--explain"],
    );
    assert_eq!(cache_stat(&out), Some((1, 1)));
    assert_eq!(
        cache_stat(&pun(
            &dir,
            &[("XDG_CACHE_HOME", ""), ("HOME", "")],
            &["--test", "--explain"]
        )),
        None
    );
}
