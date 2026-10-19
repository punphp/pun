//! `pun lint` and the lint half of `pun check` (context/plan/lint.md): levels, overrides, the command-line levels,
//! warnings and exit codes, `@pun-ignore` comments, `--fix`. Expected values are derived by hand from each rule's
//! definition (Mago's `no-eval`/`no-global` report the construct; Rector's `RemoveDoubleNegationRector`-style
//! `no-double-negation` rewrites `!!$x` to `(bool) $x`) and from rules.md §4 for the comments.
mod common;

use common::{PUN, clean_env, project, read};
use std::path::Path;
use std::process::Command;

const TOML: &str = r#"[format]
preset = "empty"

[lint.rules]
"security/no-eval" = "error"
"security/no-global" = "warn"
"complexity/no-double-negation" = "error"
"comments/no-unused-ignore" = "error"

[[lint.overrides]]
files = ["src/legacy/**"]
rules = { "security/no-eval" = "off" }
"#;

const A: &str = "<?php\n\nfunction f()\n{\n    global $y;\n    eval('1;');\n}\n\n$a = !!$b; // @pun-ignore lint/security/no-eval\n";
const LEGACY: &str = "<?php\n\neval('2;');\n";

/// `pun <args>` in `dir` under agent mode: (the lines after the summary, the summary up to its timing, exit code).
fn agent(dir: &Path, args: &[&str]) -> (Vec<String>, String, Option<i32>) {
    let out = clean_env(&mut Command::new(PUN), &["PATH", "HOME"])
        .current_dir(dir)
        .env("PUN_AGENT", "1")
        .args(args)
        .args(["--no-cache"])
        .output()
        .unwrap();
    let stdout = String::from_utf8(out.stdout).unwrap();
    let mut lines = stdout.lines().map(str::to_string);
    let summary = lines.next().unwrap_or_default();
    let summary = summary.split(" (").next().unwrap().to_string();
    (lines.collect(), summary, out.status.code())
}

fn sample() -> std::path::PathBuf {
    project(
        "lint",
        "sample",
        &[
            ("pun.toml", TOML),
            ("src/a.php", A),
            ("src/legacy/b.php", LEGACY),
        ],
    )
}

#[test]
fn levels_overrides_and_ignores() {
    let d = sample();
    let (lines, summary, code) = agent(&d, &["lint"]);
    // `src/legacy/b.php`'s eval is off by the override; the ignore comment covers nothing on its line.
    assert_eq!(summary, "pun lint: fail, 3 errors, 1 warning");
    assert_eq!(
        lines,
        [
            "src/a.php:5:5 lint/security/no-global [warn] Unsafe use of `global` keyword.",
            "src/a.php:6:5 lint/security/no-eval Unsafe use of `eval` construct.",
            "src/a.php:9:6 lint/complexity/no-double-negation [fix:safe] Use a `(bool)` cast instead of `!!`.",
            "src/a.php:9:27 lint/comments/no-unused-ignore [fix:safe] `lint/security/no-eval` suppresses no problem here",
        ]
    );
    assert_eq!(code, Some(1));
}

#[test]
fn command_line_levels_apply_in_order() {
    let d = sample();
    // Everything in `security` off, then `no-global` back on as an error; the double negation is allowed too. The
    // ignore comment names `no-eval`, which is off, so it isn't judged unused.
    let (lines, _, code) = agent(
        &d,
        &[
            "lint",
            "-A",
            "security",
            "-D",
            "security/no-global",
            "-A",
            "complexity/no-double-negation",
        ],
    );
    assert_eq!(
        lines,
        ["src/a.php:5:5 lint/security/no-global Unsafe use of `global` keyword.",]
    );
    assert_eq!(code, Some(1));
    // An unknown name is a run error.
    let (lines, _, code) = agent(&d, &["lint", "-D", "nope/nothing"]);
    assert_eq!(lines, Vec::<String>::new());
    assert_eq!(code, Some(2));
}

#[test]
fn warnings_fail_only_when_asked() {
    let d = project(
        "lint",
        "warnings",
        &[
            (
                "pun.toml",
                "[lint.rules]\n\"security/no-global\" = \"warn\"\n",
            ),
            ("a.php", "<?php\n\nfunction f()\n{\n    global $y;\n}\n"),
        ],
    );
    assert_eq!(agent(&d, &["lint"]).2, Some(0));
    assert_eq!(agent(&d, &["lint", "--deny-warnings"]).2, Some(1));
    assert_eq!(agent(&d, &["lint", "--max-warnings", "1"]).2, Some(0));
    assert_eq!(agent(&d, &["lint", "--max-warnings", "0"]).2, Some(1));
}

