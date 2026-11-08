//! `lint/pest/no-redundant-type-expectation`, ported from pest-plugin-rector's
//! `RemoveRedundantLiteralTypeExpectationRector`: in `expect('pest')->toBeString()->toStartWith('p')`, the type
//! matcher right on `expect()` (or on its `->not`) always passes for the literal it checks, so it goes, as long as a
//! later matcher keeps the chain meaningful.

use mago_syntax::cst::{
    CompositeString, Expression, Literal, MethodCall, Node, NodeKind, StringPart,
};
use pun_problem::{Edit, Safety};

use super::{func_call, has_comments, identifier, is_expect_chain, method_call, property_fetch};
use crate::rules::phpunit::expr_text;
use crate::rules::rector_code_quality::is_constant;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoRedundantTypeExpectation;

/// `SUBJECT_TRANSFORMING_METHODS`: a matcher after one of these checks another value.
const SUBJECT_TRANSFORMING_METHODS: [&[u8]; 7] = [
    b"and",
    b"json",
    b"each",
    b"match",
    b"sequence",
    b"unless",
    b"when",
];

impl Rule for NoRedundantTypeExpectation {
    fn id(&self) -> &'static str {
        "lint/pest/no-redundant-type-expectation"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        // `removeRedundantMatchersFromChain` walks the chain down from its outermost call; only the call right on
        // `expect()` can be redundant, so the pair it removes is this call (`$current`) and its object (`$inner`).
        let Node::MethodCall(current) = node else {
            return;
        };
        let Some(inner) = method_call(current.object) else {
            return;
        };
        if !is_expect_chain(current) {
            return;
        }
        let Some(negated) = redundant(inner) else {
            return;
        };
        let Some(name) = identifier(&current.method) else {
            return;
        };
        if SUBJECT_TRANSFORMING_METHODS.contains(&name) {
            return;
        }
        // `nodeOrParentsHaveComments`: Rector sets no `parent`, so only the node's own comments count. php-parser
        // gives a comment to the outermost node that starts after it.
        let current_span = node_span(current);
        let owns = |start: usize| {
            has_comments(ctx, start)
                && !ancestors.iter().any(|a| {
                    node_span(a).start == start
                        && !matches!(
                            a.kind(),
                            NodeKind::Expression
                                | NodeKind::Call
                                | NodeKind::Access
                                | NodeKind::Argument
                                | NodeKind::ArrayElement
                                | NodeKind::Statement
                        )
                })
        };
        let inner_span = node_span(inner);
        if owns(current_span.start)
            || (inner_span.start != current_span.start && has_comments(ctx, inner_span.start))
            || hash_comment_within(ctx, inner_span.start, inner_span.end)
            || inline_comment_between(ctx.source, inner_span.start, current_span.end)
        {
            return;
        }
        // `unwrapRedundantExpectation`: `expect()`, under the `->not` when the matcher was negated.
        let mut var = inner.object;
        if negated && let Some(not) = property_fetch(var) {
            var = not.object;
        }
        out.push(Report::fixed(
            current_span,
            "Remove the type matcher: the literal always has that type.",
            Safety::Safe,
            vec![Edit::new(inner_span, expr_text(ctx, var))],
        ));
    }
}

/// `SemanticExpectationAnalyzer::analyzeLiteralTypeMatcher` and `isRedundant()`: `call` is a type matcher called
/// right on `expect(literal)` or its `->not`, whose result the literal decides and makes always pass. Gives whether
/// it was negated.
fn redundant(call: &MethodCall<'_>) -> Option<bool> {
    // `isDirectExpectMethod`: names compared as written.
    let negated = match property_fetch(call.object) {
        Some(not) if identifier(&not.property) == Some(b"not") => {
            direct_expect(not.object)?;
            true
        }
        _ => {
            direct_expect(call.object)?;
            false
        }
    };
    let category = match identifier(&call.method)? {
        b"toBeArray" => "array",
        b"toBeBool" => "bool",
        b"toBeCallable" => "callable",
        b"toBeFloat" => "float",
        b"toBeInt" => "int",
        b"toBeIterable" => "iterable",
        b"toBeNull" => "null",
        b"toBeNumeric" => "numeric",
        b"toBeObject" => "object",
        b"toBeScalar" => "scalar",
        b"toBeString" => "string",
        _ => return None,
    };
    // `PestChainAnalyzer::getExpectArgument`: the first argument of the root call.
    let root = super::root_func_call(call)?;
    let subject = unparenthesize(super::first_value(&root.argument_list)?);
    let matches = matches_literal(subject, category)?;
    (matches != negated).then_some(negated)
}

