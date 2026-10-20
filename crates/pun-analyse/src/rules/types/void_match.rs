//! UsageOfVoidMatchExpressionRule (`match.void`): the result of a `match` whose arms all call void functions is
//! used (the match is not a statement's top-level expression).

use mago_codex::ttype::atomic::TAtomic;
use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::super::Checker;
use super::flow::unparen;

pub const IDENTIFIERS: &[&str] = &["match.void"];

/// The arm expressions, whose types the rule reads.
pub(super) fn typed(m: &Match, spans: &mut Vec<mago_span::Span>) {
    spans.extend(m.arms.iter().map(|a| a.expression().span()));
}

#[derive(PartialEq)]
enum Kind {
    Void,
    Never,
    Other,
}

/// `getKeepVoidType()` of an expression, as far as it decides `void`: a call typed void, a match of such arms (a
/// `never` arm adds nothing to the union).
fn kind(c: &Checker, e: &Expression) -> Kind {
    let e = unparen(e);
    if let Expression::Match(m) = e {
        return arms(c, m);
    }
    let Some(t) = c.type_of(e) else {
        return Kind::Other;
    };
    if t.types.iter().all(|a| matches!(a, TAtomic::Never)) {
        return Kind::Never;
    }
    // (a first-class callable is a partial application in Mago's syntax, not a call)
    if matches!(e, Expression::Call(_)) && t.types.iter().all(|a| matches!(a, TAtomic::Void | TAtomic::Never)) {
        Kind::Void
    } else {
        Kind::Other
    }
}

fn arms(c: &Checker, m: &Match) -> Kind {
    let mut result = Kind::Never;
    for a in m.arms.iter() {
        match kind(c, a.expression()) {
            Kind::Other => return Kind::Other,
            Kind::Void => result = Kind::Void,
            Kind::Never => {}
        }
    }
    result
}

pub(in super::super) fn void_match(c: &mut Checker, m: &Match) {
    if c.level < 2 || c.is_top_level(m.span()) {
        return;
    }
    if arms(c, m) == Kind::Void {
        c.report(m, "match.void", "Result of match expression (void) is used.".to_string(), None);
    }
}
