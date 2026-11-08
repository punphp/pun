//! `lint/dead-code/no-always-true-if`, ported from Rector 2.8.4's `RemoveAlwaysTrueIfConditionRector`
//! (rules/DeadCode/Rector/If_).
//!
//! Nothing changes in an `if` with `elseif` or `else`, nor in a file with inline HTML. An `if (a && b)` whose left side
//! PHPStan types as `true` keeps only `b` as its condition (with the checks of
//! [`super::no_always_false_condition`]). An `if` whose condition's native type is `true`
//! makes way for its statements, the `if`'s comments before the first; with no statement it goes. The condition must
//! hold no property, array item, method or static call, assignment, call of a function that asks the runtime
//! (`class_exists()`, `function_exists()`, `defined()`, ...), or variable that may be undefined or that
//! `isNonTypedFromParam` doubts; nothing
//! changes in a trait.
//!
//! php-parser's printer decides the bytes: see [`super::stmt_splice`].

use mago_codex::ttype::atomic::TAtomic;
use mago_syntax::cst::{
    Access, BinaryOperator, Call, Expression, Node, NodeKind, Statement, Variable,
};
use pun_problem::{Edit, Fix, Safety};

use super::class_method::walk;
use super::no_always_false_condition::is_safe;
use super::php_parser::{Token, previous_token_end, tokens};
use super::prefer_empty_array_identical::is_non_typed_from_param;
use super::rector_batch5::called_function_name;
use super::rector_conditions::{comments_between, if_parts};
use super::side_effect::any_node;
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
        let tokens = tokens(ctx);
        let facts = facts(ctx);
        let edits = run_edits(source, tokens, ancestors, &|chain| {
            change(source, tokens, &facts, chain)
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
    let parts = if_parts(tokens, r#if);
    if parts.has_else || parts.has_elseifs {
        return None;
    }
    // `File::containsHTML()`: the file has inline HTML (php-parser's `?>` takes the line break after it)
    if any_node(chain[0], &|n| match n {
        Node::Inline(inline) | Node::Statement(Statement::Inline(inline)) => {
            let value = inline.value;
            let value = value
                .strip_prefix(b"\r\n".as_slice())
                .or_else(|| value.strip_prefix(b"\n".as_slice()))
                .unwrap_or(value);
            !value.is_empty()
        }
        _ => false,
    }) {
        return None;
    }
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
    if !facts.native_type(node_span(cond)).is_some_and(is_true)
        || should_skip_expr(chain, cond)
        || should_skip_from_variable(facts, cond)
        || writes_variable_variable(chain, node_span(r#if).start)
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

/// `shouldSkipExpr`: a property, an array item, a method or static call, or a call of a runtime state function;
/// here also a call of a built-in or dynamic function.
fn should_skip_expr(ancestors: &[Node<'_, '_>], expr: &Expression<'_>) -> bool {
    any_node(Node::Expression(expr), &|n| match n {
        Node::Expression(
            Expression::Access(Access::Property(_) | Access::StaticProperty(_))
            | Expression::ArrayAccess(_)
            | Expression::Call(Call::Method(_) | Call::StaticMethod(_)),
        ) => true,
        // PHPStan's native scope does not narrow a call of a built-in function (`is_int($int)` is `bool` there,
        // where Mago types it `true`), so a condition with one (or with a dynamic call) is never natively `true`
        Node::Expression(Expression::Call(Call::Function(f))) => {
            called_function_name(ancestors, f.function).is_none_or(|n| {
                RUNTIME_STATE_FUNCTIONS
                    .iter()
                    .any(|r| n.eq_ignore_ascii_case(r))
                    || is_builtin_function(n.trim_start_matches('\\').as_bytes())
            })
        }
        _ => false,
    })
}

/// `shouldSkipFromVariable`: a variable `isNonTypedFromParam` doubts. This also covers Rector's `hasAssign` check:
/// Mago's analysis leaves an assignment's target variable untyped, and a target that is no variable is a property or
/// an array item, which [`should_skip_expr`] skips. Rector skips a variable whose native type is an intersection with
/// an array too, but its `NodeTypeCorrector` first makes a non-empty array a plain one and drops a list's accessory, so
/// none of the refined arrays PHPStan's native scope gives (`non-empty-array`, `non-empty-array&hasOffset('k')`,
/// `list`) is one by then.
fn should_skip_from_variable(facts: &Facts, expr: &Expression<'_>) -> bool {
    any_node(Node::Expression(expr), &|n| {
        matches!(n, Node::Expression(e @ Expression::Variable(_))
            // a variable that may be undefined (`! $scope->hasVariableType()->yes()`)
            if is_non_typed_from_param(facts, e)
                || facts.native_type(node_span(e)).is_some_and(mago_codex::ttype::union::TUnion::possibly_undefined))
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
