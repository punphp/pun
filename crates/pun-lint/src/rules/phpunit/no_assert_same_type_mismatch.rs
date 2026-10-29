//! `lint/phpunit/no-assert-same-type-mismatch`, ported from rector-phpunit's `MatchAssertSameExpectedTypeRector`
//! (rules/CodeQuality/Rector/MethodCall).
//!
//! In `assertSame('123', $a)` (or `assertEquals`) where `$a` is natively an integer, the expected string becomes the
//! integer PHP's `(int)` makes of it; in `assertSame(123, $a)` where `$a` is natively a string, the integer becomes a
//! string.

use mago_syntax::cst::{Expression, Literal, Node, NodeKind};
use pun_problem::{Edit, Safety};

use super::ptype::{native_is_integer, native_is_string};
use super::{AnyCall, arg_value, is_test_case_call_named, report};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoAssertSameTypeMismatch;

impl Rule for NoAssertSameTypeMismatch {
    fn id(&self) -> &'static str {
        "lint/phpunit/no-assert-same-type-mismatch"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall, NodeKind::StaticMethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let call = AnyCall::of(node).unwrap();
        if !is_test_case_call_named(ctx, ancestors, &call, &["assertSame", "assertEquals"]) {
            return;
        }
        let args = call.args();
        if args.len() < 2 {
            return;
        }
        let expected = arg_value(args[0]);
        let actual = arg_value(args[1]);
        let code = match unparenthesize(expected) {
            Expression::Literal(Literal::String(s)) if native_is_integer(ctx, actual) => {
                let value = String::from_utf8_lossy(s.value.unwrap_or_default());
                pun_tokens::php_compat::intval(&value).to_string()
            }
            Expression::Literal(Literal::Integer(i)) if native_is_string(ctx, actual) => {
                if call.is_name(&["assertEquals"]) {
                    return;
                }
                format!("'{}'", i.value.unwrap())
            }
            _ => return,
        };
        out.push(report(
            node_span(expected),
            "Match the expected value's type to the actual one's.",
            Safety::Safe,
            vec![Edit::new(node_span(expected), code)],
        ));
    }
}
