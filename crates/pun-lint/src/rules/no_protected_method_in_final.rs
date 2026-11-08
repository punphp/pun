//! `lint/classes/no-protected-method-in-final`, ported from Rector 2.8.4's `PrivatizeFinalClassMethodRector`
//! (rules/Privatization/Rector/ClassMethod).
//!
//! A `protected` method of a `final` class becomes `private`, except a magic method; a `final` modifier goes with it.
//! See `lint_b_privatize` for the classes Pun knows enough about; a class that extends nothing cannot be a Laravel
//! model nor call `parent::`, which Rector's other guards look for.

use mago_syntax::cst::{ClassLikeMember, Node, NodeKind};
use pun_problem::{Fix, Safety};

use super::class_method::is_magic;
use super::lint_b_privatize::{final_class, is_protected, make_private};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoProtectedMethodInFinal;

impl Rule for NoProtectedMethodInFinal {
    fn id(&self) -> &'static str {
        "lint/classes/no-protected-method-in-final"
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
            let ClassLikeMember::Method(method) = member else {
                continue;
            };
            let name = method.name.value;
            if !is_protected(&method.modifiers) || is_magic(name) {
                continue;
            }
            out.push(Report {
                span: node_span(&method.name),
                message: "Make the protected method of a final class private.".to_string(),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Unsafe,
                    edits: vec![make_private(ctx.source, &method.modifiers)],
                }),
            });
        }
    }
}
