//! `lint/pest/no-focused-test`, ported from pest-plugin-rector's `RemoveOnlyRector`: `->only()` on a Pest test
//! (`test()`, `it()`, `describe()`, `todo()` or a hook) is removed, so the whole suite runs.

use mago_syntax::cst::{MethodCall, Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::{identifier, report, root_func_call};
use crate::rules::rector_code_quality::named;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoFocusedTest;

impl Rule for NoFocusedTest {
    fn id(&self) -> &'static str {
        "lint/pest/no-focused-test"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodCall(call) = node else {
            return;
        };
        if identifier(&call.method) != Some(b"only") || !is_pest_test_chain(call) {
            return;
        }
        out.push(report(
            node_span(call),
            "Remove `->only()`, which runs this test alone.".to_string(),
            Safety::Unsafe,
            vec![Edit::new(
                Span::new(
                    node_span(unparenthesize(call.object)).end,
                    node_span(call).end,
                ),
                "",
            )],
        ));
    }
}

/// `PEST_TEST_FUNCTIONS`.
const PEST_TEST_FUNCTIONS: [&str; 8] = [
    "test",
    "it",
    "describe",
    "todo",
    "beforeEach",
    "afterEach",
    "beforeAll",
    "afterAll",
];

/// `isPestTestChain`: the chain's root is a call of one of `PEST_TEST_FUNCTIONS`.
fn is_pest_test_chain(call: &MethodCall<'_>) -> bool {
    root_func_call(call).is_some_and(|f| PEST_TEST_FUNCTIONS.iter().any(|name| named(f, name)))
}
