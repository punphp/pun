//! CallToFunctionStatementWithNoDiscardRule, including callable expressions and pipe calls.

use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::callable::TCallable;
use mago_codex::ttype::atomic::scalar::class_like_string::TClassLikeString;
use mago_codex::ttype::atomic::scalar::{TScalar, float::TFloat};
use mago_codex::ttype::union::TUnion;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::resolve::resolve_str;
use pun_phpstan::types::{ConstArray, Level, Param, Signature, Tri, Type, mixed, union};

use super::super::types::flow::unparen;
use super::super::{Checker, enums, members};
use super::no_discard::expression;

fn target<'e, 'arena>(
    s: &'e ExpressionStatement<'arena>,
) -> Option<(&'e Expression<'arena>, bool)> {
    let (e, void, piped) = expression(s);
    let name = match e {
        Expression::Call(Call::Function(f)) if !piped => f.function,
        Expression::PartialApplication(PartialApplication::Function(f)) if piped => f.function,
        _ => return None,
    };
    Some((unparen(name), void))
}

pub(super) fn typed(s: &ExpressionStatement, php_version: u32) -> Option<Span> {
    let (name, _) = target(s)?;
    (php_version >= 80500 && !matches!(name, Expression::Identifier(_))).then(|| name.span())
}

fn must_use(f: &FunctionLikeMetadata) -> bool {
    f.attributes
        .iter()
        .any(|a| a.name.as_bytes().eq_ignore_ascii_case(b"NoDiscard"))
}

fn method_must_use(c: &Checker, m: members::Method) -> bool {
    match m {
        members::Method::User { f, .. } => must_use(f),
        members::Method::Builtin { declaring, f } => c
            .codebase
            .get_method(declaring.name.as_bytes(), f.name.as_bytes())
            .is_some_and(must_use),
    }
}

/// `getCallableParametersAcceptors()` on constant strings and invokable objects.
fn string_must_use(c: &Checker, name: &[u8]) -> Option<bool> {
    let name = name.strip_prefix(b"\\").unwrap_or(name);
    if let Some((class, method)) = name
        .windows(2)
        .position(|w| w == b"::")
        .map(|i| (&name[..i], &name[i + 2..]))
    {
        let class = c.codebase.get_class_like(class)?;
        let method = members::method(c, class, method)?;
        return method.is_static().then(|| method_must_use(c, method));
    }
    c.function(name)?;
    Some(c.codebase.get_function(name).is_some_and(must_use))
}

fn literal_string(a: &TAtomic) -> Option<&[u8]> {
    match a {
        TAtomic::Scalar(TScalar::ClassLikeString(TClassLikeString::Literal { value })) => {
            Some(value.as_bytes())
        }
        _ => a.get_literal_string_value(),
    }
}

fn array_items<'t>(c: &Checker, a: &'t TArray) -> Vec<(Type, bool, &'t TUnion)> {
    match a {
        TArray::List(l) => l
            .known_elements
            .iter()
            .flat_map(|items| items.iter())
            .map(|(i, (optional, t))| (Type::ConstInt(*i as i64), *optional, t))
            .collect(),
        TArray::Keyed(k) => k
            .known_items
            .iter()
            .flat_map(|items| items.iter())
            .map(|(key, (optional, t))| {
                let key = enums::constant_value(c, &key.to_atomic()).unwrap_or(Type::Error);
                (key, *optional, t)
            })
            .collect(),
    }
}

/// The callable's value description, keeping literals and signatures structured instead of parsing Mago IDs.
fn value_type(c: &Checker, t: &TUnion) -> Type {
    union(
        t.types
            .iter()
            .map(|a| {
                if let Some(t) = enums::constant_value(c, a) {
                    return t;
                }
                match a {
                    TAtomic::Scalar(TScalar::Float(TFloat::Literal(f))) => Type::ConstFloat(f.0),
                    TAtomic::Callable(TCallable::Signature(sig)) => {
                        let signature = Box::new(Signature {
                            params: sig
                                .parameters
                                .iter()
                                .map(|p| Param {
                                    name: String::new(),
                                    ty: p
                                        .get_type_signature()
                                        .map_or_else(mixed, |t| value_type(c, t)),
                                    optional: p.has_default(),
                                    by_ref: p.is_by_reference(),
                                    variadic: p.is_variadic(),
                                })
                                .collect(),
                            ret: sig
                                .return_type
                                .as_deref()
                                .map_or_else(mixed, |t| value_type(c, t)),
                            templates: vec![],
                        });
                        if sig.is_closure {
                            Type::Closure(signature)
                        } else {
                            Type::Callable(Some(signature))
                        }
                    }
                    TAtomic::Array(a) if a.is_sealed() => {
                        let mut shape = ConstArray::empty();
                        for (key, optional, value) in array_items(c, a) {
                            if optional {
                                shape.optional.push(shape.keys.len());
                            }
                            shape.keys.push(key);
                            shape.values.push(value_type(c, value));
                        }
                        shape.is_list = Tri::from(
                            shape
                                .keys
                                .iter()
                                .enumerate()
                                .all(|(i, k)| *k == Type::ConstInt(i as i64)),
                        );
                        Type::ConstArray(Box::new(shape))
                    }
                    TAtomic::Array(a) if a.is_vanilla() => Type::array(mixed(), mixed()),
                    _ => resolve_str(&a.get_id().to_string(), None),
                }
            })
            .collect(),
    )
}

