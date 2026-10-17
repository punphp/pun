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
        .env("PUN_AGENT", "0")
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
        .env("PUN_AGENT", "0")
        .current_dir(dir)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
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
    // A coding agent in the environment switches Pint to its JSON agent reporter; Pun prints its own agent format
    // there (tests/agent.rs), and that reporter only with --format=agent (above).
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
        let (got, _) = pun_format(&dir, args, env, b"");
        // Progress symbols of the files before the aborting one, then the box.
        assert!(want.stdout.starts_with("\n  "), "{}", want.stdout);
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
                format!(
                    "pun: {}: rule not supported by Pun yet: `format/strings/explicit-variable`\n",
                    toml.display()
                ),
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
            format!(
                "pun: {}: rules not supported by Pun yet: `format/strings/explicit-variable`, `lint/security/no-homoglyph-names`\n",
                toml.display()
            ),
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
            format!(
                "pun: {}: rule not supported by Pun yet: `lint/style/prefer-early-return`\n",
                toml.display()
            ),
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
    // Without pun.toml, `--preset` names the preset: per's @PER-CS set has no expansion in Pun.
    std::fs::remove_file(dir.join("pun.toml")).unwrap();
    let (_, err, code) = pun(&dir, &["format", "--test", "--preset=per"]);
    assert_eq!(
        (err.as_str(), code),
        (
            "pun: preset per: rule not supported by Pun yet: `@PER-CS`\n",
            Some(2)
        )
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
    assert_eq!(v["format"]["rules"]["format/arrays/syntax"], true);
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

/// `pun migrate` reads a .php-cs-fixer.php through PHP-CS-Fixer itself (a project fixer class, which Pun can't run,
/// so nothing is written), then one without custom fixers.
#[test]
fn migrates_php_cs_fixer_config() {
    let Some(oracle) = oracle() else { return };
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/php-cs-fixer-custom");
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
    assert_eq!(code, Some(1));
    assert!(err.contains("bad config"), "{err}");
    assert!(!dir.join("pun.toml").exists());
    // Without PHP-CS-Fixer installed the config can't be read.
    reset(&dir, &tree);
    std::fs::remove_dir_all(dir.join("vendor/friendsofphp")).unwrap();
    let (_, err, code) = pun(&dir, &["migrate"]);
    assert_eq!(
        (err.as_str(), code),
        (
            "pun: no vendor/friendsofphp/php-cs-fixer: run `composer install` so Pun can read the config\n",
            Some(1)
        )
    );
    reset(&dir, &tree);
    let finder = "pun migrate: .php-cs-fixer.php configures its own Finder, which is not migrated: check [files] paths and [format] exclude.\n";
    let (out, err, code) = pun(&dir, &["migrate"]);
    assert_eq!(
        (out.as_str(), err, code),
        (
            "pun migrate: migrating everything detected (pick with --from): .php-cs-fixer.php\n",
            format!(
                "{finder}pun migrate: .php-cs-fixer.php uses rules Pun can't run, so nothing was written:\n  Acme/upper_todo: not a PHP-CS-Fixer or Pint rule Pun knows (a custom fixer or rule set)\n"
            ),
            Some(1)
        )
    );
    assert!(!dir.join("pun.toml").exists());
    // Agent mode: the Finder note and the unsupported rule as findings under one line.
    let (out, _, code) = pun_env(&dir, &["migrate"], &[("PUN_AGENT", "1")]);
    let (first, rest) = out.split_once('\n').unwrap();
    let out = format!("{}, Ts)\n{rest}", &first[..first.rfind(", ").unwrap()]);
    assert_eq!(
        (out.as_str(), code),
        (
            "pun migrate: fail, 1 unsupported rule, nothing written (from .php-cs-fixer.php, Ts)\n.php-cs-fixer.php finder-not-migrated configures its own Finder, which is not migrated: check [files] paths and [format] exclude.\n.php-cs-fixer.php unsupported-rule Acme/upper_todo: not a PHP-CS-Fixer or Pint rule Pun knows (a custom fixer or rule set)\n",
            Some(1)
        )
    );
    std::fs::write(
        dir.join(".php-cs-fixer.php"),
        "<?php\nreturn (new PhpCsFixer\\Config())->setRiskyAllowed(true)->setRules(['single_quote' => true, 'no_alias_functions' => true]);\n",
    )
    .unwrap();
    let (out, err, code) = pun(&dir, &["migrate"]);
    assert_eq!(code, Some(0), "{err}");
    assert_eq!(err, finder);
    assert_eq!(
        out,
        "pun migrate: migrating everything detected (pick with --from): .php-cs-fixer.php\npun migrate: pun.toml written from .php-cs-fixer.php, ignore comments rewritten in 0 file(s).\n  .php-cs-fixer.php  migrated\n"
    );
    assert_eq!(
        std::fs::read_to_string(dir.join("pun.toml")).unwrap(),
        "# Written by `pun migrate` from .php-cs-fixer.php.\n[format]\npreset = \"empty\"\n\n[format.rules]\n\"strings/single-quote\" = true\n\n[lint.rules]\n\"style/no-alias-function\" = \"error\"\n"
    );
    // single_quote is format; no_alias_functions (risky) only runs under lint / check.
    pun(&dir, &["format"]);
    let a = || std::fs::read_to_string(dir.join("app/A.php")).unwrap();
    assert_eq!(a(), "<?php\n$a = 'x';\n$b = sizeof([]);\n");
    pun(&dir, &["check", "--fix"]);
    assert_eq!(a(), "<?php\n$a = 'x';\n$b = count([]);\n");
}
