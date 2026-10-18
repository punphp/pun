//! `pun migrate` end to end. Expectations are hand-derived from pint.json and context/plan/rules.md §4, or come from the real
//! Pint (`php .oracle/pint/builds/pint`) on the unmigrated project.
#[path = "common/pty.rs"]
mod common;

use common::{pun, read};
use std::path::{Path, PathBuf};

/// `pun migrate` on a pseudo-terminal, typing `keys` (printf escapes) once it is up. The output with `\r` removed.
fn pty(dir: &Path, keys: &str) -> String {
    let (out, code) = common::pty(dir, "migrate", keys, &[]);
    assert_eq!(code, Some(0), "{out}");
    out
}

fn project(name: &str, files: &[(&str, &str)]) -> PathBuf {
    common::project("migrate", name, files)
}

const PINT: &str = r#"{"preset":"empty","rules":{"array_syntax":true,"final_class":true}}"#;
const TOML: &str = "# Written by `pun migrate` from pint.json.\n[format]\npreset = \"empty\"\n\n[format.rules]\n\"arrays/syntax\" = true\n\n[lint.rules]\n\"style/require-final-class\" = \"error\"\n";
/// Without a terminal nothing is asked: everything detected is migrated, and said so.
const PICKED: &str = "pun migrate: migrating everything detected (pick with --from): pint.json\n";
const IGNORED: &str = "<?php\n\n// @php-cs-fixer-ignore array_syntax,final_class\n$a = array(1);\n$s = '// @php-cs-fixer-ignore array_syntax';\n";
const REWRITTEN: &str = "<?php\n\n// @pun-ignore-file format/arrays/syntax,lint/style/require-final-class\n$a = array(1);\n$s = '// @php-cs-fixer-ignore array_syntax';\n";

#[test]
fn writes_pun_toml_and_rewrites_ignores() {
    let d = project(
        "rewrite",
        &[
            ("pint.json", PINT),
            ("app/Ignored.php", IGNORED),
            (
                "app/Unknown.php",
                "<?php\n// @php-cs-fixer-ignore Acme/rule\n",
            ),
            ("app/Plain.php", "<?php\n$a = array(1);\n"),
        ],
    );
    // --dry-run: the diffs (pun.toml first) and the summary, nothing written.
    let (out, err, code) = pun(&d, &["migrate", "--dry-run"]);
    assert_eq!(code, Some(0));
    assert_eq!(
        out,
        format!(
            "{PICKED}--- pun.toml\n+++ pun.toml\n@@ -1,0 +1,9 @@\n{}--- app/Ignored.php\n+++ app/Ignored.php\n@@ -1,5 +1,5 @@\n <?php\n \n-// @php-cs-fixer-ignore array_syntax,final_class\n+// @pun-ignore-file format/arrays/syntax,lint/style/require-final-class\n $a = array(1);\n $s = '// @php-cs-fixer-ignore array_syntax';\npun migrate: pun.toml to write from pint.json, ignore comments to rewrite in 1 file(s).\n  pint.json  to migrate\n",
            TOML.lines().map(|l| format!("+{l}\n")).collect::<String>()
        )
    );
    let unknown = "pun migrate: app/Unknown.php: `@php-cs-fixer-ignore Acme/rule` left as is (no Pun rule for it)\n";
    assert_eq!(err, unknown);
    assert!(!d.join("pun.toml").exists());
    assert_eq!(read(&d, "app/Ignored.php"), IGNORED);

    let (out, err, code) = pun(&d, &["migrate"]);
    assert_eq!(
        (out.as_str(), err.as_str(), code),
        (
            &*format!(
                "{PICKED}pun migrate: pun.toml written from pint.json, ignore comments rewritten in 1 file(s).\n  pint.json  migrated\n"
            ),
            unknown,
            Some(0)
        )
    );
    assert_eq!(read(&d, "pun.toml"), TOML);
    assert_eq!(read(&d, "app/Ignored.php"), REWRITTEN);
    // The rewritten file opts out of both rules: `pun check --fix` leaves it, fixes the other one.
    pun(&d, &["check", "--fix", "app/Ignored.php", "app/Plain.php"]);
    assert_eq!(read(&d, "app/Ignored.php"), REWRITTEN);
    assert_eq!(read(&d, "app/Plain.php"), "<?php\n$a = [1];\n");

    // Idempotent: a second run changes nothing.
    let (out, _, code) = pun(&d, &["migrate"]);
    assert_eq!(
        (out.as_str(), code),
        (
            &*format!(
                "{PICKED}pun migrate: pun.toml already up to date from pint.json, ignore comments rewritten in 0 file(s).\n  pint.json  migrated\n"
            ),
            Some(0)
        )
    );
    let (out, _, _) = pun(&d, &["migrate", "--dry-run"]);
    assert_eq!(
        out,
        format!(
            "{PICKED}pun migrate: pun.toml already up to date from pint.json, ignore comments to rewrite in 0 file(s).\n  pint.json  to migrate\n"
        )
    );
    // A pun.toml that says something else is never overwritten.
    std::fs::write(d.join("pun.toml"), "# mine\n").unwrap();
    let (_, err, code) = pun(&d, &["migrate"]);
    assert_eq!(
        (err.as_str(), code),
        (
            "x pun.toml exists and differs from what migrate writes; move it away first (`pun migrate --dry-run` shows the difference)\n",
            Some(2)
        )
    );
    assert_eq!(read(&d, "pun.toml"), "# mine\n");
}

