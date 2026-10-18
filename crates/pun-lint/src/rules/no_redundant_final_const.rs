//! `lint/style/no-redundant-final-const`, ported from Rector 2.6.1's `RemoveFinalFromConstRector`
//! (rules/CodingStyle/Rector/ClassConst), with `VisibilityManipulator::removeFinal`.
//!
//! In a `final class`, `final public const X` says nothing more than `public const X`: no child class can override
//! the constant. Rector removes the modifier; Pun deletes the `final` keyword and the blanks after it.

use mago_syntax::cst::{ClassLikeMember, Modifier, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety, Span};

use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoRedundantFinalConst;

impl Rule for NoRedundantFinalConst {
    fn id(&self) -> &'static str {
        "lint/style/no-redundant-final-const"
    }

    /// PhpVersionFeature::FINAL_CLASS_CONSTANTS.
    fn min_php_version(&self) -> u32 {
        80100
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
        let Node::Class(class) = node else { return };
        if !class
            .modifiers
            .iter()
            .any(|m| matches!(m, Modifier::Final(_)))
        {
            return;
        }
        for member in class.members.iter() {
            let ClassLikeMember::Constant(constant) = member else {
                continue;
            };
            let Some(Modifier::Final(keyword)) = constant
                .modifiers
                .iter()
                .find(|m| matches!(m, Modifier::Final(_)))
            else {
                continue;
            };
            let span = node_span(keyword);
            let rest = &ctx.source[span.end..];
            let blanks = rest.len() - rest.trim_start().len();
            out.push(Report {
                span,
                message: "A constant of a final class needs no `final`; remove it.".to_string(),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Safe,
                    edits: vec![Edit::new(Span::new(span.start, span.end + blanks), "")],
                }),
            });
        }
    }
}
