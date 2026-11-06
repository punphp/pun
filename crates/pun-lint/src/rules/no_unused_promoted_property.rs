//! `lint/dead-code/no-unused-promoted-property`, ported from Rector 2.6.1's `RemoveUnusedPromotedPropertyRector`
//! (rules/DeadCode/Rector/ClassMethod), with `PropertyFetchFinder::findLocalPropertyFetchesByName` and
//! `PropertyWriteonlyAnalyzer::hasClassDynamicPropertyNames`.
//!
//! A private promoted constructor parameter whose property the class never fetches goes, with its `@param` tag; when
//! the constructor reads the variable, only its modifiers go. A class with attributes, a `__get()` method, a trait,
//! a property fetched by a dynamic name or `get_object_vars($this)`, or an interface that declares the constructor
//! is left alone.
//!
//! Rector counts the fetches whose object has the class's type. Pun counts every fetch of that name in the class,
//! on any object, so it keeps a property Rector may remove.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety, Span};

use super::class_method::qualified;
use super::rector_batch5::{member_from, remove_params};
use super::rector_code_quality::named;
use super::side_effect::any_node;
use super::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoUnusedPromotedProperty;

impl Rule for NoUnusedPromotedProperty {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-unused-promoted-property"
    }

    fn min_php_version(&self) -> u32 {
        80000
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
        let (members, left_brace, attributes) = match node {
            Node::Class(c) => (&c.members, c.left_brace, &c.attribute_lists),
            Node::AnonymousClass(c) => (&c.members, c.left_brace, &c.attribute_lists),
            _ => return,
        };
        let method_named = |name: &[u8]| {
            members.iter().enumerate().find_map(|(i, m)| match m {
                ClassLikeMember::Method(m) if m.name.value.eq_ignore_ascii_case(name) => {
                    Some((i, m))
                }
                _ => None,
            })
        };
        let Some((index, constructor)) = method_named(b"__construct") else {
            return;
        };
        let params: Vec<&FunctionLikeParameter<'_>> =
            constructor.parameter_list.parameters.iter().collect();
        if params.is_empty()
            || !attributes.is_empty()
            || method_named(b"__get").is_some()
            || members
                .iter()
                .any(|m| matches!(m, ClassLikeMember::TraitUse(_)))
            || dynamic_names(node)
            || constructor_from_interface(ctx, ancestors, node)
        {
            return;
        }
        let mut removed = vec![false; params.len()];
        let mut edits = Vec::new();
        for (i, param) in params.iter().enumerate() {
            if !param
                .modifiers
                .iter()
                .any(|m| matches!(m, Modifier::Private(_)))
            {
                continue;
            }
            let name = &param.variable.name[1..];
            if fetched(node, name) {
                continue;
            }
            // `findVariableOfName`: the constructor reads the variable, so only the property goes.
            let read = any_node(
                Node::MethodBody(&constructor.body),
                &|n| matches!(n, Node::Variable(Variable::Direct(v)) if v.name == param.variable.name),
            );
            if read {
                let first = node_span(param.modifiers.first().unwrap()).start;
                // php-parser reprints from the modifiers to the type, or to the variable, so an untyped `&` goes too.
                let next = match &param.hint {
                    Some(hint) => node_span(hint).start,
                    None => node_span(&param.variable).start,
                };
                edits.push(Edit::new(Span::new(first, next), ""));
            } else {
                removed[i] = true;
            }
        }
        if removed.contains(&true) {
            let from = member_from(members, left_brace, index);
            edits.extend(remove_params(ctx, constructor, from, &removed));
        }
        if edits.is_empty() {
            return;
        }
        out.push(Report {
            span: node_span(constructor),
            message: "Remove the promoted property the class never reads.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// A fetch of the property `name` anywhere in the class: `$x->name`, `$x?->name` or `X::$name`.
fn fetched(class: Node<'_, '_>, name: &[u8]) -> bool {
    let is_name = |selector: &ClassLikeMemberSelector<'_>| matches!(selector, ClassLikeMemberSelector::Identifier(id) if id.value == name);
    any_node(class, &|n| match n {
        Node::PropertyAccess(p) => is_name(&p.property),
        Node::NullSafePropertyAccess(p) => is_name(&p.property),
        Node::StaticPropertyAccess(p) => {
            matches!(&p.property, Variable::Direct(v) if &v.name[1..] == name)
        }
        _ => false,
    })
}

/// `hasClassDynamicPropertyNames`: `get_object_vars($this)`, or a property fetched by a name that is not an
/// identifier.
fn dynamic_names(class: Node<'_, '_>) -> bool {
    any_node(class, &|n| {
        match n {
        Node::FunctionCall(call) if named(call, "get_object_vars") => {
            call.argument_list.arguments.first().is_some_and(|a| {
                matches!(unparenthesize(super::lint_b_helpers::value(a)), Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
            })
        }
        Node::PropertyAccess(p) => !matches!(p.property, ClassLikeMemberSelector::Identifier(_)),
        Node::NullSafePropertyAccess(p) => {
            !matches!(p.property, ClassLikeMemberSelector::Identifier(_))
        }
        _ => false,
    }
    })
}

/// An interface of the class declares `__construct`.
fn constructor_from_interface(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    node: Node<'_, '_>,
) -> bool {
    let facts = facts(ctx);
    let name = match node {
        Node::Class(c) => qualified(ancestors, c.name.value),
        _ => facts.anonymous_class_name(mago_span::HasSpan::span(&node)),
    };
    let codebase = &facts.codebase;
    codebase
        .get_class_like(name.as_bytes())
        .is_some_and(|class| {
            class.all_parent_interfaces.iter().any(|i| {
                codebase
                    .get_declaring_method(i.as_bytes(), b"__construct")
                    .is_some()
            })
        })
}
