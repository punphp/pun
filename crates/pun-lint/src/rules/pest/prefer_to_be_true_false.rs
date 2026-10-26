//! `lint/pest/prefer-to-be-true-false`, ported from pest-plugin-rector's `SimplifyToLiteralBooleanRector`:
//! `->toBe(true)` becomes `->toBeTrue()`, `->toBe(false)` `->toBeFalse()` and `->toBe(null)` `->toBeNull()`;
//! `->toEqual(true)` and `->toEqual(false)` only when the checked value is always a bool, and `->toBe([])` /
//! `->toEqual('')` become `->toBeEmpty()` when it is always an array / a string.

use mago_syntax::cst::{Expression, Literal, MethodCall, Node, NodeKind};
use pun_problem::Safety;

use super::{
    Is, Trinary, argument_value, is_expect_chain, is_name, matcher_subject, report,
    set_call_arguments, type_is,
};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferToBeTrueFalse;

impl Rule for PreferToBeTrueFalse {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-be-true-false"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodCall(call) = node else {
            return;
        };
        if !is_expect_chain(call)
            || !(is_name(&call.method, "toBe") || is_name(&call.method, "toEqual"))
        {
            return;
        }
        let [argument] = call.argument_list.arguments.as_slice() else {
            return;
        };
        let Some(matcher) = matcher_for_literal(ctx, call, argument_value(argument)) else {
            return;
        };
        out.push(report(
            node_span(call),
            format!("Use `{matcher}()` instead of comparing with a literal."),
            Safety::Safe,
            set_call_arguments(call, matcher, &[]),
        ));
    }
}

/// `getMatcherForLiteral`.
fn matcher_for_literal(
    ctx: &Context<'_>,
    call: &MethodCall<'_>,
    value: &Expression<'_>,
) -> Option<&'static str> {
    let strict = is_name(&call.method, "toBe");
    let definitely =
        |kind| matcher_subject(call).is_some_and(|s| type_is(ctx, s, kind) == Trinary::Yes);
    let constant = match unparenthesize(value) {
        Expression::Literal(Literal::True(_)) => Some(b"true".to_vec()),
        Expression::Literal(Literal::False(_)) => Some(b"false".to_vec()),
        Expression::Literal(Literal::Null(_)) => Some(b"null".to_vec()),
        Expression::ConstantAccess(c) => {
            let name = c.name.value();
            Some(
                name.strip_prefix(b"\\")
                    .unwrap_or(name)
                    .to_ascii_lowercase(),
            )
        }
        _ => None,
    };
    match constant.as_deref() {
        Some(b"true") if strict || definitely(Is::Boolean) => return Some("toBeTrue"),
        Some(b"false") if strict || definitely(Is::Boolean) => return Some("toBeFalse"),
        Some(b"null") if strict => return Some("toBeNull"),
        _ => {}
    }
    match unparenthesize(value) {
        Expression::Array(a) if a.elements.is_empty() && definitely(Is::Array) => Some("toBeEmpty"),
        Expression::LegacyArray(a) if a.elements.is_empty() && definitely(Is::Array) => {
            Some("toBeEmpty")
        }
        Expression::Literal(Literal::String(s))
            if s.value == Some(b"") && definitely(Is::String) =>
        {
            Some("toBeEmpty")
        }
        _ => None,
    }
}
