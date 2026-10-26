//! rector-phpunit's own fixtures for the ported `phpunit` rules, copied verbatim from rectorphp/rector-phpunit at
//! e68085d (rules-tests/...) into tests/rector/<Rector class>/Fixture (and its Source/). Oracle/ holds extra inputs
//! with what Rector 2.6.1 and that rector-phpunit (`.oracle/rector-ecosystem`, the rule alone, `phpVersion(80400)`)
//! wrote for them; Pun/ holds the documented cases where Pun differs. A fixture is `input -----\n expected`, or only
//! the input when the rule must not change it.
//!
//! rector-phpunit runs its fixtures with phpunit/phpunit 13.2 installed, so every fixture here sees the PHPUnit
//! classes of tests/phpunit-stubs (PHPUnit 13.2.6's files with empty method bodies, scripts/export-phpunit-stub.php)
//! beside its Source/ classes.

use std::path::{Path, PathBuf};

use pun_format::Pipeline;
use pun_lint::rules::facts::with_sources;
use pun_lint::rules::*;
use pun_lint::{Linter, Rule};

/// The PHP files under `dir`, recursively: `(path, code)`.
fn sources(dir: &Path, out: &mut Vec<(String, String)>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries.map(|e| e.unwrap().path()).collect();
    paths.sort();
    for path in paths {
        if path.is_dir() {
            sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "php") {
            out.push((
                path.to_string_lossy().into_owned(),
                std::fs::read_to_string(&path).unwrap(),
            ));
        }
    }
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

/// Run every fixture of tests/rector/<class>/{Fixture,Oracle,Pun} through `rules` at PHP 8.4: the text edits must
/// give the expected bytes, and the laravel preset's format of both must agree.
fn run_class(rules: Vec<Box<dyn Rule>>, class: &str) {
    let linter = Linter::new(rules, 80400);
    let preset = pun_config::rule_set(pun_config::preset_rules("laravel").unwrap()).unwrap();
    let pipeline = Pipeline::new(&preset, &pun_fixers::all).unwrap();
    let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let root = base.join("rector").join(class);
    assert!(
        root.join("Fixture").is_dir(),
        "{class}: no Fixture directory"
    );
    let mut extra = Vec::new();
    sources(&base.join("phpunit-stubs"), &mut extra);
    sources(&root.join("Source"), &mut extra);
    for sub in ["Fixture", "Oracle", "Pun"] {
        let dir = root.join(sub);
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        let mut paths: Vec<PathBuf> = entries.map(|e| e.unwrap().path()).collect();
        paths.sort();
        assert!(!paths.is_empty(), "no fixtures in {}", dir.display());
        for path in paths {
            let content = std::fs::read_to_string(&path)
                .unwrap()
                .replace("\r\n", "\n");
            let (input, expected) = match content.split_once("-----\n") {
                Some((i, e)) => (i.to_string(), e.to_string()),
                None => (content.clone(), content.clone()),
            };
            let name = format!(
                "{class} {sub}/{}",
                path.file_name().unwrap().to_string_lossy()
            );
            let fixed = with_sources(extra.clone(), || linter.fix(&path, &input)).unwrap();
            assert_eq!(fixed.source, expected, "{name}: lint --fix");
            assert_eq!(
                format(&pipeline, &fixed.source),
                format(&pipeline, &expected),
                "{name}: format(lint --fix)"
            );
            assert_eq!(fixed.applied.is_empty(), input == expected, "{name}");
        }
    }
}

fn check(rule: impl Rule + 'static, class: &str) {
    run_class(vec![Box::new(rule)], class);
}

#[test]
fn remove_empty_test_method() {
    check(NoEmptyTest, "RemoveEmptyTestMethodRector");
}

#[test]
fn assert_not_operator() {
    check(NoNegatedAssertion, "AssertNotOperatorRector");
}

#[test]
fn assert_same_bool_null_to_specific_method() {
    check(
        PreferAssertNullTrueFalse,
        "AssertSameBoolNullToSpecificMethodRector",
    );
}

#[test]
fn assert_same_true_false_to_assert_true_false() {
    check(
        PreferAssertTrueFalse,
        "AssertSameTrueFalseToAssertTrueFalseRector",
    );
}

#[test]
fn assert_false_strpos_to_contains() {
    check(
        PreferAssertStringContains,
        "AssertFalseStrposToContainsRector",
    );
}

#[test]
fn assert_is_type_method_call() {
    check(NoAssertIsType, "AssertIsTypeMethodCallRector");
}

#[test]
fn assert_contains_only_method_call() {
    check(NoAssertContainsOnly, "AssertContainsOnlyMethodCallRector");
}

#[test]
fn assert_comparison_to_specific_method() {
    check(
        PreferComparisonAssertion,
        "AssertComparisonToSpecificMethodRector",
    );
}

#[test]
fn assert_instance_of_comparison() {
    check(PreferAssertInstanceof, "AssertInstanceOfComparisonRector");
}

#[test]
fn flip_assert() {
    check(NoFlippedAssertion, "FlipAssertRector");
}

#[test]
fn assert_equals_or_assert_same_float_parameter_to_specific_methods_type() {
    check(
        NoFloatDeltaParameter,
        "AssertEqualsOrAssertSameFloatParameterToSpecificMethodsTypeRector",
    );
}

#[test]
fn assert_true_false_to_specific_method() {
    check(
        PreferSpecificTrueFalseAssertion,
        "AssertTrueFalseToSpecificMethodRector",
    );
}

#[test]
fn assert_isset_to_specific_method() {
    check(PreferAssertArrayHasKey, "AssertIssetToSpecificMethodRector");
}

#[test]
fn string_cast_assert_string_contains_string() {
    check(
        RequireStringAssertArgument,
        "StringCastAssertStringContainsStringRector",
    );
}

#[test]
fn simplify_foreach_instance_of() {
    check(PreferAssertInstancesOf, "SimplifyForeachInstanceOfRector");
}

#[test]
fn match_assert_same_expected_type() {
    check(
        NoAssertSameTypeMismatch,
        "MatchAssertSameExpectedTypeRector",
    );
}

#[test]
fn assert_equals_to_same() {
    check(PreferSameForScalars, "AssertEqualsToSameRector");
}

#[test]
fn assert_compare_on_countable_with_method_to_assert_count() {
    check(
        PreferAssertCount,
        "AssertCompareOnCountableWithMethodToAssertCountRector",
    );
}

#[test]
fn remove_expect_any_from_mock() {
    check(NoExpectsAny, "RemoveExpectAnyFromMockRector");
}

#[test]
fn use_specific_with_method() {
    check(PreferSimpleWithMatcher, "UseSpecificWithMethodRector");
}

#[test]
fn use_specific_will_method() {
    check(PreferSpecificWillMethod, "UseSpecificWillMethodRector");
}

#[test]
fn single_with_consecutive_to_with() {
    check(
        PreferSimpleMockExpectation,
        "SingleWithConsecutiveToWithRector",
    );
}

#[test]
fn narrow_identical_with_consecutive() {
    check(
        PreferNarrowWithConsecutive,
        "NarrowIdenticalWithConsecutiveRector",
    );
}

#[test]
fn remove_override_final_construct_test_case() {
    check(
        NoFinalConstructorOverride,
        "RemoveOverrideFinalConstructTestCaseRector",
    );
}

#[test]
fn no_setup_with_parent_call_override() {
    check(
        NoSetupOverrideAttribute,
        "NoSetupWithParentCallOverrideRector",
    );
}

#[test]
fn declare_strict_types_tests() {
    check(RequireStrictTypesInTests, "DeclareStrictTypesTestsRector");
}

#[test]
fn prefer_p_h_p_unit_this_call() {
    check(PreferThisCall, "PreferPHPUnitThisCallRector");
}

#[test]
fn remove_standalone_create_mock() {
    check(NoUnusedMock, "RemoveStandaloneCreateMockRector");
}

#[test]
fn replace_at_method_with_desired_matcher() {
    check(NoAtMatcher, "ReplaceAtMethodWithDesiredMatcherRector");
}

#[test]
fn get_mock_builder_get_mock_to_create_mock() {
    check(NoBareMockBuilder, "GetMockBuilderGetMockToCreateMockRector");
}

#[test]
fn create_stub_in_coalesce_arg() {
    check(PreferCreateStubInCoalesce, "CreateStubInCoalesceArgRector");
}

#[test]
fn data_provider_array_items_new_lined() {
    check(
        RequireDataProviderNewlines,
        "DataProviderArrayItemsNewLinedRector",
    );
}

#[test]
fn simpler_with_is_instance_of() {
    check(PreferWithIsInstanceOf, "SimplerWithIsInstanceOfRector");
}

#[test]
fn callback_single_assert_to_simpler() {
    check(
        PreferSimpleCallbackAssert,
        "CallbackSingleAssertToSimplerRector",
    );
}

#[test]
fn narrow_single_will_return_callback() {
    check(
        PreferNarrowWillReturnCallback,
        "NarrowSingleWillReturnCallbackRector",
    );
}

#[test]
fn assert_func_call_to_p_h_p_unit_assert() {
    check(PreferAssert, "AssertFuncCallToPHPUnitAssertRector");
}

#[test]
fn merge_with_callable_and_will_return() {
    check(
        PreferMergedWillReturnCallback,
        "MergeWithCallableAndWillReturnRector",
    );
}

#[test]
fn add_return_type_to_depended() {
    check(AddDependedTestReturnType, "AddReturnTypeToDependedRector");
}

#[test]
fn construct_class_method_to_set_up_test_case() {
    check(
        NoTestConstructor,
        "ConstructClassMethodToSetUpTestCaseRector",
    );
}
