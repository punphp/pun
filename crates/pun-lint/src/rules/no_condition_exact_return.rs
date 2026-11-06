//! `lint/dead-code/no-condition-exact-return`, ported from Rector 2.6.1's `RemoveConditionExactReturnRector`
//! (rules/DeadCode/Rector/Stmt).
//!
//! `if ($value === 'x') { return 'x'; } return $value;` returns `$value` either way: the `if` goes and the `return`
//! takes its place. The compared value must have no side effect. As in Rector, only the `if`'s own statements are
//! looked at, so an `elseif` or `else` goes with it.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use super::rector_stmts::{HOLDERS, holder_list, if_view, line_indent, reindent, remove};
use super::side_effect::has_side_effect;
use crate::syntax::{node_span, same_expression, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoConditionExactReturn;

impl Rule for NoConditionExactReturn {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-condition-exact-return"
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
        if !raw
            .windows(2)
            .any(|w| matches!((w[0], w[1]), (Statement::If(_), Statement::Return(_))))
        {
            return;
        }
        let source = ctx.source;
        let Some(list) = holder_list(source, node, ancestors) else {
            return;
        };
        let mut scope = ancestors.to_vec();
        scope.push(node);
        for (key, &stmt) in list.stmts.iter().enumerate() {
            let Statement::If(r#if) = stmt else {
                continue;
            };
            // matchSoleIfReturn.
            let view = if_view(source, r#if);
            let (1, Some(Statement::Return(sole))) = (view.stmts.len(), view.stmts.stmts.first())
            else {
                continue;
            };
            let Expression::Binary(compare) = unparenthesize(view.condition) else {
                continue;
            };
            if !matches!(
                compare.operator,
                BinaryOperator::Identical(_) | BinaryOperator::Equal(_)
            ) {
                continue;
            }
            if has_side_effect(&scope, compare.rhs) {
                continue;
            }
            if !sole
                .value
                .is_some_and(|value| same_expression(source, compare.rhs, value))
            {
                continue;
            }
            let Some(Statement::Return(next)) = list.stmts.get(key + 1) else {
                continue;
            };
            if !next
                .value
                .is_some_and(|value| same_expression(source, value, compare.lhs))
            {
                continue;
            }
            // The `return` takes the `if`'s place with the comments before it, which php-parser keeps with it.
            let moved = list.with_comments(ctx, key + 1);
            let mut edits = vec![Edit::new(
                node_span(r#if),
                reindent(
                    source,
                    moved,
                    line_indent(source, moved.start),
                    line_indent(source, node_span(r#if).start),
                ),
            )];
            edits.extend(remove(ctx, &list, &[key + 1]));
            out.push(Report {
                span: node_span(r#if),
                message: "The `return` after this `if` returns the same value; remove the `if`."
                    .to_string(),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Safe,
                    edits,
                }),
            });
            return;
        }
    }
}
