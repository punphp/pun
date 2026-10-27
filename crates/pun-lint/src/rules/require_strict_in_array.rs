//! `lint/functions/require-strict-in-array`, ported from Rector 2.6.1's `StrictInArrayRector`
//! (rules/CodingStyle/Rector/FuncCall).
//!
//! `in_array($needle, $haystack)` with exactly two arguments gets `true` as its third when the haystack's native type
//! is an array whose value type is a subtype of the needle's native type (`TypeComparator::isSubtype`, which is never
//! true for a `mixed` needle). The native types are Mago's analyser's with docblocks blanked out, and the subtype
//! check is Mago's `union_comparator::is_contained_by`. After a named argument Pun writes `strict: true`, where Rector
//! writes a positional `true` that PHP rejects.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::comparator::ComparisonResult;
use mago_codex::ttype::comparator::union_comparator::is_contained_by;
use mago_codex::ttype::get_iterable_parameters;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{Argument, Expression, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety, Span};

use super::refactor::{function_name, is_name};
use super::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct RequireStrictInArray;

impl Rule for RequireStrictInArray {
    fn id(&self) -> &'static str {
        "lint/functions/require-strict-in-array"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(call) = node else {
            return;
        };
        let Expression::Identifier(name) = call.function else {
            return;
        };
        if !is_name(&function_name(ctx, ancestors, name), "in_array") {
            return;
        }
        let args: Vec<_> = call.argument_list.arguments.iter().collect();
        let [needle, haystack] = args[..] else {
            return;
        };
        let facts = facts(ctx);
        let native = |e: &Expression<'_>| facts.native_type(node_span(unparenthesize(e))).cloned();
        let (Some(needle_type), Some(haystack_type)) =
            (native(needle.value()), native(haystack.value()))
        else {
            return;
        };
        if haystack_type.types.is_empty()
            || !haystack_type.types.iter().all(|a| matches!(a, TAtomic::Array(_)))
            || needle_type.is_mixed()
        {
            return;
        }
        // `getIterableValueType()`: the union of the value types of the arrays (an empty array has none: `never`).
        let mut values: Vec<TAtomic> = Vec::new();
        for atomic in haystack_type.types.iter() {
            if let Some((_, value)) = get_iterable_parameters(atomic, &facts.codebase) {
                for t in value.types.iter() {
                    if !values.contains(t) {
                        values.push(t.clone());
                    }
                }
            }
        }
        let value_type = TUnion::from_vec(values);
        if !is_contained_by(
            &facts.codebase,
            &value_type,
            &needle_type,
            false,
            false,
            false,
            &mut ComparisonResult::new(),
        ) {
            return;
        }
        let end = node_span(haystack).end;
        // After named arguments a positional one is a compile error, where Rector writes one.
        let text = match matches!(needle, Argument::Named(_)) || matches!(haystack, Argument::Named(_)) {
            true => ", strict: true",
            false => ", true",
        };
        out.push(Report {
            span: node_span(call),
            message: "Pass `true` to `in_array()`: the needle and the haystack's values have the same type."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(Span::new(end, end), text)],
            }),
        });
    }
}
