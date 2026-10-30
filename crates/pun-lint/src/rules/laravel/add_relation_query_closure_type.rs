//! `lint/laravel/add-relation-query-closure-type`, ported from rector-laravel 2.5.0's
//! `EloquentWhereRelationTypeHintingParameterRector` (src/Rector/MethodCall).
//!
//! A query builder's `whereHas()`, `orWhereHas()`, `whereDoesntHave()` or `orWhereDoesntHave()` whose second argument
//! is a closure or arrow function, or one of their `...Morph` forms whose third is, gets that function's first
//! parameter typed `\Illuminate\Contracts\Database\Query\Builder`. A parameter already typed with a class name stays.

use mago_syntax::cst::{Node, NodeKind};

use super::is_name;
use super::query_builder::{call_parts, closure_first_param, is_matching_call, type_param};
use crate::{Context, Report, Rule};

const METHODS: [&str; 8] = [
    "whereHas",
    "orWhereHas",
    "whereDoesntHave",
    "orWhereDoesntHave",
    "whereHasMorph",
    "orWhereHasMorph",
    "whereDoesntHaveMorph",
    "orWhereDoesntHaveMorph",
];

pub struct AddRelationQueryClosureType;

impl Rule for AddRelationQueryClosureType {
    fn id(&self) -> &'static str {
        "lint/laravel/add-relation-query-closure-type"
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
        // `getPosition`: the morph methods take the closure third, the others second
        let position = if METHODS[4..].iter().any(|m| is_name(call.name, m)) {
            2
        } else {
            1
        };
        let Some(Some(param)) = call
            .arguments
            .get(position)
            .and_then(|a| closure_first_param(a))
        else {
            return;
        };
        if !METHODS
            .iter()
            .any(|m| is_matching_call(ctx, ancestors, &call, m))
        {
            return;
        }
        out.extend(type_param(
            &node,
            param,
            "Illuminate\\Contracts\\Database\\Query\\Builder",
        ));
    }
}
