//! rector.php to `[lint]`. The Rector classes and their Pun rules, sets and reasons are rules/mapping.toml's
//! `[upstream.rector]` rows; the expected sets come from Rector's set file names (config/set/*.php).
use super::*;
use crate::migrate::{Parts, Rector, RectorConfig, compose, from_rector};
use std::collections::BTreeSet;

const FOREACH: &str = "Rector\\DeadCode\\Rector\\Foreach_\\RemoveUnusedForeachKeyRector";
const AND_TRUE: &str = "Rector\\DeadCode\\Rector\\BooleanAnd\\RemoveAndTrueRector";
const MIRROR: &str = "Rector\\DeadCode\\Rector\\Expression\\SimplifyMirrorAssignRector";
const PROMOTION: &str =
    "Rector\\Php80\\Rector\\Class_\\ClassPropertyAssignToConstructorPromotionRector";
const RENAME: &str = "Rector\\Renaming\\Rector\\FuncCall\\RenameFunctionRector";

fn strings(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

/// Prepared sets map one to one (Pest's coding-style set is not Rector's), PHP sets are `modernize`; skips split into
/// rules off everywhere, rules off for some paths and excluded paths; what Pun has no rule for is named.
#[test]
fn reads_sets_rules_and_skips() {
    let root = dir(&[]);
    let at = |p: &str| format!("{}/{p}", root.display());
    let config = RectorConfig {
        rules: strings(&[
            FOREACH,
            "App\\Rector\\MyRector",
            PROMOTION,
            RENAME,
            MIRROR,
            AND_TRUE,
            FOREACH,
        ]),
        configured: strings(&[RENAME]),
        skip_rules: serde_json::from_value(
            json!({ MIRROR: null, AND_TRUE: [at("tests"), at("app/Legacy.php")] }),
        )
        .unwrap(),
        skip_paths: vec![at("legacy"), "*/Fixtures/*".into()],
        paths: vec![at("app"), at("tests/")],
        sets: vec![
            at("vendor/rector/rector/src/Set/ValueObject/../../../config/set/dead-code.php"),
            at("vendor/pestphp/pest-plugin-rector/src/Set/../../config/sets/coding-style.php"),
            at("vendor/rector/rector/src/Set/ValueObject/../../../config/set/type-declaration.php"),
            at("vendor/rector/rector/src/Set/ValueObject/../../../config/set/php84.php"),
        ],
        php_sets: true,
    };
    let r = from_rector(&root, &config);
    assert_eq!(r.sets, ["dead-code", "types", "modernize"]);
    let want: Vec<(&str, &str)> = r
        .want
        .iter()
        .map(|(a, b)| (a.as_str(), b.as_str()))
        .collect();
    assert_eq!(
        want,
        [
            (
                "lint/dead-code/no-unused-foreach-key",
                "RemoveUnusedForeachKeyRector"
            ),
            ("lint/dead-code/no-and-true", "RemoveAndTrueRector"),
        ]
    );
    let unsupported: Vec<(&str, &str)> = (r.unsupported.iter())
        .map(|(a, b)| (a.as_str(), b.as_str()))
        .collect();
    assert_eq!(
        unsupported,
        [
            (
                "MyRector",
                "not a Rector rule Pun knows (a custom rule or another package's)"
            ),
            (
                "ClassPropertyAssignToConstructorPromotionRector",
                "only in php52..php83 upgrade sets (below the PHP 8.4 floor)"
            ),
            (
                "RenameFunctionRector",
                "configured; its configuration can't be migrated yet"
            ),
        ]
    );
    assert_eq!(
        r.overrides,
        [(
            strings(&["tests", "app/Legacy.php"]),
            "lint/dead-code/no-and-true".to_string()
        )]
    );
    assert_eq!(r.exclude, ["legacy", "*/Fixtures/*"]);
    assert_eq!(r.paths, ["app", "tests"]);
    // The project itself, and a path outside it.
    let whole = RectorConfig {
        paths: vec![root.display().to_string(), "/elsewhere/src/".into()],
        ..RectorConfig::default()
    };
    assert_eq!(from_rector(&root, &whole).paths, [".", "/elsewhere/src"]);
}

/// The composed pun.toml runs exactly what Rector ran: each wanted rule Pun can run is on (from the set or its own
/// line), the set's other rules are off, and the per-path skip is an override.
#[test]
fn composes_the_difference_with_the_sets() {
    let root = dir(&[]);
    let id = |s: &str| s.to_string();
    let rector = Rector {
        sets: vec!["dead-code"],
        want: vec![
            (
                id("lint/dead-code/no-unused-foreach-key"),
                id("RemoveUnusedForeachKeyRector"),
            ),
            (
                id("lint/complexity/no-compact"),
                id("CompactToVariablesRector"),
            ),
            // A code-quality rule (ported), outside the dead-code set: its own line.
            (
                id("lint/complexity/no-and-assignment"),
                id("AndAssignsToSeparateLinesRector"),
            ),
            // Already on through the format config's line: no second line.
            (
                id("lint/imports/no-unused"),
                id("RemoveUnusedImportsRector"),
            ),
        ],
        overrides: vec![
            (
                strings(&["tests"]),
                id("lint/dead-code/no-unused-foreach-key"),
            ),
            (strings(&["tests"]), id("lint/complexity/no-compact")),
            (strings(&["app/A.php"]), id("lint/complexity/no-compact")),
        ],
        exclude: strings(&["legacy"]),
        ..Rector::default()
    };
    let mut parts = Parts {
        format: "preset = \"empty\"\n".into(),
        lint_rules: vec![(id("imports/no-unused"), id("\"error\""))],
        ..Parts::default()
    };
    let done = compose(&root, &mut parts, &rector).unwrap();
    let text = parts.render();
    assert!(
        text.starts_with(
            "[format]\npreset = \"empty\"\n\n[lint]\nsets = [\"dead-code\"]\nexclude = [\"legacy\"]\n\n[lint.rules]\n\"imports/no-unused\" = \"error\"\n"
        ),
        "{text}"
    );
    assert!(
        text.ends_with(
            "\n[[lint.overrides]]\nfiles = [\"tests\"]\nrules = { \"dead-code/no-unused-foreach-key\" = \"off\", \"complexity/no-compact\" = \"off\" }\n\n[[lint.overrides]]\nfiles = [\"app/A.php\"]\nrules = { \"complexity/no-compact\" = \"off\" }\n"
        ),
        "{text}"
    );
    let cfg = from_toml(&root, &text).unwrap();
    let on: BTreeSet<&str> = cfg.lint.rules.iter().map(|r| r.id.as_str()).collect();
    let ported = |id: &str| rule_meta_ported(id);
    // dead-code's rules other than the wanted one are off; compact runs through its own line when ported.
    let mut expect: BTreeSet<&str> = [
        "lint/dead-code/no-unused-foreach-key",
        "lint/imports/no-unused",
        "lint/complexity/no-and-assignment",
    ]
    .into();
    assert!(text.contains("\n\"complexity/no-and-assignment\" = \"error\"\n"));
    if ported("lint/complexity/no-compact") {
        expect.insert("lint/complexity/no-compact");
        assert_eq!((done.on, done.unsupported.len()), (2, 0));
    } else {
        assert_eq!(done.on, 1);
        assert_eq!(
            done.unsupported,
            [(
                id("CompactToVariablesRector"),
                id("not supported by Pun yet")
            )]
        );
    }
    assert_eq!(on, expect);
    assert_eq!(done.off, text.matches("= \"off\"\n").count());
    assert!(done.off > 0);

    // No prepared set and no skip: no empty `sets` or `exclude` line.
    let mut parts = Parts {
        format: "preset = \"empty\"\n".into(),
        ..Parts::default()
    };
    compose(&root, &mut parts, &Rector::default()).unwrap();
    assert!(!parts.render().contains("[lint]\n"), "{}", parts.render());
}

/// mapping.toml's `ported` flag of a rule.
fn rule_meta_ported(id: &str) -> bool {
    (rules().iter().find(|(r, _)| r == id))
        .and_then(|(_, e)| e.get("ported"))
        .and_then(toml::Value::as_bool)
        == Some(true)
}
