//! `lint/phpunit/prefer-assert-instances-of`, ported from rector-phpunit's `SimplifyForeachInstanceOfRector`
//! (rules/CodeQuality/Rector/Foreach_).
//!
//! A `foreach` whose only statement is `assertInstanceOf($class, $item)` on its value becomes
//! `assertContainsOnlyInstancesOf($class, $items)`. As upstream, any method or static call named
//! `assertInstanceOf` counts, a third argument (a message) keeps the loop, and the comments before the loop go.

use mago_syntax::cst::{Call, Expression, ForeachBody, ForeachTarget, Node, NodeKind, Statement};
use pun_problem::{Edit, Safety, Span};

use super::{AnyCall, arg_value, expr_text, report, text};
use crate::rules::php_parser::{body_list, open_list, previous_token_end, tokens};
use crate::syntax::{node_span, same_expression};
use crate::{Context, Report, Rule};

pub struct PreferAssertInstancesOf;

impl Rule for PreferAssertInstancesOf {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-assert-instances-of"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Foreach])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Foreach(foreach) = node else { return };
        let tokens = tokens(ctx.source);
        let list = match &foreach.body {
            ForeachBody::Statement(statement) => body_list(&tokens, statement),
            ForeachBody::ColonDelimited(body) => open_list(
                &tokens,
                &body.statements,
                body.colon.end.offset as usize,
                mago_span::HasSpan::span(&body.end_foreach).start.offset as usize,
            ),
        };
        if list.len() != 1 || list.stmts.is_empty() {
            return;
        }
        let Statement::Expression(statement) = list.stmts[0] else {
            return;
        };
        let call = match statement.expression {
            Expression::Call(Call::Method(m)) => AnyCall::of(Node::MethodCall(m)).unwrap(),
            Expression::Call(Call::StaticMethod(s)) => {
                AnyCall::of(Node::StaticMethodCall(s)).unwrap()
            }
            _ => return,
        };
        if !call.is_name(&["assertInstanceOf"]) {
            return;
        }
        let args = call.args();
        let (Some(first), Some(second)) = (args.first().copied(), args.get(1).copied()) else {
            return;
        };
        let value = match &foreach.target {
            ForeachTarget::Value(t) => t.value,
            ForeachTarget::KeyValue(t) => t.value,
        };
        if !same_expression(ctx.source, value, arg_value(second)) || args.len() == 3 {
            return;
        }
        let code = format!(
            "{}assertContainsOnlyInstancesOf({}, {});",
            call.receiver_text(ctx),
            text(ctx, first),
            expr_text(ctx, foreach.expression),
        );
        let span = node_span(foreach);
        let from = previous_token_end(ctx.source, &tokens, span.start);
        let start = crate::rules::rector_stmts::first_comment(ctx.source, from, span.start)
            .unwrap_or(span.start);
        out.push(report(
            span,
            "Use `assertContainsOnlyInstancesOf()` instead of a loop.",
            Safety::Safe,
            vec![Edit::new(Span::new(start, span.end), code)],
        ));
    }
}
