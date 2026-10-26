//! pest-plugin-rector's own fixtures for the ported `pest` rules, copied verbatim from pestphp/pest-plugin-rector at
//! 1a4b6b7 (tests/Rules/...) into tests/rector/<Rector class>/Fixture. Oracle/ holds extra inputs with what Rector
//! 2.6.1 and the plugin at that commit (`.oracle/rector-ecosystem`, the rule alone, `withPhpVersion(80400)`) wrote for
//! them. Pun/ holds the inputs where Rector's one pass over the file leaves code that the same rule changes again (a
//! double negation is undone one `!` at a time, a type matcher becomes redundant once the one before it goes, a value
//! check before two type checks moves one statement a pass); Pun's fix rounds go on until the rule has nothing left, so
//! the expected code is the oracle's output run through the oracle again until it stops changing. The one other Pun/
//! case, RemoveRedundantPestUsesRector/Pun/InlineHtmlProject, is a file with inline HTML whose statement list
//! php-parser gives up on: Rector prints the whole file again (`<?php\n\n?>\nhtml\n<?php `), which Pun does not port,
//! so Pun leaves the file alone. A fixture is `input -----\n expected`, or only the input when the rule must not change
//! it. Each one must pass as `pun lint --fix == expected` and `format(pun lint --fix) == format(expected)`, format
//! being the laravel preset.

use std::path::{Path, PathBuf};

use mago_syntax::cst::Node;
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
            check_fixture(
                &linter,
                &pipeline,
                &format!("{class} {name}"),
                sub == "Fixture",
                &path,
                &input,
                expected,
            );
        }
    }
}

/// One fixture through `linter` as the file at `path`: `lint --fix` gives `expected` (an upstream fixture may lack
/// Rector's final newline), the formatter agrees, and the rule did the work.
fn check_fixture(
    linter: &Linter,
    pipeline: &Pipeline,
    name: &str,
    upstream: bool,
    path: &Path,
    input: &str,
    expected: String,
) {
    let fixed = linter.fix(path, input).unwrap();
    let expected = if upstream && fixed.source == format!("{expected}\n") {
        fixed.source.clone()
    } else {
        expected
    };
    assert_eq!(fixed.source, expected, "{name}: lint --fix");
    assert_eq!(
        format(pipeline, &fixed.source),
        format(pipeline, &expected),
        "{name}: format(lint --fix)"
    );
    if input != expected {
        // The rule did the work, not the formatter.
        assert_ne!(
            format(pipeline, input),
            format(pipeline, &expected),
            "{name}"
        );
        assert!(!fixed.applied.is_empty(), "{name}");
    } else {
        assert!(fixed.applied.is_empty(), "{name}");
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

#[test]
fn use_to_match_object() {
    check(PreferToMatchObject, "UseToMatchObjectRector");
}

#[test]
fn use_to_match_array() {
    check(PreferToMatchArray, "UseToMatchArrayRector");
}

#[test]
fn use_sequence_matcher() {
    check(PreferSequence, "UseSequenceMatcherRector");
}

#[test]
fn chain_expect_calls() {
    check(PreferChainedExpect::default(), "ChainExpectCallsRector");
}

#[test]
fn chain_expect_calls_without_merge_different_variables() {
    let mut rule = PreferChainedExpect::default();
    rule.configure(&serde_json::json!({ "merge-different-variables": false }))
        .unwrap();
    check(rule, "ChainExpectCallsRectorWithoutMergeDifferentVariables");
}

/// No options keep the plugin's default (merging different variables); an unknown option, a value that is not a
/// boolean, and options that are not a table are config errors.
#[test]
fn chain_expect_calls_options() {
    let mut rule = PreferChainedExpect::default();
    rule.configure(&serde_json::Value::Null).unwrap();
    check(rule, "ChainExpectCallsRector");
    for options in [
        serde_json::json!({ "merge_different_variables": false }),
        serde_json::json!({ "merge-different-variables": "no" }),
        serde_json::json!(false),
    ] {
        assert!(
            PreferChainedExpect::default().configure(&options).is_err(),
            "{options}"
        );
    }
}

#[test]
fn chain_expect_calls_and_use_to_be_empty() {
    run_class(
        vec![
            Box::new(PreferChainedExpect::default()),
            Box::new(PreferToBeEmpty),
        ],
        "ChainExpectCallsRectorAndUseToBeEmpty",
    );
}

/// The plugin's tests/Rules/Conflicts: the rule with others that change the same statements.
#[test]
fn chain_expect_calls_and_matchers() {
    run_class(
        vec![
            Box::new(PreferToHaveLength),
            Box::new(PreferToHaveCount),
            Box::new(PreferChainedExpect::default()),
        ],
        "ChainExpectCallsAndMatchers",
    );
}

#[test]
fn chain_expect_calls_and_use_each_modifier() {
    run_class(
        vec![
            Box::new(PreferChainedExpect::default()),
            Box::new(PreferEachModifier),
        ],
        "ChainExpectCallsAndUseEachModifier",
    );
}

#[test]
fn ensure_type_checks_first() {
    check(RequireTypeMatcherFirst, "EnsureTypeChecksFirstRector");
}

/// Broken/tag.php.inc: swapping a statement that ends in `?>` makes php-parser print the closure again, and Rector
/// writes the file shown there, which loses the `<?php` and does not parse. Pun's rewrite is the same, so the linter
/// refuses it instead of writing it.
#[test]
fn ensure_type_checks_first_refuses_rectors_broken_reprint() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/rector/EnsureTypeChecksFirstRector/Broken/tag.php.inc");
    let content = std::fs::read_to_string(&path).unwrap();
    let (input, _) = content.split_once("-----\n").unwrap();
    let linter = Linter::new(vec![Box::new(RequireTypeMatcherFirst)], 80400);
    let err = linter.fix(&path, input).unwrap_err();
    assert!(
        matches!(&err, pun_lint::LintError::FixBrokeSyntax { rules, .. } if rules == &["lint/pest/require-type-matcher-first"]),
        "{err:?}"
    );
}

#[test]
fn convert_assert_to_expect() {
    check(PreferExpect, "ConvertAssertToExpectRector");
}

/// The plugin's tests autoload its tests/Fixture classes, copied into Source/.
#[test]
fn uses_to_extend() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/rector/UsesToExtendRector/Source");
    let mut sources: Vec<(String, String)> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| {
            let path = e.unwrap().path();
            let code = std::fs::read_to_string(&path).unwrap();
            (path.to_string_lossy().into_owned(), code)
        })
        .collect();
    sources.sort();
    pun_lint::rules::facts::with_sources(sources, || check(PreferExtend, "UsesToExtendRector"));
}

