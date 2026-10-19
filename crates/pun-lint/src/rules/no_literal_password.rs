//! `lint/security/no-literal-password`, ported from Mago 1.57.0's linter rule `no-literal-password`
//! (rule/security/no_literal_password.rs).
//!
//! A sensitive name (a password, token, secret or API key, by name) given a non-empty string literal or an integer
//! literal is reported on the name: in an assignment, an array `key => value`, a constant, a class constant, a
//! property default, a parameter default and a named argument.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::{ArrayElement, Expression, Literal, Node, NodeKind};

use super::lint_b_security::{get_password, is_password, is_password_literal};
use crate::syntax::span;
use crate::{Context, Report, Rule};

pub struct NoLiteralPassword;

impl Rule for NoLiteralPassword {
    fn id(&self) -> &'static str {
        "lint/security/no-literal-password"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::Assignment,
            NodeKind::ArrayElement,
            NodeKind::ConstantItem,
            NodeKind::ClassLikeConstantItem,
            NodeKind::PropertyConcreteItem,
            NodeKind::FunctionLikeParameter,
            NodeKind::NamedArgument,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (name, value) = match node {
            Node::Assignment(a) => {
                let Some(name) = get_password(a.lhs) else {
                    return;
                };
                (name, a.rhs)
            }
            Node::ArrayElement(ArrayElement::KeyValue(kv)) => {
                let Expression::Literal(Literal::String(key)) = kv.key else {
                    return;
                };
                if !is_password_literal(key) {
                    return;
                }
                (kv.key.span(), kv.value)
            }
            Node::ConstantItem(c) if is_password(c.name.value) => (c.name.span, c.value),
            Node::ClassLikeConstantItem(c) if is_password(c.name.value) => (c.name.span, c.value),
            Node::PropertyConcreteItem(p) if is_password(&p.variable.name[1..]) => {
                (p.variable.span, p.value)
            }
            Node::FunctionLikeParameter(p) => {
                let Some(default) = p.default_value.as_ref() else {
                    return;
                };
                if !is_password(&p.variable.name[1..]) {
                    return;
                }
                (p.variable.span, default.value)
            }
            Node::NamedArgument(a) if is_password(a.name.value) => (a.name.span, a.value),
            _ => return,
        };
        check(name, value, out);
    }
}

fn check(name: Span, value: &Expression<'_>, out: &mut Vec<Report>) {
    let literal = match value {
        Expression::Literal(Literal::String(s)) => s.raw.len() > 2,
        Expression::Literal(Literal::Integer(_)) => true,
        _ => false,
    };
    if !literal {
        return;
    }
    out.push(Report {
        span: span(name),
        message: "Literal passwords or sensitive data should not be stored in code.".to_string(),
        help: Some(
            "Use environment variables or secure configuration management instead.".to_string(),
        ),
        fix: None,
    });
}
