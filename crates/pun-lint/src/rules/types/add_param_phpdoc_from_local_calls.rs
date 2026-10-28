//! `lint/types/add-param-phpdoc-from-local-calls`, ported from Rector 2.6.1's
//! `ClassMethodArrayDocblockParamFromLocalCallsRector` (rules/TypeDeclarationDocblocks/Rector/Class_): a method
//! parameter declared `array` gets `@param` from the types of the arguments the class's own calls of the method pass
//! (`LocalMethodCallFinder`, `CallTypesResolver::resolveTypesFromCalls()`).

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_syntax::cst::{
    Argument, ArgumentList, Call, ClassLikeMemberSelector, Expression, Node, NodeKind, PartialApplication,
};
use pun_phpstan::types::{Type, mixed};

use super::docblock::{class_name, docs_report, is_array_hint, is_this, methods};
use super::facts::facts;
use super::names::resolve_class_name;
use crate::rules::phpdoc_writer::type_mapper::{
    const_array_item_type, const_array_key_type, is_mixed, is_mixed_array, is_useful_array_tag, uniquate_types,
};
use crate::rules::phpdoc_writer::{DocTarget, decorate_param, expr_type};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct AddParamPhpdocFromLocalCalls;

impl Rule for AddParamPhpdocFromLocalCalls {
    fn id(&self) -> &'static str {
        "lint/types/add-param-phpdoc-from-local-calls"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Class(class) = node else {
            return;
        };
        let mut scope = ancestors.to_vec();
        scope.push(node);
        let Some(class_name) = class_name(&scope) else {
            return;
        };
        let mut changes = Vec::new();
        for method in methods(&class.members) {
            let params = &method.parameter_list.parameters;
            if params.is_empty() {
                continue;
            }
            let target = DocTarget::of(ctx.source, node_span(method).start);
            let Some(mut info) = target.info(ctx.source) else {
                continue;
            };
            let name = String::from_utf8_lossy(method.name.value);
            let calls = local_method_calls(ctx, &scope, class, &class_name, &name);
            let types = resolve_types_from_calls(ctx, &scope, &calls);
            let is_public = !method.modifiers.iter().any(|m| m.is_private() || m.is_protected());
            let mut changed = false;
            for (position, param) in params.iter().enumerate() {
                if !param.hint.as_ref().is_some_and(is_array_hint) {
                    continue;
                }
                let param_name = String::from_utf8_lossy(&param.variable.name[1..]).into_owned();
                let tag = info.param_tag(&param_name);
                // already known, skip
                if is_useful_array_tag(tag.map(|t| &t.ty)) {
                    continue;
                }
                // on public method, skip if there is mixed[], as caller can be anything
                if is_public && tag.is_some_and(|t| is_mixed_array(&t.ty)) {
                    continue;
                }
                let resolved = types
                    .iter()
                    .find(|(key, _)| *key == Key::Position(position))
                    .or_else(|| types.iter().find(|(key, _)| *key == Key::Name(param_name.clone())));
                let Some((_, resolved)) = resolved else {
                    continue;
                };
                // in case of array type declaration, null cannot be passed or is already casted
                let resolved = resolved.clone().remove_null();
                changed |= decorate_param(&mut info, &resolved, &param_name, param.ellipsis.is_some());
            }
            if changed {
                changes.push((target, info));
            }
        }
        if !changes.is_empty() {
            out.push(docs_report(
                node_span(&class.name),
                "Add the `@param` array types from the class's own calls.",
                ctx.source,
                &changes,
            ));
        }
    }
}

/// A call `LocalMethodCallFinder` finds: its arguments, or `None` for a first-class callable (`VariadicPlaceholder`).
type LocalCall<'ast, 'arena> = Option<&'ast ArgumentList<'arena>>;

