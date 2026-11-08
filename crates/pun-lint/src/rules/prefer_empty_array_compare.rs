//! `lint/complexity/prefer-empty-array-compare`, ported from Rector 2.8.4's `SimplifyEmptyArrayCheckRector`
//! (rules/CodeQuality/Rector/BooleanAnd) with `BinaryOpManipulator::matchFirstAndSecondConditionNode`.
//!
//! `is_array($values) && empty($values)`, in either order, becomes `$values === []`.

use mago_syntax::cst::{BinaryOperator, Construct, Expression, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::rector_code_quality::{Prec, args, call_to, equal, in_parent, text};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferEmptyArrayCompare;

impl Rule for PreferEmptyArrayCompare {
    fn id(&self) -> &'static str {
        "lint/complexity/prefer-empty-array-compare"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Binary])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Binary(and) = node else { return };
        if !matches!(and.operator, BinaryOperator::And(_)) {
            return;
        }
        // matchFirstAndSecondConditionNode: `is_array()` on the left and `empty()` on the right, or the other way.
        let Some((array, empty)) = is_array_arg(and.lhs)
            .zip(empty_arg(and.rhs))
            .or_else(|| is_array_arg(and.rhs).zip(empty_arg(and.lhs)))
        else {
            return;
        };
        if !equal(ctx, empty, array) {
            return;
        }
        let span = node_span(and);
        let identical = Prec::Infix(80, 80, 80);
        let code = format!("{} === []", text(ctx, unparenthesize(empty)));
        out.push(Report {
            span,
            message: "Compare with `[]` instead of checking `is_array()` and `empty()`."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, in_parent(span, ancestors, code, identical))],
            }),
        });
    }
}

/// The first condition: `is_array($variable)`, and its argument.
fn is_array_arg<'a>(expr: &'a Expression<'a>) -> Option<&'a Expression<'a>> {
    let value = args(call_to(expr, "is_array")?).first()?.value();
    matches!(unparenthesize(value), Expression::Variable(_)).then_some(value)
}

/// The second condition: `empty($variable)`, and its argument.
fn empty_arg<'a>(expr: &'a Expression<'a>) -> Option<&'a Expression<'a>> {
    let Expression::Construct(Construct::Empty(empty)) = unparenthesize(expr) else {
        return None;
    };
    matches!(unparenthesize(empty.value), Expression::Variable(_)).then_some(empty.value)
}
