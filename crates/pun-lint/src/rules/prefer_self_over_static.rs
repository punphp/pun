//! `lint/classes/prefer-self-over-static`, ported from Rector 2.8.4's `ConvertStaticToSelfRector`
//! (rules/CodeQuality/Rector/Class_).
//!
//! `static::` becomes `self::` where late static binding cannot pick another member: in a `final` class for a static
//! method, a static property or a constant; elsewhere for a member that is `private` or `final`.
//!
//! Rector reads the member from PHPStan's reflection, inherited members included. The syntax knows the members the
//! class declares itself, so Pun leaves a `static::` alone when the member is not one of them. Rector also walks
//! nested anonymous classes against the outer class; there `static` is the anonymous class, so Pun stops at them.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferSelfOverStatic;

impl Rule for PreferSelfOverStatic {
    fn id(&self) -> &'static str {
        "lint/classes/prefer-self-over-static"
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
        let is_final = class
            .modifiers
            .iter()
            .any(|m| matches!(m, Modifier::Final(_)));
        let mut edits = Vec::new();
        for member in &class.members {
            accesses(Node::ClassLikeMember(member), &mut |access| {
                if let Some(span) = convertible(class, is_final, access) {
                    edits.push(Edit::new(span, "self"));
                }
            });
        }
        let Some(first) = edits.first() else {
            return;
        };
        out.push(Report {
            span: first.span,
            message: "Use `self::` where late static binding cannot change the member.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// A `static::` access: the member's kind and name.
enum Access<'a> {
    Property(&'a [u8]),
    Method(&'a [u8]),
    Constant(&'a [u8]),
}

/// Every `static::method()`, `static::$property` and `static::CONSTANT` under `node`, outside nested classes, with the
/// span of its `static`.
fn accesses<'ast, 'arena>(
    node: Node<'ast, 'arena>,
    f: &mut impl FnMut((pun_problem::Span, Access<'ast>)),
) {
    let found = match node {
        Node::Class(_)
        | Node::AnonymousClass(_)
        | Node::Interface(_)
        | Node::Trait(_)
        | Node::Enum(_) => return,
        Node::StaticMethodCall(c) => match &c.method {
            ClassLikeMemberSelector::Identifier(id) => Some((c.class, Access::Method(id.value))),
            _ => None,
        },
        Node::StaticPropertyAccess(a) => match &a.property {
            Variable::Direct(v) => Some((a.class, Access::Property(&v.name[1..]))),
            _ => None,
        },
        Node::ClassConstantAccess(a) => match &a.constant {
            ClassLikeConstantSelector::Identifier(id) => {
                Some((a.class, Access::Constant(id.value)))
            }
            _ => None,
        },
        _ => None,
    };
    if let Some((Expression::Static(keyword), access)) = found {
        f((node_span(keyword), access));
    }
    node.visit_children(|child| accesses(child, f));
}

/// `shouldSkip()` inverted: the span of `static` when it can become `self`.
fn convertible(
    class: &Class<'_>,
    is_final: bool,
    (span, access): (pun_problem::Span, Access<'_>),
) -> Option<pun_problem::Span> {
    let modifiers = class
        .members
        .iter()
        .find_map(|member| match (member, &access) {
            (ClassLikeMember::Method(m), Access::Method(name))
                if m.name.value.eq_ignore_ascii_case(name) =>
            {
                Some(&m.modifiers)
            }
            (ClassLikeMember::Property(Property::Plain(p)), Access::Property(name))
                if p.items.iter().any(|i| &i.variable().name[1..] == *name) =>
            {
                Some(&p.modifiers)
            }
            (ClassLikeMember::Property(Property::Hooked(p)), Access::Property(name))
                if &p.item.variable().name[1..] == *name =>
            {
                Some(&p.modifiers)
            }
            (ClassLikeMember::Constant(c), Access::Constant(name))
                if c.items.iter().any(|i| i.name.value == *name) =>
            {
                Some(&c.modifiers)
            }
            _ => None,
        })?;
    let has = |pick: fn(&Modifier<'_>) -> bool| modifiers.iter().any(pick);
    // A constant is static; a method or property must be.
    if !matches!(access, Access::Constant(_)) && !has(|m| matches!(m, Modifier::Static(_))) {
        return None;
    }
    let keep = !is_final
        && !has(|m| matches!(m, Modifier::Final(_)))
        && !has(|m| matches!(m, Modifier::Private(_)));
    (!keep).then_some(span)
}