/// `LocalMethodCallFinder::match()`: the method and static calls of `name` in the class's methods whose caller's
/// one class is the class.
fn local_method_calls<'ast, 'arena>(
    ctx: &Context<'_>,
    scope: &[Node<'ast, 'arena>],
    class: &'ast mago_syntax::cst::Class<'arena>,
    class_name: &str,
    name: &str,
) -> Vec<LocalCall<'ast, 'arena>> {
    let mut calls = Vec::new();
    for method in methods(&class.members) {
        find(Node::Method(method), &mut |n| {
            let (caller, selector, args): (Caller<'_, '_>, _, LocalCall<'ast, 'arena>) = match n {
                Node::Call(Call::Method(c)) => (Caller::Object(c.object), &c.method, Some(&c.argument_list)),
                Node::Call(Call::StaticMethod(c)) => (Caller::Class(c.class), &c.method, Some(&c.argument_list)),
                Node::PartialApplication(PartialApplication::Method(c)) => (Caller::Object(c.object), &c.method, None),
                Node::PartialApplication(PartialApplication::StaticMethod(c)) => {
                    (Caller::Class(c.class), &c.method, None)
                }
                _ => return,
            };
            let ClassLikeMemberSelector::Identifier(id) = selector else {
                return;
            };
            if !id.value.eq_ignore_ascii_case(name.as_bytes()) {
                return;
            }
            if caller_class(ctx, scope, class_name, caller).as_deref() == Some(class_name) {
                calls.push(args);
            }
        });
    }
    calls
}

/// `BetterNodeFinder::find()` under `node`: every node, in source order.
fn find<'ast, 'arena>(node: Node<'ast, 'arena>, found: &mut dyn FnMut(Node<'ast, 'arena>)) {
    node.visit_children(|child| {
        found(child);
        find(child, found);
    });
}

enum Caller<'ast, 'arena> {
    /// `$var` of a method call.
    Object(&'ast Expression<'arena>),
    /// `$class` of a static call.
    Class(&'ast Expression<'arena>),
}

/// `ClassNameFromObjectTypeResolver::resolve()` of the caller's type: its one class name.
fn caller_class(ctx: &Context<'_>, scope: &[Node<'_, '_>], class_name: &str, caller: Caller<'_, '_>) -> Option<String> {
    let expr = match caller {
        Caller::Object(expr) if is_this(expr) => return Some(class_name.to_string()),
        Caller::Class(Expression::Self_(_) | Expression::Static(_)) => return Some(class_name.to_string()),
        // `NameTypeResolver::resolveParent()`: the parent classes
        Caller::Class(Expression::Parent(_)) => return None,
        Caller::Class(Expression::Identifier(id)) => return Some(resolve_class_name(scope, id)),
        Caller::Object(expr) | Caller::Class(expr) => expr,
    };
    let facts = facts(ctx);
    let t = facts.full_type(node_span(expr))?;
    let mut names: Vec<String> = Vec::new();
    for atomic in t.types.iter() {
        let name = match atomic {
            TAtomic::Object(TObject::Named(n)) => n.name.to_string(),
            TAtomic::Object(TObject::Enum(e)) => e.name.to_string(),
            _ => continue,
        };
        if !names.iter().any(|n| n.eq_ignore_ascii_case(&name)) {
            names.push(name);
        }
    }
    match names.as_slice() {
        [one] => Some(one.clone()),
        _ => None,
    }
}

/// An argument's position, or its name when it is named.
#[derive(PartialEq)]
enum Key {
    Position(usize),
    Name(String),
}

/// `CallTypesResolver::resolveTypesFromCalls()`: each position's (or name's) type, the types of all calls united.
fn resolve_types_from_calls(ctx: &Context<'_>, scope: &[Node<'_, '_>], calls: &[LocalCall<'_, '_>]) -> Vec<(Key, Type)> {
    let mut by_position: Vec<(Key, Vec<Type>)> = Vec::new();
    for call in calls {
        // a first-class callable can be called with anything
        let Some(args) = call else {
            return vec![];
        };
        for (position, arg) in args.arguments.iter().enumerate() {
            let (key, value) = match arg {
                Argument::Positional(p) if p.ellipsis.is_some() => return vec![],
                Argument::Positional(p) => (Key::Position(position), p.value),
                Argument::Named(n) => (Key::Name(String::from_utf8_lossy(n.name.value).into_owned()), n.value),
            };
            // skip empty array, as it doesn't add any value
            if let Expression::Array(a) = value
                && a.elements.is_empty()
            {
                continue;
            }
            if let Expression::LegacyArray(a) = value
                && a.elements.is_empty()
            {
                continue;
            }
            let t = normalize_type(ctx, expr_type(ctx, scope, value).unwrap_or_else(mixed));
            match by_position.iter_mut().find(|(k, _)| *k == key) {
                Some((_, types)) => types.push(t),
                None => by_position.push((key, vec![t])),
            }
        }
    }
    by_position
        .into_iter()
        .map(|(key, types)| (key, union_to_single_type(types)))
        .collect()
}