#[test]
fn fix_applies_safe_fixes_and_deletes_unused_ignores() {
    let d = sample();
    let (lines, summary, code) = agent(&d, &["lint", "--fix"]);
    assert_eq!(
        read(&d, "src/a.php"),
        "<?php\n\nfunction f()\n{\n    global $y;\n    eval('1;');\n}\n\n$a = (bool) $b;\n"
    );
    assert_eq!(read(&d, "src/legacy/b.php"), LEGACY);
    assert_eq!(summary, "pun lint: fail, 1 error, 1 file fixed, 1 warning");
    assert!(lines.contains(
        &"src/a.php:6:5 lint/security/no-eval Unsafe use of `eval` construct.".to_string()
    ));
    assert_eq!(code, Some(1));
}

#[test]
fn fix_off_keeps_a_rule_report_only() {
    let toml = TOML.replace(
        "\"complexity/no-double-negation\" = \"error\"",
        "\"complexity/no-double-negation\" = { level = \"error\", fix = \"off\" }",
    );
    let d = project(
        "lint",
        "fix-off",
        &[("pun.toml", &toml), ("a.php", "<?php\n\n$a = !!$b;\n")],
    );
    let (lines, _, code) = agent(&d, &["lint", "--fix"]);
    assert_eq!(read(&d, "a.php"), "<?php\n\n$a = !!$b;\n");
    assert_eq!(
        lines,
        ["a.php:3:6 lint/complexity/no-double-negation Use a `(bool)` cast instead of `!!`."]
    );
    assert_eq!(code, Some(1));
}

#[test]
fn ignore_comment_forms() {
    let source = "<?php\n\n// @pun-ignore-next-line security/no-eval\neval('1;');\neval('2;'); // @pun-ignore lint/security\n// @pun-ignore\neval('3;');\n";
    let d = project("lint", "ignores", &[("pun.toml", TOML), ("a.php", source)]);
    let (lines, _, code) = agent(&d, &["lint"]);
    assert_eq!(
        lines,
        [
            "a.php:6:4 invalid-ignore-comment `@pun-ignore` needs one or more rule IDs",
            "a.php:7:1 lint/security/no-eval Unsafe use of `eval` construct.",
        ]
    );
    assert_eq!(code, Some(1));
}

#[test]
fn lint_skips_blade_and_reports_syntax_errors() {
    let d = project(
        "lint",
        "files",
        &[
            ("pun.toml", TOML),
            ("v.blade.php", "<?php eval('1;'); ?>\n"),
            ("storage/s.php", "<?php\neval('1;');\n"),
            ("bad.php", "<?php\nfunction (\n"),
        ],
    );
    let (lines, summary, code) = agent(&d, &["lint"]);
    assert_eq!(summary, "pun lint: fail, 1 error");
    assert_eq!(lines.len(), 1, "{lines:?}");
    assert!(
        lines[0].starts_with("bad.php:") && lines[0].contains(" syntax-error "),
        "{lines:?}"
    );
    assert_eq!(code, Some(1));
}

#[test]
fn native_rule_options_reach_the_rule() {
    // `no-global` has no options (Mago's rule takes none), so options for it are a config error that names it.
    let d = project(
        "lint",
        "options",
        &[
            (
                "pun.toml",
                "[format]\npreset = \"empty\"\n[lint.rules]\n\"security/no-global\" = { level = \"error\", allow = [\"x\"] }\n",
            ),
            ("a.php", "<?php\n"),
        ],
    );
    let (lines, summary, code) = agent(&d, &["lint"]);
    assert_eq!(
        (lines, summary.as_str(), code),
        (
            vec![],
            "pun lint: error: pun.toml: `lint/security/no-global` takes no options",
            Some(2)
        )
    );
    // Overrides change levels only: one instance of a rule lints every file.
    std::fs::write(
        d.join("pun.toml"),
        "[format]\npreset = \"empty\"\n[lint.rules]\n\"security/no-global\" = \"error\"\n[[lint.overrides]]\nfiles = [\"a.php\"]\nrules = { \"security/no-global\" = { allow = [\"x\"] } }\n",
    )
    .unwrap();
    let (_, summary, code) = agent(&d, &["lint"]);
    assert_eq!(
        (summary.as_str(), code),
        (
            "pun lint: error: pun.toml: `lint/security/no-global` takes options in [lint.rules] only, not in [[lint.overrides]]",
            Some(2)
        )
    );
}

