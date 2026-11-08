//! `lint/dead-code/no-always-false-condition`, ported from Rector 2.8.4's `ReduceAlwaysFalseIfOrRector`
//! (rules/DeadCode/Rector/If_), with `SafeLeftTypeBooleanAndOrAnalyzer` (rules/DeadCode/NodeAnalyzer).
//!
//! An `if (a || b)` whose left side PHPStan types as `false` keeps only `b` as its condition. The left side must hold
//! no variable that `isNonTypedFromParam` doubts, no property or array item, and (outside a trait) no call whose
//! native type is `mixed`; in a trait it must not be an `instanceof`.

use mago_codex::ttype::atomic::TAtomic;
use mago_syntax::cst::{Access, BinaryOperator, Expression, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::prefer_empty_array_identical::is_non_typed_from_param;
use super::side_effect::any_node;
use super::types::facts::{Facts, facts};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoAlwaysFalseCondition;

impl Rule for NoAlwaysFalseCondition {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-always-false-condition"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::If])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::If(r#if) = node else { return };
        let cond = unparenthesize(r#if.condition);
        let Expression::Binary(or) = cond else { return };
        if !matches!(or.operator, BinaryOperator::Or(_)) {
            return;
        }
        let facts = facts(ctx);
        let left = unparenthesize(or.lhs);
        let is_false = facts
            .full_type(node_span(left))
            .is_some_and(|t| matches!(&t.types[..], [TAtomic::Scalar(s)] if s.is_false()));
        if !is_false || !is_safe(&facts, ancestors, left) {
            return;
        }
        let right = node_span(unparenthesize(or.rhs));
        let span = node_span(cond);
        out.push(Report {
            span,
            message: "The left side of this `||` is always false; only the right side decides."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, &ctx.source[right.start..right.end])],
            }),
        });
    }
}

/// `SafeLeftTypeBooleanAndOrAnalyzer::isSafe` on the left side `left`.
pub(crate) fn is_safe(facts: &Facts, ancestors: &[Node<'_, '_>], left: &Expression<'_>) -> bool {
    let found = |test: &dyn Fn(&Expression<'_>) -> bool| {
        any_node(
            Node::Expression(left),
            &|n| matches!(n, Node::Expression(e) if test(e)),
        )
    };
    if found(&|e| matches!(e, Expression::Variable(_)) && is_non_typed_from_param(facts, e)) {
        return false;
    }
    if found(&|e| {
        matches!(
            e,
            Expression::Access(Access::Property(_) | Access::StaticProperty(_))
                | Expression::ArrayAccess(_)
        )
    }) {
        return false;
    }
    if ancestors.iter().any(|a| matches!(a, Node::Trait(_))) {
        return !matches!(left, Expression::Binary(b) if matches!(b.operator, BinaryOperator::Instanceof(_)));
    }
    // a call PHPStan types as `mixed` natively (taken as implicit, as in `class_method`)
    !found(&|e| {
        matches!(e, Expression::Call(_) | Expression::Instantiation(_))
            && facts
                .native_type(node_span(e))
                .is_none_or(mago_codex::ttype::union::TUnion::is_mixed)
    })
}
