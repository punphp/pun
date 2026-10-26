//! `lint/phpunit/prefer-ticket-attribute`, ported from rector-phpunit's `TicketAnnotationToAttributeRector`
//! (rules/AnnotationsToAttributes/Rector/Class_).
//!
//! A test class or test method's generic `@ticket` tags become `#[\PHPUnit\Framework\Attributes\Ticket('...')]`
//! groups after its attributes, when that attribute class exists. Rector runs it only for PHPUnit 10 and later; Pun
//! leaves that to the rule's group.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::Safety;

use super::annotations::{Annotated, single_quoted, take_generic_tags};
use super::{is_in_test_class, report};
use crate::rules::types::facts::facts;
use crate::{Context, Report, Rule};

const TICKET_CLASS: &str = "PHPUnit\\Framework\\Attributes\\Ticket";

pub struct PreferTicketAttribute;

impl Rule for PreferTicketAttribute {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-ticket-attribute"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class, NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(annotated) = Annotated::of(ctx.source, node, ancestors) else {
            return;
        };
        let Some((new_doc, values)) = annotated
            .doc
            .as_ref()
            .and_then(|doc| take_generic_tags(doc, "ticket"))
        else {
            return;
        };
        let in_test_class = match node {
            Node::Method(method) => is_in_test_class(ctx, ancestors, Some(method)),
            _ => {
                let mut scope = ancestors.to_vec();
                scope.push(node);
                is_in_test_class(ctx, &scope, None)
            }
        };
        // make sure the attribute class exists
        if !in_test_class
            || facts(ctx)
                .codebase
                .get_class_like(TICKET_CLASS.as_bytes())
                .is_none()
        {
            return;
        }
        let groups: Vec<String> = values
            .iter()
            .map(|v| format!("#[\\{TICKET_CLASS}({})]", single_quoted(v)))
            .collect();
        out.push(report(
            annotated.doc.as_ref().unwrap().span,
            "Use the `#[Ticket]` attribute instead of the `@ticket` annotation.",
            Safety::Safe,
            annotated.edits(ctx.source, Some(&new_doc), &groups),
        ));
    }
}
