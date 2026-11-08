//! `lint/dead-code/no-dead-catch`, ported from Rector 2.8.4's `RemoveDeadCatchRector` (rules/DeadCode/Rector/TryCatch).
//!
//! In a `try` with several catches, a catch of one class that only rethrows its exception goes, unless a later catch
//! of a related class (or of several classes) does something else with it. A `try` whose every catch would go is left
//! alone. A later catch is related unless the codebase knows both classes and neither inheritance links them; a later
//! catch of `Throwable` or `Exception` is always related.

use mago_syntax::cst::{Expression, Hint, Node, NodeKind, Statement, TryCatchClause, Variable};
use pun_problem::{Edit, Fix, Safety, Span};

use super::laravel::is_a;
use super::rector_batch5::resolve_class_name;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoDeadCatch;

impl Rule for NoDeadCatch {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-dead-catch"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Try])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Try(r#try) = node else {
            return;
        };
        let catches: Vec<&TryCatchClause<'_>> = r#try.catch_clauses.iter().collect();
        if catches.len() == 1 {
            return;
        }
        // The class of a catch of exactly one class.
        let class = |c: &TryCatchClause<'_>| match &c.hint {
            Hint::Identifier(id) => Some(resolve_class_name(ancestors, id)),
            _ => None,
        };
        let mut removed = vec![false; catches.len()];
        for (key, catch) in catches.iter().enumerate() {
            // `just_rethrows` holds only for a catch of one class.
            let Some(own) = class(catch).filter(|_| just_rethrows(catch)) else {
                continue;
            };
            // `shouldSkipNextCatchClassParentWithSpecialTreatment`.
            let skip = catches[key + 1..].iter().any(|next| match class(next) {
                None => true,
                Some(next_class) => {
                    let catches_anything = ["Throwable", "Exception"]
                        .contains(&next_class.trim_start_matches('\\'));
                    (catches_anything || !is_provably_unrelated(ctx, &own, &next_class))
                        && !just_rethrows(next)
                }
            });
            if !skip {
                removed[key] = true;
            }
        }
        if !removed.contains(&true) || !removed.contains(&false) {
            return;
        }
        let mut edits = Vec::new();
        for (key, catch) in catches.iter().enumerate() {
            if removed[key] {
                let from = match key {
                    0 => node_span(&r#try.block).end,
                    _ => node_span(catches[key - 1]).end,
                };
                edits.push(Edit::new(Span::new(from, node_span(*catch).end), ""));
            }
        }
        out.push(Report {
            span: node_span(r#try),
            message: "Remove the catch that only rethrows its exception.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// `isProvablyUnrelated`: the codebase knows both classes and `current` is not a `next`.
fn is_provably_unrelated(ctx: &Context<'_>, current: &str, next: &str) -> bool {
    let facts = super::types::facts::facts(ctx);
    let known = |name: &str| facts.codebase.get_class_like(name.as_bytes()).is_some();
    known(current) && known(next) && !is_a(ctx, current, &[next])
}

/// `isJustThrownSameVariable`: the catch's first statement throws the caught variable, and it catches one class.
fn just_rethrows(catch: &TryCatchClause<'_>) -> bool {
    let Some(Statement::Expression(first)) = catch.block.statements.first() else {
        return false;
    };
    let Expression::Throw(throw) = unparenthesize(first.expression) else {
        return false;
    };
    let Some(variable) = &catch.variable else {
        return false;
    };
    matches!(unparenthesize(throw.exception), Expression::Variable(Variable::Direct(v)) if v.name == variable.name)
        && matches!(catch.hint, Hint::Identifier(_))
}
