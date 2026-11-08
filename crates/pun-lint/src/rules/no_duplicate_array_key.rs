//! `lint/dead-code/no-duplicate-array-key`, ported from Rector 2.8.4's `RemoveDuplicatedArrayKeyRector`
//! (rules/DeadCode/Rector/Array_), with `ExprAnalyzer::isDynamicExpr`.
//!
//! In an array literal, an item whose key a later item repeats is removed: PHP keeps only the last value. Keys are
//! compared as printed code (here: the same tokens), and only keys known at compile time count (strings, numbers,
//! constants, class constants on a written class name, arrays of those) plus plain variables. Calls, property
//! fetches and `++$i` keys are skipped. A destructuring list (`[$a, $a] = ...`) is php-parser's `List_`, not an
//! array.

use mago_syntax::cst::{ArrayElement, Expression, Node, NodeKind, TokenSeparatedSequence};
use pun_problem::{Edit, Fix, Safety, Span};

use super::php_parser::{Token, is_dynamic_expr, previous_token_end, tokens};
use crate::syntax::{node_span, same_code, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoDuplicateArrayKey;

impl Rule for NoDuplicateArrayKey {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-duplicate-array-key"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Array, NodeKind::LegacyArray])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let elements = match node {
            Node::Array(a) => &a.elements,
            _ => {
                let Node::LegacyArray(a) = node else { return };
                &a.elements
            }
        };
        let duplicated = duplicated_items(ctx.source, elements);
        if duplicated.is_empty() || is_destructuring(node, ancestors) {
            return;
        }
        let tokens = tokens(ctx);
        let items = elements.as_slice();
        let edits = duplicated
            .iter()
            .map(|&i| {
                Edit::new(
                    Span::new(
                        with_comments(ctx.source, tokens, &items[i]),
                        with_comments(ctx.source, tokens, &items[i + 1]),
                    ),
                    "",
                )
            })
            .collect();
        out.push(Report {
            span: node_span(&items[duplicated[0]]),
            message: "A later item repeats this key, so this value is lost; remove it.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// `resolveDuplicateKeysArrayItems` and `filterItemsWithSameKey`: the indices of the items whose key a later item
/// repeats, in order.
fn duplicated_items(
    source: &str,
    elements: &TokenSeparatedSequence<'_, ArrayElement<'_>>,
) -> Vec<usize> {
    let mut groups: Vec<Vec<(usize, Span)>> = Vec::new();
    for (i, element) in elements.iter().enumerate() {
        let ArrayElement::KeyValue(item) = element else {
            continue;
        };
        let key = unparenthesize(item.key);
        // local variable is mostly fine, other dynamic, just skip
        if !matches!(key, Expression::Variable(_)) && is_dynamic_expr(key) {
            continue;
        }
        let span = node_span(key);
        match groups.iter_mut().find(|g| same_code(source, g[0].1, span)) {
            Some(group) => group.push((i, span)),
            None => groups.push(vec![(i, span)]),
        }
    }
    let mut duplicated: Vec<usize> = groups
        .iter()
        .flat_map(|g| g[..g.len() - 1].iter().map(|(i, _)| *i))
        .collect();
    duplicated.sort_unstable();
    duplicated
}

/// Where an item starts with the comments before it, which php-parser keeps with it.
fn with_comments(source: &str, tokens: &[Token], element: &ArrayElement<'_>) -> usize {
    let start = node_span(element).start;
    let from = previous_token_end(source, tokens, start);
    tokens
        .iter()
        .find(|t| t.start >= from && t.end <= start && t.kind.is_comment())
        .map_or(start, |t| t.start)
}

/// Whether the array is the target of a destructuring assignment or of a `foreach`, maybe nested in another one.
fn is_destructuring(node: Node<'_, '_>, ancestors: &[Node<'_, '_>]) -> bool {
    let span = node_span(&node);
    for ancestor in ancestors.iter().rev() {
        match ancestor {
            Node::Expression(_)
            | Node::ArrayElement(_)
            | Node::KeyValueArrayElement(_)
            | Node::ValueArrayElement(_)
            | Node::Array(_)
            | Node::LegacyArray(_)
            | Node::List(_) => {}
            Node::Assignment(a) => {
                let lhs = node_span(a.lhs);
                return lhs.start <= span.start && span.end <= lhs.end;
            }
            Node::ForeachTarget(_)
            | Node::ForeachKeyValueTarget(_)
            | Node::ForeachValueTarget(_) => {
                return true;
            }
            _ => break,
        }
    }
    false
}
