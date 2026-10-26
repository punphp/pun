//! `lint/pest/prefer-to-be-instance-of`, ported from pest-plugin-rector's `UseInstanceOfMatcherRector`: `expect($x
//! instanceof User)->toBeTrue()` becomes `expect($x)->toBeInstanceOf(User::class)`.

use mago_syntax::cst::{BinaryOperator, Expression, Node, NodeKind};
use pun_problem::Safety;

use super::{
    is_expect_chain, is_name, matcher_subject, report, set_call_arguments, subject_holder,
    value_text,
};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferToBeInstanceOf;

impl Rule for PreferToBeInstanceOf {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-be-instance-of"
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
        let Some(Expression::Binary(check)) = matcher_subject(call).map(unparenthesize) else {
            return;
        };
        // The class must be a name: php-parser reads `self`, `static` and `parent` as names too.
        if !matches!(check.operator, BinaryOperator::Instanceof(_))
            || !matches!(
                check.rhs,
                Expression::Identifier(_)
                    | Expression::Self_(_)
                    | Expression::Static(_)
                    | Expression::Parent(_)
            )
        {
            return;
        }
        let Some(holder) = subject_holder(call) else {
            return;
        };
        let class = format!("{}::class", value_text(ctx, check.rhs));
        let mut edits = holder.set_arguments(ctx, check.lhs);
        edits.extend(set_call_arguments(call, "toBeInstanceOf", &[class]));
        out.push(report(
            node_span(call),
            "Use `toBeInstanceOf()` instead of checking `instanceof`.".to_string(),
            Safety::Safe,
            edits,
        ));
    }
}
