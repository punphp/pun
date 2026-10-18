//! `lint/laravel/prefer-today`, ported from rector-laravel 2.5.0's `NowFuncWithStartOfDayMethodCallToTodayFuncRector`
//! (src/Rector/FuncCall).
//!
//! `now()->startOfDay()` becomes `today()`. As upstream, the arguments of both calls are dropped.

use mago_syntax::cst::{Call, Expression, Node, NodeKind};

use super::{is_name, member_name, replace};
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
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodCall(m) = node else { return };
        if !member_name(&m.method).is_some_and(|n| is_name(n, "startOfDay")) {
            return;
        }
        if !matches!(unparenthesize(m.object), Expression::Call(Call::Function(f)) if named(f, "now"))
        {
            return;
        }
        out.push(replace(
            node_span(m),
            "Use `today()` instead of `now()->startOfDay()`.".to_string(),
            "today()".to_string(),
        ));
    }
}
