//! `pun init` end to end on scratch projects: a Laravel 13 + Pest 4 app and a plain PHP project. The expected output is
//! context/plan/init.md §2 written out by hand (without the check and baseline lines, which wait for init 4); the
//! error wording is crates/pun-config/src/schema.rs's.
mod common;

use common::pty::{pun, pun_with};
use common::read;
use pun_config::{AnalysePreset, Options};
use std::path::{Path, PathBuf};

const LARAVEL_JSON: &str = r#"{"require":{"php":"^8.4","laravel/framework":"^13.0"},"require-dev":{"pestphp/pest":"^4.0"}}"#;
const LARAVEL_LOCK: &str = r#"{"packages":[{"name":"laravel/framework","version":"v13.2.0"}],"packages-dev":[{"name":"pestphp/pest","version":"v4.1.2"},{"name":"phpunit/phpunit","version":"12.3.4"}]}"#;
const PLAIN_JSON: &str = r#"{"require":{"php":">=8.4"}}"#;
/// The terminal env: a UTF-8 locale for the glyphs, no colour.
const ENV: &[(&str, &str)] = &[("LANG", "C.UTF-8"), ("NO_COLOR", "1")];

fn laravel(name: &str, extra: &[(&str, &str)]) -> PathBuf {
    let mut files = vec![
        ("composer.json", LARAVEL_JSON),
        ("composer.lock", LARAVEL_LOCK),
    ];
    files.extend_from_slice(extra);
    common::project("init", name, &files)
}

fn plain(name: &str) -> PathBuf {
    common::project("init", name, &[("composer.json", PLAIN_JSON)])
}

fn toml(style: &str, sets: &str, analyse: &str) -> String {
    format!(
        "# Written by `pun init`.\n[format]\npreset = \"{style}\"\n\n[lint]\nsets = [{sets}]\n\n[analyse]\npreset = \"{analyse}\"\n"
    )
}

/// What `pun_config::load` makes of the written pun.toml: the format preset, the lint sets, the analyse preset.
fn loaded(d: &Path) -> (String, Vec<String>, AnalysePreset) {
    let c = pun_config::load(d, &Options::default()).unwrap();
    (c.preset, c.lint.sets, c.analyse.preset)
}

const NEXT: &str = "\n  ✔ Wrote pun.toml\n\n  Done. Run `pun check` any time; `pun check --fix` before you commit.\n  Add `pun check` to CI and your composer scripts.\n";

/// No terminal: nothing is asked, the Laravel app gets the laravel style and the defaults.
#[test]
fn laravel_defaults_without_a_terminal() {
    let d = laravel("laravel", &[]);
    let (out, err, code) = pun_with(&d, &["init"], ENV);
    assert_eq!((err.as_str(), code), ("", Some(0)));
    assert_eq!(
        out,
        format!(
            "\n  Pun · new project setup\n  Found Laravel 13, Pest 4, PHP ^8.4 (composer.json, composer.lock)\n\nCode style: laravel\nLint sets: dead-code, modernize\nAnalysis preset: recommended\n\n  Laravel, Pest and PHPUnit rules turn on by themselves (composer.lock).\n{NEXT}"
        )
    );
    assert_eq!(
        read(&d, "pun.toml"),
        toml("laravel", "\"dead-code\", \"modernize\"", "recommended")
    );
    let sets = vec!["dead-code".to_string(), "modernize".into()];
    assert_eq!(
        loaded(&d),
        ("laravel".into(), sets, AnalysePreset::Recommended)
    );

    // An existing pun.toml is never overwritten: exit 2, the file as it was.
    let (out, err, code) = pun_with(&d, &["init", "--style", "psr12"], ENV);
    assert_eq!(
        (out.as_str(), err.as_str(), code),
        (
            "",
            "✖ pun.toml exists; `pun init` doesn't overwrite it. Edit it, or move it away and run `pun init` again.\n",
            Some(2)
        )
    );
    assert!(read(&d, "pun.toml").contains("preset = \"laravel\""));
}

