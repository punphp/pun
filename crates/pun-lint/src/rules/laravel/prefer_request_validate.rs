//! `lint/laravel/prefer-request-validate`, ported from rector-laravel 2.5.0's `RequestStaticValidateToInjectRector`
//! (src/Rector/StaticCall).
//!
//! In a method of a class, a static call on the request (`Illuminate\Http\Request` or its global alias `Request`, so
//! `Request::validate(...)`) becomes a call on the method's request parameter (`$request->validate(...)`), and so does
//! the global `request()` helper: `request()` becomes `$request`, `request(...)` becomes `$request->input(...)`. The
//! helper is replaced only in a method that implements or overrides none of an ancestor's (its prototype is its own).
//! The parameter is the first one typed as a request; without one, `\Illuminate\Http\Request $request` is added last.
//!
//! Rector also replaces the calls inside a closure, a nested function or an anonymous class, where `$request` is
//! not defined; adds a second `$request` parameter next to an untyped one; uses a variadic request parameter as one
//! request; and adds a parameter after a variadic one. Pun leaves those calls and methods.

use mago_syntax::cst::{
    ArgumentList, Expression, FunctionCall, FunctionLikeParameter, FunctionLikeParameterList,
    Identifier, Method, MethodBody, Node, NodeKind, StaticMethodCall,
};
use pun_problem::{Edit, Safety, Span};

use super::member_name;
use super::object_type::{is_hint_type, is_object_type};
use super::print::kept_arguments;
use crate::rules::rector_code_quality::named;
use crate::rules::types::facts::facts;
use crate::rules::types::{class_scope, namespace_of};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

const REQUEST: &str = "Illuminate\\Http\\Request";

pub struct PreferRequestValidate;

/// A call to replace: the global `request()` helper, or a static call on the request; a static call whose method is
/// not written as a name only makes Rector add the parameter.
enum Found<'a> {
    Helper(&'a FunctionCall<'a>),
    Static(&'a StaticMethodCall<'a>, &'a [u8]),
    Dynamic,
}

impl Rule for PreferRequestValidate {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-request-validate"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Method(method) = node else { return };
        let MethodBody::Concrete(body) = &method.body else {
            return;
        };
        // `$classReflection->isClass()`
        let is_class = ancestors.iter().rev().find_map(|a| match a {
            Node::Class(_) | Node::AnonymousClass(_) => Some(true),
            Node::Interface(_) | Node::Trait(_) | Node::Enum(_) => Some(false),
            _ => None,
        });
        if is_class != Some(true) {
            return;
        }
        let own_prototype = !overrides(ctx, ancestors, method);
        let mut found = Vec::new();
        collect(Node::Block(body), &mut |n| match n {
            Node::FunctionCall(f)
                if own_prototype && named(f, "request") && !namespaced(ctx, ancestors, f) =>
            {
                found.push(Found::Helper(f));
                true
            }
            Node::StaticMethodCall(s)
                if is_object_type(ctx, ancestors, s.class, REQUEST)
                    || is_object_type(ctx, ancestors, s.class, "Request") =>
            {
                let name = member_name(&s.method);
                found.push(name.map_or(Found::Dynamic, |name| Found::Static(s, name)));
                name.is_some()
            }
            _ => false,
        });
        if found.is_empty() {
            return;
        }
        let params: Vec<&FunctionLikeParameter<'_>> =
            method.parameter_list.parameters.iter().collect();
        let mut edits = Vec::new();
        let var = match params.iter().find(|p| {
            p.hint
                .as_ref()
                .is_some_and(|h| is_hint_type(ctx, ancestors, h, REQUEST))
        }) {
            Some(p) if p.ellipsis.is_some() => return,
            Some(p) => String::from_utf8_lossy(p.variable.name).into_owned(),
            None => {
                if params.iter().any(|p| p.variable.name == b"$request")
                    || params.last().is_some_and(|p| p.ellipsis.is_some())
                {
                    return;
                }
                edits.push(add_param(ctx.source, &method.parameter_list, &params));
                "$request".to_string()
            }
        };
        for call in found {
            edits.push(match call {
                Found::Helper(f) if f.argument_list.arguments.is_empty() => {
                    Edit::new(node_span(f), var.clone())
                }
                Found::Helper(f) => Edit::new(
                    node_span(f),
                    method_call(ctx, &var, "input", &f.argument_list, node_span(f).start),
                ),
                Found::Static(s, name) => Edit::new(
                    node_span(s),
                    method_call(
                        ctx,
                        &var,
                        &String::from_utf8_lossy(name),
                        &s.argument_list,
                        node_span(s).start,
                    ),
                ),
                Found::Dynamic => continue,
            });
        }
        if edits.is_empty() {
            return;
        }
        out.push(Report::fixed(node_span(&method.name), "Use the injected request instead of the `request()` helper or a static `Request` call.", Safety::Unsafe, edits));
    }
}

