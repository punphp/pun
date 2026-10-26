//! `lint/phpunit/no-at-matcher`, ported from rector-phpunit's `ReplaceAtMethodWithDesiredMatcherRector`
//! (rules/PHPUnit90/Rector/MethodCall).
//!
//! In a test class, a method call on a method call whose arguments hold an `at()` call (`findAtMethodCall`, which
//! looks at that inner call's own arguments only) gets the matcher the count asks for: `at(0)` becomes `never()`,
//! `at(1)` `once()` and `at(n)` `exactly(n)`, the count being the last integer literal among `at()`'s arguments.
//! php-parser prints the new call: `at()`'s receiver, `->` and the name.

use mago_syntax::cst::{Call, Expression, Literal, Node, NodeKind};
use pun_problem::{Edit, Safety};

use super::{AnyCall, arg_value, expr_text, is_in_test_class, report};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoAtMatcher;

impl Rule for NoAtMatcher {
    fn id(&self) -> &'static str {
        "lint/phpunit/no-at-matcher"
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
        let Node::MethodCall(call) = node else {
            return;
        };
        let Expression::Call(Call::Method(var)) = unparenthesize(call.object) else {
            return;
        };
        if !is_in_test_class(ctx, ancestors, None) {
            return;
        }
        let at = var.argument_list.arguments.iter().find_map(|arg| {
            match unparenthesize(arg_value(arg)) {
                Expression::Call(Call::Method(at))
                    if AnyCall::of(Node::MethodCall(at)).is_some_and(|c| c.is_name(&["at"])) =>
                {
                    Some(at)
                }
                _ => None,
            }
        });
        let Some(at) = at else {
            return;
        };
        let count =
            at.argument_list.arguments.iter().fold(None, |count, arg| {
                match unparenthesize(arg_value(arg)) {
                    Expression::Literal(Literal::Integer(i)) => i.value,
                    _ => count,
                }
            });
        let matcher = match count {
            Some(0) => "never()".to_string(),
            Some(1) => "once()".to_string(),
            Some(n) => format!("exactly({n})"),
            None => return,
        };
        let span = node_span(at);
        out.push(report(
            span,
            "Use a matcher for the count instead of `at()`.",
            Safety::Unsafe,
            vec![Edit::new(
                span,
                format!("{}->{matcher}", expr_text(ctx, at.object)),
            )],
        ));
    }
}
