//! `lint/arrays/no-implicit-array-creation`, ported from phpstan-strict-rules'
//! `DisallowedImplicitArrayCreationRule` (identifier `variable.implicitArray`).
//!
//! `$a[] = 1;` or `$a['k']['j'] = 1;` where `$a` is not defined, or not defined on every path, creates an array
//! implicitly. The state of the variable is Mago's, read before the assignment as `Scope::hasVariableType()`.
//! The rule has no fix.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::Span;

use super::types::facts::facts;
use crate::{Context, Report, Rule};

pub struct NoImplicitArrayCreation;

impl Rule for NoImplicitArrayCreation {
    fn id(&self) -> &'static str {
        "lint/arrays/no-implicit-array-creation"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Program])
    }

    fn visit<'ast, 'arena>(
        &self,
        _node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        for ((start, end), name, maybe) in facts(ctx).implicit_array_writes() {
            let state = if maybe {
                "might not exist"
            } else {
                "does not exist"
            };
            out.push(Report {
                span: Span::new(start as usize, end as usize),
                message: format!(
                    "Implicit array creation is not allowed - variable {name} {state}."
                ),
                help: None,
                fix: None,
            });
        }
    }
}