/// `$var->name(args)`, as php-parser prints a new call at `at` over kept arguments.
fn method_call(
    ctx: &Context<'_>,
    var: &str,
    name: &str,
    list: &ArgumentList<'_>,
    at: usize,
) -> String {
    format!(
        "{var}->{name}({})",
        kept_arguments(ctx.source, list, at).join(", ")
    )
}

/// The calls `pick` takes, outermost first, in the method's own scope: arrow functions and an anonymous class's
/// arguments count, a closure, a function and a class's body do not. A taken call's arguments are not searched.
fn collect<'ast, 'arena>(
    node: Node<'ast, 'arena>,
    pick: &mut dyn FnMut(Node<'ast, 'arena>) -> bool,
) {
    node.visit_children(|child| {
        if pick(child) {
            return;
        }
        match child {
            Node::Function(_)
            | Node::Closure(_)
            | Node::Class(_)
            | Node::Interface(_)
            | Node::Trait(_)
            | Node::Enum(_) => {}
            Node::AnonymousClass(class) => {
                if let Some(args) = &class.argument_list {
                    collect(Node::PartialArgumentList(args), pick);
                }
            }
            _ => collect(child, pick),
        }
    });
}

/// Whether an unqualified `request()` in a namespace calls that namespace's own `request` function, which php-parser's
/// name resolution prefers when it exists.
fn namespaced(ctx: &Context<'_>, ancestors: &[Node<'_, '_>], call: &FunctionCall<'_>) -> bool {
    let (Expression::Identifier(Identifier::Local(_)), Some(ns)) =
        (call.function, namespace_of(ancestors))
    else {
        return false;
    };
    facts(ctx)
        .codebase
        .function_exists(format!("{ns}\\request").as_bytes())
}

/// Whether the method's prototype (`getPrototype()->getDeclaringClass()`) is another class's: an ancestor class has a
/// non-private method of that name (for the constructor, an abstract one), or an interface of the class has one.
fn overrides(ctx: &Context<'_>, ancestors: &[Node<'_, '_>], method: &Method<'_>) -> bool {
    let parents = class_scope(ancestors).map_or(Vec::new(), |s| s.parents);
    let facts = facts(ctx);
    let codebase = &facts.codebase;
    let constructor = method.name.value.eq_ignore_ascii_case(b"__construct");
    let declared_in = |class: &str| {
        let interface = codebase
            .get_class_like(class.as_bytes())
            .is_some_and(|c| c.kind.is_interface());
        codebase
            .get_method(class.as_bytes(), method.name.value)
            .and_then(|m| m.method_metadata.as_ref())
            .is_some_and(|m| {
                interface || (!m.visibility.is_private() && (!constructor || m.is_abstract))
            })
    };
    parents.iter().any(|p| {
        declared_in(p)
            || codebase.get_class_like(p.as_bytes()).is_some_and(|c| {
                (c.all_parent_classes.iter())
                    .chain(c.all_parent_interfaces.iter())
                    .any(|a| declared_in(&a.to_string()))
            })
    })
}

/// The edit that appends `\Illuminate\Http\Request $request` to the parameters, as php-parser's format-preserving
/// printer inserts into a list: right after `(` in an empty one, else after the last parameter, with `, `, or with
/// `,` and a new line at the last parameter's indentation when every gap between the parameters holds a new line.
fn add_param(
    source: &str,
    list: &FunctionLikeParameterList<'_>,
    params: &[&FunctionLikeParameter<'_>],
) -> Edit {
    const PARAM: &str = "\\Illuminate\\Http\\Request $request";
    let Some(last) = params.last() else {
        let at = list.left_parenthesis.end.offset as usize;
        return Edit::new(Span::new(at, at), PARAM);
    };
    let end = node_span(*last).end;
    let multiline = params.len() >= 2
        && params
            .windows(2)
            .all(|w| source[node_span(w[0]).end..node_span(w[1]).start].contains('\n'));
    let text = if multiline {
        let start = node_span(*last).start;
        let line = source[..start].rfind('\n').map_or(0, |i| i + 1);
        let indent: String = source[line..start]
            .chars()
            .take_while(|c| *c == ' ' || *c == '\t')
            .collect();
        format!(",\n{indent}{PARAM}")
    } else {
        format!(", {PARAM}")
    };
    Edit::new(Span::new(end, end), text)
}
