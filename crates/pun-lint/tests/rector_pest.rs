//! pest-plugin-rector's own fixtures for the ported `pest` rules, copied verbatim from pestphp/pest-plugin-rector at
//! 1a4b6b7 (tests/Rules/...) into tests/rector/<Rector class>/Fixture. Oracle/ holds extra inputs with what Rector
//! 2.6.1 and the plugin at that commit (`.oracle/rector-ecosystem`, the rule alone, `withPhpVersion(80400)`) wrote for
//! them. Pun/ holds the inputs where Rector's one pass over the file leaves code that the same rule changes again (a
//! double negation is undone one `!` at a time, a type matcher becomes redundant once the one before it goes); Pun's
//! fix rounds go on until the rule has nothing left, so the expected code is the oracle's output run through the oracle
//! again until it stops changing. A fixture is `input -----\n expected`, or only the input when the rule must not
//! change it. Each one must pass as `pun lint --fix == expected` and `format(pun lint --fix) == format(expected)`,
//! format being the laravel preset.

use std::path::{Path, PathBuf};

use pun_format::Pipeline;
use pun_lint::rules::*;
use pun_lint::{Linter, Rule};

/// The fixtures of one directory, sorted: `(path, input, expected)`.
fn fixtures(dir: &Path) -> Vec<(PathBuf, String, String)> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    paths.sort();
    assert!(!paths.is_empty(), "no fixtures in {}", dir.display());
    paths
        .into_iter()
        .map(|path| {
            let content = std::fs::read_to_string(&path)
                .unwrap()
                .replace("\r\n", "\n");
            let (input, expected) = match content.split_once("-----\n") {
                Some((i, e)) => (i.to_string(), e.to_string()),
                None => (content.clone(), content),
            };
            (path, input, expected)
        })
        .collect()
}

fn format(pipeline: &Pipeline, code: &str) -> String {
    match pipeline
        .run(Path::new("fixture.php"), code.as_bytes())
        .unwrap()
    {
        Some(fixed) => String::from_utf8(fixed.code).unwrap(),
        None => code.to_string(),
    }
}

fn laravel() -> Pipeline {
    let rules = pun_config::rule_set(pun_config::preset_rules("laravel").unwrap()).unwrap();
    let pipeline = Pipeline::new(&rules, &pun_fixers::all).unwrap();
    assert!(pipeline.not_ported.is_empty(), "{:?}", pipeline.not_ported);
    pipeline
}

/// Run every fixture of tests/rector/<class>/{Fixture,Oracle,Pun} (Fixture must exist) through `rules` at PHP 8.4.
///
/// An upstream fixture's expected code may lack one final newline of Rector's output: the plugin's tests compare with
/// `assertSame` and then `assertStringMatchesFormat`, whose pattern ends in `$`, which also matches before a final
/// newline. The oracle's files are Rector's output byte for byte.
fn run_class(rules: Vec<Box<dyn Rule>>, class: &str) {
    let linter = Linter::new(rules, 80400);
    let pipeline = laravel();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/rector")
        .join(class);
    assert!(
        root.join("Fixture").is_dir(),
        "{class}: no Fixture directory"
    );
    for sub in ["Fixture", "Oracle", "Pun"] {
        let dir = root.join(sub);
        if !dir.is_dir() {
            continue;
        }
        for (path, input, expected) in fixtures(&dir) {
            let name = format!("{sub}/{}", path.file_name().unwrap().to_string_lossy());
            let fixed = linter.fix(&path, &input).unwrap();
            let expected = if sub == "Fixture" && fixed.source == format!("{expected}\n") {
                fixed.source.clone()
            } else {
                expected
            };
            assert_eq!(fixed.source, expected, "{class} {name}: lint --fix");
            assert_eq!(
                format(&pipeline, &fixed.source),
                format(&pipeline, &expected),
                "{class} {name}: format(lint --fix)"
            );
            if input != expected {
                // The rule did the work, not the formatter.
                assert_ne!(
                    format(&pipeline, &input),
                    format(&pipeline, &expected),
                    "{class} {name}"
                );
                assert!(!fixed.applied.is_empty(), "{class} {name}");
            } else {
                assert!(fixed.applied.is_empty(), "{class} {name}");
            }
        }
    }
}

fn check(rule: impl Rule + 'static, class: &str) {
    run_class(vec![Box::new(rule)], class);
}

#[test]
fn tap_to_defer() {
    check(PreferDefer, "TapToDeferRector");
}

#[test]
fn use_to_be_alpha() {
    check(PreferToBeAlpha, "UseToBeAlphaRector");
}

#[test]
fn use_to_be_alpha_numeric() {
    check(PreferToBeAlphaNumeric, "UseToBeAlphaNumericRector");
}

#[test]
fn use_to_be_digits() {
    check(PreferToBeDigits, "UseToBeDigitsRector");
}

#[test]
fn use_to_be_directory() {
    check(PreferToBeDirectory, "UseToBeDirectoryRector");
}

#[test]
fn use_to_be_file() {
    check(PreferToBeFile, "UseToBeFileRector");
}

#[test]
fn use_to_be_in() {
    check(PreferToBeIn, "UseToBeInRector");
}

#[test]
fn use_to_be_infinite() {
    check(PreferToBeInfinite, "UseToBeInfiniteRector");
}

