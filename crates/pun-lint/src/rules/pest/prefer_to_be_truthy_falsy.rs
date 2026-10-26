//! `lint/pest/prefer-to-be-truthy-falsy`, ported from pest-plugin-rector's `SimplifyToBeTruthyFalsyRector`:
//! `expect((bool) $x)->toBeTrue()` and `expect(boolval($x))->toBeTrue()` become `expect($x)->toBeTruthy()`, and
//! `toBeFalse()` becomes `toBeFalsy()`.

use mago_syntax::cst::{Expression, Node, NodeKind, UnaryPrefixOperator};
use pun_problem::Safety;

use super::{argument_value, func_call, is_name, matcher_subject, rename, report, subject_holder};
use crate::rules::rector_code_quality::named;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferToBeTruthyFalsy;

impl Rule for PreferToBeTruthyFalsy {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-be-truthy-falsy"
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
        if !(is_name(&call.method, "toBeTrue") || is_name(&call.method, "toBeFalse")) {
            return;
        }
        let (Some(holder), Some(subject)) = (subject_holder(call), matcher_subject(call)) else {
            return;
        };
        let value = match unparenthesize(subject) {
            Expression::UnaryPrefix(cast)
                if matches!(
                    cast.operator,
                    UnaryPrefixOperator::BoolCast(..) | UnaryPrefixOperator::BooleanCast(..)
                ) =>
            {
                cast.operand
            }
            _ => match func_call(subject) {
                Some(f) if named(f, "boolval") => match f.argument_list.arguments.as_slice() {
                    [argument] => argument_value(argument),
                    _ => return,
                },
                _ => return,
            },
        };
        let matcher = if is_name(&call.method, "toBeTrue") {
            "toBeTruthy"
        } else {
            "toBeFalsy"
        };
        let mut edits = holder.set_arguments(ctx, value);
        edits.push(rename(call, matcher));
        out.push(report(
            node_span(call),
            format!("Use `{matcher}()` instead of casting to bool."),
            Safety::Safe,
            edits,
        ));
    }
}
