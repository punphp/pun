//! `lint/complexity/require-named-argument-order`, ported from Rector 2.6.1's `SortCallLikeNamedArgsRector`
//! (rules/CodeQuality/Rector/FuncCall) and its `NamedArgsSorter` (rules/CodeQuality/NodeManipulator).
//!
//! The named arguments of a function, method, static method or constructor call move into the order of the
//! parameters they name; positional arguments stay first, and names the callee does not declare go last. The callee
//! is looked up in the codebase (the built-in symbols and the project's), as Rector's `ReflectionResolver` does.

use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_syntax::cst::{
    Argument, ArgumentList, ClassLikeMemberSelector, Expression, Identifier, Node, NodeKind,
    Variable,
};
use pun_problem::{Edit, Fix, Safety, Span};

use super::rector_batch5::resolve_class_name;
use super::types::class_scope;
use super::types::facts::{Facts, facts};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct RequireNamedArgumentOrder;

impl Rule for RequireNamedArgumentOrder {
    fn id(&self) -> &'static str {
        "lint/complexity/require-named-argument-order"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::MethodCall,
            NodeKind::StaticMethodCall,
            NodeKind::Instantiation,
            NodeKind::FunctionCall,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (list, target) = match node {
            Node::MethodCall(call) => (
                &call.argument_list,
                Target::Method(call.object, &call.method),
            ),
            Node::StaticMethodCall(call) => (
                &call.argument_list,
                Target::Static(call.class, &call.method),
            ),
            Node::FunctionCall(call) => (&call.argument_list, Target::Function(call.function)),
            Node::Instantiation(new) => match &new.argument_list {
                Some(list) => (list, Target::New(new.class)),
                None => return,
            },
            _ => return,
        };
        let args = arguments(list);
        if args.len() <= 1 || args.iter().all(|(_, name)| name.is_none()) {
            return;
        }
        let facts = facts(ctx);
        let Some(callee) = callee(&facts, target, ancestors) else {
            return;
        };
        let Some(spans) = sorted(&args, callee) else {
            return;
        };
        // php-parser's format-preserving printer moves each argument into the place of another.
        let edits = args
            .iter()
            .zip(spans)
            .filter(|(old, new)| old.0 != *new)
            .map(|(old, new)| Edit::new(old.0, &ctx.source[new.start..new.end]))
            .collect();
        out.push(report(node_span(&node), edits));
    }
}

/// Each argument's span and, for a named one, its name.
fn arguments<'a>(list: &'a ArgumentList<'a>) -> Vec<(Span, Option<&'a [u8]>)> {
    list.arguments
        .iter()
        .map(|arg| match arg {
            Argument::Named(named) => (node_span(arg), Some(named.name.value)),
            Argument::Positional(_) => (node_span(arg), None),
        })
        .collect()
}

/// The report of a rule that sorts named arguments.
pub(super) fn report(span: Span, edits: Vec<Edit>) -> Report {
    Report {
        span,
        message: "Order the named arguments as the parameters are declared.".to_string(),
        help: None,
        fix: Some(Fix {
            safety: Safety::Safe,
            edits,
        }),
    }
}

/// `NamedArgsSorter::sortArgsToMatchReflectionParameters`: the spans of `args` with the named arguments in the order
/// of `callee`'s parameters, after the others; `None` when they already are in that order.
pub(super) fn sorted(
    args: &[(Span, Option<&[u8]>)],
    callee: &FunctionLikeMetadata,
) -> Option<Vec<Span>> {
    let order = |name: &[u8]| {
        callee
            .parameters
            .iter()
            .position(|p| p.get_name().0.as_bytes().get(1..) == Some(name))
            .unwrap_or(usize::MAX)
    };
    let mut named: Vec<&(Span, Option<&[u8]>)> =
        args.iter().filter(|(_, name)| name.is_some()).collect();
    // `usort` is stable since PHP 8.0.
    named.sort_by_key(|(_, name)| order(name.unwrap()));
    let spans: Vec<Span> = args
        .iter()
        .filter(|(_, name)| name.is_none())
        .chain(named)
        .map(|(span, _)| *span)
        .collect();
    (spans.iter().zip(args).any(|(new, old)| *new != old.0)).then_some(spans)
}