#[test]
fn use_to_be_list() {
    check(PreferToBeList, "UseToBeListRector");
}

#[test]
fn use_to_be_nan() {
    check(PreferToBeNan, "UseToBeNanRector");
}

#[test]
fn use_to_contain() {
    check(PreferToContain, "UseToContainRector");
}

#[test]
fn use_to_contain_equal() {
    check(PreferToContainEqual, "UseToContainEqualRector");
}

#[test]
fn use_to_end_with() {
    check(PreferToEndWith, "UseToEndWithRector");
}

#[test]
fn use_to_have_key() {
    check(PreferToHaveKey, "UseToHaveKeyRector");
}

#[test]
fn use_to_have_property() {
    check(PreferToHaveProperty, "UseToHavePropertyRector");
}

#[test]
fn use_to_start_with() {
    check(PreferToStartWith, "UseToStartWithRector");
}

#[test]
fn simplify_to_be_truthy_falsy() {
    check(PreferToBeTruthyFalsy, "SimplifyToBeTruthyFalsyRector");
}

#[test]
fn to_be_true_not_false() {
    check(NoNegatedBooleanMatcher, "ToBeTrueNotFalseRector");
}

#[test]
fn simplify_to_literal_boolean() {
    check(PreferToBeTrueFalse, "SimplifyToLiteralBooleanRector");
}

#[test]
fn simplify_expect_not() {
    check(PreferNotModifier, "SimplifyExpectNotRector");
}

#[test]
fn simplify_comparison_expectations() {
    check(
        PreferComparisonMatchers,
        "SimplifyComparisonExpectationsRector",
    );
}

#[test]
fn use_strict_equality_matchers() {
    check(
        PreferStrictEqualityMatchers,
        "UseStrictEqualityMatchersRector",
    );
}

#[test]
fn use_type_matchers() {
    check(PreferTypeMatchers, "UseTypeMatchersRector");
}

#[test]
fn use_instance_of_matcher() {
    check(PreferToBeInstanceOf, "UseInstanceOfMatcherRector");
}

#[test]
fn use_to_contain_only_instances_of() {
    check(
        PreferToContainOnlyInstancesOf,
        "UseToContainOnlyInstancesOfRector",
    );
}

#[test]
fn to_have_method_on_class() {
    check(PreferToHaveMethodOnClass, "ToHaveMethodOnClassRector");
}

#[test]
fn remove_only() {
    check(NoFocusedTest, "RemoveOnlyRector");
}

#[test]
fn remove_static_test_closure() {
    check(NoStaticTestClosure, "RemoveStaticTestClosureRector");
}

#[test]
fn use_to_be_lowercase() {
    check(PreferToBeLowercase, "UseToBeLowercaseRector");
}

#[test]
fn use_to_be_uppercase() {
    check(PreferToBeUppercase, "UseToBeUppercaseRector");
}

#[test]
fn use_to_be_between() {
    check(PreferToBeBetween, "UseToBeBetweenRector");
}

#[test]
fn use_to_have_count() {
    check(PreferToHaveCount, "UseToHaveCountRector");
}

#[test]
fn use_to_have_length() {
    check(PreferToHaveLength, "UseToHaveLengthRector");
}

#[test]
fn use_to_be_empty() {
    check(PreferToBeEmpty, "UseToBeEmptyRector");
}

#[test]
fn use_to_be_json() {
    check(PreferToBeJson, "UseToBeJsonRector");
}

#[test]
fn use_to_be_url() {
    check(PreferToBeUrl, "UseToBeUrlRector");
}

#[test]
fn use_to_be_uuid() {
    check(PreferToBeUuid, "UseToBeUuidRector");
}

#[test]
fn use_to_match() {
    check(PreferToMatch, "UseToMatchRector");
}

#[test]
fn use_to_equal_canonicalizing() {
    check(
        PreferToEqualCanonicalizing,
        "UseToEqualCanonicalizingRector",
    );
}

#[test]
fn use_to_equal_with_delta() {
    check(PreferToEqualWithDelta, "UseToEqualWithDeltaRector");
}

#[test]
fn use_to_have_same_size() {
    check(PreferToHaveSameSize, "UseToHaveSameSizeRector");
}

#[test]
fn fix_invalid_repeat_value() {
    check(NoInvalidRepeat, "FixInvalidRepeatValueRector");
}

#[test]
fn convert_before_all_in_describe() {
    check(NoBeforeAllInDescribe, "ConvertBeforeAllInDescribeRector");
}

#[test]
fn use_to_have_keys() {
    check(PreferToHaveKeys, "UseToHaveKeysRector");
}

#[test]
fn use_to_have_properties() {
    check(PreferToHaveProperties, "UseToHavePropertiesRector");
}

#[test]
fn simplify_filesystem_matchers() {
    check(PreferFilesystemMatchers, "SimplifyFilesystemMatchersRector");
}

#[test]
fn remove_redundant_literal_type_expectation() {
    check(
        NoRedundantTypeExpectation,
        "RemoveRedundantLiteralTypeExpectationRector",
    );
}

#[test]
fn use_each_modifier() {
    check(PreferEachModifier, "UseEachModifierRector");
}

#[test]
fn convert_expect_exception_to_throw() {
    check(NoExpectException, "ConvertExpectExceptionToThrowRector");
}

#[test]
fn use_to_throw() {
    check(PreferToThrow, "UseToThrowRector");
}