/// The plugin's fixtures for this rule are projects: `<Project>/tests/Pest.php.fixture` becomes `tests/Pest.php` for
/// the run, as its test copies it, and each `.php.inc` is linted as the `.php` file beside it.
#[test]
fn remove_redundant_pest_uses() {
    let linter = Linter::new(vec![Box::new(NoRedundantUses)], 80400);
    let pipeline = laravel();
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/rector/RemoveRedundantPestUsesRector");
    let scratch = std::env::temp_dir().join(format!("pun-redundant-uses-{}", std::process::id()));
    let mut count = 0;
    for sub in ["Fixture", "Oracle", "Pun"] {
        for project in std::fs::read_dir(root.join(sub)).unwrap() {
            let project = project.unwrap().path();
            let copy = scratch.join(sub).join(project.file_name().unwrap());
            let mut stack = vec![project.clone()];
            while let Some(dir) = stack.pop() {
                for entry in std::fs::read_dir(&dir).unwrap() {
                    let path = entry.unwrap().path();
                    let rel = path.strip_prefix(&project).unwrap();
                    if path.is_dir() {
                        stack.push(path);
                        continue;
                    }
                    let name = path.file_name().unwrap().to_string_lossy().into_owned();
                    let target = copy.join(rel);
                    std::fs::create_dir_all(target.parent().unwrap()).unwrap();
                    if name == "Pest.php.fixture" {
                        std::fs::copy(&path, target.with_file_name("Pest.php")).unwrap();
                    }
                }
            }
            let mut files: Vec<PathBuf> = Vec::new();
            let mut stack = vec![project.clone()];
            while let Some(dir) = stack.pop() {
                for entry in std::fs::read_dir(&dir).unwrap() {
                    let path = entry.unwrap().path();
                    if path.is_dir() {
                        stack.push(path);
                    } else if path.to_string_lossy().ends_with(".php.inc") {
                        files.push(path);
                    }
                }
            }
            files.sort();
            for file in files {
                let content = std::fs::read_to_string(&file).unwrap();
                let (input, expected) = match content.split_once("-----\n") {
                    Some((i, e)) => (i.to_string(), e.to_string()),
                    None => (content.clone(), content),
                };
                let rel = file
                    .strip_prefix(&project)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();
                let path = copy.join(rel.trim_end_matches(".inc"));
                let name = format!(
                    "RemoveRedundantPestUsesRector {sub}/{}/{rel}",
                    project.file_name().unwrap().to_string_lossy()
                );
                check_fixture(
                    &linter,
                    &pipeline,
                    &name,
                    sub == "Fixture",
                    &path,
                    &input,
                    expected,
                );
                count += 1;
            }
        }
    }
    std::fs::remove_dir_all(&scratch).unwrap();
    assert_eq!(count, 34);
}

