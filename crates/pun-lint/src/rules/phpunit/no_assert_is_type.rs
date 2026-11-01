//! `lint/phpunit/no-assert-is-type`, ported from rector-phpunit's `AssertIsTypeMethodCallRector`
//! (rules/PHPUnit120/Rector/Class_).
//!
//! `$this->isType('string')` becomes `$this->isString()`, and likewise for each type name PHPUnit's `isType()` takes.
//! Rector runs it only for PHPUnit 11.5 and later; Pun leaves that to the rule's group.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::{Edit, Safety};

use super::{AnyCall, is_test_case_call, new_call, string_value};
use crate::rules::laravel::get_arg;
use crate::{Context, Report, Rule};

const IS_TYPE_VALUE_TO_METHOD: &[(&str, &str)] = &[
    ("array", "isArray"),
    ("bool", "isBool"),
    ("boolean", "isBool"),
    ("callable", "isCallable"),
    ("double", "isFloat"),
    ("float", "isFloat"),
    ("integer", "isInt"),
    ("int", "isInt"),
    ("iterable", "isIterable"),
    ("null", "isNull"),
    ("numeric", "isNumeric"),
    ("object", "isObject"),
    ("real", "isFloat"),
    ("resource", "isResource"),
    ("resource (closed)", "isClosedResource"),
    ("scalar", "isScalar"),
    ("string", "isString"),
];

pub struct NoAssertIsType;

impl Rule for NoAssertIsType {
    fn id(&self) -> &'static str {
        "lint/phpunit/no-assert-is-type"
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
        if !call.is_name(&["isType"]) || !is_test_case_call(ctx, ancestors, &call) {
            return;
        }
        if call.args.arguments.len() != 1 {
            return;
        }
        let Some(arg) = get_arg(call.args, "type", 0) else {
            return;
        };
        let Some(value) = string_value(arg.value()) else {
            return;
        };
        let Some((_, method)) = IS_TYPE_VALUE_TO_METHOD.iter().find(|(k, _)| *k == value) else {
            return;
        };
        out.push(Report::fixed(
            call.span,
            format!("Use `{method}()` instead of `isType()`."),
            Safety::Safe,
            vec![Edit::new(call.span, new_call(ctx, &call, method, &[]))],
        ));
    }
}