/// A project without Laravel gets psr12; `--yes` asks nothing either. ASCII glyphs without a UTF-8 locale.
#[test]
fn plain_project_gets_psr12() {
    let d = plain("plain");
    let (out, _, code) = pun(&d, &["init", "--yes"]);
    assert_eq!(code, Some(0));
    assert_eq!(
        out,
        "\n  Pun · new project setup\n  Found PHP >=8.4 (composer.json)\n\nCode style: psr12\nLint sets: dead-code, modernize\nAnalysis preset: recommended\n\n  ok Wrote pun.toml\n\n  Done. Run `pun check` any time; `pun check --fix` before you commit.\n  Add `pun check` to CI and your composer scripts.\n"
    );
    assert_eq!(
        read(&d, "pun.toml"),
        toml("psr12", "\"dead-code\", \"modernize\"", "recommended")
    );

    // Without composer.json at all, the header says so.
    let d = common::project("init", "bare", &[]);
    let (out, _, _) = pun(&d, &["init"]);
    assert!(
        out.starts_with("\n  Pun · new project setup\n  No Laravel, Pest, PHPUnit or PHP version found in composer.json or composer.lock\n\nCode style: psr12\n"),
        "{out}"
    );
}

/// The flags replace the defaults; `--lint-sets ""` is no sets. PHPUnit is named when there is no Pest.
#[test]
fn flags_pick_the_choices() {
    let lock = r#"{"packages-dev":[{"name":"phpunit/phpunit","version":"dev-main"}]}"#;
    let d = laravel("flags", &[("composer.lock", lock)]);
    let args = [
        "init",
        "--style",
        "laravel",
        "--lint-sets",
        "types,naming",
        "--analyse",
        "max",
    ];
    let (out, _, code) = pun_with(&d, &args, ENV);
    assert_eq!(code, Some(0));
    assert!(
        out.starts_with("\n  Pun · new project setup\n  Found PHPUnit dev-main, PHP ^8.4 (composer.json, composer.lock)\n\nCode style: laravel\nLint sets: types, naming\nAnalysis preset: max\n\n  PHPUnit rules turn on by themselves (composer.lock).\n"),
        "{out}"
    );
    assert_eq!(
        read(&d, "pun.toml"),
        toml("laravel", "\"types\", \"naming\"", "max")
    );
    let sets = vec!["types".to_string(), "naming".into()];
    assert_eq!(loaded(&d), ("laravel".into(), sets, AnalysePreset::Max));

    let d = plain("no-sets");
    let (out, _, _) = pun(&d, &["init", "--lint-sets", "", "--analyse", "basic"]);
    assert!(
        out.contains("Lint sets: none\nAnalysis preset: basic\n"),
        "{out}"
    );
    assert_eq!(read(&d, "pun.toml"), toml("psr12", "", "basic"));
    assert_eq!(loaded(&d), ("psr12".into(), vec![], AnalysePreset::Basic));
}

/// An unknown set or preset is a run error in schema.rs's words, and nothing is written. `empty` is not a style init
/// takes (clap rejects it, exit 2).
#[test]
fn bad_flags_are_rejected() {
    let d = plain("bad");
    let (out, err, code) = pun_with(&d, &["init", "--lint-sets", "dead-code,speed"], ENV);
    assert_eq!(
        (out.as_str(), err.as_str(), code),
        (
            "",
            "✖ --lint-sets: `speed` is not a lint set; the sets are `dead-code`, `modernize`, `code-quality`, `types`, `early-return`, `privatization`, `naming`, `coding-style`\n",
            Some(2)
        )
    );
    let (_, err, code) = pun_with(&d, &["init", "--analyse", "level5"], ENV);
    assert_eq!(
        (err.as_str(), code),
        (
            "✖ --analyse level5: expected `basic`, `essential`, `recommended`, `strict`, `max`\n",
            Some(2)
        )
    );
    let (_, err, code) = pun(&d, &["init", "--style", "empty"]);
    assert_eq!(code, Some(2));
    assert!(
        err.contains("invalid value 'empty' for '--style <STYLE>'"),
        "{err}"
    );
    assert!(!d.join("pun.toml").exists());
}

/// An old config without a terminal: init names `pun migrate`, writes nothing and exits 2 (it did not do what was
/// asked; context/plan/init.md, assistant's call). Agent mode gets the same as one error line.
#[test]
fn old_config_without_a_terminal_points_to_migrate() {
    let d = laravel("old", &[("pint.json", "{}"), ("phpstan.neon", "")]);
    let (out, err, code) = pun_with(&d, &["init", "--yes"], ENV);
    assert_eq!(
        (out.as_str(), err.as_str(), code),
        (
            "",
            "✖ Found pint.json and phpstan.neon: this project fits `pun migrate`, which turns them into pun.toml. Nothing written.\n",
            Some(2)
        )
    );
    let (out, _, code) = pun_with(&d, &["init"], &[("PUN_AGENT", "1")]);
    assert_eq!(
        (out.as_str(), code),
        (
            "pun init: error: Found pint.json and phpstan.neon: this project fits `pun migrate`, which turns them into pun.toml. Nothing written.\n",
            Some(2)
        )
    );
    assert!(!d.join("pun.toml").exists());
}

