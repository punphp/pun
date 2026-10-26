//! `lint/phpunit/prefer-create-stub-in-coalesce`, ported from rector-phpunit's `CreateStubInCoalesceArgRector`
//! (rules/PHPUnit120/Rector/CallLike).
//!
//! In a test class, a `createMock()` call on the right of a `??` that is an argument of a call or a `new` (not a
//! function call), or an array item's value, is renamed `createStub()`: nothing can expect calls on it. Rector runs
//! it only for PHPUnit 11 and later; Pun leaves that to the rule's group.

use mago_syntax::cst::{
    ArgumentList, ArrayElement, BinaryOperator, Call, Expression, Node, NodeKind, PartialArgument,
};
use pun_problem::{Edit, Safety};

use super::{AnyCall, arg_value, is_in_test_class, report};
use crate::syntax::unparenthesize;
use crate::{Context, Report, Rule};

pub struct PreferCreateStubInCoalesce;

impl Rule for PreferCreateStubInCoalesce {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-create-stub-in-coalesce"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::MethodCall,
            NodeKind::StaticMethodCall,
            NodeKind::Instantiation,
            NodeKind::AnonymousClass,
            NodeKind::ArrayElement,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let values: Vec<&Expression<'_>> = match node {
            Node::ArrayElement(ArrayElement::KeyValue(kv)) => vec![kv.value],
            Node::ArrayElement(ArrayElement::Value(v)) => vec![v.value],
            Node::ArrayElement(ArrayElement::Variadic(v)) => vec![v.value],
            Node::MethodCall(_) | Node::StaticMethodCall(_) => AnyCall::of(node)
                .map(|c| c.args().into_iter().map(arg_value).collect())
                .unwrap_or_default(),
            Node::Instantiation(new) => args(new.argument_list.as_ref()),
            Node::AnonymousClass(new) => new
                .argument_list
                .iter()
                .flat_map(|a| a.arguments.iter())
                .filter_map(|a| match a {
                    PartialArgument::Positional(a) => Some(a.value),
                    PartialArgument::Named(a) => Some(a.value),
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        };
        let names: Vec<_> = values
            .into_iter()
            .filter_map(|value| {
                let Expression::Binary(binary) = unparenthesize(value) else {
                    return None;
                };
                if !matches!(binary.operator, BinaryOperator::NullCoalesce(_)) {
                    return None;
                }
                let Expression::Call(Call::Method(right)) = unparenthesize(binary.rhs) else {
                    return None;
                };
                AnyCall::of(Node::MethodCall(right)).filter(|c| c.is_name(&["createMock"]))
            })
            .collect();
        if names.is_empty() || !is_in_test_class(ctx, ancestors, None) {
            return;
        }
        for call in names {
            out.push(report(
                call.name_span(),
                "Use `createStub()`: nothing expects calls on this mock.",
                Safety::Safe,
                vec![Edit::new(call.name_span(), "createStub")],
            ));
        }
    }
}

fn args<'a>(list: Option<&'a ArgumentList<'a>>) -> Vec<&'a Expression<'a>> {
    list.iter()
        .flat_map(|l| l.arguments.iter())
        .map(arg_value)
        .collect()
}
