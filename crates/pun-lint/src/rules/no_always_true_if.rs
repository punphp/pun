//! `lint/dead-code/no-always-true-if`, ported from Rector 2.6.1's `RemoveAlwaysTrueIfConditionRector`
//! (rules/DeadCode/Rector/If_).
//!
//! An `if (a && b)` whose left side PHPStan types as `true` keeps only `b` as its condition (with the checks of
//! [`super::no_always_false_condition`]). An `if` without `elseif` or `else` whose condition's native type is `true`
//! makes way for its statements, the `if`'s comments before the first; with no statement it goes. The condition must
//! hold no property, array item, method or static call, assignment, call of a function that asks the runtime
//! (`class_exists()`, `function_exists()`, `defined()`, ...), or variable that `isNonTypedFromParam` doubts or whose
//! native type is a refined array; nothing changes in a trait.
//!
//! php-parser's printer decides the bytes: see [`super::stmt_splice`].

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_syntax::cst::{
    Access, AssignmentOperator, BinaryOperator, Call, Expression, Node, NodeKind, Statement,
    Variable,
};
use pun_problem::{Edit, Fix, Safety};

use super::class_method::walk;
use super::no_always_false_condition::is_safe;
use super::php_parser::{Token, previous_token_end, tokens};
use super::prefer_empty_array_identical::is_non_typed_from_param;
use super::rector_batch5::called_function_name;
use super::rector_conditions::{comments_between, if_parts};
use super::stmt_splice::{Change, NewStatement, run_edits};
use super::types::facts::{Facts, facts, is_builtin_function};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoAlwaysTrueIf;

/// `RUNTIME_STATE_FUNCTIONS`.
const RUNTIME_STATE_FUNCTIONS: [&str; 9] = [
    "class_exists",
    "interface_exists",
    "trait_exists",
    "enum_exists",
    "function_exists",
    "method_exists",
    "property_exists",
    "defined",
    "extension_loaded",
];

impl Rule for NoAlwaysTrueIf {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-always-true-if"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::If])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        if !matches!(node, Node::If(_)) {
            return;
        }
        let source = ctx.source;
        let tokens = tokens(source);
        let facts = facts(ctx);
        let edits = run_edits(source, &tokens, ancestors, &|chain| {
            change(source, &tokens, &facts, chain)
        });
        if let Some(edits) = edits {
            out.push(Report {
                span: node_span(&ancestors[ancestors.len() - 1]),
                message: "This condition is always true.".to_string(),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Safe,
                    edits,
                }),
            });
        }
    }
}

