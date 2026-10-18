//! `lint/laravel/prefer-str-starts-with`, ported from rector-laravel 2.5.0's
//! `SubStrToStartsWithOrEndsWithStaticMethodCallRector` (src/Rector/Expr).
//!
//! `substr($a, 0) === $b` becomes `\Illuminate\Support\Str::startsWith($a, $b)`, and `substr($a, -3) == $b` becomes
//! `\Illuminate\Support\Str::endsWith($a, $b)`. As upstream, a length after a start of `0` is dropped.

use mago_syntax::cst::{
    BinaryOperator, Call, Expression, Literal, Node, NodeKind, UnaryPrefixOperator,
};

use super::replace;
use crate::rules::rector_code_quality::{named, text};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferStrStartsWith;

impl Rule for PreferStrStartsWith {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-str-starts-with"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Binary])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Binary(binary) = node else { return };
        if !matches!(
            binary.operator,
            BinaryOperator::Identical(_) | BinaryOperator::Equal(_)
        ) {
            return;
        }
        let substr = |e: &'ast Expression<'arena>| match unparenthesize(e) {
            Expression::Call(Call::Function(f)) if named(f, "substr") => Some(f),
            _ => None,
        };
        let (call, other) = match (substr(binary.lhs), substr(binary.rhs)) {
            (Some(call), _) => (call, binary.rhs),
            (None, Some(call)) => (call, binary.lhs),
            (None, None) => return,
        };
        let args = &call.argument_list.arguments;
        let (Some(string), Some(start)) = (args.get(0), args.get(1)) else {
            return;
        };
        let Some(start) = int_value(start.value()) else {
            return;
        };
        let method = match start {
            0 => "startsWith",
            s if s < 0 && args.len() == 2 => "endsWith",
            _ => return,
        };
        out.push(replace(
            node_span(binary),
            format!("Use `Str::{method}()` instead of comparing `substr()`."),
            format!(
                "\\Illuminate\\Support\\Str::{method}({}, {})",
                text(ctx, unparenthesize(string.value())),
                text(ctx, unparenthesize(other))
            ),
        ));
    }
}

/// `ValueResolver::getValue` for an integer: a literal, with any number of `-` and `+` signs.
fn int_value(expr: &Expression<'_>) -> Option<i64> {
    match unparenthesize(expr) {
        Expression::Literal(Literal::Integer(i)) => i.value.and_then(|v| i64::try_from(v).ok()),
        Expression::UnaryPrefix(u) => match u.operator {
            UnaryPrefixOperator::Negation(_) => int_value(u.operand).map(|v| -v),
            UnaryPrefixOperator::Plus(_) => int_value(u.operand),
            _ => None,
        },
        _ => None,
    }
}
