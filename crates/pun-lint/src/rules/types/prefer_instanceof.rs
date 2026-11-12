//! `lint/types/prefer-instanceof`, ported from Rector 2.8.4's `InlineIsAInstanceOfRector`
//! (rules/CodeQuality/Rector/FuncCall): `is_a($object, X::class)` on an object becomes `$object instanceof X`.
//!
//! The first argument's type (`getType`, see [`super::rector_type`]) must be one object type: a class, `object`, an
//! enum or a closure, not a union, an intersection or `$this`. The second is a variable, kept as written, or a class
//! constant whose type names one class: `X::class` (and any constant whose value is a class name), spelled as the
//! class is declared, `self`, `static` and `parent` of a named class-like, and `$object::class` of a final class.
//! Rector writes the class fully qualified.
//!
//! Where the new `instanceof` needs parentheses around its left side (`($a ?? $b) instanceof X`) or around itself
//! (`(string) ($a instanceof X)`), Pun writes them; Rector's printer leaves out the first pair around a short ternary
//! (`!$a ?: $b instanceof X`), which changes what the code does (lint.md: meaning over bytes).

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_syntax::cst::{
    Argument, BinaryOperator, ClassLikeConstantSelector, Expression, Node, NodeKind,
    UnaryPrefixOperator,
};
use pun_problem::{Edit, Safety};

use super::facts::{Facts, facts};
use super::names::resolve_class_name;
use super::rector_type::{RectorType, get_type};
use super::{ClassScope, class_scope};
use crate::rules::rector_batch5::called_function_name;
use crate::rules::rector_code_quality::text;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferInstanceof;

impl Rule for PreferInstanceof {
    fn id(&self) -> &'static str {
        "lint/types/prefer-instanceof"
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
        let Node::FunctionCall(call) = node else {
            return;
        };
        if !called_function_name(ancestors, call.function)
            .is_some_and(|n| n.eq_ignore_ascii_case("is_a"))
        {
            return;
        }
        let values: Vec<&Expression<'_>> = (call.argument_list.arguments.iter())
            .map(|a| match a {
                Argument::Positional(p) if p.ellipsis.is_none() => Some(p.value),
                Argument::Named(n) => Some(n.value),
                // a spread argument: Rector writes `$a instanceof $spread` of `is_a($a, ...$spread)`
                Argument::Positional(_) => None,
            })
            .collect::<Option<_>>()
            .unwrap_or_default();
        let [object, class, ..] = values[..] else {
            return;
        };
        let facts = facts(ctx);
        if !is_object_type(&facts, object) {
            return;
        }
        let class = match unparenthesize(class) {
            Expression::Variable(v) => text(ctx, v).to_string(),
            Expression::Access(mago_syntax::cst::Access::ClassConstant(fetch)) => {
                let Some(name) = class_name(&facts, ancestors, fetch) else {
                    return;
                };
                format!("\\{name}")
            }
            _ => return,
        };
        let object = unparenthesize(object);
        let left = text(ctx, object);
        let left = if needs_parentheses(object) {
            format!("({left})")
        } else {
            left.to_string()
        };
        let mut text = format!("{left} instanceof {class}");
        let parent = (ancestors.iter().rev().copied())
            .find(|n| !matches!(n, Node::Expression(_) | Node::Call(_)));
        if wraps(parent) {
            text = format!("({text})");
        }
        out.push(Report::fixed(
            node_span(call),
            "Use `instanceof` instead of `is_a()` on an object.",
            Safety::Unsafe,
            vec![Edit::new(node_span(call), text)],
        ));
    }
}

/// `isFirstObjectType`: an `ObjectType` (a class, an enum, a closure, an anonymous class; generics allowed) or
/// `ObjectWithoutClassType`.
fn is_object_type(facts: &Facts, expr: &Expression<'_>) -> bool {
    let RectorType::Scope(Some(t)) = get_type(facts, expr) else {
        return false;
    };
    match t.types.as_ref() {
        [TAtomic::Object(TObject::Named(named))] => {
            named.intersection_types.is_none() && !named.is_this && !named.is_static
        }
        [TAtomic::Object(TObject::Any | TObject::Enum(_))] => true,
        [TAtomic::Callable(c)] => c.is_closure(),
        _ => false,
    }
}