/// What a call calls: a function name, a method of an object or a class, or a class's constructor.
pub(super) enum Target<'a> {
    Function(&'a Expression<'a>),
    Method(&'a Expression<'a>, &'a ClassLikeMemberSelector<'a>),
    Static(&'a Expression<'a>, &'a ClassLikeMemberSelector<'a>),
    New(&'a Expression<'a>),
}

/// `ReflectionResolver::resolveFunctionLikeReflectionFromCall`: the function or method `target` calls.
pub(super) fn callee<'f>(
    facts: &'f Facts,
    target: Target<'_>,
    ancestors: &[Node<'_, '_>],
) -> Option<&'f FunctionLikeMetadata> {
    let codebase = &facts.codebase;
    let (class, method) = match target {
        Target::Function(function) => {
            let Expression::Identifier(name) = function else {
                return None;
            };
            return function_names(ancestors, name)
                .iter()
                .find_map(|n| codebase.get_function(n.as_bytes()));
        }
        Target::Method(object, ClassLikeMemberSelector::Identifier(method)) => {
            (object_class(facts, ancestors, object)?, method.value)
        }
        Target::Static(class, ClassLikeMemberSelector::Identifier(method)) => {
            (class_name(ancestors, class)?, method.value)
        }
        Target::New(class) => (class_name(ancestors, class)?, &b"__construct"[..]),
        Target::Method(..) | Target::Static(..) => return None,
    };
    codebase.get_declaring_method(class.as_bytes(), method)
}

/// The class a class reference names: a name, `self`, `static` or `parent`.
fn class_name(ancestors: &[Node<'_, '_>], class: &Expression<'_>) -> Option<String> {
    match class {
        Expression::Identifier(id) => Some(resolve_class_name(ancestors, id)),
        Expression::Self_(_) | Expression::Static(_) => class_scope(ancestors)?.name,
        Expression::Parent(_) => class_scope(ancestors)?.parent_class,
        _ => None,
    }
}

/// `ClassNameFromObjectTypeResolver::resolve` on the type of a method call's object: the one class of an object type
/// (`null` left out).
fn object_class(
    facts: &Facts,
    ancestors: &[Node<'_, '_>],
    object: &Expression<'_>,
) -> Option<String> {
    if let Expression::Variable(Variable::Direct(v)) = unparenthesize(object)
        && v.name == b"$this"
    {
        return class_scope(ancestors)?.name;
    }
    let ty = facts.native_type(node_span(object))?;
    let mut classes = ty.types.iter().filter(|t| !matches!(t, TAtomic::Null));
    match (classes.next(), classes.next()) {
        (Some(TAtomic::Object(TObject::Named(named))), None) => {
            Some(String::from_utf8_lossy(named.name.as_bytes()).into_owned())
        }
        _ => None,
    }
}

/// The functions a call of `name` may reach, in the order PHP tries them: the namespaced name, then the global one.
fn function_names(ancestors: &[Node<'_, '_>], name: &Identifier<'_>) -> Vec<String> {
    let written = String::from_utf8_lossy(name.value()).into_owned();
    if let Some(fq) = written.strip_prefix('\\') {
        return vec![fq.to_string()];
    }
    let namespace = ancestors.iter().rev().find_map(|a| match a {
        Node::Namespace(ns) => ns
            .name
            .as_ref()
            .map(|n| String::from_utf8_lossy(n.value()).into_owned()),
        _ => None,
    });
    match namespace {
        Some(ns) if written.contains('\\') => vec![format!("{ns}\\{written}")],
        Some(ns) => vec![format!("{ns}\\{written}"), written],
        None => vec![written],
    }
}
