//! `lint/dead-code/no-double-self-assignment`, ported from Rector 2.8.4's `RemoveDoubleSelfAssignRector`
//! (rules/DeadCode/Rector/Assign).
//!
//! `$a = $a = f();` becomes `$a = f();`: an `=` whose value is another `=` to the same variable. Both sides must be
//! variables (`$a`, `$$a`, `${...}`), compared as code without whitespace and comments (`NodeComparator`).

use mago_syntax::cst::{AssignmentOperator, Expression, Node, NodeKind, UnaryPrefixOperator};
use pun_problem::{Edit, Fix, Safety};

use crate::syntax::{node_span, same_code, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoDoubleSelfAssignment;

impl Rule for NoDoubleSelfAssignment {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-double-self-assignment"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Assignment])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Assignment(set) = node else { return };
        if !matches!(set.operator, AssignmentOperator::Assign(_))
            || !matches!(set.lhs, Expression::Variable(_))
        {
            return;
        }
        let Expression::Assignment(inner) = unparenthesize(set.rhs) else {
            return;
        };
        // `$a = &$b` is php-parser's AssignRef, not an Assign.
        let by_reference = matches!(
            inner.rhs,
            Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Reference(_))
        );
        if !matches!(inner.operator, AssignmentOperator::Assign(_))
            || by_reference
            || !matches!(inner.lhs, Expression::Variable(_))
        {
            return;
        }
        if !same_code(ctx.source, node_span(set.lhs), node_span(inner.lhs)) {
            return;
        }
        let span = node_span(set);
        let kept = node_span(inner);
        out.push(Report {
            span,
            message: "The variable is assigned to itself twice; assign it once.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, &ctx.source[kept.start..kept.end])],
            }),
        });
    }
}
