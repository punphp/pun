//! `lint/types/add-property-type-behat-context`, ported from Rector 2.8.4's `TypedStaticPropertyInBehatContextRector`
//! (rules/TypeDeclaration/Rector/Class_): in a class that extends or implements something and is a
//! `Behat\Behat\Context\Context`, an untyped static property without a default (or with `null`) whose `@var` names a
//! class gets that class, nullable, and a `null` default; the `@var` goes when it says no more.
//!
//! The `@var` is a class when it is one name that is no phpdoc keyword (`int`, `string`, `self`, `object`, ...), or a
//! generic of one (`Collection<int, Item>`, which stays); the name is resolved through the `use` imports and the
//! namespace, whether the class exists or not, as Rector's phpdoc types are.

use mago_syntax::cst::{Modifier, Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::facts::facts;
use super::property_type::{
    ClassView, Ty, VarTag, is_null_literal, is_var_tag_useless_as, variable_span,
};
use crate::rules::phpunit::phpdoc_type::{TagType, is_name};
use crate::rules::rector_batch5::imported;
use crate::{Context, Report, Rule};

/// `BehatClassName::CONTEXT`.
const CONTEXT: &str = "Behat\\Behat\\Context\\Context";

pub struct AddPropertyTypeBehatContext;

impl Rule for AddPropertyTypeBehatContext {
    fn id(&self) -> &'static str {
        "lint/types/add-property-type-behat-context"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class, NodeKind::AnonymousClass])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (extends, implements) = match node {
            Node::Class(c) => (c.extends.is_some(), c.implements.is_some()),
            Node::AnonymousClass(c) => (c.extends.is_some(), c.implements.is_some()),
            _ => return,
        };
        if !extends && !implements {
            return;
        }
        let facts = facts(ctx);
        let Some(view) = ClassView::of(&facts, node, ancestors) else {
            return;
        };
        let is_context = facts
            .codebase
            .get_class_like(view.name.as_bytes())
            .is_some_and(|c| {
                c.all_parent_interfaces
                    .iter()
                    .any(|i| i.as_bytes().eq_ignore_ascii_case(CONTEXT.as_bytes()))
            });
        if !is_context {
            return;
        }
        let mut edits: Vec<Edit> = Vec::new();
        for prop in &view.properties() {
            if prop.item.is_none()
                || !prop
                    .modifiers()
                    .iter()
                    .any(|m| matches!(m, Modifier::Static(_)))
            {
                continue;
            }
            // hasNonNullDefault
            if prop.default().is_some_and(|d| !is_null_literal(d)) {
                continue;
            }
            let var = VarTag::of(ctx, prop.property, &view.path);
            let (written, generic, tag) = match &var.tag {
                Ok(Some(tag)) => match tag.ty {
                    TagType::Name(n) => (n, false, Some(tag)),
                    TagType::Other(o) => match o.split_once('<') {
                        Some((n, _)) if is_name(n) && o.ends_with('>') => (n, true, None),
                        _ => continue,
                    },
                    _ => continue,
                },
                Ok(None) => continue,
                Err(()) => match unread_name(&var) {
                    Some(n) => (n, true, None),
                    None => continue,
                },
            };
            // `self` is the class (`SelfObjectType`)
            let class = if written.eq_ignore_ascii_case("self") {
                view.name.clone()
            } else if is_keyword(written) {
                continue;
            } else {
                match written.strip_prefix('\\') {
                    Some(fq) => fq.to_string(),
                    None => match imported(&view.path, written) {
                        // an alias (`AliasedObjectType`) keeps its own name
                        Some(full)
                            if !written.contains('\\')
                                && !full
                                    .rsplit('\\')
                                    .next()
                                    .is_some_and(|last| last.eq_ignore_ascii_case(written)) =>
                        {
                            written.to_string()
                        }
                        Some(full) => full,
                        None => match super::namespace_of(&view.path) {
                            Some(ns) => format!("{ns}\\{written}"),
                            None => written.to_string(),
                        },
                    },
                }
            };
            let at = variable_span(prop);
            edits.push(Edit::new(
                Span::new(at.start, at.start),
                format!("?\\{class} "),
            ));
            if prop.default().is_none() {
                edits.push(Edit::new(Span::new(at.end, at.end), " = null"));
            }
            let native = Ty::Union(vec![Ty::Object(class.clone()), Ty::Null]);
            if !generic
                && let Some(tag) = tag
                && tag.description.is_empty()
                && is_var_tag_useless_as(tag, &Ty::Object(class), &native)
                && let Some(e) = var.remove(ctx.source)
            {
                edits.extend(e);
            }
        }
        let Some(first) = edits.first() else { return };
        out.push(Report::fixed(
            first.span,
            "Type the Behat context's static properties from their `@var`.",
            Safety::Unsafe,
            edits,
        ));
    }
}

/// The class name of a `@var` the shared tag reader leaves unread (a value over several lines, or a generic with
/// white space or a `,` inside its brackets, `Collection<int, Item>`) when its type is one name, maybe with brackets;
/// Rector types the property by the name and keeps the tag.
fn unread_name<'s>(var: &VarTag<'s>) -> Option<&'s str> {
    let doc = var.annotated.doc.as_ref()?;
    let text = (0..doc.len()).map(|i| doc.text(i)).find(|t| {
        t.strip_prefix("@var")
            .is_some_and(|r| r.starts_with(char::is_whitespace))
    })?;
    let value = text["@var".len()..].trim_start();
    let len = value
        .find(|c: char| !(c.is_alphanumeric() || matches!(c, '_' | '\\' | '-')))
        .unwrap_or(value.len());
    let name = &value[..len];
    if !is_name(name) {
        return None;
    }
    let mut after = &value[len..];
    if let Some(inner) = after.strip_prefix('<') {
        let mut depth = 1;
        let end = inner.find(|c| {
            match c {
                '<' => depth += 1,
                '>' => depth -= 1,
                _ => {}
            }
            depth == 0
        })?;
        after = &inner[end + 1..];
    }
    (after.is_empty() || after.starts_with(char::is_whitespace)).then_some(name)
}

/// A phpdoc name that is no class (`TypeNodeResolver`'s keywords).
fn is_keyword(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "int"
            | "integer"
            | "string"
            | "bool"
            | "boolean"
            | "float"
            | "double"
            | "real"
            | "array"
            | "iterable"
            | "callable"
            | "callback"
            | "object"
            | "mixed"
            | "void"
            | "null"
            | "true"
            | "false"
            | "resource"
            | "never"
            | "never-return"
            | "never-returns"
            | "no-return"
            | "self"
            | "static"
            | "parent"
            | "$this"
            | "scalar"
            | "number"
            | "numeric"
            | "list"
            | "non-empty-list"
            | "non-empty-array"
            | "array-key"
            | "class-string"
            | "callable-string"
            | "numeric-string"
            | "non-empty-string"
            | "non-falsy-string"
            | "truthy-string"
            | "literal-string"
            | "lowercase-string"
            | "uppercase-string"
            | "positive-int"
            | "negative-int"
            | "non-negative-int"
            | "non-positive-int"
            | "non-empty-mixed"
            | "empty"
    )
}
