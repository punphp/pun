//! `lint/pest/prefer-to-have-method-on-class`, ported from pest-plugin-rector's
//! `Pest2ToPest3\\ToHaveMethodOnClassRector`: `expect($object)->toHaveMethod('name')` (or `toHaveMethods()`) becomes
//! `expect($object::class)->toHaveMethod('name')`, as Pest 3 checks a class name; a string or a class constant is left
//! alone.

use mago_syntax::cst::{Access, Call, Expression, Literal, Node, NodeKind};
use pun_problem::{Edit, Safety};

use super::{Is, Trinary, expect_call, is_name, type_is};
use crate::rules::phpunit::expr_text;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferToHaveMethodOnClass;

impl Rule for PreferToHaveMethodOnClass {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-have-method-on-class"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodCall(call) = node else {
            return;
        };
        if !(is_name(&call.method, "toHaveMethod") || is_name(&call.method, "toHaveMethods")) {
            return;
        }
        let Some(first) = expect_call(call).and_then(|e| e.argument_list.arguments.first()) else {
            return;
        };
        let value = unparenthesize(first.value());
        if matches!(
            value,
            Expression::Access(Access::ClassConstant(_)) | Expression::Literal(Literal::String(_))
        ) || type_is(ctx, value, Is::String) == Trinary::Yes
        {
            return;
        }
        // php-parser's `pStaticDereferenceLhs`: the class of `::class` in parentheses unless it is one of these.
        let bare = matches!(
            value,
            Expression::Variable(_)
                | Expression::ArrayAccess(_)
                | Expression::Access(
                    Access::Property(_) | Access::NullSafeProperty(_) | Access::StaticProperty(_)
                )
                | Expression::Call(
                    Call::Function(_)
                        | Call::Method(_)
                        | Call::NullSafeMethod(_)
                        | Call::StaticMethod(_)
                )
                | Expression::PartialApplication(_)
                | Expression::Array(_)
                | Expression::LegacyArray(_)
        );
        let class = expr_text(ctx, value);
        let class = if bare {
            format!("{class}::class")
        } else {
            format!("({class})::class")
        };
        out.push(Report::fixed(
            node_span(call),
            "Check the methods of the object's class, as Pest 3 does.",
            Safety::Safe,
            vec![Edit::new(node_span(first), class)],
        ));
    }
}
