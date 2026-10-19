//! `lint/php-8.4/prefer-round-mode-enum`, ported from Rector 2.6.1's `RoundingModeEnumRector`
//! (rules/Php84/Rector/FuncCall).
//!
//! `round($x, $p, PHP_ROUND_HALF_UP)` becomes `round($x, $p, \RoundingMode::HalfAwayFromZero)`, and likewise for the
//! other three `PHP_ROUND_*` constants. Only a call with exactly three arguments whose third is one of those constants
//! changes.

use mago_syntax::cst::{Expression, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::lint_b_helpers::value;
use super::rector_code_quality::{args, named};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferRoundModeEnum;

impl Rule for PreferRoundModeEnum {
    fn id(&self) -> &'static str {
        "lint/php-8.4/prefer-round-mode-enum"
    }

    fn min_php_version(&self) -> u32 {
        // PhpVersionFeature::ROUNDING_MODES
        80400
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(call) = node else {
            return;
        };
        if !named(call, "round") {
            return;
        }
        let args = args(call);
        let [_, _, mode] = args.as_slice() else {
            return;
        };
        // `$modeArg instanceof ConstFetch`: `$modeArg->name->toString()`, as written, without a leading backslash.
        let Expression::ConstantAccess(constant) = value(mode) else {
            return;
        };
        let name = constant.name.value();
        let case = match name.strip_prefix(b"\\").unwrap_or(name) {
            b"PHP_ROUND_HALF_UP" => "HalfAwayFromZero",
            b"PHP_ROUND_HALF_DOWN" => "HalfTowardsZero",
            b"PHP_ROUND_HALF_EVEN" => "HalfEven",
            b"PHP_ROUND_HALF_ODD" => "HalfOdd",
            _ => return,
        };
        let span = node_span(constant);
        out.push(Report {
            span,
            message: "Use the `RoundingMode` enum instead of a `PHP_ROUND_*` constant.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, format!("\\RoundingMode::{case}"))],
            }),
        });
    }
}