/// A rule Pun can't run is named on stderr and skipped; the rest is written and migrate exits 0
/// (context/plan/migrate.md §3).
#[test]
fn skips_unsupported_rules() {
    let d = project(
        "unsupported",
        &[(
            "pint.json",
            r#"{"preset":"empty","rules":{"braces":true,"array_syntax":true}}"#,
        )],
    );
    let (out, err, code) = pun(&d, &["migrate"]);
    assert_eq!(
        (out, code),
        (
            format!(
                "{PICKED}pun migrate: pun.toml written from pint.json, ignore comments rewritten in 0 file(s).\n  pint.json  migrated\n"
            ),
            Some(0)
        )
    );
    // The reason is mapping.toml's `drop` for braces.
    assert_eq!(
        err,
        "pun migrate: pint.json: braces: deprecated upstream; successor: single_space_around_construct, control_structure_braces, control_structure_continuation_position, declare_parentheses, no_multiple_statements_per_line, braces_position, statement_indentation, no_extra_blank_lines; skipped\n"
    );
    assert_eq!(
        read(&d, "pun.toml"),
        "# Written by `pun migrate` from pint.json.\n[format]\npreset = \"empty\"\n\n[format.rules]\n\"arrays/syntax\" = true\n"
    );
    // Pint's own config errors stop it too.
    let d = project("broken", &[("pint.json", "{")]);
    let (_, err, code) = pun(&d, &["migrate"]);
    assert_eq!(
        (err, code),
        (
            format!(
                "x The configuration file [{}] is not valid JSON.: EOF while parsing an object at line 1 column 1\n",
                d.join("pint.json").display()
            ),
            Some(2)
        )
    );
}

/// `--config` / `--preset` read like `pint --config` / `pint --preset`; no pint.json is Pint's default (laravel).
#[test]
fn config_and_preset_flags() {
    let d = project(
        "flags",
        &[(
            "other.json",
            r#"{"preset":"empty","rules":{"single_quote":true}}"#,
        )],
    );
    pun(
        &d,
        &[
            "migrate",
            "--dry-run",
            "--config=other.json",
            "--preset=psr12",
        ],
    );
    let (out, _, _) = pun(
        &d,
        &[
            "migrate",
            "--dry-run",
            "--config=other.json",
            "--preset=psr12",
        ],
    );
    assert!(
        out.contains(
            "+# Written by `pun migrate` from other.json.\n+[format]\n+preset = \"psr12\"\n"
        ),
        "{out}"
    );
    assert!(out.contains("+\"strings/single-quote\" = true\n"), "{out}");
    // No pint.json and `--preset per`: every rule of @PER-CS3x0 (and the sets under it) runs natively, so nothing is
    // named as skipped.
    let (_, err, code) = pun(&d, &["migrate", "--dry-run", "--preset=per"]);
    assert_eq!((err.as_str(), code), ("", Some(0)));
    let (out, _, _) = pun(&d, &["migrate"]);
    assert_eq!(
        out,
        "pun migrate: no pint.json, .php-cs-fixer.php, phpstan.neon, rector.php or mago.toml found.\npun migrate: starter pun.toml written, ignore comments rewritten in 0 file(s).\n"
    );
    assert_eq!(
        read(&d, "pun.toml"),
        "# Written by `pun migrate`: no pint.json, .php-cs-fixer.php, phpstan.neon, rector.php or mago.toml found; starter config.\n[format]\npreset = \"laravel\"\n\n[lint.rules]\n\"refactor/rename-phpdoc-tag\" = \"error\"\n\"functions/no-unused-closure-use\" = \"error\"\n\"style/no-alias-function\" = \"error\"\n\"statements/no-empty\" = \"error\"\n\"phpdoc/no-superfluous-tags\" = { allow-mixed = true, allow-unused-params = true }\n\"functions/no-unreachable-default-argument-value\" = \"error\"\n\"imports/no-unused\" = \"error\"\n\"control-flow/no-useless-return\" = \"error\"\n\"style/require-ordered-traits\" = \"error\"\n\"phpunit/require-method-casing\" = { case = \"snake_case\" }\n\"phpunit/require-protected-setup\" = \"error\"\n\"phpdoc/no-access\" = \"error\"\n\"phpdoc/no-package\" = \"error\"\n\"phpdoc/no-useless-inheritdoc\" = \"error\"\n\"classes/prefer-self-in-final\" = \"error\"\n"
    );
}

