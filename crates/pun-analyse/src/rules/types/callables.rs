//! CallCallablesRule (level 2): calling an expression (`$f()`) checks it is callable and its arguments against the
//! callable's parameters; CallUserFuncRule (level 5): `call_user_func()` and `call_user_func_array()` check theirs.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::types::{Level, Signature, Type};

use super::super::params::{self, Acceptor, ParamType, Variants};
use super::super::{Checker, calls, declared, members};
use super::flow::unparen;

pub const IDENTIFIERS: &[&str] = &["callable.nonCallable", "callable.inaccessibleMethod", "callable.void"];

/// The callee expression of a call by expression.
pub(super) fn typed(call: &FunctionCall, spans: &mut Vec<Span>) {
    if !matches!(call.function, Expression::Identifier(_)) {
        spans.push(call.function.span());
    }
}

/// A pipe's callable and input.
pub(super) fn typed_pipe(p: &Pipe, spans: &mut Vec<Span>) {
    spans.extend([p.callable.span(), p.input.span()]);
}

/// A callable's acceptor, retaining each signature's parameter names even in a callable union.
fn of_signature(c: &Checker, s: &Signature) -> Acceptor {
    Acceptor {
        optional: s.params.iter().map(|p| p.optional || p.variadic).collect(),
        variadic: s.params.iter().any(|p| p.variadic),
        void: s.ret == Type::Void,
        names: s.params.iter().map(|p| p.name.clone()).collect(),
        types: s.params.iter().map(|p| declared::with_classes(c, || p.ty.describe(Level::TypeOnly))).collect(),
        variadics: s.params.iter().map(|p| p.variadic).collect(),
        builtin: false,
        accepts_named_arguments: pun_phpstan::types::Tri::Yes,
        param_types: s.params.iter().map(|p| ParamType::Typed(p.ty.clone())).collect(),
        by_ref: s
            .params
            .iter()
            .map(|p| if p.by_ref { pun_phpstan::symbols::ByRef::CreatesNewVariable } else { pun_phpstan::symbols::ByRef::No })
            .collect(),
        class: None,
        callee: None,
    }
}

/// `InaccessibleMethod`: no parameters, variadic.
fn inaccessible() -> Acceptor {
    Acceptor {
        optional: vec![],
        variadic: true,
        void: false,
        names: vec![],
        types: vec![],
        variadics: vec![],
        builtin: false,
        accepts_named_arguments: pun_phpstan::types::Tri::Yes,
        param_types: vec![],
        by_ref: vec![],
        class: None,
        callee: None,
    }
}

/// The acceptors of `getCallableParametersAcceptors()`; `None` when Pun does not know them. `inaccessible` gets
/// the private or protected method a callable names out of its scope.
fn acceptors(c: &Checker, t: &Type, inaccessible: &mut Option<String>) -> Option<Vec<Acceptor>> {
    let method = |class: &str, name: &str, inaccessible: &mut Option<String>| -> Option<Vec<Acceptor>> {
        let k = c.class_like(class).found()?;
        let m = members::method(c, k, name.as_bytes())?;
        if !calls::can_call(c, c.class_scope(), &m.declaring_name(), m.visibility()) {
            let v = if m.visibility().is_private() { "private" } else { "protected" };
            *inaccessible = Some(format!("Call to {v} method {}() of class {}.", m.name(), m.declaring_display(c)));
            return Some(vec![self::inaccessible()]);
        }
        Some(m.variants().all)
    };
    match t {
        Type::Closure(s) | Type::Callable(Some(s)) => Some(vec![of_signature(c, s)]),
        Type::ConstString(s) => match s.split_once("::") {
            Some((class, name)) => method(class, name, inaccessible),
            None => c.function(s.as_bytes()).map(|(_, v)| v.all),
        },
        Type::ConstArray(a) if a.values.len() == 2 => {
            let class = match &a.values[0] {
                Type::ConstString(s) => s.clone(),
                Type::Object(n, _) | Type::Static(n, _) | Type::This(n) => n.clone(),
                _ => return None,
            };
            let Type::ConstString(name) = &a.values[1] else { return None };
            method(&class, name, inaccessible)
        }
        Type::Object(n, _) | Type::Static(n, _) | Type::This(n) => method(n, "__invoke", inaccessible),
        Type::Union(ts) => ts.iter().map(|t| acceptors(c, t, inaccessible)).collect::<Option<Vec<Vec<_>>>>().map(|v| v.concat()),
        _ => None,
    }
}

/// PipeOperatorRule (level 0): only the first parameter of each callable acceptor matters.
pub(in super::super) fn pipe_by_ref(c: &mut Checker, p: &Pipe) {
    let right = unparen(p.callable);
    // RuleLevelHelper's checkThisOnly is enabled at levels 0 and 1.
    if c.level < 2 && !matches!(right, Expression::Variable(Variable::Direct(v)) if v.name == b"$this") {
        return;
    }
    let Some(t) = c.find_type_to_check(p.callable, &|t| declared::with_classes(c, || t.is_callable().yes())).ty else {
        return;
    };
    if !declared::with_classes(c, || t.is_callable().yes()) {
        return;
    }
    let Some(all) = acceptors(c, &t, &mut None) else {
        return;
    };
    for acceptor in all {
        for (by_ref, name) in acceptor.by_ref.iter().zip(&acceptor.names) {
            if *by_ref == pun_phpstan::symbols::ByRef::No {
                break;
            }
            let name = if name.is_empty() { String::new() } else { format!(" ${name}") };
            c.report(right.span(), "pipe.byRef", format!("Parameter #1{name} of callable on the right side of pipe operator is passed by reference."), None);
            return;
        }
    }
}

