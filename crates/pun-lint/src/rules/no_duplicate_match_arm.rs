//! `lint/complexity/no-duplicate-match-arm`, ported from Mago 1.57.0's `no-duplicate-match-arm`
//! (mago-linter src/rule/redundancy/no_duplicate_match_arm.rs).
//!
//! Reports a `match` arm whose result is a constant value an earlier arm already returns, so the two arms could be
//! one.

use mago_span::HasSpan;
use mago_syntax::cst::{MatchArm, Node, NodeKind};

use super::mago_const_value::{ConstantValue, get_constant_value};
use crate::syntax::span;
use crate::{Context, Report, Rule};

pub struct NoDuplicateMatchArm;

impl Rule for NoDuplicateMatchArm {
    fn id(&self) -> &'static str {
        "lint/complexity/no-duplicate-match-arm"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Match])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Match(r#match) = node else { return };
        let mut seen: Vec<ConstantValue> = Vec::new();
        for arm in &r#match.arms {
            let MatchArm::Expression(arm) = arm else {
                continue;
            };
            let Some(value) = get_constant_value(ancestors, arm.expression) else {
                continue;
            };
            if seen.iter().any(|v| v.same(&value)) {
                out.push(Report {
                    span: span(arm.expression.span()),
                    message: "This `match` arm has the same result as an earlier arm.".to_string(),
                    help: Some(
                        "Merge these arms by combining their conditions into a single arm."
                            .to_string(),
                    ),
                    fix: None,
                });
            } else {
                seen.push(value);
            }
        }
    }
}
