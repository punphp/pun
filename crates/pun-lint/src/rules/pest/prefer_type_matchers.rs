//! `lint/pest/prefer-type-matchers`, ported from pest-plugin-rector's `UseTypeMatchersRector`:
//! `expect(is_array($x))->toBeTrue()` becomes `expect($x)->toBeArray()`, and the same for the other `is_*()` functions
//! of `TYPE_FUNCTION_TO_MATCHER`.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::Safety;

use super::{
    argument_value, func_call, function_name, is_expect_chain, is_name, matcher_subject, rename,
    report, subject_holder,
};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferTypeMatchers;

impl Rule for PreferTypeMatchers {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-type-matchers"
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
        if !is_expect_chain(call) || !is_name(&call.method, "toBeTrue") {
            return;
        }
        let (Some(holder), Some(subject)) = (subject_holder(call), matcher_subject(call)) else {
            return;
        };
        let Some(function) = func_call(subject) else {
            return;
        };
        let Some((_, matcher)) = function_name(function).and_then(|name| {
            TYPE_FUNCTION_TO_MATCHER
                .iter()
                .find(|(f, _)| f.as_bytes() == name)
        }) else {
            return;
        };
        let [argument] = function.argument_list.arguments.as_slice() else {
            return;
        };
        let mut edits = holder.set_arguments(ctx, argument_value(argument));
        edits.push(rename(call, matcher));
        out.push(report(
            node_span(call),
            format!("Use `{matcher}()` instead of checking the type with a function."),
            Safety::Unsafe,
            edits,
        ));
    }
}

/// `TYPE_FUNCTION_TO_MATCHER`.
const TYPE_FUNCTION_TO_MATCHER: [(&str, &str); 14] = [
    ("is_array", "toBeArray"),
    ("is_string", "toBeString"),
    ("is_int", "toBeInt"),
    ("is_integer", "toBeInt"),
    ("is_float", "toBeFloat"),
    ("is_double", "toBeFloat"),
    ("is_bool", "toBeBool"),
    ("is_numeric", "toBeNumeric"),
    ("is_callable", "toBeCallable"),
    ("is_iterable", "toBeIterable"),
    ("is_object", "toBeObject"),
    ("is_resource", "toBeResource"),
    ("is_scalar", "toBeScalar"),
    ("is_null", "toBeNull"),
];
