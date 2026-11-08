//! `lint/complexity/prefer-array-literal`, ported from Rector 2.8.4's `ArrayMergeOfNonArraysToSimpleArrayRector`
//! (rules/CodeQuality/Rector/FuncCall).
//!
//! `array_merge([$a], [$b])` becomes `[$a, $b]`: when every argument is an array literal, their items make one
//! array. Keys and spreads stay; a by-reference item loses its `&`, as in Rector, which makes a new item of the value
//! and key. Rector prints the new array on one line, the items separated by `, `.

use mago_syntax::cst::{ArrayElement, Expression, Node, NodeKind, UnaryPrefixOperator};
use pun_problem::{Edit, Fix, Safety};

use super::rector_code_quality::{args, named, text};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferArrayLiteral;

impl Rule for PreferArrayLiteral {
    fn id(&self) -> &'static str {
        "lint/complexity/prefer-array-literal"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(c) = node else { return };
        if !named(c, "array_merge") {
            return;
        }
        let mut items = Vec::new();
        for argument in args(c) {
            let elements = match unparenthesize(argument.value()) {
                Expression::Array(a) => &a.elements,
                Expression::LegacyArray(a) => &a.elements,
                _ => return,
            };
            for element in elements.iter() {
                items.push(match element {
                    ArrayElement::KeyValue(e) => {
                        format!(
                            "{} => {}",
                            text(ctx, unparenthesize(e.key)),
                            value(ctx, e.value)
                        )
                    }
                    ArrayElement::Value(e) => value(ctx, e.value).to_string(),
                    // An unpacked item is kept as it was.
                    ArrayElement::Variadic(e) => text(ctx, e).to_string(),
                    // `[, $a]` is a list, never a value: php-parser stops with an error and Rector leaves the file.
                    ArrayElement::Missing(_) => return,
                });
            }
        }
        if items.is_empty() {
            return;
        }
        let span = node_span(c);
        out.push(Report {
            span,
            message: "`array_merge()` of array literals is one array literal.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, format!("[{}]", items.join(", ")))],
            }),
        });
    }
}

/// An item's value as `pArrayItem` prints it in a new item: without parentheses and without `&`.
fn value<'s>(ctx: &Context<'s>, expr: &Expression<'_>) -> &'s str {
    let expr = unparenthesize(expr);
    match expr {
        Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Reference(_)) => {
            text(ctx, unparenthesize(u.operand))
        }
        _ => text(ctx, expr),
    }
}
