//! `lint/pest/prefer-to-be-lowercase`, ported from pest-plugin-rector's `UseToBeLowercaseRector`:
//! `expect(strtolower($value) === $value)->toBeTrue()` (either way round) becomes `expect($value)->toBeLowercase()`
//! when `$value` is known to be a string.

use mago_syntax::cst::{BinaryOperator, Expression, Node, NodeKind};
use pun_problem::Safety;

use super::{Is, Trinary, first_value, func_call, is_name, rename, subject_holder, type_is};
use crate::rules::rector_code_quality::named;
use crate::syntax::same_expression;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferToBeLowercase;

impl Rule for PreferToBeLowercase {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-be-lowercase"
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
        if !is_name(&call.method, "toBeTrue") {
            return;
        }
        let Some(holder) = subject_holder(call) else {
            return;
        };
        let Some(Expression::Binary(identical)) = first_value(holder.arguments).map(unparenthesize)
        else {
            return;
        };
        if !matches!(identical.operator, BinaryOperator::Identical(_)) {
            return;
        }
        fn case<'a>(e: &'a Expression<'a>) -> Option<Option<&'a Expression<'a>>> {
            func_call(e)
                .filter(|f| named(f, "strtolower"))
                .map(|f| first_value(&f.argument_list))
        }
        let value = match (case(identical.lhs), case(identical.rhs)) {
            (Some(Some(inner)), _) if same_expression(ctx.source, inner, identical.rhs) => {
                identical.rhs
            }
            (_, Some(Some(inner))) if same_expression(ctx.source, identical.lhs, inner) => {
                identical.lhs
            }
            _ => return,
        };
        if type_is(ctx, value, Is::String) != Trinary::Yes {
            return;
        }
        let mut edits = holder.set_arguments(ctx, value);
        edits.push(rename(call, "toBeLowercase"));
        out.push(Report::fixed(
            node_span(call),
            "Use `toBeLowercase()` instead of comparing with `strtolower()`.",
            Safety::Unsafe,
            edits,
        ));
    }
}
