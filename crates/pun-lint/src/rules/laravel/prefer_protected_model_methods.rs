//! `lint/laravel/prefer-protected-model-methods`, ported from rector-laravel 2.7.3's
//! `MakeModelAttributesAndScopesProtectedRector` (src/Rector/ClassMethod), with `ScopeAnalyzer::isScopeMethod`
//! (src/NodeAnalyzer/ScopeAnalyzer.php).
//!
//! A public, non-static method of an Eloquent model (or of a trait) becomes `protected` when it is an attribute
//! accessor (`get…Attribute`, `set…Attribute`, or one that returns `Illuminate\Database\Eloquent\Casts\Attribute`) or
//! a scope (`scope…` whose first parameter is untyped or an Eloquent builder, or one with the `#[Scope]` attribute).
//! A method the parent class has as a public method (its own, an `@method`, or one of a `@mixin`) stays.

use mago_syntax::cst::{Method, Modifier, Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::is_a;
use super::object_type::{is_hint_type, object_class};
use crate::rules::rector_batch5::has_attribute;
use crate::rules::types::facts::facts;
use crate::rules::types::{ClassScope, class_scope};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

const MODEL: &str = "Illuminate\\Database\\Eloquent\\Model";

pub struct PreferProtectedModelMethods;

impl Rule for PreferProtectedModelMethods {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-protected-model-methods"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Method(method) = node else { return };
        let has = |f: fn(&Modifier<'_>) -> bool| method.modifiers.iter().any(f);
        if has(|m| matches!(m, Modifier::Protected(_) | Modifier::Static(_)))
            || !(is_attribute_method(ctx, ancestors, method)
                || is_scope_method(ctx, ancestors, method))
        {
            return;
        }
        let Some(ClassScope {
            name: Some(own),
            parent_class,
            ..
        }) = class_scope(ancestors)
        else {
            return;
        };
        let is_trait = ancestors.iter().rev().find_map(|a| match a {
            Node::Class(_) | Node::AnonymousClass(_) | Node::Interface(_) | Node::Enum(_) => {
                Some(false)
            }
            Node::Trait(_) => Some(true),
            _ => None,
        }) == Some(true);
        if let Some(parent) = &parent_class
            && has_public_method(ctx, parent, method.name.value, &mut Vec::new())
        {
            return;
        }
        if !is_trait && !is_a(ctx, &own, &[MODEL]) {
            return;
        }
        out.push(Report::fixed(
            node_span(&method.name),
            "Make the model's attribute or scope method protected.",
            Safety::Unsafe,
            vec![make_protected(ctx.source, method)],
        ));
    }
}

/// `isAttributeMethod`: a name matching `/^[gs]et.+Attribute$/`, or a return type that is the `Attribute` cast.
fn is_attribute_method(ctx: &Context<'_>, ancestors: &[Node<'_, '_>], method: &Method<'_>) -> bool {
    let name = method.name.value;
    if name.len() > b"getAttribute".len()
        && (name.starts_with(b"get") || name.starts_with(b"set"))
        && name.ends_with(b"Attribute")
    {
        return true;
    }
    method.return_type_hint.as_ref().is_some_and(|r| {
        is_hint_type(
            ctx,
            ancestors,
            &r.hint,
            "Illuminate\\Database\\Eloquent\\Casts\\Attribute",
        )
    })
}

/// `ScopeAnalyzer::isScopeMethod`: a name matching `/^scope[A-Z].+$/` whose first parameter is untyped or an Eloquent
/// builder, or the `#[Scope]` attribute.
fn is_scope_method(ctx: &Context<'_>, ancestors: &[Node<'_, '_>], method: &Method<'_>) -> bool {
    let name = method.name.value;
    let named = name.len() > b"scopeA".len()
        && name.starts_with(b"scope")
        && name[5].is_ascii_uppercase()
        && method.parameter_list.parameters.first().is_some_and(|p| {
            p.hint.as_ref().is_none_or(|h| {
                is_hint_type(ctx, ancestors, h, "Illuminate\\Database\\Eloquent\\Builder")
            })
        });
    named
        || has_attribute(
            ancestors,
            &method.attribute_lists,
            "Illuminate\\Database\\Eloquent\\Attributes\\Scope",
        )
}

/// `$parentClass->hasMethod($name) && $parentClass->getMethod($name)->isPublic()`, docblocks read: the class has the
/// method (its own, inherited or an `@method` one) and it is public, or else every class of one of its `@mixin` types (inherited ones
/// too) has it public, as PHPStan's `MixinMethodsClassReflectionExtension` finds it.
fn has_public_method(ctx: &Context<'_>, class: &str, name: &[u8], seen: &mut Vec<String>) -> bool {
    let facts = facts(ctx);
    let codebase = facts.documented();
    if codebase.method_exists(class.as_bytes(), name) {
        return codebase
            .get_declaring_method(class.as_bytes(), name)
            .and_then(|m| m.method_metadata.as_ref())
            .is_some_and(|m| m.visibility.is_public());
    }
    if seen.iter().any(|s| s.eq_ignore_ascii_case(class)) {
        return false;
    }
    seen.push(class.to_string());
    let Some(meta) = codebase.get_class_like(class.as_bytes()) else {
        return false;
    };
    meta.mixins.iter().any(|mixin| {
        let classes: Vec<Option<String>> =
            mixin.type_union.types.iter().map(object_class).collect();
        !classes.is_empty()
            && classes.iter().all(|c| {
                c.as_deref()
                    .is_some_and(|c| has_public_method(ctx, c, name, seen))
            })
    })
}

/// `VisibilityManipulator::makeProtected()` as php-parser prints a changed `flags`: the modifiers again in its order
/// (`pModifiers`), with `protected` for the old visibility, in place of the old ones and the blanks after them, or
/// before `function` when there were none.
fn make_protected(source: &str, method: &Method<'_>) -> Edit {
    let has = |f: fn(&Modifier<'_>) -> bool| method.modifiers.iter().any(f);
    let text = [
        (has(|m| matches!(m, Modifier::Final(_))), "final "),
        (has(|m| matches!(m, Modifier::Abstract(_))), "abstract "),
        (true, "protected "),
    ]
    .iter()
    .filter(|(on, _)| *on)
    .map(|(_, t)| *t)
    .collect::<String>();
    let Some(first) = method.modifiers.first() else {
        let at = method.function.span.start.offset as usize;
        return Edit::new(Span::new(at, at), text);
    };
    let end = node_span(method.modifiers.last().unwrap_or(first)).end;
    let rest = &source[end..];
    Edit::new(
        Span::new(
            node_span(first).start,
            end + rest.len() - rest.trim_start().len(),
        ),
        text,
    )
}
