//! `lint/refactor/rename-method`, ported from Rector 2.6.1's `RenameMethodRector` (rules/Renaming/Rector/MethodCall).
//!
//! Each rename names a class, an old method and a new one; the first rename that matches decides.
//!
//! - A call `$x->old()`, `$x?->old()` or `X::old()` (the name written as an identifier, without regard to case) on
//!   the class (`isObjectType` of the object or the class part, or an enum case of the class) gets the new name, unless
//!   the object's one class is another class than the rename's, which the codebase knows, and has both methods. With
//!   an `array-key`, the call is then read at that key (`$x->new()['key']`).
//! - In a class, an interface or a trait, a method named old gets the new name when the class is the rename's class,
//!   extends or implements it, or uses it as a trait (its own or a parent's), and has no method of the new name; an
//!   interface whose rename class has a parent or interface with the old method keeps it. A trait is matched as a
//!   class is (PHPStan's scope gives a trait declaration its reflection). An enum's methods are not renamed.
//!
//! A class's methods are the codebase's (its own, its parents', its traits' and its interfaces'); Pun's codebase has
//! no doc comments, so an `@method` tag is not a method.
//!
//! Options: `items = [{ class = "Nette\\Utils\\Html", old-method = "add", new-method = "addHtml" }]`, the
//! `MethodCallRename` value objects as `pun migrate` writes them, with `array-key` (a string or an integer) for a
//! `MethodCallRenameWithArrayKey`.

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::symbol::SymbolKind;
use mago_syntax::cst::{
    ClassLikeMember, ClassLikeMemberSelector, Expression, Identifier, Node, NodeKind, Sequence,
};
use pun_problem::{Edit, Span};
use serde_json::Value;

use super::super::class_method::qualified;
use super::super::types::facts::facts;
use super::{
    assert_class, assert_method, is_class, is_object_type, items, object, object_class_name, report, string,
};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// One `MethodCallRename` (`array_key` set for a `MethodCallRenameWithArrayKey`).
#[derive(Clone, Debug)]
pub struct MethodRename {
    pub class: String,
    pub old: String,
    pub new: String,
    pub array_key: Option<Value>,
}

#[derive(Default)]
pub struct RenameMethod {
    pub renames: Vec<MethodRename>,
}

/// Whether the codebase's class `class` has a method `name` (`ClassReflection::hasMethod` for native methods).
fn has_method(ctx: &Context<'_>, class: &ClassLikeMetadata, name: &str) -> bool {
    let codebase = &facts(ctx).codebase;
    let declares = |c: &ClassLikeMetadata| {
        c.declaring_method_ids
            .keys()
            .chain(c.appearing_method_ids.keys())
            .any(|m| m.as_bytes().eq_ignore_ascii_case(name.as_bytes()))
    };
    declares(class)
        || class
            .all_parent_interfaces
            .iter()
            .filter_map(|i| codebase.get_class_like(i.as_bytes()))
            .any(declares)
}

/// `isEnumTypeMatch`: `$call->var` is a case (a class constant fetch on a class name) of an enum named `class`.
fn is_enum_case_of(ctx: &Context<'_>, ancestors: &[Node<'_, '_>], var: &Expression<'_>, class: &str) -> bool {
    let Expression::Access(mago_syntax::cst::Access::ClassConstant(fetch)) = unparenthesize(var) else {
        return false;
    };
    let Expression::Identifier(id) = fetch.class else {
        return false;
    };
    if matches!(id, Identifier::Local(l) if [&b"self"[..], b"static", b"parent"].iter().any(|k| l.value.eq_ignore_ascii_case(k)))
    {
        return false;
    }
    let name = super::super::rector_batch5::resolve_class_name(ancestors, id);
    facts(ctx)
        .codebase
        .get_class_like(name.as_bytes())
        .is_some_and(|c| c.kind == SymbolKind::Enum && c.original_name.to_string() == class)
}

