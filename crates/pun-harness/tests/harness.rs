//! Runs the harness binary against a throwaway root (PUN_HARNESS_ROOT) that borrows the real
//! .oracle/ and scripts/ but has its own two-project corpus. Expected counts and diffs are
//! hand-derived from the corpus below: Pint (laravel preset) turns `array(...)` into `[...]`.

use std::fs;
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::{Path, PathBuf};
use std::process::Command;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn write(path: &Path, body: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, body).unwrap();
}

fn script(path: &Path, body: &str) {
    write(path, &format!("#!/bin/sh\n{body}\n"));
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
}

/// root/{.oracle,scripts} -> real ones; corpus a (pint.json with `exclude`, node_modules, a
/// symlink) and b (no pint.json); a scoreboard with one foreign row and a stale row for `a`.
fn root(name: &str) -> PathBuf {
    let r = std::env::temp_dir().join(format!("pun-harness-test-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&r);
    fs::create_dir_all(&r).unwrap();
    symlink(repo().join(".oracle"), r.join(".oracle")).unwrap();
    symlink(repo().join("scripts"), r.join("scripts")).unwrap();
    write(
        &r.join("corpus/manifest.toml"),
        "[[project]]\nname = \"a\"\n\n[[project]]\nname = \"b\"\n",
    );
    let a = r.join("corpus/a");
    fs::create_dir_all(a.join(".git")).unwrap();
    write(
        &a.join("pint.json"),
        r#"{"preset":"laravel","exclude":["skip"]}"#,
    );
    write(&a.join("app.php"), "<?php\n\n$a = array(1);   \n");
    write(&a.join("ok.php"), "<?php\n\n$a = [1];\n");
    write(&a.join("skip/x.php"), "<?php\n\n$b = array(2);\n");
    write(&a.join("readme.txt"), "hi\n");
    symlink("readme.txt", a.join("notes")).unwrap();
    write(&a.join("node_modules/m/index.js"), "x\n");
    let b = r.join("corpus/b");
    fs::create_dir_all(b.join(".git")).unwrap();
    write(&b.join("b.php"), "<?php\n\n$x = array();\n");
    write(
        &r.join("context/reports/scoreboard.md"),
        "| project | x |\n|---|---|\n| a | stale |\n| zz | 1 | 1 | 1 | 0 | 0 | 0 |\n\n## Benchmarks\n\n| zz | kept |\n",
    );
    r
}

fn harness(root: &Path, pun: &Path, args: &[&str]) -> (String, String, Option<i32>) {
    let out = Command::new(env!("CARGO_BIN_EXE_pun-harness"))
        .arg("format")
        .args(args)
        .env("PUN_HARNESS_ROOT", root)
        .env("PUN_BIN", pun)
        .output()
        .unwrap();
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code(),
    )
}

#[test]
fn full_run_with_failing_pun() {
    let r = root("full");
    // A "pun" that changes nothing, adds a file and fails.
    let pun = r.join("fake-pun");
    script(&pun, "echo new > new.txt; exit 3");
    let (out, err, code) = harness(&r, &pun, &["--diffs", "1"]);
    // a: union = app.php, new.txt, ok.php, pint.json, readme.txt, skip/x.php (symlinks are not listed).
    // Pint fixes app.php (array + trailing spaces); skip/ is excluded. pun only adds new.txt.
    let want = "a: 6 files, 4 identical, 2 differing (pint changed 1, pun changed 1)\n\
                --- oracle/app.php\n+++ pun/app.php\n@@ -1,3 +1,3 @@\n <?php\n \n-$a = [1];\n+$a = array(1);   \n\
                b: 2 files, 0 identical, 2 differing (pint changed 1, pun changed 1)\n\
                --- oracle/b.php\n+++ pun/b.php\n@@ -1,3 +1,3 @@\n <?php\n \n-$x = [];\n+$x = array();\n";
    assert_eq!(out, want, "stderr: {err}");
    assert_eq!(code, Some(1));
    assert_eq!(
        err.matches("warning: pun exited exit status: 3").count(),
        2,
        "{err}"
    );
    let board = fs::read_to_string(r.join("context/reports/scoreboard.md")).unwrap();
    assert!(board.ends_with(
        "| project | files | identical | differing | pint changed | pun changed |\n|---|---|---|---|---|---|\n\
         | a | 6 | 4 | 2 | 1 | 1 |\n| b | 2 | 0 | 2 | 1 | 1 |\n| zz | 1 | 1 | 1 | 0 | 0 | 0 |\n\n## Benchmarks\n\n| zz | kept |\n"
    ), "{board}");
    fs::remove_dir_all(&r).unwrap();
}

#[test]
fn single_fixer_identical() {
    let r = root("fixer");
    // A "pun" that writes exactly what array_syntax alone produces.
    let pun = r.join("fake-pun");
    script(
        &pun,
        "[ -f app.php ] && printf '<?php\\n\\n$a = [1];   \\n' > app.php; [ -f b.php ] && printf '<?php\\n\\n$x = [];\\n' > b.php; true",
    );
    let before = fs::read_to_string(r.join("context/reports/scoreboard.md")).unwrap();
    let (out, err, code) = harness(&r, &pun, &["--fixer", "array_syntax"]);
    // Only array_syntax runs, so the trailing spaces survive and skip/ stays excluded (pint.json's `exclude` is kept).
    // The extra single-fixer.json lives outside the copied tree.
    assert_eq!(
        out,
        "a: 5 files, 5 identical, 0 differing (pint changed 1, pun changed 1)\nb: 1 files, 1 identical, 0 differing (pint changed 1, pun changed 1)\n",
        "stderr: {err}"
    );
    assert_eq!(code, Some(0));
    // --fixer runs leave the scoreboard alone.
    assert_eq!(
        fs::read_to_string(r.join("context/reports/scoreboard.md")).unwrap(),
        before
    );

    // --corpus filter, a fixer the project doesn't configure (falls back to `true`), and no scoreboard yet.
    fs::remove_file(r.join("context/reports/scoreboard.md")).unwrap();
    let (out, err, code) = harness(
        &r,
        &pun,
        &["--corpus", "b", "--fixer", "no_trailing_whitespace"],
    );
    assert_eq!(
        out,
        "b: 1 files, 0 identical, 1 differing (pint changed 0, pun changed 1)\n--- oracle/b.php\n+++ pun/b.php\n@@ -1,3 +1,3 @@\n <?php\n \n-$x = array();\n+$x = [];\n",
        "stderr: {err}"
    );
    assert_eq!(code, Some(1));
    assert!(!r.join("context/reports/scoreboard.md").exists());
    fs::remove_dir_all(&r).unwrap();
}

#[test]
fn preset_override() {
    let r = root("preset");
    // A "pun" that, only when migrated with `--preset psr12`, writes what PSR-12 makes of app.php: trailing spaces go,
    // `array()` stays (no array_syntax in psr12, unlike a's laravel preset).
    let pun = r.join("fake-pun");
    script(
        &pun,
        "[ \"$1 $2 $3\" = 'migrate --preset psr12' ] && printf '<?php\\n\\n$a = array(1);\\n' > app.php; true",
    );
    let before = fs::read_to_string(r.join("context/reports/scoreboard.md")).unwrap();
    let (out, err, code) = harness(&r, &pun, &["--corpus", "a", "--preset", "psr12"]);
    assert_eq!(
        out, "a: 5 files, 5 identical, 0 differing (pint changed 1, pun changed 1)\n",
        "stderr: {err}"
    );
    assert_eq!(code, Some(0));
    assert_eq!(
        fs::read_to_string(r.join("context/reports/scoreboard.md")).unwrap(),
        before
    );
    fs::remove_dir_all(&r).unwrap();
}

#[test]
fn missing_pieces() {
    let r = root("missing");
    // No pun binary: identity, with a note. Only b so the run is short; b then matches its source.
    let (out, err, code) = harness(&r, &r.join("no-such-pun"), &["--corpus", "b"]);
    assert_eq!(
        out,
        "b: 1 files, 0 identical, 1 differing (pint changed 1, pun changed 0)\n--- oracle/b.php\n+++ pun/b.php\n@@ -1,3 +1,3 @@\n <?php\n \n-$x = [];\n+$x = array();\n"
    );
    assert_eq!(
        err,
        format!(
            "note: {} missing, treating pun as identity\n",
            r.join("no-such-pun").display()
        )
    );
    assert_eq!(code, Some(1));

    // A pun that exists but can't be spawned.
    write(&r.join("not-exec"), "");
    let (_, err, code) = harness(&r, &r.join("not-exec"), &["--corpus", "b"]);
    assert!(err.starts_with("Error: spawning pun migrate\n"), "{err}");
    assert_eq!(code, Some(1));

    // No php to run the oracle with.
    let out = Command::new(env!("CARGO_BIN_EXE_pun-harness"))
        .args(["format", "--corpus", "b"])
        .env("PUN_HARNESS_ROOT", &r)
        .env("PUN_BIN", r.join("no-such-pun"))
        .env("PATH", "")
        .output()
        .unwrap();
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.contains("Error: spawning pint\n"), "{err}");
    assert_eq!(out.status.code(), Some(1));

    // A corpus project that was never fetched.
    fs::remove_dir_all(r.join("corpus/b/.git")).unwrap();
    let (_, err, code) = harness(&r, &r.join("no-such-pun"), &["--corpus", "b"]);
    assert!(
        err.ends_with(&format!(
            "Error: {} missing: run scripts/corpus.sh\n",
            r.join("corpus/b").display()
        )),
        "{err}"
    );
    assert_eq!(code, Some(1));

    // No oracle.
    fs::remove_file(r.join(".oracle")).unwrap();
    let (_, err, code) = harness(&r, &r.join("no-such-pun"), &[]);
    assert_eq!(
        err,
        format!(
            "Error: {} missing: run scripts/oracle.sh\n",
            r.join(".oracle/pint/builds/pint").display()
        )
    );
    assert_eq!(code, Some(1));
    fs::remove_dir_all(&r).unwrap();
}
