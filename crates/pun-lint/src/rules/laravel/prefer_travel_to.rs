//! `lint/laravel/prefer-travel-to`, ported from rector-laravel 2.5.0's `CarbonSetTestNowToTravelToRector`
//! (src/Rector/StaticCall).
//!
//! In a subclass of `Illuminate\Foundation\Testing\TestCase`, `Carbon::setTestNow($date)` (on `Carbon\Carbon`,
//! `Carbon\CarbonImmutable` or `Illuminate\Support\Carbon`) becomes `$this->travelTo($date)`, and
//! `Carbon::setTestNow()` becomes `$this->travelTo(null)`. Pun leaves a call where `$this` is not available: in a
//! static method or a static closure.

use mago_syntax::cst::{Modifier, Node, NodeKind};

use super::{argument_texts, call, class_name, is_a, is_name, member_name, replace};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

const TEST_CASE: &str = "Illuminate\\Foundation\\Testing\\TestCase";

pub struct PreferTravelTo;

impl Rule for PreferTravelTo {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-travel-to"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::StaticMethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::StaticMethodCall(s) = node else {
            return;
        };
        if !member_name(&s.method).is_some_and(|n| is_name(n, "setTestNow")) {
            return;
        }
        if !in_test_case(ctx, ancestors) {
            return;
        }
        let Some(class) = class_name(s.class, ancestors) else {
            return;
        };
        let carbon = [
            "Carbon\\Carbon",
            "Carbon\\CarbonImmutable",
            "Illuminate\\Support\\Carbon",
        ];
        if !is_a(ctx, &class, &carbon) {
            return;
        }
        let static_scope = ancestors.iter().any(|a| match a {
            Node::Method(m) => m.modifiers.iter().any(|m| matches!(m, Modifier::Static(_))),
            Node::Closure(c) => c.r#static.is_some(),
            Node::ArrowFunction(f) => f.r#static.is_some(),
            _ => false,
        });
        if static_scope {
            return;
        }
        let args = argument_texts(ctx, &s.argument_list);
        let args = if args.is_empty() { vec!["null"] } else { args };
        out.push(replace(
            node_span(s),
            "Use `$this->travelTo()` instead of `setTestNow()` in a test.".to_string(),
            call("$this->travelTo", &args),
        ));
    }
}

/// `$scope->getClassReflection()->isSubclassOfClass(TestCase)`: the nearest class extends Laravel's `TestCase`, by
/// its `extends` name or through the classes the codebase knows.
fn in_test_case(ctx: &Context<'_>, ancestors: &[Node<'_, '_>]) -> bool {
    let Some(class) = ancestors.iter().rev().find_map(|a| match a {
        Node::Class(c) => Some(c),
        _ => None,
    }) else {
        return false;
    };
    class
        .extends
        .as_ref()
        .and_then(|e| e.types.first())
        .is_some_and(|parent| {
            is_a(
                ctx,
                &super::names::resolve_class_name(ancestors, parent),
                &[TEST_CASE],
            )
        })
}
