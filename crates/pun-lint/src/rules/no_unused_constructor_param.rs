//! `lint/dead-code/no-unused-constructor-param`, ported from Rector 2.6.1's `RemoveUnusedConstructorParamRector`
//! (rules/DeadCode/Rector/ClassMethod), with `ClassMethodParamRemover` and `ComplexNodeRemover::
//! processRemoveParamWithKeys`.
//!
//! A constructor's unused parameters go when they are the last ones (a used parameter after an unused one keeps them
//! all), with their `@param` tags. An abstract constructor, one an implemented interface declares, and the documented
//! constructor of an attribute class (its parameters can be markers) are left alone, as is a class the codebase does
//! not know.

use mago_span::HasSpan;
use mago_syntax::cst::*;
use pun_problem::{Fix, Safety};

use super::rector_batch5::{doc_comment, has_attribute, member_from, param_used, remove_params};
use super::types::facts::facts;
use super::types::namespace_of;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoUnusedConstructorParam;

impl Rule for NoUnusedConstructorParam {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-unused-constructor-param"
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
        let facts = facts(ctx);
        let (members, left_brace, attributes, name) = match node {
            Node::Class(class) => {
                let name = String::from_utf8_lossy(class.name.value);
                let name = match namespace_of(ancestors) {
                    Some(ns) => format!("{ns}\\{name}"),
                    None => name.into_owned(),
                };
                (
                    &class.members,
                    class.left_brace,
                    &class.attribute_lists,
                    name,
                )
            }
            Node::AnonymousClass(class) => (
                &class.members,
                class.left_brace,
                &class.attribute_lists,
                facts.anonymous_class_name(class.span()),
            ),
            _ => return,
        };
        let Some((index, method)) = members.iter().enumerate().find_map(|(i, m)| match m {
            ClassLikeMember::Method(m) if m.name.value.eq_ignore_ascii_case(b"__construct") => {
                Some((i, m))
            }
            _ => None,
        }) else {
            return;
        };
        let codebase = &facts.codebase;
        let Some(class) = codebase.get_class_like(name.as_bytes()).filter(|_| {
            !method.parameter_list.parameters.is_empty()
                && !matches!(method.body, MethodBody::Abstract(_))
        }) else {
            return;
        };
        // Parameters an interface requires.
        if class.all_parent_interfaces.iter().any(|i| {
            codebase.get_class_like(i.as_bytes()).is_some_and(|i| {
                i.methods
                    .iter()
                    .any(|m| m.as_bytes().eq_ignore_ascii_case(b"__construct"))
            })
        }) {
            return;
        }
        let from = member_from(members, left_brace, index);
        let mut path = ancestors.to_vec();
        path.push(node);
        if has_attribute(&path, attributes, "Attribute")
            && doc_comment(ctx.source, from, node_span(method).start).is_some()
        {
            return;
        }
        let removed: Vec<bool> = method
            .parameter_list
            .parameters
            .iter()
            .map(|p| !param_used(method, p))
            .collect();
        let Some(first) = removed.iter().position(|r| *r) else {
            return;
        };
        if !removed[first..].iter().all(|r| *r) {
            return;
        }
        out.push(Report {
            span: node_span(method),
            message: "Remove the unused parameters of this constructor.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: remove_params(ctx.source, method, from, &removed),
            }),
        });
    }
}
