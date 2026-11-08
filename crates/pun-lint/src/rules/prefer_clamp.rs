//! `lint/php-8.6/prefer-clamp`, ported from Rector 2.8.4's `MinMaxToClampRector` (rules/Php86/Rector/FuncCall).
//!
//! `max($min, min($max, $value))`, `min($max, max($min, $value))` and their argument orders become
//! `clamp($value, $min, $max)`. The inner call must have one known bound and one value; Rector asks PHPStan whether a
//! bound's native type is a constant scalar, Pun knows it where the syntax does (see `lint_b_helpers::constant`).

use mago_syntax::cst::{Argument, Call, Expression, FunctionCall, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::lint_b_helpers::{constant, unpacked, value};
use super::rector_code_quality::{args, named};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferClamp;

impl Rule for PreferClamp {
    fn id(&self) -> &'static str {
        "lint/php-8.6/prefer-clamp"
    }

    fn min_php_version(&self) -> u32 {
        // PhpVersionFeature::CLAMP
        80600
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(o) = node else { return };
        let outer = o;
        let (outer_is_max, inner_name) = if named(outer, "max") {
            (true, "min")
        } else if named(outer, "min") {
            (false, "max")
        } else {
            return;
        };
        let outer_args = args(outer);
        let [left, right] = outer_args[..] else {
            return;
        };
        if !supported(left) || !supported(right) {
            return;
        }
        let at = node_span(outer).start;
        let (inner, outer_bound) = match (inner_call(value(left)), inner_call(value(right))) {
            (Some(inner), _) => (inner, value(right)),
            (None, Some(inner)) => (inner, value(left)),
            (None, None) => return,
        };
        if !named(inner, inner_name) {
            return;
        }
        let inner_args = args(inner);
        let [first, second] = inner_args[..] else {
            return;
        };
        if !supported(first) || !supported(second) {
            return;
        }
        // matchValueAndKnownBound
        let known = |e: &Expression<'_>| constant(ancestors, e, at).is_some();
        let (value_expr, inner_bound) = match (known(value(first)), known(value(second))) {
            (true, false) => (value(second), value(first)),
            (false, true) => (value(first), value(second)),
            _ => return,
        };
        let text = |e: &Expression<'_>| {
            let e = unparenthesize(e);
            &ctx.source[node_span(e).start..node_span(e).end]
        };
        let (low, high) = if outer_is_max {
            (outer_bound, inner_bound)
        } else {
            (inner_bound, outer_bound)
        };
        let span = node_span(outer);
        out.push(Report {
            span,
            message: "Use `clamp()` instead of nested `min()` and `max()`.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(
                    span,
                    format!("clamp({}, {}, {})", text(value_expr), text(low), text(high)),
                )],
            }),
        });
    }
}

/// `isSupportedArg`: neither unpacked nor named.
fn supported(arg: &Argument<'_>) -> bool {
    !unpacked(arg) && matches!(arg, Argument::Positional(_))
}

/// `$expr instanceof FuncCall`.
fn inner_call<'a>(expr: &'a Expression<'a>) -> Option<&'a FunctionCall<'a>> {
    match unparenthesize(expr) {
        Expression::Call(Call::Function(call)) => Some(call),
        _ => None,
    }
}
