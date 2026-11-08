//! `lint/php-8.5/no-out-of-range-chr`, ported from Rector 2.8.4's `ChrArgModuloRector` (rules/Php85/Rector/FuncCall).
//!
//! PHP 8.5 deprecates passing `chr()` an integer outside 0-255, so such an argument becomes `$x % 256`. Rector asks
//! PHPStan for the argument's value; Pun knows it where the syntax does (see `lint_b_helpers::constant`).

use mago_syntax::cst::{BinaryOperator, Expression, Node, NodeKind, UnaryPrefixOperator};
use pun_problem::{Edit, Fix, Safety};

use super::lint_b_helpers::{Constant, Known, constant, known, value};
use super::rector_code_quality::{args, named};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoOutOfRangeChr;

impl Rule for NoOutOfRangeChr {
    fn id(&self) -> &'static str {
        "lint/php-8.5/no-out-of-range-chr"
    }

    fn min_php_version(&self) -> u32 {
        // PhpVersionFeature::DEPRECATE_OUTSIDE_INTERVEL_VAL_IN_CHR_FUNCTION
        80500
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
        let Node::FunctionCall(c) = node else { return };
        let call = c;
        if !named(call, "chr") {
            return;
        }
        let Some(first) = args(call).first().map(|a| value(a)) else {
            return;
        };
        if let Expression::Binary(b) = first
            && matches!(b.operator, BinaryOperator::Modulo(_))
        {
            return;
        }
        let at = node_span(call).start;
        // Rector reads the argument with `ValueResolver::getValue()`, which does not resolve `PHP_INT_MAX` and the
        // other PHP constants that `constant` knows for `MinMaxToClampRector`.
        if let Some(Known::Value(v)) = known(ancestors, first, at)
            && let Expression::ConstantAccess(_) = unsigned(v)
        {
            return;
        }
        let Some(Constant::Int(v)) = constant(ancestors, first, at) else {
            return;
        };
        if (0..=255).contains(&v) {
            return;
        }
        // The new `Mod` node takes the place of the argument's value, whose span in php-parser leaves out its
        // parentheses, so they stay around it; `constant` only knows literals, signed literals and variables, which
        // bind tighter than `%`.
        let inner = node_span(unparenthesize(first));
        let edits = vec![Edit::new(
            inner,
            format!("{} % 256", &ctx.source[inner.start..inner.end]),
        )];
        out.push(Report {
            span: node_span(first),
            message: "Wrap the out-of-range `chr()` argument in `% 256`.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// `expr` without a leading `-` or `+`.
fn unsigned<'a>(expr: &'a Expression<'a>) -> &'a Expression<'a> {
    match unparenthesize(expr) {
        Expression::UnaryPrefix(u)
            if matches!(
                u.operator,
                UnaryPrefixOperator::Negation(_) | UnaryPrefixOperator::Plus(_)
            ) =>
        {
            unsigned(u.operand)
        }
        e => e,
    }
}