impl RenameMethod {
    fn visit_call(
        &self,
        node: Node<'_, '_>,
        ancestors: &[Node<'_, '_>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (receiver, selector, is_static) = match node {
            Node::MethodCall(c) => (c.object, &c.method, false),
            Node::NullSafeMethodCall(c) => (c.object, &c.method, false),
            Node::StaticMethodCall(c) => (c.class, &c.method, true),
            _ => return,
        };
        let ClassLikeMemberSelector::Identifier(name) = selector else {
            return;
        };
        let called = String::from_utf8_lossy(name.value);
        for rename in &self.renames {
            if !called.eq_ignore_ascii_case(&rename.old) {
                continue;
            }
            let matches = match is_static {
                true => is_object_type(ctx, ancestors, receiver, &rename.class),
                false => {
                    is_enum_case_of(ctx, ancestors, receiver, &rename.class)
                        || is_object_type(ctx, ancestors, receiver, &rename.class)
                }
            };
            if !matches || self.skips_call(ctx, receiver, rename) {
                continue;
            }
            let mut edits = vec![Edit::new(node_span(name), rename.new.clone())];
            if let Some(key) = &rename.array_key {
                let end = node_span(&node).end;
                edits.push(Edit::new(Span::new(end, end), format!("[{}]", key_text(key))));
            }
            out.push(report(
                node_span(&node),
                format!("Call `{}()` instead of `{}()`.", rename.new, rename.old),
                edits,
            ));
            return;
        }
    }

    /// `shouldSkipClassMethod`: the class that declares the old method on the receiver's one class
    /// (`resolveClassReflectionSourceObject`: the receiver's class itself for a private method; a trait's method is
    /// the using class's) is another known class than the rename's, and has the new method too.
    fn skips_call(&self, ctx: &Context<'_>, receiver: &Expression<'_>, rename: &MethodRename) -> bool {
        let facts = facts(ctx);
        let codebase = &facts.codebase;
        let Some(class) = object_class_name(ctx, receiver).and_then(|n| codebase.get_class_like(n.as_bytes())) else {
            return false;
        };
        let Some((_, id)) = (class.appearing_method_ids.iter())
            .find(|(m, _)| m.as_bytes().eq_ignore_ascii_case(rename.old.as_bytes()))
        else {
            return false;
        };
        let declaring = id.get_class_name();
        let private = (codebase.get_declaring_method(declaring.as_bytes(), rename.old.as_bytes()))
            .and_then(|m| m.method_metadata.as_ref())
            .is_some_and(|m| m.visibility.is_private());
        let source = match private {
            true => class,
            false => codebase.get_class_like(declaring.as_bytes()).unwrap_or(class),
        };
        let Some(target) = codebase.get_class_like(rename.class.as_bytes()) else {
            return false;
        };
        source.original_name != target.original_name && has_method(ctx, source, &rename.new)
    }

    fn visit_class_like(
        &self,
        node: Node<'_, '_>,
        ancestors: &[Node<'_, '_>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (name, members, kind): (&[u8], &Sequence<'_, ClassLikeMember<'_>>, SymbolKind) = match node {
            Node::Class(c) => (c.name.value, &c.members, SymbolKind::Class),
            Node::Interface(i) => (i.name.value, &i.members, SymbolKind::Interface),
            Node::Trait(t) => (t.name.value, &t.members, SymbolKind::Trait),
            _ => return,
        };
        let codebase = &facts(ctx).codebase;
        let fqcn = qualified(ancestors, name);
        let own = codebase.get_class_like(fqcn.as_bytes());
        let declares = |method: &str| {
            members.iter().any(|m| {
                matches!(m, ClassLikeMember::Method(m) if m.name.value.eq_ignore_ascii_case(method.as_bytes()))
            })
        };
        let mut edits = Vec::new();
        for member in members.iter() {
            let ClassLikeMember::Method(method) = member else {
                continue;
            };
            let method_name = String::from_utf8_lossy(method.name.value);
            let renamed = self.renames.iter().find(|rename| {
                if !method_name.eq_ignore_ascii_case(&rename.old) {
                    return false;
                }
                let Some(own) = own else { return false };
                // `isMethodStaticCallOrClassMethodObjectType` on the scope's class.
                let of_type = is_class(ctx, &own.original_name.to_string(), &rename.class)
                    || uses_trait(ctx, own, &rename.class);
                if !of_type {
                    return false;
                }
                // `shouldKeepForParentInterface`.
                if kind == SymbolKind::Interface
                    && codebase.get_class_like(rename.class.as_bytes()).is_some_and(|c| {
                        c.all_parent_classes
                            .iter()
                            .chain(c.all_parent_interfaces.iter())
                            .filter_map(|p| codebase.get_class_like(p.as_bytes()))
                            .any(|p| has_method(ctx, p, &rename.old))
                    })
                {
                    return false;
                }
                !declares(&rename.new)
            });
            if let Some(rename) = renamed {
                edits.push(Edit::new(node_span(&method.name), rename.new.clone()));
            }
        }
        if edits.is_empty() {
            return;
        }
        let span = match node {
            Node::Class(c) => node_span(&c.name),
            Node::Interface(i) => node_span(&i.name),
            Node::Trait(t) => node_span(&t.name),
            _ => return,
        };
        out.push(report(span, "Rename the methods the configuration renames.".to_string(), edits));
    }
}

/// `ClassReflection::hasTraitUse`: the traits the class and its parent classes use themselves.
fn uses_trait(ctx: &Context<'_>, class: &ClassLikeMetadata, r#trait: &str) -> bool {
    let codebase = &facts(ctx).codebase;
    let direct = |c: &ClassLikeMetadata| {
        (c.used_traits.iter()).any(|t| t.as_bytes().eq_ignore_ascii_case(r#trait.as_bytes()))
    };
    direct(class)
        || class
            .all_parent_classes
            .iter()
            .filter_map(|p| codebase.get_class_like(p.as_bytes()))
            .any(direct)
}

/// `BuilderHelpers::normalizeValue` printed: a string in single quotes, an integer as it is.
fn key_text(key: &Value) -> String {
    match key {
        Value::String(s) => format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'")),
        other => other.to_string(),
    }
}

impl Rule for RenameMethod {
    fn id(&self) -> &'static str {
        "lint/refactor/rename-method"
    }

    fn configure(&mut self, options: &Value) -> Result<(), String> {
        const KEYS: &[&str] = &["class", "old-method", "new-method", "array-key"];
        let mut renames = Vec::new();
        for item in items(options)? {
            let map = object(item, KEYS, &KEYS[..3])?;
            let rename = MethodRename {
                class: string(map, "class")?,
                old: string(map, "old-method")?,
                new: string(map, "new-method")?,
                array_key: match map.get("array-key") {
                    None => None,
                    Some(key @ Value::String(_)) => Some(key.clone()),
                    Some(key) if key.is_i64() => Some(key.clone()),
                    Some(_) => return Err("`array-key` must be a string or an integer".into()),
                },
            };
            assert_class(&rename.class)?;
            assert_method(&rename.old)?;
            assert_method(&rename.new)?;
            renames.push(rename);
        }
        self.renames = renames;
        Ok(())
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::MethodCall,
            NodeKind::NullSafeMethodCall,
            NodeKind::StaticMethodCall,
            NodeKind::Class,
            NodeKind::Interface,
            NodeKind::Trait,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        if self.renames.is_empty() {
            return;
        }
        match node {
            Node::Class(_) | Node::Interface(_) | Node::Trait(_) => {
                self.visit_class_like(node, ancestors, ctx, out)
            }
            _ => self.visit_call(node, ancestors, ctx, out),
        }
    }
}
