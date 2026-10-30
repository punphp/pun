//! `lint/laravel/no-controller-middleware`, ported from Mago 1.57.0's `middleware-in-routes` (mago-linter
//! src/rule/best_practices/middleware_in_routes.rs).
//!
//! `$this->middleware(...)` in a class whose name ends in `Controller` (as written, in any part of the class but a
//! nested class-like) is reported: middleware belongs in the routes file. The innermost class-like decides, so an
//! anonymous class, an interface, a trait or an enum inside a controller is no controller. There is no fix.

use mago_syntax::cst::{Expression, Node, NodeKind, Variable};

use super::{is_name, member_name};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoControllerMiddleware;

impl Rule for NoControllerMiddleware {
    fn id(&self) -> &'static str {
        "lint/laravel/no-controller-middleware"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodCall(call) = node else { return };
        // `is_within_controller`: the innermost class-like scope is a class named `...Controller`
        let scope = ancestors.iter().rev().find_map(|a| match a {
            Node::Class(c) => Some(Some(c.name.value)),
            Node::AnonymousClass(_) | Node::Interface(_) | Node::Trait(_) | Node::Enum(_) => {
                Some(None)
            }
            _ => None,
        });
        if !scope
            .flatten()
            .is_some_and(|name| name.ends_with(b"Controller"))
        {
            return;
        }
        let is_this = matches!(call.object, Expression::Variable(Variable::Direct(v)) if v.name.eq_ignore_ascii_case(b"$this"));
        if !is_this || !member_name(&call.method).is_some_and(|n| is_name(n, "middleware")) {
            return;
        }
        out.push(Report {
            span: node_span(call),
            message: "Avoid applying middlewares in controllers.".to_string(),
            help: Some("Move the middleware to the routes file.".to_string()),
            fix: None,
        });
    }
}
