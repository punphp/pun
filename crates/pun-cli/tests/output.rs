//! Byte-for-byte output parity with the real Pint (`php .oracle/pint/builds/pint`): stdout, stderr, exit code and
//! resulting files for syntax errors (PHP's message, `-v` snippets), colours (`--ansi`, TTY via `script`, NO_COLOR),
//! every `--format` reporter, Termwind truncation, abort boxes and the prettier pre-flight check. Each case runs
//! Pint, resets the tree, runs Pun's `pint` alias in the same directory. Skipped without the oracle.
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

fn oracle() -> Option<PathBuf> {
    let p = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.oracle/pint/builds/pint");
    p.exists().then(|| p.canonicalize().unwrap())
}

fn tmp(name: &str) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let d = std::env::temp_dir().join(format!(
        "pun-out-{name}-{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d.canonicalize().unwrap()
}

/// Recreates `dir` with `files` (fresh directory, so readdir order is the same for both tools).
fn reset(dir: &Path, files: &[(&str, &str)]) {
    let _ = std::fs::remove_dir_all(dir);
    for (f, c) in files {
        let p = dir.join(f);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, c).unwrap();
    }
}

fn snapshot(dir: &Path, rel: &str, out: &mut BTreeMap<String, Vec<u8>>) {
    for e in std::fs::read_dir(dir.join(rel)).unwrap().flatten() {
        let r = format!("{rel}{}", e.file_name().to_string_lossy());
        if e.path().is_symlink() {
            continue;
        }
        if e.path().is_dir() {
            snapshot(dir, &format!("{r}/"), out)
        } else {
            out.insert(r, std::fs::read(e.path()).unwrap());
        }
    }
}

#[derive(Debug, PartialEq)]
struct Run {
    stdout: String,
    stderr: String,
    code: Option<i32>,
    files: BTreeMap<String, Vec<u8>>,
}

#[derive(Clone, Copy)]
struct Opts<'a> {
    env: &'a [(&'a str, &'a str)],
    /// Run under `script` (stdout/stderr/stdin a TTY of this many columns); output arrives merged on stdout.
    tty: Option<u16>,
    /// PATH override (default: the test's PATH).
    path: Option<&'a str>,
}

const PLAIN: Opts = Opts {
    env: &[],
    tty: None,
    path: None,
};

fn run(cmd: &[&str], dir: &Path, o: Opts) -> Run {
    let mut c = match o.tty {
        None => {
            let mut c = Command::new(cmd[0]);
            c.args(&cmd[1..]);
            c
        }
        Some(cols) => {
            let quoted: Vec<String> = cmd
                .iter()
                .map(|a| format!("'{}'", a.replace('\'', "'\\''")))
                .collect();
            let mut c = Command::new("script");
            c.args([
                "-qefc",
                &format!("stty cols {cols} rows 40; {}", quoted.join(" ")),
                "/dev/null",
            ]);
            c
        }
    };
    c.current_dir(dir).env_clear();
    // Keep LLVM_PROFILE_FILE so coverage runs don't drop .profraw files into the project.
    for k in ["HOME", "LLVM_PROFILE_FILE"] {
        if let Ok(v) = std::env::var(k) {
            c.env(k, v);
        }
    }
    c.env(
        "PATH",
        o.path
            .map_or_else(|| std::env::var("PATH").unwrap(), str::to_string),
    );
    if o.tty.is_some() {
        c.env("TERM", "xterm");
    } else {
        c.env("COLUMNS", "80");
    }
    let out = c
        .envs(o.env.iter().copied())
        .stdin(std::process::Stdio::null())
        .output()
        .unwrap();
    let mut files = BTreeMap::new();
    snapshot(dir, "", &mut files);
    let text = |b: &[u8]| String::from_utf8_lossy(b).into_owned();
    Run {
        stdout: text(&out.stdout),
        stderr: text(&out.stderr),
        code: out.status.code(),
        files,
    }
}

struct Cx {
    oracle: PathBuf,
    alias: PathBuf,
    dir: PathBuf,
}

impl Cx {
    fn new(name: &str) -> Option<Cx> {
        let oracle = oracle()?;
        let bin = tmp("bin");
        let alias = bin.join("pint");
        std::os::unix::fs::symlink(env!("CARGO_BIN_EXE_pun"), &alias).unwrap();
        Some(Cx {
            oracle,
            alias,
            dir: tmp(name),
        })
    }

