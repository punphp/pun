//! Rector's own fixtures for the dead-code, phpdoc, complexity and control-flow rules of the quality batch, copied
//! verbatim from rector-src 2.8.4 (`.oracle/rector`, rules-tests/...) into tests/rector/<Rector class>/Fixture, with
//! the classes they use in Source/. Oracle/ holds extra inputs with what Rector 2.8.4 (`.oracle/rector-ecosystem`, the
//! rule alone, `phpVersion(80400)`) wrote for them; Pun/ holds the cases where Pun differs from Rector on purpose, its
//! expected output derived by hand, each with its reason. Each one must pass as
//! `format(pun lint --fix) == format(expected)` with format being the laravel preset, and the text edits must give the
//! expected bytes before format too.
//!
//! RemoveDeadIfBlockRector/Pun/colon_leftover_branches: Rector prints the kept `elseif` and `else` of an
//! alternative-syntax `if` it prints anew after a braced body (`} else: echo 2;`), which does not parse; Pun prints them
//! in braces.
//!
//! RemoveFilterVarOnExactTypeRector/Pun/moved_operation_parentheses,
//! RemoveDeadZeroAndOneOperationRector/Pun/kept_operation_parentheses and RecastingRemovalRector/Pun/operand_parentheses:
//! Rector prints the value it keeps without the
//! parentheses the new place needs (`filter_var($i + 1, FILTER_VALIDATE_INT) * 2` becomes `$i + 1 * 2`); Pun keeps
//! the meaning (lint.md: meaning over bytes).
//!
//! RemoveDeadInstanceOfRector/Pun/unreachable_code, RemoveDeadInstanceOfAssertRector/Pun/unreachable_code and
//! RemoveTestsOverriddenPrivateMethodParameterRector/Pun/unreachable_caller: Rector types the statements after a
//! `return` with the scope before it; Mago's analysis does not reach them, so Pun leaves the `if` and the `assert()`
//! there, and the caller's argument (the parameter still goes).
//!
//! RemoveDeadInstanceOfRector/Pun/bare_body_holder: where the braceless body of an `if` is an `if` that gives way to
//! several statements, Rector prints the outer `if` anew with braces; Pun leaves both.
//!
//! RemoveAlwaysTrueIfConditionRector/Pun/self_comparison: PHPStan types `$n === $n` as `true`; Mago's analysis gives
//! `bool`, so Pun keeps the `if`.
//!
//! ExplicitReturnNullRector/Pun/generic_return_doc: Rector's type mapper prints a `@return` union it rewrites anew
//! (`array<int, string>|void` becomes `string[]|null`, `parent|void` becomes `static|null`, another type, and `self`
//! in an anonymous class becomes a name of Rector's own, `AnonymousClass6c52...`, no class); Pun prints keywords and
//! class names as Rector does and leaves a function with any other kind of type in that union alone.
//!
//! SimplifyIfReturnBoolRector/Pun/operand_parentheses: where an operand php-parser prints with its own code needs
//! parentheses in the returned value (a ternary under the new `(bool)` or in an `&&` printed anew), Rector writes
//! `(bool) $a ?: $b`, which means something else, or reprints the ternary; Pun leaves the `if` alone.
//!
//! RemoveDeadInstanceOfRector/Source/vendor/{Node,If_}.php are not upstream: they declare the nikic/php-parser 5
//! types a fixture names, which Rector's run loads from its vendor directory.
//!
//! InlineArrayReturnAssignRector/Pun/keys_rector_misreads: Rector writes a key whose value its `ValueResolver` misreads
//! as that misreading: a global or magic constant as its name (`'TOP' => 1`), an enum case or an unknown class
//! constant as `'Suit::Hearts'`, a heredoc key with the wrong value, and it crashes on a nested key under a scalar
//! value; Pun leaves those functions alone.
//!
//! RecastingRemovalRector/FixturePhp74 runs at PHP 7.4, below Pun's floor, and is not copied. Neither is
//! InlineArrayReturnAssignRector/Fixture/with_first_class_callable: `new SomeAssignedObject(...)` is not PHP, and
//! Mago's parser rejects the file.

mod lint_c;

use std::path::Path;

use lint_c::run_class;
use mago_syntax::cst::Node;
use pun_lint::rules::{
    AddPropertyTypeAssigns, AddPropertyTypeConstructor, NoAlwaysFalseCondition, NoAlwaysTrueIf,
    NoAlwaysTrueInstanceof, NoAlwaysTrueInstanceofAssert, NoConstantCondition,
    NoConstantViaVariable, NoInvalidVarTag, NoPhpdocClassCaseMismatch, NoRedundantForeachGuard,
    NoRedundantPropertyDefault, NoTypedPropertyInstanceof, NoUnusedTestMethodParam, NoUselessCast,
    NoUselessFilterVar, NoUselessMath, PreferInlineArrayReturn, PreferInstanceMethod,
    PreferPropertyExists, PreferReturnedCondition, RequireConsistentReturn,
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
fn simplify_if_return_bool() {
    check(PreferReturnedCondition, "SimplifyIfReturnBoolRector");
}

#[test]
fn remove_non_existing_var_annotation() {
    check(NoInvalidVarTag, "RemoveNonExistingVarAnnotationRector");
}

#[test]
fn inline_array_return_assign() {
    check(PreferInlineArrayReturn, "InlineArrayReturnAssignRector");
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
fn remove_default_value_from_assigned_property_on_every_node() {
    check(
        Everywhere(Box::new(NoRedundantPropertyDefault)),
        "RemoveDefaultValueFromAssignedPropertyRector",
    );
}

#[test]
fn typed_property_from_assigns_on_every_node() {
    lint_c::run_class_stubs(
        vec![Box::new(Everywhere(Box::new(
            AddPropertyTypeAssigns::default(),
        )))],
        "TypedPropertyFromAssignsRector",
        70400,
        &["php-parser-stubs", "phpunit-stubs"],
    );
}

#[test]
fn typed_property_from_strict_constructor_on_every_node() {
    lint_c::run_class_stubs(
        vec![Box::new(Everywhere(Box::new(AddPropertyTypeConstructor)))],
        "TypedPropertyFromStrictConstructorRector",
        80100,
        &["doctrine-stubs"],
    );
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
            || Box::new(PreferReturnedCondition),
            "SimplifyIfReturnBoolRector",
        ),
        (
            || Box::new(NoInvalidVarTag),
            "RemoveNonExistingVarAnnotationRector",
        ),
        (
            || Box::new(PreferInlineArrayReturn),
            "InlineArrayReturnAssignRector",
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
