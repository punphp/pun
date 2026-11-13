//! Output. Pun's own human output (format.md §9, output.md §1) against the plan's samples, derived by hand, with and
//! without colour, piped and on a terminal. Byte-for-byte parity with the real Pint (`php .oracle/pint/builds/pint`)
//! for what stays Pint's: exit codes, resulting files, every `--format` reporter (with `--ansi`), `-q`, abort boxes
//! and the prettier pre-flight check. Each Pint case runs Pint, resets the tree, runs `pun migrate` and `pun format`
//! in the same directory; human runs there compare the files each tool names (tests/common). The Pint cases are
//! skipped without the oracle.
mod common;

use common::{migrate, oracle, scratch, write_files};

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Recreates `dir` with `files` (fresh directory, so readdir order is the same for both tools).
fn reset(dir: &Path, files: &[(&str, &str)]) {
    let _ = std::fs::remove_dir_all(dir);
    write_files(dir, files);
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
            let line = format!("stty cols {cols} rows 40; {}", quoted.join(" "));
            let mut c = Command::new("script");
            // util-linux's script takes the command with -c; BSD's (macOS) after the typescript file.
            if cfg!(target_os = "macos") {
                c.args(["-q", "/dev/null", "sh", "-c", &line]);
            } else {
                c.args(["-qefc", &line, "/dev/null"]);
            }
            c
        }
    };
    common::clean_env(&mut c, &["HOME"]).current_dir(dir);
    c.env("PUN_AGENT", "0").env(
        "PATH",
        o.path
            .map_or_else(|| std::env::var("PATH").unwrap(), str::to_string),
    );
    if o.tty.is_some() {
        c.env("TERM", "xterm");
    } else {
        c.env("COLUMNS", "80");
    }
    // Pun's own output: a UTF-8 locale gets ✔ and ✖ rather than their ASCII fallbacks.
    c.env("LANG", "en_US.UTF-8");
    let out = c
        .envs(o.env.iter().copied())
        .stdin(std::process::Stdio::null())
        .output()
        .unwrap();
    // pun.toml: written by `pun migrate` for Pun's run.
    let files = common::snapshot(dir, |r| r == "pun.toml");
    // A pty turns `\n` into `\r\n`; BSD's script echoes the closed stdin as `^D` and two backspaces first.
    // Bytes that aren't UTF-8 show as `\xHH`, so a raw byte and its U+FFFD replacement differ.
    let text = |b: &[u8]| {
        let mut t = String::new();
        for chunk in b.utf8_chunks() {
            t.push_str(chunk.valid());
            for byte in chunk.invalid() {
                t.push_str(&format!("\\x{byte:02x}"));
            }
        }
        let t = t.replace("\r\n", "\n");
        t.strip_prefix("^D\x08\x08")
            .map_or(t.clone(), str::to_string)
    };
    Run {
        stdout: text(&out.stdout),
        stderr: text(&out.stderr),
        code: out.status.code(),
        files,
    }
}

const PUN: [&str; 2] = [env!("CARGO_BIN_EXE_pun"), "format"];
/// combine_consecutive_unsets deletes code, so it is a lint rule (context/plan/commands.md §4): `pun check --fix`
/// is what runs it next to the format rules, as Pint does.
const CHECK_FIX: [&str; 3] = [env!("CARGO_BIN_EXE_pun"), "check", "--fix"];

struct Cx {
    oracle: PathBuf,
    dir: PathBuf,
}

impl Cx {
    fn new(name: &str) -> Option<Cx> {
        Some(Cx {
            oracle: oracle()?,
            dir: scratch("out", name),
        })
    }

    fn pair(&self, tree: &[(&str, &str)], args: &[&str], o: Opts) -> (Run, Run) {
        self.pair_cmd(&PUN, tree, args, o)
    }

    /// `pint <args>` against `<pun> <args>`, where `pun` is the binary and its command.
    fn pair_cmd(&self, pun: &[&str], tree: &[(&str, &str)], args: &[&str], o: Opts) -> (Run, Run) {
        let php = which("php");
        reset(&self.dir, tree);
        let want = run(
            // Production PHP (php.ini-production) compiles assertions out: without a php.ini, FixerAnnotationAnalyzer's
            // assert() would fail before the "Undefined array key 1" users see.
            &[
                &[
                    php.as_str(),
                    "-d",
                    "zend.assertions=-1",
                    self.oracle.to_str().unwrap(),
                ][..],
                args,
            ]
            .concat(),
            &self.dir,
            o,
        );
        reset(&self.dir, tree);
        migrate(&self.dir);
        let got = run(&[pun, args].concat(), &self.dir, o);
        (want, got)
    }

    /// `same_abort` for `-v` error boxes in a `--format` run: in Pint's exception trace the PHAR's absolute path becomes `phar://pint/` and
    /// the PHP call frames under the throw site are dropped (Pun prints only the throw site).
    fn same_trace(&self, tree: &[(&str, &str)], args: &[&str]) -> Run {
        let (mut want, got) = self.pair(tree, args, PLAIN);
        assert_eq!((want.code, got.code), (Some(1), Some(2)), "{args:?}");
        want.code = Some(2);
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
        self.same_cmd(&PUN, tree, args, o)
    }

    fn same_cmd(&self, pun: &[&str], tree: &[(&str, &str)], args: &[&str], o: Opts) -> Run {
        self.same_exit(pun, tree, args, o, None)
    }

    /// `same_cmd`, where Pint exits 1 and Pun `code` (2: a fixer failed, Pun couldn't run; output.md §2).
    fn same_exit(
        &self,
        pun: &[&str],
        tree: &[(&str, &str)],
        args: &[&str],
        o: Opts,
        code: Option<i32>,
    ) -> Run {
        let (mut want, mut got) = self.pair_cmd(pun, tree, args, o);
        if code.is_some() {
            assert_eq!(want.code, Some(1), "{args:?}");
            want.code = code;
        }
        if common::human(args) && pun.get(1) == Some(&"check") {
            // `pun check` prints its own grouped view (output.md, Build 5), which lists failing files, not fixed
            // ones: it must name each file Pint reports an error for (`!`); exit code and files must match.
            for line in want.stdout.lines() {
                if let Some(rest) = line.trim_start().strip_prefix("! ") {
                    let file = rest.split("  ").next().unwrap().trim_end_matches('…');
                    assert!(
                        got.stdout.contains(file),
                        "{args:?}: {file} not named\n{}",
                        got.stdout
                    );
                }
            }
            (want.stdout, got.stdout) = (String::new(), String::new());
        } else if common::human(args) {
            (want.stdout, got.stdout) = common::human_pair(
                &want.stdout,
                &got.stdout,
                want.files.keys(),
                got.files.keys(),
            );
        }
        assert_eq!(got, want, "{args:?} env={:?} tty={:?}", o.env, o.tty);
        got
    }
}

