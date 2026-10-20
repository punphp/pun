//! phpstan.neon to `[analyse]`. Levels, packages and identifiers are rules/mapping.toml's `[upstream.phpstan-classes]`
//! and `[upstream.phpstan]` rows (PHPStan 2.2.7's rule registry); the parameter handling is PHPStan's own
//! (`NeonAdapter` makes paths absolute, `bleedingEdge.neon` ships in the phar).
use super::*;
use crate::migrate::Parts;
use crate::migrate::phpstan::{Phpstan, compose, from_phpstan};

fn project(neon: &str, extra: &[(&str, &str)]) -> PathBuf {
    let d = dir(&[]);
    for p in [
        "app/Legacy",
        "tests",
        "stubs",
        "vendor/larastan/larastan",
        "vendor/foo/bar",
        "build",
    ] {
        std::fs::create_dir_all(d.join(p)).unwrap();
    }
    // As phpstan/phpstan ships it: the real file is in the phar.
    std::fs::create_dir_all(d.join("vendor/phpstan/phpstan/conf")).unwrap();
    std::fs::write(
        d.join("vendor/phpstan/phpstan/conf/bleedingEdge.neon"),
        "includes:\n\t- phar://phpstan.phar/conf/bleedingEdge.neon\n",
    )
    .unwrap();
    std::fs::write(d.join("phpstan.neon"), neon).unwrap();
    std::fs::create_dir_all(d.join("app/Models")).unwrap();
    std::fs::write(d.join("app/Models/User.php"), "<?php\n").unwrap();
    for (n, c) in extra {
        std::fs::create_dir_all(d.join(n).parent().unwrap()).unwrap();
        std::fs::write(d.join(n), c).unwrap();
    }
    d
}

const NEON: &str = "includes:
    - vendor/larastan/larastan/extension.neon
    - vendor/phpstan/phpstan/conf/bleedingEdge.neon
    - phpstan-baseline.neon
    - build/custom.neon
    - vendor/phpstan/phpstan/conf/extra.neon
