//! `lint/phpunit/prefer-simple-callback-assert`, ported from rector-phpunit's `CallbackSingleAssertToSimplerRector`
//! (rules/CodeQuality/Rector/MethodCall).
//!
//! In a test class, an argument of a `with()` call that is `callback()` of a closure (capturing nothing by reference)
//! doing only `assertSame($expected, $param)` (either way round, `$param` the closure's sole parameter), optionally
//! followed by `return true;`, becomes `$this->equalTo($expected)`.

use mago_syntax::cst::{Call, Expression, Node, NodeKind, Statement, Variable};
use pun_problem::{Edit, Safety};

use super::{AnyCall, arg_value, expr_text, is_constant, is_in_test_class, report};
use crate::rules::rector_stmts::block_list;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferSimpleCallbackAssert;

impl Rule for PreferSimpleCallbackAssert {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-simple-callback-assert"
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
        if !call.is_name(&["with"]) || !is_in_test_class(ctx, ancestors, None) {
            return;
        }
        for arg in call.args() {
            let value = arg_value(arg);
            let Some(expected) = sole_assert_same_expected(ctx, value) else {
                continue;
            };
            let span = node_span(value);
            out.push(report(
                span,
                "Use `equalTo()` instead of a callback that only asserts the value.",
                Safety::Safe,
                vec![Edit::new(
                    span,
                    format!("$this->equalTo({})", expr_text(ctx, expected)),
                )],
            ));
        }
    }
}

/// `matchCallbackSoleAssertSameExpected`.
fn sole_assert_same_expected<'a>(
    ctx: &Context<'_>,
    expr: &'a Expression<'a>,
) -> Option<&'a Expression<'a>> {
    let Expression::Call(Call::Method(callback)) = expr else {
        return None;
    };
    let callback = AnyCall::of(Node::MethodCall(callback)).filter(|c| c.is_name(&["callback"]))?;
    let first = callback.args().into_iter().next()?;
    let Expression::Closure(closure) = unparenthesize(arg_value(first)) else {
        return None;
    };
    if closure
        .use_clause
        .iter()
        .flat_map(|u| u.variables.iter())
        .any(|v| v.ampersand.is_some())
    {
        return None;
    }
    let list = block_list(ctx.source, &closure.body);
    let first = match (list.len(), list.stmts.as_slice()) {
        (2, [first, Statement::Return(r)]) if r.value.is_some_and(|v| is_constant(v, "true")) => {
            first
        }
        (1, [first]) => first,
        _ => return None,
    };
    let Statement::Expression(first) = first else {
        return None;
    };
    let Expression::Call(Call::Method(assert)) = unparenthesize(first.expression) else {
        return None;
    };
    let assert = AnyCall::of(Node::MethodCall(assert)).filter(|c| c.is_name(&["assertSame"]))?;
    let [a, b] = assert.args()[..] else {
        return None;
    };
    let params: Vec<_> = closure.parameter_list.parameters.iter().collect();
    let is_sole_param = |e: &Expression<'_>| {
        params.len() == 1
            && matches!(unparenthesize(e), Expression::Variable(Variable::Direct(v)) if v.name == params[0].variable.name)
    };
    if is_sole_param(arg_value(b)) {
        return Some(arg_value(a));
    }
    if is_sole_param(arg_value(a)) {
        return Some(arg_value(b));
    }
    None
}
