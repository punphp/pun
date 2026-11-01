//! `lint/pest/no-static-test-closure`, ported from pest-plugin-rector's `RemoveStaticTestClosureRector`: a `static`
//! closure given to `test()`, `it()`, `describe()`, `beforeEach()` or `afterEach()` loses `static` when it uses
//! `$this`, which Pest binds to the test case. `beforeAll()` and `afterAll()` run without one and keep theirs.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::{function_name, pest_callback, requires_instance_binding};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoStaticTestClosure;

impl Rule for NoStaticTestClosure {
    fn id(&self) -> &'static str {
        "lint/pest/no-static-test-closure"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(call) = node else {
            return;
        };
        let Some(name) = function_name(call) else {
            return;
        };
        if name == b"beforeAll" || name == b"afterAll" {
            return;
        }
        let Some((callback, keyword, r#static)) = pest_callback(call) else {
            return;
        };
        let Some(r#static) = r#static else {
            return;
        };
        if !requires_instance_binding(callback) {
            return;
        }
        out.push(Report::fixed(
            node_span(call),
            "Remove `static` from a Pest callback that uses `$this`.",
            Safety::Safe,
            vec![Edit::new(
                Span::new(node_span(&r#static).start, node_span(&keyword).start),
                "",
            )],
        ));
    }
}
