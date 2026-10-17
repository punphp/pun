//! `pun migrate` + `pun format` against the real Pint (`php .oracle/pint/builds/pint`) on the same pint.json: same
//! stdout, exit code and resulting files for every flag combination below, run in the same directory (so diff headers
//! and absolute paths agree). Rules: only fully ported ones, so the output must be byte-identical. Skipped without the
//! oracle. Pun-only behaviour (`--explain`, `config`, unported rules, no pun.toml) is checked by hand-derived
//! expectations at the end.
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

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

fn oracle() -> Option<PathBuf> {
    let p = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.oracle/pint/builds/pint");
    p.exists().then_some(p)
}

fn tmp(name: &str) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let d = std::env::temp_dir().join(format!(
        "pun-cli-{name}-{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d.canonicalize().unwrap()
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
    for (f, c) in files {
        let p = dir.join(f);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, c).unwrap();
    }
    if dir.join("app/Fix.php").exists() {
        std::os::unix::fs::symlink("Fix.php", dir.join("app/Link.php")).unwrap();
    }
}

fn snapshot(dir: &Path, rel: &str, out: &mut BTreeMap<String, Vec<u8>>) {
    for e in std::fs::read_dir(dir.join(rel)).unwrap().flatten() {
        let r = if rel.is_empty() {
            e.file_name().to_string_lossy().into_owned()
        } else {
            format!("{rel}/{}", e.file_name().to_string_lossy())
        };
        // pun.toml: what `pun migrate` wrote for Pun's run.
        if r == ".git" || r == "pun.toml" || e.path().is_symlink() {
            continue;
        }
        if e.path().is_dir() {
            snapshot(dir, &r, out)
        } else {
            out.insert(
                r,
                std::fs::read(e.path()).unwrap_or_else(|_| b"<unreadable>".to_vec()),
            );
        }
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
    c.args(pre).args(args).current_dir(dir).env_clear();
    // Keep LLVM_PROFILE_FILE so coverage runs don't drop .profraw files into the project.
    for k in ["PATH", "HOME", "LLVM_PROFILE_FILE"] {
        if let Ok(v) = std::env::var(k) {
            c.env(k, v);
        }
    }
    // Pint names its stdin temp file with uniqid(): concurrent runs sharing /tmp can swap inputs.
    let tmp = dir.with_extension("tmp");
    std::fs::create_dir_all(&tmp).unwrap();
    c.env("COLUMNS", "80")
        .env("TMPDIR", &tmp)
        .envs(env.iter().copied())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = c.spawn().unwrap();
    child.stdin.take().unwrap().write_all(stdin).unwrap();
    let out = child.wait_with_output().unwrap();
    let mut files = BTreeMap::new();
    snapshot(dir, "", &mut files);
    (
        Run {
            stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
            code: out.status.code(),
            files,
        },
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

const PUN: &str = env!("CARGO_BIN_EXE_pun");

/// `pun migrate` in `dir`, which must succeed: Pun's runs read the pun.toml it writes from Pint's pint.json.
fn migrate(dir: &Path) {
    let out = Command::new(PUN)
        .arg("migrate")
        .current_dir(dir)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// `pun format` on `dir` after `pun migrate`.
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
    #[allow(clippy::too_many_arguments)]
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
        if args.contains(&"--parallel") {
            // Pint prints progress in worker completion order; only the multiset is stable.
            for r in [&mut want, &mut got] {
                r.stdout = r
                    .stdout
                    .split('\n')
                    .map(|l| {
                        if l.starts_with("  ")
                            && l.trim().chars().all(|c| ".⨯✓!".contains(c))
                            && !l.trim().is_empty()
                        {
                            let mut c: Vec<char> = l.chars().collect();
                            c.sort();
                            c.into_iter().collect()
                        } else {
                            l.to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
            }
        }
        assert_eq!(got, want, "{pre:?} {args:?} {env:?} (our stderr: {err})");
    }
    /// Both fail with exit 1 and the same Symfony error box on stdout.
    fn same_error(&self, dir: &Path, tree: &[(&str, &str)], args: &[&str]) {
        reset(dir, tree);
        let (want, werr) = self.pint(dir, args, &[], b"");
        reset(dir, tree);
        let (got, err) = pun_format(dir, args, &[], b"");
        assert_eq!(want.code, Some(1), "{args:?}: {werr}");
        assert_eq!(got, want, "{args:?}: our stderr {err}");
    }
}

#[test]
fn matches_pint() {
    let Some(oracle) = oracle() else { return };
    let cx = Ctx { oracle };
    let dir = tmp("p");
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
    // A coding agent in the environment switches Pint to its agent reporter.
    cx.same(&dir, &tree, &[], &["--test"], &[("AI_AGENT", "x")], b"");
    cx.same(&dir, &tree, &[], &[], &[("CLAUDECODE", "1")], b"");
    // Narrow terminals: progress wraps, descriptions are truncated (unless -v).
    cx.same(&dir, &tree, &[], &["--test"], &[("COLUMNS", "40")], b"");
    cx.same(
        &dir,
        &tree,
        &[],
        &["--test", "-v"],
        &[("COLUMNS", "40")],
        b"",
    );
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
    cx.same_error(&dir, &tree, &["nope"]);
    let mut bad_in = tree.clone();
    bad_in[1] = (
        "pint.json",
        r#"{"preset":"empty","rules":{"array_syntax":true},"in":["nodir"]}"#,
    );
    cx.same_error(&dir, &bad_in, &[]);
    // Unreadable / unwritable files: same box (see output.rs error_boxes_like_pint for the byte-exact cases).
    for (mode, args, env) in [
        (0o444, &[][..], &[][..]),
        (0o000, &["--test"], &[]),
        (0o000, &["--test"], &[("AI_AGENT", "x")]),
    ] {
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
        let (got, _) = pun_format(&dir, args, env, b"");
        // Progress symbols of the files before the aborting one (none under an agent), then the box.
        assert_eq!(
            want.stdout.starts_with("\n  "),
            env.is_empty(),
            "{}",
            want.stdout
        );
        assert_eq!((got.code, got.stdout), (want.code, want.stdout));
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

/// Pint/laravel_blade (needs prettier in crates/pun-fixers/node_modules, resolved from the project dir):
/// - embedded PHP is formatted with the configured preset's rules: under preset empty `@if(!$a)` keeps `!$a`
///   (laravel would add a space);
/// - a whitespace-only Blade file becomes empty and the later fixers are skipped (AbstractFixer::fix's
///   `0 < $tokens->count()` guard) instead of running on an empty token list.
#[test]
fn blade_like_pint() {
    let Some(oracle) = oracle() else { return };
    let modules = Path::new(env!("CARGO_MANIFEST_DIR")).join("../pun-fixers/node_modules");
    let dir = tmp("blade");
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

/// `-v` diffs pair up lines exactly like PHP-CS-Fixer's sebastian/diff (LCS + strict unified builder): reordered
/// imports (fixtures/dirty_imports.php, from pinkary-dirty: a Myers diff pairs other lines), a missing final newline, mixed line endings (warning line), and
/// a file big enough for the memory-efficient LCS (> 853 differing lines).
#[test]
fn diffs_like_pint() {
    let Some(oracle) = oracle() else { return };
    let cx = Ctx { oracle };
    let dir = tmp("diff");
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
    cx.same(&dir, &tree, &[], &["--test", "-v"], &[], b"");
}

fn pun(dir: &Path, args: &[&str]) -> (String, String, Option<i32>) {
    pun_env(dir, args, &[])
}

fn pun_env(dir: &Path, args: &[&str], env: &[(&str, &str)]) -> (String, String, Option<i32>) {
    let out = Command::new(env!("CARGO_BIN_EXE_pun"))
        .args(args)
        .current_dir(dir)
        .env_clear()
        .envs(std::env::vars().filter(|(k, _)| k == "PATH" || k == "LLVM_PROFILE_FILE"))
        .env("COLUMNS", "80")
        .envs(env.iter().copied())
        .output()
        .unwrap();
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code(),
    )
}

/// Pun-only flags, hand-derived.
#[test]
fn pun_only_behaviour() {
    let dir = tmp("own");
    reset(&dir, &files());
    // Every command needs pun.toml; `--preset` or `--no-config` runs without one.
    for cmd in [&["format"][..], &["lint"], &["check", "--fix"], &["config"]] {
        let (out, err, code) = pun(&dir, cmd);
        assert_eq!(
            (out.as_str(), err.as_str(), code),
            ("", "pun: No pun.toml found. Run `pun migrate`.\n", Some(1)),
            "{cmd:?}"
        );
    }
    let (_, _, code) = pun(&dir, &["format", "--test", "--preset=empty"]);
    assert_eq!(code, Some(0));
    let (_, _, code) = pun(&dir, &["format", "--test", "--no-config"]);
    assert_eq!(code, Some(1));
    migrate(&dir);
    let (out, _, code) = pun(&dir, &["format", "--list-files"]);
    let mut listed: Vec<&str> = out.lines().collect();
    listed.sort();
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

    // -v alone explains only delegation; a native run prints nothing extra.
    let (_, err, _) = pun(&dir, &["format", "--test", "-v", "app/Clean.php"]);
    assert_eq!(err, "");

    // A rule Pun hasn't ported is delegated to PHP-CS-Fixer, which needs one to delegate to.
    std::fs::write(
        dir.join("pun.toml"),
        "[format]\npreset = \"empty\"\n[format.rules]\narray-syntax = true\nassign-null-coalescing-to-coalesce-equal = true\n",
    )
    .unwrap();
    let (out, _, code) = pun(&dir, &["format", "--test", "app/Clean.php"]);
    assert_eq!(code, Some(1));
    assert!(
        out.contains("no PHP-CS-Fixer to delegate to: run `composer require --dev laravel/pint`,"),
        "{out}"
    );
    if let Some(o) = oracle() {
        let (out, _, code) = pun_env(
            &dir,
            &["format", "--test", "app/Clean.php"],
            &[("PUN_PINT", o.to_str().unwrap())],
        );
        assert_eq!(code, Some(0), "{out}");
    }
    // --native-only: native rules only, the rest listed (and the run is not failed for it).
    let (out, err, code) = pun(
        &dir,
        &[
            "format",
            "--test",
            "--explain",
            "--native-only",
            "app/Clean.php",
        ],
    );
    assert_eq!(code, Some(0));
    assert!(out.contains("PASS"), "{out}");
    assert!(
        err.contains("  skipped           assign_null_coalescing_to_coalesce_equal\n"),
        "{err}"
    );
    assert!(
        err.ends_with("pun: --native-only skipped 1 rules that are not ported: assign_null_coalescing_to_coalesce_equal\n"),
        "{err}"
    );

    // A broken pun.toml is a one-line error.
    std::fs::write(dir.join("pun.toml"), "[format]\nnope = 1\n").unwrap();
    let (out, err, code) = pun(&dir, &["format", "--test", "app/Clean.php"]);
    assert_eq!((out.as_str(), code), ("", Some(1)));
    assert!(
        err.starts_with(&format!(
            "pun: invalid {}: ",
            dir.join("pun.toml").display()
        )),
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
    assert_eq!(v["format"]["rules"]["format.array-syntax"], true);
    assert_eq!(
        v["sources"][1],
        dir.join("pun.toml").to_string_lossy().as_ref()
    );
    let (out, _, _) = pun(&dir, &["config", "--preset=psr12", "--config=pun.toml"]);
    let t: toml::Value = toml::from_str(&out).unwrap();
    assert_eq!(t["format"]["preset"].as_str(), Some("psr12"));
    let (_, err, code) = pun(&dir, &["config", "--preset=nope"]);
    assert_eq!((err.as_str(), code), ("pun: Preset not found.\n", Some(1)));
}

/// Delegate mode against the real Pint: rules Pun hasn't ported (combine_consecutive_unsets, align_multiline_comment,
/// interleaved with native ones by priority) send every file through PHP-CS-Fixer, so stdout, exit code and files
/// equal Pint's in every mode, including syntax errors, which stay native.
#[test]
fn delegated_rules_match_pint() {
    let Some(oracle) = oracle() else { return };
    let cx = Ctx {
        oracle: oracle.clone(),
    };
    let dir = tmp("delegate");
    let tree = [
        ("composer.json", r#"{"require":{"php":"^8.4"}}"#),
        (
            "pint.json",
            r#"{"preset":"laravel","rules":{"explicit_string_variable":true,"no_null_property_initialization":true}}"#,
        ),
        (
            "app/Strings.php",
            "<?php\n$s = \"a $b\";\n$c = array(\"x\");\n",
        ),
        ("app/Props.php", "<?php\n\nclass A { public $x = null; }\n"),
        ("app/Clean.php", "<?php\n\n$a = [1];\n"),
        ("app/Fix.php", "<?php\n$a = array(1,2);   \n"),
        ("app/Bad.php", "<?php\n$a = ;\n"),
    ];
    let env = [("PUN_PINT", oracle.to_str().unwrap())];
    for args in [
        &[][..],
        &["--test"],
        &["-v", "--test"],
        &["--format=json"],
        &["--test", "app/Strings.php"],
    ] {
        cx.same(&dir, &tree, &[], args, &env, b"");
    }
    // stdin goes through the same workers.
    reset(&dir, &tree);
    migrate(&dir);
    let (r, _) = run(
        Path::new(env!("CARGO_BIN_EXE_pun")),
        &["format", "--stdin"],
        &dir,
        &[],
        &env,
        b"<?php\n$s = \"a $b\";\n",
    );
    assert_eq!(r.stdout, "<?php\n\n$s = \"a {$b}\";\n");
    // PHP startup warnings (here an ini loading a missing extension) must not reach the worker protocol on stdout.
    let ini = tmp("delegate-ini");
    std::fs::write(
        ini.join("broken.ini"),
        "extension=pun_missing\ndisplay_startup_errors=1\ndisplay_errors=1\n",
    )
    .unwrap();
    let scan = format!(":{}", ini.display());
    let args = ["format", "--test", "app/Strings.php"];
    let clean = pun_env(&dir, &args, &env);
    let warned = pun_env(&dir, &args, &[env[0], ("PHP_INI_SCAN_DIR", &scan)]);
    assert_eq!((warned.0, warned.2), (clean.0, clean.2));
    // -v and --explain say why and list what ran in PHP.
    reset(&dir, &tree);
    migrate(&dir);
    let (_, err, _) = pun_env(&dir, &["format", "--test", "-v", "app/Clean.php"], &env);
    let why = format!(
        "delegate: 2 of {} rules are not ported (explicit_string_variable, no_null_property_initialization), so every file runs through PHP-CS-Fixer in `php` ({})\n",
        pun_config_rules(&dir),
        oracle.display()
    );
    assert_eq!(err, why);
    let (_, err, _) = pun_env(
        &dir,
        &["format", "--test", "--explain", "app/Clean.php"],
        &env,
    );
    assert!(
        err.starts_with(&why)
            && err.contains("\n  delegated      6  explicit_string_variable\n")
            && err.contains("\n  delegated    -20  no_extra_blank_lines\n"),
        "{err}"
    );
}

/// The number of format rules `pun config` resolves for `dir` (the laravel preset's, minus its risky ones, plus two).
fn pun_config_rules(dir: &Path) -> usize {
    let (out, _, _) = pun(dir, &["config", "--show", "--format=json"]);
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    v["format"]["rules"].as_object().unwrap().len()
}

/// `pun migrate` reads a .php-cs-fixer.php through PHP-CS-Fixer itself (pun-delegate's fixture: a project fixer
/// class, which Pun can't run, so nothing is written), then one without custom fixers.
#[test]
fn migrates_php_cs_fixer_config() {
    let Some(oracle) = oracle() else { return };
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../pun-delegate/tests/custom");
    let dir = tmp("pcf");
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
        (
            "app/A.php",
            "<?php\n$a = \"x\";\nin_array($a, []);\n".into(),
        ),
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
    assert_eq!(code, Some(1));
    assert!(err.contains("bad config"), "{err}");
    assert!(!dir.join("pun.toml").exists());
    reset(&dir, &tree);
    let finder = "pun migrate: .php-cs-fixer.php configures its own Finder, which is not migrated: check [format] paths and exclude.\n";
    let (out, err, code) = pun(&dir, &["migrate"]);
    assert_eq!(
        (out.as_str(), err, code),
        (
            "",
            format!(
                "{finder}pun migrate: .php-cs-fixer.php uses rules Pun can't run, so nothing was written:\n  Acme/upper_todo: not a PHP-CS-Fixer or Pint rule Pun knows (a custom fixer or rule set)\n"
            ),
            Some(1)
        )
    );
    assert!(!dir.join("pun.toml").exists());
    std::fs::write(
        dir.join(".php-cs-fixer.php"),
        "<?php\nreturn (new PhpCsFixer\\Config())->setRiskyAllowed(true)->setRules(['single_quote' => true, 'strict_param' => true]);\n",
    )
    .unwrap();
    let (out, err, code) = pun(&dir, &["migrate"]);
    assert_eq!(code, Some(0), "{err}");
    assert_eq!(err, finder);
    assert_eq!(
        out,
        "pun migrate: pun.toml written from .php-cs-fixer.php, ignore comments rewritten in 0 file(s).\n"
    );
    assert_eq!(
        std::fs::read_to_string(dir.join("pun.toml")).unwrap(),
        "# Written by `pun migrate` from .php-cs-fixer.php.\n[format]\npreset = \"empty\"\n\n[format.rules]\nsingle-quote = true\n\n[lint.rules]\nrequire-strict-param = { fix = \"apply\" }\n"
    );
    // single_quote is format; strict_param (risky) only runs under lint / check.
    pun(&dir, &["format"]);
    let a = || std::fs::read_to_string(dir.join("app/A.php")).unwrap();
    assert_eq!(a(), "<?php\n$a = 'x';\nin_array($a, []);\n");
    pun(&dir, &["check", "--fix"]);
    assert_eq!(a(), "<?php\n$a = 'x';\nin_array($a, [], true);\n");

    // A delegated fixer (explicit_string_variable isn't ported) that throws or breaks the code fails only its file,
    // with PHP-CS-Fixer's messages. The project's autoloader shadows the real class with UpperTodoFixer's hooks.
    let fake = read("UpperTodoFixer.php")
        .replacen("<?php", "<?php\nnamespace PhpCsFixer\\Fixer\\StringNotation;\nuse PhpCsFixer;\nuse RuntimeException;\nuse SplFileInfo;", 1)
        .replace("final class UpperTodoFixer", "final class ExplicitStringVariableFixer")
        .replace("'Acme/upper_todo'", "'explicit_string_variable'");
    std::fs::write(dir.join("Shadow.php"), fake).unwrap();
    std::fs::write(
        dir.join("vendor/autoload.php"),
        format!(
            "<?php\n$l = require '{}';\nrequire __DIR__.'/../Shadow.php';\nreturn $l;\n",
            real.display()
        ),
    )
    .unwrap();
    std::fs::write(
        dir.join("pun.toml"),
        "[format]\npreset = \"empty\"\n[format.rules]\nexplicit-string-variable = true\n",
    )
    .unwrap();
    std::fs::write(dir.join("app/T.php"), "<?php\n\n// THROW\n").unwrap();
    std::fs::write(dir.join("app/B.php"), "<?php\n\n// BREAK\n$a = 1;\n").unwrap();
    let (out, err, code) = pun(&dir, &["format", "--test", "--format=agent"]);
    assert_eq!(code, Some(1), "{err}");
    assert!(
        out.contains(r#"{"path":"app\/T.php","message":"boom"}"#),
        "{out}"
    );
    assert!(
        out.contains(r#"{"path":"app\/B.php","message":"Parse error: Unmatched ')' on line 4."}"#),
        "{out}"
    );
}
