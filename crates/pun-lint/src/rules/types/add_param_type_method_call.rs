//! `lint/types/add-param-type-method-call`, ported from Rector 2.6.1's `AddMethodCallBasedStrictParamTypeRector`
//! (rules/TypeDeclaration/Rector/ClassMethod) with `PrivateMethodFlagger`, `LocalMethodCallFinder`,
//! `CallTypesResolver::resolveStrictTypesFromCalls` and `ClassMethodParamTypeCompleter`: an untyped parameter of a
//! method only its own class can call (private, or protected or final in a final class that extends and implements
//! nothing) gets the union of the native types its class's calls pass at its position.
//!
//! Option: `treat-classes-as-final` (default false), Rector's `treatClassesAsFinal()` feature flag: a class that is
//! not abstract counts as final.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_syntax::cst::{
    Argument, ClassLikeMember, ClassLikeMemberSelector, Expression, Literal, Method, Modifier, Node, NodeKind,
};
use pun_problem::Edit;

use super::facts::{Facts, facts};
use super::names::resolve_class_name;
use super::param::{add_hint, param_declaration_unsorted, report};
use crate::rules::class_method::{qualified, type_of};
use crate::rules::mago_helpers::{boolean, configure};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// php-parser's `ClassMethod::$magicNames`.
const MAGIC_METHODS: [&str; 17] = [
    "__construct", "__destruct", "__call", "__callstatic", "__get", "__set", "__isset", "__unset", "__sleep",
    "__wakeup", "__tostring", "__set_state", "__clone", "__invoke", "__debuginfo", "__serialize", "__unserialize",
];

#[derive(Default)]
pub struct AddParamTypeMethodCall {
    pub treat_classes_as_final: bool,
}

impl Rule for AddParamTypeMethodCall {
    fn id(&self) -> &'static str {
        "lint/types/add-param-type-method-call"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class])
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        configure(options, |key, value| {
            match key {
                "treat-classes-as-final" => self.treat_classes_as_final = boolean(value)?,
                _ => return None,
            }
            Some(())
        })
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
        let has = |f: fn(&Modifier<'_>) -> bool| class.modifiers.iter().any(f);
        let is_final = has(|m| matches!(m, Modifier::Final(_)))
            || (self.treat_classes_as_final && !has(|m| matches!(m, Modifier::Abstract(_))));
        let open = class.extends.is_some() || class.implements.is_some();
        let name = qualified(ancestors, class.name.value);
        let facts = facts(ctx);
        let methods: Vec<&Method<'_>> = class
            .members
            .iter()
            .filter_map(|m| match m {
                ClassLikeMember::Method(m) => Some(m),
                _ => None,
            })
            .collect();
        let mut scoped = ancestors.to_vec();
        scoped.push(node);
        let mut edits: Vec<Edit> = Vec::new();
        for method in &methods {
            if method.parameter_list.parameters.is_empty() {
                continue;
            }
            let modifier = |f: fn(&Modifier<'_>) -> bool| method.modifiers.iter().any(f);
            let private = modifier(|m| matches!(m, Modifier::Private(_)));
            let protected = modifier(|m| matches!(m, Modifier::Protected(_)));
            // PrivateMethodFlagger::isClassMethodPrivate
            let only_own = private
                || (modifier(|m| matches!(m, Modifier::Final(_))) && !open)
                || (is_final && !open && protected);
            if !only_own || (!private && !protected) {
                continue;
            }
            let Some(types) = call_types(&facts, &scoped, &name, &methods, method.name.value) else {
                continue;
            };
            if is_vendor_locked(&facts, &name, method, private) {
                continue;
            }
            let params: Vec<_> = method.parameter_list.parameters.iter().collect();
            for (position, types) in types.into_iter().enumerate() {
                let (Some(types), Some(param)) = (types, params.get(position)) else {
                    continue;
                };
                if param.ellipsis.is_some() || param.hint.is_some() {
                    continue;
                }
                let Some(declaration) = param_declaration_unsorted(&types, &facts.codebase, ctx.php_version)
                else {
                    continue;
                };
                if let Some(default) = &param.default_value
                    && !accepts(&facts, &types, default.value)
                {
                    continue;
                }
                edits.push(add_hint(param, &declaration));
            }
        }
        if edits.is_empty() {
            return;
        }
        out.push(report(
            node_span(&node),
            "Add the native types of the arguments the class's own calls pass to its private methods' parameters.",
            edits,
        ));
    }
}

