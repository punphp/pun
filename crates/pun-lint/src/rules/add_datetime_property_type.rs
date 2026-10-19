//! `lint/types/add-datetime-property-type`, ported from Rector 2.6.1's `MergeDateTimePropertyTypeDeclarationRector`
//! (rules/TypeDeclaration/Rector/Class_).
//!
//! A private property typed `\DateTime` whose `@var` says `DateTimeInterface` gets the type `\DateTimeInterface`; the
//! `@var` tags go when the first has no description.

use mago_syntax::cst::{ClassLikeMember, Hint, Modifier, Node, NodeKind, Property};
use pun_problem::{Edit, Fix, Safety};

use super::lint_b_doc::Doc;
use super::lint_b_names::resolve_class_at;
use super::rector_batch5::resolve_class_name;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct AddDatetimePropertyType;

impl Rule for AddDatetimePropertyType {
    fn id(&self) -> &'static str {
        "lint/types/add-datetime-property-type"
    }

    fn min_php_version(&self) -> u32 {
        70400
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::ClassLikeMember])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::ClassLikeMember(ClassLikeMember::Property(property)) = node else {
            return;
        };
        if !matches!(
            ancestors.last(),
            Some(Node::Class(_) | Node::AnonymousClass(_))
        ) {
            return;
        }
        let (modifiers, hint) = match property {
            Property::Plain(p) => (&p.modifiers, &p.hint),
            Property::Hooked(p) => (&p.modifiers, &p.hint),
        };
        let Some(Hint::Identifier(type_name)) = hint else {
            return;
        };
        // `FullyQualified::toString() !== 'DateTime'`: compared as resolved, case and all.
        if resolve_class_name(ancestors, type_name) != "DateTime"
            || !modifiers.iter().any(|m| matches!(m, Modifier::Private(_)))
        {
            return;
        }
        let start = node_span(&node).start;
        let Some(doc) = Doc::of(ctx.source, ancestors, start) else {
            return;
        };
        let vars: Vec<bool> = (0..doc.len())
            .map(|i| doc.words(i).first() == Some(&"@var"))
            .collect();
        let Some(first) = vars.iter().position(|v| *v) else {
            return;
        };
        let words = doc.words(first);
        let Some(var_type) = words.get(1) else {
            return;
        };
        if !var_type
            .bytes()
            .all(|b| b == b'\\' || b == b'_' || b.is_ascii_alphanumeric() || b >= 0x80)
            || resolve_class_at(ancestors, doc.span.start as u32, var_type.as_bytes())
                != b"DateTimeInterface"
        {
            return;
        }
        let mut edits = vec![Edit::new(node_span(type_name), "\\DateTimeInterface")];
        let description = words[2..].iter().skip(usize::from(
            words.get(2).is_some_and(|w| w.starts_with('$')),
        ));
        if description.count() == 0 {
            edits.push(doc.remove(&vars, start));
        }
        out.push(Report {
            span: node_span(type_name),
            message: "Type the property `\\DateTimeInterface`, as its `@var` says.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits,
            }),
        });
    }
}
