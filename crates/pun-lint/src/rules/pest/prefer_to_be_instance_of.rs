//! `lint/pest/prefer-to-be-instance-of`, ported from pest-plugin-rector's `UseInstanceOfMatcherRector`: `expect($x
//! instanceof User)->toBeTrue()` becomes `expect($x)->toBeInstanceOf(User::class)`.

use mago_syntax::cst::{BinaryOperator, Expression, Node, NodeKind};
use pun_problem::Safety;

use super::{first_value, is_name, set_call_arguments, subject_holder};
use crate::rules::phpunit::expr_text;
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
        if !is_name(&call.method, "toBeTrue") {
            return;
        }
        let Some(holder) = subject_holder(call) else {
            return;
        };
        let Some(Expression::Binary(check)) = first_value(holder.arguments).map(unparenthesize)
        else {
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
        let class = format!("{}::class", expr_text(ctx, check.rhs));
        let mut edits = holder.set_arguments(ctx, check.lhs);
        edits.extend(set_call_arguments(call, "toBeInstanceOf", &[class]));
        out.push(Report::fixed(
            node_span(call),
            "Use `toBeInstanceOf()` instead of checking `instanceof`.",
            Safety::Safe,
            edits,
        ));
    }
}
