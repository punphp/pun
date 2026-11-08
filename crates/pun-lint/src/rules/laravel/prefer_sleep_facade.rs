//! `lint/laravel/prefer-sleep-facade`, ported from rector-laravel 2.7.3's `SleepFuncToSleepStaticCallRector`
//! (src/Rector/FuncCall).
//!
//! A `sleep(...)` or `usleep(...)` statement becomes `\Illuminate\Support\Sleep::sleep(...)` or `::usleep(...)`. A call
//! whose result is used is left alone.

use mago_syntax::cst::{Call, Expression, Node, NodeKind};

use super::{argument_texts, call, replace};
use crate::rules::rector_code_quality::named;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferSleepFacade;

impl Rule for PreferSleepFacade {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-sleep-facade"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::ExpressionStatement])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::ExpressionStatement(statement) = node else {
            return;
        };
        let Expression::Call(Call::Function(f)) = unparenthesize(statement.expression) else {
            return;
        };
        let method = if named(f, "sleep") {
            "sleep"
        } else if named(f, "usleep") {
            "usleep"
        } else {
            return;
        };
        out.push(replace(
            node_span(f),
            format!("Use `Sleep::{method}()` instead of `{method}()`."),
            call(
                &format!("\\Illuminate\\Support\\Sleep::{method}"),
                &argument_texts(ctx, &f.argument_list),
            ),
        ));
    }
}
