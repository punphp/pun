//! `lint/classes/prefer-new-self`, ported from Rector 2.8.4's `NewStaticToNewSelfRector`
//! (rules/CodeQuality/Rector/New_).
//!
//! In a `final` class, `new static()` becomes `new self()`: no child class can make `static` mean anything else. Rector
//! walks the whole class, closures and anonymous classes inside it included. Its `treatClassesAsFinal` feature flag
//! is off by default and has no Pun setting.

use mago_syntax::cst::{Expression, Modifier, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferNewSelf;

impl Rule for PreferNewSelf {
    fn id(&self) -> &'static str {
        "lint/classes/prefer-new-self"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Class(class) = node else { return };
        if !class
            .modifiers
            .iter()
            .any(|m| matches!(m, Modifier::Final(_)))
        {
            return;
        }
        let mut stack = vec![node];
        while let Some(current) = stack.pop() {
            if let Node::Instantiation(new) = current
                && let Expression::Static(keyword) = new.class
            {
                let span = node_span(keyword);
                out.push(Report {
                    span,
                    message: "In a final class, `new static` is `new self`.".to_string(),
                    help: None,
                    fix: Some(Fix {
                        safety: Safety::Safe,
                        edits: vec![Edit::new(span, "self")],
                    }),
                });
            }
            stack.extend(current.children());
        }
    }
}
