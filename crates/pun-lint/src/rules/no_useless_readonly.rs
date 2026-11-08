//! `lint/phpdoc/no-useless-readonly`, ported from Rector 2.8.4's `RemoveUselessReadOnlyTagRector`
//! (rules/DeadCode/Rector/Property).
//!
//! A `@readonly` tag without a description says what the native `readonly` of the class, the property or the promoted
//! parameter it documents already says: every `@readonly` tag of the doc comment goes, and the doc comment with them
//! when nothing else is left.

use mago_syntax::cst::{Modifier, Node, NodeKind};
use pun_problem::{Fix, Safety};

use super::lint_b_doc::Doc;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoUselessReadonly;

impl Rule for NoUselessReadonly {
    fn id(&self) -> &'static str {
        "lint/phpdoc/no-useless-readonly"
    }

    fn min_php_version(&self) -> u32 {
        // PhpVersionFeature::READONLY_PROPERTY
        80100
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::Class,
            NodeKind::Property,
            NodeKind::FunctionLikeParameter,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        // The class's or the parameter's own modifiers are its children; a property's sit one node down.
        let mut readonly = false;
        match node {
            Node::Property(property) => {
                readonly = property.modifiers().iter().any(is_readonly);
            }
            other => other.visit_children(|child| {
                readonly |= matches!(child, Node::Modifier(m) if is_readonly(m));
            }),
        }
        if !readonly {
            return;
        }
        let start = node_span(&node).start;
        let Some(doc) = Doc::of(ctx, ancestors, start) else {
            return;
        };
        // `getByName('readonly')`: the first `@readonly` tag, which must have no value; `removeByName()` takes all.
        let words: Vec<Vec<&str>> = (0..doc.len()).map(|i| doc.words(i)).collect();
        let removed: Vec<bool> = words
            .iter()
            .map(|w| w.first() == Some(&"@readonly"))
            .collect();
        let Some(first) = removed.iter().position(|r| *r) else {
            return;
        };
        if words[first].len() != 1 {
            return;
        }
        out.push(Report {
            span: doc.span,
            message: "Remove the `@readonly` tag that the native `readonly` already says."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![doc.remove(&removed, start)],
            }),
        });
    }
}

fn is_readonly(modifier: &Modifier<'_>) -> bool {
    matches!(modifier, Modifier::Readonly(_))
}