#[test]
fn overrides_set_levels_and_name_only_ported_rules() {
    let toml = "[format]\npreset = \"empty\"\n\n[lint.rules]\n\"security/no-eval\" = \"error\"\n\"security/no-global\" = \"warn\"\n\n[[lint.overrides]]\nfiles = [\"src/legacy/**\"]\nrules = { \"security/no-eval\" = \"warn\" }\n";
    let d = project(
        "lint",
        "override-levels",
        &[
            ("pun.toml", toml),
            ("src/a.php", "<?php\n\nfunction f()\n{\n    global $y;\n}\n"),
            ("src/legacy/b.php", LEGACY),
        ],
    );
    let global = "src/a.php:5:5 lint/security/no-global Unsafe use of `global` keyword.";
    let eval = "src/legacy/b.php:3:1 lint/security/no-eval Unsafe use of `eval` construct.";
    let warn = |l: &str| l.replacen(" Unsafe", " [warn] Unsafe", 1);
    // The override turns the error into a warning for the legacy files.
    assert_eq!(
        agent(&d, &["lint"]),
        (
            vec![warn(global), warn(eval)],
            "pun lint: pass, 2 warnings".into(),
            Some(0)
        )
    );
    // `-W`/`-D` win over both, with or without the `lint/` prefix.
    assert_eq!(
        agent(
            &d,
            &[
                "lint",
                "-W",
                "lint/security/no-global",
                "-D",
                "security/no-eval"
            ]
        ),
        (
            vec![warn(global), eval.into()],
            "pun lint: fail, 1 error, 1 warning".into(),
            Some(1)
        )
    );
    // An override that turns on a rule Pun has not ported is a config error naming it.
    std::fs::write(
        d.join("pun.toml"),
        format!("{toml}\n[[lint.overrides]]\nfiles = [\"x\"]\nrules = {{ \"complexity/no-boolean-flag-parameter\" = \"error\" }}\n"),
    )
    .unwrap();
    let (lines, summary, code) = agent(&d, &["lint"]);
    assert_eq!((lines, code), (vec![], Some(2)));
    assert!(
        summary.ends_with(
            "rule not supported by Pun yet: `lint/complexity/no-boolean-flag-parameter`"
        ),
        "{summary}"
    );
}

/// (stdout, exit code) of `pun lint --no-cache args` in `dir`, outside agent mode, with `env`.
fn plain(dir: &Path, args: &[&str], env: &[(&str, &str)]) -> (String, Option<i32>) {
    let out = clean_env(&mut Command::new(PUN), &["PATH", "HOME"])
        .current_dir(dir)
        .env("PUN_AGENT", "0")
        .envs(env.iter().copied())
        .args(["lint", "--no-cache"])
        .args(args)
        .output()
        .unwrap();
    (String::from_utf8(out.stdout).unwrap(), out.status.code())
}