/// What `refactor()` does to the `if` statement `chain` ends with.
fn change(source: &str, tokens: &[Token], facts: &Facts, chain: &[Node<'_, '_>]) -> Option<Change> {
    let Node::Statement(Statement::If(r#if)) = chain[chain.len() - 1] else {
        return None;
    };
    let cond = unparenthesize(r#if.condition);
    if let Expression::Binary(and) = cond
        && matches!(and.operator, BinaryOperator::And(_))
    {
        // `refactorIfWithBooleanAnd`
        let left = unparenthesize(and.lhs);
        let is_true = facts.full_type(node_span(left)).is_some_and(is_true);
        if !is_true || !is_safe(facts, chain, left) {
            return None;
        }
        let right = node_span(unparenthesize(and.rhs));
        let edit = Edit::new(node_span(cond), &source[right.start..right.end]);
        return Some(Change::Edits(vec![edit]));
    }
    let parts = if_parts(tokens, r#if);
    if parts.has_else || parts.has_elseifs {
        return None;
    }
    if !facts.native_type(node_span(cond)).is_some_and(is_true)
        || should_skip_expr(chain, cond)
        || should_skip_from_variable(facts, cond)
        || writes_variable_variable(chain, node_span(r#if).start)
        || has_assign(cond)
        || chain.iter().any(|a| matches!(a, Node::Trait(_)))
    {
        return None;
    }
    if parts.stmts.is_empty() {
        return Some(Change::Remove);
    }
    let mut new = NewStatement::all(source, tokens, &parts.stmts);
    // keep original comments
    let start = node_span(r#if).start;
    let mut comments = comments_between(tokens, previous_token_end(source, tokens, start), start);
    if !comments.is_empty() {
        comments.append(&mut new[0].comments);
        new[0].comments = comments;
    }
    Some(Change::Replace(new))
}

/// `isTrue()->yes()`.
fn is_true(t: &mago_codex::ttype::union::TUnion) -> bool {
    matches!(&t.types[..], [TAtomic::Scalar(s)] if s.is_true())
}

/// Whether some expression in `expr` passes `test`.
fn any_expression(expr: &Expression<'_>, test: &dyn Fn(&Expression<'_>) -> bool) -> bool {
    let mut found = false;
    walk(Node::Expression(expr), &mut |n, _| {
        if let Node::Expression(e) = n {
            found |= test(e);
        }
        !found
    });
    found
}

/// `shouldSkipExpr`: a property, an array item, a method or static call, or a call of a runtime state function;
/// here also a call of a built-in or dynamic function.
fn should_skip_expr(ancestors: &[Node<'_, '_>], expr: &Expression<'_>) -> bool {
    any_expression(expr, &|e| match e {
        Expression::Access(Access::Property(_) | Access::StaticProperty(_))
        | Expression::ArrayAccess(_)
        | Expression::Call(Call::Method(_) | Call::StaticMethod(_)) => true,
        // PHPStan's native scope does not narrow a call of a built-in function (`is_int($int)` is `bool` there,
        // where Mago types it `true`), so a condition with one (or with a dynamic call) is never natively `true`
        Expression::Call(Call::Function(f)) => called_function_name(ancestors, f.function)
            .is_none_or(|n| {
                RUNTIME_STATE_FUNCTIONS
                    .iter()
                    .any(|r| n.eq_ignore_ascii_case(r))
                    || is_builtin_function(n.trim_start_matches('\\').as_bytes())
            }),
        _ => false,
    })
}

/// `shouldSkipFromVariable`: a variable `isNonTypedFromParam` doubts, or whose native type is an intersection with an
/// array (a refined array: a non-empty one, or one with a known key, of no constant value).
fn should_skip_from_variable(facts: &Facts, expr: &Expression<'_>) -> bool {
    any_expression(expr, &|e| {
        matches!(e, Expression::Variable(_))
            && (is_non_typed_from_param(facts, e)
                || facts.native_type(node_span(e)).is_some_and(|t| {
                    t.types.iter().any(|a| match a {
                        TAtomic::Array(TArray::Keyed(k)) => {
                            k.parameters.is_some() && (k.non_empty || k.known_items.is_some())
                        }
                        TAtomic::Array(TArray::List(l)) => {
                            !l.element_type.is_never()
                                && (l.non_empty || l.known_elements.is_some())
                        }
                        _ => false,
                    })
                }))
    })
}

/// Whether the function-like around the `if` (before `at`) assigns a variable variable (`${$name} = ...`): PHPStan
/// types the variable that names, which Mago's analysis does not, so the types of the condition's variables are not
/// Mago's.
fn writes_variable_variable(ancestors: &[Node<'_, '_>], at: usize) -> bool {
    let Some(root) = ancestors.iter().rev().find(|a| {
        matches!(
            a,
            Node::Function(_) | Node::Method(_) | Node::Closure(_) | Node::ArrowFunction(_)
        )
    }) else {
        return false;
    };
    let mut found = false;
    walk(*root, &mut |n, path| {
        if found
            || (!path.is_empty()
                && matches!(
                    n,
                    Node::Function(_) | Node::Closure(_) | Node::ArrowFunction(_)
                ))
        {
            return false;
        }
        if let Node::Assignment(a) = n
            && node_span(a).start < at
            && matches!(
                unparenthesize(a.lhs),
                Expression::Variable(Variable::Indirect(_) | Variable::Nested(_))
            )
        {
            found = true;
        }
        !found
    });
    found
}

/// `findFirstInstanceOf($cond, Assign::class)`: a plain `=` (not by reference).
fn has_assign(expr: &Expression<'_>) -> bool {
    any_expression(expr, &|e| {
        matches!(e, Expression::Assignment(a) if matches!(a.operator, AssignmentOperator::Assign(_))
            && !matches!(a.rhs, Expression::UnaryPrefix(u) if u.operator.is_reference()))
    })
}
