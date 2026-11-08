//! `lint/laravel/prefer-today`, ported from rector-laravel 2.7.3's `NowFuncWithStartOfDayMethodCallToTodayFuncRector`
//! (src/Rector/FuncCall).
//!
//! `now()->startOfDay()` becomes `today()`, with the arguments of `now()` (`now('UTC')->startOfDay()` becomes
//! `today('UTC')`). As upstream, the arguments of `startOfDay()` are dropped.

use mago_syntax::cst::{Call, Expression, Node, NodeKind};

use super::{argument_texts, call, is_name, member_name, replace};
use crate::rules::rector_code_quality::named;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferToday;

impl Rule for PreferToday {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-today"
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
        let Node::MethodCall(m) = node else { return };
        if !member_name(&m.method).is_some_and(|n| is_name(n, "startOfDay")) {
            return;
        }
        let Expression::Call(Call::Function(now)) = unparenthesize(m.object) else {
            return;
        };
        if !named(now, "now") {
            return;
        }
        out.push(replace(
            node_span(m),
            "Use `today()` instead of `now()->startOfDay()`.".to_string(),
            call("today", &argument_texts(ctx, &now.argument_list)),
        ));
    }
}
