//! Git selection and run-mode flags against the real Pint (`php .oracle/pint/builds/pint`). Every scenario is a bash
//! script that builds a throwaway git repo; it runs twice (one copy per tool, so readdir order agrees) and both tools
//! must print the same stdout/stderr (repo paths normalised; human runs: the same files named, tests/common), exit the
//! same and leave the same files. Pun runs
//! `pun migrate` on the config Pint reads (pint.json in the cwd, `--config`, `--preset`), then `pun format`. Pun-only
//! flags (`--stdin`, `--staged`, `--since`) are compared with Pint run on the same content or the same explicit file
//! list.
//! Skipped without the oracle.
mod common;

use common::{oracle, scratch};

use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A committed repo on `main`: fixable files (`bad`), a clean one, a vendor file, a corpus file when available.
const BASE: &str = r#"
g() { git -c user.email=t@t -c user.name=t -c commit.gpgsign=false "$@"; }
bad() { mkdir -p "$(dirname "$1")"; printf '<?php\n$a = array(1,2);   \n' > "$1"; }
more() { printf '$b = array(3);\n' >> "$1"; }
git init -q -b main
mkdir -p app/Models
if [ -f "$CORPUS/app/Models/User.php" ]; then cp "$CORPUS/app/Models/User.php" app/Models/User.php; fi
for f in app/Mod.php app/Del.php app/Ren.php app/Staged.php app/Keep.php "app/With Space.php" sub/inner/S.php sub/T.php vendor/V.php; do bad "$f"; done
printf '<?php\n\n$a = [1];\n' > app/Clean.php
g add -A; g commit -qm init
"#;

/// On a feature branch: modified, deleted, renamed (`R`), renamed + modified (`RM`), staged + modified, a quoted
/// (spaced) name, untracked files and an untracked directory, a modified vendor file.
const DIRTY: &str = r#"
g checkout -qb feature
more app/Mod.php; rm app/Del.php
g mv app/Ren.php app/Renamed.php
g mv app/Keep.php app/Kept.php; more app/Kept.php
more app/Staged.php; g add app/Staged.php; more app/Staged.php
more "app/With Space.php"; more vendor/V.php; more sub/T.php
bad app/New.php; bad "app/New Space.php"; bad sub/inner/U.php; bad newdir/deep/X.php
"#;

/// `main` moves on after `feature` branched off; `feature` has a commit plus staged, unstaged and untracked changes.
const DIVERGED: &str = r#"
g checkout -qb feature
more app/Mod.php; g commit -qam feat
g checkout -q main
more app/Keep.php; g commit -qam main1
bad app/MainOnly.php; g add -A; g commit -qm main2
g checkout -q feature
more sub/T.php; g add sub/T.php
more sub/inner/S.php
bad sub/Untr.php; bad "sub/Un tr.php"
g mv app/Ren.php app/Renamed.php; g rm -q app/Del.php
"#;

#[derive(Debug, PartialEq)]
struct Run {
    stdout: String,
    stderr: String,
    code: Option<i32>,
    files: BTreeMap<String, Vec<u8>>,
}