    fn pair(&self, tree: &[(&str, &str)], args: &[&str], o: Opts) -> (Run, Run) {
        let php = which("php");
        reset(&self.dir, tree);
        let want = run(
            &[&[php.as_str(), self.oracle.to_str().unwrap()][..], args].concat(),
            &self.dir,
            o,
        );
        reset(&self.dir, tree);
        let got = run(
            &[&[self.alias.to_str().unwrap()][..], args].concat(),
            &self.dir,
            o,
        );
        (want, got)
    }

    /// `same` for `-v` error boxes: in Pint's exception trace the PHAR's absolute path becomes `phar://pint/` and
    /// the PHP call frames under the throw site are dropped (Pun prints only the throw site).
    fn same_trace(&self, tree: &[(&str, &str)], args: &[&str]) -> Run {
        let (mut want, got) = self.pair(tree, args, PLAIN);
        assert!(
            want.stdout.contains("Exception trace:"),
            "{args:?}: {}",
            want.stdout
        );
        want.stdout = normalise_trace(&want.stdout, &self.oracle);
        assert_eq!(got, want, "{args:?}");
        got
    }

    fn same(&self, tree: &[(&str, &str)], args: &[&str], o: Opts) -> Run {
        let (want, got) = self.pair(tree, args, o);
        assert_eq!(got, want, "{args:?} env={:?} tty={:?}", o.env, o.tty);
        got
    }
}

fn normalise_trace(out: &str, oracle: &Path) -> String {
    let frame = |l: &str| l.starts_with(' ') && !l.starts_with("  ") && l.contains(" at ");
    let out = out.replace(&format!("phar://{}/", oracle.display()), "phar://pint/");
    out.split_inclusive('\n').filter(|l| !frame(l)).collect()
}

fn which(bin: &str) -> String {
    let out = Command::new("sh")
        .args(["-c", &format!("command -v {bin}")])
        .output()
        .unwrap();
    String::from_utf8(out.stdout).unwrap().trim().to_string()
}

