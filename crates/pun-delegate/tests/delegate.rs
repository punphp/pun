//! Delegate workers against the real tools. Expectations are the output of `php .oracle/pint/builds/pint` (Pint mode)
//! and `php .oracle/php-cs-fixer/php-cs-fixer fix --config=.php-cs-fixer.php` run on tests/custom (custom fixer).
//! Skipped without the oracle.

use punphp_delegate::{Delegate, Outcome, Source};
use serde_json::json;
use std::path::{Path, PathBuf};

fn oracle() -> Option<PathBuf> {
    let p = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../.oracle")
        .canonicalize()
        .ok()?;
    p.join("pint/builds/pint").exists().then_some(p)
}

fn tmp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("pun-delegate-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

/// A project whose Composer install has PHP-CS-Fixer (the oracle's checkout, at the PHAR's bundled ref).
fn with_vendor(dir: &Path, oracle: &Path) {
    std::fs::create_dir_all(dir.join("vendor/friendsofphp/php-cs-fixer")).unwrap();
    let autoload = format!(
        "<?php\nreturn require '{}';\n",
        oracle.join("php-cs-fixer/vendor/autoload.php").display()
    );
    std::fs::write(dir.join("vendor/autoload.php"), autoload).unwrap();
}

fn custom_project(oracle: &Path, name: &str) -> PathBuf {
    let dir = tmp(name);
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/custom");
    std::fs::create_dir_all(dir.join("app")).unwrap();
    for f in [
        ".php-cs-fixer.php",
        "UpperTodoFixer.php",
        "app/A.php",
        "app/b.skip.php",
        "app/Clean.php",
    ] {
        std::fs::copy(src.join(f), dir.join(f)).unwrap();
    }
    with_vendor(&dir, oracle);
    dir
}

fn expected(name: &str) -> Vec<u8> {
    std::fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/custom/expected")
            .join(name),
    )
    .unwrap()
}

fn fixed(o: Outcome) -> (String, Vec<&'static str>) {
    match o {
        Outcome::Fixed(code, applied) => (String::from_utf8(code).unwrap(), applied),
        Outcome::Unchanged => panic!("unchanged"),
        Outcome::Failed(e) => panic!("failed: {e}"),
    }
}

/// .php-cs-fixer.php with a project fixer class: same files, bytes and applied fixers as php-cs-fixer. The fixer
/// echoes on stdout (must not corrupt the protocol) and is skipped where its supports() says no.
#[test]
fn custom_fixer_config() {
    let Some(oracle) = oracle() else { return };
    let dir = custom_project(&oracle, "custom");
    let d = Delegate::start(&dir, Source::Config(dir.join(".php-cs-fixer.php")), None).unwrap();
    assert_eq!(d.runtime, dir.join("vendor/autoload.php"));
    assert!(d.fixers.contains(&("Acme/upper_todo", 5)), "{:?}", d.fixers);

    // The rules as ConfigurationResolver resolves them (`pun migrate` reads them): sets expanded, custom ones kept.
    let rules = d.rules().unwrap();
    assert_eq!(
        (&rules["single_quote"], &rules["Acme/upper_todo"]),
        (&json!(true), &json!(true))
    );
    assert!(rules["braces_position"].is_object());
    assert!(!rules.contains_key("@PSR12"));
    let app = dir.join("app").canonicalize().unwrap();

    let read = |f: &str| std::fs::read(app.join(f)).unwrap();
    let (code, applied) = fixed(d.fix(&app.join("A.php"), &read("A.php"), &[]).unwrap());
    assert_eq!(code.as_bytes(), expected("A.php"));
    let order = [
        "class_definition",
        "single_quote",
        "Acme/upper_todo",
        "blank_line_after_opening_tag",
        "braces_position",
        "statement_indentation",
        "blank_line_after_namespace",
    ];
    assert_eq!(applied, order);
    let (code, applied) = fixed(
        d.fix(&app.join("b.skip.php"), &read("b.skip.php"), &[])
            .unwrap(),
    );
    assert_eq!(code.as_bytes(), expected("b.skip.php"));
    assert_eq!(applied, ["single_quote", "blank_line_after_opening_tag"]);
    assert!(matches!(
        d.fix(&app.join("Clean.php"), &read("Clean.php"), &[])
            .unwrap(),
        Outcome::Unchanged
    ));
}

