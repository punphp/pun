//! `lint/operators/no-loose-equality`, ported from phpstan-strict-rules 2.0.12's `DisallowedLooseComparisonRule`
//! (identifiers `equal.notAllowed` and `notEqual.notAllowed`). Every `==`, `!=` and `<>` is reported, with the
//! message PHPStan gives without bleeding edge (`includeOperandTypesInErrorMessage` off).

use mago_syntax::cst::{BinaryOperator, Node, NodeKind};

use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoLooseEquality;

impl Rule for NoLooseEquality {
    fn id(&self) -> &'static str {
        "lint/operators/no-loose-equality"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Binary])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Binary(binary) = node else { return };
        let (loose, strict) = match binary.operator {
            BinaryOperator::Equal(_) => ("==", "==="),
            BinaryOperator::NotEqual(_) | BinaryOperator::AngledNotEqual(_) => ("!=", "!=="),
            _ => return,
        };
        out.push(Report {
            span: node_span(binary),
            message: format!("Loose comparison via \"{loose}\" is not allowed."),
            help: Some(format!("Use strict comparison via \"{strict}\" instead.")),
            fix: None,
        });
    }
}
