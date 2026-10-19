//! `lint/dead-code/no-overwritten-assignment`, ported from Rector 2.6.1's `RemoveDoubleAssignRector`
//! (rules/DeadCode/Rector/Assign).
//!
//! An assignment the next statement assigns again to the same variable or property goes: `$value = 1; $value = 2;`
//! is `$value = 2;`. It stays when the next value reads the target, when the first value is a call with side effects
//! (`array_pop()`), when the target holds such a call, or, in a `try`, when the next value is one.

use mago_syntax::cst::*;
use pun_problem::{Fix, Safety};

use super::rector_batch5::remove_statements;
use super::rector_stmts::{HOLDERS, assign, expression, holder_list};
use super::side_effect::{any_node, call_has_side_effect};
use crate::syntax::{node_span, same_expression, same_node, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoOverwrittenAssignment;

impl Rule for NoOverwrittenAssignment {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-overwritten-assignment"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(HOLDERS)
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let raw = crate::syntax::statements(node);
        if !raw.windows(2).any(|w| {
            matches!(
                (w[0], w[1]),
                (Statement::Expression(_), Statement::Expression(_))
            )
        }) {
            return;
        }
        let source = ctx.source;
        let Some(list) = holder_list(source, node, ancestors) else {
            return;
        };
        let mut scope = ancestors.to_vec();
        scope.push(node);
        // php-parser's `TryCatch` holds the statements of its `try` block.
        let in_try = matches!(ancestors.last(), Some(Node::Try(t)) if matches!(node, Node::Block(b) if std::ptr::eq(b, &t.block)));
        let side_effect =
            |expr: &Expression<'_>| call_has_side_effect(&scope, unparenthesize(expr));
        let mut removed = Vec::new();
        for (key, pair) in list.stmts.windows(2).enumerate() {
            let (Some(first), Some(next)) = (
                expression(pair[0]).and_then(assign),
                expression(pair[1]).and_then(assign),
            ) else {
                continue;
            };
            if !same_expression(source, next.lhs, first.lhs) {
                continue;
            }
            // isSelfReferencing: the next value reads the target.
            if any_node(Node::Expression(next.rhs), &|n| {
                matches!(n, Node::Expression(_)) && same_node(source, Node::Expression(next.lhs), n)
            }) {
                continue;
            }
            if side_effect(first.rhs) {
                continue;
            }
            if any_node(Node::Expression(first.lhs), &|n| match n {
                Node::Expression(e) => call_has_side_effect(&scope, e),
                _ => false,
            }) {
                continue;
            }
            let target = unparenthesize(next.lhs);
            if matches!(
                target,
                Expression::Access(Access::Property(_) | Access::StaticProperty(_))
            ) && side_effect(next.rhs)
            {
                continue;
            }
            if !matches!(
                unparenthesize(first.lhs),
                Expression::Variable(_)
                    | Expression::Access(Access::Property(_) | Access::StaticProperty(_))
            ) {
                continue;
            }
            if in_try && side_effect(next.rhs) {
                continue;
            }
            removed.push(key);
        }
        if removed.is_empty() {
            return;
        }
        out.push(Report {
            span: node_span(list.stmts[removed[0]]),
            message:
                "This value is assigned again by the next statement; remove the first assignment."
                    .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: remove_statements(
                    source,
                    &list,
                    &(0..list.stmts.len())
                        .map(|i| removed.contains(&i))
                        .collect::<Vec<_>>(),
                ),
            }),
        });
    }
}