impl Cx {
    /// A run error: Pint aborts with its box (exit 1), Pun exits 2 (output.md §2). A `--format` or `-q` run prints
    /// Pint's box byte for byte; a human run prints `human` (Pun's run error, output.md §1) on stderr instead,
    /// after its usual output. The files are the same either way.
    fn same_abort(&self, tree: &[(&str, &str)], args: &[&str], o: Opts, human: &str) -> Run {
        let (mut want, mut got) = self.pair(tree, args, o);
        assert_eq!(
            (want.code, got.code),
            (Some(1), Some(2)),
            "{args:?}: {got:?}"
        );
        want.code = Some(2);
        if common::human(args) {
            assert!(
                want.stdout.ends_with("\n\n") && got.stderr == human,
                "{args:?}: pint {want:?}, pun {got:?}"
            );
            (want.stdout, got.stdout) = common::human_pair(
                &want.stdout,
                &got.stdout,
                want.files.keys(),
                got.files.keys(),
            );
            // Without a summary (an abort before any file), Pint's stdout is the box alone.
            if !want.stdout.starts_with("named: ") {
                want.stdout = String::new();
            }
            want.stderr.clone_from(&got.stderr);
        }
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

/// What stays Pint's, on one tree: a fixable file (with XML/formatter-hostile content and name), a clean one and a
/// syntax error. Exit codes and files for the human modes, `-q` byte for byte, and every `--format` reporter.
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
        &["--ansi", "--test", "-v"],
        &["--ansi", "-q", "--test"],
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
    // Symfony's colour switch from the environment still decorates the reports.
    cx.same(
        &t,
        &["--test", "--format=txt", "-v"],
        Opts {
            env: &[("FORCE_COLOR", "1")],
            ..PLAIN
        },
    );
}

/// A real terminal: the reports, `-q` and the abort box stay Pint's, decorated as Symfony decides.
#[test]
fn terminal_like_pint() {
    let Some(cx) = Cx::new("tty") else { return };
    let t = tree();
    // An abort box wraps at the width `stty -a` reports (no COLUMNS on a terminal).
    let tty = |cols| Opts {
        tty: Some(cols),
        ..PLAIN
    };
    for args in [&["--format=txt", "--test", "-v"][..], &["-q", "--test"]] {
        cx.same(&t, args, tty(50));
    }
    let wide = "--format=a-format-name-long-enough-to-wrap-in-a-fifty-column-terminal";
    cx.same_abort(&t, &[wide], tty(50), "");
    // A pty of 0 columns: Symfony falls back to 80.
    cx.same_abort(
        &t,
        &["--format=a-format-name-long-enough-to-wrap-in-an-eighty-column-terminal-if-it-were-wrapped"],
        tty(0),
        "",
    );
}

/// The tree of the human-output samples: two files `format/arrays/syntax` fixes, a clean one and, with `broken`, a
/// syntax error on its line 5 (format.md §9's sample, at its own line).
fn sample_tree(broken: bool) -> Vec<(&'static str, &'static str)> {
    let mut t = vec![
        (
            "pun.toml",
            "[format]\npreset = \"empty\"\n[format.rules]\n\"arrays/syntax\" = true\n",
        ),
        ("app/Models/User.php", "<?php\n$a = array(1);\n"),
        ("app/Http/Kernel.php", "<?php\n$b = array(2);\n"),
        ("app/Clean.php", "<?php\n$c = [3];\n"),
    ];
    if broken {
        t.push((
            "app/Broken.php",
            "<?php\nfunction f()\n{\n    return $x\n}\n",
        ));
    }
    t
}

/// `pun <args>` on a fresh copy of `tree`: stdout (the time in the summary as `T`) and the exit code.
fn sample(tree: &[(&str, &str)], args: &[&str], o: Opts) -> (String, Option<i32>) {
    let dir = scratch("out", "sample");
    reset(&dir, tree);
    let r = run(&[&[env!("CARGO_BIN_EXE_pun")][..], args].concat(), &dir, o);
    (timeless(&r.stdout), r.code)
}

/// The run's time (`0.12s`, at the end of the summary, the last line) as `T`.
fn timeless(out: &str) -> String {
    let body = out.strip_suffix('\n').unwrap_or(out);
    let reset = if body.ends_with(Z) { Z } else { "" };
    let Some(head) = body.strip_suffix(reset).and_then(|b| b.strip_suffix('s')) else {
        return out.to_string();
    };
    let head = head.trim_end_matches(|c: char| c.is_ascii_digit() || c == '.');
    format!("{head}T{reset}\n")
}

/// SGR codes of the 16-colour look (`--color=always` without `COLORTERM`/`TERM`): error, ok, rule, dim, reset.
const E: &str = "\x1b[31m";
const OK: &str = "\x1b[32m";
const R: &str = "\x1b[36m";
const D: &str = "\x1b[2m";
const Z: &str = "\x1b[0m";

/// format.md §9's samples: `pun format` (✔ and the rule IDs from the 27th column, "Formatted N files, M unchanged")
/// and `pun format --test` (✖, "N of T files need formatting · run `pun format` to fix"), exits 0 and 1.
#[test]
fn format_samples() {
    let t = sample_tree(false);
    let never = ["format", "--color=never"];
    assert_eq!(
        sample(&t, &[&never[..], &["--test"]].concat(), PLAIN),
        (
            concat!(
                " ✖ app/Http/Kernel.php        format/arrays/syntax\n",
                " ✖ app/Models/User.php        format/arrays/syntax\n",
                "2 of 3 files need formatting · run `pun format` to fix · T\n",
            )
            .to_string(),
            Some(1)
        )
    );
    assert_eq!(
        sample(&t, &never, PLAIN),
        (
            concat!(
                " ✔ app/Http/Kernel.php        format/arrays/syntax\n",
                " ✔ app/Models/User.php        format/arrays/syntax\n",
                "Formatted 2 files, 1 unchanged · 3 files in T\n",
            )
            .to_string(),
            Some(0)
        )
    );
    let dot = format!(" {D}·{Z} ");
    assert_eq!(
        sample(&t, &["format", "--color=always", "--test"], PLAIN),
        (
            format!(
                " {E}✖{Z} app/Http/Kernel.php        {R}format/arrays/syntax{Z}\n {E}✖{Z} app/Models/User.php        {R}format/arrays/syntax{Z}\n2 of 3 files need formatting{dot}run `pun format` to fix{dot}{D}T{Z}\n"
            ),
            Some(1)
        )
    );
    assert_eq!(
        sample(&t, &["format", "--color=always"], PLAIN),
        (
            format!(
                " {OK}✔{Z} app/Http/Kernel.php        {R}format/arrays/syntax{Z}\n {OK}✔{Z} app/Models/User.php        {R}format/arrays/syntax{Z}\nFormatted 2 files, 1 unchanged{dot}{D}3 files in T{Z}\n"
            ),
            Some(0)
        )
    );
    // Clean files print nothing; a clean `--test` run is one line. One file is singular.
    let clean = &t[..1].iter().chain(&t[3..]).copied().collect::<Vec<_>>();
    assert_eq!(
        sample(clean, &[&never[..], &["--test"]].concat(), PLAIN),
        ("All 1 file formatted · T\n".to_string(), Some(0))
    );
    assert_eq!(
        sample(clean, &never, PLAIN),
        (
            "Formatted 0 files, 1 unchanged · 1 file in T\n".to_string(),
            Some(0)
        )
    );
    let one = &t[..2].to_vec();
    assert_eq!(
        sample(one, &[&never[..], &["--test"]].concat(), PLAIN),
        (
            " ✖ app/Models/User.php        format/arrays/syntax\n1 of 1 file needs formatting · run `pun format` to fix · T\n"
                .to_string(),
            Some(1)
        )
    );
    // `-q` prints nothing on success.
    assert_eq!(
        sample(&t, &["format", "-q"], PLAIN),
        (String::new(), Some(0))
    );
}

/// `-v`: a dim line per unchanged file, and under each changed file its diff with old and new line numbers (removed
/// red, added green, context dim).
#[test]
fn verbose_sample() {
    let t = sample_tree(false);
    let lines = concat!(
        " ✔ app/Clean.php\n",
        " ✖ app/Http/Kernel.php        format/arrays/syntax\n",
        "   1 1 │  <?php\n",
        "   2   │ -$b = array(2);\n",
        "     2 │ +$b = [2];\n",
        " ✖ app/Models/User.php        format/arrays/syntax\n",
        "   1 1 │  <?php\n",
        "   2   │ -$a = array(1);\n",
        "     2 │ +$a = [1];\n",
    );
    assert_eq!(
        sample(&t, &["format", "--color=never", "--test", "-v"], PLAIN),
        (
            format!("{lines}2 of 3 files need formatting · run `pun format` to fix · T\n"),
            Some(1)
        )
    );
    let bar = format!("{D}│{Z}");
    let diff = |n: &str, old: &str, new: &str| {
        format!(
            "   {D}1 1{Z} {bar}  {D}<?php{Z}\n   {D}2  {Z} {bar} {E}-${n} = array({old});{Z}\n   {D}  2{Z} {bar} {OK}+${n} = [{new}];{Z}\n"
        )
    };
    let dot = format!(" {D}·{Z} ");
    assert_eq!(
        sample(&t, &["format", "--color=always", "-v"], PLAIN),
        (
            format!(
                " {D}✔{Z} {D}app/Clean.php{Z}\n {OK}✔{Z} app/Http/Kernel.php        {R}format/arrays/syntax{Z}\n{} {OK}✔{Z} app/Models/User.php        {R}format/arrays/syntax{Z}\n{}Formatted 2 files, 1 unchanged{dot}{D}3 files in T{Z}\n",
                diff("b", "2", "2"),
                diff("a", "1", "1"),
            ),
            Some(0)
        )
    );
}

/// A syntax error: a problem block with a code frame at the line PHP names (format.md §9's sample in output.md §1's
/// block), after the file lines; it counts in the summary and fails the run, with and without `--test`.
#[test]
fn syntax_error_sample() {
    let t = sample_tree(true);
    let frame = concat!(
        " ✖ syntax-error  syntax error, unexpected token \"}\", expecting \";\"\n",
        "  ╭─ app/Broken.php:5:1\n",
        " 4│     return $x\n",
        " 5│ }\n",
        "  │ ━\n",
        "  ╰─\n",
    );
    assert_eq!(
        sample(&t, &["format", "--color=never", "--test"], PLAIN),
        (
            format!(
                " ✖ app/Http/Kernel.php        format/arrays/syntax\n ✖ app/Models/User.php        format/arrays/syntax\n\n{frame}2 of 4 files need formatting · 1 error · run `pun format` to fix · T\n"
            ),
            Some(1)
        )
    );
    assert_eq!(
        sample(&t, &["format", "--color=never", "app/Broken.php"], PLAIN),
        (
            format!("{frame}Formatted 0 files, 0 unchanged · 1 error · 1 file in T\n"),
            Some(1)
        )
    );
    let dot = format!(" {D}·{Z} ");
    assert_eq!(
        sample(
            &t,
            &["format", "--color=always", "--test", "app/Broken.php"],
            PLAIN
        ),
        (
            format!(
                " {E}✖{Z} {R}syntax-error{Z}  syntax error, unexpected token \"}}\", expecting \";\"\n  {D}╭─{Z} app/Broken.php:5:1\n {D}4{Z}{D}│{Z}     return $x\n {D}5{Z}{D}│{Z} }}\n  {D}│{Z} {E}━{Z}\n  {D}╰─{Z}\n{E}1 error{Z}{dot}{D}1 file in T{Z}\n"
            ),
            Some(1)
        )
    );
}

/// output.md §1: colour is on for a terminal and off when piped; `NO_COLOR` turns it off and `--color` (or Pint's
/// `--ansi` / `--no-ansi`) overrides both. A short run shows no spinner.
#[test]
fn colour_follows_the_terminal() {
    let t = sample_tree(false);
    let plain = " ✖ app/Http/Kernel.php        format/arrays/syntax\n ✖ app/Models/User.php        format/arrays/syntax\n2 of 3 files need formatting · run `pun format` to fix · T\n";
    let dot = format!(" {D}·{Z} ");
    let painted = format!(
        " {E}✖{Z} app/Http/Kernel.php        {R}format/arrays/syntax{Z}\n {E}✖{Z} app/Models/User.php        {R}format/arrays/syntax{Z}\n2 of 3 files need formatting{dot}run `pun format` to fix{dot}{D}T{Z}\n"
    );
    let tty = Opts {
        tty: Some(80),
        ..PLAIN
    };
    let no_color = Opts {
        env: &[("NO_COLOR", "1")],
        ..PLAIN
    };
    for (args, o, want) in [
        (&["--test"][..], PLAIN, plain),
        (&["--test", "--ansi"], PLAIN, painted.as_str()),
        (&["--test", "--color=always"], no_color, &painted),
        (&["--test"], tty, &painted),
        (
            &["--test"],
            Opts {
                tty: Some(80),
                ..no_color
            },
            plain,
        ),
        (&["--test", "--color=never"], tty, plain),
        (&["--test", "--no-ansi"], tty, plain),
        (&["--test", "--ansi", "--no-ansi"], PLAIN, &painted),
    ] {
        assert_eq!(
            sample(&t, &[&["format"][..], args].concat(), o),
            (want.to_string(), Some(1)),
            "{args:?} tty={:?} env={:?}",
            o.tty,
            o.env
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
    let base = t.clone();
    t.push((
        "app/Ignored.php",
        "<?php\n\n// @php-cs-fixer-ignore single_quote,array_syntax\n$a = array(\"x\");\n",
    ));
    // `pun migrate` rewrote the annotation (hand-derived from the mapping); the rest is Pint's.
    for args in [&[][..], &["--test"], &["-v"]] {
        let (mut want, mut got) = cx.pair(&t, args, PLAIN);
        (want.stdout, got.stdout) = common::human_pair(
            &want.stdout,
            &got.stdout,
            want.files.keys(),
            got.files.keys(),
        );
        let ignored = want.files.get_mut("app/Ignored.php").unwrap();
        *ignored = String::from_utf8_lossy(ignored)
            .replace(
                "// @php-cs-fixer-ignore single_quote,array_syntax",
                "// @pun-ignore-file format/strings/single-quote,format/arrays/syntax",
            )
            .into_bytes();
        assert_eq!(got, want, "{args:?}");
    }
    // A tab right after the name: FixerAnnotationAnalyzer's "Undefined array key 1".
    for bad in [
        " nope,@Foo,",
        " single_quote,a,a",
        " \tsingle_quote",
        "\tsingle_quote",
    ] {
        let mut t = base.clone();
        t.push((
            "app/Zbad.php",
            Box::leak(
                format!("<?php\n// @php-cs-fixer-ignore{bad}\n$a = \"x\";\n").into_boxed_str(),
            ),
        ));
        cx.same_abort(&t, &["--test", "--format=json"], PLAIN, "");
        cx.same_trace(&t, &["--test", "-v", "--ansi", "--format=txt"]);
    }
}

/// Bad `--format`: Pint's abort box (stdout), wrapped to the terminal, red with --ansi; files are still fixed; Pun
/// exits 2.
#[test]
fn abort_box_like_pint() {
    let Some(cx) = Cx::new("abort") else { return };
    let t = tree();
    cx.same_abort(&t, &["--format=nope"], PLAIN, "");
    cx.same_abort(&t, &["--format=nope", "--ansi", "--test"], PLAIN, "");
    cx.same_abort(
        &t,
        &["--format=a-format-name-long-enough-to-wrap-in-a-narrow-terminal"],
        Opts {
            env: &[("COLUMNS", "30")],
            ..PLAIN
        },
        "",
    );
    cx.same_abort(
        &t,
        &[
            "--test",
            "--output-to-file=r",
            "--output-format=nope",
            "--format=json",
        ],
        PLAIN,
        "",
    );
    // COLUMNS=0: Symfony doesn't wrap at all.
    cx.same_abort(&t, &["--format=a-format-name-long-enough-to-wrap-in-an-eighty-column-terminal-if-it-were-wrapped-at-all"], Opts { env: &[("COLUMNS", "0")], ..PLAIN }, "");
}

/// A file that isn't UTF-8 keeps its bytes in the `-v` diffs of the txt, junit and xml reports, and makes the json
/// report's `json_encode` throw (audit item 11).
#[test]
fn latin1_diffs_like_pint() {
    let Some(cx) = Cx::new("latin1") else { return };
    let php = which("php");
    let oracle = cx.oracle.to_str().unwrap();
    let side = |pun: bool, args: &[&str]| {
        reset(&cx.dir, &[]);
        std::fs::create_dir_all(&cx.dir).unwrap();
        std::fs::write(cx.dir.join("a.php"), b"<?php $a = array( \"caf\xe9\" );\n").unwrap();
        if pun {
            migrate(&cx.dir);
            run(&[&PUN[..], args].concat(), &cx.dir, PLAIN)
        } else {
            run(
                &[
                    &[php.as_str(), "-d", "zend.assertions=-1", oracle][..],
                    args,
                ]
                .concat(),
                &cx.dir,
                PLAIN,
            )
        }
    };
    for f in ["--format=txt", "--format=junit", "--format=xml"] {
        let args = ["--test", "-v", f, "a.php"];
        let (want, got) = (side(false, &args), side(true, &args));
        assert!(
            want.stdout.contains("+$a = ['caf\\xe9'];"),
            "{f}: {}",
            want.stdout
        );
        assert_eq!(got, want, "{f}");
    }
    let args = ["--test", "-v", "--format=json", "a.php"];
    let (mut want, got) = (side(false, &args), side(true, &args));
    assert_eq!((want.code, got.code), (Some(1), Some(2)));
    want.code = Some(2);
    want.stdout = normalise_trace(&want.stdout, &cx.oracle);
    assert!(
        want.stdout.contains("[JsonException (5)]"),
        "{}",
        want.stdout
    );
    assert_eq!(got, want);
    // The same throw when only the report file is json.
    let args = [
        "--test",
        "-v",
        "--format=txt",
        "--output-to-file=r.json",
        "--output-format=json",
        "a.php",
    ];
    let (mut want, got) = (side(false, &args), side(true, &args));
    assert_eq!((want.code, got.code), (Some(1), Some(2)));
    want.code = Some(2);
    want.stdout = normalise_trace(&want.stdout, &cx.oracle);
    assert!(
        want.stdout.contains("[JsonException (5)]"),
        "{}",
        want.stdout
    );
    assert_eq!(got, want);
}

/// Run errors exit 2 (output.md §2). Human runs print Pun's run error (output.md §1) on stderr, red with `--ansi`
/// or `--color=always`; `--format` and `-q` runs keep Pint's box (with the PHP exception's "In X.php line N:"
/// header) on stdout, coloured by a raw `--ansi`. pint.json's own errors are `pun migrate`'s (pun-config's tests
/// have Pint's messages).
/// PHP's parse error keeps the source's bytes (`é` cut after its first byte by `%.*s`, a Latin-1 `é`), and Pint's
/// ERROR line (`--format`) and `-q` line print them as they are.
#[test]
fn error_messages_keep_the_source_bytes_like_pint() {
    let Some(cx) = Cx::new("error-bytes") else {
        return;
    };
    let php = which("php");
    let oracle = cx.oracle.to_str().unwrap();
    let side = |pun: bool, args: &[&str]| {
        reset(&cx.dir, &[]);
        std::fs::create_dir_all(&cx.dir).unwrap();
        std::fs::write(
            cx.dir.join("a.php"),
            b"<?php\n$x = 1 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaa\xc3\xa9 bbbbbbbbbbbbb';\n",
        )
        .unwrap();
        std::fs::write(cx.dir.join("b.php"), b"<?php\n$x = 1 'caf\xe9';\n").unwrap();
        if pun {
            migrate(&cx.dir);
            run(&[&PUN[..], args].concat(), &cx.dir, PLAIN)
        } else {
            let pint = [php.as_str(), "-d", "zend.assertions=-1", oracle];
            run(&[&pint[..], args].concat(), &cx.dir, PLAIN)
        }
    };
    for (args, bytes) in [
        (&["--test", "--format=txt", "a.php"][..], "a\\xc3...\""),
        (&["--test", "-q", "b.php"], "caf\\xe9\""),
    ] {
        let (want, got) = (side(false, args), side(true, args));
        assert!(want.stderr.contains(bytes), "{args:?}: {}", want.stderr);
        assert_eq!(
            (&got.stderr, got.code),
            (&want.stderr, want.code),
            "{args:?}"
        );
    }
}

#[test]
fn error_boxes_like_pint() {
    let Some(cx) = Cx::new("errors") else { return };
    let t = tree();
    let missing = "The path \"app/Missing.php\" is not readable.";
    for (args, human) in [
        (&["app/Missing.php"][..], format!("✖ {missing}\n")),
        (&["--no-ansi", "app/Missing.php"], format!("✖ {missing}\n")),
        (
            &["--ansi", "app/Missing.php"],
            format!("\x1b[31m✖ {missing}\x1b[0m\n"),
        ),
        (&["--format=json", "app/Missing.php"], String::new()),
    ] {
        cx.same_abort(&t, args, PLAIN, &human);
    }
    let pun_only = |args: &[&str]| run(&[&PUN[..], args].concat(), &cx.dir, PLAIN);
    for (color, human) in [
        ("--color=never", format!("✖ {missing}\n")),
        ("--color=always", format!("\x1b[31m✖ {missing}\x1b[0m\n")),
    ] {
        let r = pun_only(&[color, "app/Missing.php"]);
        assert_eq!((r.stdout.as_str(), r.stderr, r.code), ("", human, Some(2)));
    }
    // A top-level list is an empty config (not invalid JSON).
    let nope = "The \"nope\" directory does not exist.";
    for (cfg, human) in [
        (r#"{"in":["nope"]}"#, format!("✖ {nope}\n")),
        ("[1]", String::new()),
        (r#"[{"preset":"empty"}]"#, String::new()),
    ] {
        let mut t = tree();
        t[1] = ("pint.json", cfg);
        if human.is_empty() {
            cx.same(&t, &["--test"], PLAIN);
            continue;
        }
        cx.same_abort(&t, &["--test"], PLAIN, &human);
        cx.same_trace(&t, &["--test", "-v", "--format=json"]);
    }
    // -v: the exception class in the box and the throw site under it; -q wins over -v.
    for args in [
        &["--format=nope", "-v"][..],
        &["app/Missing.php", "-v", "--format=txt"],
        &["--test", "-v", "--format=json", "--output-to-file=app"],
    ] {
        cx.same_trace(&t, args);
    }
    cx.same_abort(&t, &["-qv", "app/Missing.php"], PLAIN, "");
    let h = run(&[&PUN[..], &["--help"]].concat(), &cx.dir, PLAIN);
    assert!(
        h.code == Some(0)
            && h.stdout
                .starts_with("Fix (or check) the layout of the given paths"),
        "{h:?}"
    );
    // Unreadable / unwritable files abort the run (kept outside the snapshotted tree).
    let outside = scratch("out", "perm").join("Fix.php");
    // --bail stops there too.
    for (mode, before, args) in [
        (0o000, &[][..], &["--test"][..]),
        (0o444, &[], &[]),
        (0o000, &["--bail", "app/Clean.php"], &[]),
        (0o444, &["-v"], &[]),
    ] {
        let pair = |cmd: &[&str]| {
            reset(&cx.dir, &t);
            if cmd[0] == PUN[0] {
                migrate(&cx.dir);
            }
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
        // Pint prints the progress of the files before the aborting one, then the box; Pun its run error alone,
        // on stderr, and exits 2.
        let got = pair(&PUN);
        // A `--format` run keeps Pint's box (no progress there), and exits 2.
        fn json<'a>(cmd: &[&'a str]) -> Vec<&'a str> {
            [cmd, &["--format=json"]].concat()
        }
        let mut want_box = pair(&json(&[&which("php"), cx.oracle.to_str().unwrap()]));
        want_box.stdout = normalise_trace(&want_box.stdout, &cx.oracle);
        let got_box = pair(&json(&PUN));
        assert_eq!(
            (got_box.stdout, got_box.code),
            (want_box.stdout, Some(2)),
            "mode {mode:o}"
        );
        let message = match mode {
            0o444 => format!(
                "Cannot write to file \"{}\" as it is not writable.",
                outside.display()
            ),
            _ => format!("Failed to read content from \"{}\".", outside.display()),
        };
        assert!(want.stdout.contains(&message[..40]), "{}", want.stdout);
        assert_eq!(
            (got.stdout.as_str(), got.stderr, got.code, got.files),
            ("", format!("✖ {message}\n"), Some(2), want.files),
            "mode {mode:o}"
        );
    }
}

/// Gitlab entries for rules GitlabReporter's FixerFactory doesn't know (Pun's own lint rules) use its "custom rule"
/// wording. Hand-derived from GitlabReporter::generate (Pint has no such rules to run).
#[test]
fn gitlab_custom_rule() {
    let dir = scratch("out", "gitlab");
    reset(
        &dir,
        &[
            (
                "pun.toml",
                "[format]\npreset = \"empty\"\n[lint.rules]\n\"pest/no-only\" = \"error\"\n",
            ),
            ("t.php", "<?php\ntest('a', fn () => 1)->only();\n"),
        ],
    );
    let r = run(
        &[env!("CARGO_BIN_EXE_pun"), "lint", "--format=gitlab"],
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
        "PHP CS Fixer 3.95.25 Adalbertus by Fabien Potencier, Dariusz Ruminski and contributors.\nCheck performed with a custom rule."
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
    // Missing packages (npm, then pnpm by lock file), with and without colours, and via --blade. Pun names the
    // rule and how to install them (hand-derived from Pint's message and its package-manager choice).
    let packages = "prettier, prettier-plugin-blade, prettier-plugin-tailwindcss";
    let needs = format!("✖ `format/laravel/blade` needs {packages} in the project");
    let install = |m: &str| {
        format!("  install them: {m} prettier prettier-plugin-blade prettier-plugin-tailwindcss\n")
    };
    let r = cx.same_abort(
        &t,
        &["--test"],
        PLAIN,
        &format!("{needs}\n{}", install("npm install -D")),
    );
    assert!(r.files.contains_key("app/Fix.php"));
    cx.same_abort(
        &t,
        &["--ansi"],
        PLAIN,
        &format!("\x1b[31m{needs}\x1b[0m\n{}", install("npm install -D")),
    );
    let r = cx.same_abort(&t, &["--test", "--format=json"], PLAIN, "");
    assert!(
        r.stdout
            .contains("prettier dependencies to be installed using [npm]"),
        "{r:?}"
    );
    cx.same_trace(&t, &["--test", "-v", "--format=json"]);
    let mut pnpm = t.clone();
    pnpm.push(("pnpm-lock.yaml", ""));
    cx.same_abort(
        &pnpm,
        &["--test"],
        PLAIN,
        &format!("{needs}\n{}", install("pnpm add -D")),
    );
    let mut flag = tree();
    flag.push(("bun.lock", ""));
    // A Bun lock file runs the probe with Bun (Pint 1.32), which would otherwise resolve the packages from its global
    // auto-install cache.
    flag.push(("bunfig.toml", "[install]\nauto = \"disable\"\n"));
    cx.same_abort(
        &flag,
        &["--blade", "--test"],
        PLAIN,
        &format!("{needs}\n{}", install("bun add -d")),
    );
    // Options instead of `true` don't trigger the check: the rule is not configurable, so Pint stops on the options
    // before it, and `pun migrate` stops with the same message.
    let mut opts = tree();
    opts[1] = (
        "pint.json",
        r#"{"preset":"laravel","rules":{"Pint/laravel_blade":{}}}"#,
    );
    reset(&cx.dir, &opts);
    let php = which("php");
    let pint = [
        php.as_str(),
        cx.oracle.to_str().unwrap(),
        "--test",
        "app/Clean.php",
    ];
    let want = run(&pint, &cx.dir, PLAIN);
    let message = "[Pint/laravel_blade] Is not configurable.";
    assert!(
        want.stdout.contains(message) && !want.stdout.contains("prettier") && want.code == Some(1),
        "{want:?}"
    );
    let got = Command::new(PUN[0])
        .arg("migrate")
        .env("PUN_AGENT", "0")
        .current_dir(&cx.dir)
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&got.stderr);
    assert!(
        !got.status.success() && stderr.contains(message),
        "{stderr}"
    );
    // Installed but too old / too new.
    let mut old = t.clone();
    let manifests: Vec<(String, String)> = [
        ("prettier", "2.1.0"),
        ("prettier-plugin-blade", "3.3.9"),
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
    // ^3.3.3 takes 3.3.9; ^0.8.1 stops before 0.9.0.
    cx.same_abort(&old, &["--test"], PLAIN, "✖ `format/laravel/blade` needs newer prettier packages: prettier 2.1.0 (needs ^3.9.6), prettier-plugin-tailwindcss 0.9.1 (needs ^0.8.1)\n  update them: yarn add -D prettier@^3.9.6 prettier-plugin-tailwindcss@^0.8.1\n");
    cx.same_abort(&old, &["--test", "--format=json"], PLAIN, "");
    // No Node.js (or Bun) on PATH.
    let bin = scratch("out", "nonode");
    for b in ["php", "sh"] {
        std::os::unix::fs::symlink(which(b), bin.join(b)).unwrap();
    }
    let no_node = Opts {
        path: Some(bin.to_str().unwrap()),
        ..PLAIN
    };
    cx.same_abort(&t, &["--test"], no_node, "✖ `format/laravel/blade` needs a JavaScript runtime (Node.js or Bun), and there is none on PATH\n  install Node.js or Bun, or turn `format/laravel/blade` off in pun.toml\n");
    cx.same_abort(&t, &["--test", "--format=json"], no_node, "");
}

/// Without a `php` on PATH Pun can't ask PHP for its wording: it falls back to its own parser's message (and
/// line), in the same problem block. Hand-derived: Pint itself always has PHP.
#[test]
fn syntax_error_without_php() {
    let dir = scratch("out", "nophp");
    reset(
        &dir,
        &[
            (
                "pun.toml",
                "[format]\npreset = \"empty\"\n[format.rules]\n\"arrays/syntax\" = true\n",
            ),
            ("a.php", "<?php\nfunction (\n"),
        ],
    );
    let r = run(
        &[&PUN[..], &["--test", "-v"]].concat(),
        &dir,
        Opts {
            path: Some("/nonexistent"),
            ..PLAIN
        },
    );
    // PHP's own wording ("Unclosed '(' on line 2" on line 3) without the "Parse error: " and the final "on line 3.";
    // line 3 is the empty line after the last line break, so the frame marks its first column.
    let (head, frame) = r.stdout.split_once('\n').unwrap();
    assert_eq!(
        head, " ✖ syntax-error  Unclosed '(' on line 2",
        "{}",
        r.stdout
    );
    assert!(
        frame.starts_with("  ╭─ a.php:3:1\n 2│ function (\n 3│\n  │ ━\n  ╰─\n"),
        "{}",
        r.stdout
    );
    assert_eq!(r.code, Some(1));
}

/// A fixer exception (combine_consecutive_unsets' "Must not add empty token", a panic in Pun) fails only that file:
/// the message, the other files still fixed, exit 2 (Pint: 1; Pun failed, output.md §2), no panic output. With a syntax error too, the reports agree.
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
        let r = cx.same_exit(&CHECK_FIX, &t, args, PLAIN, Some(2));
        assert_eq!(r.code, Some(2), "{args:?}");
    }
    t.push(("app/Bad.php", "<?php\nfunction (\n"));
    cx.same_exit(&CHECK_FIX, &t, &["--format=txt", "--test"], PLAIN, Some(2));
}

/// GitHub annotations (output.md Build 4), one workflow command per line in PHPStan's GithubErrorFormatter shape
/// (`::error file=…,line=…,col=…::message`), hand-derived: the path's `,` and `:` escaped as GitHub's escapeProperty
/// does (`%2C`, `%3A`), the first changed line of each file, `col=0` for a whole file, the rule IDs as the title;
/// syntax errors at PHP's line and the first column; fixed files print nothing.
#[test]
fn github_annotations() {
    let dir = scratch("out", "github");
    reset(
        &dir,
        &[
            (
                "pun.toml",
                "[format]\npreset = \"empty\"\n[format.rules]\n\"arrays/syntax\" = true\n[lint.rules]\n\"style/no-alias-function\" = \"error\"\n\"dead-code/no-self-assignment\" = { fix = \"off\" }\n",
            ),
            ("app/a,b:c.php", "<?php\n\n$a = array(1);\n"),
            ("app/Clean.php", "<?php\n\n$a = [1];\n"),
            ("app/Bad.php", "<?php\nfunction (\n"),
            ("app/L.php", "<?php\n$x = 1;\n$n = sizeof([]);\n$x = $x;\n"),
        ],
    );
    let pun = |args: &[&str], env: &[(&str, &str)]| {
        let r = run(
            &[&[env!("CARGO_BIN_EXE_pun")][..], args].concat(),
            &dir,
            Opts { env, ..PLAIN },
        );
        (r.stdout, r.code)
    };
    let bad = "::error file=app/Bad.php,line=3,col=1,title=syntax-error::Unclosed '(' on line 2\n";
    let format = "::error file=app/a%2Cb%3Ac.php,line=3,col=0,title=format/arrays/syntax::File needs formatting\n";
    let both = "lint/style/no-alias-function, lint/dead-code/no-self-assignment";
    let lint = format!(
        "::error file=app/L.php,line=3,col=0,title={}::{both} can fix this file\n",
        both.replace(',', "%2C")
    );
    assert_eq!(
        pun(&["format", "--test", "--format=github"], &[]),
        (format!("{bad}{format}"), Some(1))
    );
    assert_eq!(
        pun(&["lint", "--format=github"], &[]),
        (format!("{bad}{lint}"), Some(1))
    );
    assert_eq!(
        pun(&["check", "--format=github"], &[]),
        (format!("{bad}{lint}{format}"), Some(1))
    );
    // `--fix`: what it applied prints nothing, the fix it leaves (`fix = "off"`) is an error at its line.
    let left = "::error file=app/L.php,line=4,col=0,title=lint/dead-code/no-self-assignment::lint/dead-code/no-self-assignment can fix this file\n";
    let (out, code) = pun(&["lint", "--fix", "--format=github"], &[]);
    assert_eq!((out, code), (format!("{bad}{left}"), Some(1)));
    // Written fixes need nothing in CI: only the syntax error is left.
    assert_eq!(
        pun(&["format", "--format=github"], &[]),
        (bad.to_string(), Some(1))
    );
    // Under GitHub Actions the human output comes first, then the same lines; not for other values, `-q` or a
    // `--format`.
    let (out, code) = pun(
        &["format", "--test", "app/Bad.php"],
        &[("GITHUB_ACTIONS", "true")],
    );
    assert!(
        out.starts_with("\n ✖ syntax-error") || out.starts_with(" ✖ syntax-error"),
        "{out}"
    );
    assert!(out.ends_with(&format!("s\n{bad}")), "{out}");
    assert_eq!(code, Some(1));
    for (args, env) in [
        (&["format", "--test", "app/Bad.php"][..], &[][..]),
        (
            &["format", "--test", "app/Bad.php"],
            &[("GITHUB_ACTIONS", "1")],
        ),
        (
            &["format", "--test", "-q", "app/Bad.php"],
            &[("GITHUB_ACTIONS", "true")],
        ),
        (
            &["format", "--test", "--format=json", "app/Bad.php"],
            &[("GITHUB_ACTIONS", "true")],
        ),
    ] {
        let (out, code) = pun(args, env);
        assert!(!out.contains("::error"), "{args:?} {env:?}: {out}");
        assert_eq!(code, Some(1));
    }
    // A run error is `::error ::<message>`: alone under `--format=github`, after Pun's run error (stderr) under
    // GitHub Actions.
    let missing = "The path \"app/Missing.php\" is not readable.";
    assert_eq!(
        pun(&["lint", "--format=github", "app/Missing.php"], &[]),
        (format!("::error ::{missing}\n"), Some(2))
    );
    let r = run(
        &[env!("CARGO_BIN_EXE_pun"), "format", "app/Missing.php"],
        &dir,
        Opts {
            env: &[("GITHUB_ACTIONS", "true")],
            ..PLAIN
        },
    );
    assert_eq!(
        (r.stdout, r.stderr, r.code),
        (
            format!("::error ::{missing}\n"),
            format!("✖ {missing}\n"),
            Some(2)
        )
    );
}

/// A fixer that throws (combine_consecutive_unsets' Tokens::insertSlices "Must not add empty token to collection.", a
/// panic in Pun) is annotated at
/// its file as `fixer-error`, and Pun exits 2.
#[test]
fn github_fixer_error() {
    let dir = scratch("out", "github-crash");
    reset(
        &dir,
        &[
            (
                "pun.toml",
                "[format]\npreset = \"empty\"\n[lint.rules]\n\"control-flow/prefer-combined-unset\" = \"error\"\n",
            ),
            (
                "app/U.php",
                "<?php unset($a ); unset( $b );\nunset($x, ); unset( $y);",
            ),
        ],
    );
    let r = run(
        &[
            env!("CARGO_BIN_EXE_pun"),
            "check",
            "--fix",
            "--format=github",
        ],
        &dir,
        PLAIN,
    );
    assert_eq!(
        (r.stdout.as_str(), r.code),
        (
            "::error file=app/U.php,line=1,col=0,title=fixer-error::Must not add empty token to collection.\n",
            Some(2)
        ),
        "{}",
        r.stderr
    );
    // Agent mode names it the same way, at the file.
    let r = run(
        &[env!("CARGO_BIN_EXE_pun"), "check", "--fix"],
        &dir,
        Opts {
            env: &[("PUN_AGENT", "1")],
            ..PLAIN
        },
    );
    assert!(
        r.stdout
            .ends_with("\napp/U.php fixer-error Must not add empty token to collection.\n"),
        "{}",
        r.stdout
    );
    assert_eq!(r.code, Some(2));
}

/// Run errors exit 2 in every command (output.md §2) and print Pun's run error (output.md §1: one red line, the
/// hint under it) on stderr, with and without colour; findings still exit 1 and a clean run 0.
#[test]
fn run_errors_exit_2() {
    let dir = scratch("out", "runerr");
    reset(&dir, &[("app/A.php", "<?php\n$a = array(1);\n")]);
    let pun = |args: &[&str]| {
        let r = run(
            &[&[env!("CARGO_BIN_EXE_pun")][..], args].concat(),
            &dir,
            PLAIN,
        );
        (r.stdout, r.stderr, r.code)
    };
    let red = |line: &str| format!("\x1b[31m{line}\x1b[0m\n");
    // No pun.toml: the config error, for each command.
    let none = "✖ No pun.toml found. Run `pun migrate`, or `pun init` for a new project.";
    for cmd in ["format", "lint", "check"] {
        assert_eq!(
            pun(&[cmd, "--color=never"]),
            (String::new(), format!("{none}\n"), Some(2)),
            "{cmd}"
        );
        assert_eq!(
            pun(&[cmd, "--color=always"]),
            (String::new(), red(none), Some(2)),
            "{cmd}"
        );
    }
    for args in [&["config"][..], &["list-files"]] {
        assert_eq!(pun(args), (String::new(), format!("{none}\n"), Some(2)));
    }
    // A bad `--format` (Pint's box, kept for machine formats) and `--output-format`: the files are still checked.
    std::fs::write(
        dir.join("pun.toml"),
        "[format]\npreset = \"empty\"\n[format.rules]\n\"arrays/syntax\" = true\n",
    )
    .unwrap();
    let (out, err, code) = pun(&["format", "--test", "--format=nope"]);
    assert!(
        out.contains("  Format [nope] is not supported.  \n"),
        "{out}"
    );
    assert_eq!((err.as_str(), code), ("", Some(2)));
    let formats = "  use json, txt, agent, checkstyle, junit, xml, gitlab or github\n";
    let (out, err, code) = pun(&[
        "format",
        "--test",
        "--output-format=nope",
        "--output-to-file=r",
        "--color=never",
    ]);
    assert!(out.starts_with(" ✖ app/A.php"), "{out}");
    assert_eq!(
        (err, code),
        (
            format!("✖ Format [nope] is not supported.\n{formats}"),
            Some(2)
        )
    );
    // Findings exit 1, a clean run 0.
    assert_eq!(pun(&["format", "--test"]).2, Some(1));
    assert_eq!(pun(&["format"]).2, Some(0));
    assert_eq!(pun(&["format", "--test"]).2, Some(0));
    // Other commands: migrate (nothing to migrate from), list-files (a bad `--format`), explain (an unknown rule).
    assert_eq!(
        pun(&["migrate", "--from=pint"]),
        (
            String::new(),
            "✖ --from pint: no pint.json found\n".into(),
            Some(2)
        )
    );
    assert_eq!(
        pun(&["list-files", "--format=txt"]),
        (
            String::new(),
            "✖ --format=txt: expected `json`\n".into(),
            Some(2)
        )
    );
    assert_eq!(
        pun(&["explain", "nope"]),
        (
            String::new(),
            "✖ `nope` is not a Pun rule or an upstream rule name\n".into(),
            Some(2)
        )
    );
}
