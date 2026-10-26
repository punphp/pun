//! `lint/strings/require-interpolation-style`, ported from Mago 1.57.0's `string-style`
//! (mago-linter src/rule/consistency/string_style.rs).
//!
//! With `style = "interpolation"` (the default), reports an outermost `.` chain made only of string literals and
//! interpolable expressions (variables, and property, nullsafe property, array and method accesses on one), with
//! at least one of each, and rewrites it as one double-quoted string with `{...}` parts, unless a single-quoted
//! literal holds a `\`, `$` or `"`. With `style = "concatenation"`, reports every interpolated double-quoted string
//! and rewrites it as literals and expressions joined with `.`. Both fixes are safe, as in Mago, and copy the text
//! of each part as it is.

use mago_span::HasSpan;
use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use super::mago_helpers::configure;
use crate::syntax::span;
use crate::{Context, Report, Rule};

#[derive(Default)]
pub struct RequireInterpolationStyle {
    /// `style = "concatenation"`.
    pub concatenation: bool,
}

impl Rule for RequireInterpolationStyle {
    fn id(&self) -> &'static str {
        "lint/strings/require-interpolation-style"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Binary, NodeKind::CompositeString])
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        configure(options, |key, value| {
            match (key, value.as_str()?) {
                ("style", "interpolation") => self.concatenation = false,
                ("style", "concatenation") => self.concatenation = true,
                _ => return None,
            }
            Some(())
        })
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        if self.concatenation {
            prefer_concatenation(node, out);
        } else {
            prefer_interpolation(node, ancestors, out);
        }
    }
}

fn edit(start: u32, end: u32, text: &str) -> Edit {
    Edit::new(pun_problem::Span::new(start as usize, end as usize), text)
}

fn prefer_interpolation(node: Node<'_, '_>, ancestors: &[Node<'_, '_>], out: &mut Vec<Report>) {
    let Node::Binary(binary) = node else { return };
    if !binary.operator.is_concatenation() {
        return;
    }
    // The grandparent (the parent is the `Expression` that wraps this node).
    if let Some(Node::Binary(parent)) = ancestors.len().checked_sub(2).map(|i| ancestors[i])
        && parent.operator.is_concatenation()
    {
        return;
    }
    let mut parts = Vec::new();
    if !collect(binary.lhs, &mut parts) || !collect(binary.rhs, &mut parts) {
        return;
    }
    let has_literal = parts.iter().any(|p| matches!(p, Part::Literal { .. }));
    let has_interpolable = parts.iter().any(|p| matches!(p, Part::Interpolable(_)));
    if !has_literal || !has_interpolable {
        return;
    }
    let fixable = parts
        .iter()
        .all(|p| !matches!(p, Part::Literal { safe: false, .. }));
    let fix = fixable.then(|| {
        let mut edits = Vec::new();
        match parts[0] {
            Part::Interpolable(s) => edits.push(edit(s.start.offset, s.start.offset, "\"{")),
            Part::Literal { span: s, .. } => {
                edits.push(edit(s.start.offset, s.start.offset + 1, "\""))
            }
        }
        match parts[parts.len() - 1] {
            Part::Interpolable(s) => edits.push(edit(s.end.offset, s.end.offset, "}\"")),
            Part::Literal { span: s, .. } => edits.push(edit(s.end.offset - 1, s.end.offset, "\"")),
        }
        for pair in parts.windows(2) {
            let (start, end, glue) = match (&pair[0], &pair[1]) {
                (Part::Literal { span: l, .. }, Part::Literal { span: r, .. }) => {
                    (l.end.offset - 1, r.start.offset + 1, "")
                }
                (Part::Literal { span: l, .. }, Part::Interpolable(r)) => {
                    (l.end.offset - 1, r.start.offset, "{")
                }
                (Part::Interpolable(l), Part::Literal { span: r, .. }) => {
                    (l.end.offset, r.start.offset + 1, "}")
                }
                (Part::Interpolable(l), Part::Interpolable(r)) => {
                    (l.end.offset, r.start.offset, "}{")
                }
            };
            edits.push(edit(start, end, glue));
        }
        Fix {
            safety: Safety::Safe,
            edits,
        }
    });
    out.push(Report {
        span: span(binary.span()),
        message: "String concatenation can be replaced with interpolation.".into(),
        help: Some(
            "Use a double-quoted string with `{$variable}` syntax instead of concatenation.".into(),
        ),
        fix,
    });
}

