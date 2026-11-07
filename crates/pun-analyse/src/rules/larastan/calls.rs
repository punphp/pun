//! NoUselessWithFunctionCallsRule, NoUselessValueFunctionCallsRule, NoEnvCallsOutsideOfConfigRule, NoModelMakeRule,
//! NoAuthFacadeInRequestScopeRule and NoAuthHelperInRequestScopeRule.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::class_like_string::TClassLikeString;
use mago_codex::ttype::union::TUnion;
use mago_names::ResolvedNames;
use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::super::Checker;
use super::{MODEL, REQUEST, is_a, is_object_of, octane};

const AUTH_FACADE: &[u8] = b"illuminate\\support\\facades\\auth";

/// `$node->name->toString()` after php-parser's NameResolver: an unqualified function name as written (it falls back
/// to the global function), else the resolved name.
pub(super) fn written_name(names: &ResolvedNames, id: &Identifier) -> String {
    let text = match id {
        Identifier::Local(l) if !names.is_imported(&l.span) => l.value,
        Identifier::FullyQualified(f) => &f.value[1..],
        _ => names.get(&id.span()),
    };
    String::from_utf8_lossy(text).into_owned()
}

pub(crate) fn function_call<'arena>(c: &mut Checker<'_, 'arena>, call: &FunctionCall<'arena>) {
    let Some(larastan) = c.larastan else {
        return;
    };
    let Expression::Identifier(id) = call.function else {
        return;
    };
    super::collectors::function_call(c, call, id);
    let name = written_name(c.names, id);
    let args = &call.argument_list.arguments;
    if name.eq_ignore_ascii_case("with") {
        useless_with(c, call, args.as_slice());
    } else if name.eq_ignore_ascii_case("value") {
        useless_value(c, call, args.as_slice());
    }
    if name == "env"
        && larastan
            .switches
            .contains(crate::LarastanSwitches::NO_ENV_CALLS_OUTSIDE_OF_CONFIG)
        && !in_config_directory(c, larastan)
    {
        c.report(
            call,
            "larastan.noEnvCallsOutsideOfConfig",
            "Called 'env' outside of the config directory which returns null when the config is cached, use \
             'config'."
                .to_string(),
            None,
        );
    }
}

/// NoUselessWithFunctionCallsRule.
fn useless_with<'arena>(
    c: &mut Checker<'_, 'arena>,
    call: &FunctionCall<'arena>,
    args: &[Argument<'arena>],
) {
    let message = match args {
        [] => return,
        [_] => {
            "Calling the helper function 'with()' with only one argument simply returns the value itself. If you \
             want to chain methods on a construct, use '(new ClassName())->foo()' instead"
        }
        [_, callback, ..] => {
            if !c
                .type_of(callback.value())
                .is_some_and(|t| not_callable(c, t))
            {
                return;
            }
            "Calling the helper function 'with()' without a callable as the second argument simply returns the \
             value without doing anything"
        }
    };
    c.report(
        call,
        "larastan.uselessConstructs.with",
        message.to_string(),
        None,
    );
}

/// NoUselessValueFunctionCallsRule.
fn useless_value<'arena>(
    c: &mut Checker<'_, 'arena>,
    call: &FunctionCall<'arena>,
    args: &[Argument<'arena>],
) {
    let Some(first) = args.first() else {
        return;
    };
    if !c.type_of(first.value()).is_some_and(cannot_be_closure) {
        return;
    }
    c.report(
        call,
        "larastan.uselessConstructs.value",
        "Calling the helper function 'value()' without a closure as the first argument simply returns the first \
         argument without doing anything"
            .to_string(),
        None,
    );
}

/// `$type->isCallable()->no()`.
fn not_callable(c: &Checker, ty: &TUnion) -> bool {
    !ty.types.is_empty()
        && ty.types.iter().all(|t| match t {
            TAtomic::Null | TAtomic::Void | TAtomic::Never => true,
            TAtomic::Scalar(TScalar::Bool(_) | TScalar::Integer(_) | TScalar::Float(_)) => true,
            TAtomic::Scalar(TScalar::String(s)) => s
                .get_known_literal_value()
                .is_some_and(|v| string_callable(c, v) == Some(false)),
            TAtomic::Scalar(TScalar::ClassLikeString(TClassLikeString::Literal { value })) => {
                string_callable(c, value.as_bytes()) == Some(false)
            }
            TAtomic::Object(TObject::Named(n)) => object_not_callable(c, n.name.as_bytes()),
            TAtomic::Object(TObject::Enum(e)) => object_not_callable(c, e.name.as_bytes()),
            _ => false,
        })
}

