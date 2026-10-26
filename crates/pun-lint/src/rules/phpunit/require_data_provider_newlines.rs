//! `lint/phpunit/require-data-provider-newlines`, ported from rector-phpunit's `DataProviderArrayItemsNewLinedRector`
//! (rules/CodeQuality/Rector/ClassMethod).
//!
//! In a public method of a test class that is not a test (its name does not start with `test` and it has no
//! `#[Test]`), every returned array (`BetterNodeFinder::findInstanceOf`, nested functions included) with an item that
//! starts on the line the item before it ends is printed with an item a line: Rector's `NEWLINED_ARRAY_PRINT`, which
//! is `[`, each item one level deeper than the `return` after its comments with a trailing comma, and `]`. An item's
//! own lines move with it.

use mago_syntax::cst::{ArrayElement, Expression, MethodBody, Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::{is_in_test_class, is_public, report};
use crate::rules::rector_batch5::has_attribute;
use crate::rules::rector_stmts::{line_indent, previous_token_end, reindent, tokens};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct RequireDataProviderNewlines;

impl Rule for RequireDataProviderNewlines {
    fn id(&self) -> &'static str {
        "lint/phpunit/require-data-provider-newlines"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Method(method) = node else {
            return;
        };
        let MethodBody::Concrete(block) = &method.body else {
            return;
        };
        if !is_public(method)
            || !is_in_test_class(ctx, ancestors, Some(method))
            || method.name.value.starts_with(b"test")
            || has_attribute(
                ancestors,
                &method.attribute_lists,
                "PHPUnit\\Framework\\Attributes\\Test",
            )
        {
            return;
        }
        let source = ctx.source;
        let mut stack: Vec<Node> = Vec::new();
        Node::Block(block).visit_children(|child| stack.push(child));
        let mut returns = Vec::new();
        while let Some(node) = stack.pop() {
            if let Node::Return(r) = node {
                returns.push(r);
            }
            node.visit_children(|child| stack.push(child));
        }
        for r in returns {
            let Some(value) = r.value else {
                continue;
            };
            let array = unparenthesize(value);
            let (elements, open): (Vec<&ArrayElement<'_>>, usize) = match array {
                Expression::Array(a) => (
                    a.elements.iter().collect(),
                    a.left_bracket.end.offset as usize,
                ),
                Expression::LegacyArray(a) => (
                    a.elements.iter().collect(),
                    a.left_parenthesis.end.offset as usize,
                ),
                _ => continue,
            };
            let spans: Vec<Span> = elements.iter().map(|e| node_span(*e)).collect();
            let line = |pos: usize| source[..pos].bytes().filter(|&b| b == b'\n').count();
            let reprint = spans.windows(2).any(|w| line(w[1].start) == line(w[0].end));
            if !reprint {
                continue;
            }
            let indent = line_indent(source, node_span(r).start);
            let inner = format!("{indent}    ");
            let mut text = String::from("[");
            let mut from = open;
            for (element, span) in elements.iter().zip(&spans) {
                let before = previous_token_end(source, from, span.start);
                for comment in tokens(source, before, span.start)
                    .into_iter()
                    .filter(|t| t.kind.is_comment())
                {
                    text.push('\n');
                    text.push_str(&inner);
                    text.push_str(&source[comment.start..comment.end]);
                }
                text.push('\n');
                if !matches!(element, ArrayElement::Missing(_)) {
                    text.push_str(&inner);
                    text.push_str(&reindent(
                        source,
                        *span,
                        line_indent(source, span.start),
                        &inner,
                    ));
                }
                text.push(',');
                from = span.end;
            }
            text.push('\n');
            text.push_str(indent);
            text.push(']');
            let span = node_span(array);
            out.push(report(
                span,
                "Put each item of the data provider's array on a line of its own.",
                Safety::Safe,
                vec![Edit::new(span, text)],
            ));
        }
    }
}
