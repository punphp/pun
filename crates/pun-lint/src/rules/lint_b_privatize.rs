//! What the three `PrivatizeFinalClass*Rector` ports share (Rector 2.6.1, rules/Privatization): a `protected` member
//! of a `final` class becomes `private`.
//!
//! Rector asks PHPStan's reflection whether a parent class, an interface or a trait declares or uses the member, and
//! skips a class whose parent or interfaces it cannot load. The syntax knows only the class itself, so Pun changes a
//! class that extends nothing and uses no trait, where no ancestor can hold the member (an interface's members are
//! public, so a `protected` one of the same name could not exist).

use mago_syntax::cst::{Class, ClassLikeMember, Modifier, Node, Sequence};
use pun_problem::{Edit, Span};

use super::rector_batch5::modifiers_text;
use crate::syntax::node_span;

/// The class when it is `final`, extends nothing and uses no trait.
pub fn final_class<'a>(node: Node<'a, '_>) -> Option<&'a Class<'a>> {
    let Node::Class(class) = node else {
        return None;
    };
    let is_final = class
        .modifiers
        .iter()
        .any(|m| matches!(m, Modifier::Final(_)));
    let uses_trait = class
        .members
        .iter()
        .any(|m| matches!(m, ClassLikeMember::TraitUse(_)));
    (is_final && class.extends.is_none() && !uses_trait).then_some(class)
}

/// `isProtected()`: the `protected` keyword (an asymmetric `protected(set)` is no visibility of its own).
pub fn is_protected(modifiers: &Sequence<'_, Modifier<'_>>) -> bool {
    modifiers
        .iter()
        .any(|m| matches!(m, Modifier::Protected(_)))
        && !modifiers.iter().any(|m| {
            matches!(
                m,
                Modifier::PublicSet(_) | Modifier::ProtectedSet(_) | Modifier::PrivateSet(_)
            )
        })
}

/// `VisibilityManipulator::makePrivate()` as php-parser prints a changed `flags`: the modifiers again in its order
/// (`pModifiers`), each followed by a space, in place of the old ones and the blanks after them.
pub fn make_private(source: &str, modifiers: &Sequence<'_, Modifier<'_>>) -> Edit {
    let start = node_span(modifiers.first().expect("a protected member has modifiers")).start;
    let end = node_span(modifiers.last().expect("a protected member has modifiers")).end;
    let rest = &source[end..];
    let next = end + (rest.len() - rest.trim_start().len());
    Edit::new(
        Span::new(start, next),
        modifiers_text(modifiers).replacen("protected ", "private ", 1),
    )
}
