//! `lint/pest/prefer-sequence`, ported from pest-plugin-rector's `UseSequenceMatcherRector`: consecutive
//! `expect($items[0])->toBe('a'); expect($items[1])->toBe('b');` become
//! `expect($items)->sequence(fn($e) => $e->toBe('a'), fn($e) => $e->toBe('b'));`.

use mago_syntax::cst::{Expression, Literal, MethodCall, Node, NodeKind};
use pun_problem::{Safety, Span};

use super::{
    Item, STATEMENT_LISTS, comment_spans, expect_call, first_value, kept_text, method_call,
    print_args, print_member, property_fetch, report, rewrite_list, statement_expression,
    statement_list,
};
use crate::syntax::{node_span, same_expression, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferSequence;

impl Rule for PreferSequence {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-sequence"
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
        let mut items: Vec<Item<'_>> = Vec::new();
        let mut i = 0;
        let mut changed = false;
        while i < stmts.len() {
            let Some((variable, chains)) = sequence_group(ctx, stmts, i) else {
                items.push(Item::Old(i));
                i += 1;
                continue;
            };
            let rebuilt: Vec<_> = chains.iter().map(|c| bottom(c)).collect();
            let count = chains.len();
            let comments: Vec<Span> = stmts[i..i + count]
                .iter()
                .flat_map(|s| comment_spans(ctx, node_span(*s).start))
                .collect();
            items.push(Item::New {
                print: Box::new(move |indent| {
                    let arrows: Vec<String> = rebuilt
                        .iter()
                        .map(|(calls, property)| {
                            format!("fn($e) => {}", on_param(ctx, calls, *property, indent))
                        })
                        .collect();
                    format!(
                        "expect({})->sequence({});",
                        kept_text(ctx, node_span(unparenthesize(variable)), indent),
                        arrows.join(", ")
                    )
                }),
                comments,
            });
            i += count;
            changed = true;
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
                "Use `sequence()` instead of consecutive `expect()` calls on indexes.".to_string(),
                Safety::Safe,
                edits,
            ));
        }
    }
}

/// `collectSequenceGroup`: from `start`, the statements `expect($var[0])...`, `expect($var[1])...` and so on, with
/// the same `$var`; at least two.
fn sequence_group<'a>(
    ctx: &Context<'_>,
    stmts: &[&'a mago_syntax::cst::Statement<'a>],
    start: usize,
) -> Option<(&'a Expression<'a>, Vec<&'a MethodCall<'a>>)> {
    let mut chains = Vec::new();
    let mut base: Option<&Expression<'_>> = None;
    for statement in &stmts[start..] {
        let Some(call) = statement_expression(statement).and_then(method_call) else {
            break;
        };
        let Some(expect) = expect_call(call) else {
            break;
        };
        let Some(Expression::ArrayAccess(access)) =
            first_value(&expect.argument_list).map(unparenthesize)
        else {
            break;
        };
        let Expression::Literal(Literal::Integer(index)) = unparenthesize(access.index) else {
            break;
        };
        if index.value != Some(chains.len() as u64) {
            break;
        }
        match base {
            None => base = Some(access.array),
            Some(b) if !same_expression(ctx.source, b, access.array) => break,
            Some(_) => {}
        }
        chains.push(call);
    }
    if chains.len() < 2 {
        return None;
    }
    Some((base?, chains))
}

/// The method calls of a chain from the top down, and whether the expression under them is a property fetch (else it
/// is the `expect()` call, so `rebuildChainOnParam` never refuses a chain here).
fn bottom<'a>(call: &'a MethodCall<'a>) -> (Vec<&'a MethodCall<'a>>, bool) {
    let mut calls = vec![call];
    let mut current = call.object;
    while let Some(m) = method_call(current) {
        calls.push(m);
        current = m.object;
    }
    (calls, property_fetch(current).is_some())
}

/// `rebuildChainOnParam` printed: the chain's method calls on `$e`, or on `$e->not` when a property fetch is under
/// them.
fn on_param(ctx: &Context<'_>, calls: &[&MethodCall<'_>], property: bool, indent: usize) -> String {
    let mut out = String::from(if property { "$e->not" } else { "$e" });
    for c in calls.iter().rev() {
        out.push_str(&format!(
            "->{}({})",
            print_member(ctx, &c.method, indent),
            print_args(ctx, &c.argument_list, indent)
        ));
    }
    out
}
