//! `lint/pest/prefer-to-have-keys`, ported from pest-plugin-rector's `UseToHaveKeysRector`: a run of
//! `->toHaveKey('a')->toHaveKey('b')` with literal strings becomes `->toHaveKeys(['a', 'b'])`, when the checked value
//! is known to be an array.

use mago_syntax::cst::{Expression, Literal, Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::{Is, Trinary, has_not, is_name, matcher_subject, method_call, rename, type_is};
use crate::rules::phpunit::expr_text;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferToHaveKeys;

impl Rule for PreferToHaveKeys {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-have-keys"
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
        if !is_name(&call.method, "toHaveKey") {
            return;
        }
        if matcher_subject(call).is_none_or(|s| type_is(ctx, s, Is::Array) != Trinary::Yes) {
            return;
        }
        let mut keys = Vec::new();
        let mut first = None;
        let mut current = Some(call);
        while let Some(c) = current.filter(|c| is_name(&c.method, "toHaveKey")) {
            if has_not(c) {
                return;
            }
            let [argument] = c.argument_list.arguments.as_slice() else {
                return;
            };
            let key = argument.value();
            if !matches!(unparenthesize(key), Expression::Literal(Literal::String(_))) {
                return;
            }
            keys.insert(0, expr_text(ctx, key));
            first = Some((c, argument));
            current = method_call(c.object);
        }
        let (Some((first, argument)), 2..) = (first, keys.len()) else {
            return;
        };
        out.push(Report::fixed(
            node_span(call),
            "Use `toHaveKeys()` instead of a run of `toHaveKey()`.",
            Safety::Safe,
            vec![
                rename(first, "toHaveKeys"),
                Edit::new(node_span(argument), format!("[{}]", keys.join(", "))),
                Edit::new(Span::new(node_span(first).end, node_span(call).end), ""),
            ],
        ));
    }
}
