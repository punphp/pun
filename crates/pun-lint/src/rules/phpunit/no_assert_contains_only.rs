//! `lint/phpunit/no-assert-contains-only`, ported from rector-phpunit's `AssertContainsOnlyMethodCallRector`
//! (rules/PHPUnit110/Rector/CallLike).
//!
//! `assertContainsOnly('int', $a)` becomes `assertContainsOnlyInt($a)`, and likewise for each type name, unless the
//! `$isNativeType` argument may be `false`. Rector runs it only for PHPUnit 11.5 and later; Pun leaves that to the
//! rule's group.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::{Edit, Safety};

use super::{AnyCall, expr_text, is_test_case_call, new_call, string_value};
use crate::rules::laravel::get_arg;
use crate::rules::rector_code_quality::is_constant;
use crate::{Context, Report, Rule};

const TYPE_VALUE_TO_METHOD: &[(&str, &str)] = &[
    ("array", "assertContainsOnlyArray"),
    ("bool", "assertContainsOnlyBool"),
    ("boolean", "assertContainsOnlyBool"),
    ("callable", "assertContainsOnlyCallable"),
    ("double", "assertContainsOnlyFloat"),
    ("float", "assertContainsOnlyFloat"),
    ("int", "assertContainsOnlyInt"),
    ("integer", "assertContainsOnlyInt"),
    ("iterable", "assertContainsOnlyIterable"),
    ("null", "assertContainsOnlyNull"),
    ("numeric", "assertContainsOnlyNumeric"),
    ("object", "assertContainsOnlyObject"),
    ("real", "assertContainsOnlyFloat"),
    ("resource", "assertContainsOnlyResource"),
    ("resource (closed)", "assertContainsOnlyClosedResource"),
    ("scalar", "assertContainsOnlyScalar"),
    ("string", "assertContainsOnlyString"),
];

pub struct NoAssertContainsOnly;

impl Rule for NoAssertContainsOnly {
    fn id(&self) -> &'static str {
        "lint/phpunit/no-assert-contains-only"
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
        if !is_test_case_call(ctx, ancestors, &call) || !call.is_name(&["assertContainsOnly"]) {
            return;
        }
        let (Some(type_arg), Some(haystack)) = (
            get_arg(call.args, "type", 0),
            get_arg(call.args, "haystack", 1),
        ) else {
            return;
        };
        let Some(value) = string_value(type_arg.value()) else {
            return;
        };
        let Some((_, method)) = TYPE_VALUE_TO_METHOD.iter().find(|(k, _)| *k == value) else {
            return;
        };
        if let Some(native) = get_arg(call.args, "isNativeType", 2)
            && !is_constant(native.value(), "true")
            && !is_constant(native.value(), "null")
        {
            return;
        }
        let mut args = vec![expr_text(ctx, haystack.value())];
        if let Some(message) = get_arg(call.args, "message", 3) {
            args.push(expr_text(ctx, message.value()));
        }
        out.push(Report::fixed(
            call.span,
            format!("Use `{method}()` instead of `assertContainsOnly()`."),
            Safety::Safe,
            vec![Edit::new(call.span, new_call(ctx, &call, method, &args))],
        ));
    }
}