/// Agent mode asks nothing and reports in one line, the next steps as its item.
#[test]
fn agent_mode() {
    let d = laravel("agent", &[]);
    let (out, _, code) = pun_with(&d, &["init"], &[("PUN_AGENT", "1")]);
    assert_eq!(code, Some(0));
    let (summary, item) = out.split_once('\n').unwrap();
    assert!(
        summary.starts_with("pun init: ok, pun.toml written, style laravel, lint sets dead-code modernize, analyse recommended"),
        "{out}"
    );
    assert_eq!(
        item,
        "next: run `pun check`, `pun check --fix` before committing; add `pun check` to CI and composer scripts\n"
    );
    assert_eq!(
        read(&d, "pun.toml"),
        toml("laravel", "\"dead-code\", \"modernize\"", "recommended")
    );
}

/// On a terminal: three enters take the defaults, and the flow ends as init.md §2 does.
#[test]
fn terminal_accepts_the_defaults() {
    let d = laravel("tty", &[]);
    let (out, code) = common::pty::pty(&d, "init", "\\r\\r\\r", ENV);
    assert_eq!(code, Some(0), "{out}");
    assert!(
        out.contains("  Pun · new project setup\n  Found Laravel 13, Pest 4, PHP ^8.4 (composer.json, composer.lock)\n"),
        "{out}"
    );
    for answer in [
        "Code style: laravel",
        "Lint sets (space toggles, enter confirms): dead-code, modernize",
        "Analysis preset: recommended",
    ] {
        assert!(out.contains(answer), "{answer}: {out}");
    }
    assert!(
        out.ends_with(&format!(
            "\n  Laravel, Pest and PHPUnit rules turn on by themselves (composer.lock).\n{NEXT}"
        )),
        "{out}"
    );
    assert_eq!(
        read(&d, "pun.toml"),
        toml("laravel", "\"dead-code\", \"modernize\"", "recommended")
    );

    // Down from psr12 wraps to laravel; tick `code-quality` too; one down to `strict`.
    let d = plain("tty-pick");
    let (out, code) = common::pty::pty(&d, "init", "\\033[B\\r\\033[B\\033[B \\r\\033[B\\r", ENV);
    assert_eq!(code, Some(0), "{out}");
    assert!(out.contains("Code style: laravel"), "{out}");
    assert_eq!(
        read(&d, "pun.toml"),
        toml(
            "laravel",
            "\"dead-code\", \"modernize\", \"code-quality\"",
            "strict"
        )
    );

    // CI asks nothing, even on a terminal.
    let d = plain("tty-ci");
    let (out, code) = common::pty::pty(&d, "init", "", &[("CI", "true"), ENV[0], ENV[1]]);
    assert_eq!(code, Some(0), "{out}");
    assert!(
        out.contains("\nCode style: psr12\nLint sets: dead-code, modernize\n"),
        "{out}"
    );
}

/// On a terminal an old config is offered to `pun migrate`: yes runs it (its checklist then takes enter), no goes on
/// with init.
#[test]
fn terminal_offers_migrate() {
    let d = laravel("tty-migrate", &[("pint.json", r#"{"preset":"psr12"}"#)]);
    let (out, code) = common::pty::pty(&d, "init", "\\r\\r", ENV);
    assert_eq!(code, Some(0), "{out}");
    assert!(
        out.contains("Found pint.json. Migrate it instead?"),
        "{out}"
    );
    assert!(
        out.ends_with("pun migrate: pun.toml written from pint.json, ignore comments rewritten in 0 file(s).\n  pint.json  migrated\n"),
        "{out}"
    );
    assert!(read(&d, "pun.toml").starts_with("# Written by `pun migrate` from pint.json.\n"));

    let d = laravel("tty-no-migrate", &[("pint.json", "{}")]);
    let (out, code) = common::pty::pty(&d, "init", "n\\r\\r\\r", ENV);
    assert_eq!(code, Some(0), "{out}");
    assert!(out.ends_with(NEXT), "{out}");
    assert_eq!(
        read(&d, "pun.toml"),
        toml("laravel", "\"dead-code\", \"modernize\"", "recommended")
    );
}
