//! Git selection and run-mode flags against the real Pint (`php .oracle/pint/builds/pint`). Every scenario is a bash
//! script that builds a throwaway git repo; it runs twice (one copy per tool, so readdir order agrees) and both tools
//! must print the same stdout/stderr (repo paths normalised), exit the same and leave the same files. Pun-only flags
//! (`--stdin`, `--staged`, `--since`) are compared with Pint run on the same content or the same explicit file list.
//! Skipped without the oracle.
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

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

fn oracle() -> Option<PathBuf> {
    let p = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.oracle/pint/builds/pint");
    p.exists().then_some(p)
}

fn tmp(name: &str) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let d = std::env::temp_dir().join(format!("pun-flags-{name}-{}-{}", std::process::id(), N.fetch_add(1, Ordering::Relaxed)));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d.canonicalize().unwrap()
}

fn snapshot(dir: &Path, rel: &str, out: &mut BTreeMap<String, Vec<u8>>) {
    for e in std::fs::read_dir(dir.join(rel)).unwrap().flatten() {
        let r = if rel.is_empty() { e.file_name().to_string_lossy().into_owned() } else { format!("{rel}/{}", e.file_name().to_string_lossy()) };
        if r == ".git" || r == "home" || e.path().is_symlink() {
            continue;
        }
        if e.path().is_dir() { snapshot(dir, &r, out) } else { out.insert(r, std::fs::read(e.path()).unwrap()); }
    }
}

#[derive(Debug, PartialEq)]
struct Run {
    stdout: String,
    stderr: String,
    code: Option<i32>,
    files: BTreeMap<String, Vec<u8>>,
}

/// Builds `BASE + script` in a fresh dir and runs `program` from `sub` with an isolated HOME/TMPDIR (no Pint result
/// cache, no Pun cache). Repo paths in the output become `ROOT`.
fn run(program: &[&str], script: &str, sub: &str, args: &[&str], stdin: &[u8]) -> Run {
    let dir = tmp("repo");
    let corpus = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus/laravel-laravel");
    let ok = Command::new("bash").args(["-ec", &format!("{BASE}\n{script}")]).current_dir(&dir).env("CORPUS", corpus).output().unwrap();
    assert!(ok.status.success(), "setup: {}", String::from_utf8_lossy(&ok.stderr));
    let home = dir.join("home");
    std::fs::create_dir_all(&home).unwrap();
    let mut c = Command::new(program[0]);
    c.args(&program[1..]).args(args).current_dir(dir.join(sub)).env_clear();
    for k in ["PATH", "LLVM_PROFILE_FILE"] {
        if let Ok(v) = std::env::var(k) {
            c.env(k, v);
        }
    }
    c.env("HOME", &home).env("TMPDIR", &home).env("COLUMNS", "80").stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = c.spawn().unwrap();
    child.stdin.take().unwrap().write_all(stdin).unwrap();
    let out = child.wait_with_output().unwrap();
    let mut files = BTreeMap::new();
    snapshot(&dir, "", &mut files);
    let root = dir.to_string_lossy().into_owned();
    let norm = |b: &[u8]| String::from_utf8_lossy(b).replace(&root, "ROOT").replace(&root.replace('/', "\\/"), "ROOT");
    Run { stdout: norm(&out.stdout), stderr: norm(&out.stderr), code: out.status.code(), files }
}

struct Cx {
    oracle: String,
    alias: String,
}

