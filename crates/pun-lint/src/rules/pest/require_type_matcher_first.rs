//! `lint/pest/require-type-matcher-first`, ported from pest-plugin-rector's `EnsureTypeChecksFirstRector`: in an
//! `expect()` chain, type matchers (`toBeInt()`, `toBeInstanceOf()`, ...) move before value matchers, and an
//! `expect()` with value matchers only swaps with the next one on the same value when that one checks a type.

use mago_syntax::cst::{ClassLikeMemberSelector, Node, NodeKind, Variable};
use pun_problem::Safety;

use super::{
    Item, Link, STATEMENT_LISTS, chain_links, chain_root, expect_argument, has_not,
    is_expect_chain, kept_text, method_call, print_links, report, rewrite_list,
    statement_expression, statement_list,
};
use crate::syntax::{node_span, same_expression};
use crate::{Context, Report, Rule};

pub struct RequireTypeMatcherFirst;

const TYPE_MATCHERS: [&[u8]; 13] = [
    b"toBeInt",
    b"toBeString",
    b"toBeArray",
    b"toBeFloat",
    b"toBeBool",
    b"toBeNull",
    b"toBeInstanceOf",
    b"toBeNumeric",
    b"toBeIterable",
    b"toBeCallable",
    b"toBeObject",
    b"toBeScalar",
    b"toBeResource",
];

impl Rule for RequireTypeMatcherFirst {
    fn id(&self) -> &'static str {
        "lint/pest/require-type-matcher-first"
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
        let list = statement_list(ctx, node);
        let stmts = &list.stmts;
        let call_of = |i: usize| statement_expression(stmts[i]).and_then(method_call);
        // `$stmts` as refactor() changes it (old indexes), while its foreach walks the list as it was.
        let mut live: Vec<usize> = (0..stmts.len()).collect();
        let mut texts: Vec<Option<String>> = vec![None; stmts.len()];
        let mut changed = false;
        for key in 0..stmts.len() {
            let Some(call) = call_of(key).filter(|c| is_expect_chain(c)) else {
                continue;
            };
            let methods = chain_links(call);
            let reordered = reorder(&methods);
            if reordered
                .iter()
                .zip(&methods)
                .any(|(a, b)| !same_link(a, b))
            {
                // `$stmt->expr = $this->rebuildMethodChain($root, $newMethods)`: printed again on one line.
                let span = node_span(stmts[key]);
                let expr = node_span(call);
                let indent = super::indent_at(ctx, span.start);
                let root = chain_root(call).map(node_span).unwrap();
                texts[key] = Some(format!(
                    "{}{}{}{}",
                    &ctx.source[span.start..expr.start],
                    kept_text(ctx, root, indent),
                    print_links(ctx, &reordered, indent),
                    &ctx.source[expr.end..span.end]
                ));
                changed = true;
                continue;
            }
            let Some(&next) = live.get(key + 1) else {
                continue;
            };
            let Some(next_call) = call_of(next) else {
                continue;
            };
            if !is_expect_chain(next_call) || has_not(next_call) {
                continue;
            }
            let (Some(first_arg), Some(second_arg)) =
                (expect_argument(call), expect_argument(next_call))
            else {
                continue;
            };
            if !same_expression(ctx.source, first_arg, second_arg) {
                continue;
            }
            let (first_type, first_other) = partition(&methods);
            let (second_type, _) = partition(&chain_links(next_call));
            let first_safe = first_other
                .iter()
                .all(|m| name(m).is_none_or(is_safe_non_type));
            if first_type.is_empty()
                && !first_other.is_empty()
                && !second_type.is_empty()
                && first_safe
            {
                live[key] = next;
                live[key + 1] = key;
                changed = true;
            }
        }
        if !changed {
            return;
        }
        let mut items: Vec<Item<'_>> = live
            .iter()
            .map(|&i| match texts[i].take() {
                Some(text) => Item::Changed(i, text),
                None => Item::Old(i),
            })
            .collect();
        if list.nop {
            items.push(Item::Old(stmts.len()));
        }
        let Some(edits) = rewrite_list(ctx, node, ancestors, &list, &items) else {
            return;
        };
        out.push(report(
            node_span(stmts[0]),
            "Check the type before the value in `expect()` chains.".to_string(),
            Safety::Safe,
            edits,
        ));
    }
}

