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
    // The decided text (PLAN.md §2), by hand.
    assert_eq!(
        err(&d, Options::default()),
        "No pun.toml found. Run `pun migrate`, or `pun init` for a new project."
    );
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

/// A lint rule as `[lint.rules]` resolves it, for expectations.
fn lr(id: &str, level: Level, fix: bool, options: Value, fixer: Option<&'static str>) -> LintRule {
    LintRule {
        id: id.into(),
        level,
        fix,
        options,
        fixer,
        named: true,
    }
}

fn ids(rules: &[LintRule]) -> Vec<&str> {
    rules.iter().map(|r| r.id.as_str()).collect()
}

/// A Laravel 11 project: composer.lock locks laravel/framework v11.9.2.
const LARAVEL_LOCK: &str = r#"{"packages":[{"name":"laravel/framework","version":"v11.9.2"}]}"#;

#[test]
fn pun_toml() {
    let d = dir(&[
        ("composer.json", r#"{"require":{"php":"^8.4"}}"#),
        (
            "pun.toml",
            r#"[php]
version = "8.5"
[files]
paths = ["src", "tests"]
exclude = ["build"]
[format]
preset = "laravel"
exclude = ["a"]
not-name = ["n.php"]
not-path = ["p"]
[format.rules]
"operators/concat-spacing" = { spacing = "one" }
"format/arrays/syntax" = false
"laravel/blade" = true
[lint.rules]
"style/no-alias-function" = "error"
"lint/style/require-final-class" = "warn"
"statements/require-strict-types" = "off"
"functions/no-unreachable-default-argument-value" = { level = "warn", fix = "off" }
"types/no-loose-comparison" = "error"
"style/require-ordered-traits" = { fix = "off" }
"style/prefer-cast-over-intval" = { level = "error" }
"imports/no-unused" = "error"
"phpdoc/no-superfluous-tags" = { allow-mixed = true }
"pest/no-only" = { level = "off", fix = "off" }
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
    // The laravel preset's 4 risky fixers and 11 code-deleting ones are lint rules: off unless [lint] turns them on
    // (context/plan/commands.md §4).
    for risky in [
        "no_alias_functions",
        "no_unreachable_default_argument_value",
        "ordered_traits",
        "php_unit_set_up_tear_down_visibility",
        "no_unused_imports",
        "no_useless_return",
        "no_empty_statement",
        "lambda_not_used_import",
        "no_superfluous_phpdoc_tags",
        "general_phpdoc_tag_rename",
        "phpdoc_no_access",
        "phpdoc_no_package",
        "phpdoc_no_useless_inheritdoc",
        "self_static_accessor",
        "php_unit_method_casing",
    ] {
        assert!(!n.contains(&risky), "{risky}");
    }
    assert!(c.blade);
    // By ID; fixers hand-read from [upstream.php-cs-fixer] of rules/mapping.toml.
    use Level::{Error, Warn};
    assert_eq!(
        c.lint.rules,
        [
            lr(
                "lint/functions/no-unreachable-default-argument-value",
                Warn,
                false,
                Value::Null,
                Some("no_unreachable_default_argument_value")
            ),
            lr(
                "lint/imports/no-unused",
                Error,
                true,
                Value::Null,
                Some("no_unused_imports")
            ),
            lr(
                "lint/phpdoc/no-superfluous-tags",
                Error,
                true,
                json!({"allow_mixed": true}),
                Some("no_superfluous_phpdoc_tags")
            ),
            lr(
                "lint/style/no-alias-function",
                Error,
                true,
                Value::Null,
                Some("no_alias_functions")
            ),
            lr(
                "lint/style/prefer-cast-over-intval",
                Error,
                true,
                Value::Null,
                Some("modernize_types_casting")
            ),
            lr(
                "lint/style/require-final-class",
                Warn,
                true,
                Value::Null,
                Some("final_class")
            ),
            lr(
                "lint/style/require-ordered-traits",
                Error,
                false,
                Value::Null,
                Some("ordered_traits")
            ),
            lr(
                "lint/types/no-loose-comparison",
                Error,
                true,
                Value::Null,
                Some("strict_comparison")
            ),
        ]
    );
    // [files] is every command's; each command's exclude adds to it.
    assert_eq!(
        c.files,
        Files {
            paths: vec!["src".into(), "tests".into()],
            exclude: vec!["build".into()]
        }
    );
    assert_eq!(
        (
            c.finder.in_dirs,
            c.finder.exclude,
            c.finder.not_name,
            c.finder.not_path
        ),
        (
            vec!["src".into(), "tests".into()],
            vec!["build".into(), "a".into()],
            vec!["n.php".into()],
            vec!["p".into()]
        )
    );
    assert_eq!(c.lint.exclude, ["build"]);
    assert_eq!(c.php, "8.5");
    assert_eq!(c.sources, [d.join("composer.json"), d.join("pun.toml")]);
    // Options take their Pun names and values (rules/mapping.toml [options]) and reach the fixer under
    // PHP-CS-Fixer's.
    let d = dir(&[(
        "pun.toml",
        "[format]\npreset = \"empty\"\n[format.rules]\n\"imports/ordered\" = { sort = \"alphabetical\", order = [\"class\", \"const\"] }\n",
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
    // Nothing named, no composer.json: PHP 8.4, no lint rules, no files.
    assert_eq!(c.php, "8.4");
    assert!(c.lint.rules.is_empty() && c.files == Files::default());
}

/// `[php] version` overrides composer.json's `require.php`, whose lowest version is the default.
#[test]
fn php_version() {
    let c = |files: &[(&str, &str)]| load(&dir(files), &Options::default()).unwrap().php;
    let toml = ("pun.toml", "");
    assert_eq!(
        c(&[toml, ("composer.json", r#"{"require":{"php":"^8.5"}}"#)]),
        "8.5"
    );
    assert_eq!(
        c(&[toml, ("composer.json", r#"{"require":{"php":">=8.4.3"}}"#)]),
        "8.4"
    );
    assert_eq!(c(&[toml, ("composer.json", "{}")]), "8.4");
    assert_eq!(
        c(&[
            ("pun.toml", "[php]\nversion = \"8.6\"\n"),
            ("composer.json", r#"{"require":{"php":"^8.4"}}"#)
        ]),
        "8.6"
    );
}

/// `[lint] sets` turn on every rule tagged with them, at `error`. Rule lists hand-read from [rules.lint] of
/// rules/mapping.toml.
#[test]
fn lint_sets() {
    let lint = |files: &[(&str, &str)]| load(&dir(files), &Options::default()).unwrap().lint;
    let l = lint(&[("pun.toml", "[lint]\nsets = [\"early-return\"]\n")]);
    assert_eq!(l.sets, ["early-return"]);
    let early = [
        ("lint/control-flow/no-else-after-return", None),
        ("lint/control-flow/no-useless-else", Some("no_useless_else")),
        ("lint/style/prefer-early-continue", None),
        ("lint/style/prefer-early-return", None),
        ("lint/style/prefer-early-return-of-variable", None),
        ("lint/style/prefer-early-return-over-prepared-value", None),
    ];
    assert_eq!(
        l.rules,
        early.map(|(id, fixer)| LintRule {
            named: false,
            ..lr(id, Level::Error, true, Value::Null, fixer)
        })
    );
    // `types` has 79 rules, four of them Laravel's: those need laravel/framework in composer.lock.
    let types = "[lint]\nsets = [\"types\"]\n";
    let l = lint(&[("pun.toml", types)]);
    assert_eq!((l.rules.len(), l.ecosystems.len()), (75, 0));
    assert!(!ids(&l.rules).contains(&"lint/laravel/add-relation-generics"));
    let l = lint(&[("pun.toml", types), ("composer.lock", LARAVEL_LOCK)]);
    assert_eq!(
        (l.rules.len(), l.ecosystems),
        (79, vec![crate::Ecosystem::Laravel])
    );
    assert!(ids(&l.rules).contains(&"lint/laravel/add-relation-generics"));
    // The laravel group switched off.
    let off = format!("{types}[lint.groups]\nlaravel = \"off\"\n");
    let l = lint(&[("pun.toml", &off), ("composer.lock", LARAVEL_LOCK)]);
    assert_eq!((l.rules.len(), l.ecosystems.len()), (75, 0));
    // `modernize` has 37 rules; an upgrade group's run from its PHP version on: on 8.4, the 6 php-8.5 rules and the
    // 1 php-8.6 rule are off.
    let modernize = "[lint]\nsets = [\"modernize\"]\n";
    let l = lint(&[("pun.toml", modernize)]);
    assert_eq!(l.rules.len(), 30);
    assert!(ids(&l.rules).contains(&"lint/php-8.4/prefer-array-find"));
    assert!(!ids(&l.rules).contains(&"lint/php-8.5/no-null-array-key"));
    let l = lint(&[(
        "pun.toml",
        &format!("[php]\nversion = \"8.5\"\n{modernize}"),
    )]);
    assert_eq!(l.rules.len(), 36);
    assert!(ids(&l.rules).contains(&"lint/php-8.5/no-null-array-key"));
    // Two sets: the union, 6 + 75 (no rule is in both).
    let l = lint(&[("pun.toml", "[lint]\nsets = [\"early-return\", \"types\"]\n")]);
    assert_eq!(l.rules.len(), 81);
}

/// A group's level covers every rule of the group, whether a set has it or not; a rule's own line wins.
#[test]
fn lint_groups() {
    let lint = |toml: &str| {
        load(&dir(&[("pun.toml", toml)]), &Options::default())
            .unwrap()
            .lint
    };
    let level = |l: &Lint, id: &str| l.rules.iter().find(|r| r.id == id).map(|r| r.level);
    // 92 style rules at warn, the set's two control-flow rules at error.
    let l = lint("[lint]\nsets = [\"early-return\"]\n[lint.groups]\nstyle = \"warn\"\n");
    assert_eq!(l.rules.len(), 94);
    assert!(
        l.rules
            .iter()
            .all(|r| r.level == Level::Warn || r.id.starts_with("lint/control-flow/no-"))
    );
    assert_eq!(
        level(&l, "lint/control-flow/no-useless-else"),
        Some(Level::Error)
    );
    let l = lint(
        "[lint]\nsets = [\"early-return\"]\n[lint.groups]\nstyle = \"off\"\n[lint.rules]\n\"style/prefer-early-return\" = \"warn\"\n",
    );
    assert_eq!(
        ids(&l.rules),
        [
            "lint/control-flow/no-else-after-return",
            "lint/control-flow/no-useless-else",
            "lint/style/prefer-early-return"
        ]
    );
    assert_eq!(
        level(&l, "lint/style/prefer-early-return"),
        Some(Level::Warn)
    );
    let l = lint(
        "[lint]\nsets = [\"early-return\"]\n[lint.rules]\n\"control-flow/no-useless-else\" = \"off\"\n",
    );
    assert_eq!(l.rules.len(), 5);
    // A rule no fixer runs keeps its options as written (they are Pun's own names, not PHP-CS-Fixer's).
    let l =
        lint("[lint.rules]\n\"style/prefer-early-return\" = { level = \"warn\", max-depth = 2 }\n");
    assert_eq!(
        l.rules,
        [LintRule {
            id: "lint/style/prefer-early-return".into(),
            level: Level::Warn,
            fix: true,
            options: json!({"max-depth": 2}),
            fixer: None,
            named: true,
        }]
    );
    // A group turns an upgrade on whatever the Laravel version (no composer.lock here).
    let l = lint("[lint.groups]\nlaravel-12 = \"error\"\n");
    assert_eq!(ids(&l.rules), ["lint/laravel-12/prefer-scope-attribute"]);
    assert!(l.rules.iter().all(|r| !r.named && r.level == Level::Error));
}

/// What a set alone may turn on in an upgrade group: the locked Laravel version gates `laravel-<n>` (no set has such
/// a rule yet, so this is checked here and not through `load`).
#[test]
fn upgrade_gates() {
    let gates = |lock: Option<&str>| {
        let d = dir(&lock
            .map(|l| ("composer.lock", l))
            .into_iter()
            .collect::<Vec<_>>());
        composer::detect(&d).unwrap()
    };
    let check = |composer: &composer::Composer, group: &str| {
        let g = schema::Gates {
            php: (8, 4),
            composer,
        };
        g.allows(group, &g.ecosystems(|_| false))
    };
    let l11 = gates(Some(LARAVEL_LOCK));
    assert!(check(&l11, "laravel-11") && check(&l11, "laravel-10") && check(&l11, "laravel-5.7"));
    assert!(!check(&l11, "laravel-12"));
    assert!(check(&l11, "laravel") && !check(&l11, "pest") && !check(&l11, "doctrine"));
    assert!(check(&l11, "php-8.4") && !check(&l11, "php-8.5") && check(&l11, "style"));
    // A branch has no version, so no upgrade group runs from a set; no lock, no Laravel at all.
    let branch = gates(Some(
        r#"{"packages":[{"name":"laravel/framework","version":"dev-main"}]}"#,
    ));
    assert!(check(&branch, "laravel") && !check(&branch, "laravel-5.7"));
    assert!(!check(&gates(None), "laravel-5.7"));
}

#[test]
fn lint_overrides() {
    let c = load(
        &dir(&[(
            "pun.toml",
            "[[lint.overrides]]\nfiles = [\"tests/**\"]\nrules = { \"pest/no-only\" = \"off\", \"style/require-final-class\" = { level = \"warn\", fix = \"off\" } }\n\n[[lint.overrides]]\nfiles = [\"app/Legacy.php\", \"database/**\"]\n",
        )]),
        &Options::default(),
    )
    .unwrap();
    assert_eq!(
        c.lint.overrides,
        [
            Override {
                files: vec!["tests/**".into()],
                rules: vec![
                    LintSetting {
                        id: "lint/pest/no-only".into(),
                        level: None,
                        fix: true,
                        options: Value::Null
                    },
                    LintSetting {
                        id: "lint/style/require-final-class".into(),
                        level: Some(Level::Warn),
                        fix: false,
                        options: Value::Null
                    },
                ]
            },
            Override {
                files: vec!["app/Legacy.php".into(), "database/**".into()],
                rules: vec![]
            },
        ]
    );
    // Overrides change levels per path at run time; they turn nothing on for the whole project.
    assert!(c.lint.rules.is_empty());
}

/// Rule counts counted in [rules.analyse] of rules/mapping.toml: basic 155, essential 104, recommended 80, strict
/// 35, max 4; 18 of them Laravel's (0, 4, 9, 4, 1).
#[test]
fn analyse() {
    let an = |files: &[(&str, &str)]| load(&dir(files), &Options::default()).unwrap().analyse;
    let has = |a: &Analyse, id: &str| a.rules.iter().any(|r| r.id == id);
    // No [analyse]: recommended, [files] paths and exclude.
    let a = an(&[(
        "pun.toml",
        "[files]\npaths = [\"app\"]\nexclude = [\"storage\"]\n",
    )]);
    assert_eq!(
        a,
        Analyse {
            preset: crate::AnalysePreset::Recommended,
            paths: vec!["app".into()],
            exclude: vec!["storage".into()],
            baseline: None,
            switches: vec![],
            ecosystems: vec![],
            rules: a.rules.clone(),
        }
    );
    assert_eq!(a.rules.len(), 155 + 104 + 80 - 13);
    assert!(has(&a, "analyse/undefined/method") && !has(&a, "analyse/types/missing-return-type"));
    let preset = |p: &str, lock: bool| {
        let toml = format!("[analyse]\npreset = \"{p}\"\n");
        let mut files = vec![("pun.toml", toml.as_str())];
        if lock {
            files.push(("composer.lock", LARAVEL_LOCK));
        }
        let a = an(&files);
        (a.rules.len(), a.switches)
    };
    assert_eq!(preset("basic", false), (155, vec![]));
    assert_eq!(preset("essential", true), (155 + 104, vec![]));
    assert_eq!(preset("recommended", true), (155 + 104 + 80, vec![]));
    assert_eq!(
        preset("strict", true),
        (155 + 104 + 80 + 35, vec!["union-types", "nullables"])
    );
    assert_eq!(
        preset("max", false),
        (
            378 - 18,
            vec![
                "union-types",
                "nullables",
                "explicit-mixed",
                "implicit-mixed"
            ]
        )
    );
    let a = an(&[(
        "pun.toml",
        r#"[files]
paths = ["app"]
exclude = ["storage"]
[analyse]
preset = "basic"
paths = ["app", "routes"]
exclude = ["tests/**"]
baseline = "pun-baseline.toml"
[analyse.rules]
"types/missing-return-type" = "on"
"analyse/undefined/method" = { magic = true }
"undefined/function" = "off"
"types/argument-not-numeric" = "on"
laravel = "on"
"laravel/useless-with-call" = "off"
"#,
    )]);
    assert_eq!(
        (a.paths, a.exclude, a.baseline),
        (
            vec!["app".into(), "routes".into()],
            vec!["storage".into(), "tests/**".into()],
            Some("pun-baseline.toml".into())
        )
    );
    // basic, plus 2 named, plus the 18 Laravel rules but one, minus undefined/function.
    assert_eq!(a.rules.len(), 155 + 2 + 17 - 1);
    let rule = |id: &str| {
        a.rules
            .iter()
            .find(|r| r.id == id)
            .map(|r| r.options.clone())
    };
    assert_eq!(
        rule("analyse/undefined/method"),
        Some(json!({"magic": true}))
    );
    assert_eq!(rule("analyse/types/missing-return-type"), Some(Value::Null));
    assert_eq!(rule("analyse/undefined/function"), None);
    assert_eq!(rule("analyse/laravel/useless-with-call"), None);
    // laravel/framework locked, the group switched off.
    let a = an(&[
        ("pun.toml", "[analyse.rules]\nlaravel = \"off\"\n"),
        ("composer.lock", LARAVEL_LOCK),
    ]);
    assert_eq!(
        (a.rules.len(), a.ecosystems.len()),
        (155 + 104 + 80 - 13, 0)
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
    // Hand-read from rules/mapping.toml.
    assert_eq!(
        rule_of("no_alias_functions"),
        Some("lint/style/no-alias-function")
    );
    assert_eq!(rule_of("no_unused_imports"), Some("lint/imports/no-unused"));
    assert_eq!(rule_of("Pint/laravel_blade"), Some("format/laravel/blade"));
    assert_eq!(rule_of("Pun/no_only"), Some("lint/pest/no-only"));
    assert_eq!(rule_of("braces"), None);
    assert_eq!(
        fixer_of("format", "format/imports/ordered"),
        Some("ordered_imports")
    );
    assert_eq!(
        fixer_of("format", "imports/ordered"),
        Some("ordered_imports")
    );
    assert_eq!(fixer_of("lint", "imports/ordered"), None);
    assert_eq!(fixer_of("lint", "format/imports/ordered"), None);
    // The old names are gone.
    assert_eq!(fixer_of("format", "ordered-imports"), None);
    assert_eq!(fixer_of("format", "format.ordered-imports"), None);
}

#[test]
fn rule_metadata() {
    // Hand-read from rules/mapping.toml.
    let m = rule("lint/dead-code/no-self-assignment").unwrap();
    assert_eq!((m.fix, m.summary), ("safe", None));
    assert_eq!(
        rule("lint/types/require-strict-param").unwrap().fix,
        "mixed"
    );
    let m = rule("format/arrays/syntax").unwrap();
    assert_eq!(
        (m.fix, m.summary),
        (
            "safe",
            Some("PHP arrays should be declared using the configured syntax.")
        )
    );
    // A moved format rule keeps its summary under its lint ID.
    assert_eq!(
        rule("lint/imports/no-unused").unwrap().summary,
        Some("Unused `use` statements must be removed.")
    );
    // Analyse rules have no fix.
    assert_eq!(rule("analyse/types/argument").unwrap().fix, "none");
    assert!(rule("dead-code/no-self-assignment").is_none());
    assert!(rule("lint.no-self-assignment").is_none());
    assert!(rule("format/imports/no-unused").is_none());
}

#[test]
fn show() {
    let d = dir(&[(
        "pun.toml",
        "[files]\npaths = [\"app\"]\nexclude = [\"build\"]\n[format]\npreset = \"empty\"\nexclude = [\"gen\"]\n[format.rules]\n\"operators/concat-spacing\" = { spacing = \"one\" }\n\"strings/single-quote\" = true\n[lint.rules]\n\"style/require-final-class\" = \"error\"\n\"style/no-alias-function\" = { level = \"warn\", fix = \"off\", sets = [\"@internal\"] }\n\"types/no-loose-comparison\" = { fix = \"off\" }\n[[lint.overrides]]\nfiles = [\"tests/**\"]\nrules = { \"style/require-final-class\" = \"off\", \"types/no-loose-comparison\" = \"warn\" }\n[analyse]\npreset = \"basic\"\n[analyse.rules]\n\"undefined/method\" = { magic = true }\n",
    )]);
    let v = load(&d, &Options::default()).unwrap().show();
    // Rules as pun.toml writes them: `<group>/<name>` under their command's table.
    assert_eq!(
        v["lint"],
        json!({
            "sets": [],
            "exclude": ["build"],
            "ecosystems": [],
            "rules": {
                "style/no-alias-function": {"level": "warn", "fix": "off", "sets": ["@internal"]},
                "style/require-final-class": "error",
                "types/no-loose-comparison": {"fix": "off"},
            },
            "overrides": [{
                "files": ["tests/**"],
                "rules": {"style/require-final-class": "off", "types/no-loose-comparison": "warn"},
            }],
        })
    );
    assert_eq!(
        v["format"],
        json!({
            "preset": "empty", "exclude": ["build", "gen"], "not-name": [], "not-path": [],
            "rules": {"operators/concat-spacing": "one", "strings/single-quote": true},
        })
    );
    assert_eq!(
        (&v["files"], &v["php"]),
        (
            &json!({"paths": ["app"], "exclude": ["build"]}),
            &json!({"version": "8.4"}),
        )
    );
    assert_eq!(
        v["analyse"]["rules"]["undefined/method"],
        json!({"magic": true})
    );
    assert_eq!(v["analyse"]["rules"]["undefined/function"], "on");
    assert_eq!(v["analyse"]["preset"], "basic");
    // No baseline, no key (TOML has no null); a baseline is a path.
    assert!(v["analyse"].get("baseline").is_none());
    let d = dir(&[("pun.toml", "[analyse]\nbaseline = \"b.toml\"\n")]);
    assert_eq!(
        load(&d, &Options::default()).unwrap().show()["analyse"]["baseline"],
        "b.toml"
    );
}

/// `--preset per|symfony` is an error naming the replacement (format 1), with or without a pun.toml, and a typed one
/// so the CLI exits 2; pun.toml's `[format] preset = "per"` is the same type. An unknown preset stays "Preset not
/// found." (a plain error).
#[test]
fn removed_presets() {
    let removed = |d: &Path, o: Options| load(d, &o).unwrap_err().is::<RemovedPreset>();
    for p in ["per", "symfony"] {
        let want = format!(
            "--preset {p} was removed: use --preset empty with its rules under [format.rules] in pun.toml (`pun migrate --preset {p}` expands it)"
        );
        let none = dir(&[]);
        assert_eq!(err(&none, preset(p)), want);
        assert!(removed(&none, preset(p)));
        let d = dir(&[("pun.toml", "[format]\npreset = \"psr12\"\n")]);
        assert_eq!(err(&d, preset(p)), want);
        let d = dir(&[("pun.toml", &format!("[format]\npreset = \"{p}\"\n"))]);
        assert!(removed(&d, Options::default()));
    }
    assert!(!removed(&dir(&[]), preset("nope")));
    // The kept presets still load through --preset.
    for p in FORMAT_PRESETS {
        assert_eq!(load(&dir(&[]), &preset(p)).unwrap().preset, p);
    }
}

/// The text after `docs      ` is mapping.toml's URL with the line of the rule's own row.
fn explained(name: &str, id_key: &str) -> String {
    let out = explain(name).unwrap();
    let (head, docs) = out.rsplit_once("docs      ").unwrap();
    let line: usize = docs
        .strip_prefix(&format!("{MAPPING_URL}#L"))
        .unwrap()
        .trim_end()
        .parse()
        .unwrap();
    let row = MAPPING.lines().nth(line - 1).unwrap();
    assert!(row.starts_with(&format!("\"{id_key}\" = ")), "{row}");
    head.to_string()
}

/// Expected text by hand from each rule's row in mapping.toml, the rows of `[upstream.*]` that map to it and its
/// group's line in `[meta.groups]`.
#[test]
fn explain_rules() {
    assert_eq!(
        explained("format/arrays/syntax", "arrays/syntax"),
        "format/arrays/syntax
PHP arrays should be declared using the configured syntax.

group     arrays: Array literals, list() destructuring and array access
presets   laravel
fix       safe
upstream  array_syntax (php-cs-fixer)
          array-style (mago-linter)
"
    );
    // Without the command part, the same rule.
    assert_eq!(
        explained("arrays/syntax", "arrays/syntax"),
        explained("format/arrays/syntax", "arrays/syntax")
    );
    assert_eq!(
        explained("lint/style/no-alias-function", "style/no-alias-function"),
        "lint/style/no-alias-function
Master functions shall be used instead of aliases.

group     style: One consistent way to write the same thing
sets      coding-style
fix       unsafe
upstream  no_alias_functions (php-cs-fixer)
          no-alias-function (mago-linter)
"
    );
    // No summary; the analyse preset; PHPStan's identifier and parameters, Mago's codes.
    assert_eq!(
        explained("analyse/undefined/method", "undefined/method"),
        "analyse/undefined/method

group     undefined: References to something declared nowhere
preset    basic
fix       none
upstream  CallMethodsRule (phpstan-classes)
          method.notFound (phpstan)
          checkThisOnly (phpstan-parameters)
          reportMagicMethods (phpstan-parameters)
          NonExistentMethod (mago-analyzer)
          PossiblyNonExistentMethod (mago-analyzer)
"
    );
    // Intent, and a Pun-only rule.
    assert_eq!(
        explained(
            "lint/comments/no-unused-ignore",
            "comments/no-unused-ignore"
        ),
        "lint/comments/no-unused-ignore
A @pun-ignore comment that suppressed nothing. Fix removes it.

group     comments: Comments other than PHPDoc, including ignore comments
sets      code-quality
intent    suspicious
fix       safe
upstream  none (Pun's own rule)
"
    );
    // A lint rule's fix, and a rule in no preset.
    let out = explained("analyse/casts/useless", "casts/useless");
    assert!(
        out.contains(
            "preset    recommended\nfix       none\nfixed by  lint/dead-code/no-useless-cast\n"
        ),
        "{out}"
    );
    let out = explained(
        "format/arrays/no-space-in-empty",
        "arrays/no-space-in-empty",
    );
    assert!(
        out.contains("\npresets   none (opt-in by name or group)\n"),
        "{out}"
    );
}

/// An upstream or old name is answered with the Pun ID; names without a rule are errors that say why.
#[test]
fn explain_names() {
    let ok = |n: &str| explain(n).unwrap();
    assert_eq!(
        ok("method.notFound"),
        format!(
            "`method.notFound` is `analyse/undefined/method` in Pun.\n\n{}",
            ok("analyse/undefined/method")
        )
    );
    // The old `format.` / `lint.` names, through pun.toml's loose match.
    assert!(ok("lint.no-alias-functions").starts_with(
        "`lint.no-alias-functions` is `lint/style/no-alias-function` in Pun.\n\nlint/style/no-alias-function\n"
    ));
    let err = |n: &str| format!("{:#}", explain(n).unwrap_err());
    assert_eq!(
        err("checkUnionTypes"),
        "`checkUnionTypes` (phpstan-parameters) is `[analyse] union-types = true` in pun.toml, not a rule"
    );
    assert_eq!(
        err("curly_braces_position"),
        "`curly_braces_position` (php-cs-fixer) has no Pun rule: deprecated upstream; successor: braces_position"
    );
    assert_eq!(
        err("OverrideFinalPropertyHook"),
        "`OverrideFinalPropertyHook` (mago-analyzer) has no Pun rule: no PHPStan-parity equivalent; Mago analyzer is not a migrate input"
    );
    assert_eq!(
        err("nope"),
        "`nope` is not a Pun rule or an upstream rule name (see crates/pun-config/rules/mapping.toml)"
    );
}

/// Every error names what to write instead.
#[test]
fn pun_toml_errors() {
    let d = dir(&[]);
    let e = |toml: &str| {
        std::fs::write(d.join("pun.toml"), toml).unwrap();
        let p = d.join("pun.toml").display().to_string();
        err(&d, Options::default()).replace(&p, "pun.toml")
    };
    // Removed keys and values.
    for (toml, msg) in [
        (
            "preset = \"recommended\"\n",
            "the top-level `preset = \"recommended\"` was removed: use `[lint] sets` for lint rules, `[analyse] preset` for analyse and `[format] preset` for format",
        ),
        (
            "[format]\npaths = [\"app\"]\n",
            "[format] paths moved to [files] paths, which every command shares",
        ),
        (
            "[format]\npreset = \"per\"\n",
            "[format] preset = \"per\" was removed: use preset = \"empty\" with its rules under [format.rules] (`pun migrate` expands a pint.json that used it)",
        ),
        (
            "[format]\npreset = \"symfony\"\n",
            "[format] preset = \"symfony\" was removed: use preset = \"empty\" with its rules under [format.rules] (`pun migrate` expands a pint.json that used it)",
        ),
        (
            "[lint.rules]\n\"style/require-final-class\" = { fix = \"apply\" }\n",
            "`style/require-final-class`: fix = \"apply\" was removed; `--fix` applies the fix of every enabled rule, so leave `fix` out",
        ),
        (
            "[lint.rules]\n\"style/require-final-class\" = { fix = \"safe-only\" }\n",
            "`style/require-final-class`: fix = \"safe-only\" was removed; `--fix` applies every fix, and fix = \"off\" keeps a rule report-only",
        ),
        (
            "[lint.rules]\n\"style/require-final-class\" = \"info\"\n",
            "`style/require-final-class` = \"info\": the `info` level was removed; use \"warn\"",
        ),
        (
            "[lint.groups]\nstyle = \"info\"\n",
            "`style` = \"info\": the `info` level was removed; use \"warn\"",
        ),
        (
            "[lint.rules]\n\"style/require-final-class\" = { severity = \"warn\" }\n",
            "`style/require-final-class`: `severity` was removed; use `level` (\"error\", \"warn\" or \"off\")",
        ),
        (
            "[lint.rules]\n\"style/require-final-class\" = true\n",
            "`style/require-final-class` = true: lint rules take a level; use \"error\"",
        ),
        (
            "[lint.rules]\n\"style/require-final-class\" = false\n",
            "`style/require-final-class` = false: lint rules take a level; use \"off\"",
        ),
        (
            "[analyse]\nlevel = 5\n",
            "[analyse] level: PHPStan's levels are presets in Pun; use preset = \"basic\" | \"essential\" | \"recommended\" | \"strict\" | \"max\": level 5 is preset = \"recommended\"",
        ),
        (
            "[analyse]\nlevel = \"max\"\n",
            "[analyse] level: PHPStan's levels are presets in Pun; use preset = \"basic\" | \"essential\" | \"recommended\" | \"strict\" | \"max\"",
        ),
        (
            "[analyse]\npreset = 8\n",
            "[analyse] preset = 8: PHPStan level 8 is preset = \"strict\"",
        ),
        (
            "[analyse.rules]\n\"undefined/method\" = \"warn\"\n",
            "[analyse.rules] `undefined/method` = \"warn\": analyse rules are \"on\" or \"off\" (or a table of options); there are no levels",
        ),
        // Bad values.
        (
            "[format]\npreset = \"pint\"\n",
            "[format] preset = \"pint\": expected `laravel`, `psr12` or `empty`",
        ),
        (
            "[php]\nversion = \"8\"\n",
            "[php] version = \"8\": expected a PHP version as major.minor, such as \"8.4\"",
        ),
        (
            "[php]\nversion = \"eight.four\"\n",
            "[php] version = \"eight.four\": expected a PHP version as major.minor, such as \"8.4\"",
        ),
        (
            "[lint]\nsets = [\"recommended\"]\n",
            "[lint] sets: `recommended` is not a lint set; the sets are `dead-code`, `modernize`, `code-quality`, `types`, `early-return`, `privatization`, `naming`, `coding-style`",
        ),
        (
            "[lint.groups]\nnope = \"error\"\n",
            "[lint.groups]: `nope` is not a group (see [meta.groups] in crates/pun-config/rules/mapping.toml)",
        ),
        (
            "[lint.groups]\nstyle = true\n",
            "[lint.groups] `style` = true: expected \"error\", \"warn\" or \"off\"",
        ),
        (
            "[lint.rules]\n\"style/require-final-class\" = \"loud\"\n",
            "`style/require-final-class` = \"loud\": expected \"error\", \"warn\" or \"off\"",
        ),
        (
            "[lint.rules]\n\"style/require-final-class\" = { level = 2 }\n",
            "`style/require-final-class`: level = 2: expected \"error\", \"warn\" or \"off\"",
        ),
        (
            "[lint.rules]\n\"style/require-final-class\" = { fix = \"yes\" }\n",
            "`style/require-final-class`: fix = \"yes\": the only value is \"off\"",
        ),
        (
            "[lint.rules]\n\"style/require-final-class\" = 1\n",
            "`style/require-final-class` = 1: expected \"error\", \"warn\", \"off\" or a table",
        ),
        (
            "[analyse]\npreset = \"level-5\"\n",
            "[analyse] preset = \"level-5\": expected `basic`, `essential`, `recommended`, `strict`, `max`",
        ),
        (
            "[analyse.rules]\nlaravel = { magic = true }\n",
            "[analyse.rules] `laravel` is a group: it takes \"on\" or \"off\"",
        ),
        (
            "[format.rules]\n\"strings/single-quote\" = \"yes\"\n",
            "`strings/single-quote` must be true, false or a table of options",
        ),
        (
            "[format.rules]\nnope = true\n",
            "`nope` is not a format rule (see crates/pun-config/rules/mapping.toml)",
        ),
        (
            "[lint.rules]\nnope = \"error\"\n",
            "`nope` is not a lint rule (see crates/pun-config/rules/mapping.toml)",
        ),
        (
            "[analyse.rules]\nnope = \"on\"\n",
            "`nope` is not a analyse rule (see crates/pun-config/rules/mapping.toml)",
        ),
        // `ordered` is imports/ordered, attributes/ordered and types/ordered: not guessed.
        (
            "[format.rules]\nordered = true\n",
            "`ordered` is not a format rule (see crates/pun-config/rules/mapping.toml)",
        ),
    ] {
        let got = e(toml);
        assert_eq!(got, format!("pun.toml: {msg}"), "{toml}");
    }
    // serde's own errors for unknown keys.
    assert!(e("[format]\nexlude = [\"typo\"]\n").contains("unknown field `exlude`"));
    assert!(e("[lint]\nset = []\n").contains("unknown field `set`"));
    assert!(e("[[lint.overrides]]\nrules = {}\n").contains("missing field `files`"));
    // No aliases: an old Pun name, an upstream name or a rule of the other command names the ID to use
    // (IDs hand-read from rules/mapping.toml).
    for (table, name, id) in [
        // Old format names and the kebab-cased fixer, under either table.
        ("format", "format.array-syntax", "format/arrays/syntax"),
        ("format", "array-syntax", "format/arrays/syntax"),
        ("lint", "laravel-blade", "format/laravel/blade"),
        // The 40 code-deleting fixers are lint rules now (context/plan/commands.md §4).
        ("format", "no-unused-imports", "lint/imports/no-unused"),
        ("format", "imports/no-unused", "lint/imports/no-unused"),
        (
            "format",
            "self-static-accessor",
            "lint/classes/prefer-self-in-final",
        ),
        // Old lint and analyse names, and the auto-derived ones before the naming pass.
        (
            "lint",
            "lint.no-alias-function",
            "lint/style/no-alias-function",
        ),
        ("lint", "final-class", "lint/style/require-final-class"),
        ("format", "final-class", "lint/style/require-final-class"),
        (
            "lint",
            "simplify-mirror-assign",
            "lint/dead-code/no-mirror-assignment",
        ),
        (
            "lint",
            "no-redundant-pest-uses",
            "lint/pest/no-redundant-uses",
        ),
        (
            "lint",
            "analyse.undefined-method",
            "analyse/undefined/method",
        ),
        ("analyse", "undefined-method", "analyse/undefined/method"),
        // Upstream names: PHP-CS-Fixer, Pint, Rector, PHPStan.
        ("format", "array_syntax", "format/arrays/syntax"),
        ("format", "Pint/laravel_blade", "format/laravel/blade"),
        (
            "lint",
            "StrictInArray",
            "lint/functions/require-strict-in-array",
        ),
        ("lint", "method.notFound", "analyse/undefined/method"),
        ("analyse", "method.notFound", "analyse/undefined/method"),
        // A rule of the other command, full or short.
        ("lint", "imports/ordered", "format/imports/ordered"),
        (
            "format",
            "style/require-final-class",
            "lint/style/require-final-class",
        ),
        ("lint", "format/imports/ordered", "format/imports/ordered"),
    ] {
        let value = if table == "format" { "true" } else { "\"off\"" };
        let got = e(&format!("[{table}.rules]\n\"{name}\" = {value}\n"));
        assert_eq!(got, format!("pun.toml: `{name}` is `{id}` in pun.toml"));
    }
    // The same names inside an override.
    assert_eq!(
        e("[[lint.overrides]]\nfiles = [\"tests/**\"]\nrules = { final_class = \"off\" }\n"),
        "pun.toml: `final_class` is `lint/style/require-final-class` in pun.toml"
    );
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
    // `pun format` runs psr12 without its 2 code-deleting rules, which are lint rules (context/plan/commands.md §4).
    let psr12 = load(&dir(&[]), &preset("psr12")).unwrap();
    let mut want = expected(&cases[0]);
    want.retain(|(r, _)| r != "ordered_class_elements" && r != "no_unused_imports");
    assert_eq!(resolved(psr12.rules), want);
    for case in &cases {
        let rules = case["rules"].as_object().unwrap().clone();
        assert_eq!(resolved(rule_set(rules).unwrap()), expected(case));
    }
    // The @auto sets read the project's composer.json, so Pun has no definition: they stay as one rule, reported as
    // not ported.
    let rules = json!({"@auto": true}).as_object().unwrap().clone();
    assert_eq!(
        resolved(rule_set(rules).unwrap()),
        [("@auto".to_string(), Value::Null)]
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
        from_pint(
            &dir(&[("pint.json", json)]).join("pint.json"),
            None,
            &|_| true,
        )
        .unwrap()
    }

    /// Hand-derived from Pint's laravel preset (docs/presets/laravel.json, in its order) and the mapping: its 4 risky
    /// fixers and 11 code-deleting ones are lint rules (context/plan/commands.md §4).
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
"operators/concat-spacing" = "one"
"laravel/blade" = true
"arrays/syntax" = false

[lint.rules]
"refactor/rename-phpdoc-tag" = "error"
"functions/no-unused-closure-use" = "error"
"style/no-alias-function" = "error"
"statements/no-empty" = "error"
"phpdoc/no-superfluous-tags" = { allow-mixed = true, allow-unused-params = true }
"functions/no-unreachable-default-argument-value" = "error"
"imports/no-unused" = "error"
"control-flow/no-useless-return" = "error"
"style/require-ordered-traits" = "error"
"phpunit/require-method-casing" = { case = "snake_case" }
"phpunit/require-protected-setup" = "error"
"phpdoc/no-access" = "error"
"phpdoc/no-package" = "error"
"phpdoc/no-useless-inheritdoc" = "error"
"classes/prefer-self-in-final" = "error"
"style/require-final-class" = "error"
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
            let lint =
                (c.lint.rules.iter()).map(|l| (l.fixer.unwrap().to_string(), l.options.clone()));
            let mut got: Vec<(String, Value)> = (c.rules.iter())
                .map(|r| (r.name.clone(), r.options.clone()))
                .chain(lint)
                .collect();
            let mut want: Vec<(String, Value)> =
                want.into_iter().map(|r| (r.name, r.options)).collect();
            got.sort_by(|a, b| a.0.cmp(&b.0));
            want.sort_by(|a, b| a.0.cmp(&b.0));
            assert_eq!(got, want, "{json}");
            // At `error` with their fix on: `pun check --fix` applies them as Pint did.
            assert!((c.lint.rules.iter()).all(|l| l.fix && l.level == Level::Error && l.named));
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
            r#"{"preset":"empty","rules":{"braces":true,"Custom/fixer":true,"header_comment":{"header":"h","validator":null},"final_internal_class":{"include":null},"@auto":true}}"#,
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
                "final_internal_class: an option is null, which TOML can't hold",
                "@auto: not a PHP-CS-Fixer or Pint rule Pun knows (a custom fixer or rule set)",
            ]
        );
        // The rest still migrates.
        assert_eq!(
            m.toml,
            "[format]\npreset = \"empty\"\n\n[format.rules]\n\"comments/header\" = { header = \"h\" }\n"
        );
    }

    /// pun.toml has no per or symfony preset: their sets expand (sets.json, PHP-CS-Fixer's definitions) and every rule
    /// is written over `empty`. Hand-derived from @PER-CS3x0 and @PSR12 in sets.json, with per's no_unused_imports.
    #[test]
    fn expands_per_and_symfony() {
        let m = pint(r#"{"preset":"per"}"#);
        assert!(m.unsupported.is_empty());
        let c = from_toml(&dir(&[]), &m.toml).unwrap();
        assert_eq!(c.preset, "empty");
        let rules = names(&c.rules);
        // From @PER-CS3x0 itself, from @PER-CS2x0 under it, and from @PSR12 under that.
        for r in [
            "types_spaces",
            "attribute_block_no_spaces",
            "concat_space",
            "lowercase_cast",
        ] {
            assert!(rules.contains(&r), "{r}");
        }
        let lint: Vec<_> = c.lint.rules.iter().map(|l| l.fixer.unwrap()).collect();
        // @PSR12's ordered_class_elements and per's no_unused_imports delete code, so they are lint rules.
        assert_eq!(lint, ["ordered_class_elements", "no_unused_imports"]);
        let m = pint(r#"{"preset":"symfony","rules":{"no_empty_comment":false}}"#);
        let c = from_toml(&dir(&[]), &m.toml).unwrap();
        assert!(
            names(&c.rules).contains(&"phpdoc_align")
                && !names(&c.rules).contains(&"no_empty_comment")
        );
    }

    /// A rule Pun has no fixer for is named and skipped; one the preset brings is switched off, so pun.toml runs.
    /// Hand-derived from the laravel preset: single_quote (format) and final_class (lint, pint.json's) left out.
    #[test]
    fn skips_rules_pun_cant_run() {
        let runs = |n: &str| n != "single_quote" && n != "final_class";
        let d = dir(&[("pint.json", r#"{"rules":{"final_class":true}}"#)]);
        let m = from_pint(&d.join("pint.json"), None, &runs).unwrap();
        assert_eq!(
            m.unsupported,
            [
                "single_quote: not supported by Pun yet",
                "final_class: not supported by Pun yet"
            ]
        );
        assert!(
            m.toml
                .contains("\n[format.rules]\n\"strings/single-quote\" = false\n"),
            "{}",
            m.toml
        );
        assert!(!m.toml.contains("require-final-class"));
    }

    #[test]
    fn php_cs_fixer_rules() {
        let rules =
            json!({"@PSR12": true, "strict_param": true, "concat_space": {"spacing": "one"}});
        let finder = json!({"in": ["src", "/elsewhere"], "exclude": [], "notName": ["*.inc"], "notPath": "x/y.php", "unsupported": ["depth()"]});
        let m = from_rules(
            rules.as_object().unwrap().clone(),
            finder.as_object().unwrap(),
            &|_| true,
        )
        .unwrap();
        assert_eq!(m.unsupported, ["Finder depth(): no pun.toml key for it"]);
        assert!(
            m.toml.starts_with("[files]\npaths = [\"src\", \"/elsewhere\"]\n\n[format]\npreset = \"empty\"\nnot-name = [\"*.inc\"]\nnot-path = [\"x/y.php\"]\n\n[format.rules]\n"),
            "{}",
            m.toml
        );
        let c = from_toml(&dir(&[]), &m.toml).unwrap();
        assert_eq!(c.preset, "empty");
        assert!(
            names(&c.rules).contains(&"braces_position")
                && names(&c.rules).contains(&"concat_space")
        );
        let fixers: Vec<_> = c.lint.rules.iter().map(|l| l.fixer.unwrap()).collect();
        assert_eq!(fixers, ["ordered_class_elements", "strict_param"]);
    }

    /// Messages are Pint's (ConfigurationJsonRepository, RuleSet; real Pint 1.29.3 on the same pint.json).
    #[test]
    fn pint_errors() {
        let e = |files: &[(&str, &str)]| {
            from_pint(&dir(files).join("pint.json"), None, &|_| true)
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
            from_pint(&dir(&[]).join("pint.json"), Some("psr12"), &|_| true)
                .unwrap()
                .toml,
            "[format]\npreset = \"psr12\"\n\n[lint.rules]\n\"classes/require-member-order\" = { order = [\"use_trait\"] }\n\"imports/no-unused\" = \"error\"\n"
        );
        assert_eq!(
            e(&[
                ("a.json", r#"{"extend":"b.json"}"#),
                ("pint.json", r#"{"extend":"a.json"}"#)
            ]),
            "Pint configuration cannot extend from more than 1 file."
        );
        assert!(e(&[("pint.json", "1")]).ends_with("pint.json] is not valid JSON."));
        // An unreadable pint.json names the file (a directory, which root can't read either).
        let d = dir(&[]);
        std::fs::create_dir(d.join("pint.json")).unwrap();
        let err = from_pint(&d.join("pint.json"), None, &|_| true)
            .err()
            .unwrap();
        assert_eq!(
            format!("{err:#}"),
            format!(
                "can't read {}: Is a directory (os error 21)",
                d.join("pint.json").display()
            )
        );
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
        let m = from_pint(&d.join("custom.json"), None, &|_| true).unwrap();
        assert_eq!(
            m.toml,
            "[files]\npaths = [\"src\"]\n\n[format]\npreset = \"empty\"\nexclude = [\"x\", \"b\"]\nnot-name = [\"n\", \"m\"]\n\n[format.rules]\n\"strings/single-quote\" = true\n\"operators/concat-spacing\" = \"one\"\n"
        );
    }
}

mod options;
