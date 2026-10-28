//! `lint/laravel/prefer-environment-check`, ported from rector-laravel 2.5.0's `AppEnvironmentComparisonToParameterRector`
//! (src/Rector/Expr).
//!
//! A call of `environment()` without arguments, on an `Illuminate\Contracts\Foundation\Application` or through the
//! `Illuminate\Support\Facades\App` facade or a class named `App`, compared with `===`, `==`, `!==`, `!=` or `<>` to
//! something (on either side), or passed to `in_array()` as the needle, takes that something as its argument:
//! `$app->environment() === 'staging'` becomes `$app->environment('staging')`, and a negated comparison gets a `!`.
//! Compared with `'local'` or `'production'`, a facade call or a call on an `Illuminate\Foundation\Application`
//! becomes `isLocal()` or `isProduction()` instead.

use mago_syntax::cst::{BinaryOperator, Call, Expression, Node, NodeKind};

use super::object_type::is_object_type;
use super::{get_arg, is_name, member_name, replace};
use crate::rules::rector_code_quality::{named, text};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferEnvironmentCheck;

impl Rule for PreferEnvironmentCheck {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-environment-check"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall, NodeKind::Binary])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let valid = |e: &Expression<'_>| valid_call(ctx, ancestors, e);
        let (call, other, negated) = match node {
            Node::FunctionCall(f) => {
                if !named(f, "in_array") {
                    return;
                }
                let (Some(needle), Some(haystack)) = (
                    get_arg(&f.argument_list, "needle", 0),
                    get_arg(&f.argument_list, "haystack", 1),
                ) else {
                    return;
                };
                if !valid(needle.value()) {
                    return;
                }
                (needle.value(), haystack.value(), false)
            }
            Node::Binary(b) => {
                let negated = match b.operator {
                    BinaryOperator::Identical(_) | BinaryOperator::Equal(_) => false,
                    BinaryOperator::NotIdentical(_)
                    | BinaryOperator::NotEqual(_)
                    | BinaryOperator::AngledNotEqual(_) => true,
                    _ => return,
                };
                if valid(b.lhs) {
                    (b.lhs, b.rhs, negated)
                } else if valid(b.rhs) {
                    (b.rhs, b.lhs, negated)
                } else {
                    return;
                }
            }
            _ => return,
        };
        let call = unparenthesize(call);
        let other = unparenthesize(other);
        let (method, list) = match call {
            Expression::Call(Call::Method(m)) => (&m.method, &m.argument_list),
            Expression::Call(Call::StaticMethod(s)) => (&s.method, &s.argument_list),
            _ => unreachable!("valid_call checked the call"),
        };
        let replacement = match other {
            Expression::Literal(mago_syntax::cst::Literal::String(s)) => {
                method_name(ctx, ancestors, call, s.value)
            }
            _ => None,
        };
        let (method, list) = (node_span(method), node_span(list));
        let span = node_span(call);
        let code = match replacement {
            Some(name) => format!(
                "{}{name}{}",
                &ctx.source[span.start..method.start],
                &ctx.source[method.end..span.end]
            ),
            None => format!(
                "{}({})",
                &ctx.source[span.start..list.start],
                text(ctx, other)
            ),
        };
        let code = if negated { format!("!{code}") } else { code };
        out.push(replace(
            node_span(&node),
            "Pass the environment to `environment()` instead of comparing its result.".to_string(),
            code,
        ));
    }
}

/// `validMethodCall`: `environment()` without arguments on an application or through the `App` facade.
fn valid_call(ctx: &Context<'_>, ancestors: &[Node<'_, '_>], expr: &Expression<'_>) -> bool {
    match unparenthesize(expr) {
        Expression::Call(Call::Method(m)) => {
            member_name(&m.method).is_some_and(|n| is_name(n, "environment"))
                && m.argument_list.arguments.is_empty()
                && is_object_type(
                    ctx,
                    ancestors,
                    m.object,
                    "Illuminate\\Contracts\\Foundation\\Application",
                )
        }
        Expression::Call(Call::StaticMethod(s)) => {
            member_name(&s.method).is_some_and(|n| is_name(n, "environment"))
                && s.argument_list.arguments.is_empty()
                && (is_object_type(ctx, ancestors, s.class, "Illuminate\\Support\\Facades\\App")
                    || is_object_type(ctx, ancestors, s.class, "App"))
        }
        _ => false,
    }
}

/// `getMethodName`: `isLocal` or `isProduction` for a string literal `'local'` or `'production'`, except on a method
/// call whose object is no `Illuminate\Foundation\Application`.
fn method_name(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    call: &Expression<'_>,
    value: Option<&[u8]>,
) -> Option<&'static str> {
    if let Expression::Call(Call::Method(m)) = call
        && !is_object_type(ctx, ancestors, m.object, "Illuminate\\Foundation\\Application")
    {
        return None;
    }
    match value? {
        b"local" => Some("isLocal"),
        b"production" => Some("isProduction"),
        _ => None,
    }
}
