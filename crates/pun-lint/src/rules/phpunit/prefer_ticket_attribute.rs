//! `lint/phpunit/prefer-ticket-attribute`, ported from rector-phpunit's `TicketAnnotationToAttributeRector`
//! (rules/AnnotationsToAttributes/Rector/Class_).
//!
//! A test class or test method's generic `@ticket` tags become `#[\PHPUnit\Framework\Attributes\Ticket('...')]`
//! groups after its attributes, when that attribute class exists. Rector runs it only for PHPUnit 10 and later; Pun
//! leaves that to the rule's group.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::Safety;

use super::annotations::{Annotated, TagValue, named_tags, single_quoted};
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
        let annotated = Annotated::of(ctx.source, node, ancestors).unwrap();
        let Some(doc) = annotated.doc.as_ref() else {
            return;
        };
        if !doc.text_all().contains("@ticket") {
            return;
        }
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
        let Some(tags) = named_tags(ctx, ancestors, doc, "ticket") else {
            return;
        };
        let mut removed = vec![false; doc.len()];
        let mut groups = Vec::new();
        for (i, value) in tags {
            if let TagValue::Generic(value) = value {
                removed[i] = true;
                groups.push(format!("#[\\{TICKET_CLASS}({})]", single_quoted(&value)));
            }
        }
        if groups.is_empty() {
            return;
        }
        let new_doc = doc.print(&removed);
        let Some(edits) = annotated.edits(ctx.source, Some(&new_doc), &groups) else {
            return;
        };
        out.push(report(
            doc.span,
            "Use the `#[Ticket]` attribute instead of the `@ticket` annotation.",
            Safety::Safe,
            edits,
        ));
    }
}
