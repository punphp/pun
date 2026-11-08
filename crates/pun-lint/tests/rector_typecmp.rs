//! Rector's own fixtures for the rules built on Rector's type comparison (`TypeComparator`, `StaticTypeMapper`) and
//! for `RemoveUnusedVariableAssignRector`,
//! copied verbatim from rector-src 2.8.4 (`.oracle/rector`, rules-tests/...) into tests/rector/<Rector class>/Fixture,
//! with the classes they use in Source/. Oracle/ holds extra inputs with what Rector 2.8.4 (`.oracle/rector-ecosystem`,
//! the rule alone, `phpVersion(80400)` unless the test says otherwise) wrote for them. Each one must pass as
//! `format(pun lint --fix) == format(expected)` with format being the laravel preset, and the text edits must give the
//! expected bytes before format too.
//!
//! RemoveUnusedVariableAssignRector/Pun/statements.php.inc: Rector 2.8.4 reruns the rule on the same AST until it
//! stops changing, and keeps `$ref = 2;` after removing the closure that took `$ref` by reference, as the variable keeps
//! the `IS_BYREF_VAR` attribute set when the file was parsed; Pun's next fix round parses the new source, finds no
//! reference and removes the assignment.

mod lint_c;

use lint_c::{run_class, run_class_with};
use pun_lint::Rule;
use pun_lint::rules::{
    NoInvalidReturnTag, NoRedundantNullableCheck, NoUnusedAssignment, NoUselessParamTag,
    NoUselessPhpdocType, NoUselessReturnTag, NoUselessVarTag,
};

fn check(rule: impl Rule + 'static, class: &str) {
    run_class(vec![Box::new(rule)], class);
}

#[test]
fn remove_useless_var_tag() {
    // the laravel preset's `no_superfluous_phpdoc_tags` and `phpdoc_trim` remove the same tags in these fixtures
    run_class_with(
        vec![Box::new(NoUselessVarTag)],
        "RemoveUselessVarTagRector",
        80400,
        &[
            "Fixture/remove_equal_type.php.inc",
            "Fixture/some_class.php.inc",
        ],
    );
}

#[test]
fn remove_useless_param_tag() {
    // the laravel preset's `no_superfluous_phpdoc_tags` and `phpdoc_trim` remove the same tags in these fixtures
    run_class_with(
        vec![Box::new(NoUselessParamTag)],
        "RemoveUselessParamTagRector",
        80400,
        &[
            "Fixture/do_not_duplicate_annotations.php.inc",
            "Fixture/fixture.php.inc",
            "Fixture/fixture_fqcn_docblock.php.inc",
            "Fixture/fixture_fqcn_param_return.php.inc",
            "Fixture/fixture_namespaced_arg_return_type.php.inc",
            "Fixture/fixture_namespaced_param_return.php.inc",
            "Fixture/fixture_param_classname.php.inc",
            "Fixture/on_function.php.inc",
            "Fixture/remove_nullable_type.php.inc",
            "Fixture/same_namespace_param_type.php.inc",
            "Fixture/same_namespace_param_type_deep.php.inc",
            "Fixture/union_unsorted.php.inc",
            "Oracle/function_likes.php.inc",
            "Oracle/mixed_params.php.inc",
        ],
    );
}

#[test]
fn remove_useless_return_tag() {
    // the laravel preset's `no_superfluous_phpdoc_tags` and `phpdoc_trim` remove the same tags in these fixtures
    run_class_with(
        vec![Box::new(NoUselessReturnTag)],
        "RemoveUselessReturnTagRector",
        80400,
        &[
            "Fixture/fixture.php.inc",
            "Fixture/fixture_fqcn_param_return.php.inc",
            "Fixture/fixture_namespaced_arg_return_type.php.inc",
            "Fixture/fixture_namespaced_param_return.php.inc",
            "Fixture/fixture_return_bool.php.inc",
            "Fixture/on_function.php.inc",
            "Fixture/phpstan-return.php.inc",
            "Fixture/psalm-return.php.inc",
            "Fixture/remove_null.php.inc",
            "Fixture/remove_return_mixed2.php.inc",
            "Fixture/remove_return_nullable.php.inc",
            "Fixture/return_void_on_void.php.inc",
            "Oracle/function_likes.php.inc",
            "Oracle/mixed_returns.php.inc",
        ],
    );
}

#[test]
fn remove_return_tag_incompatible_with_native_type() {
    check(
        NoInvalidReturnTag,
        "RemoveReturnTagIncompatibleWithNativeTypeRector",
    );
}

#[test]
fn remove_useless_union_return_docblock() {
    // the fixtures' config sets `phpVersion(PhpVersionFeature::UNION_TYPES)`
    // the laravel preset's `no_superfluous_phpdoc_tags` removes the same tags in these
    run_class_with(
        vec![Box::new(NoUselessPhpdocType)],
        "RemoveUselessUnionReturnDocblockRector",
        80000,
        &[
            "Oracle/function_likes.php.inc",
            "Oracle/mixed_unions.php.inc",
        ],
    );
}

#[test]
fn simplify_if_nullable_return() {
    check(NoRedundantNullableCheck, "SimplifyIfNullableReturnRector");
}

#[test]
fn remove_unused_variable_assign() {
    check(NoUnusedAssignment, "RemoveUnusedVariableAssignRector");
}
