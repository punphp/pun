//! rector-laravel's `QueryBuilderAnalyzer` (src/NodeAnalyzer/QueryBuilderAnalyzer.php) on Mago's types, and what the
//! rules that type a query closure's parameter share.

use mago_syntax::cst::{Expression, FunctionLikeParameter, Hint, Node};
use pun_problem::{Fix, Safety};

use super::object_type::{is_object_type, name_class, object_class, special_class};
use super::print::set_param_type;
use super::{is_a, is_name, member_name};
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report};

const MODEL: &str = "Illuminate\\Database\\Eloquent\\Model";

/// A method or static call: its object or class, whether it is static, its name and its arguments.
pub(crate) struct CallParts<'a> {
    pub on: &'a Expression<'a>,
    pub is_static: bool,
    pub name: &'a [u8],
    pub arguments: Vec<&'a Expression<'a>>,
}

/// The parts of a `MethodCall` or `StaticMethodCall` node whose method is written as a name.
pub(crate) fn call_parts<'a>(n: &Node<'a, 'a>) -> Option<CallParts<'a>> {
    let (on, is_static, method, list) = match n {
        Node::StaticMethodCall(s) => (s.class, true, &s.method, &s.argument_list),
        _ => {
            let Node::MethodCall(m) = n else { return None };
            (m.object, false, &m.method, &m.argument_list)
        }
    };
    Some(CallParts {
        on,
        is_static,
        name: member_name(method)?,
        arguments: list.arguments.iter().map(|a| a.value()).collect(),
    })
}

/// `isMatchingCall($node, $method)`: the call is named `method` and is on a query builder
/// (`Illuminate\Contracts\Database\Query\Builder`), or is a static call a model forwards to its query builder.
pub(crate) fn is_matching_call(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    call: &CallParts<'_>,
    method: &str,
) -> bool {
    if !is_name(call.name, method) {
        return false;
    }
    if call.is_static {
        return is_proxy_call(ctx, ancestors, call);
    }
    is_object_type(
        ctx,
        ancestors,
        call.on,
        "Illuminate\\Contracts\\Database\\Query\\Builder",
    )
}

/// `isProxyCall`: a static call on a model class that has no native method of that name (it goes through
/// `__callStatic` to the query builder). The class is the one written, or the one of the object the expression holds.
fn is_proxy_call(ctx: &Context<'_>, ancestors: &[Node<'_, '_>], call: &CallParts<'_>) -> bool {
    let facts = facts(ctx);
    let class = match unparenthesize(call.on) {
        Expression::Identifier(id) => name_class(ancestors, id),
        e @ (Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_)) => {
            special_class(ancestors, e)
        }
        e => facts
            .full_type(node_span(e))
            .filter(|t| t.types.len() == 1)
            .and_then(|t| object_class(&t.types[0])),
    };
    class.is_some_and(|class| {
        is_a(ctx, &class, &[MODEL])
            && (facts.codebase)
                .get_declaring_method(class.as_bytes(), call.name)
                .is_none()
    })
}

/// The first parameter of the closure or arrow function `expr`, when it is one.
pub(crate) fn closure_first_param<'a>(
    expr: &'a Expression<'a>,
) -> Option<Option<&'a FunctionLikeParameter<'a>>> {
    match unparenthesize(expr) {
        Expression::Closure(c) => Some(c.parameter_list.parameters.first()),
        Expression::ArrowFunction(a) => Some(a.parameter_list.parameters.first()),
        _ => None,
    }
}

/// `$param->type instanceof Name`: a class name, `self` or `parent` (php-parser's `Identifier` holds the built-in
/// types and `static`).
fn is_name_type(param: &FunctionLikeParameter<'_>) -> bool {
    matches!(
        param.hint,
        Some(Hint::Identifier(_) | Hint::Self_(_) | Hint::Parent(_))
    )
}

/// The report that gives `param` the type `\class`, unless its type is already a name.
pub(crate) fn type_param(
    node: &Node<'_, '_>,
    param: &FunctionLikeParameter<'_>,
    class: &str,
) -> Option<Report> {
    if is_name_type(param) {
        return None;
    }
    Some(Report {
        span: node_span(node),
        message: format!("Type the query closure's parameter as `{class}`."),
        help: None,
        fix: Some(Fix {
            safety: Safety::Unsafe,
            edits: vec![set_param_type(param, &format!("\\{class}"))],
        }),
    })
}
