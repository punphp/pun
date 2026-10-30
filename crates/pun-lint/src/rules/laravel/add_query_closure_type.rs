//! `lint/laravel/add-query-closure-type`, ported from rector-laravel 2.5.0's `EloquentWhereTypeHintClosureParameterRector`
//! (src/Rector/MethodCall).
//!
//! A query builder's `where()` or `orWhere()` (a method call on an `Illuminate\Contracts\Database\Query\Builder`, or a
//! static call a model forwards to its query) whose first argument is a closure or arrow function gets that function's
//! first parameter typed: `\Illuminate\Contracts\Database\Eloquent\Builder` when the call is on a model, else
//! `\Illuminate\Contracts\Database\Query\Builder`. A parameter already typed with a class name stays.

use mago_syntax::cst::{Node, NodeKind};

use super::object_type::is_object_type;
use super::query_builder::{call_parts, closure_first_param, is_matching_call, type_param};
use crate::{Context, Report, Rule};

pub struct AddQueryClosureType;

impl Rule for AddQueryClosureType {
    fn id(&self) -> &'static str {
        "lint/laravel/add-query-closure-type"
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
        let Some(call) = call_parts(&node) else {
            return;
        };
        let Some(Some(param)) = call.arguments.first().and_then(|a| closure_first_param(a)) else {
            return;
        };
        if !is_matching_call(ctx, ancestors, &call, "where")
            && !is_matching_call(ctx, ancestors, &call, "orWhere")
        {
            return;
        }
        let class = if is_object_type(
            ctx,
            ancestors,
            call.on,
            "Illuminate\\Database\\Eloquent\\Model",
        ) {
            "Illuminate\\Contracts\\Database\\Eloquent\\Builder"
        } else {
            "Illuminate\\Contracts\\Database\\Query\\Builder"
        };
        out.extend(type_param(&node, param, class));
    }
}