const CHECKLIST: &[(&str, &str)] = &[
    ("pint.json", PINT),
    ("phpstan.neon", "parameters:\n    level: 5\n"),
    ("rector.php", "<?php\n"),
];

/// Without a terminal every detected config is listed; only Pint and PHP-CS-Fixer are migrated, the rest is
/// reported and left as it was. `--from` picks; one that isn't there is an error.
#[test]
fn lists_every_detected_config() {
    let files = [
        ("pint.json", PINT),
        (".php-cs-fixer.dist.php", "<?php\nreturn 1;\n"),
        ("phpstan.dist.neon", "parameters:\n    level: 5\n"),
        ("rector.php", "<?php\n"),
        ("mago.toml", "[linter]\n"),
    ];
    let d = project("all", &files);
    let (out, err, code) = pun(&d, &["migrate"]);
    assert_eq!(
        (out.as_str(), err.as_str(), code),
        (
            "pun migrate: migrating everything detected (pick with --from): pint.json, .php-cs-fixer.dist.php, phpstan.dist.neon, rector.php, mago.toml
pun migrate: pun.toml written from pint.json, ignore comments rewritten in 0 file(s).
  pint.json               migrated
  .php-cs-fixer.dist.php  skipped: pint.json gives [format]; .php-cs-fixer.dist.php left untouched
  phpstan.dist.neon       skipped: PHPStan migration is not supported yet; phpstan.dist.neon left untouched
  rector.php              skipped: Rector migration is not supported yet; rector.php left untouched
  mago.toml               skipped: Mago migration is not supported yet; mago.toml left untouched
",
            "",
            Some(0)
        )
    );
    assert_eq!(read(&d, "pun.toml"), TOML);
    for (f, c) in files {
        assert_eq!(read(&d, f), c);
    }
    // --from: only those, never a prompt; without Pint or PHP-CS-Fixer there is no pun.toml to write.
    std::fs::remove_file(d.join("pun.toml")).unwrap();
    let (out, _, code) = pun(&d, &["migrate", "--from", "phpstan,mago"]);
    assert_eq!(
        (out.as_str(), code),
        (
            "pun migrate: no Pint or PHP-CS-Fixer config selected, pun.toml not written.
  pint.json               not selected
  .php-cs-fixer.dist.php  not selected
  phpstan.dist.neon       skipped: PHPStan migration is not supported yet; phpstan.dist.neon left untouched
  rector.php              not selected
  mago.toml               skipped: Mago migration is not supported yet; mago.toml left untouched
",
            Some(0)
        )
    );
    assert!(!d.join("pun.toml").exists());
    let d = project("from-missing", &[("pint.json", PINT)]);
    let (out, err, code) = pun(&d, &["migrate", "--from=pint,php-cs-fixer"]);
    assert_eq!(
        (out.as_str(), err.as_str(), code),
        (
            "",
            "x --from php-cs-fixer: no .php-cs-fixer.php or .php-cs-fixer.dist.php found\n",
            Some(2)
        )
    );
    let (_, err, _) = pun(&d, &["migrate", "--from=pint", "--config=nope.json"]);
    assert_eq!(err, "x --from pint: no nope.json found\n");
}

/// PHPStan alone: nothing gives [format], so pun.toml is the starter, and it says why.
#[test]
fn starter_beside_unsupported_configs() {
    let d = project(
        "phpstan-only",
        &[("phpstan.neon.dist", "parameters:\n    level: 5\n")],
    );
    let (out, _, code) = pun(&d, &["migrate", "--yes"]);
    assert_eq!(
        (out.as_str(), code),
        (
            "pun migrate: migrating everything detected (pick with --from): phpstan.neon.dist
pun migrate: starter pun.toml written, ignore comments rewritten in 0 file(s).
  phpstan.neon.dist  skipped: PHPStan migration is not supported yet; phpstan.neon.dist left untouched
",
            Some(0)
        )
    );
    assert!(read(&d, "pun.toml").starts_with(
        "# Written by `pun migrate`: no pint.json or .php-cs-fixer.php found; starter config.\n[format]\npreset = \"laravel\"\n"
    ));
}

