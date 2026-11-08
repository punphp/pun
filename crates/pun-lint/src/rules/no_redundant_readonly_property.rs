//! `lint/classes/no-redundant-readonly-property`, ported from Rector 2.8.4's
//! `RemoveReadonlyPropertyVisibilityOnReadonlyClassRector` (rules/CodeQuality/Rector/Class_) with
//! `VisibilityManipulator::removeReadonly`.
//!
//! In a `readonly` class every property is readonly, so `readonly` on a property or a promoted constructor parameter
//! says nothing: it goes. A property or parameter left without a visibility becomes `public`, which it was. php-parser
//! prints the changed modifiers again in its own order (`pModifiers`), which Pun does too.

use mago_syntax::cst::{
    ClassLikeMember, HookedProperty, Modifier, Node, NodeKind, PlainProperty, Property, Sequence,
};
use pun_problem::{Edit, Fix, Safety, Span};

use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoRedundantReadonlyProperty;

impl Rule for NoRedundantReadonlyProperty {
    fn id(&self) -> &'static str {
        "lint/classes/no-redundant-readonly-property"
    }

    fn min_php_version(&self) -> u32 {
        // PhpVersionFeature::READONLY_CLASS
        80200
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class, NodeKind::AnonymousClass])
    }

    fn visit<'ast, 'arena>(
        &self,
        n: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (modifiers, members) = match n {
            Node::Class(c) => (&c.modifiers, &c.members),
            _ => {
                let Node::AnonymousClass(c) = n else { return };
                (&c.modifiers, &c.members)
            }
        };
        if !has_readonly(modifiers) {
            return;
        }
        // Class_::getMethod('__construct'): the first method of that name, in any case.
        let constructor = members.iter().find_map(|m| match m {
            ClassLikeMember::Method(m) if m.name.value.eq_ignore_ascii_case(b"__construct") => {
                Some(m)
            }
            _ => None,
        });
        if let Some(constructor) = constructor {
            for param in constructor.parameter_list.parameters.iter() {
                if has_readonly(&param.modifiers) {
                    // Param::isPromoted() after removing `readonly`: any modifier left, or hooks.
                    let promoted = param.modifiers.len() > 1 || param.hooks.is_some();
                    report(ctx, &param.modifiers, !promoted, out);
                }
            }
        }
        for member in members.iter() {
            let ClassLikeMember::Property(property) = member else {
                continue;
            };
            let (Property::Plain(PlainProperty { modifiers, .. })
            | Property::Hooked(HookedProperty { modifiers, .. })) = property;
            if has_readonly(modifiers) {
                // VisibilityManipulator::publicize: a property without `public`, `protected` or `private` is public
                // and gets `public` written.
                let visible = modifiers.iter().any(|m| {
                    matches!(
                        m,
                        Modifier::Public(_) | Modifier::Protected(_) | Modifier::Private(_)
                    )
                });
                report(ctx, modifiers, !visible, out);
            }
        }
    }
}

fn has_readonly(modifiers: &Sequence<'_, Modifier<'_>>) -> bool {
    modifiers.iter().any(|m| matches!(m, Modifier::Readonly(_)))
}

/// The edit that prints `modifiers` again without `readonly`, with `public` added when `add_public`.
fn report(
    ctx: &Context<'_>,
    modifiers: &Sequence<'_, Modifier<'_>>,
    add_public: bool,
    out: &mut Vec<Report>,
) {
    let readonly = modifiers
        .iter()
        .find(|m| matches!(m, Modifier::Readonly(_)))
        .expect("checked by the caller");
    let first = node_span(modifiers.first().expect("not empty"));
    let last = node_span(modifiers.last().expect("not empty"));
    let rest = &ctx.source[last.end..];
    let end = last.end + (rest.len() - rest.trim_start().len());
    out.push(Report {
        span: node_span(readonly),
        message: "The class is readonly, so `readonly` on its properties is redundant.".to_string(),
        help: None,
        fix: Some(Fix {
            safety: Safety::Safe,
            edits: vec![Edit::new(
                Span::new(first.start, end),
                print_modifiers(modifiers, add_public),
            )],
        }),
    });
}

/// php-parser's `pModifiers` for the flags of `modifiers` without `readonly`: each modifier and a space, in its fixed
/// order.
pub(super) fn print_modifiers(modifiers: &Sequence<'_, Modifier<'_>>, add_public: bool) -> String {
    let has = |f: fn(&Modifier<'_>) -> bool| modifiers.iter().any(f);
    let order: [(&str, bool); 9] = [
        ("final", has(|m| matches!(m, Modifier::Final(_)))),
        ("abstract", has(|m| matches!(m, Modifier::Abstract(_)))),
        (
            "public",
            add_public || has(|m| matches!(m, Modifier::Public(_))),
        ),
        ("protected", has(|m| matches!(m, Modifier::Protected(_)))),
        ("private", has(|m| matches!(m, Modifier::Private(_)))),
        ("public(set)", has(|m| matches!(m, Modifier::PublicSet(_)))),
        (
            "protected(set)",
            has(|m| matches!(m, Modifier::ProtectedSet(_))),
        ),
        (
            "private(set)",
            has(|m| matches!(m, Modifier::PrivateSet(_))),
        ),
        ("static", has(|m| matches!(m, Modifier::Static(_)))),
    ];
    order
        .iter()
        .filter(|(_, on)| *on)
        .map(|(name, _)| format!("{name} "))
        .collect()
}