/// `resolveClassName`: the one class the type of the class constant `fetch` names (`getObjectClassNames()` of its
/// type, a string naming a class as declared made a `class-string` of it by Rector's `GenericClassStringTypeCorrector`).
fn class_name(
    facts: &Facts,
    ancestors: &[Node<'_, '_>],
    fetch: &mago_syntax::cst::ClassConstantAccess<'_>,
) -> Option<String> {
    let ClassLikeConstantSelector::Identifier(constant) = &fetch.constant else {
        return None;
    };
    if !constant.value.eq_ignore_ascii_case(b"class") {
        // a constant whose value names a class as declared
        let t = facts.full_type(node_span(fetch))?;
        let [TAtomic::Scalar(TScalar::String(s))] = t.types.as_ref() else {
            return None;
        };
        return declared(
            facts,
            &String::from_utf8_lossy(s.get_known_literal_value()?),
        );
    }
    let scope = || class_scope(ancestors).filter(|s| s.name.is_some());
    match unparenthesize(fetch.class) {
        Expression::Identifier(id) => {
            let written = String::from_utf8_lossy(id.value());
            let name = match written.get(..10) {
                Some(prefix) if prefix.eq_ignore_ascii_case("namespace\\") => {
                    match super::namespace_of(ancestors) {
                        Some(ns) => format!("{ns}\\{}", &written[10..]),
                        None => written[10..].to_string(),
                    }
                }
                _ => resolve_class_name(ancestors, id),
            };
            declared(facts, &name)
        }
        Expression::Self_(_) | Expression::Static(_) => scope()?.name,
        Expression::Parent(_) => declared(facts, &scope()?.parent_class?),
        object => final_class(facts, object, scope()),
    }
}

/// `$object::class`: a `class-string` of the object's class when that class is final, a `string` otherwise.
fn final_class(
    facts: &Facts,
    object: &Expression<'_>,
    scope: Option<ClassScope>,
) -> Option<String> {
    let t = facts.full_type(node_span(object))?;
    let name = match t.types.as_ref() {
        [TAtomic::Object(TObject::Named(named))] if named.intersection_types.is_none() => {
            if named.is_this {
                let scope = scope?;
                if !scope.is_final {
                    return None;
                }
                return scope.name;
            }
            named.name.to_string()
        }
        [TAtomic::Object(TObject::Enum(e))] => e.get_name().to_string(),
        _ => return None,
    };
    let class = facts.codebase.get_class_like(name.as_bytes())?;
    (class.flags.is_final() || class.kind == mago_codex::symbol::SymbolKind::Enum)
        .then(|| class.original_name.to_string())
}

/// `name` when a class-like is declared under exactly that name (Rector's corrector compares the reflection's
/// name with the string).
fn declared(facts: &Facts, name: &str) -> Option<String> {
    let class = facts.codebase.get_class_like(name.as_bytes())?;
    (class.original_name.as_bytes() == name.as_bytes()).then(|| name.to_string())
}

/// Whether the left side of `instanceof` needs parentheses: an operation that binds looser than `instanceof`.
pub(super) fn needs_parentheses(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::Binary(b) => !matches!(b.operator, BinaryOperator::Exponentiation(_)),
        Expression::UnaryPrefix(u) => matches!(u.operator, UnaryPrefixOperator::Not(_)),
        Expression::Conditional(_)
        | Expression::Assignment(_)
        | Expression::Yield(_)
        | Expression::Throw(_)
        | Expression::Pipe(_)
        | Expression::ArrowFunction(_)
        | Expression::Construct(_) => true,
        _ => false,
    }
}

/// Whether the `instanceof` that takes the call's place needs parentheses there: under an operator that binds
/// tighter (a cast, `@`, `-`, `+`, `~`, `**`).
fn wraps(parent: Option<Node<'_, '_>>) -> bool {
    match parent {
        Some(Node::UnaryPrefix(u)) => !matches!(u.operator, UnaryPrefixOperator::Not(_)),
        Some(Node::Binary(b)) => matches!(
            b.operator,
            BinaryOperator::Exponentiation(_) | BinaryOperator::Instanceof(_)
        ),
        _ => false,
    }
}
