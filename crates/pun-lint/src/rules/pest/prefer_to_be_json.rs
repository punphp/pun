//! `lint/pest/prefer-to-be-json`, ported from pest-plugin-rector's `UseToBeJsonRector`: `expect(json_decode($string)
//! !== null)->toBeTrue()` becomes `expect($string)->toBeJson()`; `=== null`, `toBeFalse()` or a `->not` give
//! `->not->toBeJson()`. The new call is made on the chain's `expect()`, so what came between is dropped.

use mago_syntax::cst::{BinaryOperator, Expression, FunctionCall, Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::{
    Is, Trinary, expect_call, first_value, func_call, function_name, has_not, identifier,
    set_first_argument, type_is,
};
use crate::rules::rector_code_quality::is_constant;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferToBeJson;

impl Rule for PreferToBeJson {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-be-json"
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
        let method = match identifier(&call.method) {
            Some(m @ (b"toBeTrue" | b"toBeFalse")) => m,
            _ => return,
        };
        let Some(expect) = expect_call(call) else {
            return;
        };
        let Some(Expression::Binary(comparison)) =
            first_value(&expect.argument_list).map(unparenthesize)
        else {
            return;
        };
        let not_identical = match comparison.operator {
            BinaryOperator::NotIdentical(_) => true,
            BinaryOperator::Identical(_) => false,
            _ => return,
        };
        let Some(decode) = json_decode_call(comparison.lhs, comparison.rhs) else {
            return;
        };
        let Some(string) = first_value(&decode.argument_list) else {
            return;
        };
        if type_is(ctx, string, Is::String) != Trinary::Yes {
            return;
        }
        let valid = (not_identical == (method == b"toBeTrue")) != has_not(call);
        let mut edits = set_first_argument(ctx, &expect.argument_list, string);
        let not = if valid { "" } else { "->not" };
        edits.push(Edit::new(
            Span::new(node_span(expect).end, node_span(call).end),
            format!("{not}->toBeJson()"),
        ));
        out.push(Report::fixed(
            node_span(call),
            "Use `toBeJson()` instead of checking `json_decode()` against null.",
            Safety::Unsafe,
            edits,
        ));
    }
}

/// `extractJsonDecodeCall`: the call of `json_decode` (named exactly) on one side of the comparison, with `null` on the
/// other (the left side first).
fn json_decode_call<'a>(
    left: &'a Expression<'a>,
    right: &'a Expression<'a>,
) -> Option<&'a FunctionCall<'a>> {
    let call = match (func_call(left), func_call(right)) {
        (Some(f), _) if is_constant(right, "null") => f,
        (_, Some(f)) if is_constant(left, "null") => f,
        _ => return None,
    };
    (function_name(call) == Some(b"json_decode")).then_some(call)
}
