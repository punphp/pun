//! `lint/phpunit/no-value-object-mock`, ported from rector-phpunit's `DirectInstanceOverMockArgRector`
//! (rules/CodeQuality/Rector/CallLike).
//!
//! In a `TestCase` class, a `createMock()` call of Symfony's `Request` or `RequestStack` that is an argument of a
//! method call, a static call or a `new`, or an array item's value, becomes `new \...\Request()`.

use mago_syntax::cst::{
    Access, ArgumentList, ArrayElement, Call, ClassLikeConstantSelector, Expression, Node,
    NodeKind, PartialArgument,
};
use pun_problem::{Edit, Safety};

use super::{AnyCall, TEST_CASE, arg_value, class_is, report, resolve, string_value};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

const REQUEST: &str = "Symfony\\Component\\HttpFoundation\\Request";
const REQUEST_STACK: &str = "Symfony\\Component\\HttpFoundation\\RequestStack";

pub struct NoValueObjectMock;

impl Rule for NoValueObjectMock {
    fn id(&self) -> &'static str {
        "lint/phpunit/no-value-object-mock"
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
        let mocks: Vec<(&Expression<'_>, String)> = values
            .into_iter()
            .map(unparenthesize)
            .filter_map(|value| Some((value, mocked_class(ancestors, value)?)))
            .collect();
        if mocks.is_empty() {
            return;
        }
        // `$scope->getClassReflection()->is(TestCase)`.
        let Some(at) = ancestors.iter().rposition(|a| {
            matches!(
                a,
                Node::Class(_)
                    | Node::AnonymousClass(_)
                    | Node::Interface(_)
                    | Node::Trait(_)
                    | Node::Enum(_)
            )
        }) else {
            return;
        };
        if !class_is(ctx, &ancestors[..=at], &[TEST_CASE]) {
            return;
        }
        for (value, class) in mocks {
            out.push(report(
                node_span(value),
                "Use a real instance instead of mocking this value object.",
                Safety::Safe,
                vec![Edit::new(node_span(value), format!("new \\{class}()"))],
            ));
        }
    }
}

/// `matchCreateMockMethodCallArg` and the class its first argument names, when that is `Request` or `RequestStack`.
fn mocked_class(ancestors: &[Node<'_, '_>], value: &Expression<'_>) -> Option<String> {
    let Expression::Call(Call::Method(call)) = value else {
        return None;
    };
    let call = AnyCall::of(Node::MethodCall(call))?;
    if !call.is_name(&["createMock"]) {
        return None;
    }
    let first = arg_value(call.args().into_iter().next()?);
    let class = match first {
        Expression::Access(Access::ClassConstant(access)) => {
            let (Expression::Identifier(class), ClassLikeConstantSelector::Identifier(constant)) =
                (access.class, &access.constant)
            else {
                return None;
            };
            if !constant.value.eq_ignore_ascii_case(b"class") {
                return None;
            }
            resolve(ancestors, class)
        }
        _ => string_value(first)?,
    };
    [REQUEST, REQUEST_STACK]
        .contains(&class.as_str())
        .then_some(class)
}

fn args<'a>(list: Option<&'a ArgumentList<'a>>) -> Vec<&'a Expression<'a>> {
    list.iter()
        .flat_map(|l| l.arguments.iter())
        .map(arg_value)
        .collect()
}
