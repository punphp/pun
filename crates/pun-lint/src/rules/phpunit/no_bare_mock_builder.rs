//! `lint/phpunit/no-bare-mock-builder`, ported from rector-phpunit's `GetMockBuilderGetMockToCreateMockRector`
//! (rules/PHPUnit60/Rector/MethodCall).
//!
//! `$this->getMockBuilder(X)->getMock()`, with any of `disableOriginalConstructor()`, `disableOriginalClone()`,
//! `disableArgumentCloning()` and `disallowMockingUnknownTypes()` between, becomes `$this->createMock(X)` where `$this`
//! is a `TestCase`. php-parser prints the new call with `getMockBuilder()`'s arguments.

use mago_syntax::cst::{Call, Expression, Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::{AnyCall, TEST_CASE, maybe_multiline, receiver_is};
use crate::rules::laravel::is_variable;
use crate::rules::rector_stmts::line_indent;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

const USELESS_METHOD_NAMES: &[&str] = &[
    "disableOriginalConstructor",
    "disableOriginalClone",
    "disableArgumentCloning",
    "disallowMockingUnknownTypes",
];

pub struct NoBareMockBuilder;

impl Rule for NoBareMockBuilder {
    fn id(&self) -> &'static str {
        "lint/phpunit/no-bare-mock-builder"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let call = AnyCall::of(node).unwrap();
        if !call.is_name(&["getMock"]) {
            return;
        }
        let mut current = call.receiver;
        let builder = loop {
            let Expression::Call(Call::Method(inner)) = unparenthesize(current) else {
                return;
            };
            let inner = AnyCall::of(Node::MethodCall(inner)).unwrap();
            if !inner.is_name(USELESS_METHOD_NAMES) {
                break inner;
            }
            current = inner.receiver;
        };
        if !is_variable(unparenthesize(builder.receiver), "this")
            || !receiver_is(ctx, ancestors, &builder, &[TEST_CASE])
            || !builder.is_name(&["getMockBuilder"])
        {
            return;
        }
        let args: Vec<Span> = builder.args.arguments.iter().map(node_span).collect();
        let from = builder.args.left_parenthesis.end.offset as usize;
        let text = format!(
            "$this->createMock({})",
            maybe_multiline(ctx, from, &args, line_indent(ctx.source, call.span.start))
        );
        out.push(Report::fixed(
            call.span,
            "Use `createMock()` instead of a bare mock builder.",
            Safety::Safe,
            vec![Edit::new(call.span, text)],
        ));
    }
}
