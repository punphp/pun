//! `lint/laravel/no-negated-contains`, ported from rector-laravel 2.5.0's
//! `AvoidNegatedCollectionContainsOrDoesntContainRector` (src/Rector/BooleanNot).
//!
//! `!$collection->contains(...)` becomes `$collection->doesntContain(...)`, and `!$collection->doesntContain(...)`
//! becomes `$collection->contains(...)`, when the collection is an `Illuminate\Support\Enumerable`.

use mago_syntax::cst::{Call, Expression, Node, NodeKind, UnaryPrefixOperator};

use super::object_type::is_object_type;
use super::{is_name, member_name, replace};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoNegatedContains;

impl Rule for NoNegatedContains {
    fn id(&self) -> &'static str {
        "lint/laravel/no-negated-contains"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::UnaryPrefix])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::UnaryPrefix(not) = node else { return };
        if !matches!(not.operator, UnaryPrefixOperator::Not(_)) {
            return;
        }
        let Expression::Call(Call::Method(call)) = unparenthesize(not.operand) else {
            return;
        };
        let Some(name) = member_name(&call.method) else {
            return;
        };
        let replacement = if is_name(name, "contains") {
            "doesntContain"
        } else if is_name(name, "doesntContain") {
            "contains"
        } else {
            return;
        };
        if !is_object_type(ctx, ancestors, call.object, "Illuminate\\Support\\Enumerable") {
            return;
        }
        let span = node_span(call);
        let method = node_span(&call.method);
        let code = format!(
            "{}{replacement}{}",
            &ctx.source[span.start..method.start],
            &ctx.source[method.end..span.end]
        );
        out.push(replace(
            node_span(not),
            format!("Use `{replacement}()` instead of negating `{}()`.", String::from_utf8_lossy(name)),
            code,
        ));
    }
}