/// A fixer exception fails that file only (Runner::processException); a dying worker aborts the run, and the next
/// request gets a fresh worker.
#[test]
fn worker_errors() {
    let Some(oracle) = oracle() else { return };
    let dir = custom_project(&oracle, "errors");
    let d = Delegate::start(&dir, Source::Config(dir.join(".php-cs-fixer.php")), None).unwrap();
    match d
        .fix(&dir.join("app/T.php"), b"<?php\n\n// THROW\n", &[])
        .unwrap()
    {
        Outcome::Failed(e) => assert_eq!(e, "boom"),
        _ => panic!("expected a failure"),
    }
    let e = d
        .fix(&dir.join("app/crash.php"), b"<?php\n\n// x\n", &[])
        .unwrap_err();
    assert_eq!(
        e.to_string(),
        "the PHP worker exited unexpectedly (see its output above)"
    );
    let (code, _) = fixed(
        d.fix(
            &dir.join("app/A.php"),
            &std::fs::read(dir.join("app/A.php")).unwrap(),
            &[],
        )
        .unwrap(),
    );
    assert_eq!(code.as_bytes(), expected("A.php"));
}

/// Pint mode: the merged rule map (a set, `false` entries) through the project's Pint PHAR, like Pint's Config.
/// Outputs and applied lists are Pint's `--format=json` for the same files and pint.json.
#[test]
fn pint_rules_via_project_phar() {
    let Some(oracle) = oracle() else { return };
    let dir = tmp("pint");
    std::fs::create_dir_all(dir.join("vendor/laravel/pint/builds")).unwrap();
    std::os::unix::fs::symlink(
        oracle.join("pint/builds/pint"),
        dir.join("vendor/laravel/pint/builds/pint"),
    )
    .unwrap();
    let rules = json!({"combine_consecutive_unsets": true, "single_quote": true, "@PSR12": true, "no_unused_imports": false, "braces_position": false});
    let d = Delegate::start(&dir, Source::Rules(rules), None).unwrap();
    assert_eq!(d.runtime, dir.join("vendor/laravel/pint/builds/pint"));
    assert!(d.fixers.contains(&("combine_consecutive_unsets", 24)));
    assert!(
        d.fixers.contains(&("class_definition", 36)),
        "@PSR12 expanded"
    );
    assert!(
        !d.fixers.iter().any(|(n, _)| *n == "braces_position"),
        "false disables a rule of the set"
    );

    let (code, applied) = fixed(
        d.fix(
            &dir.join("app/a.php"),
            b"<?php\nunset($a);\nunset($b);\n$c = \"x\";\n",
            &[],
        )
        .unwrap(),
    );
    assert_eq!(code, "<?php\n\nunset($a, $b);\n\n$c = 'x';\n");
    assert_eq!(
        applied,
        [
            "combine_consecutive_unsets",
            "single_quote",
            "blank_line_after_opening_tag"
        ]
    );
    // `skip` (the file's `@pun-ignore-file` fixers) works like the `@php-cs-fixer-ignore` case below.
    let (code, applied) = fixed(
        d.fix(
            &dir.join("app/a.php"),
            b"<?php\nunset($a);\nunset($b);\n$c = \"x\";\n",
            &["single_quote", "nope"],
        )
        .unwrap(),
    );
    assert_eq!(code, "<?php\n\nunset($a, $b);\n\n$c = \"x\";\n");
    assert_eq!(
        applied,
        ["combine_consecutive_unsets", "blank_line_after_opening_tag"]
    );
    let b = b"<?php\n// @php-cs-fixer-ignore single_quote\nunset($a);\nunset($b);\n$c = \"x\";\n";
    let (code, applied) = fixed(d.fix(&dir.join("app/b.php"), b, &[]).unwrap());
    assert_eq!(
        code,
        "<?php\n\n// @php-cs-fixer-ignore single_quote\nunset($a, $b);\n\n$c = \"x\";\n"
    );
    assert_eq!(
        applied,
        ["combine_consecutive_unsets", "blank_line_after_opening_tag"]
    );

    // Pint aborts the whole run with this message (Runner::validateRulesNamesForExceptions).
    let e = d
        .fix(
            Path::new("/tmp/dp1/app/c.php"),
            b"<?php\n// @php-cs-fixer-ignore nope\n$c = \"x\";\n",
            &[],
        )
        .unwrap_err();
    assert_eq!(
        e.to_string(),
        "@php-cs-fixer-ignore annotation(s) used for rules that are not in the current set of enabled rules:\n- nope (unknown rule)\n\n\
         Please check your annotation(s) usage in /tmp/dp1/app/c.php to ensure that these rules are included, or update your \
         annotation(s) usage if they have been replaced by other rules in the version of PHP CS Fixer you are using."
    );
}

