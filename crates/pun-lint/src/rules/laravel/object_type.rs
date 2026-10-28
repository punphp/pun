//! Rector's `NodeTypeResolver::isObjectType` (rector/rector src/NodeTypeResolver/NodeTypeResolver.php) on Mago's
//! types: whether an expression, a class name or a class declaration is an object of a given class, the way the
//! rector-laravel rules ask it. The type is PHPStan's `getType()`, docblocks read (`Facts::full_type`); a class name
//! is the object of the class it names (`NameTypeResolver`, `self` and `static` the enclosing class, `parent` its
//! parent).

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::reference::TReference;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{Access, Expression, Hint, Node, Variable};

use super::is_a;
use super::names::resolve_class_name;
use crate::Context;
use crate::rules::types::class_scope;
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};

/// `isObjectType($expr, new ObjectType($class))`.
pub(crate) fn is_object_type(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    expr: &Expression<'_>,
    class: &str,
) -> bool {
    match unparenthesize(expr) {
        Expression::Identifier(id) => {
            name_class(ancestors, id).is_some_and(|name| is_a(ctx, &name, &[class]))
        }
        Expression::Access(Access::ClassConstant(_)) => false,
        Expression::Static(_) | Expression::Self_(_) | Expression::Parent(_) => {
            special_class(ancestors, expr).is_some_and(|name| is_a(ctx, &name, &[class]))
        }
        e => match facts(ctx).full_type(node_span(e)) {
            Some(t) => is_type_of(ctx, t, class),
            None => unanalysed_param_hint(ancestors, e).is_some_and(|h| is_hint_type(ctx, ancestors, h, class)),
        },
    }
}

/// The type hint of the parameter a variable stands for in a function Mago did not analyse (a second declaration of
/// a function's name, which PHPStan analyses as it is): the variable is a parameter of the enclosing function and the
/// body never assigns it.
fn unanalysed_param_hint<'a>(ancestors: &[Node<'a, 'a>], expr: &Expression<'_>) -> Option<&'a Hint<'a>> {
    let Expression::Variable(Variable::Direct(v)) = expr else {
        return None;
    };
    let function = ancestors.iter().rev().find_map(|a| match a {
        Node::Function(f) => Some(Some(*f)),
        Node::Closure(_) | Node::ArrowFunction(_) | Node::Method(_) => Some(None),
        _ => None,
    })??;
    let param = (function.parameter_list.parameters.iter()).find(|p| p.variable.name == v.name)?;
    let mut assigned = false;
    let mut stack = vec![Node::Block(&function.body)];
    while let Some(node) = stack.pop() {
        if let Node::Assignment(a) = node
            && matches!(a.lhs, Expression::Variable(Variable::Direct(l)) if l.name == v.name)
        {
            assigned = true;
        }
        node.visit_children(|c| stack.push(c));
    }
    (!assigned).then_some(param.hint.as_ref()?)
}

/// `isObjectType` on a resolved type: `mixed` is not; a single object is when its class is `class` or a subclass,
/// or uses the trait `class`; otherwise, without `null` and `false`, every part must be an object of a subclass
/// (`ObjectType::isSuperTypeOf()->yes()`).
pub(crate) fn is_type_of(ctx: &Context<'_>, t: &TUnion, class: &str) -> bool {
    if t.is_mixed() {
        return false;
    }
    if let [atomic] = &t.types[..]
        && let Some(name) = object_class(atomic)
    {
        return is_a(ctx, &name, &[class]);
    }
    let mut kept = t.types.iter().filter(|a| match a {
        TAtomic::Null => false,
        TAtomic::Scalar(TScalar::Bool(b)) => !b.is_false(),
        _ => true,
    });
    let mut any = false;
    kept.all(|a| {
        any = true;
        object_class(a).is_some_and(|name| is_subclass(ctx, &name, class))
    }) && any
}

