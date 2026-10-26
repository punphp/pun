//! `lint/phpunit/prefer-assert-array-has-key`, ported from rector-phpunit's `AssertIssetToSpecificMethodRector`
//! (rules/CodeQuality/Rector/MethodCall).
//!
//! `assertTrue(isset($a['k']))` becomes `assertArrayHasKey('k', $a)`, and `assertFalse` becomes
//! `assertArrayNotHasKey`, unless the array is a superglobal or an `ArrayAccess` object.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_syntax::cst::{Construct, Expression, Node, NodeKind, Variable};
use pun_problem::Safety;

use super::{
    AnyCall, NewArg, arg_value, expr_text, is_test_case_call_named, old_args, renamed, report,
    rewrite_args,
};
use crate::rules::laravel::is_a;
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

const SUPER_GLOBAL_VARIABLE_NAMES: &[&str] = &[
    "GLOBALS", "_SERVER", "_GET", "_POST", "_FILES", "_COOKIE", "_SESSION", "_REQUEST", "_ENV",
];

const RENAME: &[(&str, &str)] = &[
    ("assertTrue", "assertArrayHasKey"),
    ("assertFalse", "assertArrayNotHasKey"),
];

pub struct PreferAssertArrayHasKey;

impl Rule for PreferAssertArrayHasKey {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-assert-array-has-key"
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
        let Some(call) = AnyCall::of(node) else {
            return;
        };
        if !is_test_case_call_named(ctx, ancestors, &call, &["assertTrue", "assertFalse"]) {
            return;
        }
        let Some(first) = call.args().first().copied() else {
            return;
        };
        let Expression::Construct(Construct::Isset(isset)) = unparenthesize(arg_value(first))
        else {
            return;
        };
        let Some(Expression::ArrayAccess(access)) = isset.values.first().map(|e| unparenthesize(e))
        else {
            return;
        };
        if let Expression::Variable(Variable::Direct(v)) = unparenthesize(access.array)
            && SUPER_GLOBAL_VARIABLE_NAMES
                .iter()
                .any(|n| v.name[1..] == *n.as_bytes())
        {
            return;
        }
        if is_array_access(ctx, access.array) {
            return;
        }
        let Some(name) = renamed(&call, RENAME) else {
            return;
        };
        let new: Vec<NewArg> = [
            NewArg::New(expr_text(ctx, access.index).to_string()),
            NewArg::New(expr_text(ctx, access.array).to_string()),
        ]
        .into_iter()
        .chain(old_args(&call, 1))
        .collect();
        out.push(report(
            call.span,
            &format!("Use `{name}()` instead of asserting `isset()`."),
            Safety::Safe,
            rewrite_args(ctx, &call, Some(name), &new),
        ));
    }
}

/// `isObjectType($expr, new ObjectType('ArrayAccess'))`: every type Mago gives the expression is an object of a class
/// that implements `ArrayAccess`. Mago narrows a value an earlier `assertTrue(isset(...))` asserted on to `never`, where
/// PHPStan keeps its class, so the native type (without PHPUnit's `@phpstan-assert`) counts too.
fn is_array_access(ctx: &Context<'_>, expr: &Expression<'_>) -> bool {
    let facts = facts(ctx);
    let span = node_span(expr);
    let all_array_access = |t: &mago_codex::ttype::union::TUnion| {
        !t.types.is_empty()
            && t.types.iter().all(|a| match a {
                TAtomic::Object(TObject::Named(n)) => {
                    is_a(ctx, &n.name.to_string(), &["ArrayAccess"])
                }
                _ => false,
            })
    };
    facts.full_type(span).is_some_and(all_array_access)
        || facts.native_type(span).is_some_and(all_array_access)
}
