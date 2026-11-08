//! `lint/laravel/no-useless-with-call`, ported from rector-laravel 2.7.3's `RemoveRedundantWithCallsRector`
//! (src/Rector/FuncCall).
//!
//! `with($x)` becomes `$x`; so does `with($x, $y)` when `$y`'s type cannot be callable (`isCallable()->no()`): an
//! integer, a float, a boolean, `null`, a string literal that names no function or static method, or an object of a final
//! class the codebase knows without `__invoke()`. Any other string, an array, an object of another class, `mixed`, a
//! value of no known type or `never` (Mago's type for `new` of an unknown class) may be callable, and the call stays.
//! An unpacked first argument (`with(...$list)`) stays too, where Rector writes the array in place of its first item.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{Node, NodeKind};

use super::print::moved_value;
use super::replace;
use crate::rules::rector_code_quality::named;
use crate::rules::types::facts::{Facts, facts};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoUselessWithCall;

impl Rule for NoUselessWithCall {
    fn id(&self) -> &'static str {
        "lint/laravel/no-useless-with-call"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(c) = node else { return };
        if !named(c, "with") {
            return;
        }
        let value = match c.argument_list.arguments.as_slice() {
            [value] => value,
            [value, callback] => {
                let facts = facts(ctx);
                let callback = node_span(unparenthesize(callback.value()));
                if (facts.full_type(callback)).is_none_or(|t| may_be_callable(&facts, t)) {
                    return;
                }
                value
            }
            _ => return,
        };
        // Rector puts the unpacked array in place of its first item; Pun leaves the c.
        if value.is_unpacked() {
            return;
        }
        let span = node_span(c);
        out.push(replace(
            span,
            "Remove the `with()` c: it only returns its argument.".to_string(),
            moved_value(ctx, span, ancestors, unparenthesize(value.value())),
        ));
    }
}

/// Whether a value of type `t` may be callable.
fn may_be_callable(facts: &Facts, t: &TUnion) -> bool {
    t.types.iter().any(|a| match a {
        TAtomic::Null | TAtomic::Void => false,
        TAtomic::Scalar(TScalar::Integer(_) | TScalar::Float(_) | TScalar::Bool(_)) => false,
        TAtomic::Scalar(s @ TScalar::String(_)) => match s.get_known_literal_string_value() {
            Some(value) => is_callable_string(facts, value),
            None => true,
        },
        TAtomic::Object(TObject::Named(n)) => (facts.codebase.get_class_like(n.name.as_bytes()))
            .is_none_or(|class| {
                let invoke = facts
                    .codebase
                    .get_declaring_method(class.name.as_bytes(), b"__invoke");
                !class.flags.is_final() || invoke.is_some()
            }),
        _ => true,
    })
}

/// Whether `ConstantStringType::isCallable()` is not `no`: the string names a function, or is `Class::method` with a
/// static method of a class the codebase knows, a class it does not know, or a class not declared `final` without the
/// method.
fn is_callable_string(facts: &Facts, value: &[u8]) -> bool {
    if facts.codebase.get_function(value).is_some() {
        return true;
    }
    let Some(at) = value.windows(2).position(|w| w == b"::") else {
        return false;
    };
    let (class, method) = (&value[..at], &value[at + 2..]);
    let word = |b: &u8, first: bool| {
        b.is_ascii_alphabetic() || *b == b'_' || *b >= 0x7f || (!first && b.is_ascii_digit())
    };
    let valid_class = class
        .iter()
        .enumerate()
        .all(|(i, b)| word(b, i == 0) || *b == b'\\');
    let valid_method = method.iter().enumerate().all(|(i, b)| word(b, i == 0));
    if class.is_empty() || method.is_empty() || !valid_class || !valid_method {
        return false;
    }
    let class = class.strip_prefix(b"\\").unwrap_or(class);
    let Some(metadata) = facts.codebase.get_class_like(class) else {
        return true;
    };
    match facts.codebase.get_declaring_method(class, method) {
        Some(m) => m.method_metadata.as_ref().is_some_and(|m| m.is_static),
        None => !metadata.flags.is_final(),
    }
}
