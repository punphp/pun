//! `lint/pest/no-before-all-in-describe`, ported from pest-plugin-rector's `ConvertBeforeAllInDescribeRector`: in a
//! `describe()` callback (and the callbacks of the `describe()` calls in it), `beforeAll()` becomes `beforeEach()` and
//! `afterAll()` becomes `afterEach()`, as Pest does not run the `*All` hooks there.

use mago_syntax::cst::{Closure, Node, NodeKind, Statement};
use pun_problem::{Edit, Safety};

use super::{Callback, func_call, function_name, pest_callback, report};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoBeforeAllInDescribe;

impl Rule for NoBeforeAllInDescribe {
    fn id(&self) -> &'static str {
        "lint/pest/no-before-all-in-describe"
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
        if function_name(call) != Some(b"describe") {
            return;
        }
        let Some((Callback::Closure(closure), ..)) = pest_callback(call) else {
            return;
        };
        let mut edits = Vec::new();
        invalid_hooks(closure, &mut edits);
        if edits.is_empty() {
            return;
        }
        out.push(report(
            node_span(call),
            "Use `beforeEach()` and `afterEach()` inside `describe()`.".to_string(),
            Safety::Safe,
            edits,
        ));
    }
}

/// `HookSemanticAnalyzer::findInvalidDescribeHooks`: each statement of the callback that is a call of `beforeAll()` or
/// `afterAll()` gets the edit that renames it, and the callbacks of the `describe()` calls are searched too.
fn invalid_hooks(closure: &Closure<'_>, edits: &mut Vec<Edit>) {
    for statement in closure.body.statements.iter() {
        let Statement::Expression(statement) = statement else {
            continue;
        };
        let Some(call) = func_call(statement.expression) else {
            continue;
        };
        let replacement = match function_name(call) {
            Some(b"beforeAll") => "beforeEach",
            Some(b"afterAll") => "afterEach",
            Some(b"describe") => {
                if let Some((Callback::Closure(nested), ..)) = pest_callback(call) {
                    invalid_hooks(nested, edits);
                }
                continue;
            }
            _ => continue,
        };
        edits.push(Edit::new(
            node_span(unparenthesize(call.function)),
            replacement,
        ));
    }
}