/// The object is a call of `expect` (written so, without a namespace other than a leading `\`).
fn direct_expect(object: &Expression<'_>) -> Option<()> {
    let call = func_call(object)?;
    (super::function_name(call)? == b"expect").then_some(())
}

/// `deterministicLiteralCategory` and `matchesLiteralCategory`: whether a literal of a known type is of `category`;
/// none for anything else.
fn matches_literal(expr: &Expression<'_>, category: &str) -> Option<bool> {
    Some(match expr {
        Expression::Literal(Literal::String(s)) => string_matches(s.value, category),
        Expression::CompositeString(CompositeString::Document(d))
            if d.parts.iter().all(|p| matches!(p, StringPart::Literal(_))) =>
        {
            let mut value = Vec::new();
            for part in &d.parts {
                if let StringPart::Literal(l) = part {
                    value.extend_from_slice(l.value.unwrap_or(l.raw));
                }
            }
            string_matches(Some(&value), category)
        }
        Expression::Literal(Literal::Integer(_)) => {
            matches!(category, "int" | "numeric" | "scalar")
        }
        Expression::Literal(Literal::Float(_)) => {
            matches!(category, "float" | "numeric" | "scalar")
        }
        Expression::Array(_) | Expression::LegacyArray(_) => {
            matches!(category, "array" | "iterable")
        }
        Expression::Closure(_) | Expression::ArrowFunction(_) => {
            matches!(category, "callable" | "object")
        }
        Expression::Instantiation(_) | Expression::AnonymousClass(_) => category == "object",
        Expression::Access(mago_syntax::cst::Access::ClassConstant(c)) => {
            // A class constant other than `::class` (compared as written) has no known type.
            match &c.constant {
                mago_syntax::cst::ClassLikeConstantSelector::Identifier(id)
                    if id.value == b"class" =>
                {
                    matches!(category, "string" | "scalar")
                }
                _ => return None,
            }
        }
        _ if is_constant(expr, "true") || is_constant(expr, "false") => {
            matches!(category, "bool" | "scalar")
        }
        _ if is_constant(expr, "null") => category == "null",
        _ => return None,
    })
}

/// A string literal: always a string and a scalar, numeric when PHP's `is_numeric()` says so.
fn string_matches(value: Option<&[u8]>, category: &str) -> bool {
    match category {
        "string" | "scalar" => true,
        "numeric" => value.is_some_and(is_numeric),
        _ => false,
    }
}

/// PHP's `is_numeric()` of a string: optional whitespace around a sign, digits with an optional fraction and an
/// exponent.
fn is_numeric(s: &[u8]) -> bool {
    let ws = |c: &u8| matches!(c, b' ' | b'\t' | b'\n' | b'\r' | b'\x0b' | b'\x0c');
    let start = s.iter().position(|c| !ws(c)).unwrap_or(s.len());
    let end = s.iter().rposition(|c| !ws(c)).map_or(start, |e| e + 1);
    let mut t = &s[start..end];
    if let [b'+' | b'-', rest @ ..] = t {
        t = rest;
    }
    let digits = |t: &mut &[u8]| {
        let n = t.iter().take_while(|c| c.is_ascii_digit()).count();
        *t = &t[n..];
        n
    };
    let mut count = digits(&mut t);
    if let [b'.', rest @ ..] = t {
        t = rest;
        count += digits(&mut t);
    }
    if count == 0 {
        return false;
    }
    if let [b'e' | b'E', rest @ ..] = t {
        t = rest;
        if let [b'+' | b'-', rest @ ..] = t {
            t = rest;
        }
        if digits(&mut t) == 0 {
            return false;
        }
    }
    t.is_empty()
}

/// `nodeOrDescendantsHaveComments`: a comment inside the call.
fn hash_comment_within(ctx: &Context<'_>, start: usize, end: usize) -> bool {
    super::super::php_parser::tokens(ctx)
        .iter()
        .any(|t| t.start >= start && t.end <= end && t.kind.is_comment())
}

/// `hasInlineCommentBetween`: `//` or `/*` in the code from `start` to `end`, cut with `mb_substr`, which counts the
/// byte offsets php-parser gives as characters.
fn inline_comment_between(source: &str, start: usize, end: usize) -> bool {
    let segment: String = source.chars().skip(start).take(end - start).collect();
    segment.contains("//") || segment.contains("/*")
}
