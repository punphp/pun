//! `lint/dead-code/no-null-default-argument` and `lint/dead-code/no-null-default-named-argument`, ported from Rector
//! 2.6.1's `RemoveNullArgOnNullDefaultParamRector` and `RemoveNullNamedArgOnNullDefaultParamRector`
//! (rules/DeadCode/Rector/MethodCall) with their `CallLikeParamDefaultResolver` (rules/DeadCode/NodeAnalyzer).
//!
//! A `null` argument for a parameter whose default is `null` goes: the trailing positional ones (the first rule, on
//! calls without named arguments) or the named ones (the second rule). The callee is looked up in the codebase as
//! `ReflectionResolver` does; a variadic one (also one that reads `func_get_args()`) and `Ds\Map::get()` are left
//! alone.

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_syntax::cst::{Argument, ArgumentList, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety, Span};

use super::lint_b_helpers::{unpacked, value};
use super::rector_code_quality::is_constant;
use super::require_named_argument_order::{Target, callee};
use super::types::facts::facts;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

const TARGETS: &[NodeKind] = &[
    NodeKind::MethodCall,
    NodeKind::StaticMethodCall,
    NodeKind::Instantiation,
    NodeKind::FunctionCall,
];

pub struct NoNullDefaultArgument;

impl Rule for NoNullDefaultArgument {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-null-default-argument"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(TARGETS)
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some((list, target)) = call(node) else {
            return;
        };
        let args = list.arguments.as_slice();
        if args.is_empty()
            || args
                .iter()
                .any(|a| matches!(a, Argument::Named(_)) || unpacked(a))
        {
            return;
        }
        let facts = facts(ctx);
        let Some(nulls) =
            callee(&facts, target, ancestors).and_then(|c| null_positions(&facts.codebase, c))
        else {
            return;
        };
        let mut removed = Vec::new();
        for (position, arg) in args.iter().enumerate().rev() {
            if !is_constant(value(arg), "null") || !nulls.contains(&position) {
                break;
            }
            removed.push(position);
        }
        report(node, list, removed, out);
    }
}

pub struct NoNullDefaultNamedArgument;

impl Rule for NoNullDefaultNamedArgument {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-null-default-named-argument"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(TARGETS)
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some((list, target)) = call(node) else {
            return;
        };
        let args = list.arguments.as_slice();
        // An unpacked argument hides which parameters are bound.
        if args.iter().any(unpacked) || !args.iter().any(|a| matches!(a, Argument::Named(_))) {
            return;
        }
        let facts = facts(ctx);
        let Some(callee) = callee(&facts, target, ancestors) else {
            return;
        };
        let Some(nulls) = null_positions(&facts.codebase, callee) else {
            return;
        };
        let removed = args
            .iter()
            .enumerate()
            .filter(|(_, arg)| {
                let Argument::Named(named) = arg else {
                    return false;
                };
                // `resolvePositionParameterByName`: the parameter of that exact name.
                is_constant(named.value, "null")
                    && callee
                        .parameters
                        .iter()
                        .position(|p| p.get_name().0.as_bytes().get(1..) == Some(named.name.value))
                        .is_some_and(|p| nulls.contains(&p))
            })
            .map(|(i, _)| i)
            .collect();
        report(node, list, removed, out);
    }
}

/// The argument list of a call and what it calls.
fn call<'a>(node: Node<'a, 'a>) -> Option<(&'a ArgumentList<'a>, Target<'a>)> {
    match node {
        Node::MethodCall(c) => Some((&c.argument_list, Target::Method(c.object, &c.method))),
        Node::StaticMethodCall(c) => Some((&c.argument_list, Target::Static(c.class, &c.method))),
        Node::FunctionCall(c) => Some((&c.argument_list, Target::Function(c.function))),
        Node::Instantiation(new) => Some((new.argument_list.as_ref()?, Target::New(new.class))),
        _ => None,
    }
}

/// `resolveNullPositions`: the positions of the parameters whose default is `null`; `None` for `Ds\Map::get()` and
/// for a variadic callee.
fn null_positions(
    codebase: &CodebaseMetadata,
    callee: &FunctionLikeMetadata,
) -> Option<Vec<usize>> {
    let ds_map_get = codebase
        .get_method(b"Ds\\Map", b"get")
        .is_some_and(|m| std::ptr::eq(m, callee));
    if ds_map_get
        || callee.flags.uses_func_get_args()
        || callee.parameters.iter().any(|p| p.flags.is_variadic())
    {
        return None;
    }
    Some(
        callee
            .parameters
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                p.default_type
                    .as_ref()
                    .is_some_and(|d| d.type_union.is_null())
            })
            .map(|(i, _)| i)
            .collect(),
    )
}

/// The report that removes the arguments at `removed` (any order): each run of them goes with the separator before
/// it, or, for a run at the start, with the text up to the next argument, as php-parser's printer removes them.
fn report(
    node: Node<'_, '_>,
    list: &ArgumentList<'_>,
    mut removed: Vec<usize>,
    out: &mut Vec<Report>,
) {
    if removed.is_empty() {
        return;
    }
    removed.sort_unstable();
    let spans: Vec<Span> = list.arguments.iter().map(node_span).collect();
    let mut edits = Vec::new();
    let mut i = 0;
    while i < removed.len() {
        let first = removed[i];
        let mut last = first;
        while i + 1 < removed.len() && removed[i + 1] == last + 1 {
            i += 1;
            last += 1;
        }
        i += 1;
        let span = if first > 0 {
            Span::new(spans[first - 1].end, spans[last].end)
        } else if let Some(next) = spans.get(last + 1) {
            Span::new(spans[0].start, next.start)
        } else {
            Span::new(spans[0].start, spans[last].end)
        };
        edits.push(Edit::new(span, ""));
    }
    out.push(Report {
        span: node_span(&node),
        message: "Remove the `null` argument: the parameter's default is already `null`."
            .to_string(),
        help: None,
        fix: Some(Fix {
            safety: Safety::Safe,
            edits,
        }),
    });
}