/// Runtime lookup order and the config errors PHP-CS-Fixer reports at startup.
#[test]
fn runtimes_and_startup_errors() {
    let Some(oracle) = oracle() else { return };
    let dir = tmp("runtimes");
    let rules = || Source::Rules(json!({"single_quote": true}));
    let e = Delegate::start(&dir, rules(), None).err().unwrap();
    assert_eq!(
        e.to_string(),
        "no PHP-CS-Fixer to delegate to: run `composer require --dev laravel/pint`, set PUN_PINT to a Pint PHAR, or pass --native-only"
    );
    let e = Delegate::start(&dir, Source::Config(dir.join(".php-cs-fixer.php")), None)
        .err()
        .unwrap();
    assert_eq!(
        e.to_string(),
        "no vendor/friendsofphp/php-cs-fixer: run `composer install` so Pun can load the config's fixers"
    );

    // PUN_PINT (passed in by the CLI) wins; without a PHAR, a Composer install of PHP-CS-Fixer is used.
    let phar = oracle.join("pint/builds/pint");
    assert_eq!(
        Delegate::start(&dir, rules(), Some(phar.clone()))
            .unwrap()
            .runtime,
        phar
    );
    with_vendor(&dir, &oracle);
    let d = Delegate::start(&dir, rules(), None).unwrap();
    assert_eq!(d.runtime, dir.join("vendor/autoload.php"));
    let (code, _) = fixed(
        d.fix(&dir.join("a.php"), b"<?php\n$a = \"x\";\n", &[])
            .unwrap(),
    );
    assert_eq!(code, "<?php\n$a = 'x';\n");

    // Pint's own message for an unknown rule (`pint --config` with {"rules":{"nope":true}}).
    let e = Delegate::start(&dir, Source::Rules(json!({"nope": true})), None)
        .err()
        .unwrap();
    assert_eq!(e.to_string(), "The rules contain unknown fixers: \"nope\".");
    // php-cs-fixer's risky check for a config that doesn't allow risky rules.
    std::fs::write(
        dir.join("risky.php"),
        "<?php return (new PhpCsFixer\\Config())->setRules(['strict_comparison' => true]);",
    )
    .unwrap();
    let e = Delegate::start(&dir, Source::Config(dir.join("risky.php")), None)
        .err()
        .unwrap();
    assert_eq!(
        e.to_string(),
        "The rules contain risky fixers (\"strict_comparison\"), but they are not allowed to run. Perhaps you forget to use --allow-risky=yes option?"
    );
}

/// The cache signature changes with the installed packages.
#[test]
fn signature_tracks_packages() {
    let Some(oracle) = oracle() else { return };
    let dir = custom_project(&oracle, "signature");
    let d = Delegate::start(&dir, Source::Config(dir.join(".php-cs-fixer.php")), None).unwrap();
    let b = d.signature();
    std::fs::create_dir_all(dir.join("vendor/composer")).unwrap();
    std::fs::write(dir.join("vendor/composer/installed.json"), "{}").unwrap();
    assert_ne!(b, d.signature());
}
