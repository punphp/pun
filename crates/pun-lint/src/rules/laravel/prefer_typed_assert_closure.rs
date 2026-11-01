//! `lint/laravel/prefer-typed-assert-closure`, ported from rector-laravel 2.5.0's
//! `AssertWithClassStringToTypeHintedClosureRector` (src/Rector/StaticCall) and its `FacadeAssertionAnalyzer`.
//!
//! `Bus::assertDispatched(Job::class, function ($job) {...})` (the `Illuminate\Support\Facades\Bus` or `Event`
//! facade) becomes `Bus::assertDispatched(function (\Job $job) {...})`: the class string goes and types the first
//! parameter of the closure or arrow function. php-parser prints the call anew, its arguments on one line, and moves
//! the closure's lines to the indentation of the call's line.

use mago_syntax::cst::{ClassLikeMemberSelector, Expression, Node, NodeKind, StaticMethodCall};
use pun_problem::{Edit, Safety};

use super::is_name;
use super::object_type::{class_string_class, is_object_type};
use super::print::{moved_with_edit, set_param_type};
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferTypedAssertClosure;

impl Rule for PreferTypedAssertClosure {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-typed-assert-closure"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::StaticMethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::StaticMethodCall(
            call @ StaticMethodCall {
                method: ClassLikeMemberSelector::Identifier(method),
                ..
            },
        ) = node
        else {
            return;
        };
        if !is_name(method.value, "assertDispatched") {
            return;
        }
        if !is_object_type(
            ctx,
            ancestors,
            call.class,
            "Illuminate\\Support\\Facades\\Bus",
        ) && !is_object_type(
            ctx,
            ancestors,
            call.class,
            "Illuminate\\Support\\Facades\\Event",
        ) {
            return;
        }
        let [class, callback] = call.argument_list.arguments.as_slice() else {
            return;
        };
        let callback = unparenthesize(callback.value());
        let param = match callback {
            Expression::Closure(c) => c.parameter_list.parameters.first(),
            Expression::ArrowFunction(a) => a.parameter_list.parameters.first(),
            _ => return,
        };
        let Some(param) = param else { return };
        let facts = facts(ctx);
        let Some(class) = (facts.full_type(node_span(unparenthesize(class.value()))))
            .and_then(|t| class_string_class(ctx, t))
        else {
            return;
        };
        let typed = moved_with_edit(
            ctx.source,
            node_span(callback),
            &set_param_type(param, &format!("\\{class}")),
            node_span(call).start,
        );
        out.push(Report::fixed(
            node_span(call),
            format!("Type the closure's parameter as `{class}` instead of passing its class."),
            Safety::Unsafe,
            vec![Edit::new(
                node_span(&call.argument_list),
                format!("({typed})"),
            )],
        ));
    }
}
