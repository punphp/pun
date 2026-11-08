//! RegularExpressionPatternRule (`regexp.pattern`): the constant patterns of `preg_*` calls must compile; from level 5
//! RegularExpressionQuotingRule (`argument.invalidPregQuote`): a `preg_quote()` in a concatenated pattern escapes its
//! delimiter.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;

use super::super::Checker;

pub(crate) const IDENTIFIERS: &[&str] = &["argument.invalidPregQuote"];
use super::flow::unparen;

/// The function's lowercase name as written (`(string) $functionCall->name`), for a call by name.
fn name(c: &Checker, call: &FunctionCall) -> Option<String> {
    let Expression::Identifier(id) = call.function else {
        return None;
    };
    Some(
        c.written_or_resolved(id)
            .trim_start_matches('\\')
            .to_ascii_lowercase(),
    )
}

fn first_argument<'a, 'arena>(call: &'a FunctionCall<'arena>) -> Option<&'a Expression<'arena>> {
    call.argument_list.arguments.first().map(|a| a.value())
}

/// The expressions whose type [`patterns`] reads: the pattern, and the parts of a concatenated one.
pub(super) fn typed(call: &FunctionCall, out: &mut Vec<Span>) {
    let Expression::Identifier(id) = call.function else {
        return;
    };
    if !id
        .value()
        .to_ascii_lowercase()
        .rsplit(|b| *b == b'\\')
        .next()
        .is_some_and(|n| n.starts_with(b"preg_"))
    {
        return;
    }
    for (i, a) in call.argument_list.arguments.iter().enumerate() {
        out.push(a.value().span());
        if i == 0 {
            concat_leaves(a.value(), out);
            for item in items(a.value()) {
                out.push(item.span());
                concat_leaves(item, out);
            }
        }
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
    if c.level >= 5 {
        quoting(c, call, &name);
    }
    if !name.starts_with("preg_") || call.argument_list.arguments.is_empty() {
        return;
    }
    // (the arguments are not empty)
    let Some(pattern) = first_argument(call) else {
        return;
    };
    let mut patterns: Vec<Vec<u8>> = Vec::new();
    let atoms: Vec<TAtomic> = c
        .type_of(pattern)
        .map(|t| t.types.to_vec())
        .unwrap_or_default();
    if matches!(
        name.as_str(),
        "preg_match"
            | "preg_match_all"
            | "preg_split"
            | "preg_grep"
            | "preg_replace"
            | "preg_replace_callback"
            | "preg_filter"
    ) {
        let strings = if matches!(unparen(pattern), Expression::Binary(b) if matches!(b.operator, BinaryOperator::StringConcat(_)))
        {
            concat(c, pattern)
        } else {
            constant_strings(c, pattern)
        };
        patterns.extend(strings);
    }
    if matches!(
        name.as_str(),
        "preg_replace" | "preg_replace_callback" | "preg_filter"
    ) {
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
                patterns.extend(
                    items
                        .keys()
                        .filter_map(ArrayKey::get_string)
                        .map(<[u8]>::to_vec),
                );
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
        TArray::List(l) if l.element_type.is_never() => l
            .known_elements
            .iter()
            .flat_map(|k| k.values().map(|(_, v)| v))
            .collect(),
        TArray::Keyed(k) if k.parameters.is_none() => k
            .known_items
            .iter()
            .flat_map(|k| k.values().map(|(_, v)| v))
            .collect(),
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
    if strings.len() == t.types.len() {
        strings
    } else {
        Vec::new()
    }
}

/// `RegexExpressionHelper::resolvePatternConcat()`: a `preg_quote()` call stands for `(?:.*)`.
fn concat(c: &Checker, e: &Expression) -> Vec<Vec<u8>> {
    match unparen(e) {
        Expression::Call(Call::Function(f)) if matches!(f.function, Expression::Identifier(id) if id.value().rsplit(|b| *b == b'\\').next().is_some_and(|n| n.eq_ignore_ascii_case(b"preg_quote"))) =>
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

/// `ArgumentsNormalizer::reorderFuncArguments()`'s argument for parameter `index` named `parameter`.
// ponytail: positional or named, not unpacked arguments
pub(in crate::rules) fn argument<'a, 'arena>(
    call: &'a FunctionCall<'arena>,
    index: usize,
    parameter: &str,
) -> Option<&'a Expression<'arena>> {
    let args = &call.argument_list.arguments;
    args.iter()
        .find(|a| matches!(a, Argument::Named(n) if n.name.value == parameter.as_bytes()))
        .or_else(|| {
            args.get(index)
                .filter(|a| matches!(a, Argument::Positional(p) if p.ellipsis.is_none()))
        })
        .map(|a| a.value())
}

/// RegularExpressionQuotingRule.
fn quoting(c: &mut Checker, call: &FunctionCall, name: &str) {
    if !matches!(
        name,
        "preg_match"
            | "preg_match_all"
            | "preg_filter"
            | "preg_grep"
            | "preg_replace"
            | "preg_replace_callback"
            | "preg_split"
    ) {
        return;
    }
    let Some(pattern) = argument(call, 0, "pattern").map(unparen) else {
        return;
    };
    // preg_replace, preg_replace_callback and preg_filter also take an array of patterns, each one in its own right
    // (another function's array pattern is no concatenation, so it is skipped below)
    let patterns: Vec<&Expression> =
        if matches!(pattern, Expression::Array(_) | Expression::LegacyArray(_))
            && matches!(
                name,
                "preg_filter" | "preg_replace" | "preg_replace_callback"
            )
        {
            items(pattern)
        } else {
            vec![pattern]
        };
    for pattern in patterns {
        if let Expression::Binary(concat) = unparen(pattern)
            && matches!(concat.operator, BinaryOperator::StringConcat(_))
        {
            quote_delimiters(c, concat);
        }
    }
}

/// The values of an array literal's items (an unpacked one too, as `$item->value`).
fn items<'a, 'arena>(e: &'a Expression<'arena>) -> Vec<&'a Expression<'arena>> {
    let elements = match unparen(e) {
        Expression::Array(Array { elements, .. })
        | Expression::LegacyArray(LegacyArray { elements, .. }) => elements,
        _ => return vec![],
    };
    elements
        .iter()
        .filter_map(ArrayElement::get_value)
        .collect()
}

fn quote_delimiters(c: &mut Checker, concat: &Binary) {
    // RegexExpressionHelper::getPatternDelimiters(): the first character of the leftmost operand's constant strings
    let mut left = concat.lhs;
    while let Expression::Binary(b) = unparen(left)
        && matches!(b.operator, BinaryOperator::StringConcat(_))
    {
        left = b.lhs;
    }
    let delimiters: Vec<char> = constant_strings(c, left)
        .iter()
        .filter_map(|s| String::from_utf8_lossy(s).trim_start().chars().next())
        .collect();
    if !delimiters.is_empty() {
        validate(c, concat, &delimiters);
    }
}

/// `validateQuoteDelimiters()`.
fn validate(c: &mut Checker, concat: &Binary, delimiters: &[char]) {
    for side in [concat.lhs, concat.rhs] {
        match unparen(side) {
            Expression::Call(Call::Function(f)) if matches!(f.function, Expression::Identifier(id) if id.value().strip_prefix(b"\\").unwrap_or(id.value()).eq_ignore_ascii_case(b"preg_quote")) => {
                if let Some(message) = pregquote(c, f, delimiters) {
                    c.report(f.span(), "argument.invalidPregQuote", message, None);
                }
            }
            Expression::Binary(b) if matches!(b.operator, BinaryOperator::StringConcat(_)) => {
                validate(c, b, delimiters);
            }
            _ => {}
        }
    }
}

/// Delimiters `preg_quote()` always escapes.
fn default_escaped(d: char) -> bool {
    ".\\+*?[^]$(){}=!<>|:-#".contains(d)
}

/// `validatePregQuote()`.
fn pregquote(c: &Checker, call: &FunctionCall, delimiters: &[char]) -> Option<String> {
    let delimiters: Vec<char> = delimiters
        .iter()
        .copied()
        .filter(|d| !default_escaped(*d))
        .collect();
    if delimiters.is_empty() {
        return None;
    }
    let Some(quote) = argument(call, 1, "delimiter") else {
        return Some(match delimiters.as_slice() {
            [d] => format!("Call to preg_quote() is missing delimiter {d} to be effective."),
            _ => "Call to preg_quote() is missing delimiter parameter to be effective.".to_string(),
        });
    };
    constant_strings(c, quote)
        .iter()
        .map(|q| String::from_utf8_lossy(q).into_owned())
        .find_map(|q| {
            // a delimiter of more than one character is never escaped by default
            let escaped = q.chars().count() == 1 && q.chars().all(default_escaped);
            (!escaped && !delimiters.iter().any(|d| q == d.to_string())).then(|| {
                match delimiters.as_slice() {
                    [d] => format!(
                        "Call to preg_quote() uses invalid delimiter {q} while pattern uses {d}."
                    ),
                    _ => format!("Call to preg_quote() uses invalid delimiter {q}."),
                }
            })
        })
}
