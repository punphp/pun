//! `lint/style/prefer-early-return-of-variable`, ported from Rector 2.6.1's `ReturnEarlyIfVariableRector`
//! (rules/EarlyReturn/Rector/StmtsAwareInterface), with the static, global and by-reference variables of
//! `VariableAnalyzer`.
//!
//! An `if` whose only statement assigns the variable the next statement returns returns the value directly:
//! `if ($value === 50) { $value = 100; } return $value;` becomes `if ($value === 50) { return 100; } return $value;`.
//! A `@var` tag on the assignment, a `static` or `global` variable and a variable used by reference keep it.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use super::rector_code_quality::equal;
use super::rector_stmts::{
    HOLDERS, assign, expression, has_var_tag, holder_list, if_view, is_static_or_global,
    is_used_by_reference,
};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferEarlyReturnOfVariable;

impl Rule for PreferEarlyReturnOfVariable {
    fn id(&self) -> &'static str {
        "lint/style/prefer-early-return-of-variable"
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
        // A cheap look first: an `if` right before a `return`.
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
        for (key, &stmt) in list.stmts.iter().enumerate() {
            // matchNextStmtReturnVariable: the next statement returns a variable.
            let Some(Statement::Return(r#return)) = list.stmts.get(key + 1) else {
                continue;
            };
            let Some(returned) = r#return.value.map(unparenthesize) else {
                continue;
            };
            if !matches!(returned, Expression::Variable(_)) {
                continue;
            }
            let Statement::If(r#if) = stmt else {
                continue;
            };
            let view = if_view(source, r#if);
            if view.else_.is_some() || !view.elseifs.is_empty() || view.stmts.len() != 1 {
                continue;
            }
            let only = view.stmts.stmts[0];
            // matchOnlyIfStmtReturnExpr.
            let Some(assign) = expression(only).and_then(assign) else {
                continue;
            };
            if has_var_tag(
                source,
                view.stmts.with_comments(ctx, 0).start,
                node_span(only).start,
            ) {
                continue;
            }
            let Expression::Variable(variable) = unparenthesize(assign.lhs) else {
                continue;
            };
            if let Variable::Direct(direct) = variable {
                let name = &direct.name[1..];
                let mut chain = ancestors.to_vec();
                chain.push(node);
                chain.push(Node::Statement(stmt));
                if is_static_or_global(&chain, name)
                    || is_used_by_reference(&chain, name, node_span(direct).start)
                {
                    continue;
                }
            }
            if !equal(ctx, assign.lhs, returned) {
                continue;
            }
            let value = node_span(unparenthesize(assign.rhs));
            out.push(Report {
                span: node_span(only),
                message: "Return the value here instead of assigning the variable returned next."
                    .to_string(),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Safe,
                    edits: vec![Edit::new(
                        node_span(only),
                        format!("return {};", &source[value.start..value.end]),
                    )],
                }),
            });
            return;
        }
    }
}
