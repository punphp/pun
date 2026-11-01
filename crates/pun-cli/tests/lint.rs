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
            // Without `comments/no-unused-ignore` on, an ignore that covers nothing is fine.
            (
                "a.php",
                "<?php\n\nfunction f()\n{\n    global $y; // @pun-ignore lint/security/no-eval\n}\n",
            ),
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
/// file) has no `prefix()` to stay compatible with. A class outside the project stays unknown:
/// AddArrowFunctionReturnTypeRector leaves `fn () => Job::go()->job` alone (Rector run on these files), where Mago
/// types the access on `?Unknown` as `null`. A class Composer autoloads is found on demand: `Old`'s psr-0 parent in
/// vendor/ has no `prefix()`, so Rector (with the project's vendor/autoload.php) adds `: string` there too.
#[test]
fn types_rules_see_the_project_classes() {
    let d = project(
        "lint",
        "codebase",
        &[
            (
                "pun.toml",
                "[lint.rules]\n\"types/add-string-return-type-from-scalar\" = \"error\"\n\"types/add-arrow-function-return-type\" = \"error\"\n",
            ),
            (
                "src/Job.php",
                "<?php\n\nnamespace App;\n\nuse Vendor\\PendingDispatch;\n\nclass Job\n{\n    public static function go(): ?PendingDispatch\n    {\n        return null;\n    }\n}\n",
            ),
            (
                "src/T.php",
                "<?php\n\nuse App\\Job;\n\n$x = fn () => Job::go()->job;\n",
            ),
            (
                "src/Base.php",
                "<?php\n\nnamespace App;\n\nabstract class Base\n{\n}\n",
            ),
            (
                "src/Child.php",
                "<?php\n\nnamespace App;\n\nclass Child extends Base\n{\n    public function prefix()\n    {\n        return '';\n    }\n}\n",
            ),
            (
                "composer.json",
                "{\"autoload\": {\"psr-0\": {\"Legacy_\": \"vendor/legacy/lib/\"}}}\n",
            ),
            (
                "vendor/legacy/lib/Legacy/Base.php",
                "<?php\n\nclass Legacy_Base\n{\n}\n",
            ),
            (
                "src/Old.php",
                "<?php\n\nnamespace App;\n\nclass Old extends \\Legacy_Base\n{\n    public function prefix()\n    {\n        return '';\n    }\n}\n",
            ),
            (
                "vendor/mapped/Base.php",
                "<?php\n\nnamespace Mapped;\n\nclass Base\n{\n}\n",
            ),
            ("lib/LocalBase.php", "<?php\n\nclass Local_Base\n{\n}\n"),
            (
                "src/FromMap.php",
                "<?php\n\nnamespace App;\n\nclass FromMap extends \\Mapped\\Base\n{\n    public function prefix()\n    {\n        return '';\n    }\n}\n",
            ),
            (
                "src/FromLib.php",
                "<?php\n\nnamespace App;\n\nclass FromLib extends \\Local_Base\n{\n    public function prefix()\n    {\n        return '';\n    }\n}\n",
            ),
        ],
    );
    // `composer dump-autoload` of `{"autoload": {"classmap": ["lib/", "vendor/mapped/"]}}`, with symfony/cache's
    // class named by the byte 0xA9; Rector types both `prefix()` methods
    std::fs::create_dir_all(d.join("vendor/composer")).unwrap();
    std::fs::write(
        d.join("vendor/composer/autoload_classmap.php"),
        [
            &b"<?php\n\n// autoload_classmap.php @generated by Composer\n\n$vendorDir = dirname(__DIR__);\n$baseDir = dirname($vendorDir);\n\nreturn array(\n    '"[..],
            b"\xa9",
            b"' => $vendorDir . '/symfony/cache/Traits/ValueWrapper.php',\n    'Composer\\\\InstalledVersions' => $vendorDir . '/composer/InstalledVersions.php',\n    'Local_Base' => $baseDir . '/lib/LocalBase.php',\n    'Mapped\\\\Base' => $vendorDir . '/mapped/Base.php',\n);\n",
        ]
        .concat(),
    )
    .unwrap();
    let (lines, summary, code) = agent(
        &d,
        &[
            "lint",
            "src/Child.php",
            "src/T.php",
            "src/Old.php",
            "src/FromMap.php",
            "src/FromLib.php",
        ],
    );
    assert_eq!(summary, "pun lint: fail, 4 errors");
    assert!(lines[0].starts_with("src/Child.php:7:"), "{lines:?}");
    assert!(lines[1].starts_with("src/FromLib.php:7:"), "{lines:?}");
    assert!(lines[2].starts_with("src/FromMap.php:7:"), "{lines:?}");
    assert!(lines[3].starts_with("src/Old.php:7:"), "{lines:?}");
    assert_eq!(code, Some(1));
}

