//! `lint/phpunit/prefer-specific-true-false-assertion`, ported from rector-phpunit's
//! `AssertTrueFalseToSpecificMethodRector` (rules/CodeQuality/Rector/MethodCall).
//!
//! `assertTrue(is_dir($a))` becomes `assertDirectoryExists($a)`, and likewise for the functions upstream lists and for
//! `empty()`, with `assertFalse`, `assertNotTrue` and `assertNotFalse`. A name PHPUnit's `Assert` class lacks (where
//! the codebase knows the class) takes its successor (`assertFileNotExists` becomes `assertFileDoesNotExist`).

use mago_codex::ttype::atomic::TAtomic;
use mago_syntax::cst::{Call, Construct, Expression, Identifier, Node, NodeKind};
use pun_problem::Safety;

use super::{
    ASSERT, AnyCall, NewArg, expr_text, is_test_case_call_named, old_args, rewrite_args, text,
};
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

const FUNCTION_NAME_WITH_ASSERT_METHOD_NAMES: &[(&str, &str, &str)] = &[
    ("is_readable", "assertIsReadable", "assertNotIsReadable"),
    (
        "array_key_exists",
        "assertArrayHasKey",
        "assertArrayNotHasKey",
    ),
    ("array_search", "assertContains", "assertNotContains"),
    ("in_array", "assertContains", "assertNotContains"),
    ("empty", "assertEmpty", "assertNotEmpty"),
    ("file_exists", "assertFileExists", "assertFileNotExists"),
    (
        "is_dir",
        "assertDirectoryExists",
        "assertDirectoryNotExists",
    ),
    ("is_infinite", "assertInfinite", "assertFinite"),
    ("is_null", "assertNull", "assertNotNull"),
    ("is_writable", "assertIsWritable", "assertNotIsWritable"),
    ("is_nan", "assertNan", ""),
    ("is_a", "assertInstanceOf", "assertNotInstanceOf"),
    (
        "str_contains",
        "assertStringContainsString",
        "assertStringNotContainsString",
    ),
];

const RENAMED_ASSERT_METHOD_NAMES: &[(&str, &str)] = &[
    ("assertFileNotExists", "assertFileDoesNotExist"),
    ("assertDirectoryNotExists", "assertDirectoryDoesNotExist"),
];

pub struct PreferSpecificTrueFalseAssertion;

impl Rule for PreferSpecificTrueFalseAssertion {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-specific-true-false-assertion"
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
        let names = [
            "assertTrue",
            "assertFalse",
            "assertNotTrue",
            "assertNotFalse",
        ];
        if !is_test_case_call_named(ctx, ancestors, &call, &names) {
            return;
        }
        let Some(first) = call.args().first().copied() else {
            return;
        };
        let value = unparenthesize(first.value());
        // the function's arguments, or the operand of `empty()`
        let (function, inner): (&str, Result<Vec<_>, &Expression<'_>>) = match value {
            Expression::Construct(Construct::Empty(empty)) => ("empty", Err(empty.value)),
            Expression::Call(Call::Function(f)) => {
                let name = match f.function {
                    Expression::Identifier(Identifier::Local(id)) => id.value,
                    Expression::Identifier(Identifier::FullyQualified(id)) => &id.value[1..],
                    _ => return,
                };
                // a name that is not UTF-8 is none of the functions
                let name = std::str::from_utf8(name).unwrap_or_default();
                (name, Ok(f.argument_list.arguments.iter().collect()))
            }
            _ => return,
        };
        let Some(&(function, assert, not_assert)) = FUNCTION_NAME_WITH_ASSERT_METHOD_NAMES
            .iter()
            .find(|(f, _, _)| *f == function)
        else {
            return;
        };
        if function == "is_a" {
            let Some(subject) = inner.as_ref().ok().and_then(|args| args.first()) else {
                return;
            };
            if is_string_type(ctx, subject.value()) {
                return;
            }
        }
        let written = call.name().unwrap_or_default();
        let mut name = None;
        if written == b"assertTrue" || written == b"assertNotFalse" {
            name = Some(existing_assert_method_name(ctx, assert));
        }
        if !not_assert.is_empty() && (written == b"assertFalse" || written == b"assertNotTrue") {
            name = Some(existing_assert_method_name(ctx, not_assert));
        }
        let new: Vec<NewArg> = match &inner {
            Err(empty) => std::iter::once(NewArg::New(expr_text(ctx, empty).to_string()))
                .chain(old_args(&call, 1))
                .collect(),
            Ok(args) => {
                let moved = |i: usize| NewArg::New(text(ctx, args[i]).to_string());
                let front: Vec<NewArg> = match function {
                    "in_array" | "array_search" if args.len() == 3 => vec![moved(0), moved(1)],
                    "is_a" | "str_contains" => {
                        if args.len() < 2 {
                            return;
                        }
                        vec![moved(1), moved(0)]
                    }
                    _ => (0..args.len()).map(moved).collect(),
                };
                front.into_iter().chain(old_args(&call, 1)).collect()
            }
        };
        let message = format!("Use a dedicated assertion instead of asserting `{function}()`.");
        out.push(Report::fixed(
            call.span,
            &message,
            Safety::Unsafe,
            rewrite_args(ctx, &call, name, &new),
        ));
    }
}

/// `resolveExistingAssertMethodName`: the name, unless the codebase knows PHPUnit's `Assert` class without it.
fn existing_assert_method_name(ctx: &Context<'_>, method: &'static str) -> &'static str {
    let facts = facts(ctx);
    if facts.codebase.get_class_like(ASSERT.as_bytes()).is_none()
        || facts
            .codebase
            .method_exists(ASSERT.as_bytes(), method.as_bytes())
    {
        return method;
    }
    RENAMED_ASSERT_METHOD_NAMES
        .iter()
        .find(|(old, _)| *old == method)
        .map_or(method, |(_, new)| new)
}

/// `$type instanceof StringType`: a string type, a literal string or a class string, not a union.
fn is_string_type(ctx: &Context<'_>, expr: &Expression<'_>) -> bool {
    let facts = facts(ctx);
    facts
        .full_type(node_span(expr))
        .is_some_and(|t| matches!(t.types.as_ref(), [TAtomic::Scalar(s)] if s.is_any_string()))
}
