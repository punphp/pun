//! `lint/laravel/prefer-collection-all`, ported from rector-laravel 2.5.0's `ConvertEnumerableToArrayToAllRector`
//! (src/Rector/MethodCall).
//!
//! `toArray()` (or `?->toArray()`) on an `Illuminate\Support\Enumerable` becomes `all()` when the collection's values
//! (its `TValue`, through the class's `@extends` and `@implements`) cannot be `Arrayable`: `toArray()` only differs
//! from `all()` by turning `Arrayable` values into arrays. Values of a type PHPStan cannot rule out (`mixed`, an
//! interface, a class that is not final) keep `toArray()`.
//!
//! PHPStan narrows a receiver to a method's `@phpstan-this-out` type after the call; Mago does not, so a receiver
//! that gets such a call anywhere in the function keeps `toArray()` (Pun/skip_this_out: Rector may still change it).

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::get_specialized_template_type;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{Expression, Node, NodeKind};

use super::object_type::{Tri, is_object_type, object_class, object_super_of_union};
use super::{is_name, member_name, replace};
use crate::rules::types::facts::{Facts, facts};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

const ENUMERABLE: &str = "Illuminate\\Support\\Enumerable";

pub struct PreferCollectionAll;

impl Rule for PreferCollectionAll {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-collection-all"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall, NodeKind::NullSafeMethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (object, method) = match node {
            Node::MethodCall(c) => (c.object, &c.method),
            Node::NullSafeMethodCall(c) => (c.object, &c.method),
            _ => return,
        };
        if !member_name(method).is_some_and(|n| is_name(n, "toArray"))
            || !is_object_type(ctx, ancestors, object, ENUMERABLE)
        {
            return;
        }
        let facts = facts(ctx);
        let Some(t) = facts.full_type(node_span(object)) else {
            return;
        };
        if this_out_called(ctx, ancestors, object) {
            return;
        }
        let Some(values) = value_type(ctx, t) else {
            return;
        };
        if object_super_of_union(ctx, "Illuminate\\Contracts\\Support\\Arrayable", &values) != Tri::No {
            return;
        }
        out.push(replace(
            node_span(method),
            "Use `all()`: no value of the collection is `Arrayable`.".to_string(),
            "all".to_string(),
        ));
    }
}

/// `TypeCombinator::removeNull($type)->getTemplateType(Enumerable::class, 'TValue')`, docblocks read: the `TValue` of
/// each object part of the type, together.
fn value_type(ctx: &Context<'_>, t: &TUnion) -> Option<TUnion> {
    let facts = facts(ctx);
    let codebase = facts.documented();
    let enumerable = codebase.get_class_like(ENUMERABLE.as_bytes())?;
    let name = (enumerable.get_template_type_names().into_iter()).find(|n| n.as_bytes() == b"TValue")?;
    let mut types = Vec::new();
    for atomic in t.types.iter().filter(|a| !matches!(a, TAtomic::Null)) {
        let TAtomic::Object(mago_codex::ttype::atomic::object::TObject::Named(named)) = atomic else {
            return None;
        };
        let class = codebase.get_class_like(named.name.as_bytes())?;
        let value = get_specialized_template_type(
            codebase,
            name,
            enumerable.name,
            class,
            named.type_parameters.as_deref(),
        )?;
        types.extend(value.types.iter().cloned());
    }
    Some(TUnion::from_vec(types))
}

/// Whether the function `object` is in (or the file, outside functions) calls a method with a `this-out` tag (PHPDoc
/// `@phpstan-this-out`, `@psalm-this-out` or `@this-out`) on the same receiver, written the same way. PHPStan gives
/// the receiver that tag's type after the call (`Collection::put()` adds the value's type to `TValue`); Mago does not,
/// so the values `toArray()` sees are not known and it stays.
fn this_out_called(ctx: &Context<'_>, ancestors: &[Node<'_, '_>], object: &Expression<'_>) -> bool {
    let facts = facts(ctx);
    let text = |e: &Expression<'_>| &ctx.source[node_span(e).start..node_span(e).end];
    let receiver = text(unparenthesize(object));
    let Some(&root) = (ancestors.iter())
        .find(|a| matches!(a, Node::Function(_) | Node::Method(_)))
        .or(ancestors.first())
    else {
        return false;
    };
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        node.visit_children(|c| stack.push(c));
        let (called, method) = match node {
            Node::MethodCall(c) => (c.object, &c.method),
            Node::NullSafeMethodCall(c) => (c.object, &c.method),
            _ => continue,
        };
        let (Some(method), Some(t)) = (member_name(method), facts.full_type(node_span(called))) else {
            continue;
        };
        if text(unparenthesize(called)) != receiver {
            continue;
        }
        let tagged = (t.types.iter().filter_map(object_class))
            .any(|class| this_out_tagged(&facts, class.as_bytes(), method).is_some());
        if tagged {
            return true;
        }
    }
    false
}

/// `Some` when the method `method` of `class` has a `this-out` tag in its doc comment: the last doc comment before its
/// name, with only modifiers and attributes after it.
fn this_out_tagged(facts: &Facts, class: &[u8], method: &[u8]) -> Option<()> {
    let name = facts.codebase.get_declaring_method(class, method)?.name_span?;
    let source = facts.source_of(name.file_id)?;
    let before = &source[..name.start.offset as usize];
    let start = before.rfind("/**")?;
    let end = start + before[start..].find("*/")?;
    (!before[end..].contains([';', '{', '}'])
        && ["@phpstan-this-out", "@psalm-this-out", "@this-out"]
            .iter()
            .any(|tag| before[start..end].contains(tag)))
    .then_some(())
}