/// Every rule that reads facts sees the project's other files, not only the `types` rules: Rector 2.6.1's
/// MakeInheritedMethodVisibilitySameAsParentRector (run on these two files) makes `Child::run()` protected like the
/// `Base::run()` it overrides, from another file.
#[test]
fn fact_rules_see_the_project_classes() {
    let child = "<?php\n\nnamespace App;\n\nclass Child extends Base\n{\n    public function run()\n    {\n    }\n}\n";
    let d = project(
        "lint",
        "parent",
        &[
            (
                "pun.toml",
                "[lint.rules]\n\"style/require-parent-visibility\" = \"error\"\n",
            ),
            (
                "src/Base.php",
                "<?php\n\nnamespace App;\n\nclass Base\n{\n    protected function run()\n    {\n    }\n}\n",
            ),
            ("src/Child.php", child),
        ],
    );
    let (_, code) = plain(&d, &["--fix", "src/Child.php"], &[]);
    assert_eq!(
        read(&d, "src/Child.php"),
        child.replace("public function run()", "protected function run()")
    );
    assert_eq!(code, Some(0));
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
    // Pun's github format also goes to a report file.
    assert_eq!(
        plain(
            &d,
            &["--output-to-file=r.txt", "--output-format=github", "-q"],
            &[]
        )
        .1,
        Some(1)
    );
    assert_eq!(read(&d, "r.txt"), annotations);
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
    // The unused-ignore check itself isn't judged either.
    let ignore = "<?php\n$a = 1; // @pun-ignore lint/style/no-alias-function\n$b = 2; // @pun-ignore lint/comments/no-unused-ignore\n";
    let d = project(
        "lint",
        "token",
        &[
            ("pun.toml", toml),
            ("a.php", &format!("{ignore}$n = sizeof($a); eval('1;');\n")),
        ],
    );
    let alias = "a.php lint/style/no-alias-function [fix:unsafe] Master functions shall be used instead of aliases.";
    // `$n = count($a); eval` puts eval at column 17 (at 18 before the fix), on line 4.
    let (lines, _, code) = agent(&d, &["lint"]);
    assert_eq!(
        lines,
        [
            "a.php:4:17 lint/security/no-eval Unsafe use of `eval` construct.",
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

/// Rector 2.6.1 with PHPStan 2.2.7, all five class-method rules, PHP 8.4. Only Child.php is selected for editing;
/// the parent and trait in External.php must still protect required and used methods and resolve defaults.
#[test]
fn class_method_rules_see_project_declarations() {
    let external = r#"<?php

abstract class Base
{
    public function __construct(int $n = 1) {}
    abstract public function required();
    public function passthrough(int $n): int { return $n; }
    protected function defaults($n = 1) {}
}

trait CallsKept
{
    public function callKept() { $this->kept(); }
}
"#;
    let child = r#"<?php

final class Child extends Base
{
    use CallsKept;

    public function __construct(int $n = 1)
    {
        parent::__construct($n);
    }

    public function required()
    {
    }

    public function passthrough(int $n): int
    {
        return parent::passthrough($n);
    }

    public function execute()
    {
        parent::defaults(1);
    }

    private function kept()
    {
    }

    private function gone()
    {
    }
}
"#;
    let expected = r#"<?php

final class Child extends Base
{
    use CallsKept;

    public function required()
    {
    }

    public function execute()
    {
        parent::defaults();
    }

    private function kept()
    {
    }
}
"#;
    let config = r#"[format]
preset = "empty"

[lint.rules]
"dead-code/no-empty-method" = "error"
"dead-code/no-redundant-default-argument" = "error"
"dead-code/no-unused-private-method" = "error"
"dead-code/no-useless-constructor" = "error"
"dead-code/no-useless-override" = "error"
"#;
    let d = project(
        "lint",
        "class-method-project",
        &[
            ("pun.toml", config),
            ("Child.php", child),
            ("External.php", external),
        ],
    );
    let out = clean_env(&mut Command::new(PUN), &["PATH", "HOME"])
        .current_dir(&d)
        .env("PUN_AGENT", "0")
        .args(["lint", "--fix", "--no-cache", "Child.php"])
        .output()
        .unwrap();
    assert!(out.status.success(), "{out:?}");
    assert_eq!(read(&d, "Child.php"), expected);
    assert_eq!(read(&d, "External.php"), external);
    std::fs::remove_dir_all(d).unwrap();
}
