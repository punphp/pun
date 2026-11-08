//! `lint/complexity/prefer-in-array-over-search`, ported from Rector 2.8.4's `SimplifyArraySearchRector`
//! (rules/CodeQuality/Rector/Identical) with `BinaryOpManipulator::matchFirstAndSecondConditionNode`.
//!
//! `array_search($a, $b) !== false` becomes `in_array($a, $b)` and `array_search($a, $b) === false` becomes
//! `!in_array($a, $b)`, with `false` on either side. The arguments stay as they are, a strict third one included.

use mago_syntax::cst::{BinaryOperator, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::rector_code_quality::{Prec, args, call_to, in_parent, is_constant, text};
use super::rector_conditions::match_first_and_second;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferInArrayOverSearch;

impl Rule for PreferInArrayOverSearch {
    fn id(&self) -> &'static str {
        "lint/complexity/prefer-in-array-over-search"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Binary])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Binary(b) = node else { return };
        let negated = match b.operator {
            BinaryOperator::Identical(_) => true,
            BinaryOperator::NotIdentical(_) => false,
            _ => return,
        };
        let Some((search, _)) = match_first_and_second(
            b.lhs,
            b.rhs,
            |e| call_to(e, "array_search").is_some(),
            |e| is_constant(e, "false"),
        ) else {
            return;
        };
        let call = call_to(search, "array_search").expect("matched above");
        // NodeFactory::createFuncCall prints the arguments again, each as it was, separated by `, `.
        let arguments: Vec<&str> = args(call).iter().map(|a| text(ctx, *a)).collect();
        let call = format!("in_array({})", arguments.join(", "));
        let span = node_span(b);
        let code = if negated {
            in_parent(span, ancestors, format!("!{call}"), Prec::Prefix(30))
        } else {
            in_parent(span, ancestors, call, Prec::None)
        };
        out.push(Report {
            span,
            message: "Use `in_array()` instead of comparing `array_search()` with `false`."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, code)],
            }),
        });
    }
}
