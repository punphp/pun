//! `lint/functions/no-void-reference-return`, ported from Mago 1.57.0's linter rule `no-void-reference-return`
//! (rule/deprecation/no_void_reference_return.rs).
//!
//! From PHP 8.2, returning by reference (`function &f(): void`) from a `void` function, method, closure or arrow
//! function, or from a `set` property hook, is deprecated. The `&` is reported.

use mago_span::Span;
use mago_syntax::cst::{FunctionLikeReturnTypeHint, Hint, Node, NodeKind};

use crate::syntax::span;
use crate::{Context, Report, Rule};

pub struct NoVoidReferenceReturn;

impl Rule for NoVoidReferenceReturn {
    fn id(&self) -> &'static str {
        "lint/functions/no-void-reference-return"
    }

    fn min_php_version(&self) -> u32 {
        80200
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::Function,
            NodeKind::Method,
            NodeKind::Closure,
            NodeKind::ArrowFunction,
            NodeKind::PropertyHook,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (kind, ampersand, hint): (&str, Option<Span>, Option<&FunctionLikeReturnTypeHint>) =
            match node {
                Node::Function(f) => ("function", f.ampersand, f.return_type_hint.as_ref()),
                Node::Method(m) => ("method", m.ampersand, m.return_type_hint.as_ref()),
                Node::Closure(c) => ("closure", c.ampersand, c.return_type_hint.as_ref()),
                Node::ArrowFunction(a) => {
                    ("arrow function", a.ampersand, a.return_type_hint.as_ref())
                }
                _ => {
                    let Node::PropertyHook(h) = node else { return };
                    if h.name.value != b"set" {
                        return;
                    }
                    let Some(ampersand) = h.ampersand else {
                        return;
                    };
                    report(out, ampersand, "Returning by reference from a set property hook is deprecated since PHP 8.0.".to_string());
                    return;
                }
            };
        let Some(ampersand) = ampersand else { return };
        if !hint.is_some_and(|h| matches!(h.hint, Hint::Void(_))) {
            return;
        }
        report(
            out,
            ampersand,
            format!("Returning by reference from a void {kind} is deprecated since PHP 8.0."),
        );
    }
}

fn report(out: &mut Vec<Report>, ampersand: Span, message: String) {
    out.push(Report {
        span: span(ampersand),
        message,
        help: Some(
            "Consider removing the `&` to comply with PHP 8.0 standards and avoid future issues."
                .to_string(),
        ),
        fix: None,
    });
}