/// `ClassMethodParamVendorLockResolver::isVendorLocked`: a magic method, or a method that is not private and an
/// interface of the class declares.
fn is_vendor_locked(facts: &Facts, class: &str, method: &Method<'_>, private: bool) -> bool {
    let name = String::from_utf8_lossy(method.name.value).to_ascii_lowercase();
    if MAGIC_METHODS.contains(&name.as_str()) {
        return true;
    }
    if private {
        return false;
    }
    facts.codebase.get_class_like(class.as_bytes()).is_some_and(|c| {
        c.all_parent_interfaces
            .iter()
            .any(|i| facts.codebase.get_method(i.as_bytes(), method.name.value).is_some())
    })
}

/// `LocalMethodCallFinder::match` and `CallTypesResolver::resolveStrictTypesFromCalls`: for each argument position of
/// the calls of `method` on the class (`$this->method()`, `self::method()`, on any value of the class) in its
/// methods, the union of the arguments' native types in the order met (literal values made general, arrays one
/// `array`); `None` at a position where one is `mixed`. `None` for all when a call unpacks an argument.
fn call_types(
    facts: &Facts,
    scoped: &[Node<'_, '_>],
    class: &str,
    methods: &[&Method<'_>],
    method: &[u8],
) -> Option<Vec<Option<Vec<TAtomic>>>> {
    let mut positions: Vec<Option<Vec<TAtomic>>> = Vec::new();
    let mut unpacked = false;
    for m in methods {
        walk(Node::Method(m), &mut |node| {
            let (selector, args, on_class) = match node {
                Node::MethodCall(call) => (
                    &call.method,
                    &call.argument_list,
                    type_of(facts, call.object).is_some_and(|t| is_class(t.types.as_ref(), class)),
                ),
                Node::StaticMethodCall(call) => (
                    &call.method,
                    &call.argument_list,
                    static_class(scoped, call.class).is_some_and(|c| c == class),
                ),
                _ => return,
            };
            let ClassLikeMemberSelector::Identifier(id) = selector else {
                return;
            };
            if !on_class || !id.value.eq_ignore_ascii_case(method) {
                return;
            }
            for (position, arg) in args.arguments.iter().enumerate() {
                let value = match arg {
                    Argument::Positional(p) if p.ellipsis.is_some() => {
                        unpacked = true;
                        return;
                    }
                    Argument::Positional(p) => p.value,
                    Argument::Named(n) => n.value,
                };
                if positions.len() <= position {
                    positions.resize(position + 1, Some(Vec::new()));
                }
                let native = facts
                    .native_type(node_span(unparenthesize(value)))
                    .filter(|_| !is_optional_offset(facts, value));
                let Some(types) = &mut positions[position] else {
                    continue;
                };
                match native.and_then(|t| strict_types(facts, t.types.as_ref())) {
                    Some(native) => {
                        for t in native {
                            push_unique(types, t);
                        }
                    }
                    None => positions[position] = None,
                }
            }
        });
    }
    (!unpacked).then_some(positions)
}

/// `normalizeType()` and `TypeFactory::uniquateTypes` of a native type's members: `$this` is its class, a class the
/// codebase does not know is `mixed` (`None`), a literal value its general type.
fn strict_types(facts: &Facts, types: &[TAtomic]) -> Option<Vec<TAtomic>> {
    let mut out = Vec::new();
    for t in types {
        let t = match t {
            TAtomic::Mixed(_) => return None,
            TAtomic::Object(TObject::Named(n)) => {
                if facts.codebase.get_class_like(n.name.as_bytes()).is_none() {
                    return None;
                }
                let mut n = n.clone();
                n.is_this = false;
                n.is_static = false;
                TAtomic::Object(TObject::Named(n))
            }
            TAtomic::Scalar(s) if s.is_int() => TAtomic::Scalar(TScalar::int()),
            TAtomic::Scalar(s) if s.is_float() => TAtomic::Scalar(TScalar::float()),
            TAtomic::Scalar(s) if s.is_any_string() => TAtomic::Scalar(TScalar::string()),
            TAtomic::Scalar(s) if s.is_bool() => TAtomic::Scalar(TScalar::bool()),
            other => other.clone(),
        };
        out.push(t);
    }
    (!out.is_empty()).then_some(out)
}

/// Add `atomic` unless an equal type is in already: an object of the same class, or an array.
fn push_unique(types: &mut Vec<TAtomic>, atomic: TAtomic) {
    let class = |a: &TAtomic| match a {
        TAtomic::Object(TObject::Named(n)) => Some(n.name.as_bytes().to_ascii_lowercase()),
        _ => None,
    };
    let seen = types.iter().any(|t| {
        t == &atomic
            || matches!(t, TAtomic::Array(_)) && matches!(atomic, TAtomic::Array(_))
            || class(t).is_some() && class(t) == class(&atomic)
    });
    if !seen {
        types.push(atomic);
    }
}

/// `ClassNameFromObjectTypeResolver::resolve($type) === $className`: the one object class of the type is the class.
fn is_class(types: &[TAtomic], class: &str) -> bool {
    let [TAtomic::Object(TObject::Named(n))] = types else {
        return false;
    };
    n.name.as_bytes() == class.as_bytes()
}

/// The class a static call's class part names: `self`, `static` or a name (`parent` is another class).
fn static_class(scoped: &[Node<'_, '_>], class: &Expression<'_>) -> Option<String> {
    let at = scoped.len() - 1;
    match unparenthesize(class) {
        Expression::Self_(_) | Expression::Static(_) => match scoped[at] {
            Node::Class(c) => Some(qualified(&scoped[..at], c.name.value)),
            _ => None,
        },
        Expression::Identifier(id) => Some(resolve_class_name(&scoped[..at], id)),
        _ => None,
    }
}

/// `ClassMethodParamTypeCompleter::isAcceptedByDefault`: the new type accepts the parameter's default value.
fn accepts(facts: &Facts, types: &[TAtomic], default: &Expression<'_>) -> bool {
    let Some(default) = type_of(facts, default) else {
        return false;
    };
    default.types.iter().all(|d| {
        types.iter().any(|t| match (t, d) {
            (TAtomic::Null, TAtomic::Null) => true,
            (TAtomic::Scalar(t), TAtomic::Scalar(d)) => {
                (t.is_int() && d.is_int())
                    || (t.is_float() && (d.is_float() || d.is_int()))
                    || (t.is_any_string() && d.is_any_string())
                    || (t.is_bool() && d.is_bool())
            }
            (TAtomic::Array(_), TAtomic::Array(_)) => true,
            _ => false,
        })
    })
}

/// Call `f` on every node under `node`, closures and nested classes included (`BetterNodeFinder::find`).
fn walk<'ast, 'arena>(node: Node<'ast, 'arena>, f: &mut dyn FnMut(Node<'ast, 'arena>)) {
    node.visit_children(|child| {
        f(child);
        walk(child, f);
    });
}

/// Whether `expr` reads a key an array shape may lack (`$shape['dirname']` of `pathinfo()`): PHPStan's native type of
/// it is none Rector maps (checked with Rector 2.6.1), where Mago adds `null`, unless the shape has the key.
fn is_optional_offset(facts: &Facts, expr: &Expression<'_>) -> bool {
    let Expression::ArrayAccess(access) = unparenthesize(expr) else {
        return false;
    };
    let key = match unparenthesize(access.index) {
        Expression::Literal(Literal::String(s)) => match s.value {
            Some(v) => match ArrayKey::from(String::from_utf8_lossy(v)) {
                ArrayKey::String(word) => ArrayKey::from_string(word),
                other => other,
            },
            None => return false,
        },
        Expression::Literal(Literal::Integer(i)) => match i.value.and_then(|v| i64::try_from(v).ok()) {
            Some(v) => ArrayKey::Integer(v),
            None => return false,
        },
        _ => return false,
    };
    let nullable = facts
        .native_type(node_span(unparenthesize(expr)))
        .is_some_and(|t| t.types.iter().any(|a| matches!(a, TAtomic::Null)));
    let has_key = facts.native_type(node_span(unparenthesize(access.array))).is_some_and(|array| {
        array.types.iter().all(|a| match a {
            TAtomic::Array(TArray::Keyed(k)) => k
                .known_items
                .as_ref()
                .is_some_and(|items| items.get(&key).is_some_and(|(optional, _)| !*optional)),
            _ => false,
        })
    });
    nullable && !has_key
}
