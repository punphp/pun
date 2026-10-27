//! Rector's own fixtures for the dead-code, phpdoc, complexity and control-flow rules of the quality batch, copied
//! verbatim from rector-src 2.6.1 (`.oracle/rector`, rules-tests/...) into tests/rector/<Rector class>/Fixture, with
//! the classes they use in Source/. Oracle/ holds extra inputs with what Rector 2.6.1 (`.oracle/rector-ecosystem`, the
//! rule alone, `phpVersion(80400)`) wrote for them; Pun/ holds the cases where Pun differs from Rector on purpose, its
//! expected output derived by hand, each with its reason. Each one must pass as
//! `format(pun lint --fix) == format(expected)` with format being the laravel preset, and the text edits must give the
//! expected bytes before format too.
//!
//! RemoveDeadIfBlockRector/Pun/colon_leftover_branches: Rector prints the kept `elseif` and `else` of an
//! alternative-syntax `if` it prints anew after a braced body (`} else: echo 2;`), which does not parse; Pun prints them
//! in braces.
//!
//! ThrowWithPreviousExceptionRector/Pun/caught_passed_positionally: where the constructor takes the previous exception
//! after the arguments, Rector passes the caught exception again even when an argument already is it, so its own
//! output changes on every run; Pun leaves a `new` with the caught exception as a positional argument alone, so that
//! its fix rounds settle.
//!
//! RemoveFilterVarOnExactTypeRector/Pun/moved_operation_parentheses,
//! RemoveDeadZeroAndOneOperationRector/Pun/kept_operation_parentheses and RecastingRemovalRector/Pun/operand_parentheses:
//! Rector prints the value it keeps without the
//! parentheses the new place needs (`filter_var($i + 1, FILTER_VALIDATE_INT) * 2` becomes `$i + 1 * 2`); Pun keeps
//! the meaning (lint.md: meaning over bytes).
//!
//! RecastingRemovalRector/FixturePhp74 runs at PHP 7.4, below Pun's floor, and is not copied.

mod lint_c;

use lint_c::run_class;
use pun_lint::Rule;
use pun_lint::rules::{
    NoConstantCondition, NoUnusedTestMethodParam, NoUselessCast, NoUselessFilterVar, NoUselessMath,
    PreferInstanceMethod, RequirePreviousException,
};

fn check(rule: impl Rule + 'static, class: &str) {
    run_class(vec![Box::new(rule)], class);
}

#[test]
fn remove_dead_if_block() {
    check(NoConstantCondition, "RemoveDeadIfBlockRector");
}

#[test]
fn locally_called_static_method_to_non_static() {
    check(
        PreferInstanceMethod,
        "LocallyCalledStaticMethodToNonStaticRector",
    );
}

#[test]
fn remove_tests_overridden_private_method_parameter() {
    check(
        NoUnusedTestMethodParam,
        "RemoveTestsOverriddenPrivateMethodParameterRector",
    );
}

#[test]
fn throw_with_previous_exception() {
    check(RequirePreviousException, "ThrowWithPreviousExceptionRector");
}

#[test]
fn remove_filter_var_on_exact_type() {
    check(NoUselessFilterVar, "RemoveFilterVarOnExactTypeRector");
}

#[test]
fn remove_dead_zero_and_one_operation() {
    check(NoUselessMath, "RemoveDeadZeroAndOneOperationRector");
}

#[test]
fn recasting_removal() {
    check(NoUselessCast, "RecastingRemovalRector");
}