/// None means not definitely callable; Maybe preserves a callable PHPDoc's unknown NoDiscard contract.
fn callable_must_use(c: &Checker, a: &TAtomic) -> Option<Tri> {
    match a {
        // Mago resolves callable aliases to signatures before storing expression types.
        TAtomic::Callable(TCallable::Signature(sig)) => Some(
            sig.invocation_target()
                .and_then(|id| c.codebase.get_function_like(&id))
                .map_or(Tri::Maybe, |f| Tri::from(must_use(f))),
        ),
        TAtomic::Array(array) => {
            let items = array_items(c, array);
            if !array.is_sealed()
                || items.len() != 2
                || items.iter().any(|(_, optional, _)| *optional)
            {
                return None;
            }
            let receiver = items.iter().find(|(k, _, _)| *k == Type::ConstInt(0))?.2;
            let methods = items.iter().find(|(k, _, _)| *k == Type::ConstInt(1))?.2;
            let mut result = Tri::No;
            for r in receiver.types.iter() {
                let names = r.get_all_object_names();
                let static_name = literal_string(r);
                let names: Vec<_> = names
                    .iter()
                    .map(|n| n.as_bytes())
                    .chain(static_name)
                    .collect();
                if names.is_empty() {
                    return None;
                }
                for name in names {
                    let class = c.codebase.get_class_like(name)?;
                    for m in methods.types.iter() {
                        let method = members::method(c, class, literal_string(m)?)?;
                        if static_name.is_some() && !method.is_static() {
                            return None;
                        }
                        result = result.or(Tri::from(method_must_use(c, method)));
                    }
                }
            }
            Some(result)
        }
        _ if literal_string(a).is_some() => string_must_use(c, literal_string(a)?).map(Tri::from),
        _ => {
            let names = a.get_all_object_names();
            let mut values = names
                .iter()
                .filter_map(|n| c.codebase.get_class_like(n.as_bytes()))
                .filter_map(|class| {
                    members::method(c, class, b"__invoke").map(|m| Tri::from(method_must_use(c, m)))
                });
            let first = values.next()?;
            Some(values.fold(first, Tri::or))
        }
    }
}

pub(super) fn statement(c: &mut Checker, s: &ExpressionStatement) {
    if c.php_version < 80500 {
        return;
    }
    let Some((name, void)) = target(s) else {
        return;
    };
    let (kind, subject, required) = if let Expression::Identifier(id) = name {
        let fallback = match id {
            Identifier::Local(l) if !c.names.is_imported(&l.span) => Some(l.value),
            _ => None,
        };
        let Some((name, _)) = std::iter::once(c.names.get(&id.span()))
            .chain(fallback)
            .find_map(|name| c.function(name))
        else {
            return;
        };
        let required = c
            .codebase
            .get_function(name.as_bytes())
            .is_some_and(must_use);
        ("function", format!("{name}()"), Tri::from(required))
    } else {
        let Some(t) = c.type_of(name) else { return };
        let mut required = Tri::No;
        // TypeCombinator::union() keeps the last of equal types, such as two closures of one signature
        let values: Vec<_> = t
            .types
            .iter()
            .map(|a| value_type(c, &TUnion::from_atomic(a.clone())))
            .collect();
        for (i, atom) in t.types.iter().enumerate() {
            let Some(value) = callable_must_use(c, atom) else {
                return;
            };
            if !values[i + 1..].contains(&values[i]) {
                required = required.or(value);
            }
        }
        let description =
            super::super::declared::with_classes(c, || value_type(c, t).describe(Level::Value));
        ("callable", description, required)
    };
    if void && required.no() {
        c.report(
            s,
            &format!("{kind}.inVoidCast"),
            format!(
                "Call to {kind} {subject} in (void) cast but {kind} allows discarding return value."
            ),
            None,
        );
    } else if !void && required.yes() {
        c.report_non_ignorable(
            s,
            &format!("{kind}.resultDiscarded"),
            format!("Call to {kind} {subject} on a separate line discards return value."),
            None,
        );
    }
}
