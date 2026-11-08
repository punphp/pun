//! `lint/style/no-assign-in-ternary-branch`, ported from Rector 2.8.4's `TernaryConditionVariableAssignmentRector`
//! (rules/CodingStyle/Rector/Ternary).
//!
//! `$value ? $a = 1 : $a = 0;` becomes `$a = $value ? 1 : 0;`: when both branches of a ternary assign to the same
//! variable, the ternary chooses the value and is assigned once. A ternary that is itself assigned
//! (`$x = $value ? $a = 1 : $a = 0`) stays.

use mago_syntax::cst::{Expression, Node, NodeKind, Variable};
use pun_problem::{Edit, Fix, Safety};

use super::rector_code_quality::{Prec, in_parent, operand};
use super::rector_conditions::is_assign;
use super::rector_stmts::is_plain_assign;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoAssignInTernaryBranch;

impl Rule for NoAssignInTernaryBranch {
    fn id(&self) -> &'static str {
        "lint/style/no-assign-in-ternary-branch"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Conditional])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Conditional(t) = node else { return };
        let Some(then) = t.then else { return };
        let (Expression::Assignment(if_assign), Expression::Assignment(else_assign)) =
            (unparenthesize(then), unparenthesize(t.r#else))
        else {
            return;
        };
        if !is_assign(then) || !is_assign(t.r#else) {
            return;
        }
        // Two variables with the same written name; `$$a` names no variable (php-parser compares two name nodes).
        let (
            Expression::Variable(Variable::Direct(if_var)),
            Expression::Variable(Variable::Direct(else_var)),
        ) = (
            unparenthesize(if_assign.lhs),
            unparenthesize(else_assign.lhs),
        )
        else {
            return;
        };
        if if_var.name != else_var.name {
            return;
        }
        let span = node_span(t);
        // AttributeKey::IS_ASSIGNED_TO: the ternary is the value of a plain assignment.
        if let Some(Node::Assignment(parent)) = ancestors
            .iter()
            .rev()
            .find(|a| !matches!(a, Node::Expression(_) | Node::Parenthesized(_)))
            && is_plain_assign(parent)
            && node_span(unparenthesize(parent.rhs)) == span
        {
            return;
        }
        // The ternary prints again (pExpr_Ternary): the condition as the left operand, the new values as they were.
        let code = format!(
            "{} = {} ? {} : {}",
            super::rector_code_quality::text(ctx, if_var),
            operand(ctx, t.condition, 150, Some(150)),
            super::rector_code_quality::text(ctx, unparenthesize(if_assign.rhs)),
            operand(ctx, else_assign.rhs, 150, None)
        );
        out.push(Report {
            span,
            message:
                "Assign the ternary's result once instead of assigning in both of its branches."
                    .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(
                    span,
                    in_parent(span, ancestors, code, Prec::Prefix(160)),
                )],
            }),
        });
    }
}
