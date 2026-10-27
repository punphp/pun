//! `lint/phpunit/prefer-with-is-instance-of`, ported from rector-phpunit's `SimplerWithIsInstanceOfRector`
//! (rules/CodeQuality/Rector/MethodCall).
//!
//! In a test class, an argument of a `with()` call that is `callback()` of a closure doing only
//! `assertInstanceOf(X, ...)` and a `return`, or only `return $x instanceof X;`, becomes `$this->isInstanceOf(X)`. A
//! class name `X` of `instanceof` is written `\X::class` (resolved), `self`, `parent` and `static` as they are.

use mago_syntax::cst::{BinaryOperator, Call, Expression, Node, NodeKind, Statement};
use pun_problem::{Edit, Safety};

use super::{AnyCall, arg_value, expr_text, is_in_test_class, report, resolve};
use crate::rules::rector_stmts::block_list;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferWithIsInstanceOf;

impl Rule for PreferWithIsInstanceOf {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-with-is-instance-of"
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
            let Some(class) = sole_instanceof_class(ctx, ancestors, value) else {
                continue;
            };
            let span = node_span(value);
            out.push(report(
                span,
                "Use `isInstanceOf()` instead of a callback that checks the type.",
                Safety::Safe,
                vec![Edit::new(span, format!("$this->isInstanceOf({class})"))],
            ));
        }
    }
}

/// `matchCallbackSoleInstanceofCheckClassName`: the class a `callback()` of a closure checks, as printed.
fn sole_instanceof_class(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    expr: &Expression<'_>,
) -> Option<String> {
    let Expression::Call(Call::Method(callback)) = expr else {
        return None;
    };
    let callback = AnyCall::of(Node::MethodCall(callback)).filter(|c| c.is_name(&["callback"]))?;
    let first = callback.args().into_iter().next()?;
    let Expression::Closure(closure) = unparenthesize(arg_value(first)) else {
        return None;
    };
    let list = block_list(ctx.source, &closure.body);
    match (list.len(), list.stmts.as_slice()) {
        (2, [Statement::Expression(first), Statement::Return(_)]) => {
            let Expression::Call(Call::Method(assert)) = unparenthesize(first.expression) else {
                return None;
            };
            let assert = AnyCall::of(Node::MethodCall(assert))
                .filter(|c| c.is_name(&["assertInstanceOf"]))?;
            let class = assert.args().into_iter().next()?;
            Some(expr_text(ctx, arg_value(class)).to_string())
        }
        (1, [Statement::Return(r)]) => {
            let Expression::Binary(binary) = unparenthesize(r.value?) else {
                return None;
            };
            if !matches!(binary.operator, BinaryOperator::Instanceof(_)) {
                return None;
            }
            let Expression::Identifier(name) = binary.rhs else {
                return match binary.rhs {
                    Expression::Self_(k) | Expression::Static(k) | Expression::Parent(k) => {
                        let written = String::from_utf8_lossy(k.value).into_owned();
                        Some(match written.as_str() {
                            "self" | "static" | "parent" => format!("{written}::class"),
                            _ => format!("\\{written}::class"),
                        })
                    }
                    _ => None,
                };
            };
            Some(format!("\\{}::class", resolve(ancestors, name)))
        }
        _ => None,
    }
}
