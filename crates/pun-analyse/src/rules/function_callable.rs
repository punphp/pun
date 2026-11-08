//! FunctionCallableRule: first-class function callables, not method/static callables, pipes or partial applications.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::types::{Level, Type};

use super::types::flow::unparen;
use super::{Checker, DISCOVERING_SYMBOLS_TIP, declared, internal};

/// RuleLevelHelper's `checkThisOnly` applies to expression callables too, including parenthesized `$this`.
fn checked_expression(e: &Expression, level: u8) -> bool {
    !matches!(e, Expression::Identifier(_))
        && (level >= 2
            || matches!(unparen(e), Expression::Variable(Variable::Direct(v)) if v.name == b"$this"))
}

/// Only first-class function callables need their callee's type, not PHP 8.5 bound-argument applications.
pub(super) fn typed_expressions(program: &Program, level: u8) -> Vec<Span> {
    struct Wanted(u8, Vec<Span>);
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Wanted {
        fn walk_in_function_partial_application(
            &mut self,
            f: &'ast FunctionPartialApplication<'arena>,
            (): &mut (),
        ) {
            if internal::is_callable(&f.argument_list) && checked_expression(f.function, self.0) {
                self.1.push(unparen(f.function).span());
            }
        }
    }
    let mut wanted = Wanted(level, Vec::new());
    wanted.walk_program(program, &mut ());
    wanted.1
}

pub(super) fn check<'arena>(c: &mut Checker<'_, 'arena>, f: &FunctionPartialApplication<'arena>) {
    let whole = f.function.span().join(f.argument_list.span());
    if c.piped.contains(&whole) || !internal::is_callable(&f.argument_list) {
        return;
    }
    if let Expression::Identifier(id) = f.function {
        named(c, id);
        return;
    }
    if !checked_expression(f.function, c.level) {
        return;
    }
    let found = c.find_type_to_check(unparen(f.function), &|t| {
        declared::with_classes(c, || t.is_callable().yes())
    });
    for name in &found.unknown {
        c.report(
            f,
            "class.notFound",
            format!("Creating callable from an unknown class {name}."),
            Some(DISCOVERING_SYMBOLS_TIP),
        );
    }
    let Some(mut ty) = found.ty else { return };
    // NullsafeOperatorHelper removes only the chain's short-circuit null, not a nullable property/return. Mago
    // records that distinction; the syntax check prevents a copied nullsafe result from being unwrapped too.
    if super::assign::contains_nullsafe(f.function)
        && c.type_of(unparen(f.function))
            .is_some_and(|raw| raw.has_nullsafe_null())
    {
        ty = ty.remove_null();
        if matches!(ty, Type::Never(_)) {
            return;
        }
    }
    // Mago's bare callable/Closure carry synthetic variadic signatures, not known callable signatures. Match
    // their raw sentinels, not a converted signature's shape: explicit variadic PHPDoc signatures stay precise.
    for (common, bare) in [
        (
            mago_codex::ttype::get_mixed_callable(),
            Type::Callable(None),
        ),
        (
            mago_codex::ttype::get_mixed_closure(),
            Type::Object("Closure".into(), vec![], false),
        ),
    ] {
        if c.type_of(unparen(f.function))
            .is_some_and(|raw| raw.types.iter().any(|atom| *atom == common.types[0]))
        {
            let signature = c.to_phpstan(&common);
            let restore = |t: &Type| {
                if *t == signature {
                    bare.clone()
                } else {
                    t.clone()
                }
            };
            ty = if ty == signature {
                restore(&ty)
            } else {
                ty.traverse(&restore)
            };
        }
    }
    let callable = declared::with_classes(c, || ty.is_callable());
    if callable.no() || c.level >= 7 && callable.maybe() {
        let value = declared::with_classes(c, || ty.describe(Level::Value));
        let not = if callable.no() {
            "it's not"
        } else {
            "it might not be"
        };
        c.report(
            f,
            "callable.nonCallable",
            format!("Creating callable from {value} but {not} a callable."),
            None,
        );
    }
}

fn named<'arena>(c: &mut Checker<'_, 'arena>, id: &Identifier<'arena>) {
    let fallback = match id {
        Identifier::Local(l) if !c.names.is_imported(&l.span) => Some(l.value),
        _ => None,
    };
    if let Some((called, display)) = std::iter::once(c.names.get(&id.span()))
        .chain(fallback)
        .find_map(|name| c.function(name).map(|(display, _)| (name, display)))
    {
        if c.on(crate::rules::Switches::CHECK_FUNCTION_NAME_CASE)
            && display.as_bytes().eq_ignore_ascii_case(called)
            && display.as_bytes() != called
        {
            let name = c.written_or_resolved(id);
            c.report(
                id,
                "function.nameCase",
                format!("Call to function {display}() with incorrect case: {name}"),
                None,
            );
        }
        return;
    }
    let name = c.written_or_resolved(id);
    if !c.in_function_exists(&name) {
        c.report(
            id,
            "function.notFound",
            format!("Function {name} not found."),
            None,
        );
    }
}