fn tree() -> Vec<(&'static str, &'static str)> {
    vec![
        ("composer.json", r#"{"require":{"php":"^8.4"}}"#),
        ("pint.json", r#"{"preset":"laravel"}"#),
        (
            "app/Fix.php",
            "<?php\n$a = array( 1,2 );\nif($a){echo \"x\";}\n",
        ),
        ("app/Clean.php", "<?php\n\necho 1;\n"),
        ("app/Crlf.php", "<?php\r\n$a = array(1);\r\n"),
        (
            "app/Bad.php",
            "<?php\n\n// <info>tags</info> and \\<escapes>\n$a = [\n    1,\nfunction (\n\n$b = 1;\n$c = 2;\n$d = 3;\n$e = 4;\n$f = 5;\n",
        ),
        (
            "app/a&b <x>.php",
            "<?php\n$s = \"]]>\";\n$t = '<fg=red>x</>';   \n",
        ),
        (
            "app/Long/VeryLongDirectoryNameToForceTruncation/OfTheDescriptionToo.php",
            "<?php\n$c = array( 1,2 );\n",
        ),
    ]
}

/// The whole matrix on one tree: a fixable file (with XML/formatter-hostile content and name), a clean one and a
/// syntax error (PHP's message, `-v` snippet with tag-like comments).
#[test]
fn every_mode_like_pint() {
    let Some(cx) = Cx::new("modes") else { return };
    let t = tree();
    for args in [
        &[][..],
        &["--test"],
        &["-v"],
        &["--test", "-v"],
        &["-q"],
        &["--test", "-q"],
        &["--bail", "-v"],
        &["--ansi"],
        &["--ansi", "--test", "-v"],
        &["--ansi", "-v"],
        &["--ansi", "-q", "--test"],
        &["--ansi", "--no-ansi", "--test"],
        &["--no-ansi", "--test", "-v"],
    ] {
        cx.same(&t, args, PLAIN);
    }
    for format in [
        "json",
        "txt",
        "agent",
        "checkstyle",
        "junit",
        "xml",
        "gitlab",
    ] {
        let f = format!("--format={format}");
        for extra in [
            &[][..],
            &["--test"],
            &["--test", "-v"],
            &["--ansi", "--test", "-v"],
            &["--ansi"],
        ] {
            cx.same(&t, &[&[f.as_str()][..], extra].concat(), PLAIN);
        }
        // A clean run (junit's "All OK", empty checkstyle/xml/gitlab).
        cx.same(&t, &[f.as_str(), "--test", "app/Clean.php"], PLAIN);
        // The file report is the raw (escaped when decorated) reporter output.
        let file = format!("--output-to-file=report.{format}");
        cx.same(&t, &["--test", "-v", "--ansi", &f, &file], PLAIN);
        cx.same(
            &t,
            &["--test", "-q", "--output-format", format, &file],
            PLAIN,
        );
    }
    // Colour switches from the environment.
    cx.same(
        &t,
        &["--test"],
        Opts {
            env: &[("FORCE_COLOR", "1")],
            ..PLAIN
        },
    );
    cx.same(
        &t,
        &["--test", "-v"],
        Opts {
            env: &[("COLLISION_FORCE_COLORS", "1")],
            ..PLAIN
        },
    );
    // Termwind: truncation (`…`), clipping under -v, and a path wider than the box, across widths.
    for cols in ["40", "45", "52", "53", "54", "55", "75", "81", "120"] {
        cx.same(
            &t,
            &["--test"],
            Opts {
                env: &[("COLUMNS", cols)],
                ..PLAIN
            },
        );
        cx.same(
            &t,
            &["--test", "-v"],
            Opts {
                env: &[("COLUMNS", cols)],
                ..PLAIN
            },
        );
    }
}

/// A real terminal: Symfony decorates, Collision highlights `-v` code, the width comes from `stty -a`.
#[test]
fn terminal_like_pint() {
    let Some(cx) = Cx::new("tty") else { return };
    let t = tree();
    for (args, cols) in [
        (&["--test"][..], 80),
        (&["-v"], 80),
        (&["--test", "-v"], 50),
        (&["--test"], 0),
        (&["--no-ansi", "--test", "-v"], 80),
        (&["--format=txt", "--test", "-v"], 80),
        (&["-q", "--test"], 80),
    ] {
        cx.same(
            &t,
            args,
            Opts {
                tty: Some(cols),
                ..PLAIN
            },
        );
    }
    for env in [
        &[("NO_COLOR", "1")][..],
        &[("TERM", "dumb")],
        &[("TERM", "dumb"), ("COLORTERM", "truecolor")],
        &[("COLUMNS", "60")],
    ] {
        cx.same(
            &t,
            &["--test", "-v"],
            Opts {
                tty: Some(80),
                env,
                ..PLAIN
            },
        );
    }
}

/// `// @php-cs-fixer-ignore`: the named rules skip that file; a rule that isn't enabled, a duplicated value or a tab
/// after the annotation name aborts the run with Pint's box (Runner / FixerAnnotationAnalyzer exceptions).
#[test]
fn ignore_annotations_like_pint() {
    let Some(cx) = Cx::new("ignore") else { return };
    let mut t = tree();
    t.retain(|(f, _)| !f.starts_with("app/Bad"));
    t.push((
        "app/Ignored.php",
        "<?php\n\n// @php-cs-fixer-ignore single_quote,array_syntax\n$a = array(\"x\");\n",
    ));
    for args in [&[][..], &["--test"], &["-v"]] {
        cx.same(&t, args, PLAIN);
    }
    // A tab right after the name: FixerAnnotationAnalyzer's "Undefined array key 1".
    for bad in [
        " nope,@Foo,",
        " single_quote,a,a",
        " \tsingle_quote",
        "\tsingle_quote",
    ] {
        let mut t = t.clone();
        t.push((
            "app/Zbad.php",
            Box::leak(
                format!("<?php\n// @php-cs-fixer-ignore{bad}\n$a = \"x\";\n").into_boxed_str(),
            ),
        ));
        let r = cx.same(&t, &["--test"], PLAIN);
        assert_eq!(r.code, Some(1), "{bad}");
        cx.same_trace(&t, &["--test", "-v", "--ansi"]);
    }
}

/// Bad `--format`: Pint's abort box (stdout), wrapped to the terminal, red with --ansi; files are still fixed.
#[test]
fn abort_box_like_pint() {
    let Some(cx) = Cx::new("abort") else { return };
    let t = tree();
    cx.same(&t, &["--format=nope"], PLAIN);
    cx.same(&t, &["--format=nope", "--ansi", "--test"], PLAIN);
    cx.same(
        &t,
        &["--format=a-format-name-long-enough-to-wrap-in-a-narrow-terminal"],
        Opts {
            env: &[("COLUMNS", "30")],
            ..PLAIN
        },
    );
    cx.same(
        &t,
        &[
            "--test",
            "--output-to-file=r",
            "--output-format=nope",
            "--format=json",
        ],
        PLAIN,
    );
    // COLUMNS=0: Symfony doesn't wrap at all.
    cx.same(&t, &["--format=a-format-name-long-enough-to-wrap-in-an-eighty-column-terminal-if-it-were-wrapped-at-all"], Opts { env: &[("COLUMNS", "0")], ..PLAIN });
}

/// Option errors (ArgvInput: unknown, value on a flag, missing value) and run errors (with the PHP exception's
/// "In X.php line N:" header) as Pint's box on stdout, exit 1, coloured by a raw `--ansi`/`--no-ansi`.
#[test]
fn error_boxes_like_pint() {
    let Some(cx) = Cx::new("errors") else { return };
    let t = tree();
    for args in [
        &["--nope"][..],
        &["--nope=1", "--test"],
        &["-z"],
        &["--te"],
        &["--test=1"],
        &["--config"],
        &["--nope", "--ansi"],
        &["--no-ansi", "--ansi", "--nope"],
        &["-q", "--nope"],
        &["app/Missing.php"],
        &["--ansi", "app/Missing.php"],
        &["--config=http://example.com/pint.json"],
        // `pun format`'s own flags are unknown options to Pint.
        &["--watch"],
        &["--explain"],
        &["--native-only"],
        &["--stdin"],
        &["--staged"],
        &["--since=HEAD"],
        &["--no-cache"],
        &["--list-files"],
    ] {
        let r = cx.same(&t, args, PLAIN);
        assert_eq!(r.code, Some(1), "{args:?}");
    }
    // A top-level list is an empty config (not invalid JSON); an unknown preset (Pint 1.29) finds no files, then
    // its summary fails on the preset's name.
    for cfg in [
        r#"{"in":["nope"]}"#,
        r#"{"extend":"b.json"}"#,
        r#"{"rules":{"array_syntax":1}}"#,
        r#"{"rules":{"@PSR12":null}}"#,
        "{bad",
        "[1]",
        r#"[{"preset":"empty"}]"#,
        r#"{"preset":"nope"}"#,
    ] {
        let mut t = tree();
        t[1] = ("pint.json", cfg);
        t.push(("b.json", r#"{"extend":"c.json"}"#));
        let r = cx.same(&t, &["--test"], PLAIN);
        assert_eq!(r.code, Some(1), "{cfg}");
        if !cfg.starts_with('[') {
            cx.same_trace(&t, &["--test", "-v"]);
        }
    }
    for args in [
        &["--preset=nope"][..],
        &["--preset=nope", "--test", "--format=json"],
        &["--preset=nope", "--format=txt"],
        &["--preset=nope", "-q"],
        &["--preset=nope", "--ansi"],
    ] {
        cx.same(&t, args, PLAIN);
    }
    // -v: the exception class in the box and the throw site under it, for option errors too; -q wins over -v.
    for args in [
        &["--nope", "-v"][..],
        // (not -vv: the PHAR's stub then prints Box's requirements check on stderr)
        &["-z", "-v"],
        &["-vz"],
        &["--test=1", "--verbose"],
        &["--config", "-v"],
        &["--watch", "-v", "--ansi"],
        &["--format=nope", "-v"],
        &["--config=http://example.com/pint.json", "-v"],
        &["app/Missing.php", "-v"],
        &["--test", "-v", "--format=json", "--output-to-file=app"],
        &["-vq", "--nope"],
    ] {
        cx.same_trace(&t, args);
    }
    cx.same(&t, &["-qv", "--nope"], PLAIN);
    // ArgvInput lets a flag repeat.
    cx.same(&t, &["--test", "--test"], PLAIN);
    // Help stays clap's (Pint's help differs): exit 0, not a box.
    let ours = |args: &[&str]| {
        run(
            &[&[cx.alias.to_str().unwrap()][..], args].concat(),
            &cx.dir,
            PLAIN,
        )
    };
    let h = ours(&["--help"]);
    assert!(
        h.code == Some(0)
            && h.stdout
                .starts_with("Fix the coding style of the given path\n"),
        "{h:?}"
    );
    // Unreadable / unwritable files abort the run (kept outside the snapshotted tree).
    let outside = tmp("perm").join("Fix.php");
    // --bail stops there too, after the progress of the files before it.
    for (mode, before, args) in [
        (0o000, &[][..], &["--test"][..]),
        (0o444, &[], &[]),
        (0o000, &["--bail", "app/Clean.php"], &[]),
        (0o444, &["-v"], &[]),
    ] {
        let pair = |cmd: &[&str]| {
            reset(&cx.dir, &t);
            std::fs::write(&outside, "<?php\n$a = array( 1,2 );\n").unwrap();
            std::fs::set_permissions(&outside, std::os::unix::fs::PermissionsExt::from_mode(mode))
                .unwrap();
            let r = run(
                &[cmd, before, &[outside.to_str().unwrap()], args].concat(),
                &cx.dir,
                PLAIN,
            );
            std::fs::set_permissions(
                &outside,
                std::os::unix::fs::PermissionsExt::from_mode(0o644),
            )
            .unwrap();
            r
        };
        let mut want = pair(&[&which("php"), cx.oracle.to_str().unwrap()]);
        want.stdout = normalise_trace(&want.stdout, &cx.oracle);
        assert!(
            want.stdout.contains("\nIn ") && want.code == Some(1),
            "{want:?}"
        );
        assert_eq!(pair(&[cx.alias.to_str().unwrap()]), want, "mode {mode:o}");
    }
}

/// Gitlab entries for rules GitlabReporter's FixerFactory doesn't know (Pun's own, enabled via pun.toml) use its
/// "custom rule" wording. Hand-derived from GitlabReporter::generate (Pint has no such rules to run).
#[test]
fn gitlab_custom_rule() {
    let dir = tmp("gitlab");
    reset(
        &dir,
        &[
            (
                "pun.toml",
                "[format]\npreset = \"empty\"\n[format.rules]\n\"Pun/no_only\" = true\n",
            ),
            ("t.php", "<?php\ntest('a', fn () => 1)->only();\n"),
        ],
    );
    let r = run(
        &[
            env!("CARGO_BIN_EXE_pun"),
            "format",
            "--test",
            "--format=gitlab",
        ],
        &dir,
        PLAIN,
    );
    let v: serde_json::Value = serde_json::from_str(&r.stdout).unwrap();
    assert_eq!(
        v[0]["description"], "PHP-CS-Fixer.Pun/no_only (custom rule)",
        "{}",
        r.stdout
    );
    assert_eq!(
        v[0]["content"]["body"],
        "PHP CS Fixer 3.95.17 Adalbertus by Fabien Potencier, Dariusz Ruminski and contributors.\nCheck performed with a custom rule."
    );
    // md5("t.phpPun/no_only")
    assert_eq!(v[0]["fingerprint"], "147051f627619bd7f4d79ce3046a3395");
}

/// EnsurePrettierIsConfigured: `"Pint/laravel_blade": true` needs Node and prettier (+ plugins) in the project.
#[test]
fn prettier_preflight_like_pint() {
    let Some(cx) = Cx::new("prettier") else {
        return;
    };
    let mut t = tree();
    t[1] = (
        "pint.json",
        r#"{"preset":"laravel","rules":{"Pint/laravel_blade":true}}"#,
    );
    // Missing packages (npm, then pnpm by lock file), with and without colours, and via --blade.
    let r = cx.same(&t, &["--test"], PLAIN);
    assert!(
        r.stdout
            .contains("prettier dependencies to be installed using [npm]")
            && r.code == Some(1),
        "{r:?}"
    );
    cx.same(&t, &["--ansi"], PLAIN);
    cx.same_trace(&t, &["--test", "-v"]);
    let mut pnpm = t.clone();
    pnpm.push(("pnpm-lock.yaml", ""));
    cx.same(&pnpm, &["--test"], PLAIN);
    let mut flag = tree();
    flag.push(("bun.lock", ""));
    cx.same(&flag, &["--blade", "--test"], PLAIN);
    // Options instead of `true` don't trigger the check.
    let mut opts = tree();
    opts[1] = (
        "pint.json",
        r#"{"preset":"laravel","rules":{"Pint/laravel_blade":{}}}"#,
    );
    let (want, got) = cx.pair(&opts, &["--test", "app/Clean.php"], PLAIN);
    assert!(
        !want.stdout.contains("prettier") && !got.stdout.contains("prettier"),
        "{want:?} / {got:?}"
    );
    // Installed but too old / too new.
    let mut old = t.clone();
    let manifests: Vec<(String, String)> = [
        ("prettier", "2.1.0"),
        ("prettier-plugin-blade", "3.2.9"),
        ("prettier-plugin-tailwindcss", "0.9.1"),
    ]
    .iter()
    .flat_map(|(n, v)| {
        [
            (
                format!("node_modules/{n}/package.json"),
                format!(r#"{{"name":"{n}","version":"{v}","main":"i.js"}}"#),
            ),
            (format!("node_modules/{n}/i.js"), String::new()),
        ]
    })
    .collect();
    old.extend(manifests.iter().map(|(a, b)| (a.as_str(), b.as_str())));
    old.push(("yarn.lock", ""));
    cx.same(&old, &["--test"], PLAIN);
    // No Node.js on PATH.
    let bin = tmp("nonode");
    for b in ["php", "sh"] {
        std::os::unix::fs::symlink(which(b), bin.join(b)).unwrap();
    }
    cx.same(
        &t,
        &["--test"],
        Opts {
            path: Some(bin.to_str().unwrap()),
            ..PLAIN
        },
    );
}

/// Without a `php` on PATH Pun can't ask PHP for its wording: it falls back to its own parser's message (and
/// line), keeping Pint's layout. Hand-derived: Pint itself always has PHP.
#[test]
fn syntax_error_without_php() {
    let dir = tmp("nophp");
    reset(
        &dir,
        &[
            (
                "pint.json",
                r#"{"preset":"empty","rules":{"array_syntax":true}}"#,
            ),
            ("a.php", "<?php\nfunction (\n"),
        ],
    );
    let alias = tmp("bin2").join("pint");
    std::os::unix::fs::symlink(env!("CARGO_BIN_EXE_pun"), &alias).unwrap();
    let r = run(
        &[alias.to_str().unwrap(), "--test", "-v"],
        &dir,
        Opts {
            path: Some("/nonexistent"),
            ..PLAIN
        },
    );
    let line = r
        .stdout
        .lines()
        .find(|l| l.starts_with("  ! a.php "))
        .unwrap_or_default();
    assert!(
        line.contains(" Parse error: ") && line.ends_with(" on line 3.  "),
        "{}",
        r.stdout
    );
    assert!(
        r.stdout.contains("\n      2▕ function (\n  ➜   3▕\n"),
        "{}",
        r.stdout
    );
}

/// A fixer exception (combine_consecutive_unsets' "Must not add empty token", a panic in Pun) fails only that file:
/// `!` with the message, the other files still fixed, exit 1, no panic output. With a syntax error too, the summary's
/// `invalid + exception` array union drops the exception.
#[test]
fn fixer_exception_like_pint() {
    let Some(cx) = Cx::new("exception") else {
        return;
    };
    let mut t = vec![
        (
            "pint.json",
            r#"{"preset":"empty","rules":{"combine_consecutive_unsets":true,"array_syntax":true}}"#,
        ),
        (
            "app/U.php",
            "<?php unset($a ); unset( $b );\nunset($x, ); unset( $y);",
        ),
        (
            "app/Fix.php",
            "<?php\n$a = array(1);\nunset($a); unset($b);\n",
        ),
    ];
    for args in [
        &[][..],
        &["--test"],
        &["--test", "-v"],
        &["--format=json"],
        &["--format=agent"],
        &["--test", "-q"],
    ] {
        let r = cx.same(&t, args, PLAIN);
        assert_eq!(r.code, Some(1), "{args:?}");
    }
    t.push(("app/Bad.php", "<?php\nfunction (\n"));
    cx.same(&t, &["--test"], PLAIN);
    cx.same(&t, &["--format=txt", "--test"], PLAIN);
}
