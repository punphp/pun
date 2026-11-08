//! rector.php to `[lint]`. The Rector classes and their Pun rules, sets and reasons are rules/mapping.toml's
//! `[upstream.rector]` rows; the expected sets come from Rector's set file names (config/set/*.php).
use super::*;
use crate::migrate::{Configured, Parts, Rector, RectorConfig, compose, from_rector};
use std::collections::{BTreeMap, BTreeSet};

const FOREACH: &str = "Rector\\DeadCode\\Rector\\Foreach_\\RemoveUnusedForeachKeyRector";
const AND_TRUE: &str = "Rector\\DeadCode\\Rector\\BooleanAnd\\RemoveAndTrueRector";
const MIRROR: &str = "Rector\\DeadCode\\Rector\\Expression\\SimplifyMirrorAssignRector";
const PROMOTION: &str =
    "Rector\\Php80\\Rector\\Class_\\ClassPropertyAssignToConstructorPromotionRector";
const RENAME: &str = "Rector\\Renaming\\Rector\\FuncCall\\RenameFunctionRector";
const NOT_EQUAL: &str = "Rector\\CodeQuality\\Rector\\NotEqual\\CommonNotEqualRector";

fn strings(v: &[&str]) -> Vec<String> {
    v.iter().map(std::string::ToString::to_string).collect()
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
            NOT_EQUAL,
        ]),
        configured: serde_json::from_value(json!({
            RENAME: {"configuration": {"split": "explode"}, "constants": []},
        }))
        .unwrap(),
        skip_rules: serde_json::from_value(
            json!({ MIRROR: null, AND_TRUE: [at("tests"), at("app/Legacy.php")], NOT_EQUAL: [at("tests")] }),
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
            ("lint/refactor/rename-function", "RenameFunctionRector"),
            ("lint/dead-code/no-and-true", "RemoveAndTrueRector"),
            (
                "format/operators/standardize-not-equals",
                "CommonNotEqualRector"
            ),
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

/// A configured rule's configuration becomes its options. The input is what crates/pun-cli/src/rector_config.php printed
/// for a rector.php with these four `withConfiguredRule()` calls, run on .oracle/rector-ecosystem's Rector: a data map
/// keeps its keys, a value object list goes under `items` with its properties kebab-cased, Rector's option constant
/// `exclude_methods` is kebab-cased. What TOML can't hold, or a key pun.toml keeps for the level, is named.
#[test]
fn carries_rule_configurations() {
    let root = dir(&[]);
    let printed = json!({"Rector\\Renaming\\Rector\\FuncCall\\RenameFunctionRector": {"configuration": {"split": "explode", "join": "implode"}, "constants": []}, "Rector\\Removing\\Rector\\FuncCall\\RemoveFuncCallArgRector": {"configuration": [{"@props": {"function": "ldap_first_attribute", "argumentPosition": 2}}], "constants": []}, "RectorLaravel\\Rector\\PropertyFetch\\OptionalToNullsafeOperatorRector": {"configuration": {"exclude_methods": ["present"]}, "constants": ["exclude_methods"]}, "Rector\\Renaming\\Rector\\ConstFetch\\RenameConstantRector": {"configuration": {"OLD": "NEW"}, "constants": []}});
    let mut configured: BTreeMap<String, Configured> = serde_json::from_value(printed).unwrap();
    // By hand: an unset value-object property is left out; a null, a closure and a `level` key can't be carried.
    let by_hand = [
        (
            "Rector\\Renaming\\Rector\\Cast\\RenameCastRector",
            json!([{"@props": {"fromCastKind": 3, "scope": null, "strict": true, "ratio": 0.5}}]),
        ),
        (
            "Rector\\Renaming\\Rector\\Name\\RenameClassRector",
            json!({"Old": null}),
        ),
        (
            // core's class is deprecated in 2.8.4; rector-laravel's copy is the one the rule maps to
            "RectorLaravel\\Rector\\FuncCall\\FuncCallToStaticCallRector",
            json!([{"@unsupported": "Closure"}]),
        ),
        (
            "Rector\\Renaming\\Rector\\ClassConstFetch\\RenameClassConstFetchRector",
            json!({"level": "x"}),
        ),
    ];
    for (class, configuration) in &by_hand {
        configured.insert(
            class.to_string(),
            Configured {
                configuration: configuration.clone(),
                constants: vec![],
            },
        );
    }
    let config = RectorConfig {
        rules: configured.keys().cloned().collect(),
        configured,
        ..RectorConfig::default()
    };
    let r = from_rector(&root, &config);
    let options: Vec<(&str, String)> = (r.options.iter())
        .map(|(id, t)| (id.as_str(), toml::Value::Table(t.clone()).to_string()))
        .collect();
    assert_eq!(
        options,
        [
            (
                "lint/laravel/prefer-nullsafe",
                "{ exclude-methods = [\"present\"] }".to_string()
            ),
            (
                "lint/refactor/remove-function-argument",
                "{ items = [{ function = \"ldap_first_attribute\", argument-position = 2 }] }"
                    .into()
            ),
            (
                "lint/refactor/rename-cast",
                "{ items = [{ from-cast-kind = 3, strict = true, ratio = 0.5 }] }".into()
            ),
            ("lint/refactor/rename-constant", "{ OLD = \"NEW\" }".into()),
            (
                "lint/refactor/rename-function",
                "{ split = \"explode\", join = \"implode\" }".into()
            ),
        ]
    );
    let unsupported: Vec<(&str, &str)> = (r.unsupported.iter())
        .map(|(a, b)| (a.as_str(), b.as_str()))
        .collect();
    assert_eq!(
        unsupported,
        [
            (
                "FuncCallToStaticCallRector",
                "configured; its configuration holds a Closure, which pun.toml can't"
            ),
            (
                "RenameClassConstFetchRector",
                "configured; its configuration has a `level` key, which pun.toml keeps for the rule's level"
            ),
            (
                "RenameClassRector",
                "configured; its configuration holds a null, which pun.toml can't"
            ),
        ]
    );

    // Composed: a ported rule gets its options on its line, even when a set already turns it on.
    // lint/complexity/no-and-assignment is ported and in code-quality (mapping.toml); its options are made up.
    let rector = Rector {
        sets: vec!["code-quality"],
        want: vec![
            (
                "lint/complexity/no-and-assignment".to_string(),
                "AndAssignsToSeparateLinesRector".to_string(),
            ),
            (
                "lint/refactor/rename-function".to_string(),
                "RenameFunctionRector".to_string(),
            ),
        ],
        options: BTreeMap::from([
            (
                "lint/complexity/no-and-assignment".to_string(),
                toml::Table::from_iter([("items".to_string(), toml::Value::Integer(1))]),
            ),
            (
                "lint/refactor/rename-function".to_string(),
                r.options["lint/refactor/rename-function"].clone(),
            ),
        ]),
        ..Rector::default()
    };
    let mut parts = Parts {
        format: "preset = \"empty\"\n".into(),
        ..Parts::default()
    };
    let done = compose(&root, &mut parts, &rector).unwrap();
    let text = parts.render();
    assert!(
        text.contains("\n\"complexity/no-and-assignment\" = { items = 1 }\n"),
        "{text}"
    );
    // lint/refactor/rename-function is ported, so it gets its line with the options.
    assert!(rule_meta_ported("lint/refactor/rename-function"));
    assert!(
        text.contains(
            "\n\"refactor/rename-function\" = { split = \"explode\", join = \"implode\" }\n"
        ),
        "{text}"
    );
    assert_eq!(done.on, 2);
    let cfg = from_toml(&root, &text).unwrap();
    let rule = cfg
        .lint
        .rules
        .iter()
        .find(|r| r.id == "lint/complexity/no-and-assignment");
    assert_eq!(rule.unwrap().options, json!({"items": 1}));
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

    // No prepared set and no skip: no empty `sets` or `exclude` line. A Rector rule that is a format rule in Pun
    // writes nothing: the laravel preset runs standardize-not-equals (presets/laravel.json), an empty one is named.
    let not_equal = Rector {
        want: vec![(
            id("format/operators/standardize-not-equals"),
            id("CommonNotEqualRector"),
        )],
        ..Rector::default()
    };
    for (preset, unsupported) in [
        ("laravel", vec![]),
        (
            "empty",
            vec![(
                id("CommonNotEqualRector"),
                id(
                    "the format rule `format/operators/standardize-not-equals` in Pun; turn it on in [format.rules]",
                ),
            )],
        ),
    ] {
        let mut parts = Parts {
            format: format!("preset = \"{preset}\"\n"),
            ..Parts::default()
        };
        let done = compose(&root, &mut parts, &not_equal).unwrap();
        assert_eq!((done.on, done.unsupported), (0, unsupported));
        assert!(!parts.render().contains("[lint]\n"), "{}", parts.render());
        assert!(
            !parts.render().contains("standardize"),
            "{}",
            parts.render()
        );
    }
}

/// mapping.toml's `ported` flag of a rule.
fn rule_meta_ported(id: &str) -> bool {
    (rules().iter().find(|(r, _)| r == id))
        .and_then(|(_, e)| e.get("ported"))
        .and_then(toml::Value::as_bool)
        == Some(true)
}