/// `ConcatPart`, without `Other`: a part of any other kind makes [`collect`] fail, as it makes Mago skip the chain.
enum Part {
    Literal { span: mago_span::Span, safe: bool },
    Interpolable(mago_span::Span),
}

/// `collect_concat_side`: false when a part is neither a string literal nor interpolable.
fn collect(expr: &Expression<'_>, parts: &mut Vec<Part>) -> bool {
    match expr {
        Expression::Binary(binary) if binary.operator.is_concatenation() => {
            return collect(binary.lhs, parts) && collect(binary.rhs, parts);
        }
        Expression::Literal(Literal::String(literal)) => parts.push(Part::Literal {
            span: literal.span,
            safe: matches!(literal.kind, LiteralStringKind::DoubleQuoted)
                || !literal
                    .raw
                    .iter()
                    .any(|&b| matches!(b, b'\\' | b'$' | b'"')),
        }),
        other if is_interpolable(other) => parts.push(Part::Interpolable(other.span())),
        _ => return false,
    }
    true
}

fn is_interpolable(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::Variable(_) => true,
        Expression::Access(Access::Property(access)) => is_interpolable(access.object),
        Expression::Access(Access::NullSafeProperty(access)) => is_interpolable(access.object),
        Expression::ArrayAccess(access) => is_interpolable(access.array),
        Expression::Call(Call::Method(call)) => is_interpolable(call.object),
        Expression::Call(Call::NullSafeMethod(call)) => is_interpolable(call.object),
        _ => false,
    }
}

fn prefer_concatenation(node: Node<'_, '_>, out: &mut Vec<Report>) {
    let Node::CompositeString(CompositeString::Interpolated(string)) = node else {
        return;
    };
    let parts: Vec<&StringPart<'_>> = string
        .parts
        .iter()
        .filter(|p| !matches!(p, StringPart::Literal(l) if l.raw.is_empty()))
        .collect();
    let fix = (!parts.is_empty()).then(|| {
        let (left, right) = (string.left_double_quote, string.right_double_quote);
        let mut edits = Vec::new();
        match parts[0] {
            StringPart::Literal(_) => {}
            StringPart::BracedExpression(braced) => {
                edits.push(edit(left.start.offset, braced.left_brace.end.offset, ""))
            }
            StringPart::Expression(_) => edits.push(edit(left.start.offset, left.end.offset, "")),
        }
        match parts[parts.len() - 1] {
            StringPart::Literal(_) => {}
            StringPart::BracedExpression(braced) => {
                edits.push(edit(braced.right_brace.start.offset, right.end.offset, ""))
            }
            StringPart::Expression(_) => edits.push(edit(right.start.offset, right.end.offset, "")),
        }
        for pair in parts.windows(2) {
            let start = match pair[0] {
                StringPart::Literal(l) => l.span.end.offset,
                StringPart::Expression(e) => e.span().end.offset,
                StringPart::BracedExpression(b) => b.right_brace.start.offset,
            };
            let end = match pair[1] {
                StringPart::Literal(l) => l.span.start.offset,
                StringPart::Expression(e) => e.span().start.offset,
                StringPart::BracedExpression(b) => b.left_brace.end.offset,
            };
            // Mago also has a `(Literal, Literal)` arm, but the lexer emits one literal part per run of text in a
            // double-quoted string, so two literal parts are never adjacent.
            let glue = match (pair[0], pair[1]) {
                (StringPart::Literal(_), _) => "\" . ",
                (_, StringPart::Literal(_)) => " . \"",
                _ => " . ",
            };
            edits.push(edit(start, end, glue));
        }
        Fix {
            safety: Safety::Safe,
            edits,
        }
    });
    out.push(Report {
        span: span(string.span()),
        message: "String interpolation can be replaced with concatenation.".into(),
        help: Some("Use the `.` concatenation operator with single-quoted strings instead of interpolation.".into()),
        fix,
    });
}
