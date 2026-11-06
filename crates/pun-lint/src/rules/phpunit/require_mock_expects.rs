//! `lint/phpunit/require-mock-expects`, ported from rector-phpunit's `DecorateWillReturnMapWithExpectsMockRector`
//! (rules/CodeQuality/Rector/Expression).
//!
//! In a test method other than `setUp()` (`TestsNodeAnalyzer::isTestClassMethod()`), each expression statement, however
//! deep, that is a `->willReturnMap([...])` call chain on a `MockObject` without `expects()` at its start gets
//! `->expects($this->atLeast(N))` there, N the number of the array's items.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{Array, Call, Expression, MethodBody, Node, NodeKind, Statement};
use pun_problem::{Edit, Safety, Span};

use super::annotations::named_tags;
use super::prefer_create_stub_expression::is_mock_object;
use super::{AnyCall, is_public};
use crate::rules::lint_b_doc::Doc;
use crate::rules::rector_batch5::has_attribute;
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct RequireMockExpects;

impl Rule for RequireMockExpects {
    fn id(&self) -> &'static str {
        "lint/phpunit/require-mock-expects"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Method(method) = node else { return };
        // allowed as can be flexible
        if method.name.value.eq_ignore_ascii_case(b"setUp") {
            return;
        }
        // only decorate inside test methods, skip helpers and other non-test methods
        let is_test = is_public(method)
            && (method.name.value.starts_with(b"test")
                || has_attribute(
                    ancestors,
                    &method.attribute_lists,
                    "PHPUnit\\Framework\\Attributes\\Test",
                )
                || Doc::of(ctx, ancestors, node_span(method).start).is_some_and(|doc| {
                    named_tags(ctx, ancestors, &doc, "test").is_some_and(|t| !t.is_empty())
                }));
        if !is_test {
            return;
        }
        let MethodBody::Concrete(_) = &method.body else {
            return;
        };
        let mut stack = vec![node];
        while let Some(node) = stack.pop() {
            if let Node::Statement(Statement::Expression(statement)) = node
                && let Some(edit) = refactor_expression(ctx, statement.expression)
            {
                out.push(Report::fixed(
                    node_span(statement),
                    "Add expects() to a mock configured with willReturnMap().",
                    Safety::Safe,
                    vec![edit],
                ));
            }
            node.visit_children(|child| stack.push(child));
        }
    }
}

/// `refactorExpression`: the edit that puts `->expects($this->atLeast(N))` after the receiver of the chain.
fn refactor_expression(ctx: &Context<'_>, expr: &Expression<'_>) -> Option<Edit> {
    let Expression::Call(Call::Method(call)) = expr else {
        return None;
    };
    let method_call = AnyCall::of(Node::MethodCall(call))?;
    if !method_call.is_name(&["willReturnMap"]) {
        return None;
    }
    let mut topmost = call;
    while let Expression::Call(Call::Method(inner)) = topmost.object {
        topmost = inner;
    }
    // already covered
    if AnyCall::of(Node::MethodCall(topmost)).is_some_and(|c| c.is_name(&["expects"])) {
        return None;
    }
    let facts = facts(ctx);
    let receiver = unparenthesize(topmost.object);
    if !facts
        .full_type(node_span(receiver))
        .is_some_and(|t| is_mock_object_type(&facts.codebase, t))
    {
        return None;
    }
    // count values in will map arg
    let arg = method_call.args().into_iter().next()?;
    let Expression::Array(Array { elements, .. }) = unparenthesize(arg.value()) else {
        return None;
    };
    let at = node_span(receiver).end;
    Some(Edit::new(
        Span::new(at, at),
        format!("->expects($this->atLeast({}))", elements.len()),
    ))
}

/// `isObjectType($expr, new ObjectType(MockObject))` on a type that is no single object: without `null` and `false`,
/// every type is a `MockObject`.
fn is_mock_object_type(codebase: &mago_codex::metadata::CodebaseMetadata, t: &TUnion) -> bool {
    let mut kept = t.types.iter().filter(|a| match a {
        TAtomic::Null => false,
        TAtomic::Scalar(TScalar::Bool(b)) => !b.is_false(),
        _ => true,
    });
    let mut any = false;
    kept.all(|a| {
        any = true;
        is_mock_object(codebase, a)
    }) && any
}