/// ObjectType::isCallable()->no(): a known final class without `__invoke`.
fn object_not_callable(c: &Checker, name: &[u8]) -> bool {
    c.codebase.get_class_like(name).is_some_and(|k| {
        k.flags.is_final() && super::super::members::method(c, k, b"__invoke").is_none()
    })
}

/// ConstantStringType::isCallable() on PHP 8: `Some(true)` for a function or a static `Class::method`, `None`
/// (maybe) for a `Class::method` of an unknown class or a missing method of a class that is not final.
fn string_callable(c: &Checker, value: &[u8]) -> Option<bool> {
    let value = value.strip_prefix(b"\\").unwrap_or(value);
    if !value.is_empty() && c.function(value).is_some() {
        return Some(true);
    }
    // '#^([a-zA-Z_\x7f-\xff\\][a-zA-Z0-9_\x7f-\xff\\]*)::([a-zA-Z_\x7f-\xff][a-zA-Z0-9_\x7f-\xff]*)\z#'
    let Some((class, method)) = value
        .windows(2)
        .position(|w| w == b"::")
        .map(|i| (&value[..i], &value[i + 2..]))
        .filter(|(k, m)| label(k, b'\\') && label(m, b'_'))
    else {
        return Some(false);
    };
    // an unknown class: maybe
    let k = c.codebase.get_class_like(class)?;
    match super::super::members::method(c, k, method) {
        // callable instance methods are PHP 7 only
        Some(method) => Some(method.is_static()),
        None if k.flags.is_final() => Some(false),
        None => None,
    }
}

/// A PHP label (`[a-zA-Z_\x7f-\xff][a-zA-Z0-9_\x7f-\xff]*`), `extra` also allowed anywhere.
fn label(s: &[u8], extra: u8) -> bool {
    let word = |b: &u8| b.is_ascii_alphabetic() || *b == b'_' || *b >= 0x7f || *b == extra;
    s.first().is_some_and(word) && s.iter().all(|b| word(b) || b.is_ascii_digit())
}

/// `$type->isSuperTypeOf(new ClosureType())->no()`.
fn cannot_be_closure(ty: &TUnion) -> bool {
    !ty.types.is_empty()
        && ty.types.iter().all(|t| match t {
            TAtomic::Null | TAtomic::Void | TAtomic::Never => true,
            TAtomic::Scalar(_) | TAtomic::Array(_) | TAtomic::Resource(_) => true,
            TAtomic::Object(TObject::Named(n)) => {
                !n.name.as_bytes().eq_ignore_ascii_case(b"closure")
            }
            TAtomic::Object(TObject::Enum(_)) => true,
            _ => false,
        })
}

/// `isCalledOutsideOfConfig()`'s negation: the file is below one of the config directories.
fn in_config_directory(c: &Checker, larastan: &super::Larastan) -> bool {
    let file = std::path::absolute(c.file_name()).unwrap_or_default();
    larastan
        .config_directories
        .iter()
        .flat_map(|g| glob(g))
        .any(|d| std::path::Path::new(&d).is_dir() && file.to_string_lossy().starts_with(&d))
}

/// PHP's `glob()` without flags on an absolute pattern: `*`, `?` and `[...]` within one path segment, the
/// matches sorted.
fn glob(pattern: &str) -> Vec<String> {
    let mut found = vec![String::new()];
    for segment in pattern.split('/').filter(|s| !s.is_empty()) {
        if !segment.contains(['*', '?', '[']) {
            found
                .iter_mut()
                .for_each(|base| *base = format!("{base}/{segment}"));
            continue;
        }
        let mut next = Vec::new();
        for base in &found {
            let mut names: Vec<String> =
                std::fs::read_dir(if base.is_empty() { "/" } else { base })
                    .into_iter()
                    .flatten()
                    .flatten()
                    .map(|e| e.file_name().to_string_lossy().into_owned())
                    .filter(|n| {
                        (!n.starts_with('.') || segment.starts_with('.'))
                            && pun_phpstan::php::fnmatch(segment, n)
                    })
                    .collect();
            names.sort();
            next.extend(names.into_iter().map(|n| format!("{base}/{n}")));
        }
        found = next;
    }
    found.retain(|p| std::path::Path::new(p).exists());
    found
}