/// Agent mode never asks: one summary line, then a line per config.
#[test]
fn agent_mode() {
    let unknown = (
        "app/Unknown.php",
        "<?php\n// @php-cs-fixer-ignore Acme/rule\n",
    );
    let d = project("agent", &[CHECKLIST, &[unknown]].concat());
    let agent = |args: &[&str]| {
        let (out, _, code) = pun(&d, &[args, &["--agent"]].concat());
        assert_eq!(code, Some(0));
        out
    };
    let out = agent(&["migrate", "--dry-run"]);
    let (head, rest) = out.split_once('\n').unwrap();
    assert!(
        head.starts_with("pun migrate: dry-run, pun.toml to write (preset empty, 1 format rule, 1 lint rule), ignore comments to rewrite in 0 files (from pint.json, "),
        "{head}"
    );
    assert!(rest.starts_with("pint.json migrate\nphpstan.neon not-supported skipped: PHPStan migration is not supported yet; phpstan.neon left untouched\nrector.php not-supported skipped: Rector migration is not supported yet; rector.php left untouched\napp/Unknown.php ignore-not-migrated `@php-cs-fixer-ignore Acme/rule` left as is (no Pun rule for it)\n--- pun.toml\n"), "{rest}");
    let out = agent(&["migrate", "--from=rector"]);
    assert!(
        out.starts_with(
            "pun migrate: ok, no Pint or PHP-CS-Fixer config selected, pun.toml not written ("
        ),
        "{out}"
    );
    assert!(out.ends_with("s)\npint.json not-selected\nphpstan.neon not-selected\nrector.php not-supported skipped: Rector migration is not supported yet; rector.php left untouched\n"), "{out}");
    let d = project("agent-empty", &[]);
    let (out, _, _) = pun(&d, &["migrate", "--agent"]);
    assert!(out.starts_with("pun migrate: ok, starter pun.toml written (preset laravel, 115 format rules, 15 lint rules), ignore comments rewritten in 0 files (no pint.json, .php-cs-fixer.php, phpstan.neon, rector.php or mago.toml found, "), "{out}");
}

/// On a terminal: a checklist of the detected configs, all checked. Down + space unchecks PHPStan.
#[test]
fn checklist_on_a_terminal() {
    let d = project("tty", CHECKLIST);
    let out = pty(&d, "\\033[B \\r");
    assert!(
        out.contains("Migrate which configs? (space toggles, enter confirms)"),
        "{out}"
    );
    assert!(out.contains("> [x] Pint          pint.json\n  [x] PHPStan       phpstan.neon  (not supported yet)\n  [x] Rector        rector.php    (not supported yet)\n"), "{out}");
    assert!(
        out.ends_with(
            "pun migrate: pun.toml written from pint.json, ignore comments rewritten in 0 file(s).
  pint.json     migrated
  phpstan.neon  not selected
  rector.php    skipped: Rector migration is not supported yet; rector.php left untouched
"
        ),
        "{out}"
    );
    assert_eq!(read(&d, "pun.toml"), TOML);
    // Esc: nothing selected, nothing written.
    std::fs::remove_file(d.join("pun.toml")).unwrap();
    let out = pty(&d, "\\033");
    assert!(
        out.ends_with(
            "pun migrate: nothing selected, pun.toml not written.
  pint.json     not selected
  phpstan.neon  not selected
  rector.php    not selected
"
        ),
        "{out}"
    );
    assert!(!d.join("pun.toml").exists());
}

/// No config at all: the terminal is asked before the starter pun.toml is written.
#[test]
fn starter_on_a_terminal() {
    let d = project("tty-empty", &[]);
    let out = pty(&d, "n");
    assert!(out.starts_with("pun migrate: no pint.json, .php-cs-fixer.php, phpstan.neon, rector.php or mago.toml found.\n"), "{out}");
    assert!(out.contains("Write a starter pun.toml?"), "{out}");
    assert!(out.ends_with("pun migrate: nothing written.\n"), "{out}");
    assert!(!d.join("pun.toml").exists());
    let out = pty(&d, "\\r");
    assert!(
        out.ends_with(
            "pun migrate: starter pun.toml written, ignore comments rewritten in 0 file(s).\n"
        ),
        "{out}"
    );
    assert!(read(&d, "pun.toml").starts_with("# Written by `pun migrate`: no pint.json, .php-cs-fixer.php, phpstan.neon, rector.php or mago.toml found; starter config.\n"));
}