/// Lint problems in every report: the agent line's fix safety, PHP-CS-Fixer's json (each rule once among the
/// file's fixers), the github format and the annotations after human output (`::error` workflow commands at the
/// problem's line and column).
#[test]
fn problems_in_every_report() {
    let toml = "[format]\npreset = \"empty\"\n\n[lint.rules]\n\"strings/no-single-argument-sprintf\" = \"error\"\n\"security/no-eval\" = \"error\"\n";
    let d = project(
        "lint",
        "reports",
        &[
            ("pun.toml", toml),
            (
                "a.php",
                "<?php\n$n = sprintf('x');\neval('1;');\neval('2;');\n",
            ),
        ],
    );
    let sprintf = "`sprintf()` with only a format does nothing; use the format itself.";
    let (lines, _, code) = agent(&d, &["lint"]);
    assert_eq!(
        lines,
        [
            format!("a.php:2:6 lint/strings/no-single-argument-sprintf [fix:unsafe] {sprintf}"),
            "a.php:3:1 lint/security/no-eval Unsafe use of `eval` construct.".into(),
            "a.php:4:1 lint/security/no-eval Unsafe use of `eval` construct.".into(),
        ]
    );
    assert_eq!(code, Some(1));
    assert_eq!(
        plain(&d, &["-v", "--format=json"], &[]),
        (
            r#"{"about":"PHP CS Fixer 3.95.17 Adalbertus by Fabien Potencier, Dariusz Ruminski and contributors.","files":[{"name":"a.php","appliedFixers":["lint\/strings\/no-single-argument-sprintf","lint\/security\/no-eval"]}],"time":{"total":0},"memory":0}"#.into(),
            Some(1)
        )
    );
    let annotations = format!(
        "::error file=a.php,line=2,col=6,title=lint/strings/no-single-argument-sprintf::{sprintf}\n::error file=a.php,line=3,col=1,title=lint/security/no-eval::Unsafe use of `eval` construct.\n::error file=a.php,line=4,col=1,title=lint/security/no-eval::Unsafe use of `eval` construct.\n"
    );
    assert_eq!(
        plain(&d, &["--format=github"], &[]),
        (annotations.clone(), Some(1))
    );
    let (out, code) = plain(&d, &["--color=never"], &[("GITHUB_ACTIONS", "true")]);
    assert!(out.ends_with(&annotations), "{out}");
    assert_eq!(code, Some(1));
}

/// A token lint rule (PHP-CS-Fixer's `no_alias_functions`, `style/no-alias-function`) beside native ones: an ignore
/// naming it is not judged unused (its problems are per file, not per line), the native problems' positions are in
/// the code after its fix (as the snippet shows it), and at `warn` it fails the run only with `--deny-warnings`.
#[test]
fn token_rules_beside_native_ones() {
    let toml = "[format]\npreset = \"empty\"\n\n[lint.rules]\n\"style/no-alias-function\" = \"warn\"\n\"comments/no-unused-ignore\" = \"error\"\n\"security/no-eval\" = \"error\"\n";
    let ignore = "<?php\n$a = 1; // @pun-ignore lint/style/no-alias-function\n";
    let d = project(
        "lint",
        "token",
        &[
            ("pun.toml", toml),
            ("a.php", &format!("{ignore}$n = sizeof($a); eval('1;');\n")),
        ],
    );
    let alias = "a.php lint/style/no-alias-function [fix:unsafe] Master functions shall be used instead of aliases.";
    // `$n = count($a); eval` puts eval at column 17 (at 18 before the fix).
    let (lines, _, code) = agent(&d, &["lint"]);
    assert_eq!(
        lines,
        [
            "a.php:3:17 lint/security/no-eval Unsafe use of `eval` construct.",
            alias
        ]
    );
    assert_eq!(code, Some(1));
    std::fs::write(d.join("a.php"), format!("{ignore}$n = sizeof($a);\n")).unwrap();
    assert_eq!(agent(&d, &["lint"]).2, Some(0));
    assert_eq!(agent(&d, &["lint", "--deny-warnings"]).2, Some(1));
}

/// `--dirty` picks the git-changed files (untracked ones included), narrowed to what `pun lint` checks: the Blade
/// template is changed but never linted.
#[test]
fn dirty_narrows_to_the_lint_files() {
    let eval = "<?php\n\neval('1;');\n";
    let d = project(
        "lint",
        "dirty",
        &[("pun.toml", TOML), ("a.php", eval), ("b.php", eval)],
    );
    let git = |args: &[&str]| {
        let ok = Command::new("git")
            .args([
                "-c",
                "user.name=t",
                "-c",
                "user.email=t@t",
                "-c",
                "commit.gpgsign=false",
            ])
            .args(args)
            .current_dir(&d)
            .output()
            .unwrap()
            .status
            .success();
        assert!(ok, "git {args:?}");
    };
    git(&["init", "-q"]);
    git(&["add", "."]);
    git(&["commit", "-qm", "init"]);
    std::fs::write(d.join("b.php"), format!("{eval}\n")).unwrap();
    std::fs::write(d.join("v.blade.php"), "<?php eval('1;'); ?>\n").unwrap();
    let (lines, _, code) = agent(&d, &["lint", "--dirty"]);
    assert_eq!(
        lines,
        ["b.php:3:1 lint/security/no-eval Unsafe use of `eval` construct."]
    );
    assert_eq!(code, Some(1));
}
