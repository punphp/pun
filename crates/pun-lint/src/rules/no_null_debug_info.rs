//! `lint/php-8.5/no-null-debug-info`, ported from Rector 2.6.1's `NullDebugInfoReturnRector`
//! (rules/Php85/Rector/ClassMethod).
//!
//! PHP 8.5 deprecates returning `null` from `__debugInfo()`, so `return null;` and a bare `return;` in that method
//! return `[]`. Returns inside nested functions, closures and classes are theirs, and stay.

use mago_syntax::cst::{MethodBody, Node, NodeKind, Return};
use pun_problem::{Edit, Fix, Safety, Span};

use super::rector_code_quality::is_constant;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoNullDebugInfo;

impl Rule for NoNullDebugInfo {
    fn id(&self) -> &'static str {
        "lint/php-8.5/no-null-debug-info"
    }

    fn min_php_version(&self) -> u32 {
        // PhpVersionFeature::DEPRECATED_NULL_DEBUG_INFO_RETURN
        80500
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Method(method) = node else { return };
        if !method.name.value.eq_ignore_ascii_case(b"__debugInfo") {
            return;
        }
        let MethodBody::Concrete(body) = &method.body else {
            return;
        };
        let mut edits = Vec::new();
        returns(Node::Block(body), &mut |r| match r.value {
            None => {
                let end = node_span(&r.r#return).end;
                edits.push(Edit::new(Span::new(end, end), " []"));
            }
            Some(value) if is_constant(value, "null") => {
                edits.push(Edit::new(node_span(unparenthesize(value)), "[]"));
            }
            Some(_) => {}
        });
        let Some(first) = edits.first() else { return };
        out.push(Report {
            span: first.span,
            message: "Return an array instead of null from `__debugInfo()`.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// Every `return` under `node`, outside nested classes, functions and closures.
fn returns<'ast, 'arena>(node: Node<'ast, 'arena>, f: &mut impl FnMut(&'ast Return<'arena>)) {
    node.visit_children(|child| match child {
        Node::Class(_) | Node::AnonymousClass(_) | Node::Function(_) | Node::Closure(_) => {}
        Node::Return(r) => {
            f(r);
            returns(child, f);
        }
        _ => returns(child, f),
    });
}
