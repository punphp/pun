//! `lint/pest/no-focused-test`, ported from pest-plugin-rector's `RemoveOnlyRector`: `->only()` on a Pest test
//! (`test()`, `it()`, `describe()`, `todo()` or a hook) is removed, so the whole suite runs.

use mago_syntax::cst::{Expression, MethodCall, Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::{func_call, identifier, method_call, property_fetch, report};
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

/// `isPestTestChain`: the chain's root (`PestChainAnalyzer::getRootFuncCall`, under its method calls and then its
/// property fetches) is a call of one of `PEST_TEST_FUNCTIONS`.
fn is_pest_test_chain(call: &MethodCall<'_>) -> bool {
    let mut current: &Expression<'_> = call.object;
    while let Some(m) = method_call(current) {
        current = m.object;
    }
    while let Some(p) = property_fetch(current) {
        current = p.object;
    }
    func_call(current).is_some_and(|f| PEST_TEST_FUNCTIONS.iter().any(|name| named(f, name)))
}
