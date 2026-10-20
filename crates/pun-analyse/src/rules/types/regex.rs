//! RegularExpressionPatternRule (`regexp.pattern`): the constant patterns of `preg_*` calls must compile.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;

use super::super::Checker;
use super::flow::unparen;

/// The function's lowercase name as written (`(string) $functionCall->name`), for a call by name.
fn name(c: &Checker, call: &FunctionCall) -> Option<String> {
    let Expression::Identifier(id) = call.function else {
        return None;
    };
    Some(c.written_or_resolved(id).trim_start_matches('\\').to_ascii_lowercase())
}

fn first_argument<'a, 'arena>(call: &'a FunctionCall<'arena>) -> Option<&'a Expression<'arena>> {
    call.argument_list.arguments.first().map(|a| a.value())
}

/// The expressions whose type [`patterns`] reads: the pattern, and the parts of a concatenated one.
pub(super) fn typed(call: &FunctionCall, out: &mut Vec<Span>) {
    let Expression::Identifier(id) = call.function else {
        return;
    };
    if !id.value().to_ascii_lowercase().rsplit(|b| *b == b'\\').next().is_some_and(|n| n.starts_with(b"preg_")) {
        return;
    }
    if let Some(p) = first_argument(call) {
        out.push(p.span());
        concat_leaves(p, out);
    }
}

fn concat_leaves(e: &Expression, out: &mut Vec<Span>) {
    if let Expression::Binary(b) = unparen(e)
        && matches!(b.operator, BinaryOperator::StringConcat(_))
    {
        for side in [b.lhs, b.rhs] {
            out.push(side.span());
            concat_leaves(side, out);
        }
    }
}

pub(in super::super) fn function_call(c: &mut Checker, call: &FunctionCall) {
    let Some(name) = name(c, call) else {
        return;
    };
    if !name.starts_with("preg_") || call.argument_list.arguments.is_empty() {
        return;
    }
    // (the arguments are not empty)
    let Some(pattern) = first_argument(call) else { return };
    let mut patterns: Vec<Vec<u8>> = Vec::new();
    let atoms: Vec<TAtomic> = c.type_of(pattern).map(|t| t.types.to_vec()).unwrap_or_default();
    if matches!(
        name.as_str(),
        "preg_match" | "preg_match_all" | "preg_split" | "preg_grep" | "preg_replace" | "preg_replace_callback" | "preg_filter"
    ) {
        let strings = if matches!(unparen(pattern), Expression::Binary(b) if matches!(b.operator, BinaryOperator::StringConcat(_)))
        {
            concat(c, pattern)
        } else {
            constant_strings(c, pattern)
        };
        patterns.extend(strings);
    }
    if matches!(name.as_str(), "preg_replace" | "preg_replace_callback" | "preg_filter") {
        for a in &atoms {
            if let TAtomic::Array(arr) = a {
                for v in values(arr) {
                    patterns.extend(atom_strings(v));
                }
            }
        }
    }
    if name == "preg_replace_callback_array" {
        for a in &atoms {
            if let TAtomic::Array(TArray::Keyed(k)) = a
                && k.parameters.is_none()
                && let Some(items) = &k.known_items
            {
                patterns.extend(items.keys().filter_map(ArrayKey::get_string).map(<[u8]>::to_vec));
            }
        }
    }
    for p in patterns {
        let p = String::from_utf8_lossy(&p).into_owned();
        if let Err(message) = pun_phpstan::php::preg(&p) {
            c.report(
                call.span(),
                "regexp.pattern",
                format!("Regex pattern is invalid: {message}"),
                None,
            );
        }
    }
}

/// The values of a constant array (`getConstantArrays()`).
fn values(a: &TArray) -> Vec<&mago_codex::ttype::union::TUnion> {
    match a {
        TArray::List(l) if l.element_type.is_never() => l.known_elements.iter().flat_map(|k| k.values().map(|(_, v)| v)).collect(),
        TArray::Keyed(k) if k.parameters.is_none() => k.known_items.iter().flat_map(|k| k.values().map(|(_, v)| v)).collect(),
        _ => Vec::new(),
    }
}

fn atom_strings(t: &mago_codex::ttype::union::TUnion) -> Vec<Vec<u8>> {
    t.types
        .iter()
        .filter_map(|a| a.get_literal_string_value().map(<[u8]>::to_vec))
        .collect()
}

/// `getConstantStrings()` of an expression's type.
fn constant_strings(c: &Checker, e: &Expression) -> Vec<Vec<u8>> {
    if let Expression::Literal(Literal::String(s)) = unparen(e) {
        return s.value.map(|v| vec![v.to_vec()]).unwrap_or_default();
    }
    let Some(t) = c.type_of(e) else {
        return Vec::new();
    };
    let strings = atom_strings(t);
    if strings.len() == t.types.len() { strings } else { Vec::new() }
}

/// `RegexExpressionHelper::resolvePatternConcat()`: a `preg_quote()` call stands for `(?:.*)`.
fn concat(c: &Checker, e: &Expression) -> Vec<Vec<u8>> {
    match unparen(e) {
        Expression::Call(Call::Function(f))
            if matches!(f.function, Expression::Identifier(id) if id.value().rsplit(|b| *b == b'\\').next().is_some_and(|n| n.eq_ignore_ascii_case(b"preg_quote"))) =>
        {
            vec![b"(?:.*)".to_vec()]
        }
        Expression::Binary(b) if matches!(b.operator, BinaryOperator::StringConcat(_)) => {
            let (left, right) = (concat(c, b.lhs), concat(c, b.rhs));
            let mut out = Vec::new();
            for l in &left {
                for r in &right {
                    out.push([l.as_slice(), r.as_slice()].concat());
                }
            }
            out
        }
        e => constant_strings(c, e),
    }
}
