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

/// A 20 000-term concatenation nests as deep as its terms in the syntax tree; the parser and the lint walk recurse
/// that deep, which overflows a 2 MiB worker stack (it aborts the run).
#[test]
fn lint_survives_a_deep_expression() {
    let deep = format!("<?php\n\n$a = 1{};\neval('1;');\n", " . 'a'".repeat(20_000));
    let d = project("lint", "deep", &[("pun.toml", TOML), ("a.php", &deep)]);
    let (lines, summary, code) = agent(&d, &["lint"]);
    assert_eq!(summary, "pun lint: fail, 1 error");
    assert!(lines[0].starts_with("a.php:4:"), "{lines:?}");
    assert_eq!(code, Some(1));
}

/// The types rules see classes in the project's other files, also when only one file is named: Rector 2.6.1's
/// StringReturnTypeFromStrictScalarReturnsRector adds `: string` to `Child::prefix()` once it knows `Base` (in another
/// file) has no `prefix()` to stay compatible with.
#[test]
fn types_rules_see_the_project_classes() {
    let d = project(
        "lint",
        "codebase",
        &[
            (
                "pun.toml",
                "[lint.rules]\n\"types/add-string-return-type-from-scalar\" = \"error\"\n",
            ),
            (
                "src/Base.php",
                "<?php\n\nnamespace App;\n\nabstract class Base\n{\n}\n",
            ),
            (
                "src/Child.php",
                "<?php\n\nnamespace App;\n\nclass Child extends Base\n{\n    public function prefix()\n    {\n        return '';\n    }\n}\n",
            ),
        ],
    );
    let (lines, summary, code) = agent(&d, &["lint", "src/Child.php"]);
    assert_eq!(summary, "pun lint: fail, 1 error");
    assert!(lines[0].starts_with("src/Child.php:7:"), "{lines:?}");
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