/// Builds `BASE + script` in a fresh dir and runs `program` from `sub` with an isolated HOME/TMPDIR, after `pun migrate <migrate>` there when given. Repo paths in the output become `ROOT`.
fn run(
    program: &[&str],
    script: &str,
    sub: &str,
    args: &[&str],
    stdin: &[u8],
    migrate: Option<&[&str]>,
) -> Run {
    // One path per test thread, so a path wrapped in an error box is the same for both tools.
    thread_local! { static REPO: PathBuf = scratch("flags", "repo"); }
    let dir = REPO.with(PathBuf::clone);
    std::fs::remove_dir_all(&dir).unwrap();
    std::fs::create_dir_all(&dir).unwrap();
    let corpus = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus/laravel-laravel");
    let ok = Command::new("bash")
        .args(["-ec", &format!("{BASE}\n{script}")])
        .current_dir(&dir)
        .env("CORPUS", corpus)
        .output()
        .unwrap();
    assert!(
        ok.status.success(),
        "setup: {}",
        String::from_utf8_lossy(&ok.stderr)
    );
    let home = dir.join("home");
    std::fs::create_dir_all(&home).unwrap();
    if let Some(m) = migrate {
        let out = Command::new(env!("CARGO_BIN_EXE_pun"))
            .arg("migrate")
            .args(m)
            .env("PUN_AGENT", "0")
            .current_dir(dir.join(sub))
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
    let mut c = Command::new(program[0]);
    c.args(&program[1..]).args(args).current_dir(dir.join(sub));
    common::clean_env(&mut c, &["PATH"]);
    c.env("HOME", &home)
        .env("PUN_AGENT", "0")
        .env("TMPDIR", &home)
        .env("COLUMNS", "200")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = c.spawn().unwrap();
    child.stdin.take().unwrap().write_all(stdin).unwrap();
    let out = child.wait_with_output().unwrap();
    let files = common::snapshot(&dir, |r| {
        r == ".git" || r == "home" || r.ends_with("pun.toml")
    });
    let root = dir.to_string_lossy().into_owned();
    let norm = |b: &[u8]| {
        String::from_utf8_lossy(b)
            .replace(&root, "ROOT")
            .replace(&root.replace('/', "\\/"), "ROOT")
    };
    Run {
        stdout: norm(&out.stdout),
        stderr: norm(&out.stderr),
        code: out.status.code(),
        files,
    }
}

struct Cx {
    oracle: String,
}

impl Cx {
    fn new() -> Option<Self> {
        let oracle = oracle()?.to_string_lossy().into_owned();
        Some(Cx { oracle })
    }
    fn pint(&self, script: &str, sub: &str, args: &[&str], stdin: &[u8]) -> Run {
        run(&["php", &self.oracle], script, sub, args, stdin, None)
    }
    /// `pun migrate` with Pint's config flags (`--config`, `--preset`), then `pun format` with the rest. `--no-config`
    /// needs no pun.toml, so it skips the migration and keeps the flags.
    fn ours(&self, script: &str, sub: &str, args: &[&str], stdin: &[u8]) -> Run {
        let config = |a: &&str| a.starts_with("--config=") || a.starts_with("--preset=");
        let pun = [env!("CARGO_BIN_EXE_pun"), "format"];
        if args.contains(&"--no-config") {
            return run(&pun, script, sub, args, stdin, None);
        }
        let mut args: Vec<&str> = args.to_vec();
        if let Some(i) = args.iter().position(|a| *a == "--config") {
            let v = args.remove(i + 1);
            args[i] = Box::leak(format!("--config={v}").into_boxed_str());
        }
        let (migrate, rest): (Vec<&str>, Vec<&str>) = args.into_iter().partition(config);
        run(&pun, script, sub, &rest, stdin, Some(&migrate))
    }
    /// Same output, exit code and files; `pint_args` for Pun-only spellings.
    fn same_as(
        &self,
        script: &str,
        sub: &str,
        args: &[&str],
        pint_args: &[&str],
        stdin: &[u8],
    ) -> Run {
        let want = self.pint(script, sub, pint_args, stdin);
        let mut want = want;
        let mut got = self.ours(script, sub, args, stdin);
        if common::human(args) {
            (want.stdout, got.stdout) = common::human_pair(
                &want.stdout,
                &got.stdout,
                want.files.keys(),
                got.files.keys(),
            );
        }
        assert_eq!(got, want, "{sub:?} {args:?}");
        want
    }
    fn same(&self, script: &str, sub: &str, args: &[&str]) -> Run {
        self.same_as(script, sub, args, args, b"")
    }
    /// Pint aborts (exit 1) with its error box carrying `message`; Pun exits 2 (output.md §2). A `--format` or `-q`
    /// run prints the same box; a human run prints `pun` (Pun's run error, ASCII without a UTF-8 locale) on stderr
    /// instead, after its usual output. The files are the same.
    fn same_error(&self, script: &str, args: &[&str], message: &str, pun: &str) {
        let mut want = self.pint(script, "", args, b"");
        let mut got = self.ours(script, "", args, b"");
        let squash = |s: &str| s.split_whitespace().collect::<String>();
        assert_eq!(want.code, Some(1), "{args:?}: {want:?}");
        assert!(
            squash(&want.stdout).contains(&squash(message)),
            "{args:?}: pint said {}",
            want.stdout
        );
        assert_eq!(got.code, Some(2), "{args:?}: {got:?}");
        if common::human(args) {
            (want.stdout, got.stdout) = common::human_pair(
                &want.stdout,
                &got.stdout,
                want.files.keys(),
                got.files.keys(),
            );
            assert_eq!(common::strip_ansi(&got.stderr), pun, "{args:?}");
            // Without a summary (an abort before any file), Pint's stdout is the box alone.
            if !want.stdout.starts_with("named: ") {
                want.stdout = String::new();
            }
            got.stderr = want.stderr.clone();
        }
        got.code = want.code;
        assert_eq!(got, want, "{args:?}");
    }
}

/// The `--format=txt` file list, sorted (Pint lists in readdir order).
fn listed(r: &Run) -> Vec<&str> {
    let mut v: Vec<&str> = r
        .stdout
        .lines()
        .filter_map(|l| {
            l.trim_start()
                .split_once(") ")
                .filter(|(n, _)| n.parse::<u32>().is_ok())
                .map(|x| x.1)
        })
        .collect();
    v.sort();
    v
}

/// `--dirty` = `git status --short -- **.php` in the cwd, minus exact "D", "R" mapped to the new name; quoted
/// names (spaces) and "RM" entries match nothing; the finder still drops vendor. From a subdirectory git's paths
/// and the finder are both relative to the cwd.
#[test]
fn dirty_matches_pint() {
    let Some(cx) = Cx::new() else { return };
    let r = cx.same(DIRTY, "", &["--dirty", "--test", "--format=txt"]);
    assert_eq!(
        listed(&r),
        [
            "app/Mod.php",
            "app/New.php",
            "app/Renamed.php",
            "app/Staged.php",
            "newdir/deep/X.php",
            "sub/T.php",
            "sub/inner/U.php"
        ]
    );
    cx.same(DIRTY, "", &["--dirty"]);
    cx.same(DIRTY, "", &["--dirty", "app/Clean.php"]);
    let r = cx.same(DIRTY, "sub", &["--dirty", "--test", "--format=txt"]);
    assert_eq!(listed(&r), ["T.php", "inner/U.php"]);
    // A clean tree ("No dirty files found."): an empty, successful run. No repository at all: Pint 1.32 aborts.
    cx.same("", "", &["--dirty"]);
    let message = "The [--dirty] option is only available when using Git.";
    cx.same_error("rm -rf .git", &["--dirty", "--format=json"], message, "");
    cx.same_error(
        "rm -rf .git",
        &["--dirty"],
        message,
        &format!("x {message}\n"),
    );
}

/// `--diff=<branch>` = committed since the merge-base (`branch...HEAD`, so main's own later commits don't count),
/// staged, unstaged and untracked; deletions are filtered out. `git diff` prints repo-root paths, so from a
/// subdirectory only the untracked files (cwd-relative) survive. An unknown branch aborts the run.
#[test]
fn diff_matches_pint() {
    let Some(cx) = Cx::new() else { return };
    let r = cx.same(DIVERGED, "", &["--diff=main", "--test", "--format=txt"]);
    assert_eq!(
        listed(&r),
        [
            "app/Mod.php",
            "app/Renamed.php",
            "sub/T.php",
            "sub/Un tr.php",
            "sub/Untr.php",
            "sub/inner/S.php"
        ]
    );
    cx.same(DIVERGED, "", &["--diff=main"]);
    let r = cx.same(DIVERGED, "sub", &["--diff=main", "--test", "--format=txt"]);
    assert_eq!(listed(&r), ["Un tr.php", "Untr.php"]);
    let message = "The [--diff] option is only available when using Git.";
    cx.same_error(DIVERGED, &["--diff=nope", "--format=json"], message, "");
    // --dirty wins over --diff.
    cx.same(
        DIVERGED,
        "",
        &["--diff=main", "--dirty", "--test", "--format=txt"],
    );
}

/// Git-selected files go through the resolver like path arguments: a symlink is fixed as its target (and shows as
/// that file), a dangling one aborts the run.
#[test]
fn git_selection_resolves_symlinks_like_pint() {
    let Some(cx) = Cx::new() else { return };
    let link = "ln -s Mod.php app/Link.php; ln -s ../vendor/V.php app/VLink.php";
    let r = cx.same(link, "", &["--dirty"]);
    assert_eq!(r.files["app/Mod.php"], b"<?php\n\n$a = [1, 2];\n");
    cx.same(link, "", &["--diff=main", "--test", "--format=json"]);
    // A dangling symlink is not a file to Symfony's Finder (7.4.17), so it is never selected.
    cx.same("ln -s Missing.php app/Dangling.php", "", &["--dirty"]);
}

/// `--repair` fixes and fails when anything changed, `--bail` stops at the first fixable file without writing.
#[test]
fn repair_and_bail_match_pint() {
    let Some(cx) = Cx::new() else { return };
    let broken = "printf '<?php\\nfunction (\\n' > app/Broken.php";
    for args in [
        &["--repair"][..],
        &["--repair", "app/Clean.php"],
        &["--repair", "--format=json"],
        &["--repair", "app/Broken.php"],
        &["--bail"],
        &["--bail", "-v"],
        &["--bail", "--repair"],
        &["--bail", "app/Clean.php"],
        &["--bail", "app/Broken.php", "app/Mod.php"],
    ] {
        cx.same(broken, "", args);
    }
}

/// `--parallel`/`-p` only changes scheduling; `--max-processes` takes anything (intval), with or without -p.
#[test]
fn parallel_matches_pint() {
    let Some(cx) = Cx::new() else { return };
    for args in [
        &["--parallel"][..],
        &["-p", "--test"],
        &["--parallel", "--max-processes=1", "--test"],
        &["--parallel", "--max-processes=abc", "--test"],
        &["--max-processes=3", "--test"],
    ] {
        cx.same("", "", args);
    }
}

/// Pint's `--cache-file` (its presets never cache, so nothing is written) and Pun's old `--no-cache` are accepted
/// and change nothing.
#[test]
fn cache_flags_are_ignored_like_pint() {
    let Some(cx) = Cx::new() else { return };
    // Pint 1.32 writes PHP-CS-Fixer's cache there (keyed by the PHP runtime's version); Pun has no cache, so the
    // file is the only difference.
    for args in [
        &["--cache-file=c/pint.cache", "--test"][..],
        &["--cache-file", "c/pint.cache"],
    ] {
        let mut want = cx.pint("", "", args, b"");
        assert!(want.files.remove("c/pint.cache").is_some(), "{args:?}");
        let mut got = cx.ours("", "", args, b"");
        if common::human(args) {
            (want.stdout, got.stdout) = common::human_pair(
                &want.stdout,
                &got.stdout,
                want.files.keys(),
                got.files.keys(),
            );
        }
        assert_eq!(got, want, "{args:?}");
    }
    cx.same_as("", "", &["--no-cache", "--test"], &["--test"], b"");
}

/// `--config` (relative to the cwd, from a subdirectory too), `--no-config` (beats `--config` and the cwd's
/// pint.json), and a subdirectory's own pint.json is not looked up from its parent.
#[test]
fn config_flags_match_pint() {
    let Some(cx) = Cx::new() else { return };
    let cfg = r#"mkdir -p cfg; printf '{"preset":"psr12","exclude":["sub"]}' > cfg/other.json
printf '{"preset":"empty","rules":{"array_syntax":true},"notName":["Keep.php"]}' > pint.json"#;
    for (sub, args) in [
        ("", &["--test"][..]),
        ("", &["--no-config", "--test"]),
        ("", &["--config=cfg/other.json"]),
        ("", &["--config", "cfg/other.json", "--no-config", "--test"]),
        ("", &["--no-config", "--preset=psr12", "--test"]),
        ("sub", &["--config=../cfg/other.json", "--test"]),
        ("sub", &["--test"]),
        // A missing --config file is silently ignored: laravel preset.
        ("", &["--config=missing.json"]),
    ] {
        cx.same(cfg, sub, args);
    }
    // A broken pint.json: Pint's error box; `pun migrate` stops with the same message (with the absolute path).
    let bad = &format!("{cfg}\nprintf 'not json' > bad.json");
    let r = cx.pint(bad, "", &["--config=bad.json"], b"");
    assert_eq!(r.code, Some(1));
    assert!(
        r.stdout
            .contains("The configuration file [bad.json] is not valid JSON."),
        "{}",
        r.stdout
    );
    let r = run(
        &[env!("CARGO_BIN_EXE_pun"), "migrate", "--config=bad.json"],
        bad,
        "",
        &[],
        b"",
        None,
    );
    assert_eq!(r.code, Some(2));
    assert!(
        r.stderr
            .starts_with("x The configuration file [ROOT/bad.json] is not valid JSON."),
        "{}",
        r.stderr
    );
}

/// `--output-to-file` writes only with `--output-format` or `--format`; an unwritable target is PHP's
/// file_put_contents warning as an uncaught exception, after the normal output.
#[test]
fn output_to_file_matches_pint() {
    let Some(cx) = Cx::new() else { return };
    for args in [
        &["--test", "--output-to-file=r.json", "--output-format=json"][..],
        &["--output-to-file=r.json", "--output-format=json"],
        &["--test", "--output-to-file=r.txt"],
        &["--test", "--output-to-file=r.txt", "--format=txt"],
        &[
            "--test",
            "--output-to-file=r.xml",
            "--format=json",
            "--output-format=checkstyle",
        ],
        &["--test", "--output-to-file=r.x", "--output-format=gitlab"],
        &[
            "--test",
            "-q",
            "--output-to-file=r.x",
            "--output-format=junit",
        ],
        &["--dirty", "--output-to-file=r.json", "--output-format=json"],
    ] {
        cx.same("bad app/New.php", "", args);
    }
    // Run errors: exit 2 for Pun (output.md §2), Pint's box kept for -q and --format runs.
    let formats = "use json, txt, agent, checkstyle, junit, xml, gitlab or github";
    for (args, message, pun) in [
        (
            &["--test", "--output-to-file=r.x", "--output-format=nope"][..],
            "Format [nope] is not supported.".to_string(),
            format!("x Format [nope] is not supported.\n  {formats}\n"),
        ),
        (
            &[
                "--test",
                "-q",
                "--output-to-file=r.x",
                "--output-format=nope",
            ],
            "Format [nope] is not supported.".into(),
            String::new(),
        ),
        (
            &[
                "--test",
                "--output-to-file=nodir/r.json",
                "--output-format=json",
            ],
            "file_put_contents(nodir/r.json): Failed to open stream: No such file or directory"
                .into(),
            "x can't write the report to nodir/r.json: No such file or directory\n".into(),
        ),
        (
            &[
                "--test",
                "--output-to-file=app",
                "--output-format=json",
                "--ansi",
            ],
            "file_put_contents(app): Failed to open stream: Is a directory".into(),
            "x can't write the report to app: Is a directory\n".into(),
        ),
        (
            &["--test", "--format=json", "--output-to-file=app"],
            "file_put_contents(app): Failed to open stream: Is a directory".into(),
            String::new(),
        ),
    ] {
        cx.same_error("bad app/New.php", args, &message, &pun);
    }
}

/// Stdin (`-`, `--stdin-filename` alone, or Pun's `--stdin`): fixed code on stdout; excluded names, invalid code and
/// dry runs (`--test`/`--bail`, which leave Pint's temp file alone) echo the input; always exit 0.
#[test]
fn stdin_matches_pint() {
    let Some(cx) = Cx::new() else { return };
    let cfg = r#"printf '{"preset":"empty","rules":{"array_syntax":true},"notName":["Keep.php"]}' > pint.json"#;
    for input in [
        &b"<?php\n$a = array(1,2);   \n"[..],
        b"<?php\nfunction (\n",
        b"",
    ] {
        for (args, pint_args) in [
            (&["--stdin"][..], &["-"][..]),
            (&["--stdin", "--test"], &["-", "--test"]),
            (&["--stdin", "--no-config"], &["-", "--no-config"]),
            (&["-", "--bail"], &["-", "--bail"]),
            (
                &["--stdin-filename=app/X.php"],
                &["--stdin-filename=app/X.php"],
            ),
            (
                &["--stdin", "--stdin-filename=app/Keep.php"],
                &["-", "--stdin-filename=app/Keep.php"],
            ),
            (
                &["--stdin-filename=storage/a/X.php", "-"],
                &["--stdin-filename=storage/a/X.php", "-"],
            ),
            (
                &["--stdin-filename=app/X.php", "--repair"],
                &["--stdin-filename=app/X.php", "--repair"],
            ),
        ] {
            cx.same_as(cfg, "", args, pint_args, input);
        }
    }
    // A fixer exception (a panic in Pun) echoes the input too. combine_consecutive_unsets is a lint rule in Pun, so
    // `pun format` leaves it out and `pun check --fix` is the run that throws.
    let cfg =
        r#"printf '{"preset":"empty","rules":{"combine_consecutive_unsets":true}}' > pint.json"#;
    let input = b"<?php unset($a ); unset( $b );\nunset($x, ); unset( $y);";
    let r = cx.same_as(cfg, "", &["-"], &["-"], input);
    assert!(
        r.stdout.starts_with("<?php unset($a ); unset( $b );") && r.stderr.is_empty(),
        "{r:?}"
    );
    let pun = [env!("CARGO_BIN_EXE_pun"), "check", "--fix"];
    assert_eq!(run(&pun, cfg, "", &["-"], input, Some(&[])), r);
}

/// Pun's `--staged` (`git diff --cached`) and `--since=<ref>` (`git diff <ref>` against the worktree, no merge-base,
/// plus untracked) select exactly what Pint processes when given that file list.
#[test]
fn staged_and_since_match_pint_on_the_same_files() {
    let Some(cx) = Cx::new() else { return };
    let txt = ["--test", "--format=txt"];
    // Our list is in finder order, Pint's in argument order.
    for (flag, files) in [
        ("--staged", &["app/Renamed.php", "sub/T.php"][..]),
        // main's own commits count too (Keep.php); MainOnly.php is absent from the worktree.
        (
            "--since=main",
            &[
                "app/Keep.php",
                "app/Mod.php",
                "app/Renamed.php",
                "sub/T.php",
                "sub/inner/S.php",
                "sub/Untr.php",
                "sub/Un tr.php",
            ],
        ),
    ] {
        let got = cx.ours(DIVERGED, "", &[&txt[..], &[flag]].concat(), b"");
        let want = cx.pint(DIVERGED, "", &[&txt[..], files].concat(), b"");
        assert_eq!(
            (listed(&got), got.code),
            (listed(&want), want.code),
            "{flag}"
        );
        assert_eq!(listed(&got).len(), files.len(), "{flag}");
    }
    // An unknown ref aborts like Pint's `--diff=nope`, naming `--since`.
    let got = cx.ours(DIVERGED, "", &["--since=nope"], b"");
    assert_eq!(
        (common::strip_ansi(&got.stderr), got.code),
        (
            "x The [--since] option is only available when using Git.\n".into(),
            Some(2)
        )
    );
}
