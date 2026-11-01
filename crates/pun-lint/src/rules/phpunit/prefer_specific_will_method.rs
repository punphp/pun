//! `lint/phpunit/prefer-specific-will-method`, ported from rector-phpunit's `UseSpecificWillMethodRector`
//! (rules/CodeQuality/Rector/MethodCall).
//!
//! In a test class, `->will($this->returnValue($a))` becomes `->willReturn($a)`, and likewise for `returnArgument`,
//! `returnCallback`, `returnSelf`, `returnValueMap`, `onConsecutiveCalls` and `throwException`;
//! `->will(new ReturnValueMap($map))` becomes `->willReturnMap($map)`.

use mago_syntax::cst::{Call, Expression, Node, NodeKind};
use pun_problem::Safety;

use super::{AnyCall, NewArg, is_in_test_class, resolve, rewrite_args, text};
use crate::syntax::unparenthesize;
use crate::{Context, Report, Rule};

const NESTED_METHOD_TO_RENAME_MAP: &[(&str, &str)] = &[
    ("returnArgument", "willReturnArgument"),
    ("returnCallback", "willReturnCallback"),
    ("returnSelf", "willReturnSelf"),
    ("returnValue", "willReturn"),
    ("returnValueMap", "willReturnMap"),
    ("onConsecutiveCalls", "willReturnOnConsecutiveCalls"),
    ("throwException", "willThrowException"),
];

pub struct PreferSpecificWillMethod;

impl Rule for PreferSpecificWillMethod {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-specific-will-method"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall, NodeKind::StaticMethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let call = AnyCall::of(node).unwrap();
        if !is_in_test_class(ctx, ancestors, None) || !call.is_name(&["will"]) {
            return;
        }
        let Some(first) = call.args().first().copied() else {
            return;
        };
        let (name, inner) = match unparenthesize(first.value()) {
            Expression::Instantiation(new) => {
                let Expression::Identifier(class) = new.class else {
                    return;
                };
                if !resolve(ancestors, class)
                    .eq_ignore_ascii_case("PHPUnit\\Framework\\MockObject\\Stub\\ReturnValueMap")
                {
                    return;
                }
                let args: Vec<_> = new
                    .argument_list
                    .iter()
                    .flat_map(|l| l.arguments.iter())
                    .collect();
                ("willReturnMap", args)
            }
            Expression::Call(Call::Method(m)) => {
                let nested = AnyCall::of(Node::MethodCall(m)).unwrap();
                let Some(&(_, name)) = NESTED_METHOD_TO_RENAME_MAP
                    .iter()
                    .find(|(old, _)| nested.is_name(&[old]))
                else {
                    return;
                };
                (name, nested.args())
            }
            Expression::Call(Call::StaticMethod(s)) => {
                let nested = AnyCall::of(Node::StaticMethodCall(s)).unwrap();
                let Some(&(_, name)) = NESTED_METHOD_TO_RENAME_MAP
                    .iter()
                    .find(|(old, _)| nested.is_name(&[old]))
                else {
                    return;
                };
                (name, nested.args())
            }
            _ => return,
        };
        let new: Vec<NewArg> = inner
            .iter()
            .map(|a| NewArg::New(text(ctx, *a).to_string()))
            .collect();
        out.push(Report::fixed(
            call.span,
            format!("Use `{name}()` instead of `will()`."),
            Safety::Unsafe,
            rewrite_args(ctx, &call, Some(name), &new),
        ));
    }
}
