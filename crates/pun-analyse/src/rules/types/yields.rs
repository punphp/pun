//! Level 3's generators: YieldInGeneratorRule (`generator.outOfFunction`, `generator.returnType`), YieldTypeRule and
//! YieldFromTypeRule (`generator.keyType`, `generator.valueType`, `generator.nonIterable`, `generator.sendType`,
//! `generator.void`), for the yields of a function-like whose return type [`super::returns`] knows.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::types::{Level, NeverKind, Tri, Type, is_super_type_of, mixed, recommended_level};

use super::super::{Checker, declared};
use super::missing_return::generator_return;

pub(super) const IDENTIFIERS: &[&str] = &[
    "generator.outOfFunction",
    "generator.returnType",
    "generator.keyType",
    "generator.valueType",
    "generator.nonIterable",
    "generator.sendType",
    "generator.void",
];

/// The expressions whose type the rules read.
pub(super) fn typed(y: &Yield, spans: &mut Vec<Span>) {
    match y {
        Yield::Value(v) => spans.extend(v.value.map(|e| e.span())),
        Yield::Pair(p) => spans.extend([p.key.span(), p.value.span()]),
        Yield::From(f) => spans.push(f.iterator.span()),
    }
}

/// The yields outside any function-like.
#[derive(Default)]
struct Outside(Vec<Span>);

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Outside {
    fn walk_in_yield(&mut self, y: &'ast Yield<'arena>, (): &mut ()) {
        self.0.push(y.span());
    }
    fn walk_closure(&mut self, _: &'ast Closure<'arena>, (): &mut ()) {}
    fn walk_arrow_function(&mut self, _: &'ast ArrowFunction<'arena>, (): &mut ()) {}
    fn walk_function(&mut self, _: &'ast Function<'arena>, (): &mut ()) {}
    fn walk_method(&mut self, _: &'ast Method<'arena>, (): &mut ()) {}
    fn walk_property_hook(&mut self, _: &'ast PropertyHook<'arena>, (): &mut ()) {}
}

pub(in super::super) fn program(c: &mut Checker, program: &Program) {
    if c.level < 3 {
        return;
    }
    let mut o = Outside::default();
    o.walk_program(program, &mut ());
    for at in o.0 {
        c.report(
            at,
            "generator.outOfFunction",
            "Yield can be used only inside a function.".into(),
            None,
        );
    }
}

/// `getTemplateType(Generator::class, 'TSend')`, `None` for ErrorType.
fn generator_send(t: &Type) -> Option<Type> {
    match t {
        Type::Object(name, args, _) if name.eq_ignore_ascii_case("Generator") => {
            Some(args.get(2).cloned().unwrap_or_else(mixed))
        }
        Type::Never(_) => Some(Type::Never(NeverKind::Implicit)),
        _ => None,
    }
}

/// One `Generator expects key|value type` check: `None` when accepted.
fn expects(
    c: &Checker,
    what: &str,
    accepting: &Type,
    accepted: &Type,
    strict: bool,
) -> Option<(&'static str, String)> {
    if c.levels().accepts(accepting, accepted, strict) {
        return None;
    }
    let level = recommended_level(accepting, Some(accepted));
    let identifier = if what == "key" {
        "generator.keyType"
    } else {
        "generator.valueType"
    };
    let message = format!(
        "Generator expects {what} type {}, {} given.",
        accepting.describe(level),
        accepted.describe(level)
    );
    Some((identifier, message))
}

/// YieldFromTypeRule's checks of `yield from`, `None` for a non-iterable (reported).
fn from(
    c: &mut Checker,
    f: &YieldFrom,
    ret: &Type,
    strict: bool,
) -> Option<(Vec<(&'static str, String)>, Type)> {
    let t = c.phpstan_type(f.iterator)?;
    let iterable = t.is_iterable();
    if iterable.no() || (iterable.maybe() && !matches!(t, Type::Mixed { .. }) && c.level >= 7) {
        let described = declared::with_classes(c, || t.describe(Level::TypeOnly));
        let message = format!(
            "Argument of an invalid type {described} passed to yield from, only iterables are supported."
        );
        c.report(f.iterator, "generator.nonIterable", message, None);
        return None;
    }
    let mut errors = Vec::new();
    declared::with_classes(c, || {
        errors.extend(expects(
            c,
            "key",
            &ret.iterable_key_type(),
            &t.iterable_key_type(),
            strict,
        ));
        errors.extend(expects(
            c,
            "value",
            &ret.iterable_value_type(),
            &t.iterable_value_type(),
            strict,
        ));
        let (Some(theirs), Some(ours)) = (generator_send(&t), generator_send(ret)) else {
            return;
        };
        let fits = is_super_type_of(&theirs, &ours);
        if fits.no() || (fits.maybe() && c.level >= 7) {
            let message = format!(
                "Generator expects delegated TSend type {}, {} given.",
                theirs.describe(Level::TypeOnly),
                ours.describe(Level::TypeOnly)
            );
            errors.push(("generator.sendType", message));
        }
    });
    // `$scope->getType($yieldFrom)`: what the delegated generator returns
    Some((errors, generator_return(&t).unwrap_or_else(mixed)))
}

/// The yields of a function-like returning `ret`, with the spans of those that are statements (whose result is not
/// used: `isInFirstLevelStatement()`).
pub(super) fn check(
    c: &mut Checker,
    yields: &[&Yield],
    statements: &[Span],
    ret: &Type,
    strict: bool,
) {
    if matches!(ret, Type::Mixed { .. }) {
        return;
    }
    let generator = if matches!(
        ret,
        Type::Never(NeverKind::Explicit | NeverKind::NonAccepting)
    ) {
        Tri::No
    } else {
        ret.is_iterable().and(Tri::from(!ret.is_array().yes()))
    };
    for y in yields {
        if generator.no() || (generator.maybe() && c.level >= 7) {
            let message = "Yield can be used only with these return types: Generator, Iterator, Traversable, iterable.";
            c.report(y, "generator.returnType", message.into(), None);
        }
        let (key, value) = match y {
            Yield::From(f) => {
                let Some((errors, result)) = from(c, f, ret, strict) else {
                    continue;
                };
                for (identifier, message) in errors {
                    c.report(f.iterator, identifier, message, None);
                }
                if !statements.contains(&y.span()) && result.is_void().yes() {
                    c.report(
                        y,
                        "generator.void",
                        "Result of yield from (void) is used.".into(),
                        None,
                    );
                }
                continue;
            }
            Yield::Pair(p) => (c.phpstan_type(p.key), c.phpstan_type(p.value)),
            Yield::Value(v) => (
                Some(Type::Int),
                v.value.map_or(Some(Type::Null), |e| c.phpstan_type(e)),
            ),
        };
        let errors = declared::with_classes(c, || {
            let key = key.and_then(|k| expects(c, "key", &ret.iterable_key_type(), &k, strict));
            let value =
                value.and_then(|v| expects(c, "value", &ret.iterable_value_type(), &v, strict));
            key.into_iter().chain(value).collect::<Vec<_>>()
        });
        for (identifier, message) in errors {
            c.report(y, identifier, message, None);
        }
        // `$scope->getType($yield)`: what the generator is sent
        if !statements.contains(&y.span()) && generator_send(ret).is_some_and(|s| s.is_void().yes())
        {
            c.report(
                y,
                "generator.void",
                "Result of yield (void) is used.".into(),
                None,
            );
        }
    }
}