impl Cx {
    fn new() -> Option<Self> {
        let oracle = oracle()?.to_string_lossy().into_owned();
        let p = tmp("bin").join("pint");
        std::os::unix::fs::symlink(env!("CARGO_BIN_EXE_pun"), &p).unwrap();
        Some(Cx { oracle, alias: p.to_string_lossy().into_owned() })
    }
    fn pint(&self, script: &str, sub: &str, args: &[&str], stdin: &[u8]) -> Run {
        run(&["php", &self.oracle], script, sub, args, stdin)
    }
    fn ours(&self, script: &str, sub: &str, args: &[&str], stdin: &[u8]) -> Run {
        run(&[&self.alias], script, sub, args, stdin)
    }
    /// Same output, exit code and files; `pint_args` for Pun-only spellings.
    fn same_as(&self, script: &str, sub: &str, args: &[&str], pint_args: &[&str], stdin: &[u8]) -> Run {
        let want = self.pint(script, sub, pint_args, stdin);
        let mut got = self.ours(script, sub, args, stdin);
        if args.iter().any(|a| *a == "--parallel" || *a == "-p") {
            // Pint prints progress in worker completion order; only the multiset is stable.
            let sorted = |s: &str| s.split('\n').map(|l| if l.starts_with("  ") && !l.trim().is_empty() && l.trim().chars().all(|c| ".⨯✓!".contains(c)) { let mut c: Vec<char> = l.chars().collect(); c.sort(); c.into_iter().collect() } else { l.to_string() }).collect::<Vec<_>>().join("\n");
            got.stdout = if sorted(&got.stdout) == sorted(&want.stdout) { want.stdout.clone() } else { got.stdout };
        }
        assert_eq!(got, want, "{sub:?} {args:?}");
        want
    }
    fn same(&self, script: &str, sub: &str, args: &[&str]) -> Run {
        self.same_as(script, sub, args, args, b"")
    }
    /// Both exit 1; Pint's message (in a Symfony box on stdout) contains ours (on stderr), whitespace ignored.
    fn same_error(&self, script: &str, args: &[&str], message: &str) {
        let want = self.pint(script, "", args, b"");
        let got = self.ours(script, "", args, b"");
        let squash = |s: &str| s.split_whitespace().collect::<String>();
        assert_eq!((got.code, want.code), (Some(1), Some(1)), "{args:?}: {want:?} / {got:?}");
        assert!(squash(&want.stdout).contains(&squash(message)), "{args:?}: pint said {}", want.stdout);
        assert_eq!(squash(&got.stderr), squash(message), "{args:?}");
    }
}

/// The `--format=txt` file list, sorted (Pint lists in readdir order).
fn listed(r: &Run) -> Vec<&str> {
    let mut v: Vec<&str> = r.stdout.lines().filter_map(|l| l.trim_start().split_once(") ").filter(|(n, _)| n.parse::<u32>().is_ok()).map(|x| x.1)).collect();
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
    assert_eq!(listed(&r), ["app/Mod.php", "app/New.php", "app/Renamed.php", "app/Staged.php", "newdir/deep/X.php", "sub/T.php", "sub/inner/U.php"]);
    cx.same(DIRTY, "", &["--dirty"]);
    cx.same(DIRTY, "", &["--dirty", "app/Clean.php"]);
    let r = cx.same(DIRTY, "sub", &["--dirty", "--test", "--format=txt"]);
    assert_eq!(listed(&r), ["T.php", "inner/U.php"]);
    // A clean tree ("No dirty files found.") and no repository at all: an empty, successful run.
    cx.same("", "", &["--dirty"]);
    cx.same("rm -rf .git", "", &["--dirty", "--format=json"]);
}