parameters:
    level: 5
    paths:
        - app
        - tests/
    excludePaths:
        - app/Legacy
        - /tmp
    phpVersion: 80400
    tmpDir: build/phpstan
    checkUninitializedProperties: true
    checkMissingTypehints: true
    treatPhpDocTypesAsCertain: true
    universalObjectCratesClasses: [App\\Bag]
    checkBenevolentUnionTypes: true
    additionalConstructors: [App\\Bag::setUp]
    exceptions:
        implicitThrows: false
    stubFiles:
        - stubs/Foo.stub
    ignoreErrors:
        - '#^Call to an undefined method#'
        -
            identifier: method.notFound
            path: app/Models/User.php
            count: 2
        -
            message: '#Access#'
            identifiers: [property.notFound, number.alwaysTrue]
            paths: [app/*]
        -
            rawMessage: 'Exact.'
            identifier: no.suchIdentifier
        -
            identifier: whitespace.bom
        -
            identifier: phpstanApi.method
";

/// phpstan/extension-installer's GeneratedConfig.php as version 1.4 writes it (trimmed to three packages).
const GENERATED: &str = "<?php declare(strict_types = 1);

namespace PHPStan\\ExtensionInstaller;

final class GeneratedConfig
{

\tpublic const EXTENSIONS = array (
  'foo/bar' => 
  array (
    'install_path' => '/p/vendor/foo/bar',
    'relative_install_path' => '../../../foo/bar',
    'extra' => NULL,
    'version' => '1.0.0',
  ),
  'phpstan/phpstan-mockery' => 
  array (
    'install_path' => '/p/vendor/phpstan/phpstan-mockery',
    'relative_install_path' => '../../phpstan-mockery',
    'extra' => 
    array (
      'includes' => 
      array (
        0 => 'extension.neon',
      ),
    ),
    'version' => '2.0.0',
  ),
  'phpstan/phpstan-strict-rules' => 
  array (
    'install_path' => '/p/vendor/phpstan/phpstan-strict-rules',
    'relative_install_path' => '../../phpstan-strict-rules',
    'extra' => 
    array (
      'includes' => 
      array (
        0 => 'rules.neon',
      ),
    ),
    'version' => '2.0.7',
  ),
);

\tpublic const NOT_INSTALLED = array (
  'phpstan/phpstan-phpunit' => 
  array (
  ),
);

}
";

/// Each part of the config lands where the plan puts it; what Pun can't take is named.
#[test]
fn reads_level_paths_ignores_and_includes() {
    let d = project(
        NEON,
        &[
            ("vendor/larastan/larastan/extension.neon", ""),
            ("vendor/foo/bar/rules.neon", ""),
            ("vendor/phpstan/phpstan/conf/extra.neon", ""),
            ("vendor/phpstan/phpstan-mockery/extension.neon", ""),
            ("vendor/phpstan/phpstan-strict-rules/rules.neon", ""),
            // A package's file included from a subdirectory is the package's.
            (
                "build/custom.neon",
                "includes:\n    - ../vendor/foo/bar/rules.neon\nparameters:\n    checkFunctionNameCase: true\n",
            ),
            (
                "phpstan-baseline.neon",
                "parameters:\n    ignoreErrors:\n        - '#x#'\n",
            ),
        ],
    );
    let installer = d.join("vendor/phpstan/extension-installer/src");
    std::fs::create_dir_all(&installer).unwrap();
    let generated = GENERATED.replace("'/p/", &format!("'{}/", d.display()));
    std::fs::write(installer.join("GeneratedConfig.php"), generated).unwrap();
    let p = from_phpstan(&d, &d.join("phpstan.neon")).unwrap();
    assert_eq!(p.level, 5);
    assert_eq!(p.paths, ["app", "tests"]);
    assert_eq!(p.exclude, ["app/Legacy", "/tmp"]);
    assert_eq!(p.stubs, ["stubs/Foo.stub"]);
    assert_eq!(p.baselines, ["phpstan-baseline.neon"]);
    // strict-rules through the extension installer; NOT_INSTALLED is not read.
    assert_eq!(
        p.extensions.iter().collect::<Vec<_>>(),
        ["larastan/larastan", "phpstan/phpstan-strict-rules"]
    );
    let names: Vec<&str> = p.unsupported.iter().map(|u| u.0.as_str()).collect();
    assert_eq!(
        names,
        [
            "vendor/phpstan/phpstan/conf/bleedingEdge.neon",
            "build/custom.neon",
            "foo/bar",
            "phpstan/phpstan-mockery",
            "parameter checkMissingTypehints",
            "parameter phpVersion",
            "parameter universalObjectCratesClasses",
            "ignoreErrors no.suchIdentifier",
            "ignoreErrors whitespace.bom",
            "ignoreErrors phpstanApi.method",
        ]
    );
    let why = |n: &str| &p.unsupported.iter().find(|u| u.0 == n).unwrap().1;
    assert_eq!(
        why("ignoreErrors whitespace.bom"),
        "`whitespace.bom` is the format rule `format/whitespace/encoding` in Pun; ignore it there"
    );
    // Against PHPStan's value at level 5 (conf/config.level*.neon, conf/config.neon): checkFunctionNameCase (false)
    // is an option of the rules that read it, checkUninitializedProperties gates a rule (compose turns it on),
    // treatPhpDocTypesAsCertain is already true, checkMissingTypehints is level 6's, and only a lint rule reads
    // universalObjectCratesClasses.
    assert_eq!(
        p.changed,
        [
            (
                "additionalConstructors".to_string(),
                pun_phpstan::Value::Array(pun_phpstan::Array::list([pun_phpstan::Value::Str(
                    "App\\Bag::setUp".into()
                )]))
            ),
            (
                "checkBenevolentUnionTypes".to_string(),
                pun_phpstan::Value::Bool(true)
            ),
            (
                "checkFunctionNameCase".to_string(),
                pun_phpstan::Value::Bool(true)
            ),
            (
                "exceptions.implicitThrows".to_string(),
                pun_phpstan::Value::Bool(false)
            ),
        ]
    );
    assert_eq!(
        why("parameter checkMissingTypehints"),
        "the level sets it; Pun's analyse presets decide it"
    );
    assert_eq!(
        why("parameter universalObjectCratesClasses"),
        "no pun.toml setting for this parameter yet"
    );
    assert_eq!(
        why("ignoreErrors phpstanApi.method"),
        "`phpstanApi.method`: PHPStan extension-API checks; Pun has no PHP extension API"
    );
    // The baseline's own ignoreErrors are not read.
    assert_eq!(
        p.ignores,
        [
            "[[analyse.ignore]]\nmessage = \"#^Call to an undefined method#\"\n",
            "[[analyse.ignore]]\nrule = \"undefined/method\"\nidentifier = \"method.notFound\"\npaths = [\"app/Models/User.php\"]\ncount = 2\n",
            "[[analyse.ignore]]\nrule = \"undefined/property\"\nidentifier = \"property.notFound\"\nmessage = \"#Access#\"\npaths = [\"app/*\"]\n",
            "[[analyse.ignore]]\nrule = \"dead-code/number-comparison-always-constant\"\nidentifier = \"number.alwaysTrue\"\nmessage = \"#Access#\"\npaths = [\"app/*\"]\n",
        ]
    );
    let mut parts = Parts::default();
    let done = compose(&d, &mut parts, &p).unwrap();
    // CallToNonExistentFunctionRule reads checkFunctionNameCase (its constructor's #[AutowiredParameter]),
    // CallMethodsRule that and checkBenevolentUnionTypes (through RuleLevelHelper), UninitializedPropertyRule
    // additionalConstructors (through ConstructorsHelper). MethodThrowTypeCovarianceRule reads
    // exceptions.implicitThrows, but it runs only under exceptions.check.throwTypeCovariance, so that one is applied
    // nowhere.
    let text = parts.render();
    for line in [
        "\"undefined/function\" = { check-function-name-case = true }",
        "\"undefined/method\" = { check-benevolent-union-types = true, check-function-name-case = true }",
        "\"classes/uninitialized-property\" = { additional-constructors = ['App\\Bag::setUp'] }",
        "\"types/throws-not-covariant\" = \"off\"",
    ] {
        assert!(text.contains(&format!("\n{line}\n")), "{line}\n{text}");
    }
    assert_eq!(done.options, 3);
    let cfg = crate::from_toml(&d, &text).unwrap();
    let function = cfg
        .analyse
        .rules
        .iter()
        .find(|r| r.id == "analyse/undefined/function");
    assert_eq!(
        function.unwrap().options,
        serde_json::json!({"check-function-name-case": true})
    );
    assert!(
        parts.render().starts_with(
            "[analyse]\npreset = \"recommended\"\npaths = [\"app\", \"tests\"]\nexclude = [\"app/Legacy\", \"/tmp\"]\nstubs = [\"stubs/Foo.stub\"]\n"
        ),
        "{}",
        parts.render()
    );
}

/// `level: max` is 10; a level PHPStan rejects stops the migration with PHPStan's message.
#[test]
fn reads_max_and_rejects_bad_levels() {
    let d = project("parameters:\n    level: max\n", &[]);
    assert_eq!(from_phpstan(&d, &d.join("phpstan.neon")).unwrap().level, 10);
    let d = project("parameters:\n    level: 11\n", &[]);
    let e = from_phpstan(&d, &d.join("phpstan.neon"))
        .unwrap_err()
        .to_string();
    assert!(
        e.starts_with("Level config file phar://")
            && e.ends_with("/conf/config.level11.neon was not found."),
        "{e}"
    );
}

/// Level 5 is the `recommended` preset: every core rule up to level 5 stays on, deprecation-rules' checks (in
/// `recommended`) are off without their include, and so is a check a PHPStan parameter keeps off; at level 4 the level-5 checks are off too. strict-rules' checks run
/// with the include when their parameter is on: an analyse rule outside the preset gets `"on"`, a lint rule Pun hasn't
/// ported is named. The ignores render as `[[analyse.ignore]]` tables pun.toml accepts.
#[test]
fn composes_the_level_extensions_and_parameters() {
    // PHPStan's resolved parameters at a level, with extensions and strictRules parameters added.
    let at_in =
        |level: u8, ext: &[&str], strict: &[(&str, bool)], files: &[(&str, &str)], format: &str| {
            let d = project(&format!("parameters:\n    level: {level}\n"), files);
            let mut p: Phpstan = from_phpstan(&d, &d.join("phpstan.neon")).unwrap();
            p.extensions = ext.iter().map(|e| e.to_string()).collect();
            let mut rules = pun_phpstan::Array::new();
            for (k, v) in strict {
                rules.set(pun_phpstan::Key::from(*k), pun_phpstan::Value::Bool(*v));
            }
            p.parameters.set(
                pun_phpstan::Key::from("strictRules"),
                pun_phpstan::Value::Array(rules),
            );
            p.ignores =
                vec!["[[analyse.ignore]]\nrule = \"undefined/method\"\nmessage = \"#x#\"\n".into()];
            let mut parts = Parts {
                format: format.into(),
                ..Parts::default()
            };
            let done = compose(&d, &mut parts, &p).unwrap();
            (done, parts, d)
        };
    let at = |level: u8, ext: &[&str], strict: &[(&str, bool)]| {
        at_in(level, ext, strict, &[("composer.json", "{}")], "")
    };
    let lines = |parts: &Parts, v: &str| -> Vec<String> {
        (parts.analyse_rules.iter())
            .filter(|(_, x)| x == v)
            .map(|(k, _)| k.clone())
            .collect()
    };
    let (done, parts, d) = at(5, &[], &[]);
    assert_eq!(done.preset, AnalysePreset::Recommended);
    let off = lines(&parts, "\"off\"");
    assert!(off.contains(&"deprecated/class".to_string()), "{off:?}");
    // checkUninitializedProperties is false by default (PHPStan's config.neon).
    assert!(
        off.contains(&"classes/uninitialized-property".to_string()),
        "{off:?}"
    );
    // Bleeding edge only (featureToggles.checkPrintfParameterTypes).
    assert!(
        off.contains(&"types/printf-argument".to_string()),
        "{off:?}"
    );
    assert!(
        !off.contains(&"types/random-int-range".to_string()),
        "{off:?}"
    );
    assert_eq!((done.on, done.off), (0, off.len()));
    let cfg = crate::from_toml(&d, &parts.render()).unwrap();
    assert_eq!(cfg.analyse.ignore.len(), 1);
    assert_eq!(
        cfg.analyse.ignore[0].rule.as_deref(),
        Some("analyse/undefined/method")
    );
    // deprecation-rules' class check is off; PHPStan's own deprecated-cast check (level 0) stays.
    let has = |id: &str| cfg.analyse.rules.iter().any(|r| r.id == id);
    assert!(!has("analyse/deprecated/class") && has("analyse/deprecated/cast"));

    let (done, parts, _) = at(4, &[], &[]);
    assert_eq!(done.preset, AnalysePreset::Recommended);
    assert!(lines(&parts, "\"off\"").contains(&"types/random-int-range".to_string()));

    let strict = "phpstan/phpstan-strict-rules";
    let params = [
        ("booleansInConditions", true),
        ("disallowedEmpty", true),
        ("uselessCast", false),
    ];
    let (done, parts, _) = at(0, &[strict], &params);
    assert_eq!(done.preset, AnalysePreset::Basic);
    let on = lines(&parts, "\"on\"");
    assert!(
        on.contains(&"operators/non-boolean-and-operand".to_string()),
        "{on:?}"
    );
    assert!(!on.contains(&"casts/useless".to_string()));
    assert!(
        !on.contains(&"control-flow/non-boolean-while".to_string()),
        "needs bleedingEdge too"
    );
    // `empty()` is the lint rule style/no-empty: on in [lint.rules] once ported, named until then.
    match done.lint {
        1 => assert!(
            parts
                .lint_rules
                .contains(&("style/no-empty".into(), "\"error\"".into()))
        ),
        _ => assert_eq!(
            done.unsupported,
            [(
                "lint/style/no-empty".to_string(),
                "a PHPStan check that is this lint rule in Pun, not supported yet".to_string()
            )]
        ),
    }
    // Laravel locked without Larastan: the whole `laravel` group off, no line per Larastan rule.
    let (done, parts, _) = at_in(5, &[], &[], &[("composer.lock", super::LARAVEL_LOCK)], "");
    assert!((parts.analyse_rules.iter()).any(|(k, v)| k == "laravel" && v == "\"off\""));
    assert!(
        !parts
            .analyse_rules
            .iter()
            .any(|(k, _)| k.starts_with("laravel/"))
    );
    assert_eq!(done.off, lines(&parts, "\"off\"").len());

    // strict-rules' backtick ban is the format rule functions/backtick-to-shell-exec, which an empty format config
    // doesn't run.
    let backtick = [("disallowedBacktick", true)];
    let format_rule = (
        "format/functions/backtick-to-shell-exec".to_string(),
        "a PHPStan check that is this format rule in Pun; turn it on in [format.rules]".to_string(),
    );
    let (done, ..) = at_in(0, &[strict], &backtick, &[], "preset = \"empty\"\n");
    // PHPStan's BOM check (level 0) is the format rule whitespace/encoding, which the empty config doesn't run either.
    let encoding = (
        "format/whitespace/encoding".to_string(),
        format_rule.1.clone(),
    );
    assert_eq!(done.unsupported, [format_rule, encoding]);
}

/// Values read off rules/mapping.toml by hand: an exact identifier, a `%s` pattern, a Larastan identifier.
#[test]
fn identifiers_and_groups() {
    use crate::rule_of_identifier;
    assert_eq!(
        rule_of_identifier("function.notFound"),
        Some("analyse/undefined/function")
    );
    assert_eq!(
        rule_of_identifier("greater.alwaysTrue"),
        Some("analyse/dead-code/number-comparison-always-constant")
    );
    assert_eq!(
        rule_of_identifier("larastan.configCollection"),
        Some("analyse/laravel/config-collection")
    );
    assert_eq!(
        rule_of_identifier("whitespace.bom"),
        Some("format/whitespace/encoding")
    );
    assert_eq!(rule_of_identifier("no.such.identifier"), None);
    assert_eq!(rule_of_identifier(".alwaysTrue"), None);
    assert!(crate::groups().iter().any(|g| g == "dead-code"));
    assert!(!crate::groups().iter().any(|g| g == "dead"));
}

/// Without a level or `customRulesetUsed`, PHPStan stops with "No rules detected" (CommandHelper); the migrated
/// `[analyse]` turns every analyse group off, so `pun analyse` stops alike. With a level, rules run.
#[test]
fn no_level_runs_no_rules() {
    let d = project(
        "parameters:\n    paths: [app]\n",
        &[("composer.json", "{}")],
    );
    let p = from_phpstan(&d, &d.join("phpstan.neon")).unwrap();
    assert!(p.no_rules);
    let mut parts = Parts::default();
    let done = compose(&d, &mut parts, &p).unwrap();
    assert!(done.off > 10 && done.on == 0, "{done:?}");
    std::fs::write(d.join("pun.toml"), parts.render()).unwrap();
    let cfg = load(&d, &Options::default()).unwrap();
    assert!(cfg.analyse.rules.is_empty());
    let d = project("parameters:\n    level: 0\n", &[]);
    assert!(!from_phpstan(&d, &d.join("phpstan.neon")).unwrap().no_rules);
}