/// The same link of the old chain: what PHP's `!==` on the method lists compares.
fn same_link(a: &Link<'_>, b: &Link<'_>) -> bool {
    let span = |l: &Link<'_>| match l {
        Link::Method(m) => node_span(*m),
        Link::Property(p) => node_span(*p),
    };
    span(a) == span(b)
}

/// `resolveMethodName`: the name as written, a variable's name for `->$name`, none otherwise.
fn name<'a>(link: &Link<'a>) -> Option<&'a [u8]> {
    let selector = match link {
        Link::Method(m) => &m.method,
        Link::Property(p) => &p.property,
    };
    match selector {
        ClassLikeMemberSelector::Identifier(id) => Some(id.value),
        ClassLikeMemberSelector::Variable(Variable::Direct(v)) => Some(&v.name[1..]),
        _ => None,
    }
}

fn is_type_matcher(name: &[u8]) -> bool {
    TYPE_MATCHERS.contains(&name)
}

/// `isSafeNonTypeMatcher`: a matcher or modifier Pest documents, which moving a type check across leaves right.
fn is_safe_non_type(name: &[u8]) -> bool {
    const PREFIXES: [&[u8]; 8] = [
        b"toBe",
        b"toContain",
        b"toEndWith",
        b"toEqual",
        b"toHave",
        b"toMatch",
        b"toStartWith",
        b"toThrow",
    ];
    const NAMES: [&[u8]; 12] = [
        b"and",
        b"dd",
        b"ddUnless",
        b"ddWhen",
        b"each",
        b"json",
        b"match",
        b"not",
        b"ray",
        b"sequence",
        b"unless",
        b"when",
    ];
    PREFIXES.iter().any(|p| name.starts_with(p)) || NAMES.contains(&name)
}

/// `partitionTypeAndNonType`.
fn partition<'a>(methods: &[Link<'a>]) -> (Vec<Link<'a>>, Vec<Link<'a>>) {
    methods
        .iter()
        .copied()
        .partition(|m| name(m).is_some_and(is_type_matcher))
}

/// `reorderWithinAndSegments`: between `->and()` calls, the units (a matcher with the `not` before it) that check a
/// type move before the others, when all of them are known matchers and the types come after a value check without
/// a value check after a type one.
fn reorder<'a>(methods: &[Link<'a>]) -> Vec<Link<'a>> {
    let mut result = Vec::new();
    let mut segment: Vec<Link<'a>> = Vec::new();
    for m in methods {
        if name(m) == Some(&b"and"[..]) {
            flush(&mut segment, &mut result);
            result.push(*m);
            continue;
        }
        segment.push(*m);
    }
    flush(&mut segment, &mut result);
    result
}

fn flush<'a>(segment: &mut Vec<Link<'a>>, result: &mut Vec<Link<'a>>) {
    let keep = segment.iter().any(|m| match name(m) {
        Some(b"each") => true,
        Some(n) => !is_type_matcher(n) && !is_safe_non_type(n),
        None => false,
    });
    // groupIntoUnits: each `not` stays with the matcher after it.
    let mut units: Vec<Vec<Link<'a>>> = Vec::new();
    let mut pending = Vec::new();
    for m in segment.iter() {
        pending.push(*m);
        if name(m) != Some(&b"not"[..]) {
            units.push(std::mem::take(&mut pending));
        }
    }
    if !pending.is_empty() {
        units.push(pending);
    }
    let is_type = |unit: &Vec<Link<'_>>| unit.last().and_then(name).is_some_and(is_type_matcher);
    let (types, others): (Vec<_>, Vec<_>) = units.iter().cloned().partition(|u| is_type(u));
    let mut needs = false;
    let mut found_other = false;
    for unit in &units {
        if is_type(unit) {
            if found_other {
                needs = true;
                break;
            }
        } else {
            found_other = true;
        }
    }
    if needs {
        // A value check after a type check after a value check: interleaved, left alone.
        let mut found_type = false;
        let mut other_after_type = false;
        for unit in &units {
            if is_type(unit) {
                if other_after_type {
                    needs = false;
                    break;
                }
                found_type = true;
            } else if found_type {
                other_after_type = true;
            }
        }
    }
    if keep || types.is_empty() || others.is_empty() || !needs {
        result.append(segment);
        return;
    }
    result.extend(types.into_iter().chain(others).flatten());
    segment.clear();
}
