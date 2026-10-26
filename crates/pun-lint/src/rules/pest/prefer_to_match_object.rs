//! `lint/pest/prefer-to-match-object`, ported from pest-plugin-rector's `UseToMatchObjectRector`: consecutive
//! `expect($user)->toHaveProperty('name', 'Nuno');` statements on the same object become
//! `expect($user)->toMatchObject(['name' => 'Nuno', ...]);`.

use mago_syntax::cst::{Expression, MethodCall, Node, NodeKind};
use pun_problem::{Safety, Span};

use super::{
    Is, Item, STATEMENT_LISTS, Trinary, argument_value, comment_spans, expect_argument,
    expect_call, has_not, is_expect_chain, is_name, kept_text, method_call, report, rewrite_list,
    statement_expression, statement_list, type_is,
};
use crate::syntax::{node_span, same_expression, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferToMatchObject;

impl Rule for PreferToMatchObject {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-match-object"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(STATEMENT_LISTS)
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(list) = statement_list(ctx, node) else {
            return;
        };
        let stmts = &list.stmts;
        let chain = |i: usize| {
            stmts
                .get(i)
                .and_then(|s| statement_expression(s))
                .and_then(method_call)
                .filter(|c| is_expect_chain(c))
        };
        let mut items: Vec<Item<'_>> = Vec::new();
        let mut i = 0;
        let mut changed = false;
        while i < stmts.len() {
            let Some(first) = chain(i) else {
                items.push(Item::Old(i));
                i += 1;
                continue;
            };
            let (Some(property), Some(subject)) =
                (property_with_value(first), expect_argument(first))
            else {
                items.push(Item::Old(i));
                i += 1;
                continue;
            };
            if type_is(ctx, subject, Is::Object) == Trinary::No {
                items.push(Item::Old(i));
                i += 1;
                continue;
            }
            let mut properties = vec![property];
            let mut j = i + 1;
            while let Some(next) = chain(j) {
                if !expect_argument(next).is_some_and(|s| same_expression(ctx.source, subject, s)) {
                    break;
                }
                let Some(property) = property_with_value(next) else {
                    break;
                };
                properties.push(property);
                j += 1;
            }
            if properties.len() < 2 {
                items.push(Item::Old(i));
                i += 1;
                continue;
            }
            // `chain` keeps only chains on `expect()`.
            let expect = expect_call(first).map(node_span).unwrap();
            let comments: Vec<Span> = stmts[i..j]
                .iter()
                .flat_map(|s| comment_spans(ctx, node_span(*s).start))
                .collect();
            items.push(Item::New {
                print: Box::new(move |indent| {
                    let text =
                        |e: &Expression<'_>| kept_text(ctx, node_span(unparenthesize(e)), indent);
                    let pairs: Vec<String> = properties
                        .iter()
                        .map(|(k, v)| format!("{} => {}", text(k), text(v)))
                        .collect();
                    format!(
                        "{}->toMatchObject([{}]);",
                        kept_text(ctx, expect, indent),
                        pairs.join(", ")
                    )
                }),
                comments,
            });
            changed = true;
            i = j;
        }
        if list.nop {
            items.push(Item::Old(stmts.len()));
        }
        if !changed {
            return;
        }
        if let Some(edits) = rewrite_list(ctx, node, ancestors, &list, &items) {
            out.push(report(
                node_span(stmts[0]),
                "Use `toMatchObject()` instead of consecutive `toHaveProperty()` checks."
                    .to_string(),
                Safety::Safe,
                edits,
            ));
        }
    }
}

/// `extractPropertyWithValue`: the name and the value of `->toHaveProperty(name, value)`, not after `->not`.
fn property_with_value<'a>(
    call: &'a MethodCall<'a>,
) -> Option<(&'a Expression<'a>, &'a Expression<'a>)> {
    if !is_name(&call.method, "toHaveProperty") || has_not(call) {
        return None;
    }
    let [key, value] = call.argument_list.arguments.as_slice() else {
        return None;
    };
    Some((argument_value(key), argument_value(value)))
}
