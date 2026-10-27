//! `lint/phpunit/no-mock-intersection-property`, ported from rector-phpunit's `SingleMockPropertyTypeRector`
//! (rules/CodeQuality/Rector/Class_).
//!
//! In a test class, a property typed with a union or an intersection of exactly two types, one of them
//! `PHPUnit\Framework\MockObject\MockObject`, is typed with that one alone, written as it was.

use mago_syntax::cst::{ClassLikeMember, Hint, Node, NodeKind, Property};
use pun_problem::{Edit, Safety};

use super::{class_body, is_in_test_class, report, resolve, text};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

/// `MockObject::class`.
const MOCK_OBJECT: &str = "PHPUnit\\Framework\\MockObject\\MockObject";

pub struct NoMockIntersectionProperty;

impl Rule for NoMockIntersectionProperty {
    fn id(&self) -> &'static str {
        "lint/phpunit/no-mock-intersection-property"
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
        let Some((members, _, _)) = class_body(node) else {
            return;
        };
        let mut scope = ancestors.to_vec();
        scope.push(node);
        if !is_in_test_class(ctx, &scope, None) {
            return;
        }
        for member in members.iter() {
            let hint = match member {
                ClassLikeMember::Property(Property::Plain(p)) => p.hint.as_ref(),
                ClassLikeMember::Property(Property::Hooked(p)) => p.hint.as_ref(),
                _ => None,
            };
            let mut types = Vec::new();
            match hint {
                Some(hint @ (Hint::Union(_) | Hint::Intersection(_))) => flatten(hint, &mut types),
                _ => continue,
            }
            if types.len() != 2 {
                continue;
            }
            let Some(mock) = types.into_iter().find(|t| {
                matches!(t, Hint::Identifier(id) if resolve(&scope, id).eq_ignore_ascii_case(MOCK_OBJECT))
            }) else {
                continue;
            };
            let hint = hint.unwrap();
            out.push(report(
                node_span(hint),
                "Type a mock property with MockObject alone.",
                Safety::Safe,
                vec![Edit::new(node_span(hint), text(ctx, mock))],
            ));
        }
    }
}

/// The types of a union or an intersection, as php-parser lists them in its `UnionType` or `IntersectionType`.
fn flatten<'a>(hint: &'a Hint<'a>, out: &mut Vec<&'a Hint<'a>>) {
    match hint {
        Hint::Union(u) => {
            flatten_kind(u.left, true, out);
            flatten_kind(u.right, true, out);
        }
        Hint::Intersection(i) => {
            flatten_kind(i.left, false, out);
            flatten_kind(i.right, false, out);
        }
        _ => out.push(hint),
    }
}

/// A side of a union (`union`) or an intersection: a nested one of the same kind adds its types.
fn flatten_kind<'a>(hint: &'a Hint<'a>, union: bool, out: &mut Vec<&'a Hint<'a>>) {
    match hint {
        Hint::Union(_) if union => flatten(hint, out),
        Hint::Intersection(_) if !union => flatten(hint, out),
        _ => out.push(hint),
    }
}
