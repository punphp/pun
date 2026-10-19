//! `lint/classes/no-explicit-constructor-call`, ported from phpstan-strict-rules 2.0.12's
//! `IllegalConstructorMethodCallRule` (identifier `constructor.call`). `$object->__construct(...)` and
//! `$object?->__construct(...)` (PHPStan runs its `MethodCall` rules on a nullsafe call too), in any case, are
//! reported; a first-class callable (`$object->__construct(...)` with `...`) is another node type upstream and is not.

use mago_syntax::cst::{ClassLikeMemberSelector, Node, NodeKind};

use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoExplicitConstructorCall;

impl Rule for NoExplicitConstructorCall {
    fn id(&self) -> &'static str {
        "lint/classes/no-explicit-constructor-call"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall, NodeKind::NullSafeMethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let method = match node {
            Node::NullSafeMethodCall(call) => &call.method,
            _ => {
                let Node::MethodCall(c) = node else { return };
                &c.method
            }
        };
        let ClassLikeMemberSelector::Identifier(name) = method else {
            return;
        };
        if !name.value.eq_ignore_ascii_case(b"__construct") {
            return;
        }
        out.push(Report {
            span: node_span(&node),
            message: "Call to __construct() on an existing object is not allowed.".to_string(),
            help: None,
            fix: None,
        });
    }
}
