//! `lint/laravel/no-negated-blank-filled`, ported from rector-laravel 2.7.3's
//! `NotFilledBlankFuncCallToBlankFilledFuncCallRector` (src/Rector/FuncCall).
//!
//! `!filled($x)` becomes `blank($x)`, and `!blank($x)` becomes `filled($x)`.

use super::{argument_texts, call, replace};
use crate::rules::rector_code_quality::named;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};
use mago_syntax::cst::{Call, Expression, Node, NodeKind, UnaryPrefixOperator};

pub struct NoNegatedBlankFilled;

impl Rule for NoNegatedBlankFilled {
    fn id(&self) -> &'static str {
        "lint/laravel/no-negated-blank-filled"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::UnaryPrefix])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::UnaryPrefix(not) = node else { return };
        if !matches!(not.operator, UnaryPrefixOperator::Not(_)) {
            return;
        }
        let Expression::Call(Call::Function(f)) = unparenthesize(not.operand) else {
            return;
        };
        let method = if named(f, "filled") {
            "blank"
        } else if named(f, "blank") {
            "filled"
        } else {
            return;
        };
        out.push(replace(
            node_span(not),
            format!("Use `{method}()` instead of negating its opposite."),
            call(method, &argument_texts(ctx, &f.argument_list)),
        ));
    }
}
