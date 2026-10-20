//! FunctionCallParametersCheck's checks of level 5 (`checkFunctionArgumentTypes`, `checkArgumentsPassedByReference`):
//! each argument's type against its parameter's (`argument.type`), by-reference parameters given no variable
//! (`argument.byRef`) and unpacked non-iterables (`argument.unpackNonIterable`).

use mago_span::Span;
use pun_phpstan::symbols::ByRef;
use pun_phpstan::types::{Type, mixed, recommended_level};

use super::super::params::{Acceptor, ArgKind, Call, Entry, ParamType};
use super::super::{Checker, declared};
use super::returns::strict_types;

pub const IDENTIFIERS: &[&str] = &["argument.type", "argument.byRef", "argument.unpackNonIterable"];

/// `describeParameter()`: `Parameter #1 $a`, `Parameter $a` (named), `Named argument b for variadic parameter ...$a`.
fn describe(acceptor: &Acceptor, p: usize, position_or_name: Option<Result<usize, &str>>) -> String {
    let variadic = acceptor.variadics[p];
    let head = match position_or_name {
        Some(Ok(i)) => format!("Parameter #{}", i + 1),
        Some(Err(name)) if variadic => format!("Named argument {name} for variadic parameter"),
        _ => "Parameter".to_string(),
    };
    let dots = if variadic { "..." } else { "" };
    format!("{head} {dots}${}", acceptor.names[p])
}

/// The parameter's declared type, `None` without one (an implicit `mixed`) or for variants combined with types Pun
/// does not join.
pub(in super::super) fn declared_type(c: &Checker, acceptor: &Acceptor, p: usize) -> Option<Type> {
    match &acceptor.param_types[p] {
        ParamType::Builtin(s) => Some(pun_phpstan::resolve::resolve_str(s, acceptor.class.as_deref())),
        ParamType::User => {
            let (class, name) = acceptor.callee.as_ref()?;
            let (f, declaring) = match class {
                Some(k) => (c.codebase.get_method(k.as_bytes(), name.as_bytes())?, c.codebase.get_class_like(k.as_bytes())),
                None => (c.codebase.get_function(name.as_bytes())?, None),
            };
            // ponytail: computed per argument, not cached per callee
            declared::codex_params(c, f, declaring).into_iter().nth(p)
        }
    }
}

/// A type only the call resolves: a template type (ResolvedFunctionVariant), `static`/`$this` or a conditional type.
pub(in super::super) fn is_late(t: &Type) -> bool {
    let late = std::cell::Cell::new(matches!(t, Type::Template(_) | Type::Static(..) | Type::This(_) | Type::Conditional(_)));
    t.traverse(&|i| {
        if matches!(i, Type::Template(_) | Type::Static(..) | Type::This(_) | Type::Conditional(_)) {
            late.set(true);
        }
        i.clone()
    });
    late.get()
}

/// The parameter's type for the check, `None` when it is not checked.
// ponytail: types resolved per call in PHPStan (see [`is_late()`]) are skipped
fn parameter_type(c: &Checker, acceptor: &Acceptor, p: usize) -> Option<Type> {
    declared_type(c, acceptor, p).filter(|t| !is_late(t) && *t != mixed())
}

/// `$methodReflection->returnsByReference()->yes()` of method `name` of class `class`.
fn returns_by_ref(c: &Checker, class: Option<&str>, name: &[u8]) -> bool {
    class.and_then(|n| c.class_like(n).found()).is_some_and(|k| {
        matches!(super::super::members::method(c, k, name), Some(super::super::members::Method::User { f, .. }) if f.flags.is_by_reference())
    })
}

impl Checker<'_, '_> {
    fn type_at(&self, span: Span) -> Option<Type> {
        self.types.get(&span).map(|t| self.to_phpstan(t))
    }
}

/// The rest of `check()` after `processArguments()`, `parameters` being each argument's parameter.
pub(in super::super) fn arguments(c: &mut Checker, acceptor: &Acceptor, args: &[Entry], parameters: &[Option<usize>], call: &Call) {
    // `%s of function foo expects ...`: the subject with a lowercase first letter
    let of = call.subject[..1].to_ascii_lowercase() + &call.subject[1..];
    let strict = strict_types(c);
    let levels = c.levels();
    for (i, (e, p)) in args.iter().zip(parameters).enumerate() {
        if e.unpack {
            let ty = c.type_at(e.value);
            let found = c.find_type_to_check_of(ty, &|t| t.is_iterable().yes());
            if let Some(t) = found
                && !t.is_iterable().yes()
            {
                let described = declared::with_classes(c, || t.describe(pun_phpstan::types::Level::TypeOnly));
                let message = format!("Only iterables can be unpacked, {described} given in argument #{}.", i + 1);
                c.report(e.span, "argument.unpackNonIterable", message, None);
            }
        }
        let Some(p) = *p else {
            continue;
        };
        let by_ref = acceptor.by_ref[p];
        let value = c.type_at(e.value).map(|t| if e.unpack { t.iterable_value_type() } else { t });
        // a built-in's by-reference parameter that creates the variable takes anything
        if (by_ref != ByRef::CreatesNewVariable || !acceptor.builtin)
            && let (Some(param), Some(value)) = (parameter_type(c, acceptor, p), value)
        {
            let strict = strict && (!acceptor.builtin || !param.is_callable().yes());
            let message = declared::with_classes(c, || {
                (!levels.accepts(&param, &value, strict)).then(|| {
                    let level = recommended_level(&param, Some(&value));
                    format!(
                        "{} of {of} expects {}, {} given.",
                        describe(acceptor, p, Some(e.name.map_or(Ok(i), Err))),
                        param.describe(level),
                        value.describe(level)
                    )
                })
            });
            if let Some(message) = message {
                c.report(e.span, "argument.type", message, None);
            }
        }
        if by_ref == ByRef::No {
            continue;
        }
        let fine = match e.kind {
            ArgKind::Nullsafe | ArgKind::Other => false,
            ArgKind::Writable | ArgKind::OtherCall => true,
            ArgKind::Function(id) => c.codebase.get_function(c.names.get(id)).is_some_and(|f| f.flags.is_by_reference()),
            // callReturnsByReference(): a method PHPStan does not find returns no reference
            ArgKind::MethodCall(object, Some(name)) => {
                let class = if c.written(*object) == "$this" {
                    c.class_scope().map(|k| String::from_utf8_lossy(k.name.as_bytes()).into_owned())
                } else {
                    c.type_at(*object).and_then(|t| t.object_class_names().into_iter().next())
                };
                returns_by_ref(c, class.as_deref(), name)
            }
            ArgKind::StaticCall(class, Some(name)) => {
                let class = match class {
                    Some(id) => Some(String::from_utf8_lossy(c.names.get(id)).into_owned()),
                    None => c.class_scope().map(|k| String::from_utf8_lossy(k.name.as_bytes()).into_owned()),
                };
                returns_by_ref(c, class.as_deref(), name)
            }
            ArgKind::MethodCall(_, None) | ArgKind::StaticCall(_, None) => false,
        };
        if fine {
            continue;
        }
        let parameter = describe(acceptor, p, e.name.is_none().then_some(Ok(i)));
        let message = match call.kind {
            super::super::params::CallKind::New => {
                format!(" {parameter} of {of} is passed by reference, so it expects variables only")
            }
            super::super::params::CallKind::Attribute => {
                format!("{parameter} of {of} is passed by reference, so it expects variables only")
            }
            _ => format!("{parameter} of {of} is passed by reference, so it expects variables only."),
        };
        c.report(e.span, "argument.byRef", message, None);
    }
}
