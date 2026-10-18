//! `pun migrate` + `pun format` against the real Pint (`php .oracle/pint/builds/pint`) on the same pint.json: same
//! exit code, resulting files and machine-format stdout for every flag combination below, run in the same directory
//! (so diff headers and absolute paths agree). Human output is Pun's own: those runs compare the files each tool
//! names (tests/common). Rules: only fully ported ones, so the output must be byte-identical. Skipped without the
//! oracle. Pun-only behaviour (`--explain`, `config`, unported rules, no pun.toml) is checked by hand-derived
//! expectations at the end.
mod common;
#[path = "common/prettier.rs"]
mod prettier;

use common::{PUN, migrate, oracle, scratch, write_files};

use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const PINT_JSON: &str = r#"{"preset":"empty","rules":{"array_syntax":true,"no_trailing_whitespace":true,"trim_array_spaces":true,"whitespace_after_comma_in_array":true}}"#;
const LONG: &str = "app/Long/VeryLongDirectoryNameToForceTruncationOfTheDescription/File.php";

fn files() -> Vec<(&'static str, &'static str)> {
    vec![
        ("composer.json", r#"{"require":{"php":"^8.4"}}"#),
        ("pint.json", PINT_JSON),
        ("app/Fix.php", "<?php\n$a = array(1,2);   \n"),
        ("app/Clean.php", "<?php\n$a = [1];\n"),
        ("app/Empty.php", ""),
        ("app/with spaçe.php", "<?php\n$b = array( 3 );\n"),
        (LONG, "<?php\n$c = array( 1,2 );\n"),
    ]
}

/// Writes `files` into `dir` (after wiping everything but `.git`), plus a symlinked file.
fn reset(dir: &Path, files: &[(&str, &str)]) {
    for e in std::fs::read_dir(dir).unwrap().flatten() {
        if e.file_name() != ".git" {
            let p = e.path();
            if p.is_dir() && !p.is_symlink() {
                std::fs::remove_dir_all(p).unwrap()
            } else {
                std::fs::remove_file(p).unwrap()
            }
        }
    }
    write_files(dir, files);
    if dir.join("app/Fix.php").exists() {
        std::os::unix::fs::symlink("Fix.php", dir.join("app/Link.php")).unwrap();
    }
}

#[derive(Debug, PartialEq)]
struct Run {
    stdout: String,
    code: Option<i32>,
    files: BTreeMap<String, Vec<u8>>,
}

fn run(
    program: &Path,
    pre: &[&str],
    dir: &Path,
    args: &[&str],
    env: &[(&str, &str)],
    stdin: &[u8],
) -> (Run, String) {
    let mut c = Command::new(program);
    common::clean_env(&mut c, &["PATH", "HOME"]);
    c.args(pre).args(args).current_dir(dir);
    // Pint names its stdin temp file with uniqid(): concurrent runs sharing /tmp can swap inputs.
    let tmp = dir.with_extension("tmp");
    std::fs::create_dir_all(&tmp).unwrap();
    c.env("COLUMNS", "80")
        .env("TMPDIR", &tmp)
        .env("PUN_AGENT", "0")
        .envs(env.iter().copied())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = c.spawn().unwrap();
    child.stdin.take().unwrap().write_all(stdin).unwrap();
    let out = child.wait_with_output().unwrap();
    // pun.toml: what `pun migrate` wrote for Pun's run.
    let files = common::snapshot(dir, |r| r == ".git" || r == "pun.toml");
    (
        Run {
            stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
            code: out.status.code(),
            files,
        },
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn pun_format(dir: &Path, args: &[&str], env: &[(&str, &str)], stdin: &[u8]) -> (Run, String) {
    migrate(dir);
    run(Path::new(PUN), &["format"], dir, args, env, stdin)
}

struct Ctx {
    oracle: PathBuf,
}

impl Ctx {
    fn pint(&self, dir: &Path, args: &[&str], env: &[(&str, &str)], stdin: &[u8]) -> (Run, String) {
        run(
            Path::new("php"),
            &[self.oracle.to_str().unwrap()],
            dir,
            args,
            env,
            stdin,
        )
    }
    /// Runs Pint, resets the tree, runs `pun migrate` and `pun <pre>` (default `format`), compares stdout, exit code
    /// and files.
    fn same(
        &self,
        dir: &Path,
        tree: &[(&str, &str)],
        pre: &[&str],
        args: &[&str],
        env: &[(&str, &str)],
        stdin: &[u8],
    ) {
        self.same_as(dir, tree, pre, args, args, env, stdin);
    }
    /// Like `same`, but Pint gets `pint_args` (for pun-only spellings such as `--check`).
    #[expect(
        clippy::too_many_arguments,
        reason = "`same` plus the arguments Pint gets"
    )]
    fn same_as(
        &self,
        dir: &Path,
        tree: &[(&str, &str)],
        pre: &[&str],
        args: &[&str],
        pint_args: &[&str],
        env: &[(&str, &str)],
        stdin: &[u8],
    ) {
        reset(dir, tree);
        let (mut want, _) = self.pint(dir, pint_args, env, stdin);
        reset(dir, tree);
        migrate(dir);
        let pre = if pre.is_empty() { &["format"][..] } else { pre };
        let (mut got, err) = run(Path::new(PUN), pre, dir, args, env, stdin);
        if common::human(args) {
            (want.stdout, got.stdout) = common::human_pair(
                &want.stdout,
                &got.stdout,
                want.files.keys(),
                got.files.keys(),
            );
        }
        assert_eq!(got, want, "{pre:?} {args:?} {env:?} (our stderr: {err})");
    }
    /// Pint aborts with exit 1 and its error box (on stdout) naming `message`; Pun, a run error (output.md §1),
    /// prints `message` as one line on stderr and exits 2, and the files are the same.
    fn same_error(&self, dir: &Path, tree: &[(&str, &str)], args: &[&str], message: &str) {
        reset(dir, tree);
        let (mut want, werr) = self.pint(dir, args, &[], b"");
        reset(dir, tree);
        let (got, err) = pun_format(dir, args, &[], b"");
        assert_eq!(want.code, Some(1), "{args:?}: {werr}");
        assert!(want.stdout.contains(message), "{}", want.stdout);
        assert_eq!(err, format!("x {message}\n"), "{args:?}");
        (want.stdout, want.code) = (String::new(), Some(2));
        assert_eq!(got, want, "{args:?}");
    }
}

#[test]
fn matches_pint() {
    let Some(oracle) = oracle() else { return };
    let cx = Ctx { oracle };
    let dir = scratch("cli", "p");
    let tree = files();
    for args in [
        &[][..],
        &["--test"],
        &["--test", "-v"],
        &["-v"],
        &["--bail"],
        &["--bail", "-v"],
        &["--repair"],
        &["--format=json"],
        &["--test", "--format=json"],
        &["--test", "--format=json", "-v"],
        &["--format=txt"],
        &["--test", "--format=txt"],
        &["--test", "--format=txt", "-v"],
        &["--format=agent"],
        &["--test", "--format=agent"],
        &["-q"],
        &["--test", "-q"],
        &[
            "--test",
            "--output-to-file=report.json",
            "--output-format=json",
        ],
        &["--format=txt", "--output-to-file=report.txt"],
        &["app/Fix.php"],
        &["app/Clean.php"],
        &["app/Clean.php", "--format=agent"],
        &["app/Clean.php", "--test", "--format=txt"],
        &[
            "app",
            "--test",
            "--parallel",
            "--max-processes=2",
            "--no-interaction",
        ],
        &["--test", "--preset=empty"],
    ] {
        cx.same(&dir, &tree, &[], args, &[], b"");
    }
    // A coding agent in the environment switches Pint to its JSON agent reporter; Pun prints its own agent format
    // there (tests/agent.rs), and that reporter only with --format=agent (above).
    // `pun check` is `pun format --test` here (no lint rules); `--check` is `--test`.
    cx.same_as(&dir, &tree, &["check"], &[], &["--test"], &[], b"");
    cx.same_as(
        &dir,
        &tree,
        &["format"],
        &["--check", "--format=json"],
        &["--test", "--format=json"],
        &[],
        b"",
    );

    // Stdin: fixed code on stdout, excluded paths and invalid code echoed back.
    for args in [
        &["-"][..],
        &["--stdin-filename=app/x.php", "-"],
        &["--stdin-filename=storage/x.php", "-"],
    ] {
        cx.same(&dir, &tree, &[], args, &[], b"<?php\n$a = array(1,2);\n");
        cx.same(&dir, &tree, &[], args, &[], b"<?php\nfunction (\n");
        cx.same(&dir, &tree, &[], args, &[], b"<?php\n$a = [1];\n");
    }

    // Git selection: outside git, in a clean repo, with changes.
    cx.same(&dir, &tree, &[], &["--dirty", "--test"], &[], b"");
    cx.same(&dir, &tree, &[], &["--diff=main", "--test"], &[], b"");

    // Errors: unknown reporter, missing path, missing `in` dir, unreadable / unwritable files.
    cx.same_error(
        &dir,
        &tree,
        &["nope"],
        r#"The path "nope" is not readable."#,
    );
    let mut bad_in = tree.clone();
    bad_in[1] = (
        "pint.json",
        r#"{"preset":"empty","rules":{"array_syntax":true},"in":["nodir"]}"#,
    );
    cx.same_error(
        &dir,
        &bad_in,
        &[],
        r#"The "nodir" directory does not exist."#,
    );
    // Unreadable / unwritable files: same box (see output.rs error_boxes_like_pint for the byte-exact cases).
    // Under an agent Pint drops the progress; Pun prints one error line instead (tests/agent.rs).
    let env: &[(&str, &str)] = &[];
    for (mode, args) in [(0o444, &[][..]), (0o000, &["--test"])] {
        let locked = || {
            reset(&dir, &tree);
            std::fs::set_permissions(
                dir.join("app/Fix.php"),
                std::os::unix::fs::PermissionsExt::from_mode(mode),
            )
            .unwrap();
        };
        locked();
        let (want, _) = cx.pint(&dir, args, env, b"");
        locked();
        let (got, err) = pun_format(&dir, args, env, b"");
        // Pint prints the progress symbols of the files before the aborting one, then the box (exit 1); Pun its
        // run error alone, on stderr (exit 2).
        let fix = dir.join("app/Fix.php");
        let message = match mode {
            0o444 => format!(
                r#"Cannot write to file "{}" as it is not writable."#,
                fix.display()
            ),
            _ => format!(r#"Failed to read content from "{}"."#, fix.display()),
        };
        assert!(want.stdout.starts_with("\n  "), "{}", want.stdout);
        assert_eq!(want.code, Some(1));
        assert_eq!(
            (got.stdout, err, got.code),
            (String::new(), format!("x {message}\n"), Some(2))
        );
    }
    std::fs::set_permissions(
        dir.join("app/Fix.php"),
        std::os::unix::fs::PermissionsExt::from_mode(0o644),
    )
    .unwrap();

    let git = |args: &[&str]| {
        let ok = Command::new("git")
            .args([
                "-c",
                "user.email=t@t",
                "-c",
                "user.name=t",
                "-c",
                "commit.gpgsign=false",
            ])
            .args(args)
            .current_dir(&dir)
            .output()
            .unwrap()
            .status
            .success();
        assert!(ok, "git {args:?}");
    };
    reset(&dir, &tree);
    git(&["init", "-q", "-b", "main"]);
    git(&["add", "."]);
    git(&["commit", "-qm", "init"]);
    let mut changed = tree.clone();
    changed.push(("app/New.php", "<?php\n$n = array(1);\n"));
    cx.same(&dir, &tree, &[], &["--dirty"], &[], b"");
    cx.same(&dir, &changed, &[], &["--dirty", "--test"], &[], b"");
    cx.same(
        &dir,
        &changed,
        &[],
        &["--diff=main", "--test", "--format=json"],
        &[],
        b"",
    );
}

/// Pint/laravel_blade (needs prettier in crates/pun-fixers/node_modules, tests/common/prettier.rs, resolved from the
/// project dir):
/// - embedded PHP is formatted with the configured preset's rules: under preset empty `@if(!$a)` keeps `!$a`
///   (laravel would add a space);
/// - a whitespace-only Blade file becomes empty and the later fixers are skipped (AbstractFixer::fix's
///   `0 < $tokens->count()` guard) instead of running on an empty token list.
#[test]
fn blade_like_pint() {
    let Some(oracle) = oracle() else { return };
    let Some(modules) = prettier::node_modules() else {
        eprintln!("skipping: no prettier in crates/pun-fixers/node_modules (bun install there)");
        return;
    };
    let dir = scratch("cli", "blade");
    for (preset, blade, check) in [
        (
            "empty",
            "<div>\n@if(!$a)\n{{ !$b }}\n@endif\n</div>\n",
            "@if (!$a)",
        ),
        ("laravel", "\n  \n", ""),
    ] {
        let pint_json = format!(r#"{{"preset":"{preset}","rules":{{"Pint/laravel_blade":true}}}}"#);
        let tree = [
            ("composer.json", r#"{"require":{"php":"^8.4"}}"#),
            ("pint.json", &pint_json),
            ("resources/v.blade.php", blade),
        ];
        let mut runs = vec![];
        for pint in [true, false] {
            reset(&dir, &tree);
            std::os::unix::fs::symlink(&modules, dir.join("node_modules")).unwrap();
            runs.push(match pint {
                true => {
                    run(
                        Path::new("php"),
                        &[oracle.to_str().unwrap()],
                        &dir,
                        &["--format=json"],
                        &[],
                        b"",
                    )
                    .0
                }
                false => pun_format(&dir, &["--format=json"], &[], b"").0,
            });
        }
        assert_eq!(runs[1], runs[0], "preset {preset}");
        let out = String::from_utf8_lossy(&runs[0].files["resources/v.blade.php"]).into_owned();
        assert!(
            if check.is_empty() {
                out.is_empty()
            } else {
                out.contains(check)
            },
            "{out:?}"
        );
    }
}

/// `-v` diffs (here in the txt report, which prints them as Pint does) pair up lines exactly like PHP-CS-Fixer's sebastian/diff (LCS + strict unified builder): reordered
/// imports (fixtures/dirty_imports.php, from pinkary-dirty: a Myers diff pairs other lines), a missing final newline, mixed line endings (warning line), and
/// a file big enough for the memory-efficient LCS (> 853 differing lines).
#[test]
fn diffs_like_pint() {
    let Some(oracle) = oracle() else { return };
    let cx = Ctx { oracle };
    let dir = scratch("cli", "diff");
    let big: String = std::iter::once("<?php\n".to_string())
        .chain((0..1000).map(|i| format!("$a{i} = array({i});\n$b = {};\n", i % 7)))
        .collect();
    let tree = [
        ("composer.json", r#"{"require":{"php":"^8.4"}}"#),
        ("pint.json", r#"{"preset":"laravel"}"#),
        (
            "app/Imports.php",
            include_str!("fixtures/dirty_imports.php"),
        ),
        ("app/NoEol.php", "<?php\n\n$a = array(1);\n$b = 2;"),
        // No line break in the old file at all: no "different line endings" warning (detectUnmatchedLineEndings).
        ("app/OneLine.php", "<?php $a = array(1);"),
        ("app/Crlf.php", "<?php\r\n\r\n$a = array(1);\n"),
        ("app/Big.php", &big),
    ];
    cx.same(
        &dir,
        &tree,
        &[],
        &["--test", "-v", "--format=txt"],
        &[],
        b"",
    );
}

fn pun(dir: &Path, args: &[&str]) -> (String, String, Option<i32>) {
    pun_env(dir, args, &[])
}

fn pun_env(dir: &Path, args: &[&str], env: &[(&str, &str)]) -> (String, String, Option<i32>) {
    let out = common::clean_env(&mut Command::new(PUN), &["PATH"])
        .args(args)
        .current_dir(dir)
        .env("COLUMNS", "80")
        .env("PUN_AGENT", "0")
        .envs(env.iter().copied())
        .output()
        .unwrap();
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code(),
    )
}

/// `pun list-files` (context/plan/config.md §7), expected sets derived by hand from the plan's example.
#[test]
fn list_files() {
    let dir = scratch("cli", "list");
    reset(&dir, &[]);
    for f in [
        "app/A.php",
        "app/B.php",
        "tests/T.php",
        "storage/S.php",
        "vendor/v/V.php",
    ] {
        std::fs::create_dir_all(dir.join(f).parent().unwrap()).unwrap();
        std::fs::write(dir.join(f), "<?php\n").unwrap();
    }
    std::fs::write(
        dir.join("pun.toml"),
        "[files]\npaths = [\"app\", \"tests\"]\nexclude = [\"storage/**\"]\n\n[analyse]\nexclude = [\"tests/**\"]\n",
    )
    .unwrap();
    let ok = |s: &str| (s.to_string(), String::new(), Some(0));
    assert_eq!(
        pun(&dir, &["list-files"]),
        ok("app/A.php\napp/B.php\ntests/T.php\n")
    );
    assert_eq!(
        pun(&dir, &["list-files", "--command", "analyse"]),
        ok("app/A.php\napp/B.php\n")
    );
    assert_eq!(
        pun(&dir, &["list-files", "-0", "tests"]),
        ok("tests/T.php\0")
    );
    assert_eq!(
        pun(
            &dir,
            &["list-files", "--zero-terminate", "--command=lint", "app"]
        ),
        ok("app/A.php\0app/B.php\0")
    );
    assert_eq!(
        pun(
            &dir,
            &["list-files", "--format=json", "--command", "analyse"]
        ),
        ok("[\"app/A.php\",\"app/B.php\"]\n")
    );
    // JSON wins over agent mode; agent mode puts a summary line first.
    assert_eq!(
        pun_env(
            &dir,
            &["list-files", "--format", "json", "tests"],
            &[("PUN_AGENT", "1")]
        ),
        ok("[\"tests/T.php\"]\n")
    );
    let (out, _, code) = pun_env(
        &dir,
        &["list-files", "--command", "analyse"],
        &[("PUN_AGENT", "1")],
    );
    let (summary, rest) = out.split_once('\n').unwrap();
    assert!(
        summary.starts_with("pun list-files: 2 files, analyse set ("),
        "{summary}"
    );
    assert_eq!((rest, code), ("app/A.php\napp/B.php\n", Some(0)));
    let (out, _, _) = pun_env(&dir, &["list-files", "tests"], &[("PUN_AGENT", "1")]);
    assert!(out.starts_with("pun list-files: 1 file ("), "{out}");
    assert_eq!(
        pun(&dir, &["list-files", "--format=txt"]),
        (
            String::new(),
            "x --format=txt: expected `json`\n".into(),
            Some(2)
        )
    );
    assert_eq!(
        pun(&dir, &["list-files", "nope"]),
        (
            String::new(),
            "x The path \"nope\" is not readable.\n".into(),
            Some(2)
        )
    );
    // The hidden `pun format --list-files` flag is gone.
    assert_eq!(pun(&dir, &["format", "--list-files"]).2, Some(2));
}

/// Pun-only flags, hand-derived.
#[test]
fn pun_only_behaviour() {
    let dir = scratch("cli", "own");
    reset(&dir, &files());
    // Every command needs pun.toml; `--preset` or `--no-config` runs without one.
    for cmd in [
        &["format"][..],
        &["lint"],
        &["check", "--fix"],
        &["config"],
        &["list-files"],
    ] {
        let (out, err, code) = pun(&dir, cmd);
        assert_eq!(
            (out.as_str(), err.as_str(), code),
            (
                "",
                "x No pun.toml found. Run `pun migrate`, or `pun init` for a new project.\n",
                Some(2)
            ),
            "{cmd:?}"
        );
    }
    let (_, _, code) = pun(&dir, &["format", "--test", "--preset=empty"]);
    assert_eq!(code, Some(0));
    let (_, _, code) = pun(&dir, &["format", "--test", "--no-config"]);
    assert_eq!(code, Some(1));
    migrate(&dir);
    // `pun list-files --command format`: Pint's finder (the symlinked file counts), sorted.
    let (out, _, code) = pun(&dir, &["list-files", "--command", "format"]);
    let listed: Vec<&str> = out.lines().collect();
    assert_eq!(
        listed,
        [
            "app/Clean.php",
            "app/Empty.php",
            "app/Fix.php",
            "app/Link.php",
            LONG,
            "app/with spaçe.php"
        ]
    );
    assert_eq!(code, Some(0));

    let (_, err, code) = pun(&dir, &["format", "--test", "--explain"]);
    assert_eq!(code, Some(1));
    let lines: Vec<&str> = err.lines().collect();
    assert_eq!(lines[0], "native (PHP 8.4), preset empty, 4 rules");
    assert_eq!(lines.len(), 5);
    assert!(
        lines[1..].iter().all(|l| l.starts_with("  native ")),
        "{err}"
    );
    for rule in [
        "array_syntax",
        "no_trailing_whitespace",
        "trim_array_spaces",
        "whitespace_after_comma_in_array",
    ] {
        assert!(
            lines.iter().any(|l| l.ends_with(&format!("  {rule}"))),
            "{err}"
        );
    }

    // -v alone prints nothing on stderr.
    let (_, err, _) = pun(&dir, &["format", "--test", "-v", "app/Clean.php"]);
    assert_eq!(err, "");

    // No PHP fallback: a rule Pun hasn't ported stops every command, named under its Pun ID (exit 2).
    std::fs::write(
        dir.join("pun.toml"),
        "[format]\npreset = \"empty\"\n[format.rules]\n\"arrays/syntax\" = true\n\"strings/explicit-variable\" = true\n",
    )
    .unwrap();
    let toml = dir.join("pun.toml");
    for cmd in [&["format", "--test"][..], &["lint"], &["check", "--fix"]] {
        let (out, err, code) = pun(&dir, cmd);
        assert_eq!(
            (out.as_str(), err, code),
            (
                "",
                "x pun.toml  rule not supported by Pun yet: `format/strings/explicit-variable`\n  remove it from pun.toml to run the other rules\n".to_string(),
                Some(2)
            ),
            "{cmd:?}"
        );
    }
    // Lint rules count too, even for `pun format`; several are listed together.
    std::fs::write(
        dir.join("pun.toml"),
        "[format]\npreset = \"empty\"\n[format.rules]\n\"strings/explicit-variable\" = true\n[lint.rules]\n\"security/no-homoglyph-names\" = \"error\"\n",
    )
    .unwrap();
    let (_, err, code) = pun(&dir, &["format", "--test"]);
    assert_eq!(
        (err, code),
        (
            "x pun.toml  rules not supported by Pun yet: `format/strings/explicit-variable`, `lint/security/no-homoglyph-names`\n  remove them from pun.toml to run the other rules\n".to_string(),
            Some(2)
        )
    );
    // Agent mode: the same message as its one-line error, same exit code.
    let (out, _, code) = pun_env(&dir, &["lint"], &[("PUN_AGENT", "1")]);
    assert_eq!(
        (out, code),
        (
            format!(
                "pun lint: error: {}: rules not supported by Pun yet: `format/strings/explicit-variable`, `lint/security/no-homoglyph-names`\n",
                toml.display()
            ),
            Some(2)
        )
    );
    // A named lint rule no fixer runs yet stops the run too; a set skips the rules Pun can't run yet (code-quality
    // has `lint/security/no-homoglyph-names`, whose fixer isn't ported).
    std::fs::write(
        dir.join("pun.toml"),
        "[format]\npreset = \"empty\"\n[lint]\nsets = [\"code-quality\"]\n[lint.rules]\n\"style/prefer-early-return\" = \"error\"\n",
    )
    .unwrap();
    let (_, err, code) = pun(&dir, &["lint"]);
    assert_eq!(
        (err, code),
        (
            "x pun.toml  rule not supported by Pun yet: `lint/style/prefer-early-return`\n  remove it from pun.toml to run the other rules\n".to_string(),
            Some(2)
        )
    );
    std::fs::write(
        dir.join("pun.toml"),
        "[format]\npreset = \"empty\"\n[lint]\nsets = [\"code-quality\"]\n",
    )
    .unwrap();
    let (out, err, code) = pun_env(&dir, &["lint", "app/Clean.php"], &[("PUN_AGENT", "1")]);
    assert_eq!((err.as_str(), code), ("", Some(1)));
    assert!(
        out.ends_with("\napp/Clean.php lint/statements/require-strict-types [fix:unsafe] Force a strict types mode in all files.\n"),
        "{out}"
    );
    // `--preset per|symfony` was removed (format 1): exit 2 with one line naming `empty`, `[format.rules]` and
    // `pun migrate`, through every command that takes `--preset`.
    std::fs::remove_file(dir.join("pun.toml")).unwrap();
    for args in [
        &["format", "--test", "--preset=per"][..],
        &["check", "--preset=symfony"],
        &["lint", "--preset=per"],
        &["config", "--preset=symfony"],
    ] {
        let p = args.last().unwrap().strip_prefix("--preset=").unwrap();
        let (out, err, code) = pun(&dir, args);
        assert_eq!(
            (out.as_str(), err, code),
            (
                "",
                format!(
                    "x --preset {p} was removed: use --preset empty with its rules under [format.rules] in pun.toml (`pun migrate --preset {p}` expands it)\n"
                ),
                Some(2)
            ),
            "{args:?}"
        );
    }
    // pun.toml's `[format] preset = "symfony"` is the same removed preset: exit 2.
    std::fs::write(dir.join("pun.toml"), "[format]\npreset = \"symfony\"\n").unwrap();
    let (_, err, code) = pun(&dir, &["format", "--test"]);
    assert_eq!(code, Some(2), "{err}");

    // A broken pun.toml is a one-line error.
    std::fs::write(dir.join("pun.toml"), "[format]\nnope = 1\n").unwrap();
    let (out, err, code) = pun(&dir, &["format", "--test", "app/Clean.php"]);
    assert_eq!((out.as_str(), code), ("", Some(2)));
    assert!(
        err.starts_with(&format!("x invalid {}: ", dir.join("pun.toml").display())),
        "{err}"
    );

    reset(&dir, &files());
    migrate(&dir);

    // --staged: only files with staged changes (none outside git).
    let (out, _, code) = pun(&dir, &["format", "--staged", "--test", "--format=json"]);
    assert_eq!(code, Some(0));
    assert!(out.contains(r#""files":[]"#), "{out}");

    // config: the resolved config under Pun rule IDs, TOML by default, JSON on request.
    let (out, _, code) = pun(&dir, &["config", "--show", "--format=json"]);
    assert_eq!(code, Some(0));
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["format"]["preset"], "empty");
    assert_eq!(v["format"]["rules"]["arrays/syntax"], true);
    let (out, _, _) = pun(&dir, &["config", "--preset=psr12", "--config=pun.toml"]);
    let t: toml::Value = toml::from_str(&out).unwrap();
    assert_eq!(t["format"]["preset"].as_str(), Some("psr12"));
    let (_, err, code) = pun(&dir, &["config", "--preset=nope"]);
    assert_eq!((err.as_str(), code), ("x Preset not found.\n", Some(2)));
}

/// `pun migrate` reads a .php-cs-fixer.php and its Finder through PHP-CS-Fixer itself: a project fixer class, which
/// Pun can't run, is named and skipped, then one without custom fixers.
#[test]
fn migrates_php_cs_fixer_config() {
    let Some(oracle) = oracle() else { return };
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/php-cs-fixer-custom");
    let dir = scratch("cli", "pcf");
    let read = |f: &str| std::fs::read_to_string(src.join(f)).unwrap();
    let real = oracle
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("php-cs-fixer/vendor/autoload.php");
    let autoload = format!("<?php\nreturn require '{}';\n", real.display());
    let tree: Vec<(&str, String)> = vec![
        ("composer.json", r#"{"require":{"php":"^8.4"}}"#.into()),
        (".php-cs-fixer.php", read(".php-cs-fixer.php")),
        ("UpperTodoFixer.php", read("UpperTodoFixer.php")),
        ("app/A.php", "<?php\n$a = \"x\";\n$b = sizeof([]);\n".into()),
        ("vendor/autoload.php", autoload),
        ("vendor/friendsofphp/php-cs-fixer/.keep", String::new()),
    ];
    let tree: Vec<(&str, &str)> = tree.iter().map(|(a, b)| (*a, b.as_str())).collect();
    // A config PHP-CS-Fixer can't load: its error, nothing written.
    reset(&dir, &tree);
    std::fs::write(
        dir.join(".php-cs-fixer.php"),
        "<?php\nthrow new Exception('bad config');\n",
    )
    .unwrap();
    let (_, err, code) = pun(&dir, &["migrate"]);
    assert_eq!(code, Some(2));
    assert!(err.contains("bad config"), "{err}");
    assert!(!dir.join("pun.toml").exists());
    // Without PHP-CS-Fixer installed the config can't be read.
    reset(&dir, &tree);
    std::fs::remove_dir_all(dir.join("vendor/friendsofphp")).unwrap();
    let (_, err, code) = pun(&dir, &["migrate"]);
    assert_eq!(
        (err.as_str(), code),
        (
            "x no vendor/friendsofphp/php-cs-fixer: run `composer install` so Pun can read the config\n",
            Some(2)
        )
    );
    // The custom fixer is named and skipped; the rest migrates, the Finder's `in` to [files] paths.
    reset(&dir, &tree);
    let skipped = "pun migrate: .php-cs-fixer.php: Acme/upper_todo: not a PHP-CS-Fixer or Pint rule Pun knows (a custom fixer or rule set); skipped\n";
    let (out, err, code) = pun(&dir, &["migrate"]);
    assert_eq!(
        (out.as_str(), err.as_str(), code),
        (
            "pun migrate: migrating everything detected (pick with --from): .php-cs-fixer.php\npun migrate: pun.toml written from .php-cs-fixer.php, ignore comments rewritten in 0 file(s).\n  .php-cs-fixer.php  migrated\n",
            skipped,
            Some(0)
        )
    );
    let toml = std::fs::read_to_string(dir.join("pun.toml")).unwrap();
    assert!(
        toml.starts_with("# Written by `pun migrate` from .php-cs-fixer.php.\n[files]\npaths = [\"app\"]\n\n[format]\npreset = \"empty\"\n\n[format.rules]\n")
            && toml.contains("\n\"strings/single-quote\" = true\n"),
        "{toml}"
    );
    // Agent mode: the skipped rule as a finding under one line.
    reset(&dir, &tree);
    let (out, _, code) = pun_env(&dir, &["migrate"], &[("PUN_AGENT", "1")]);
    let (first, rest) = out.split_once('\n').unwrap();
    assert_eq!(code, Some(0));
    assert!(
        first.starts_with("pun migrate: ok, pun.toml written (preset empty, "),
        "{first}"
    );
    assert_eq!(
        rest,
        ".php-cs-fixer.php migrated\n.php-cs-fixer.php unsupported Acme/upper_todo: not a PHP-CS-Fixer or Pint rule Pun knows (a custom fixer or rule set)\n"
    );
    // A Finder over the project root: no [files] paths; exclude and notName go to [format]; depth() and
    // ignoreVCSIgnored() have no key.
    reset(&dir, &tree);
    std::fs::write(
        dir.join(".php-cs-fixer.php"),
        "<?php\nreturn (new PhpCsFixer\\Config())->setRiskyAllowed(true)->setRules(['single_quote' => true, 'no_alias_functions' => true])\n    ->setFinder(PhpCsFixer\\Finder::create()->in(__DIR__)->exclude('legacy')->notName('*.inc')->depth('< 3')->ignoreVCSIgnored(true));\n",
    )
    .unwrap();
    let (out, err, code) = pun(&dir, &["migrate"]);
    assert_eq!(code, Some(0), "{err}");
    assert_eq!(
        err,
        "pun migrate: .php-cs-fixer.php: Finder depth(): no pun.toml key for it; skipped\npun migrate: .php-cs-fixer.php: Finder ignoreVCSIgnored(): no pun.toml key for it; skipped\n"
    );
    assert_eq!(
        out,
        "pun migrate: migrating everything detected (pick with --from): .php-cs-fixer.php\npun migrate: pun.toml written from .php-cs-fixer.php, ignore comments rewritten in 0 file(s).\n  .php-cs-fixer.php  migrated\n"
    );
    assert_eq!(
        std::fs::read_to_string(dir.join("pun.toml")).unwrap(),
        "# Written by `pun migrate` from .php-cs-fixer.php.\n[format]\npreset = \"empty\"\nexclude = [\"legacy\"]\nnot-name = [\"*.inc\"]\n\n[format.rules]\n\"strings/single-quote\" = true\n\n[lint.rules]\n\"style/no-alias-function\" = \"error\"\n"
    );
    // single_quote is format; no_alias_functions (risky) only runs under lint / check.
    pun(&dir, &["format"]);
    let a = || std::fs::read_to_string(dir.join("app/A.php")).unwrap();
    assert_eq!(a(), "<?php\n$a = 'x';\n$b = sizeof([]);\n");
    pun(&dir, &["check", "--fix"]);
    assert_eq!(a(), "<?php\n$a = 'x';\n$b = count([]);\n");
}

/// `pun config` on a hand-written pun.toml. Expected text derived by hand from config.md §3 and mapping.toml: each
/// command's exclude adds `[files] exclude`; `tags = "warn"` turns on the one lint rule of `tags`; the analyse groups
/// switched off are every group with a `basic` rule, so only the rule named after them runs; `concat-spacing` has one
/// option, so it prints as that option's value (format.md Decisions, "Option names").
#[test]
fn config_prints_resolved_schema() {
    let dir = scratch("cli", "config");
    let toml = r#"[php]
version = "8.5"

[files]
paths = ["app", "tests"]
exclude = ["storage/**"]

[format]
preset = "empty"
exclude = ["app/Generated/**"]
not-name = ["*.blade.php"]
not-path = ["app/Legacy.php"]

[format.rules]
"arrays/syntax" = true
"operators/concat-spacing" = { spacing = "one" }

[lint]
exclude = ["database/**"]

[lint.groups]
tags = "warn"

[lint.rules]
"style/no-alias-function" = { fix = "off" }
"dead-code/no-self-assignment" = "error"

[[lint.overrides]]
files = ["tests/**"]
rules = { "dead-code/no-self-assignment" = "off" }

[analyse]
preset = "basic"
exclude = ["tests/**"]
baseline = "pun-baseline.toml"

[analyse.rules]
arrays = "off"
attributes = "off"
casing = "off"
casts = "off"
classes = "off"
comments = "off"
control-flow = "off"
dead-code = "off"
debug = "off"
deprecated = "off"
functions = "off"
imports = "off"
operators = "off"
phpdoc = "off"
statements = "off"
strings = "off"
types = "off"
undefined = "off"
"undefined/method" = { magic = true }
"#;
    std::fs::write(dir.join("pun.toml"), toml).unwrap();
    let want = r#"# Resolved from pun.toml.

[php]
version = "8.5"

[files]
paths = ["app", "tests"]
exclude = ["storage/**"]

[format]
preset = "empty"
exclude = ["storage/**", "app/Generated/**"]
not-name = ["*.blade.php"]
not-path = ["app/Legacy.php"]

[format.rules]
"arrays/syntax" = true
"operators/concat-spacing" = "one"

[lint]
sets = []
exclude = ["storage/**", "database/**"]
ecosystems = []

[lint.rules]
"dead-code/no-self-assignment" = "error"
"tags/no-invalid-open-tag" = "warn"

[lint.rules."style/no-alias-function"]
fix = "off"

[[lint.overrides]]
files = ["tests/**"]

[lint.overrides.rules]
"dead-code/no-self-assignment" = "off"

[analyse]
preset = "basic"
paths = ["app", "tests"]
exclude = ["storage/**", "tests/**"]
switches = []
ecosystems = []
baseline = "pun-baseline.toml"

[analyse.rules."undefined/method"]
magic = true
"#;
    assert_eq!(
        pun(&dir, &["config"]),
        (want.into(), String::new(), Some(0))
    );
    // Nothing read (a preset, no pun.toml or composer.json): no comment, and PHP 8.4.
    let (out, _, code) = pun(
        &scratch("cli", "config-none"),
        &["config", "--preset=empty"],
    );
    assert!(out.starts_with("[php]\nversion = \"8.4\"\n"), "{out}");
    assert_eq!(code, Some(0));
}

/// `pun explain`: a lint rule by ID, the same rule by its PHP-CS-Fixer name (named first), and a name that is no rule.
/// Expected text from mapping.toml's `style/no-alias-function` row, its upstream rows and `[meta.groups] style`.
#[test]
fn explain_rule() {
    let dir = scratch("cli", "explain");
    let text = "lint/style/no-alias-function
Master functions shall be used instead of aliases.

group     style: One consistent way to write the same thing
sets      coding-style
fix       unsafe
upstream  no_alias_functions (php-cs-fixer)
          no-alias-function (mago-linter)
docs      https://github.com/punphp/pun/blob/main/crates/pun-config/rules/mapping.toml#L";
    let (out, err, code) = pun(&dir, &["explain", "lint/style/no-alias-function"]);
    assert!(out.starts_with(text), "{out}");
    assert_eq!((err.as_str(), code), ("", Some(0)));
    let (out, _, code) = pun(&dir, &["explain", "no_alias_functions"]);
    let note = "`no_alias_functions` is `lint/style/no-alias-function` in Pun.\n\n";
    assert!(out.starts_with(&format!("{note}{text}")), "{out}");
    assert_eq!(code, Some(0));
    assert_eq!(
        pun(&dir, &["explain", "nope"]),
        (
            String::new(),
            "x `nope` is not a Pun rule or an upstream rule name (see crates/pun-config/rules/mapping.toml)\n".into(),
            Some(2)
        )
    );
}

/// `pun lint --mago` names the file it can't read (hand-derived: Pun's own wording for a read error).
#[cfg(feature = "mago-lint")]
#[test]
fn mago_lint_read_error_names_the_file() {
    let dir = scratch("cli", "mago-unreadable");
    reset(
        &dir,
        &[
            ("pun.toml", "[format]\npreset = \"empty\"\n"),
            ("a.php", "<?php\necho 1;\n"),
        ],
    );
    let locked = dir.join("a.php");
    std::fs::set_permissions(&locked, std::os::unix::fs::PermissionsExt::from_mode(0o000)).unwrap();
    // Root reads a mode-000 file: there is no read error to report.
    if std::fs::read(&locked).is_err() {
        let (_, err, code) = pun(&dir, &["lint", "--mago"]);
        assert_eq!(
            (err, code),
            (
                format!(
                    "x can't read {}: Permission denied (os error 13)\n",
                    locked.display()
                ),
                Some(2)
            )
        );
    }
}