/// `NodeTypeResolver::isObjectType` on a type declaration (`ParamTypeResolver`, a return type). A class name matches
/// when it is the class or a subclass (`self` is the enclosing class, `parent` its parents, all of which must match;
/// `static` is `mixed`). A nullable or union type matches when, without `null` (nothing left but `null` does not
/// match) and then without `false`, every part is such a class name: PHPStan's `isSuperTypeOf` of what is left, where
/// nothing left (`never`) is a match. An intersection type is `mixed`.
pub(crate) fn is_hint_type(ctx: &Context<'_>, ancestors: &[Node<'_, '_>], hint: &Hint<'_>, class: &str) -> bool {
    let named = |h: &Hint<'_>| match h {
        Hint::Identifier(id) => is_a(ctx, &resolve_class_name(ancestors, id), &[class]),
        Hint::Self_(_) => class_scope(ancestors)
            .and_then(|s| s.name)
            .is_some_and(|own| is_a(ctx, &own, &[class])),
        Hint::Parent(_) => {
            // `NameTypeResolver::resolveParent`: every parent class, none being `mixed`
            let facts = facts(ctx);
            let parents: Vec<String> = class_scope(ancestors)
                .and_then(|s| s.name.zip(s.parent_class))
                .map(|(own, parent)| {
                    let all = facts.codebase.get_class_like(own.as_bytes());
                    all.map_or(vec![parent], |c| {
                        c.all_parent_classes.iter().map(|p| p.to_string()).collect()
                    })
                })
                .unwrap_or_default();
            !parents.is_empty() && parents.iter().all(|p| is_a(ctx, p, &[class]))
        }
        _ => false,
    };
    let mut parts = Vec::new();
    match hint {
        Hint::Identifier(_) | Hint::Self_(_) | Hint::Parent(_) => return named(hint),
        // `?T` is `T|null` when `T` is not `mixed`
        Hint::Nullable(n) => return named(n.hint) || matches!(n.hint, Hint::False(_)),
        Hint::Union(_) | Hint::False(_) => flatten(hint, &mut parts),
        _ => return false,
    }
    let without_null: Vec<&Hint<'_>> = parts
        .into_iter()
        .filter(|h| !matches!(h, Hint::Null(_)))
        .collect();
    !without_null.is_empty()
        && without_null
            .into_iter()
            .filter(|h| !matches!(h, Hint::False(_)))
            .all(named)
}

fn flatten<'a>(hint: &'a Hint<'a>, parts: &mut Vec<&'a Hint<'a>>) {
    match hint {
        Hint::Union(u) => {
            flatten(u.left, parts);
            flatten(u.right, parts);
        }
        h => parts.push(h),
    }
}

/// The class of an object type (a named object, `$this`, `static`, or an enum case). Mago types a value of a class it
/// does not know as a reference to the name, where PHPStan has an object of that class, which matches only its own
/// name.
pub(crate) fn object_class(atomic: &TAtomic) -> Option<String> {
    match atomic {
        TAtomic::Object(TObject::Named(n)) => Some(n.name.to_string()),
        TAtomic::Object(TObject::Enum(e)) => Some(e.name.to_string()),
        TAtomic::Reference(TReference::Symbol { name, .. }) => Some(name.to_string()),
        _ => None,
    }
}

/// `ObjectType::isSuperTypeOf` between classes: the same class, or a class the codebase knows that extends or
/// implements it. Traits do not count.
fn is_subclass(ctx: &Context<'_>, name: &str, class: &str) -> bool {
    if name.eq_ignore_ascii_case(class) {
        return true;
    }
    let facts = facts(ctx);
    (facts.codebase.get_class_like(name.as_bytes())).is_some_and(|c| {
        (c.all_parent_classes.iter())
            .chain(c.all_parent_interfaces.iter())
            .any(|p| p.as_bytes().eq_ignore_ascii_case(class.as_bytes()))
    })
}

/// The class a name written as a class (`Foo`, `\Foo`, `self`, `static`, `parent`) stands for.
pub(crate) fn name_class(
    ancestors: &[Node<'_, '_>],
    id: &mago_syntax::cst::Identifier<'_>,
) -> Option<String> {
    let written = id.value();
    let scope = || class_scope(ancestors);
    if written.eq_ignore_ascii_case(b"self") || written.eq_ignore_ascii_case(b"static") {
        return scope()?.name;
    }
    if written.eq_ignore_ascii_case(b"parent") {
        return scope()?.parent_class;
    }
    Some(resolve_class_name(ancestors, id))
}

/// The class `self`, `static` or `parent` stands for.
pub(crate) fn special_class(ancestors: &[Node<'_, '_>], expr: &Expression<'_>) -> Option<String> {
    let scope = class_scope(ancestors)?;
    match expr {
        Expression::Parent(_) => scope.parent_class,
        _ => scope.name,
    }
}

