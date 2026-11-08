//! `lint/classes/no-protected-constant-in-final`, ported from Rector 2.8.4's `PrivatizeFinalClassConstantRector`
//! (rules/Privatization/Rector/ClassConst).
//!
//! A `protected` constant of a `final` class becomes `private`, one declared alone (`const A = 1, B = 2;` stays);
//! a `final` modifier goes with it.
//! See `lint_b_privatize` for the classes Pun knows enough about.

use mago_syntax::cst::{ClassLikeMember, Node, NodeKind};
use pun_problem::{Fix, Safety};

use super::lint_b_privatize::{final_class, is_protected, make_private};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoProtectedConstantInFinal;

impl Rule for NoProtectedConstantInFinal {
    fn id(&self) -> &'static str {
        "lint/classes/no-protected-constant-in-final"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(class) = final_class(node) else {
            return;
        };
        for member in class.members.iter() {
            let ClassLikeMember::Constant(constant) = member else {
                continue;
            };
            if constant.items.len() != 1 || !is_protected(&constant.modifiers) {
                continue;
            }
            out.push(Report {
                span: node_span(constant),
                message: "Make the protected constant of a final class private.".to_string(),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Unsafe,
                    edits: vec![make_private(ctx.source, &constant.modifiers)],
                }),
            });
        }
    }
}
