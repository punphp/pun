//! CheckDispatchArgumentTypesCompatibleWithClassConstructorRule, registered twice: for jobs (the Bus `Dispatchable`
//! trait) and for events (the Events `Dispatchable` trait). `Job::dispatch(...)` passes its arguments to the
//! constructor; at level 0 FunctionCallParametersCheck counts them.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::super::{Checker, members, params};

/// One registration: the trait, the methods it dispatches with, and the kind in the messages.
type Dispatchable = (&'static [u8], &'static [&'static [u8]], &'static str);

/// The two registrations.
const DISPATCHABLES: &[Dispatchable] = &[
    (
        b"Illuminate\\Foundation\\Bus\\Dispatchable",
        &[
            b"dispatch",
            b"dispatchIf",
            b"dispatchUnless",
            b"dispatchSync",
            b"dispatchNow",
            b"dispatchAfterResponse",
        ],
        "job",
    ),
    (
        b"Illuminate\\Foundation\\Events\\Dispatchable",
        &[b"dispatch", b"dispatchIf", b"dispatchUnless"],
        "event",
    ),
];

/// `dispatchIf()` and `dispatchUnless()`, whose first argument is the condition.
pub(super) fn conditional(method: &[u8]) -> bool {
    method == b"dispatchIf" || method == b"dispatchUnless"
}

pub(super) fn check<'arena>(c: &mut Checker<'_, 'arena>, call: &StaticMethodCall<'arena>) {
    let ClassLikeMemberSelector::Identifier(method) = &call.method else {
        return;
    };
    // `$scope->resolveName($node->class)`
    // (an unknown class stops PHPStan: `getClass()` throws)
    let class = match call.class {
        Expression::Identifier(id) => Some(c.names.get(&id.span()).to_vec()),
        Expression::Self_(_) | Expression::Static(_) => {
            c.class_scope().map(|k| k.name.as_bytes().to_vec())
        }
        Expression::Parent(_) => c
            .class_scope()
            .and_then(|k| k.direct_parent_class)
            .map(|p| p.as_bytes().to_vec()),
        _ => None,
    };
    let Some((class, k)) = class.and_then(|n| c.codebase.get_class_like(&n).map(|k| (n, k))) else {
        return;
    };
    for (dispatchable, methods, kind) in DISPATCHABLES {
        if !methods.contains(&method.value) || !c.codebase.class_uses_trait(&class, dispatchable) {
            continue;
        }
        let display = c.display(k);
        let identifier = format!("larastan.{kind}s.noConstructor");
        let kind = format!("{}{}", kind[..1].to_uppercase(), &kind[1..]);
        let args = call.argument_list.arguments.as_slice();
        let Some(constructor) = members::method(c, k, b"__construct") else {
            let required = usize::from(conditional(method.value));
            if args.len() > required {
                c.report(
                    call,
                    &identifier,
                    format!(
                        "{kind} class {display} does not have a constructor and must be dispatched without any \
                         parameters."
                    ),
                    None,
                );
            }
            continue;
        };
        let mut counts = params::count(&call.argument_list);
        if conditional(method.value) {
            let Some(first) = args.first() else {
                continue;
            };
            let boolean = c.type_of(first.value()).is_some_and(|t| {
                !t.types.is_empty()
                    && t.types
                        .iter()
                        .all(|a| matches!(a, TAtomic::Scalar(TScalar::Bool(_))))
            });
            if !boolean {
                continue;
            }
            // the condition is no constructor argument
            match first {
                Argument::Named(_) => {
                    counts.named = args[1..].iter().any(|a| matches!(a, Argument::Named(_)));
                }
                Argument::Positional(_) => counts.positional -= 1,
            }
        }
        let acceptor = params::select(constructor.variants(), counts.positional, counts.unpack);
        if counts.named {
            continue;
        }
        let min = acceptor.optional.iter().filter(|o| !**o).count();
        let max = (!acceptor.variadic).then_some(acceptor.optional.len());
        let invoked = if counts.unpack {
            max.map_or(min, |m| m.max(min))
        } else {
            counts.positional
        };
        if invoked >= min {
            continue;
        }
        let s = if invoked == 1 { "" } else { "s" };
        let method = String::from_utf8_lossy(method.value);
        let subject = format!(
            "{kind} class {display} constructor invoked with {invoked} parameter{s} in {display}::{method}()"
        );
        let message = match max {
            Some(m) if m == min => format!("{subject}, {min} required."),
            None => format!("{subject}, at least {min} required."),
            Some(m) => format!("{subject}, {min}-{m} required."),
        };
        c.report(call, "arguments.count", message, None);
    }
}