/// The class a class-string type names, for `$type->getClassStringObjectType()->getClassName()` when the type is
/// one class string: a literal class string (`Foo::class`), `class-string<Foo>`, or a literal string that names a
/// class-like the codebase knows (`ConstantStringType::isClassString()`).
pub(crate) fn class_string_class(ctx: &Context<'_>, t: &TUnion) -> Option<String> {
    let [atomic] = &t.types[..] else {
        return None;
    };
    let TAtomic::Scalar(scalar) = atomic else {
        return None;
    };
    match scalar {
        TScalar::ClassLikeString(c) => match (c.literal_value(), c.constraint()) {
            (Some(value), _) => Some(value.to_string()),
            // `static` and `$this` name the class as declared (Mago keeps their name in lowercase)
            (None, Some(TAtomic::Object(TObject::Named(n)))) if n.is_static || n.is_this => {
                let facts = facts(ctx);
                let class = facts.codebase.get_class_like(n.name.as_bytes())?;
                Some(class.original_name.to_string())
            }
            (None, constraint) => object_class(constraint?),
        },
        s => {
            let value = std::str::from_utf8(s.get_known_literal_string_value()?).ok()?;
            let facts = facts(ctx);
            facts
                .codebase
                .get_class_like(value.as_bytes())
                .map(|_| value.to_string())
        }
    }
}

/// PHPStan's `TrinaryLogic`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Tri {
    No,
    Maybe,
    Yes,
}

/// `ObjectType($sup)->isSuperTypeOf(ObjectType($sub))` between class names: the same name, or `sub` extending or
/// implementing `sup`, is yes; a class the codebase does not know, `sup` extending `sub`, or an interface on either
/// side with a class that is not final on the other, is maybe; a trait or anything else is no.
pub(crate) fn class_super_of(ctx: &Context<'_>, sup: &str, sub: &str) -> Tri {
    exact_class_super_of(ctx, sup, sub, false)
}

/// [`class_super_of`] where `sup` may be an exact class (PHPStan 2's type of `new Foo()`, whose class reflection is
/// `asFinal()`): final, so no class is its subclass (`isSubclassOfClass` of a final class is false).
pub(crate) fn exact_class_super_of(ctx: &Context<'_>, sup: &str, sub: &str, exact: bool) -> Tri {
    if sup.eq_ignore_ascii_case(sub) {
        return Tri::Yes;
    }
    let facts = facts(ctx);
    let (Some(a), Some(b)) = (
        facts.codebase.get_class_like(sup.as_bytes()),
        facts.codebase.get_class_like(sub.as_bytes()),
    ) else {
        return Tri::Maybe;
    };
    if a.kind.is_trait() || b.kind.is_trait() {
        return Tri::No;
    }
    let is_final = |c: &mago_codex::metadata::class_like::ClassLikeMetadata| {
        c.flags.is_final() || c.kind.is_enum()
    };
    if !exact && is_subclass(ctx, sub, sup) {
        return Tri::Yes;
    }
    if is_subclass(ctx, sup, sub)
        || (a.kind.is_interface() && !is_final(b))
        || (b.kind.is_interface() && !exact && !is_final(a))
    {
        return Tri::Maybe;
    }
    Tri::No
}

/// `ObjectType($class)->isSuperTypeOf($type)` for a type Mago gives: one part as [`object_super_of`]; several as
/// `UnionType::isSubTypeOf`, the answer they all give, else maybe.
pub(crate) fn object_super_of_union(ctx: &Context<'_>, class: &str, t: &TUnion) -> Tri {
    let mut answers = t.types.iter().map(|a| object_super_of(ctx, class, a));
    let first = answers.next().unwrap_or(Tri::Yes);
    if answers.all(|a| a == first) { first } else { Tri::Maybe }
}

/// `ObjectType($class)->isSuperTypeOf()` of one part of a type: an object of a class as [`class_super_of`] (a closure
/// is a `Closure`), `object`, `mixed`, `callable`, a template and a template Mago left unresolved (PHPStan's error
/// type, a `mixed`) are maybe, `never` is yes, and anything that is not an object is no.
pub(crate) fn object_super_of(ctx: &Context<'_>, class: &str, atomic: &TAtomic) -> Tri {
    if let Some(name) = object_class(atomic) {
        return class_super_of(ctx, class, &name);
    }
    match atomic {
        TAtomic::Callable(c) if c.get_signature().is_some_and(|s| s.is_closure) => {
            class_super_of(ctx, class, "Closure")
        }
        // `callable` is a compound type that may be an object
        TAtomic::Callable(_)
        | TAtomic::Object(_)
        | TAtomic::Mixed(_)
        | TAtomic::GenericParameter(_)
        | TAtomic::Placeholder => Tri::Maybe,
        TAtomic::Never => Tri::Yes,
        _ => Tri::No,
    }
}