/// Every test directory of this file.
const DIRECTORIES: [&str; 62] = [
    "ChainExpectCallsAndMatchers",
    "ChainExpectCallsAndUseEachModifier",
    "ChainExpectCallsRector",
    "ChainExpectCallsRectorAndUseToBeEmpty",
    "ChainExpectCallsRectorWithoutMergeDifferentVariables",
    "ConvertAssertToExpectRector",
    "ConvertBeforeAllInDescribeRector",
    "ConvertExpectExceptionToThrowRector",
    "EnsureTypeChecksFirstRector",
    "FixInvalidRepeatValueRector",
    "RemoveOnlyRector",
    "RemoveRedundantLiteralTypeExpectationRector",
    "RemoveRedundantPestUsesRector",
    "RemoveStaticTestClosureRector",
    "SimplifyComparisonExpectationsRector",
    "SimplifyExpectNotRector",
    "SimplifyFilesystemMatchersRector",
    "SimplifyToBeTruthyFalsyRector",
    "SimplifyToLiteralBooleanRector",
    "TapToDeferRector",
    "ToBeTrueNotFalseRector",
    "ToHaveMethodOnClassRector",
    "UseEachModifierRector",
    "UseInstanceOfMatcherRector",
    "UseSequenceMatcherRector",
    "UseStrictEqualityMatchersRector",
    "UseToBeAlphaNumericRector",
    "UseToBeAlphaRector",
    "UseToBeBetweenRector",
    "UseToBeDigitsRector",
    "UseToBeDirectoryRector",
    "UseToBeEmptyRector",
    "UseToBeFileRector",
    "UseToBeInRector",
    "UseToBeInfiniteRector",
    "UseToBeJsonRector",
    "UseToBeListRector",
    "UseToBeLowercaseRector",
    "UseToBeNanRector",
    "UseToBeUppercaseRector",
    "UseToBeUrlRector",
    "UseToBeUuidRector",
    "UseToContainEqualRector",
    "UseToContainOnlyInstancesOfRector",
    "UseToContainRector",
    "UseToEndWithRector",
    "UseToEqualCanonicalizingRector",
    "UseToEqualWithDeltaRector",
    "UseToHaveCountRector",
    "UseToHaveKeyRector",
    "UseToHaveKeysRector",
    "UseToHaveLengthRector",
    "UseToHavePropertiesRector",
    "UseToHavePropertyRector",
    "UseToHaveSameSizeRector",
    "UseToMatchArrayRector",
    "UseToMatchObjectRector",
    "UseToMatchRector",
    "UseToStartWithRector",
    "UseToThrowRector",
    "UseTypeMatchersRector",
    "UsesToExtendRector",
];

/// A rule that the engine calls on every node, not only on its `targets()`.
struct Everywhere(Box<dyn Rule>);

impl Rule for Everywhere {
    fn id(&self) -> &'static str {
        self.0.id()
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &pun_lint::Context<'_>,
        out: &mut Vec<pun_lint::Report>,
    ) {
        self.0.visit(node, ancestors, ctx, out);
    }
}

/// The pest rules together over every input of this file's fixtures: called on every node, each rule reports what
/// it reports on its targets (`targets()` only spares the engine work), and none trips on another rule's code.
#[test]
fn pest_rules_ignore_nodes_outside_their_targets() {
    let pest = || {
        pun_lint::rules::all()
            .into_iter()
            .filter(|r| r.id().starts_with("lint/pest/"))
    };
    assert_eq!(pest().count(), 58);
    let targeted = Linter::new(pest().collect(), 80400);
    let everywhere = Linter::new(
        pest()
            .map(|r| Box::new(Everywhere(r)) as Box<dyn Rule>)
            .collect(),
        80400,
    );
    let mut stack: Vec<PathBuf> = DIRECTORIES
        .iter()
        .map(|d| {
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/rector")
                .join(d)
        })
        .collect();
    let mut count = 0;
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if !path.to_string_lossy().ends_with(".php.inc") {
                continue;
            }
            let content = std::fs::read_to_string(&path).unwrap();
            let input = content.split("-----\n").next().unwrap();
            assert_eq!(
                everywhere.check(&path, input).unwrap(),
                targeted.check(&path, input).unwrap(),
                "{}",
                path.display()
            );
            count += 1;
        }
    }
    assert!(count > 250, "{count}");
}
