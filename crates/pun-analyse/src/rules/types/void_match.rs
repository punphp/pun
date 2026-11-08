//! UsageOfVoidMatchExpressionRule (`match.void`): the result of a `match` whose arms all call void functions is
//! used (the match is not a statement's top-level expression).

use mago_codex::ttype::union::TUnion;
use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::super::Checker;

pub(super) const IDENTIFIERS: &[&str] = &["match.void"];

/// The match result, whose arm union retains void independently of ordinary call values.
pub(super) fn typed(m: &Match, spans: &mut Vec<mago_span::Span>) {
    spans.push(m.span());
}

pub(in super::super) fn void_match(c: &mut Checker, m: &Match) {
    if c.level < 2 || c.is_top_level(m.span()) {
        return;
    }
    if c.keep_void_type_at(m.span()).is_some_and(TUnion::is_void) {
        c.report(m, "match.void", "Result of match expression (void) is used.".to_string(), None);
    }
}
