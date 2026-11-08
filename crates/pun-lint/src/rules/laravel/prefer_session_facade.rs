//! `SessionVariableToSessionFacadeRector` from rector-laravel 2.7.3.

use mago_syntax::cst::{Expression, Node, NodeKind, Variable};

use super::{call, in_string, is_by_ref_argument, is_written, replace};
use crate::rules::rector_code_quality::text;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferSessionFacade;

impl Rule for PreferSessionFacade {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-session-facade"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::IssetConstruct,
            NodeKind::Unset,
            NodeKind::ArrayAccess,
            NodeKind::Assignment,
            NodeKind::FunctionCall,
            NodeKind::Variable,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (method, args) = match node {
            Node::Assignment(a) if a.operator.is_assign() => {
                let Some(key) = session_key(a.lhs) else {
                    return;
                };
                // PhpParser has a separate AssignRef node; it is not an ordinary assignment.
                if a.rhs.is_reference() {
                    return;
                }
                ("put", vec![text(ctx, key), text(ctx, a.rhs)])
            }
            Node::ArrayAccess(a) if is_session(a.array) => {
                if is_written(node_span(a), ancestors)
                    || is_by_ref_argument(ctx, node_span(a), ancestors)
                    || in_string(ancestors)
                {
                    return;
                }
                ("get", vec![text(ctx, a.index)])
            }
            Node::FunctionCall(f) => {
                // The upstream replacement table compares getName() case-sensitively.
                let method = match text(ctx, f.function).trim_start_matches('\\') {
                    "session_regenerate_id" => "regenerate",
                    "session_unset" => "flush",
                    "session_destroy" => "destroy",
                    "session_start" => "start",
                    _ => return,
                };
                (method, vec![])
            }
            Node::IssetConstruct(i) => {
                let first = i.values.iter().next().unwrap();
                let Some(key) = session_key(first) else {
                    return;
                };
                ("has", vec![text(ctx, key)])
            }
            Node::Unset(u) => {
                let first = u.values.iter().next().unwrap();
                let Some(key) = session_key(first) else {
                    return;
                };
                ("forget", vec![text(ctx, key)])
            }
            Node::Variable(Variable::Direct(v)) if v.name.eq_ignore_ascii_case(b"$_SESSION") => {
                let span = node_span(v);
                if is_written(span, ancestors)
                    || is_by_ref_argument(ctx, span, ancestors)
                    || in_string(ancestors)
                    || ancestors.iter().any(|a| match a {
                        Node::ArrayAccess(a) => node_span(a.array) == span,
                        Node::ArrayAppend(a) => node_span(a.array) == span,
                        _ => false,
                    })
                {
                    return;
                }
                ("all", vec![])
            }
            _ => return,
        };
        let mut code = call(
            &format!("\\Illuminate\\Support\\Facades\\Session::{method}"),
            &args,
        );
        if matches!(node, Node::Unset(_)) {
            code.push(';');
        }
        out.push(replace(
            node_span(&node),
            format!("Use `Session::{method}()` for session access."),
            code,
        ));
    }
}

fn session_key<'a>(expr: &'a Expression<'a>) -> Option<&'a Expression<'a>> {
    match unparenthesize(expr) {
        Expression::ArrayAccess(a) if is_session(a.array) => Some(a.index),
        _ => None,
    }
}

fn is_session(expr: &Expression<'_>) -> bool {
    matches!(unparenthesize(expr), Expression::Variable(Variable::Direct(v)) if v.name.eq_ignore_ascii_case(b"$_SESSION"))
}
