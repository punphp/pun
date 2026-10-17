//! `pun migrate` end to end. Expectations are hand-derived from pint.json and context/RULES.md §7, or come from the real
//! Pint (`php .oracle/pint/builds/pint`) on the unmigrated project.
use std::path::{Path, PathBuf};
use std::process::Command;

fn pun(dir: &Path, args: &[&str]) -> (String, String, Option<i32>) {
    let out = Command::new(env!("CARGO_BIN_EXE_pun"))
        .args(args)
        .current_dir(dir)
        .env_clear()
        .envs(std::env::vars().filter(|(k, _)| k == "PATH" || k == "LLVM_PROFILE_FILE"))
        .output()
        .unwrap();
    let s = |b: Vec<u8>| String::from_utf8(b).unwrap();
    (s(out.stdout), s(out.stderr), out.status.code())
}

fn project(name: &str, files: &[(&str, &str)]) -> PathBuf {
    let d = std::env::temp_dir().join(format!("pun-migrate-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    for (f, c) in files {
        std::fs::create_dir_all(d.join(f).parent().unwrap()).unwrap();
        std::fs::write(d.join(f), c).unwrap();
    }
    d.canonicalize().unwrap()
}

fn read(d: &Path, f: &str) -> String {
    std::fs::read_to_string(d.join(f)).unwrap()
}

const PINT: &str = r#"{"preset":"empty","rules":{"array_syntax":true,"final_class":true}}"#;
const TOML: &str = "# Written by `pun migrate` from pint.json.\n[format]\npreset = \"empty\"\n\n[format.rules]\narray-syntax = true\n\n[lint.rules]\nrequire-final-class = { fix = \"apply\" }\n";
const IGNORED: &str = "<?php\n\n// @php-cs-fixer-ignore array_syntax,final_class\n$a = array(1);\n$s = '// @php-cs-fixer-ignore array_syntax';\n";
const REWRITTEN: &str = "<?php\n\n// @pun-ignore-file format.array-syntax,lint.require-final-class\n$a = array(1);\n$s = '// @php-cs-fixer-ignore array_syntax';\n";

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
    // --dry-run: diffs only (pun.toml first), nothing written.
    let (out, err, code) = pun(&d, &["migrate", "--dry-run"]);
    assert_eq!(code, Some(0));
    assert_eq!(
        out,
        format!(
            "--- pun.toml\n+++ pun.toml\n@@ -1,0 +1,9 @@\n{}--- app/Ignored.php\n+++ app/Ignored.php\n@@ -1,5 +1,5 @@\n <?php\n \n-// @php-cs-fixer-ignore array_syntax,final_class\n+// @pun-ignore-file format.array-syntax,lint.require-final-class\n $a = array(1);\n $s = '// @php-cs-fixer-ignore array_syntax';\n",
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
            "pun migrate: pun.toml written from pint.json, ignore comments rewritten in 1 file(s).\n",
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
            "pun migrate: pun.toml already up to date from pint.json, ignore comments rewritten in 0 file(s).\n",
            Some(0)
        )
    );
    let (out, _, _) = pun(&d, &["migrate", "--dry-run"]);
    assert_eq!(out, "");
    // A pun.toml that says something else is never overwritten.
    std::fs::write(d.join("pun.toml"), "# mine\n").unwrap();
    let (_, err, code) = pun(&d, &["migrate"]);
    assert_eq!(
        (err.as_str(), code),
        (
            "pun: pun.toml exists and differs from what migrate writes; move it away first (`pun migrate --dry-run` shows the difference)\n",
            Some(1)
        )
    );
    assert_eq!(read(&d, "pun.toml"), "# mine\n");
}

#[test]
fn stops_on_unsupported_rules() {
    let d = project(
        "unsupported",
        &[(
            "pint.json",
            r#"{"rules":{"braces":true,"array_syntax":true}}"#,
        )],
    );
    let (out, err, code) = pun(&d, &["migrate"]);
    assert_eq!((out.as_str(), code), ("", Some(1)));
    assert!(
        err.starts_with("pun migrate: pint.json uses rules Pun can't run, so nothing was written:\n  braces: deprecated upstream;"),
        "{err}"
    );
    assert!(!d.join("pun.toml").exists());
    // Pint's own config errors stop it too.
    let d = project("broken", &[("pint.json", "{")]);
    let (_, err, code) = pun(&d, &["migrate"]);
    assert_eq!(
        (err, code),
        (
            format!(
                "pun: The configuration file [{}] is not valid JSON.: EOF while parsing an object at line 1 column 1\n",
                d.join("pint.json").display()
            ),
            Some(1)
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
    assert!(out.contains("+single-quote = true\n"), "{out}");
    pun(&d, &["migrate"]);
    assert_eq!(
        read(&d, "pun.toml"),
        "# Written by `pun migrate`: no pint.json or .php-cs-fixer.php found; wrote pun.toml with the laravel preset.\n[format]\npreset = \"laravel\"\n\n[lint.rules]\nno-alias-function = { fix = \"apply\" }\nno-unreachable-default-argument-value = { fix = \"apply\" }\nrequire-ordered-traits = { fix = \"apply\" }\nrequire-protected-setup = { fix = \"apply\" }\n"
    );
}
