//! `lint/complexity/no-redundant-write-visibility`, ported from Mago 1.57.0's `no-redundant-write-visibility`
//! (mago-linter src/rule/redundancy/no_redundant_write_visibility.rs).
//!
//! Reports a property's write visibility (`public(set)`, `protected(set)`, `private(set)`) that repeats its read
//! visibility, so it has no effect.

use mago_span::HasSpan;
use mago_syntax::cst::{Modifier, ModifierSequenceExt, Node, NodeKind};

use crate::syntax::span;
use crate::{Context, Report, Rule};

pub struct NoRedundantWriteVisibility;

impl Rule for NoRedundantWriteVisibility {
    fn id(&self) -> &'static str {
        "lint/complexity/no-redundant-write-visibility"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Property])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Property(property) = node else {
            return;
        };
        let modifiers = property.modifiers();
        let (Some(write), Some(read)) = (
            modifiers.get_first_write_visibility(),
            modifiers.get_first_read_visibility(),
        ) else {
            return;
        };
        if matches!(
            (read, write),
            (Modifier::Public(_), Modifier::PublicSet(_))
                | (Modifier::Protected(_), Modifier::ProtectedSet(_))
                | (Modifier::Private(_), Modifier::PrivateSet(_))
        ) {
            out.push(Report {
                span: span(write.span()),
                message: "Identical write visibility has no effect.".to_string(),
                help: Some("Remove the redundant write visibility modifier.".to_string()),
                fix: None,
            });
        }
    }
}
