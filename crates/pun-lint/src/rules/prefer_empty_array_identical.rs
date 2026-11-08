//! `lint/complexity/prefer-empty-array-identical`, ported from Rector 2.8.4's `SimplifyEmptyCheckOnEmptyArrayRector`
//! (rules/CodeQuality/Rector/Empty_), as it runs on PHPStan 2.3.5.
//!
//! `empty($array)` is `$array === []` and `!empty($array)` is `$array !== []` when `$array` is an array: a variable
//! whose type and native type are arrays, or an own property declared `array` (`syntax_types::is_array_property`).

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use mago_codex::ttype::TType;

use super::side_effect::any_node;
use super::syntax_types::is_array_property;
use super::types::facts::{Facts, facts};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferEmptyArrayIdentical;

impl Rule for PreferEmptyArrayIdentical {
    fn id(&self) -> &'static str {
        "lint/complexity/prefer-empty-array-identical"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::EmptyConstruct, NodeKind::UnaryPrefix])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (span, empty, negated) = match node {
            Node::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Not(_)) => {
                let Expression::Construct(Construct::Empty(empty)) = unparenthesize(u.operand)
                else {
                    return;
                };
                (node_span(u), empty, true)
            }
            Node::EmptyConstruct(empty) => {
                // `!empty(...)` is the `!`'s: Rector refactors the `BooleanNot` first.
                let outer = ancestors.iter().rposition(|n| {
                    !matches!(
                        n,
                        Node::Expression(_) | Node::Construct(_) | Node::Parenthesized(_)
                    )
                });
                if let Some(Node::UnaryPrefix(u)) = outer.map(|i| ancestors[i])
                    && matches!(u.operator, UnaryPrefixOperator::Not(_))
                {
                    return;
                }
                (node_span(empty), empty, false)
            }
            _ => return,
        };
        let value = unparenthesize(empty.value);
        if !is_allowed(ctx, ancestors, value) {
            return;
        }
        let s = node_span(value);
        let text = &ctx.source[s.start..s.end];
        let operator = if negated { "!==" } else { "===" };
        out.push(Report {
            span,
            message: "Compare the array with `[]` instead of using `empty()`.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, format!("{text} {operator} []"))],
            }),
        });
    }
}

/// `isAllowedExpr`: the value's type is an array, and it is a variable that `isAllowedVariable` passes or an own
/// property declared (or only ever assigned) an array.
fn is_allowed(ctx: &Context<'_>, ancestors: &[Node<'_, '_>], value: &Expression<'_>) -> bool {
    let facts = facts(ctx);
    let span = node_span(value);
    if !(facts.full_type(span)).is_some_and(|t| !t.types.is_empty() && t.is_array()) {
        return false;
    }
    match value {
        Expression::Variable(Variable::Direct(v)) => {
            !is_superglobal(v.name)
                && !is_non_typed_from_param(&facts, value)
                && !merged_in_loop(&facts, ancestors, v.name)
        }
        Expression::Access(Access::Property(_) | Access::StaticProperty(_)) => {
            is_array_property(ancestors, value, &|v, _| is_non_typed_from_param(&facts, v))
        }
        _ => false,
    }
}

/// `ExprAnalyzer::isNonTypedFromParam`: the variable's native type is `mixed`, may be undefined, is a union other
/// than its type, or does not cover its type (for an array type, is not an array).
pub(crate) fn is_non_typed_from_param(facts: &Facts, variable: &Expression<'_>) -> bool {
    let span = node_span(variable);
    let full = facts.full_type(span);
    facts.native_type(span).is_none_or(|native| {
        native.is_mixed()
            || native.possibly_undefined()
            || native.possibly_undefined_from_try()
            || (full.is_some_and(|t| t.is_array()) && !native.is_array())
            || (native.types.len() > 1 && full != Some(native))
    })
}

/// The rest of `isNonTypedFromParam`'s union check. A loop around the check that writes the variable merges its
/// values, which PHPStan keeps as a union of constant arrays (`array{}|array{array{frames: array{mixed}}}`) and
/// Mago does not carry back to the check. That union differs from the full type when a written value's native
/// type differs from its full type (the docblocks give it a more precise one).
fn merged_in_loop(facts: &Facts, ancestors: &[Node<'_, '_>], name: &[u8]) -> bool {
    ancestors
        .iter()
        .rev()
        .take_while(|n| !matches!(n, Node::Function(_) | Node::Method(_) | Node::Closure(_)))
        .any(|n| {
            matches!(
                n,
                Node::Foreach(_) | Node::For(_) | Node::While(_) | Node::DoWhile(_)
            ) && any_node(*n, &|m| {
                matches!(m, Node::Assignment(a) if root_is(a.lhs, name) && {
                    let span = node_span(a.rhs);
                    facts.native_type(span).map(|t| t.get_id())
                        != facts.full_type(span).map(|t| t.get_id())
                })
            })
        })
}

/// Whether `lhs` writes the variable `name` or an element of it.
fn root_is(lhs: &Expression<'_>, name: &[u8]) -> bool {
    match unparenthesize(lhs) {
        Expression::Variable(Variable::Direct(v)) => v.name == name,
        Expression::ArrayAccess(a) => root_is(a.array, name),
        Expression::ArrayAppend(a) => root_is(a.array, name),
        _ => false,
    }
}

/// `ReservedKeywordAnalyzer::isNativeVariable`.
fn is_superglobal(name: &[u8]) -> bool {
    matches!(
        name,
        b"$GLOBALS"
            | b"$_SERVER"
            | b"$_GET"
            | b"$_POST"
            | b"$_FILES"
            | b"$_COOKIE"
            | b"$_SESSION"
            | b"$_REQUEST"
            | b"$_ENV"
    )
}
