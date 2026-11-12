//! `lint/complexity/prefer-explicit-array-comparison`, ported from Rector 2.8.4's `ArrayExplicitBoolCompareRector`
//! (rules/CodeQuality/Rector/If_).
//!
//! A variable whose native type is an array, used as the whole condition of an `if`, an `elseif` or a ternary, is
//! compared with `[]`: `if ($items)` becomes `if ($items !== [])` and `if (!$items)` becomes `if ($items === [])`.
//! The short ternary `?:` stays, and so does a property or any other expression.

use mago_codex::ttype::atomic::TAtomic;
use mago_syntax::cst::{Expression, Node, NodeKind};
use pun_problem::{Edit, Safety};

use super::rector_code_quality::text;
use super::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferExplicitArrayComparison;

impl Rule for PreferExplicitArrayComparison {
    fn id(&self) -> &'static str {
        "lint/complexity/prefer-explicit-array-comparison"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::If,
            NodeKind::IfStatementBodyElseIfClause,
            NodeKind::IfColonDelimitedBodyElseIfClause,
            NodeKind::Conditional,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let condition = match node {
            Node::If(i) => i.condition,
            Node::IfStatementBodyElseIfClause(c) => c.condition,
            Node::IfColonDelimitedBodyElseIfClause(c) => c.condition,
            // skip short ternary
            Node::Conditional(t) if t.then.is_some() => t.condition,
            _ => return,
        };
        let condition = unparenthesize(condition);
        let (variable, operator) = match condition {
            Expression::UnaryPrefix(not) if not.operator.is_not() => {
                (unparenthesize(not.operand), "===")
            }
            other => (other, "!=="),
        };
        // cheap bail - rule only ever rewrites a bare variable, skip costly type resolution otherwise
        if !matches!(variable, Expression::Variable(_)) {
            return;
        }
        // ArrayTypeAnalyzer::isArrayType: the native type is an array.
        if !facts(ctx)
            .native_type(node_span(variable))
            .is_some_and(|t| {
                !t.types.is_empty() && t.types.iter().all(|a| matches!(a, TAtomic::Array(_)))
            })
        {
            return;
        }
        let span = node_span(condition);
        out.push(Report::fixed(
            span,
            "Compare the array with `[]` instead of using it as a condition.",
            Safety::Safe,
            vec![Edit::new(
                span,
                format!("{} {operator} []", text(ctx, variable)),
            )],
        ));
    }
}
