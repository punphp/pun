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
//! RemoveDeadInstanceOfRector/Pun/unreachable_code and RemoveTestsOverriddenPrivateMethodParameterRector/Pun/
//! unreachable_caller: Rector types the statements after a `return` with the scope before it; Mago's analysis does not
//! reach them, so Pun leaves the `if` there, and the caller's argument (the parameter still goes).
//!
//! RemoveDeadInstanceOfRector/Pun/bare_body_holder: where the braceless body of an `if` is an `if` that gives way to
//! several statements, Rector prints the outer `if` anew with braces; Pun leaves both.
//!
//! RemoveAlwaysTrueIfConditionRector/Pun/self_comparison: PHPStan types `$n === $n` as `true`; Mago's analysis gives
//! `bool`, so Pun keeps the `if`.
//!
//! ExplicitReturnNullRector/Pun/generic_return_doc: Rector's type mapper prints a `@return` union it rewrites anew
//! (`array<int, string>|void` becomes `string[]|null`); Pun prints keywords and class names as Rector does and leaves
//! a function with any other kind of type in that union alone.
//!
//! RemoveDeadInstanceOfRector/Source/vendor/{Node,If_}.php are not upstream: they declare the nikic/php-parser 5
//! types a fixture names, which Rector's run loads from its vendor directory.
//!
//! RecastingRemovalRector/FixturePhp74 runs at PHP 7.4, below Pun's floor, and is not copied.

mod lint_c;

use std::path::Path;

use lint_c::run_class;
use mago_syntax::cst::Node;
use pun_lint::rules::{
    NoAlwaysFalseCondition, NoAlwaysTrueIf, NoAlwaysTrueInstanceof, NoAlwaysTrueInstanceofAssert,
    NoConstantCondition, NoConstantViaVariable, NoPhpdocClassCaseMismatch, NoRedundantForeachGuard,
    NoTypedPropertyInstanceof, NoUnusedTestMethodParam, NoUselessCast, NoUselessFilterVar,
    NoUselessMath, PreferInstanceMethod, PreferPropertyExists, RequireConsistentReturn,
    RequirePreviousException,
};
use pun_lint::{Linter, Rule};

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

#[test]
fn remove_dead_instance_of() {
    check(NoAlwaysTrueInstanceof, "RemoveDeadInstanceOfRector");
}

#[test]
fn remove_typed_property_dead_instance_of() {
    check(
        NoTypedPropertyInstanceof,
        "RemoveTypedPropertyDeadInstanceOfRector",
    );
}

#[test]
fn remove_unused_non_empty_array_before_foreach() {
    check(
        NoRedundantForeachGuard,
        "RemoveUnusedNonEmptyArrayBeforeForeachRector",
    );
}

#[test]
fn reduce_always_false_if_or() {
    check(NoAlwaysFalseCondition, "ReduceAlwaysFalseIfOrRector");
}

#[test]
fn remove_always_true_if_condition() {
    check(NoAlwaysTrueIf, "RemoveAlwaysTrueIfConditionRector");
}

#[test]
fn remove_dead_instance_of_assert() {
    check(
        NoAlwaysTrueInstanceofAssert,
        "RemoveDeadInstanceOfAssertRector",
    );
}

#[test]
fn variable_const_fetch_to_class_const_fetch() {
    check(
        NoConstantViaVariable,
        "VariableConstFetchToClassConstFetchRector",
    );
}

#[test]
fn explicit_return_null() {
    check(RequireConsistentReturn, "ExplicitReturnNullRector");
}

#[test]
fn isset_on_property_object_to_property_exists() {
    check(
        PreferPropertyExists,
        "IssetOnPropertyObjectToPropertyExistsRector",
    );
}

#[test]
fn fix_class_case_sensitivity_var_docblock() {
    check(
        NoPhpdocClassCaseMismatch,
        "FixClassCaseSensitivityVarDocblockRector",
    );
}

/// A rule called on every node, not only the kinds of its `targets()`.
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

#[test]
fn rules_ignore_nodes_outside_their_targets() {
    // `targets()` only spares the engine work: called on every node, each rule reports what it reports on its own.
    let cases: Vec<(fn() -> Box<dyn Rule>, &str)> = vec![
        (|| Box::new(NoConstantCondition), "RemoveDeadIfBlockRector"),
        (
            || Box::new(PreferInstanceMethod),
            "LocallyCalledStaticMethodToNonStaticRector",
        ),
        (
            || Box::new(NoUnusedTestMethodParam),
            "RemoveTestsOverriddenPrivateMethodParameterRector",
        ),
        (
            || Box::new(RequirePreviousException),
            "ThrowWithPreviousExceptionRector",
        ),
        (
            || Box::new(NoUselessFilterVar),
            "RemoveFilterVarOnExactTypeRector",
        ),
        (
            || Box::new(NoUselessMath),
            "RemoveDeadZeroAndOneOperationRector",
        ),
        (|| Box::new(NoUselessCast), "RecastingRemovalRector"),
        (
            || Box::new(NoAlwaysTrueInstanceof),
            "RemoveDeadInstanceOfRector",
        ),
        (
            || Box::new(NoTypedPropertyInstanceof),
            "RemoveTypedPropertyDeadInstanceOfRector",
        ),
        (
            || Box::new(NoRedundantForeachGuard),
            "RemoveUnusedNonEmptyArrayBeforeForeachRector",
        ),
        (
            || Box::new(NoAlwaysFalseCondition),
            "ReduceAlwaysFalseIfOrRector",
        ),
        (
            || Box::new(NoAlwaysTrueIf),
            "RemoveAlwaysTrueIfConditionRector",
        ),
        (
            || Box::new(NoConstantViaVariable),
            "VariableConstFetchToClassConstFetchRector",
        ),
        (
            || Box::new(RequireConsistentReturn),
            "ExplicitReturnNullRector",
        ),
        (
            || Box::new(NoPhpdocClassCaseMismatch),
            "FixClassCaseSensitivityVarDocblockRector",
        ),
        (
            || Box::new(PreferPropertyExists),
            "IssetOnPropertyObjectToPropertyExistsRector",
        ),
        (
            || Box::new(NoAlwaysTrueInstanceofAssert),
            "RemoveDeadInstanceOfAssertRector",
        ),
    ];
    for (rule, class) in cases {
        let targeted = Linter::new(vec![rule()], 80400);
        let everywhere = Linter::new(vec![Box::new(Everywhere(rule()))], 80400);
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/rector")
            .join(class);
        let mut files = Vec::new();
        for dir in ["Fixture", "Oracle", "Pun"] {
            if let Ok(entries) = std::fs::read_dir(root.join(dir)) {
                files.extend(entries.map(|e| e.unwrap().path()));
            }
        }
        assert!(!files.is_empty(), "{class}");
        for path in files {
            let content = std::fs::read_to_string(&path).unwrap();
            let input = content.split("-----\n").next().unwrap();
            assert_eq!(
                everywhere.check(&path, input).unwrap(),
                targeted.check(&path, input).unwrap(),
                "{}",
                path.display()
            );
        }
    }
}
