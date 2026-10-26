//! `lint/pest/prefer-extend`, ported from pest-plugin-rector's `UsesToExtendRector` (Pest2ToPest3): `uses(...)` and
//! `pest()->uses(...)` become `pest()->extend(...)` for classes and `pest()->use(...)` for traits.

use mago_codex::symbol::SymbolKind;
use mago_syntax::cst::{
    Access, ArgumentList, Expression, FunctionCall, MethodCall, Node, NodeKind,
};
use pun_problem::{Edit, Safety, Span};

use super::{
    argument_value, function_name, indent_at, is_name, kept_text, method_call, print_args,
    print_member, report, root_func_call,
};
use crate::rules::lint_b_names::resolve_class;
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferExtend;

impl Rule for PreferExtend {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-extend"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall, NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        // Rector goes down from the outermost call of a chain, and the call it replaces takes the ones below it.
        let span = match node {
            Node::MethodCall(call) => node_span(call),
            Node::FunctionCall(call) => node_span(call),
            _ => return,
        };
        if object_of_parent_call(ancestors, span) {
            return;
        }
        let indent = indent_at(ctx, span.start);
        let printed = match node {
            Node::FunctionCall(call) if is_uses(call) => {
                transform(ctx, ancestors, None, &call.argument_list, &[], indent)
            }
            Node::MethodCall(call) => refactor_method_call(ctx, ancestors, call, indent),
            _ => None,
        };
        let Some(printed) = printed else {
            return;
        };
        out.push(report(
            span,
            "Use `pest()->extend()` and `pest()->use()` instead of `uses()`.".to_string(),
            Safety::Safe,
            vec![Edit::new(span, printed)],
        ));
    }
}

/// The node at `span` is the object of a method call (through parentheses), which Rector visits first.
fn object_of_parent_call(ancestors: &[Node<'_, '_>], span: Span) -> bool {
    let parent = ancestors
        .iter()
        .rev()
        .find(|a| node_span(*a) != span && !matches!(a, Node::Parenthesized(_)));
    matches!(parent, Some(Node::MethodCall(m)) if node_span(unparenthesize(m.object)) == span)
}

/// `isName($funcCall, 'uses')`.
fn is_uses(call: &FunctionCall<'_>) -> bool {
    function_name(call).is_some_and(|n| n.eq_ignore_ascii_case(b"uses"))
}

/// `refactorMethodCall`.
fn refactor_method_call(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    node: &MethodCall<'_>,
    indent: usize,
) -> Option<String> {
    // A chain on `uses(...)`: all its method calls go after the new ones.
    let mut calls = vec![node];
    let mut current = node.object;
    while let Some(m) = method_call(current) {
        calls.push(m);
        current = m.object;
    }
    if let Expression::Call(mago_syntax::cst::Call::Function(f)) = unparenthesize(current)
        && is_uses(f)
    {
        calls.reverse();
        return transform(ctx, ancestors, None, &f.argument_list, &calls, indent);
    }
    // A chain on `pest()` with a `->uses(...)` call: the calls after it go after the new ones, the ones before go.
    let uses_index = calls
        .iter()
        .position(|c| is_name(&c.method, "uses") && pest_call(c).is_some())?;
    let after = calls
        .iter()
        .position(|c| is_name(&c.method, "uses"))
        .unwrap_or(calls.len());
    let mut methods: Vec<&MethodCall<'_>> = calls[..after].to_vec();
    methods.reverse();
    let uses = calls[uses_index];
    transform(
        ctx,
        ancestors,
        pest_call(uses),
        &uses.argument_list,
        &methods,
        indent,
    )
}

/// `getPestFuncCall`: the call of `pest()` at the root of the chain.
fn pest_call<'a>(call: &'a MethodCall<'a>) -> Option<&'a FunctionCall<'a>> {
    root_func_call(call)
        .filter(|f| function_name(f).is_some_and(|n| n.eq_ignore_ascii_case(b"pest")))
}

/// `transformUsesArgs` / `transformPestUsesCall`: `pest()` (a new one, or the chain's own) with `->extend()` for the
/// classes, `->use()` for the traits, and then `methods`.
fn transform(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    pest: Option<&FunctionCall<'_>>,
    arguments: &ArgumentList<'_>,
    methods: &[&MethodCall<'_>],
    indent: usize,
) -> Option<String> {
    let (classes, traits) = classes_and_traits(ctx, ancestors, arguments, indent);
    if classes.is_empty() && traits.is_empty() {
        return None;
    }
    let mut out = match pest {
        Some(call) => kept_text(ctx, node_span(call), indent),
        None => "pest()".to_string(),
    };
    if !classes.is_empty() {
        out.push_str(&format!("->extend({})", classes.join(", ")));
    }
    if !traits.is_empty() {
        out.push_str(&format!("->use({})", traits.join(", ")));
    }
    for m in methods {
        out.push_str(&format!(
            "->{}({})",
            print_member(ctx, &m.method, indent),
            print_args(ctx, &m.argument_list, indent)
        ));
    }
    Some(out)
}

/// `separateClassesAndTraits`: the `X::...` arguments on a written class name, as printed, split by whether the
/// class (resolved by the file's imports) is a known trait.
fn classes_and_traits(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    arguments: &ArgumentList<'_>,
    indent: usize,
) -> (Vec<String>, Vec<String>) {
    let facts = facts(ctx);
    let (mut classes, mut traits) = (Vec::new(), Vec::new());
    for argument in arguments.arguments.iter() {
        let value = unparenthesize(argument_value(argument));
        let Expression::Access(Access::ClassConstant(fetch)) = value else {
            continue;
        };
        // `$classConstFetch->class instanceof Name`: `static`, `self` and `parent` are names too, of no class.
        let resolved = match unparenthesize(fetch.class) {
            Expression::Identifier(name) => resolve_class(ancestors, name),
            Expression::Static(_) | Expression::Self_(_) | Expression::Parent(_) => Vec::new(),
            _ => continue,
        };
        let text = kept_text(ctx, node_span(value), indent);
        match facts.codebase.get_class_like(&resolved) {
            Some(class) if class.kind == SymbolKind::Trait => traits.push(text),
            _ => classes.push(text),
        }
    }
    (classes, traits)
}
