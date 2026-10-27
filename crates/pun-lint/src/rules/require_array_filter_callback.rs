//! `lint/functions/require-array-filter-callback`, ported from phpstan-strict-rules 2.0.12's `ArrayFilterStrictRule`
//! (`arrayFilter.strict`), with PHPStan's defaults: PHPDoc types certain, `checkNullables` off (as below level 8).
//!
//! A call of PHP's `array_filter()` (the name `FuncCallNameResolver` resolves, arguments in parameter order as
//! `ArgumentsNormalizer::reorderArgs` puts them, a skipped `callback` taking its default `null`) is reported:
//!
//! - With only the array, unless the array's value type is `bool`, an array, or a union whose every member is
//!   always truthy or always falsy with both kinds present (`Foo|null`, `0|1`). PHPStan merges a `0` into an integer
//!   range it touches (`positive-int|0` is `int<0, max>`) and `true|false` into `bool`.
//! - With a callback whose type is `null`: `Parameter #2 of array_filter() cannot be null ... (null given).`

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::int::TInteger;
use mago_codex::ttype::get_iterable_parameters;
use mago_syntax::cst::{Argument, Expression, Node, NodeKind};

use super::refactor::{function_name, is_name};
use super::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct RequireArrayFilterCallback;

/// One argument after `ArgumentsNormalizer::reorderArgs`: an expression, or a skipped parameter's default (`null`
/// for `callback`).
enum Arg<'a> {
    Given(&'a Expression<'a>),
    Default,
}

/// `reorderArgs` for `array_filter(array $array, ?callable $callback = null, int $mode = 0)`. Without names the
/// arguments stay as written. A named one moves to its parameter's place unless that place is taken (then it is
/// dropped), and a name no parameter has goes after the rest. Holes before the last place take the default, and a
/// hole at `$array`, which has none, makes PHPStan give up (`None`).
fn reorder<'a>(args: &[&'a Argument<'a>]) -> Option<Vec<Arg<'a>>> {
    const PARAMS: [&[u8]; 3] = [b"array", b"callback", b"mode"];
    let mut slots: Vec<Option<&'a Expression<'a>>> = Vec::new();
    let mut appended = Vec::new();
    for (i, arg) in args.iter().enumerate() {
        let at = match arg {
            Argument::Positional(_) => i,
            Argument::Named(named) => match PARAMS.iter().position(|p| named.name.value == *p) {
                Some(at) => at,
                None => {
                    appended.push(Arg::Given(arg.value()));
                    continue;
                }
            },
        };
        if slots.len() <= at {
            slots.resize(at + 1, None);
        }
        if slots[at].is_none() {
            slots[at] = Some(arg.value());
        }
    }
    if slots.is_empty() {
        return Some(appended);
    }
    slots[0]?;
    let mut out: Vec<Arg<'a>> = slots.into_iter().map(|s| s.map_or(Arg::Default, Arg::Given)).collect();
    out.append(&mut appended);
    Some(out)
}

/// `Type::toBoolean()` of one member: `Some(true)` always truthy, `Some(false)` always falsy (`array{}` too, which
/// Mago leaves undecided), `None` either.
fn truthiness(atomic: &TAtomic) -> Option<bool> {
    if atomic.is_truthy() {
        Some(true)
    } else if atomic.is_falsy() || matches!(atomic, TAtomic::Array(array) if array.is_empty()) {
        Some(false)
    } else {
        None
    }
}

/// Whether the array value type `values` lets `array_filter()` go without a callback.
fn allowed(values: &[TAtomic]) -> bool {
    let is_bool = |a: &TAtomic| matches!(a, TAtomic::Scalar(TScalar::Bool(_)));
    // `isBoolean()->yes()`, `true|false` being `bool`.
    if !values.is_empty() && values.iter().all(is_bool) {
        return true;
    }
    if let [one] = values {
        return matches!(one, TAtomic::Array(_));
    }
    if values.is_empty() {
        return false;
    }
    let has = |b: bool| values.iter().any(|a| matches!(a, TAtomic::Scalar(TScalar::Bool(x)) if x.value == Some(b)));
    if has(true) && has(false) {
        return false;
    }
    // A `0` next to or inside an integer range merges into it.
    let zero = values.iter().any(|a| matches!(a, TAtomic::Scalar(TScalar::Integer(TInteger::Literal(0)))));
    let touches_zero = values.iter().any(|a| match a {
        TAtomic::Scalar(TScalar::Integer(i)) if !i.is_literal() => {
            i.get_minimum_value().is_none_or(|min| min <= 1) && i.get_maximum_value().is_none_or(|max| max >= -1)
        }
        _ => false,
    });
    if zero && touches_zero {
        return false;
    }
    let (mut truthy, mut falsy) = (false, false);
    for atomic in values {
        match truthiness(atomic) {
            Some(true) => truthy = true,
            Some(false) => falsy = true,
            None => return false,
        }
    }
    truthy && falsy
}

impl Rule for RequireArrayFilterCallback {
    fn id(&self) -> &'static str {
        "lint/functions/require-array-filter-callback"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(call) = node else { return };
        let Expression::Identifier(name) = call.function else {
            return;
        };
        if !is_name(&function_name(ctx, ancestors, name), "array_filter") {
            return;
        }
        let given: Vec<&Argument<'_>> = call.argument_list.arguments.iter().collect();
        let Some(args) = reorder(&given) else {
            return;
        };
        let facts = facts(ctx);
        let message = match &args[..] {
            [Arg::Given(array)] => {
                let mut values: Vec<TAtomic> = Vec::new();
                let known = facts.full_type(node_span(unparenthesize(array))).is_some_and(|t| {
                    t.types.iter().all(|atomic| match get_iterable_parameters(atomic, &facts.codebase) {
                        Some((_, value)) => {
                            for v in value.types.iter() {
                                if !values.contains(v) {
                                    values.push(v.clone());
                                }
                            }
                            true
                        }
                        None => false,
                    })
                });
                if known && allowed(&values) {
                    return;
                }
                "Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics."
            }
            [_, callback, ..] => {
                let null = match callback {
                    Arg::Default => true,
                    Arg::Given(e) => facts
                        .full_type(node_span(unparenthesize(e)))
                        .is_some_and(|t| !t.types.is_empty() && t.types.iter().all(TAtomic::is_null)),
                };
                if !null {
                    return;
                }
                "Parameter #2 of array_filter() cannot be null to avoid loose comparison semantics (null given)."
            }
            _ => return,
        };
        out.push(Report {
            span: node_span(call),
            message: message.to_string(),
            help: None,
            fix: None,
        });
    }
}