pub(crate) fn static_method_call<'arena>(
    c: &mut Checker<'_, 'arena>,
    call: &StaticMethodCall<'arena>,
) {
    let Some(larastan) = c.larastan else {
        return;
    };
    super::dispatch::check(c, call);
    super::config::check(c, call);
    super::relations::static_method_call(c, call);
    super::collectors::static_method_call(c, call);
    let ClassLikeMemberSelector::Identifier(method) = &call.method else {
        return;
    };
    if larastan
        .switches
        .contains(crate::LarastanSwitches::NO_MODEL_MAKE)
        && method.value == b"make"
        && called_on_model(c, call.class)
    {
        c.report(
            call,
            "larastan.noModelMake",
            "Called 'Model::make()' which performs unnecessary work, use 'new Model()'."
                .to_string(),
            None,
        );
    }
    if larastan
        .switches
        .contains(crate::LarastanSwitches::CHECK_AUTH_CALLS_WHEN_IN_REQUEST_SCOPE)
        && let Expression::Identifier(id) = call.class
        && is_a(c, c.names.get(&id.span()), AUTH_FACADE)
    {
        let message = match method.value {
            b"check" => {
                "Do not use Auth::check() in a class that has access to the request. Use ${}->user() !== null \
                 instead."
            }
            b"user" => {
                "Do not use Auth::user() in a class that has access to the request. Use ${}->user() instead."
            }
            b"guest" => {
                "Do not use Auth::guest() in a class that has access to the request. Use ${}->user() === null \
                 instead."
            }
            _ => return,
        };
        request_scope(c, call, message, "larastan.noAuthFacadeInRequestScope");
    }
}

/// NoModelMakeRule::isCalledOnModel().
fn called_on_model<'arena>(c: &Checker<'_, 'arena>, class: &Expression<'arena>) -> bool {
    match class {
        Expression::Identifier(id) => is_a(c, c.names.get(&id.span()), MODEL),
        // `self`, `static` and `parent`: Scope::resolveName()
        Expression::Self_(_) | Expression::Static(_) => c.class_scope().is_some_and(|k| is_a(c, k.name.as_bytes(), MODEL)),
        Expression::Parent(_) => c
            .class_scope()
            .and_then(|k| k.direct_parent_class)
            .is_some_and(|p| is_a(c, p.as_bytes(), MODEL)),
        e => c.type_of(e).is_some_and(|t| match t.types.as_ref() {
            // a constant string is a class-string when it names a class
            [
                TAtomic::Scalar(TScalar::ClassLikeString(TClassLikeString::Literal { value })),
                ..,
            ] => is_a(c, value.as_bytes(), MODEL),
            [TAtomic::Scalar(TScalar::String(s)), ..] => s
                .get_known_literal_value()
                .is_some_and(|v| is_a(c, v, MODEL)),
            _ => is_object_of(c, t, MODEL),
        }),
    }
}

/// Reports `message` (with `%s` as `this` or `request`) in a Request subclass or where `$request` is a Request.
fn request_scope(c: &mut Checker, at: &impl HasSpan, message: &str, identifier: &str) {
    let variable = if c
        .class_scope()
        .is_some_and(|k| super::class_is(c, k, REQUEST))
    {
        "this"
    } else if c.request_in_scope() {
        "request"
    } else {
        return;
    };
    c.report(at, identifier, message.replacen("{}", variable, 1), None);
}

pub(crate) fn method_call<'arena>(c: &mut Checker<'_, 'arena>, call: &MethodCall<'arena>) {
    let Some(larastan) = c.larastan else {
        return;
    };
    super::console::check(c, call);
    super::relations::method_call(c, call);
    super::collectors::method_call(c, call);
    if larastan
        .switches
        .contains(crate::LarastanSwitches::NO_UNNECESSARY_COLLECTION_CALL)
    {
        super::collections::unnecessary(c, call, &larastan.no_unnecessary_collection_call_methods);
    }
    if larastan
        .switches
        .contains(crate::LarastanSwitches::NO_UNNECESSARY_ENUMERABLE_TO_ARRAY_CALLS)
    {
        super::collections::check(c, call);
    }
    if larastan
        .switches
        .contains(crate::LarastanSwitches::CHECK_OCTANE_COMPATIBILITY)
    {
        octane::check(c, call);
    }
    if !larastan
        .switches
        .contains(crate::LarastanSwitches::CHECK_AUTH_CALLS_WHEN_IN_REQUEST_SCOPE)
    {
        return;
    }
    let ClassLikeMemberSelector::Identifier(method) = &call.method else {
        return;
    };
    let message = match method.value {
        b"check" => {
            "Do not use auth()->check() in a class that has access to the request. Use ${}->user() !== null instead."
        }
        b"user" => {
            "Do not use auth()->user() in a class that has access to the request. Use ${}->user() instead."
        }
        b"guest" => {
            "Do not use auth()->guest() in a class that has access to the request. Use ${}->user() === null instead."
        }
        _ => return,
    };
    // auth() without arguments is the AuthManager (Larastan's AuthExtension)
    let Expression::Call(Call::Function(f)) = call.object else {
        return;
    };
    let Expression::Identifier(id) = f.function else {
        return;
    };
    if written_name(c.names, id) != "auth" || !f.argument_list.arguments.is_empty() {
        return;
    }
    request_scope(c, call, message, "larastan.noAuthHelperInRequestScope");
}
