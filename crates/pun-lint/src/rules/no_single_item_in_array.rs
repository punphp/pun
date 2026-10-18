//! `lint/complexity/no-single-item-in-array`, ported from Rector 2.6.1's `SingleInArrayToCompareRector`
//! (rules/CodeQuality/Rector/FuncCall).
//!
//! `in_array($value, ['one'])` becomes `$value == 'one'`, or `$value === 'one'` when the call passes a third
//! argument (whatever its value). Under a `!` the comparison is negated: `$value != 'one'`, `$value !== 'one'`.

use mago_syntax::cst::{ArrayElement, Expression, FunctionCall, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety, Span};

use super::rector_code_quality::{Prec, args, call_to, in_parent, infix, named};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoSingleItemInArray;

impl Rule for NoSingleItemInArray {
    fn id(&self) -> &'static str {
        "lint/complexity/no-single-item-in-array"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::UnaryPrefix, NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (span, call, negated) = match node {
            Node::UnaryPrefix(not) if not.operator.is_not() => {
                let Some(call) = call_to(not.operand, "in_array") else {
                    return;
                };
                (node_span(not), call, true)
            }
            Node::FunctionCall(call) => {
                // Rector visits the `!` first and replaces it with the whole comparison.
                let parent = ancestors.iter().rev().find(|a| {
                    !matches!(
                        a,
                        Node::Expression(_) | Node::Call(_) | Node::Parenthesized(_)
                    )
                });
                if matches!(parent, Some(Node::UnaryPrefix(u)) if u.operator.is_not())
                    || !named(call, "in_array")
                {
                    return;
                }
                (node_span(call), call, false)
            }
            _ => return,
        };
        let Some((needle, item)) = single_item(call) else {
            return;
        };
        let strict = args(call).len() > 2;
        let op = match (strict, negated) {
            (true, false) => "===",
            (true, true) => "!==",
            (false, false) => "==",
            (false, true) => "!=",
        };
        let compare = Prec::Infix(80, 80, 80);
        let code = in_parent(
            span,
            ancestors,
            infix(ctx, needle, op, item, (80, 80)),
            compare,
        );
        report(out, span, op, code);
    }
}

/// resolveArrayItem: the needle and the only item of an `in_array()` whose haystack is an array literal with one
/// item that is not spread. A key on the item is ignored.
fn single_item<'a>(call: &'a FunctionCall<'a>) -> Option<(&'a Expression<'a>, &'a Expression<'a>)> {
    let args = args(call);
    let elements = match unparenthesize(args.get(1)?.value()) {
        Expression::Array(a) => &a.elements,
        Expression::LegacyArray(a) => &a.elements,
        _ => return None,
    };
    let item = match elements.iter().collect::<Vec<_>>().as_slice() {
        [ArrayElement::Value(e)] => e.value,
        [ArrayElement::KeyValue(e)] => e.value,
        _ => return None,
    };
    Some((args[0].value(), item))
}

fn report(out: &mut Vec<Report>, span: Span, op: &str, code: String) {
    out.push(Report {
        span,
        message: format!("Use `{op}` instead of `in_array()` with one item."),
        help: None,
        fix: Some(Fix {
            safety: Safety::Safe,
            edits: vec![Edit::new(span, code)],
        }),
    });
}
