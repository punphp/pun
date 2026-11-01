//! `lint/laravel/prefer-status-assertion-method`, ported from rector-laravel 2.5.0's `AssertStatusToAssertMethodRector`
//! (src/Rector/MethodCall).
//!
//! On an `Illuminate\Testing\TestResponse`, `assertStatus(200)` becomes `assertOk()`, and the same for the other
//! statuses with their own assertion (204, 401, 403, 404, 405, 410, 422, 500, 503). The one argument is any
//! expression whose type is an integer with one known value (a constant counts). With several possible values
//! (`$a ? 404 : 403`), Rector asserts the first one PHPStan lists, which changes what the test checks; Pun leaves the
//! call. The arguments go, so php-parser prints the call anew (`$receiver->assertOk()`), without the line break or
//! the parentheses that stood around the receiver.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_syntax::cst::{Node, NodeKind};
use pun_problem::Safety;

use super::object_type::is_object_type;
use super::print::reprinted_call;
use super::{is_name, member_name};
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferStatusAssertionMethod;

impl Rule for PreferStatusAssertionMethod {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-status-assertion-method"
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
        let Node::MethodCall(call) = node else { return };
        if !member_name(&call.method).is_some_and(|n| is_name(n, "assertStatus")) {
            return;
        }
        let [argument] = call.argument_list.arguments.as_slice() else {
            return;
        };
        if !is_object_type(
            ctx,
            ancestors,
            call.object,
            "Illuminate\\Testing\\TestResponse",
        ) {
            return;
        }
        let facts = facts(ctx);
        let t = facts.full_type(node_span(unparenthesize(argument.value())));
        // `isInteger()->yes()` and `getConstantScalarValues()`, one value
        let values: Option<Vec<i64>> = t.and_then(|t| {
            (t.types.iter())
                .map(|a| match a {
                    TAtomic::Scalar(TScalar::Integer(i)) => i.get_literal_value(),
                    _ => None,
                })
                .collect()
        });
        let Some(&[value]) = values.as_deref() else {
            return;
        };
        let method = match value {
            200 => "assertOk",
            204 => "assertNoContent",
            401 => "assertUnauthorized",
            403 => "assertForbidden",
            404 => "assertNotFound",
            405 => "assertMethodNotAllowed",
            410 => "assertGone",
            422 => "assertUnprocessable",
            500 => "assertInternalServerError",
            503 => "assertServiceUnavailable",
            _ => return,
        };
        out.push(Report::fixed(
            node_span(call),
            format!("Use `{method}()` instead of `assertStatus({value})`."),
            Safety::Safe,
            reprinted_call(
                ctx.source,
                call.object,
                node_span(call).end,
                &format!("->{method}()"),
            ),
        ));
    }
}
