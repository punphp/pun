//! `lint/php-8.5/prefer-array-first-last`, ported from Rector 2.6.1's `ArrayFirstLastRector`
//! (rules/Php85/Rector/ArrayDimFetch).
//!
//! `$a[array_key_first($a)]` becomes `array_first($a)` (and `_last` likewise); `array_values($a)[0]` becomes
//! `array_first($a)` and `array_values($a)[count($a) - 1]` becomes `array_last($a)`. A fetch that is written to
//! (assigned, compound-assigned or unset) stays.

use mago_syntax::cst::{BinaryOperator, Expression, Literal, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::lint_b_helpers::value;
use super::rector_code_quality::{args, call_to, equal, named};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferArrayFirstLast;

impl Rule for PreferArrayFirstLast {
    fn id(&self) -> &'static str {
        "lint/php-8.5/prefer-array-first-last"
    }

    fn min_php_version(&self) -> u32 {
        // PhpVersionFeature::ARRAY_FIRST_LAST
        80500
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::ArrayAccess])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::ArrayAccess(fetch) = node else {
            return;
        };
        let text = |e: &Expression<'_>| {
            let e = unparenthesize(e);
            &ctx.source[node_span(e).start..node_span(e).end]
        };
        let index = unparenthesize(fetch.index);
        let new = if let Expression::Call(mago_syntax::cst::Call::Function(dim)) = index {
            // refactorArrayKeyPattern
            let function = if named(dim, "array_key_first") {
                "array_first"
            } else if named(dim, "array_key_last") {
                "array_last"
            } else {
                return;
            };
            let [arg] = args(dim)[..] else { return };
            if !equal(ctx, fetch.array, value(arg)) || written(node_span(fetch), ancestors) {
                return;
            }
            format!("{function}({})", text(fetch.array))
        } else {
            // refactorArrayValuesPattern
            let Some(values) = call_to(fetch.array, "array_values") else {
                return;
            };
            if !matches!(index, Expression::Literal(Literal::Integer(_)))
                && !matches!(index, Expression::Binary(b) if matches!(b.operator, BinaryOperator::Subtraction(_)))
            {
                return;
            }
            let [arg] = args(values)[..] else { return };
            if written(node_span(fetch), ancestors) {
                return;
            }
            let array = value(arg);
            match index {
                Expression::Literal(Literal::Integer(i)) if i.value == Some(0) => {
                    format!("array_first({})", text(array))
                }
                Expression::Binary(b) => {
                    let Some(count) = call_to(b.lhs, "count") else {
                        return;
                    };
                    let [counted] = args(count)[..] else { return };
                    if !matches!(unparenthesize(b.rhs), Expression::Literal(Literal::Integer(i)) if i.value == Some(1))
                        || !equal(ctx, array, value(counted))
                    {
                        return;
                    }
                    format!("array_last({})", text(array))
                }
                _ => return,
            }
        };
        out.push(Report {
            span: node_span(fetch),
            message: "Use `array_first()` or `array_last()`.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(node_span(fetch), new)],
            }),
        });
    }
}

/// `shouldSkip`: the fetch is (inside) the target of an assignment, a compound assignment or `unset()`.
fn written(span: pun_problem::Span, ancestors: &[Node<'_, '_>]) -> bool {
    ancestors.iter().any(|a| match a {
        Node::Assignment(assignment) => {
            let lhs = node_span(assignment.lhs);
            lhs.start <= span.start && span.end <= lhs.end
        }
        Node::Unset(_) => true,
        _ => false,
    })
}
