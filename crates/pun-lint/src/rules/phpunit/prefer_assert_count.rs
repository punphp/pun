//! `lint/phpunit/prefer-assert-count`, ported from rector-phpunit's
//! `AssertCompareOnCountableWithMethodToAssertCountRector` (rules/CodeQuality/Rector/MethodCall).
//!
//! `assertSame(5, count($a))` (or `sizeof()`, `iterator_count()`, or `$a->count()` on a `Countable`) becomes
//! `assertCount(5, $a)`; `assertNotSame` and `assertNotEquals` become `assertNotCount`. A `Generator` is left alone.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_syntax::cst::{Call, Expression, Node, NodeKind};
use pun_problem::Safety;

use super::{AnyCall, NewArg, arg_value, expr_text, is_test_case_call_named, report, rewrite_args};
use crate::rules::laravel::is_a;
use crate::rules::rector_code_quality::named;
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferAssertCount;

impl Rule for PreferAssertCount {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-assert-count"
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
            "assertSame",
            "assertNotSame",
            "assertEquals",
            "assertNotEquals",
        ];
        if !is_test_case_call_named(ctx, ancestors, &call, &names) {
            return;
        }
        let args = call.args();
        if args.len() < 2 {
            return;
        }
        let counted = match unparenthesize(arg_value(args[1])) {
            Expression::Call(Call::Function(f))
                if named(f, "count") || named(f, "sizeof") || named(f, "iterator_count") =>
            {
                let Some(first) = f.argument_list.arguments.first() else {
                    return;
                };
                let value = arg_value(first);
                if all_objects_of(ctx, value, "Generator") {
                    return;
                }
                value
            }
            Expression::Call(Call::Method(m))
                if m.argument_list.arguments.is_empty()
                    && super::AnyCall::of(Node::MethodCall(m))
                        .is_some_and(|c| c.is_name(&["count"])) =>
            {
                if !all_objects_of(ctx, m.object, "Countable") {
                    return;
                }
                m.object
            }
            _ => return,
        };
        let name = if call.is_name(&["assertSame", "assertEquals"]) {
            "assertCount"
        } else {
            "assertNotCount"
        };
        let new: Vec<NewArg> = (0..args.len())
            .map(|i| {
                if i == 1 {
                    NewArg::New(expr_text(ctx, counted).to_string())
                } else {
                    NewArg::Old(i)
                }
            })
            .collect();
        out.push(report(
            call.span,
            &format!("Use `{name}()` to compare a count."),
            Safety::Safe,
            rewrite_args(ctx, &call, Some(name), &new),
        ));
    }
}

/// `ObjectType($class)->isSuperTypeOf($type)->yes()` on the type Mago gives `expr`: every part is an object of
/// `class` or a subclass.
fn all_objects_of(ctx: &Context<'_>, expr: &Expression<'_>, class: &str) -> bool {
    let facts = facts(ctx);
    facts
        .full_type(node_span(unparenthesize(expr)))
        .is_some_and(|t| {
            !t.types.is_empty()
                && t.types.iter().all(|a| match a {
                    TAtomic::Object(TObject::Named(n)) => is_a(ctx, &n.name.to_string(), &[class]),
                    _ => false,
                })
        })
}
