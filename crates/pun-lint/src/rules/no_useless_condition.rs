//! `lint/dead-code/no-useless-condition`, ported from Rector 2.6.1's `RemoveDeadConditionAboveReturnRector`
//! (rules/DeadCode/Rector/Return_).
//!
//! An `if` without `elseif` or `else`, right before a `return`, whose body is empty or returns the same thing, does
//! nothing the `return` doesn't: it goes, unless its condition has a side effect.

use mago_syntax::cst::*;
use pun_problem::{Fix, Safety};

use super::rector_stmts::{HOLDERS, holder_list, if_view, remove};
use super::side_effect::has_side_effect;
use crate::syntax::{node_span, same_node};
use crate::{Context, Report, Rule};

pub struct NoUselessCondition;

impl Rule for NoUselessCondition {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-useless-condition"
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
            if !matches!(stmt, Statement::Return(_)) || key == 0 {
                continue;
            }
            let Statement::If(previous) = list.stmts[key - 1] else {
                continue;
            };
            let view = if_view(source, previous);
            if !view.elseifs.is_empty() || view.else_.is_some() {
                continue;
            }
            if has_side_effect(&scope, view.condition) {
                continue;
            }
            match view.stmts.len() {
                0 => {}
                1 => match view.stmts.stmts.first() {
                    Some(first @ Statement::Return(_))
                        if same_node(source, Node::Statement(first), Node::Statement(stmt)) => {}
                    _ => return,
                },
                _ => return,
            }
            out.push(Report {
                span: node_span(previous),
                message: "This `if` returns what the `return` after it returns; remove it."
                    .to_string(),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Safe,
                    edits: remove(ctx, &list, &[key - 1]),
                }),
            });
            return;
        }
    }
}