/// `--diff=<branch>` = committed since the merge-base (`branch...HEAD`, so main's own later commits don't count),
/// staged, unstaged and untracked; deletions are filtered out. `git diff` prints repo-root paths, so from a
/// subdirectory only the untracked files (cwd-relative) survive. An unknown branch is an empty run.
#[test]
fn diff_matches_pint() {
    let Some(cx) = Cx::new() else { return };
    let r = cx.same(DIVERGED, "", &["--diff=main", "--test", "--format=txt"]);
    assert_eq!(listed(&r), ["app/Mod.php", "app/Renamed.php", "sub/T.php", "sub/Un tr.php", "sub/Untr.php", "sub/inner/S.php"]);
    cx.same(DIVERGED, "", &["--diff=main"]);
    let r = cx.same(DIVERGED, "sub", &["--diff=main", "--test", "--format=txt"]);
    assert_eq!(listed(&r), ["Un tr.php", "Untr.php"]);
    cx.same(DIVERGED, "", &["--diff=nope", "--format=json"]);
    // --dirty wins over --diff.
    cx.same(DIVERGED, "", &["--diff=main", "--dirty", "--test", "--format=txt"]);
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
    cx.same_error("ln -s Missing.php app/Dangling.php", &["--dirty"], r#"The path "ROOT/app/Dangling.php" is not readable."#);
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
    for args in [&["--parallel"][..], &["-p", "--test"], &["--parallel", "--max-processes=1", "--test"], &["--parallel", "--max-processes=abc", "--test"], &["--max-processes=3", "--test"]] {
        cx.same("", "", args);
    }
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
    let bad = &format!("{cfg}\nprintf 'not json' > bad.json");
    cx.same_error(bad, &["--config=bad.json"], "The configuration file [bad.json] is not valid JSON.");
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
        &["--test", "--output-to-file=r.xml", "--format=json", "--output-format=checkstyle"],
        &["--test", "--output-to-file=r.x", "--output-format=gitlab"],
        &["--test", "--output-to-file=r.x", "--output-format=nope"],
        &["--test", "-q", "--output-to-file=r.x", "--output-format=junit"],
        &["--dirty", "--output-to-file=r.json", "--output-format=json"],
        &["--test", "--output-to-file=nodir/r.json", "--output-format=json"],
        &["--test", "--output-to-file=app", "--output-format=json", "--ansi"],
    ] {
        cx.same("bad app/New.php", "", args);
    }
}

/// Stdin (`-`, `--stdin-filename` alone, or Pun's `--stdin`): fixed code on stdout; excluded names, invalid code and
/// dry runs (`--test`/`--bail`, which leave Pint's temp file alone) echo the input; always exit 0.
#[test]
fn stdin_matches_pint() {
    let Some(cx) = Cx::new() else { return };
    let cfg = r#"printf '{"preset":"empty","rules":{"array_syntax":true},"notName":["Keep.php"]}' > pint.json"#;
    for input in [&b"<?php\n$a = array(1,2);   \n"[..], b"<?php\nfunction (\n", b""] {
        for (args, pint_args) in [
            (&["--stdin"][..], &["-"][..]),
            (&["--stdin", "--test"], &["-", "--test"]),
            (&["--stdin", "--no-config"], &["-", "--no-config"]),
            (&["-", "--bail"], &["-", "--bail"]),
            (&["--stdin-filename=app/X.php"], &["--stdin-filename=app/X.php"]),
            (&["--stdin", "--stdin-filename=app/Keep.php"], &["-", "--stdin-filename=app/Keep.php"]),
            (&["--stdin-filename=storage/a/X.php", "-"], &["--stdin-filename=storage/a/X.php", "-"]),
            (&["--stdin-filename=app/X.php", "--repair"], &["--stdin-filename=app/X.php", "--repair"]),
        ] {
            cx.same_as(cfg, "", args, pint_args, input);
        }
    }
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
        ("--since=main", &["app/Keep.php", "app/Mod.php", "app/Renamed.php", "sub/T.php", "sub/inner/S.php", "sub/Untr.php", "sub/Un tr.php"]),
    ] {
        let got = cx.ours(DIVERGED, "", &[&txt[..], &[flag]].concat(), b"");
        let want = cx.pint(DIVERGED, "", &[&txt[..], files].concat(), b"");
        assert_eq!((listed(&got), got.code), (listed(&want), want.code), "{flag}");
        assert_eq!(listed(&got).len(), files.len(), "{flag}");
    }
    cx.same_as(DIVERGED, "", &["--since=nope"], &["--diff=nope"], b"");
}
