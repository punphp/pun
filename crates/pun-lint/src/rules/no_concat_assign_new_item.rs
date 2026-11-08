//! `lint/complexity/no-concat-assign-new-item`, ported from Rector 2.8.4's `NewArrayItemConcatAssignToAssignRector`
//! (rules/CodeQuality/Rector/AssignOp).
//!
//! `$values[] .= $name;` becomes `$values[] = $name;`: a new array item starts as `null`, so the concatenation adds
//! nothing to it.

use mago_syntax::cst::{AssignmentOperator, Expression, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety, Span};

use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoConcatAssignNewItem;

impl Rule for NoConcatAssignNewItem {
    fn id(&self) -> &'static str {
        "lint/complexity/no-concat-assign-new-item"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Assignment])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Assignment(a) = node else { return };
        // `$values[]` is php-parser's `ArrayDimFetch` without a `dim`.
        if !matches!(a.operator, AssignmentOperator::Concat(_))
            || !matches!(a.lhs, Expression::ArrayAppend(_))
        {
            return;
        }
        // The new `Assign` prints ` = ` between the kept array item and value.
        let edit = Edit::new(
            Span::new(node_span(a.lhs).end, node_span(a.rhs).start),
            " = ",
        );
        out.push(Report {
            span: node_span(a),
            message: "Use `=` instead of `.=` on a new array item.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![edit],
            }),
        });
    }
}