/// `normalizeType()`: an object of a class PHPStan does not know is `mixed`.
fn normalize_type(ctx: &Context<'_>, t: Type) -> Type {
    match t {
        Type::This(name) => Type::Object(name, vec![]),
        Type::Object(name, _) if facts(ctx).codebase.get_class_like(name.as_bytes()).is_none() => mixed(),
        t => t,
    }
}

/// `unionToSingleType()` with `$removeMixedArray`. (`narrowParentObjectTreeToSingleObjectChildType()` gives its type
/// back: its loop tests the first type against itself too.)
fn union_to_single_type(types: Vec<Type>) -> Type {
    let types: Vec<Type> = types.into_iter().filter(|t| !is_array_mixed_mixed_type(t)).collect();
    let united = create_mixed_passed_or_union_type(&types);
    if let Type::Union(members) = &united
        && members
            .iter()
            .any(|t| matches!(t, Type::Array(..)) && is_array_mixed_mixed_type(t))
    {
        return mixed();
    }
    united
}

/// `isArrayMixedMixedType()`: an array of `mixed` or nothing, by `mixed` keys or none.
fn is_array_mixed_mixed_type(t: &Type) -> bool {
    let (key, item) = match t {
        Type::Array(key, item) => ((**key).clone(), (**item).clone()),
        Type::ConstArray(a) => (const_array_key_type(a), const_array_item_type(a)),
        _ => return false,
    };
    let mixed_or_never = |t: &Type| is_mixed(t) || matches!(t, Type::Never(_));
    mixed_or_never(&item) && mixed_or_never(&key)
}

/// `TypeFactory::createMixedPassedOrUnionType()`.
fn create_mixed_passed_or_union_type(types: &[Type]) -> Type {
    let mut unwrapped = Vec::new();
    for t in types {
        for flat in flatten_types(t) {
            match flat {
                Type::ConstArray(a) => unwrapped.extend(unwrap_constant_array_types(&a)),
                t => unwrapped.push(t),
            }
        }
    }
    let unique = uniquate_types(&unwrapped);
    // `createUnionOrSingleType()`
    match unique.len() {
        0 => mixed(),
        1 => unique.into_iter().next().expect("one type"),
        _ if unique.iter().any(is_mixed) => mixed(),
        _ => Type::Union(unique),
    }
}

/// `TypeUtils::flattenTypes()`. A constant array with optional keys (`getAllArrays()`) stands for `mixed`.
fn flatten_types(t: &Type) -> Vec<Type> {
    let members = match t {
        Type::Union(types) | Type::Benevolent(types) => types.clone(),
        t => vec![t.clone()],
    };
    members
        .into_iter()
        .map(|t| match t {
            Type::ConstArray(a) if !a.optional.is_empty() => mixed(),
            t => t,
        })
        .collect()
}

/// `unwrapConstantArrayTypes()`: an `array<key, item>` per item type, by the key type of its position.
fn unwrap_constant_array_types(a: &pun_phpstan::types::ConstArray) -> Vec<Type> {
    let keys = flatten_types(&const_array_key_type(a));
    let items = flatten_types(&const_array_item_type(a));
    let mut out = Vec::new();
    for (position, item) in items.into_iter().enumerate() {
        let key = keys.get(position).cloned().unwrap_or_else(mixed);
        match item {
            Type::ConstArray(inner) => {
                for inner in unwrap_constant_array_types(&inner) {
                    // preserve outer array -> inner array structure: array<outerKey, innerArray>
                    out.push(Type::Array(Box::new(key.clone()), Box::new(inner)));
                }
            }
            item => out.push(Type::Array(Box::new(key), Box::new(item))),
        }
    }
    out
}