/// CallCallablesRule on `$callable(...)`.
pub(in super::super) fn call(c: &mut Checker, call: &FunctionCall) {
    if !matches!(call.function, Expression::Identifier(_)) {
        invoke(c, call.function, &params::count(&call.argument_list), call.span());
    }
}

/// `$input |> $callable`: PHPStan analyses it as `$callable($input)` (`f(...)` callables are calls.rs's).
pub(in super::super) fn pipe(c: &mut Checker, p: &Pipe) {
    if !matches!(p.callable, Expression::PartialApplication(_)) {
        invoke(c, p.callable, &params::Counts::of_value(p.input), p.span());
    }
}

fn invoke(c: &mut Checker, callee: &Expression, counts: &params::Counts, at: Span) {
    if c.level < 2 {
        return;
    }
    let found = {
        let c = &*c;
        c.find_type_to_check(callee, &|t| declared::with_classes(c, || t.is_callable().yes()))
    };
    for n in &found.unknown {
        c.report(at, "class.notFound", format!("Invoking callable on an unknown class {n}."), Some(super::super::DISCOVERING_SYMBOLS_TIP));
    }
    let Some(t) = found.ty else {
        return;
    };
    // (a `mixed` left by findTypeToCheck() is a StrictMixedType, never callable)
    let callable = if matches!(t, Type::Mixed { .. }) { pun_phpstan::types::Tri::No } else { declared::with_classes(c, || t.is_callable()) };
    let value = declared::with_classes(c, || t.describe(Level::Value));
    if callable.no() || c.level >= 7 && callable.maybe() {
        let not = if callable.no() { "it's not" } else { "it might not be" };
        c.report(at, "callable.nonCallable", format!("Trying to invoke {value} but {not} a callable."), None);
        return;
    }
    let mut inaccessible = None;
    let Some(all) = acceptors(c, &t, &mut inaccessible) else {
        return;
    };
    if let Some(message) = inaccessible.filter(|_| all.len() == 1) {
        c.report(at, "callable.inaccessibleMethod", message, None);
    }
    let described = if matches!(t, Type::Closure(_)) { "closure".to_string() } else { format!("callable {value}") };
    let first_level = c.is_top_level(at);
    check(c, all, counts, at, &described, params::CallKind::Callable, first_level);
}

/// FunctionCallParametersCheck with the callable's description.
fn check(
    c: &mut Checker,
    all: Vec<Acceptor>,
    counts: &params::Counts,
    at: Span,
    described: &str,
    kind: params::CallKind,
    first_level: bool,
) {
    // CallCallablesRule / ArgumentsNormalizer combine the whole callable type's contract with AND, before
    // signature selection (whose combineAcceptors uses OR).
    let accepts_named = all.iter().fold(pun_phpstan::types::Tri::Yes, |accepts, a| accepts.and(a.accepts_named(c)));
    let mut acceptor = params::select(c, Variants { all, named: None }, counts);
    acceptor.accepts_named_arguments = accepts_named;
    let subject = described[..1].to_ascii_uppercase() + &described[1..];
    let call = params::Call { kind, subject: &subject, void_message: Some(format!("Result of {described} (void) is used.")) };
    params::check(c, &acceptor, counts, at, call, first_level);
}

/// CallUserFuncRule on `call_user_func()` and `call_user_func_array()`.
pub(in super::super) fn call_user_func(c: &mut Checker, call: &FunctionCall) {
    let Expression::Identifier(id) = call.function else {
        return;
    };
    if c.level < 5 || call.argument_list.arguments.is_empty() {
        return;
    }
    let fallback = match id {
        Identifier::Local(l) if !c.names.is_imported(&l.span) => Some(l.value),
        _ => None,
    };
    let Some((function, _)) = std::iter::once(c.names.get(&id.span())).chain(fallback).find_map(|n| c.function(n)) else {
        return;
    };
    let function = function.as_str();
    if !matches!(function, "call_user_func" | "call_user_func_array") {
        return;
    }
    let args: Vec<&Argument> = call.argument_list.arguments.iter().collect();
    // the callback: the first positional argument or the one named `callback`
    let Some(callback) = args.iter().enumerate().position(|(i, a)| match a {
        Argument::Named(n) => n.name.value == b"callback",
        Argument::Positional(_) => i == 0,
    }) else {
        return;
    };
    let rest = args.iter().enumerate().filter(|(i, _)| *i != callback).map(|(_, a)| *a);
    let counts = if function == "call_user_func" {
        params::Counts::of(rest)
    } else {
        // the arguments array: the second positional argument or the one named `args`
        let Some(array) = args.iter().enumerate().find_map(|(i, a)| match a {
            Argument::Positional(p) if i == 1 && p.ellipsis.is_none() => Some(p.value),
            Argument::Named(n) if n.name.value == b"args" => Some(n.value),
            _ => None,
        }) else {
            return;
        };
        let elements: Vec<&ArrayElement> = match unparen(array) {
            Expression::Array(a) => a.elements.iter().collect(),
            _ => return,
        };
        let Some(counts) = params::Counts::of_array(&elements) else {
            return;
        };
        counts
    };
    let value = args[callback].value();
    let Some(t) = c.phpstan_type(value).filter(|t| declared::with_classes(c, || t.is_callable().yes())) else {
        return;
    };
    let mut inaccessible = None;
    let Some(all) = acceptors(c, &t, &mut inaccessible) else {
        return;
    };
    let first_level = c.is_top_level(call.span());
    let described = format!("callable passed to {function}()");
    check(c, all, &counts, call.span(), &described, params::CallKind::Function, first_level);
}
