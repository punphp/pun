//! `lint/phpunit/prefer-narrow-will-return-callback`, ported from rector-phpunit's
//! `NarrowSingleWillReturnCallbackRector` (rules/CodeQuality/Rector/MethodCall) and its `MatchAndReturnMatch`.
//!
//! In a test class, `willReturnCallback()` of a closure holding only a `match` of a single arm `1 =>` that calls
//! `assertSame()` or `assertEquals()` on a test case (and optionally `return match (...) { ... };`) becomes `with()`
//! of the assertion's expected value (an array's item values a separate argument each), followed by
//! `->willReturn(...)` of the returned match's first arm.

use mago_syntax::cst::{
    ArrayElement, Call, Expression, Literal, Match, MatchArm, Node, NodeKind, Statement,
};
use pun_problem::{Edit, Safety, Span};

use super::{
    AnyCall, NewArg, arg_value, expr_text, is_in_test_class, is_test_case_call_named, report,
    rewrite_args,
};
use crate::rules::rector_batch5::merge;
use crate::rules::rector_stmts::block_list;
use crate::syntax::unparenthesize;
use crate::{Context, Report, Rule};

pub struct PreferNarrowWillReturnCallback;

impl Rule for PreferNarrowWillReturnCallback {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-narrow-will-return-callback"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let call = AnyCall::of(node).unwrap();
        if !call.is_name(&["willReturnCallback"]) || !is_in_test_class(ctx, ancestors, None) {
            return;
        }
        let [arg] = call.args()[..] else {
            return;
        };
        let Expression::Closure(closure) = unparenthesize(arg_value(arg)) else {
            return;
        };
        let list = block_list(ctx.source, &closure.body);
        let (consecutive, will_return) = match (list.len(), list.stmts.as_slice()) {
            (1, [Statement::Expression(e)]) => (e.expression, None),
            (2, [Statement::Expression(e), Statement::Return(r)]) => {
                match r.value.map(unparenthesize) {
                    Some(Expression::Match(m)) => (e.expression, Some(m)),
                    _ => return,
                }
            }
            _ => return,
        };
        let Expression::Match(consecutive) = unparenthesize(consecutive) else {
            return;
        };
        let Some(body) = sole_arm_body_with_condition_one(consecutive) else {
            return;
        };
        let Some(assert) = (match unparenthesize(body) {
            Expression::Call(Call::Method(m)) => AnyCall::of(Node::MethodCall(m)),
            Expression::Call(Call::StaticMethod(s)) => AnyCall::of(Node::StaticMethodCall(s)),
            _ => None,
        }) else {
            return;
        };
        let mut scope = ancestors.to_vec();
        scope.push(node);
        if !is_test_case_call_named(ctx, &scope, &assert, &["assertSame", "assertEquals"]) {
            return;
        }
        let Some(expected) = assert.args().into_iter().next().map(arg_value) else {
            return;
        };
        let elements: Option<Vec<&ArrayElement<'_>>> = match unparenthesize(expected) {
            Expression::Array(a) => Some(a.elements.iter().collect()),
            Expression::LegacyArray(a) => Some(a.elements.iter().collect()),
            _ => None,
        };
        let args: Vec<NewArg> = match elements {
            Some(elements) => {
                let mut args = Vec::new();
                for element in elements {
                    let value = match element {
                        ArrayElement::KeyValue(kv) => kv.value,
                        ArrayElement::Value(v) => v.value,
                        ArrayElement::Variadic(v) => v.value,
                        ArrayElement::Missing(_) => return,
                    };
                    args.push(NewArg::New(expr_text(ctx, value).to_string()));
                }
                args
            }
            None => vec![NewArg::New(expr_text(ctx, expected).to_string())],
        };
        let mut edits = rewrite_args(ctx, &call, Some("with"), &args);
        if let Some(will_return) = will_return {
            let Some(first) = will_return.arms.iter().next() else {
                return;
            };
            let end = call.span.end;
            edits.push(Edit::new(
                Span::new(end, end),
                format!("->willReturn({})", expr_text(ctx, first.expression())),
            ));
        }
        out.push(report(
            call.span,
            "Use `with()` instead of a callback that matches a single invocation.",
            Safety::Safe,
            merge(edits),
        ));
    }
}

/// `matchSingleMatchArmBodyWithConditionOne`.
fn sole_arm_body_with_condition_one<'a>(m: &'a Match<'a>) -> Option<&'a Expression<'a>> {
    let [MatchArm::Expression(arm)] = m.arms.as_slice() else {
        return None;
    };
    let [condition] = arm.conditions.as_slice() else {
        return None;
    };
    match unparenthesize(condition) {
        Expression::Literal(Literal::Integer(i)) if i.value == Some(1) => Some(arm.expression),
        _ => None,
    }
}
