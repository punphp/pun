//! `pun migrate` end to end. Expectations are hand-derived from pint.json and context/plan/rules.md §4, or come from the real
//! Pint (`php .oracle/pint/builds/pint`) on the unmigrated project.
mod common;

use common::pty::{pun, pun_with};
use common::read;
use std::path::{Path, PathBuf};

/// `pun migrate` on a pseudo-terminal, typing `keys` (printf escapes) once it is up. The output with `\r` and ANSI
/// sequences removed.
fn pty(dir: &Path, keys: &str) -> String {
    let (out, code) = common::pty::pty(dir, "migrate", keys, &[("NO_COLOR", "1")]);
    assert_eq!(code, Some(0), "{out}");
    common::pty::strip_ansi(&out)
}

fn project(name: &str, files: &[(&str, &str)]) -> PathBuf {
    common::project("migrate", name, files)
}

/// `out` with the rule counts of `[analyse]` lines as `N`: they follow mapping.toml's levels and presets.
fn counts(out: &str) -> String {
    let words: Vec<String> = out.split(' ').map(String::from).collect();
    (words.iter().enumerate())
        .map(|(i, w)| {
            let next = words.get(i + 1).map(|n| n.trim_end_matches([',', '\n']));
            let number = !w.is_empty() && w.bytes().all(|b| b.is_ascii_digit());
            match next {
                Some("on" | "off") if number => "N".to_string(),
                _ => w.clone(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

const PINT: &str = r#"{"preset":"empty","rules":{"array_syntax":true,"final_class":true}}"#;
const TOML: &str = "# Written by `pun migrate` from pint.json.\n[format]\npreset = \"empty\"\n\n[format.rules]\n\"arrays/syntax\" = true\n\n[lint.rules]\n\"style/require-final-class\" = \"error\"\n";
/// The line pint.json gets (ASCII glyphs: the tests run without a UTF-8 locale).
const MOVED: &str = " ok pint.json  -> [format] preset empty, 1 rule changed, 1 rule -> [lint]\n";
const PR: &str = "   Want them? Send a PR to https://github.com/punphp/pun\n";
const IGNORED: &str = "<?php\n\n// @php-cs-fixer-ignore array_syntax,final_class\n$a = array(1);\n$s = '// @php-cs-fixer-ignore array_syntax';\n";
const UNKNOWN: &str = " ! app/Unknown.php  `@php-cs-fixer-ignore Acme/rule` left as is (no Pun rule for it); skipped\n";
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
            "--- pun.toml\n+++ pun.toml\n@@ -1,0 +1,9 @@\n{}--- app/Ignored.php\n+++ app/Ignored.php\n@@ -1,5 +1,5 @@\n <?php\n \n-// @php-cs-fixer-ignore array_syntax,final_class\n+// @pun-ignore-file format/arrays/syntax,lint/style/require-final-class\n $a = array(1);\n $s = '// @php-cs-fixer-ignore array_syntax';\n ok pint.json        -> [format] preset empty, 1 rule changed, 1 rule -> [lint]\n ok ignore comments  -> @pun-ignore-file in 1 file\n{UNKNOWN}{PR} - Dry run: would write pun.toml\n",
            TOML.lines().map(|l| format!("+{l}\n")).collect::<String>()
        )
    );
    assert_eq!(err, "");
    assert!(!d.join("pun.toml").exists());
    assert_eq!(read(&d, "app/Ignored.php"), IGNORED);

    let (out, err, code) = pun(&d, &["migrate"]);
    assert_eq!(
        (out.as_str(), err.as_str(), code),
        (
            &*format!(
                " ok pint.json        -> [format] preset empty, 1 rule changed, 1 rule -> [lint]\n ok ignore comments  -> @pun-ignore-file in 1 file\n{UNKNOWN}{PR} ok Wrote pun.toml\n"
            ),
            "",
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
            &*format!("{MOVED}{UNKNOWN}{PR} ok pun.toml is already up to date\n"),
            Some(0)
        )
    );
    let (out, _, _) = pun(&d, &["migrate", "--dry-run"]);
    assert_eq!(
        out,
        format!("{MOVED}{UNKNOWN}{PR} - Dry run: pun.toml is already up to date\n")
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
    // With a UTF-8 locale: the glyphs of context/plan/migrate.md §3. The reason is mapping.toml's `drop` for braces.
    let (out, err, code) = pun_with(&d, &["migrate"], &[("LANG", "en_US.UTF-8")]);
    assert_eq!(
        (out.as_str(), err.as_str(), code),
        (
            " ✔ pint.json  → [format] preset empty, 1 rule changed\n ⚠ braces  deprecated upstream; successor: single_space_around_construct, control_structure_braces, control_structure_continuation_position, declare_parentheses, no_multiple_statements_per_line, braces_position, statement_indentation, no_extra_blank_lines; skipped\n   Want them? Send a PR to https://github.com/punphp/pun\n ✔ Wrote pun.toml\n",
            "",
            Some(0)
        )
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
    let (out, _, code) = pun(&d, &["migrate", "--dry-run", "--preset=per"]);
    assert_eq!(code, Some(0));
    assert!(!out.contains("Want them?"), "{out}");
    // A pint.json with `"preset": "symfony"`: only the preset's rules that are still `ported = false` in mapping.toml
    // are named (every format rule of @Symfony is native; the lint rules are being ported, so which ones varies).
    let s = project("symfony", &[("pint.json", r#"{"preset":"symfony"}"#)]);
    let (out, _, code) = pun(&s, &["migrate", "--dry-run"]);
    assert_eq!(code, Some(0));
    for line in out.lines().filter(|l| l.starts_with(" ! ")) {
        let name = line[3..].split_once("  ").unwrap().0;
        let id = pun_config::rule_of(name).unwrap();
        assert!(
            line.ends_with("  not supported by Pun yet; skipped"),
            "{line}"
        );
        assert!(id.starts_with("lint/"), "{line}");
    }
    let (out, _, _) = pun(&d, &["migrate"]);
    assert_eq!(
        out,
        "No pint.json, .php-cs-fixer.php, phpstan.neon, rector.php or mago.toml found.\n ok Wrote starter pun.toml (preset laravel)\n"
    );
    assert_eq!(
        read(&d, "pun.toml"),
        "# Written by `pun migrate`: no pint.json, .php-cs-fixer.php, phpstan.neon, rector.php or mago.toml found; starter config.\n[format]\npreset = \"laravel\"\n\n[lint.rules]\n\"refactor/rename-phpdoc-tag\" = \"error\"\n\"functions/no-unused-closure-use\" = \"error\"\n\"style/no-alias-function\" = \"error\"\n\"statements/no-empty\" = \"error\"\n\"phpdoc/no-superfluous-tags\" = { allow-mixed = true, allow-unused-params = true }\n\"functions/no-unreachable-default-argument-value\" = \"error\"\n\"imports/no-unused\" = \"error\"\n\"control-flow/no-useless-return\" = \"error\"\n\"style/require-ordered-traits\" = \"error\"\n\"phpunit/require-method-casing\" = { case = \"snake-case\" }\n\"phpunit/require-protected-setup\" = \"error\"\n\"phpdoc/no-access\" = \"error\"\n\"phpdoc/no-package\" = \"error\"\n\"phpdoc/no-useless-inheritdoc\" = \"error\"\n\"classes/prefer-self-in-final\" = \"error\"\n"
    );
}

const CHECKLIST: &[(&str, &str)] = &[
    ("pint.json", PINT),
    ("phpstan.neon", "parameters:\n    level: 5\n"),
    ("rector.php", "<?php\n"),
];

/// Without a terminal every detected config is listed and migrated: pint.json gives [format], phpstan.neon
/// [analyse]; rector.php is only read through the project's Rector, and Mago is not supported yet. `--from` picks; one
/// that isn't there is an error.
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
        (counts(&out).as_str(), err.as_str(), code),
        (
            " ok pint.json               -> [format] preset empty, 1 rule changed, 1 rule -> [lint]
 - .php-cs-fixer.dist.php  not migrated: pint.json gives [format], as in Pint
 ok phpstan.dist.neon       -> [analyse] preset recommended (level 5), N on, N off
 ! rector.php              not read: run `composer install` so Pun can read it with Rector; left untouched
 ! format/whitespace/encoding  a PHPStan check that is this format rule in Pun; turn it on in [format.rules]; skipped
 ! mago.toml                   Mago config: not supported by Pun yet; left untouched
   Want them? Send a PR to https://github.com/punphp/pun
 ok Wrote pun.toml
",
            "",
            Some(0)
        )
    );
    let toml = read(&d, "pun.toml");
    assert!(
        toml.starts_with(TOML)
            && toml[TOML.len()..].starts_with("\n[analyse]\npreset = \"recommended\"\n"),
        "{toml}"
    );
    for (f, c) in files {
        assert_eq!(read(&d, f), c);
    }
    // --from: only those, never a prompt; without Pint or PHP-CS-Fixer there is no pun.toml to write.
    std::fs::remove_file(d.join("pun.toml")).unwrap();
    let (out, _, code) = pun(&d, &["migrate", "--from", "phpstan,mago"]);
    assert_eq!(
        (out.as_str(), code),
        (
            " - pint.json               not selected
 - .php-cs-fixer.dist.php  not selected
 - phpstan.dist.neon       not migrated: pun.toml needs the format config too
 - rector.php              not selected
 ! mago.toml  Mago config: not supported by Pun yet; left untouched
   Want them? Send a PR to https://github.com/punphp/pun
 - No Pint or PHP-CS-Fixer config selected; pun.toml not written
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

/// An I/O error names its file, and a PHP fatal error in the old config shows PHP's message (stderr, as stdout is
/// empty). Directories stand in for unreadable and unwritable files, as root can read a mode-000 file.
#[test]
fn errors_name_the_file() {
    let d = project("unreadable", &[]);
    std::fs::create_dir(d.join("pint.json")).unwrap();
    let (_, err, code) = pun(&d, &["migrate", "--yes"]);
    assert_eq!(
        (err.as_str(), code),
        (
            format!(
                "x can't read {}: Is a directory (os error 21)\n",
                d.join("pint.json").display()
            )
            .as_str(),
            Some(2)
        )
    );
    let d = project("unwritable", &[("pint.json", PINT)]);
    std::fs::create_dir(d.join("pun.toml")).unwrap();
    let (_, err, code) = pun(&d, &["migrate", "--yes"]);
    assert_eq!(
        (err.as_str(), code),
        (
            format!(
                "x can't write {}: Is a directory (os error 21)\n",
                d.join("pun.toml").display()
            )
            .as_str(),
            Some(2)
        )
    );
    let d = project(
        "php-fatal",
        &[
            ("vendor/friendsofphp/php-cs-fixer/.keep", ""),
            (
                "vendor/autoload.php",
                "<?php\nfunction f() {}\nfunction f() {}\n",
            ),
            (".php-cs-fixer.php", "<?php\nreturn 1;\n"),
        ],
    );
    let (_, err, code) = pun(&d, &["migrate", "--yes"]);
    let autoload = d.join("vendor/autoload.php");
    let autoload = autoload.display();
    assert_eq!(
        (err.as_str(), code),
        (
            format!("x Fatal error: Cannot redeclare function f() (previously declared in {autoload}:2) in {autoload} on line 3\n").as_str(),
            Some(2)
        )
    );
}

/// PHPStan alone: nothing gives [format], so pun.toml is the starter beside phpstan.neon's [analyse].
#[test]
fn starter_beside_phpstan() {
    let d = project(
        "phpstan-only",
        &[("phpstan.neon.dist", "parameters:\n    level: 5\n")],
    );
    let (out, _, code) = pun(&d, &["migrate", "--yes"]);
    assert_eq!(
        (counts(&out).as_str(), code),
        (
            " ok phpstan.neon.dist  -> [analyse] preset recommended (level 5), N on, N off\n ok Wrote starter pun.toml (preset laravel)\n",
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
    assert!(rest.starts_with("pint.json migrate\nphpstan.neon migrate\nrector.php not-read not read: run `composer install` so Pun can read it with Rector; left untouched\nphpstan.neon unsupported format/whitespace/encoding: a PHPStan check that is this format rule in Pun; turn it on in [format.rules]\napp/Unknown.php ignore-not-migrated `@php-cs-fixer-ignore Acme/rule` left as is (no Pun rule for it)\nsend a PR for what is skipped: https://github.com/punphp/pun\n--- pun.toml\n"), "{rest}");
    let out = agent(&["migrate", "--from=rector"]);
    assert!(
        out.starts_with(
            "pun migrate: ok, no Pint or PHP-CS-Fixer config selected, pun.toml not written ("
        ),
        "{out}"
    );
    assert!(out.ends_with("s)\npint.json not-selected\nphpstan.neon not-selected\nrector.php not-read not read: run `composer install` so Pun can read it with Rector; left untouched\n"), "{out}");
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
    // dialoguer's ColorfulTheme, as `pun init` has (without colour it draws no cursor): all checked, then PHPStan not.
    assert!(
        out.contains(
            "✔ Pint          pint.json\n✔ PHPStan       phpstan.neon\n✔ Rector        rector.php\n"
        ),
        "{out}"
    );
    assert!(
        out.contains(
            "✔ Pint          pint.json\n⬚ PHPStan       phpstan.neon\n✔ Rector        rector.php\n"
        ),
        "{out}"
    );
    assert!(
        out.ends_with(
            " ok pint.json     -> [format] preset empty, 1 rule changed, 1 rule -> [lint]\n - phpstan.neon  not selected\n ! rector.php    not read: run `composer install` so Pun can read it with Rector; left untouched\n ok Wrote pun.toml\n"
        ),
        "{out}"
    );
    assert_eq!(read(&d, "pun.toml"), TOML);
    // Esc: nothing selected, nothing written.
    std::fs::remove_file(d.join("pun.toml")).unwrap();
    let out = pty(&d, "\\033");
    assert!(
        out.ends_with(
            " - pint.json     not selected\n - phpstan.neon  not selected\n - rector.php    not selected\n - Nothing selected; pun.toml not written\n"
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
    // The pty echoes the typed answer before migrate prints.
    assert!(
        out.contains(
            "No pint.json, .php-cs-fixer.php, phpstan.neon, rector.php or mago.toml found.\n"
        ),
        "{out}"
    );
    assert!(out.contains("Write a starter pun.toml?"), "{out}");
    assert!(out.ends_with(" - Nothing written\n"), "{out}");
    assert!(!d.join("pun.toml").exists());
    let out = pty(&d, "\\r");
    assert!(
        out.ends_with(" ok Wrote starter pun.toml (preset laravel)\n"),
        "{out}"
    );
    assert!(read(&d, "pun.toml").starts_with("# Written by `pun migrate`: no pint.json, .php-cs-fixer.php, phpstan.neon, rector.php or mago.toml found; starter config.\n"));
}

/// With phpstan.neon migrated, `@phpstan-ignore` comments become `@pun-ignore` with each identifier's rule
/// (rules/mapping.toml: method.notFound and function.notFound are analyse/undefined/{method,function}) and the
/// parenthesised comments as the reason. An identifier Pun doesn't know, and `@phpstan-ignore-next-line` (no
/// identifier), are left and named.
#[test]
fn rewrites_phpstan_ignores() {
    let php = "<?php\n\n$a->foo(); // @phpstan-ignore method.notFound (a macro adds it), function.notFound\n/** @phpstan-ignore acme.rule */\n$b = 1;\n// @phpstan-ignore-next-line\n$c = 2;\n";
    let d = project(
        "phpstan-ignores",
        &[
            (
                "phpstan.neon",
                "parameters:\n    level: 5\n    paths: [app]\n",
            ),
            ("app/A.php", php),
        ],
    );
    let (out, _, code) = pun(&d, &["migrate", "--dry-run", "--yes"]);
    assert_eq!(code, Some(0), "{out}");
    assert!(
        out.contains("-$a->foo(); // @phpstan-ignore method.notFound (a macro adds it), function.notFound\n+$a->foo(); // @pun-ignore analyse/undefined/method,analyse/undefined/function -- a macro adds it\n"),
        "{out}"
    );
    assert!(
        counts(&out).contains(" ok ignore comments  -> @pun-ignore in 1 file\n ! app/A.php  `@phpstan-ignore acme.rule` left as is (`acme.rule` is not a PHPStan identifier Pun knows); skipped\n ! app/A.php  `@phpstan-ignore-next-line` left as is (it names no identifier, and a Pun ignore names its rules); skipped\n"),
        "{out}"
    );
    // A dry run writes nothing.
    assert_eq!(read(&d, "app/A.php"), php);
}

/// On a terminal, after PHPStan's exact rules, migrate offers the closest preset (level 5: `recommended`,
/// context/plan/analyse.md) with what it would change; one empty file has no problems either way. `y` drops the
/// on/off lines, the default (`n`) keeps them.
#[test]
fn offers_the_closest_preset() {
    let files = [
        (
            "phpstan.neon",
            "parameters:\n    level: 5\n    paths: [app]\n",
        ),
        ("app/A.php", "<?php\n"),
    ];
    let d = project("tty-preset", &files);
    let out = pty(&d, "\\ry");
    assert!(
        out.contains("You're on PHPStan level 5; the closest Pun preset is `recommended`. Switch to it? (0 new problems, 0 fewer)"),
        "{out}"
    );
    assert!(out.contains(", switched to the preset\n"), "{out}");
    let toml = read(&d, "pun.toml");
    assert!(
        !toml.contains("\"on\"") && !toml.contains("\"off\""),
        "{toml}"
    );
    std::fs::remove_file(d.join("pun.toml")).unwrap();
    let out = pty(&d, "\\rn");
    assert!(!out.contains("switched to the preset"), "{out}");
    assert!(read(&d, "pun.toml").contains(" = \"off\"\n"));
}

/// `.oracle/<rel>` (scripts/oracle.sh), canonical; `None` skips the cases that need it.
fn oracle(rel: &str) -> Option<PathBuf> {
    let p = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../.oracle")
        .join(rel);
    p.exists().then(|| p.canonicalize().unwrap())
}

/// phpstan.neon with a baseline, phpstan-strict-rules (its checks are Pun lint and format rules), a parameter that
/// is a rule option, and an ignored pattern. Expected from rules/mapping.toml: `additionalConstructors` is the
/// `additional-constructors` option, strict-rules' `backtick.notAllowed` the format rule
/// `functions/backtick-to-shell-exec`.
#[test]
fn phpstan_baseline_extensions_options_and_ignores() {
    let Some(strict) = oracle("phpstan/vendor/phpstan/phpstan-strict-rules") else {
        return;
    };
    let neon = "includes:\n    - phpstan-baseline.neon\n    - vendor/phpstan/phpstan-strict-rules/rules.neon\nparameters:\n    level: 9\n    paths:\n        - src\n    additionalConstructors:\n        - App\\Foo::init\n    ignoreErrors:\n        - '#Call to an undefined method#'\n";
    let d = project(
        "phpstan-extras",
        &[
            ("phpstan.neon", neon),
            (
                "phpstan-baseline.neon",
                "parameters:\n    ignoreErrors: []\n",
            ),
            ("src/a.php", "<?php\n"),
        ],
    );
    std::fs::create_dir_all(d.join("vendor/phpstan")).unwrap();
    std::os::unix::fs::symlink(strict, d.join("vendor/phpstan/phpstan-strict-rules")).unwrap();
    let (out, _, code) = pun(&d, &["migrate", "--yes"]);
    assert_eq!(code, Some(0), "{out}");
    let out = counts(&out);
    let line = out.lines().next().unwrap();
    assert!(
        line.starts_with(" ok phpstan.neon  -> [analyse] preset max (level 9), N on, N off, "),
        "{out}"
    );
    assert!(
        line.ends_with(" rules -> [lint], 1 parameter -> rule options, 1 error pattern ignored"),
        "{out}"
    );
    assert!(
        out.contains(" ! format/functions/backtick-to-shell-exec  a PHPStan check that is this format rule in Pun; turn it on in [format.rules]; skipped\n"),
        "{out}"
    );
    assert!(
        out.contains("\n ! phpstan-baseline.neon was not copied. Run `pun analyse --generate-baseline` for a Pun baseline.\n"),
        "{out}"
    );
    let toml = read(&d, "pun.toml");
    assert!(
        toml.contains(
            "\"types/readonly-property-write\" = { additional-constructors = ['App\\Foo::init'] }\n"
        ),
        "{toml}"
    );
    assert!(
        toml.contains("[[analyse.ignore]]\nmessage = \"#Call to an undefined method#\"\n"),
        "{toml}"
    );
}

/// rector.php read through Rector 2.6.1 (`.oracle/rector-bin`): its paths become `[lint] exclude` for the files
/// outside them, a rule skipped for a path an override, and what Pun can't run is listed (a reason shared by more
/// than three rules once, with the count).
#[test]
fn rector_paths_skips_and_unsupported_rules() {
    let Some(vendor) = oracle("rector-bin/vendor") else {
        return;
    };
    let rector = "<?php\n\nuse Rector\\Config\\RectorConfig;\nuse Rector\\DeadCode\\Rector\\ClassMethod\\RemoveUselessParamTagRector;\n\nreturn RectorConfig::configure()\n    ->withPaths([__DIR__.'/src'])\n    ->withPreparedSets(deadCode: true)\n    ->withPhpSets(php53: true)\n    ->withSkip([RemoveUselessParamTagRector::class => [__DIR__.'/src/Sub']]);\n";
    let d = project(
        "rector",
        &[
            ("composer.json", "{\"require\":{\"php\":\"^8.4\"}}"),
            ("rector.php", rector),
            ("src/a.php", "<?php\n"),
            ("src/Sub/b.php", "<?php\n"),
            ("scripts/x.php", "<?php\n"),
            ("tools/deep/y.php", "<?php\n"),
        ],
    );
    std::os::unix::fs::symlink(vendor, d.join("vendor")).unwrap();
    let (out, err, code) = pun(&d, &["migrate", "--yes"]);
    assert_eq!(code, Some(0), "{out}{err}");
    let out = counts(&out);
    assert!(
        out.starts_with(" ok rector.php  -> [lint] sets dead-code, modernize; N on, N off"),
        "{out}"
    );
    assert!(
        out.contains(", 3 paths excluded, 1 per-path skip\n"),
        "{out}"
    );
    // php53 (withPhpSets below 8.4) adds many upgrade rules that share one reason.
    assert!(
        out.lines().any(|l| l.starts_with(" ! 13 Rector rules ")
            && l.ends_with(
                "  only in php52..php83 upgrade sets (below the PHP 8.4 floor); skipped"
            )),
        "{out}"
    );
    assert!(out.contains(PR), "{out}");
    let toml = read(&d, "pun.toml");
    assert!(
        toml.contains("exclude = [\"rector.php\", \"scripts\", \"tools\"]\n"),
        "{toml}"
    );
    assert!(
        toml.contains("[[lint.overrides]]\nfiles = [\"src/Sub\"]\nrules = { \"phpdoc/no-useless-param\" = \"off\" }\n"),
        "{toml}"
    );
    // Rector's paths cover the project: nothing excluded.
    std::fs::remove_file(d.join("pun.toml")).unwrap();
    std::fs::write(
        d.join("rector.php"),
        rector.replace("__DIR__.'/src'", "__DIR__"),
    )
    .unwrap();
    let (out, _, code) = pun(&d, &["migrate", "--yes"]);
    assert_eq!(code, Some(0), "{out}");
    assert!(!out.contains("excluded"), "{out}");
    // A rector.php Rector can't load stops the migration with its message: the config's own exception (on stdout)
    // or a fatal error (on stderr).
    std::fs::remove_file(d.join("pun.toml")).unwrap();
    std::fs::write(d.join("rector.php"), "<?php\nreturn 1;\n").unwrap();
    let (_, err, code) = pun(&d, &["migrate", "--yes"]);
    assert_eq!(
        (err.as_str(), code),
        (
            "x rector.php must return RectorConfig::configure() or a closure taking RectorConfig\n",
            Some(2)
        )
    );
    std::fs::write(
        d.join("rector.php"),
        "<?php\nfunction f() {}\nfunction f() {}\n",
    )
    .unwrap();
    let (_, err, code) = pun(&d, &["migrate", "--yes"]);
    let file = d.join("rector.php");
    let file = file.display();
    assert_eq!(
        (err, code),
        (
            format!(
                "x Fatal error: Cannot redeclare function f() (previously declared in {file}:2) in {file} on line 3\n"
            ),
            Some(2)
        )
    );
}

/// A `@phpstan-ignore` that names a check Pun has as a format rule (rules/mapping.toml: `whitespace.fileEnd` is
/// `format/tags/no-closing`) is left: format rules are not ignored per line.
#[test]
fn phpstan_ignore_of_a_format_check_is_left() {
    let php = "<?php\n\n$a = 1; // @phpstan-ignore whitespace.fileEnd\n";
    let d = project(
        "phpstan-format-ignore",
        &[
            (
                "phpstan.neon",
                "parameters:\n    level: 5\n    paths: [app]\n",
            ),
            ("app/A.php", php),
        ],
    );
    let (out, _, code) = pun(&d, &["migrate", "--dry-run", "--yes"]);
    assert_eq!(code, Some(0), "{out}");
    assert!(
        out.contains(" ! app/A.php  `@phpstan-ignore whitespace.fileEnd` left as is (`whitespace.fileEnd` is the format rule `format/tags/no-closing`, ignored per file); skipped\n"),
        "{out}"
    );
}

/// Mago is detected but can't be migrated yet: the checklist says so.
#[test]
fn checklist_marks_unsupported_configs() {
    let d = project("tty-mago", &[("pint.json", PINT), ("mago.toml", "")]);
    let out = pty(&d, "\\r");
    assert!(
        out.contains("Mago          mago.toml  (not supported yet)\n"),
        "{out}"
    );
}
