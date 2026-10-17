use super::*;
use serde_json::json;

fn dir(files: &[(&str, &str)]) -> PathBuf {
    static N: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let d = std::env::temp_dir().join(format!("pun-config-{}-{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    for (n, c) in files {
        std::fs::write(d.join(n), c).unwrap();
    }
    d
}

fn err(d: &Path, o: Options) -> String {
    format!("{:#}", load(d, &o).unwrap_err())
}

fn names(rules: &[Rule]) -> Vec<&str> {
    rules.iter().map(|r| r.name.as_str()).collect()
}

fn preset(p: &str) -> Options {
    Options {
        preset: Some(p.into()),
        ..Default::default()
    }
}

#[test]
fn needs_pun_toml() {
    let d = dir(&[("pint.json", r#"{"preset":"psr12"}"#)]);
    assert_eq!(err(&d, Options::default()), NO_CONFIG);
    // An explicit preset or --no-config runs without one (pint.json is never read).
    assert_eq!(load(&d, &preset("psr12")).unwrap().preset, "psr12");
    let c = load(
        &d,
        &Options {
            no_config: true,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!((c.preset.as_str(), c.sources.len()), ("laravel", 0));
    // --config names the file; a missing one is an error, not "no config".
    let o = Options {
        config: Some("other.toml".into()),
        ..Default::default()
    };
    assert!(err(&d, o.clone()).starts_with("can't read "));
    std::fs::write(d.join("other.toml"), "[format]\npreset = \"empty\"\n").unwrap();
    let c = load(&d, &o).unwrap();
    assert_eq!(
        (c.preset.as_str(), c.sources),
        ("empty", vec![d.join("other.toml")])
    );
    assert_eq!(err(&d, preset("nope")), "Preset not found.");
}

#[test]
fn pun_toml() {
    let d = dir(&[
        ("composer.json", r#"{"require":{"php":"^8.4"}}"#),
        (
            "pun.toml",
            r#"[php]
version = "8.5"
[format]
preset = "laravel"
paths = ["src"]
exclude = ["a"]
not-name = ["n.php"]
not-path = ["p"]
[format.rules]
concat-space = { spacing = "one" }
"format.array-syntax" = false
laravel-blade = true
[lint.rules]
no-alias-function = { fix = "apply" }
"lint.require-final-class" = "warn"
require-strict-types = "off"
no-unreachable-default-argument-value = { severity = "error", fix = "safe-only" }
prefer-mb-functions = false
no-loose-comparison = true
require-ordered-traits = { fix = "off" }
prefer-cast-over-intval = { fix = "apply", severity = "warn" }
[analyse]
paths = ["app"]
"#,
        ),
    ]);
    let c = load(&d, &Options::default()).unwrap();
    let n = names(&c.rules);
    // `false` drops a preset rule; overrides keep the preset's position.
    assert!(!n.contains(&"array_syntax"));
    let concat = n.iter().position(|n| *n == "concat_space").unwrap();
    assert!(concat < n.iter().position(|n| *n == "Pint/laravel_blade").unwrap());
    assert_eq!(c.rules[concat].options, json!({"spacing": "one"}));
    // The laravel preset's 4 risky fixers are lint rules: off unless [lint.rules] turns them on.
    for risky in [
        "no_alias_functions",
        "no_unreachable_default_argument_value",
        "ordered_traits",
        "php_unit_set_up_tear_down_visibility",
    ] {
        assert!(!n.contains(&risky), "{risky}");
    }
    assert!(c.blade);
    let lint: Vec<(&str, bool)> = c.lint.iter().map(|(r, a)| (r.name.as_str(), *a)).collect();
    assert_eq!(
        lint,
        [
            ("no_alias_functions", true),
            ("final_class", false),
            ("no_unreachable_default_argument_value", false),
            ("strict_comparison", false),
            ("ordered_traits", false),
            ("modernize_types_casting", true),
        ]
    );
    assert!(c.lint.iter().all(|(r, _)| r.options.is_null()));
    assert_eq!(
        (
            c.finder.in_dirs,
            c.finder.exclude,
            c.finder.not_name,
            c.finder.not_path
        ),
        (
            vec!["src".into()],
            vec!["a".into()],
            vec!["n.php".into()],
            vec!["p".into()]
        )
    );
    assert_eq!(c.php, "8.5");
    assert_eq!(c.sources, [d.join("composer.json"), d.join("pun.toml")]);
    // Options keep their values; only top-level keys go back to PHP-CS-Fixer's snake_case.
    let d = dir(&[(
        "pun.toml",
        "[format]\npreset = \"empty\"\n[format.rules]\nordered-imports = { sort-algorithm = \"alpha\", imports-order = [\"class\", \"const\"] }\n",
    )]);
    let c = load(
        &d,
        &Options {
            blade: true,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(names(&c.rules), ["ordered_imports", "Pint/laravel_blade"]);
    assert_eq!(
        c.rules[0].options,
        json!({"sort_algorithm": "alpha", "imports_order": ["class", "const"]})
    );
    assert!(c.blade);
    assert_eq!(c.php, "8.4");
    // Earlier Pun names (`aliases` in mapping.toml) still work.
    let d = dir(&[(
        "pun.toml",
        "[format]\npreset = \"empty\"\n[format.rules]\ncurly-to-square-bracket-array-string = true\n",
    )]);
    assert_eq!(
        names(&load(&d, &Options::default()).unwrap().rules),
        ["normalize_index_brace"]
    );
    let d = dir(&[("pun.toml", "[lint.rules]\nfinal-class = true\n")]);
    assert_eq!(
        load(&d, &Options::default()).unwrap().lint[0].0.name,
        "final_class"
    );
}

#[test]
fn pun_toml_errors() {
    let e = |toml: &str| err(&dir(&[("pun.toml", toml)]), Options::default());
    // The four old tiers; Step 1 replaces them (context/plan/config.md §5).
    for tier in ["essential", "recommended", "strict", "pedantic"] {
        let toml = format!("preset = \"{tier}\"\n");
        assert!(load(&dir(&[("pun.toml", &toml)]), &Options::default()).is_ok());
    }
    assert!(e("preset = \"laravel\"\n").contains(
        "unknown variant `laravel`, expected one of `essential`, `recommended`, `strict`, `pedantic`"
    ));
    assert!(e("[format]\nexlude = [\"typo\"]\n").contains("unknown field `exlude`"));
    assert!(e("[format.rules]\nnope = true\n").ends_with(
        "`nope` is not a format rule Pun can run (see crates/pun-config/rules/mapping.toml)"
    ));
    assert!(e("[lint.rules]\nnope = true\n").contains("`nope` is not a lint rule"));
    assert!(
        e("[format.rules]\nfinal-class = true\n")
            .ends_with("`final-class` is `lint.require-final-class` in pun.toml")
    );
    // Upstream names and rules of the other command name the ID to use.
    assert!(
        e("[format.rules]\narray_syntax = true\n")
            .ends_with("`array_syntax` is `format.array-syntax` in pun.toml")
    );
    assert!(
        e("[lint.rules]\nordered-imports = true\n")
            .ends_with("`ordered-imports` is `format.ordered-imports` in pun.toml")
    );
    assert!(
        e("[format.rules]\n\"format.require-final-class\" = true\n")
            .ends_with("`format.require-final-class` is `lint.require-final-class` in pun.toml")
    );
    assert!(
        e("[format.rules]\nsingle-quote = \"yes\"\n")
            .ends_with("`single-quote` must be true, false or a table of options")
    );
    assert!(
        e("[lint.rules]\nrequire-final-class = 1\n")
            .ends_with("`require-final-class` must be true, false or a table of options")
    );
    assert!(
        e("[lint.rules]\nrequire-final-class = \"loud\"\n").ends_with(
            "`require-final-class` = \"loud\": expected \"error\", \"warn\", \"info\" or \"off\""
        )
    );
    assert!(
        e("[lint.rules]\nrequire-final-class = { fix = \"yes\" }\n")
            .ends_with("`require-final-class`: fix must be \"apply\", \"safe-only\" or \"off\"")
    );
}

#[test]
fn composer_gate() {
    let d = dir(&[("composer.json", r#"{"require":{"php":"^8.2"}}"#)]);
    assert!(err(&d, preset("laravel")).contains("PHP < 8.4"));
    // The harness's escape hatch for pre-8.4 corpora. ponytail: process-wide env; no other test reads it.
    unsafe { std::env::set_var("PUN_IGNORE_PHP_CONSTRAINT", "1") };
    let ok = load(&d, &preset("laravel")).is_ok();
    unsafe { std::env::remove_var("PUN_IGNORE_PHP_CONSTRAINT") };
    assert!(ok);
    assert!(load(&dir(&[("composer.json", "{}")]), &preset("laravel")).is_ok());
    let d = dir(&[("composer.json", "{")]);
    assert!(err(&d, preset("laravel")).starts_with("invalid "));
    let d = dir(&[("composer.json", r#"{"require":{"php":">="}}"#)]);
    assert!(err(&d, preset("laravel")).starts_with("invalid constraint"));
}

#[test]
fn mapping_lookups() {
    assert_eq!(
        rule_of("no_alias_functions"),
        Some("lint.no-alias-function")
    );
    assert_eq!(rule_of("Pint/laravel_blade"), Some("format.laravel-blade"));
    assert_eq!(rule_of("Pun/no_only"), Some("lint.no-only"));
    assert_eq!(rule_of("braces"), None);
    assert_eq!(
        fixer_of("format", "format.ordered-imports"),
        Some("ordered_imports")
    );
    assert_eq!(
        fixer_of("format", "ordered-imports"),
        Some("ordered_imports")
    );
    assert_eq!(fixer_of("lint", "ordered-imports"), None);
}

#[test]
fn rule_metadata() {
    // Hand-read from rules/mapping.toml.
    let m = rule("lint.no-self-assignment").unwrap();
    assert_eq!(
        (m.fix, m.tier, m.summary, m.aliases.as_slice()),
        (
            "safe",
            Some("recommended"),
            None,
            &["simplify-mirror-assign"][..]
        )
    );
    let m = rule("lint.require-strict-param").unwrap();
    assert_eq!((m.fix, m.tier), ("mixed", Some("strict")));
    let m = rule("format.array-syntax").unwrap();
    assert_eq!(
        (m.fix, m.tier, m.summary, m.aliases.len()),
        (
            "safe",
            None,
            Some("PHP arrays should be declared using the configured syntax."),
            0
        )
    );
    // Analyse rules have no fix.
    let m = rule("analyse.argument-count").unwrap();
    assert_eq!((m.fix, m.tier), ("none", Some("essential")));
    assert!(rule("no-self-assignment").is_none());
    assert!(rule("lint.array-syntax").is_none());
}

#[test]
fn show() {
    let d = dir(&[(
        "pun.toml",
        "[format]\npreset = \"empty\"\n[format.rules]\nconcat-space = { spacing = \"one\" }\nsingle-quote = true\n[lint.rules]\nrequire-final-class = { fix = \"apply\" }\nno-alias-function = { sets = [\"@internal\"] }\n",
    )]);
    assert_eq!(
        load(&d, &Options::default()).unwrap().show(),
        json!({
            "sources": [d.join("pun.toml")],
            "php": {"version": "8.4"},
            "format": {"preset": "empty", "rules": {"format.concat-space": {"spacing": "one"}, "format.single-quote": true}},
            "lint": {"rules": {"lint.require-final-class": {"fix": "apply"}, "lint.no-alias-function": {"sets": ["@internal"]}}},
            "finder": {"in": [], "exclude": [], "notName": [], "notPath": []},
        })
    );
    // A rule Pun can't run (a set it can't expand) shows under its upstream name.
    let c = load(&dir(&[]), &preset("per")).unwrap();
    assert_eq!(c.show()["format"]["rules"]["@PER-CS"], true);
}

/// Rule sets resolve like PHP-CS-Fixer's RuleSet (tests/rule_sets.json: `(new RuleSet($rules))->getRules()` of the
/// pinned version): same rules, options and order. The first case is Pint's psr12 preset.
#[test]
fn rule_sets_like_php_cs_fixer() {
    let cases: Vec<Value> = serde_json::from_str(include_str!("../tests/rule_sets.json")).unwrap();
    let resolved = |rules: Vec<Rule>| -> Vec<(String, Value)> {
        rules.into_iter().map(|r| (r.name, r.options)).collect()
    };
    let expected = |case: &Value| -> Vec<(String, Value)> {
        let m = case["expected"].as_object().unwrap();
        m.iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    if *v == Value::Bool(true) {
                        Value::Null
                    } else {
                        v.clone()
                    },
                )
            })
            .collect()
    };
    let psr12 = load(&dir(&[]), &preset("psr12")).unwrap();
    assert_eq!(resolved(psr12.rules), expected(&cases[0]));
    for case in &cases {
        let rules = case["rules"].as_object().unwrap().clone();
        assert_eq!(resolved(rule_set(rules).unwrap()), expected(case));
    }
    // Sets pun has no definition for stay as one rule, so they are reported as not ported.
    let rules = json!({"@PER-CS": true}).as_object().unwrap().clone();
    assert_eq!(
        resolved(rule_set(rules).unwrap()),
        [("@PER-CS".to_string(), Value::Null)]
    );
    let rules = json!({"@PSR12": {}}).as_object().unwrap().clone();
    assert_eq!(
        rule_set(rules).unwrap_err().to_string(),
        "Nested rule set \"@PSR12\" configuration must be a boolean."
    );
}

mod migrate {
    use super::*;
    use crate::migrate::{from_pint, from_rules};

    fn pint(json: &str) -> crate::migrate::Migration {
        from_pint(&dir(&[("pint.json", json)]).join("pint.json"), None).unwrap()
    }

    /// Hand-derived from Pint's laravel preset (docs/presets/laravel.json) and the mapping.
    #[test]
    fn writes_a_delta_and_moves_risky_rules_to_lint() {
        let m = pint(
            r#"{"exclude":["a"],"notPath":"y","rules":{"concat_space":{"spacing":"one"},"array_syntax":false,"single_quote":true,"final_class":true,"Pint/laravel_blade":true}}"#,
        );
        assert!(m.unsupported.is_empty());
        assert_eq!(
            m.toml,
            r#"[format]
preset = "laravel"
exclude = ["a"]
not-path = ["y"]

[format.rules]
concat-space = { spacing = "one" }
laravel-blade = true
array-syntax = false

[lint.rules]
no-alias-function = { fix = "apply" }
no-unreachable-default-argument-value = { fix = "apply" }
require-ordered-traits = { fix = "apply" }
require-protected-setup = { fix = "apply" }
require-final-class = { fix = "apply" }
"#
        );
        // Re-running gives the same file; laravel's single_quote already is `true`, so it isn't written.
        assert_eq!(
            pint(r#"{"rules":{"single_quote":true}}"#).toml,
            pint("{}").toml
        );
    }

    /// The migrated pun.toml runs exactly what Pint ran: `pun format`'s rules plus the applied lint rules equal
    /// PHP-CS-Fixer's resolved rule list (checked against its fixture in `rule_sets_like_php_cs_fixer`).
    #[test]
    fn round_trips() {
        for (preset, json) in [
            (
                "psr12",
                r#"{"preset":"psr12","in":"app","rules":{"concat_space":{"spacing":"one"},"braces_position":false,"no_alias_functions":{"sets":["@all"]},"@PSR2":true}}"#,
            ),
            (
                "laravel",
                r#"{"rules":{"ordered_imports":{"sort_algorithm":"length"},"no_unused_imports":false,"strict_comparison":true}}"#,
            ),
            (
                "empty",
                r#"{"preset":"empty","rules":{"@PSR12":true,"@PSR2":false,"single_quote":true}}"#,
            ),
        ] {
            let m = pint(json);
            let c = from_toml(&dir(&[]), &m.toml).unwrap();
            assert_eq!(c.preset, preset);
            let mut want = preset_rules(preset).unwrap();
            want.extend(
                serde_json::from_str::<Value>(json).unwrap()["rules"]
                    .as_object()
                    .unwrap()
                    .clone(),
            );
            let want = rule_set(want).unwrap();
            let mut got: Vec<(String, Value)> =
                (c.rules.iter().chain(c.lint.iter().map(|(r, _)| r)))
                    .map(|r| (r.name.clone(), r.options.clone()))
                    .collect();
            let mut want: Vec<(String, Value)> =
                want.into_iter().map(|r| (r.name, r.options)).collect();
            got.sort_by(|a, b| a.0.cmp(&b.0));
            want.sort_by(|a, b| a.0.cmp(&b.0));
            assert_eq!(got, want, "{json}");
            assert!(c.lint.iter().all(|(_, apply)| *apply));
        }
    }

    #[test]
    fn visibility_required_becomes_modifier_keywords() {
        let mk = |json: &str| {
            let c = from_toml(&dir(&[]), &pint(json).toml).unwrap();
            assert!(!names(&c.rules).contains(&"visibility_required"));
            c.rules
                .into_iter()
                .find(|r| r.name == "modifier_keywords")
                .map(|r| r.options)
        };
        // Pinkary: laravel's modifier_keywords (method, property) + visibility_required (all elements).
        assert_eq!(
            mk(r#"{"rules":{"visibility_required":true}}"#),
            Some(Value::Null)
        );
        assert_eq!(
            mk(r#"{"rules":{"visibility_required":{"elements":["const","method"]}}}"#),
            Some(json!({"elements": ["method", "property", "const"]}))
        );
        // laravel-framework: preset empty, visibility_required alone.
        assert_eq!(
            mk(r#"{"preset":"empty","rules":{"visibility_required":{"elements":["method"]}}}"#),
            Some(json!({"elements": ["method"]}))
        );
        assert_eq!(
            mk(
                r#"{"rules":{"modifier_keywords":true,"visibility_required":{"elements":["method"]}}}"#
            ),
            Some(Value::Null)
        );
    }

    #[test]
    fn lists_unsupported_rules() {
        let m = pint(
            r#"{"preset":"empty","rules":{"braces":true,"Custom/fixer":true,"header_comment":{"header":null},"@PER-CS":true}}"#,
        );
        assert_eq!(m.unsupported.len(), 4);
        assert!(
            m.unsupported[0].starts_with(
                "braces: deprecated upstream; successor: single_space_around_construct"
            )
        );
        assert_eq!(
            m.unsupported[1..],
            [
                "Custom/fixer: not a PHP-CS-Fixer or Pint rule Pun knows (a custom fixer or rule set)",
                "header_comment: an option is null, which TOML can't hold",
                "@PER-CS: not a PHP-CS-Fixer or Pint rule Pun knows (a custom fixer or rule set)",
            ]
        );
        // The per preset carries @PER-CS itself, so a per project migrates (and its runs stop on it).
        assert!(pint(r#"{"preset":"per"}"#).unsupported.is_empty());
    }

    #[test]
    fn php_cs_fixer_rules() {
        let rules =
            json!({"@PSR12": true, "strict_param": true, "concat_space": {"spacing": "one"}});
        let m = from_rules(rules.as_object().unwrap().clone()).unwrap();
        let c = from_toml(&dir(&[]), &m.toml).unwrap();
        assert_eq!(c.preset, "empty");
        assert!(
            names(&c.rules).contains(&"braces_position")
                && names(&c.rules).contains(&"concat_space")
        );
        assert_eq!(
            names(&c.lint.iter().map(|(r, _)| r.clone()).collect::<Vec<_>>()),
            ["strict_param"]
        );
    }

    /// Messages are Pint's (ConfigurationJsonRepository, RuleSet; real Pint 1.29.3 on the same pint.json).
    #[test]
    fn pint_errors() {
        let e = |files: &[(&str, &str)]| {
            from_pint(&dir(files).join("pint.json"), None)
                .err()
                .unwrap()
                .to_string()
        };
        assert_eq!(
            e(&[("pint.json", r#"{"preset":"nope"}"#)]),
            "Preset not found."
        );
        // A top-level JSON list is an empty config (laravel preset), not invalid JSON; no file is too.
        assert_eq!(pint("[1]").toml, pint("{}").toml);
        assert_eq!(
            from_pint(&dir(&[]).join("pint.json"), Some("psr12"))
                .unwrap()
                .toml,
            "[format]\npreset = \"psr12\"\n"
        );
        assert_eq!(
            e(&[
                ("a.json", r#"{"extend":"b.json"}"#),
                ("pint.json", r#"{"extend":"a.json"}"#)
            ]),
            "Pint configuration cannot extend from more than 1 file."
        );
        assert!(e(&[("pint.json", "1")]).ends_with("pint.json] is not valid JSON."));
        assert!(e(&[("pint.json", "{")]).ends_with("pint.json] is not valid JSON."));
        assert_eq!(
            e(&[("pint.json", r#"{"rules":{"final_class":null}}"#)]),
            "[final_class] Rule must be enabled (true), disabled (false) or configured (non-empty, assoc array). Other values are not allowed. To disable the rule, use \"FALSE\" instead of \"NULL\"."
        );
        assert_eq!(
            e(&[("pint.json", r#"{"rules":{"@PSR12":1}}"#)]),
            "[@PSR12] Set must be enabled (true) or disabled (false). Other values are not allowed."
        );
        for (base, given) in [
            ("{bad", "null"),
            ("\"x\"", "string"),
            ("1", "int"),
            ("1.5", "float"),
            ("true", "true"),
            ("false", "false"),
        ] {
            assert_eq!(
                e(&[
                    ("pint.json", r#"{"extend":"base.json"}"#),
                    ("base.json", base)
                ]),
                format!(
                    "array_replace_recursive(): Argument #1 ($array) must be of type array, {given} given"
                )
            );
        }
        assert_eq!(
            e(&[("pint.json", r#"{"extend":"base.json"}"#)]),
            "Path must not be empty"
        );
    }

    /// `extend` merges like array_replace_recursive: lists by index, so ["x"] over ["a","b"] is ["x","b"].
    #[test]
    fn extend() {
        let d = dir(&[
            (
                "base.json",
                r#"{"preset":"empty","exclude":["a","b"],"notName":["n"],"rules":{"single_quote":true}}"#,
            ),
            (
                "custom.json",
                r#"{"extend":"base.json","exclude":["x"],"notName":["n","m"],"in":"src","rules":{"concat_space":{"spacing":"one"}}}"#,
            ),
        ]);
        let m = from_pint(&d.join("custom.json"), None).unwrap();
        assert_eq!(
            m.toml,
            "[format]\npreset = \"empty\"\npaths = [\"src\"]\nexclude = [\"x\", \"b\"]\nnot-name = [\"n\", \"m\"]\n\n[format.rules]\nsingle-quote = true\nconcat-space = { spacing = \"one\" }\n"
        );
    }
}
