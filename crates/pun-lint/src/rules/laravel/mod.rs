//! The `laravel` group: rules ported from rector-laravel 2.5.0 (driftingly/rector-laravel, src/Rector/...), each in
//! its own file, and what they share.
//!
//! rector-laravel builds new nodes with Rector's `NodeFactory` and prints them with php-parser's printer: a new call is
//! pretty-printed (`name(arg, arg)`), a class it names is fully qualified (`\Illuminate\Support\Env`), and the nodes it
//! keeps from the old code keep their own text. The rules here write the same text.

mod conditional_helpers;
mod names;
mod no_negated_blank_filled;
mod no_request_all;
mod prefer_abort_if;
mod prefer_back_helper;
mod prefer_casts_method;
mod prefer_env_helper;
mod prefer_exception_class_string;
mod prefer_facade_over_app;
mod prefer_nullsafe;
mod prefer_report_if;
mod prefer_request_server;
mod prefer_session_facade;
mod prefer_sleep_facade;
mod prefer_str_starts_with;
mod prefer_throw_if;
mod prefer_to_route_helper;
mod prefer_today;

pub use no_negated_blank_filled::NoNegatedBlankFilled;
pub use no_request_all::NoRequestAll;
pub use prefer_abort_if::PreferAbortIf;
pub use prefer_back_helper::PreferBackHelper;
pub use prefer_casts_method::PreferCastsMethod;
pub use prefer_env_helper::PreferEnvHelper;
pub use prefer_exception_class_string::PreferExceptionClassString;
pub use prefer_facade_over_app::PreferFacadeOverApp;
pub use prefer_nullsafe::PreferNullsafe;
pub use prefer_report_if::PreferReportIf;
pub use prefer_request_server::PreferRequestServer;
pub use prefer_session_facade::PreferSessionFacade;
pub use prefer_sleep_facade::PreferSleepFacade;
pub use prefer_str_starts_with::PreferStrStartsWith;
pub use prefer_throw_if::PreferThrowIf;
pub use prefer_to_route_helper::PreferToRouteHelper;
pub use prefer_today::PreferToday;

use mago_syntax::cst::{
    ArgumentList, Assignment, AssignmentOperator, Call, ClassLikeMemberSelector, CompositeString,
    Expression, Node, UnaryPrefixOperator, Variable,
};
use pun_problem::{Edit, Fix, Safety, Span};

use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};
use names::resolve_class_name;

/// The group's rules, in a fixed order.
pub(crate) fn all() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(NoNegatedBlankFilled),
        Box::new(PreferBackHelper),
        Box::new(PreferToRouteHelper),
        Box::new(PreferEnvHelper),
        Box::new(PreferExceptionClassString),
        Box::new(PreferSleepFacade),
        Box::new(PreferToday),
        Box::new(PreferRequestServer),
        Box::new(PreferStrStartsWith),
        Box::new(PreferSessionFacade),
        Box::new(PreferAbortIf),
        Box::new(PreferReportIf),
        Box::new(PreferThrowIf),
        Box::new(PreferFacadeOverApp),
        Box::new(NoRequestAll),
        Box::new(PreferCastsMethod),
    ]
}

/// `NodeNameResolver::isName` on an identifier (a method name): ASCII case-insensitive.
pub(crate) fn is_name(name: &[u8], wanted: &str) -> bool {
    name.eq_ignore_ascii_case(wanted.as_bytes())
}

/// The name of a method call written as a plain identifier (`->name(...)`), not `->$name(...)` or `->{...}(...)`.
pub(crate) fn member_name<'a>(selector: &ClassLikeMemberSelector<'a>) -> Option<&'a [u8]> {
    match selector {
        ClassLikeMemberSelector::Identifier(id) => Some(id.value),
        _ => None,
    }
}

/// The fully qualified class name (without the leading `\`) of a static call's or `new`'s class written as a name;
/// `self`, `static`, `parent` and expressions have none.
pub(crate) fn class_name(expr: &Expression<'_>, ancestors: &[Node<'_, '_>]) -> Option<String> {
    match expr {
        Expression::Identifier(id) => Some(resolve_class_name(ancestors, id)),
        _ => None,
    }
}

/// Whether the expression is the variable `$name` (`name` without the `$`).
pub(crate) fn is_variable(expr: &Expression<'_>, name: &str) -> bool {
    matches!(unparenthesize(expr), Expression::Variable(Variable::Direct(v)) if &v.name[1..] == name.as_bytes())
}

/// The text of each argument, as php-parser's printer keeps an argument node: the source from its name or `...` to the
/// end of its value.
pub(crate) fn argument_texts<'s>(ctx: &Context<'s>, list: &ArgumentList<'_>) -> Vec<&'s str> {
    list.arguments
        .iter()
        .map(|a| {
            let s = node_span(a);
            &ctx.source[s.start..s.end]
        })
        .collect()
}

/// `name(arg, arg)`: a call php-parser's printer writes for a new call node over kept arguments.
pub(crate) fn call(name: &str, args: &[&str]) -> String {
    format!("{name}({})", args.join(", "))
}

/// The nearest ancestor that is a different node from the one at `span` (Mago wraps a node in `Expression`, `Call`,
/// `Access` and the like, all with the same span).
fn parent<'n, 'ast, 'arena>(
    span: Span,
    ancestors: &'n [Node<'ast, 'arena>],
) -> Option<(usize, &'n Node<'ast, 'arena>)> {
    ancestors
        .iter()
        .enumerate()
        .rev()
        .find(|(_, a)| node_span(*a) != span && !matches!(a, Node::Parenthesized(_)))
}

/// Whether the expression at `span` is written to, so that a call put in its place would not parse: an assignment's
/// target (also inside a destructuring list), a reference, `++`/`--`, `isset()`, `unset()` or a `foreach` target.
/// Rector rewrites some of these into code PHP rejects; Pun leaves them.
pub(crate) fn is_written(span: Span, ancestors: &[Node<'_, '_>]) -> bool {
    let mut at = span;
    let mut index = ancestors.len();
    loop {
        let Some((i, node)) = parent(at, &ancestors[..index]) else {
            return false;
        };
        match node {
            Node::Assignment(Assignment { lhs, .. }) => return node_span(*lhs) == at,
            Node::UnaryPrefix(u) => {
                return matches!(
                    u.operator,
                    UnaryPrefixOperator::Reference(_)
                        | UnaryPrefixOperator::PreIncrement(_)
                        | UnaryPrefixOperator::PreDecrement(_)
                );
            }
            Node::UnaryPostfix(_) | Node::IssetConstruct(_) | Node::Unset(_) => return true,
            Node::Foreach(foreach) => {
                let target = node_span(&foreach.target);
                return target.start <= at.start && at.end <= target.end;
            }
            // `$a['x']['y'] = ...` writes to `$a['x']` too.
            Node::ArrayAccess(access) if node_span(access.array) == at => {
                at = node_span(node);
                index = i;
            }
            // An item of `[...] = ` or `list(...) =`: go on up to the assignment.
            Node::ArrayElement(_)
            | Node::KeyValueArrayElement(_)
            | Node::ValueArrayElement(_)
            | Node::Array(_)
            | Node::List(_) => {
                at = node_span(node);
                index = i;
            }
            _ => return false,
        }
    }
}

/// Whether the node at `span` lies in a string (`"{$a['x']}"`, a heredoc or a backtick command), as php-parser's
/// `InterpolatedString` or `ShellExec` parts.
pub(crate) fn in_string(ancestors: &[Node<'_, '_>]) -> bool {
    ancestors.iter().any(|a| {
        matches!(
            a,
            Node::CompositeString(
                CompositeString::Interpolated(_)
                    | CompositeString::Document(_)
                    | CompositeString::ShellExecute(_)
            )
        )
    })
}

/// rector-laravel's `ArrayDimFetchContextNodeVisitor::IS_IN_SUPERGLOBAL_ASSIGN`: the node lies in an assignment to
/// `$_SERVER`, `$_GET`, `$_POST`, `$_REQUEST` or `$_ENV` (or one of their items), in `isset()`, in `unset()` or in an
/// interpolated string.
pub(crate) fn in_superglobal_assign(ancestors: &[Node<'_, '_>]) -> bool {
    const SUPERGLOBALS: [&str; 5] = ["_SERVER", "_GET", "_POST", "_REQUEST", "_ENV"];
    ancestors.iter().any(|a| match a {
        Node::Assignment(assign) if matches!(assign.operator, AssignmentOperator::Assign(_)) => {
            let var = match unparenthesize(assign.lhs) {
                Expression::ArrayAccess(access) => access.array,
                other => other,
            };
            SUPERGLOBALS.iter().any(|name| is_variable(var, name))
        }
        Node::IssetConstruct(_) | Node::Unset(_) => true,
        Node::CompositeString(CompositeString::Interpolated(_) | CompositeString::Document(_)) => {
            true
        }
        _ => false,
    })
}

/// The object of a method call when it is a call of the global function `name` with no arguments.
pub(crate) fn is_bare_call(expr: &Expression<'_>, name: &str) -> bool {
    matches!(
        unparenthesize(expr),
        Expression::Call(Call::Function(f))
            if super::rector_code_quality::named(f, name) && f.argument_list.arguments.is_empty()
    )
}

/// A report on `span` whose fix writes `code` there.
pub(crate) fn replace(span: Span, message: String, code: String) -> Report {
    Report {
        span,
        message,
        help: None,
        fix: Some(Fix {
            safety: Safety::Unsafe,
            edits: vec![Edit::new(span, code)],
        }),
    }
}

/// `RedirectBackToBackHelperRector` and `RedirectRouteToToRouteHelperRector`: `redirect()->method(...)` (the helper
/// called with no arguments) and `Redirect::method(...)` (the `Illuminate\Support\Facades\Redirect` facade) become
/// `helper(...)`, with the method's arguments.
pub(crate) fn redirect_helper(
    node: Node<'_, '_>,
    ancestors: &[Node<'_, '_>],
    ctx: &Context<'_>,
    method: &str,
    helper: &str,
    out: &mut Vec<Report>,
) {
    let (span, list) = match node {
        Node::MethodCall(m)
            if member_name(&m.method).is_some_and(|n| is_name(n, method))
                && is_bare_call(m.object, "redirect") =>
        {
            (node_span(m), &m.argument_list)
        }
        Node::StaticMethodCall(s)
            if member_name(&s.method).is_some_and(|n| is_name(n, method))
                && class_name(s.class, ancestors).is_some_and(|c| {
                    c.eq_ignore_ascii_case("Illuminate\\Support\\Facades\\Redirect")
                }) =>
        {
            (node_span(s), &s.argument_list)
        }
        _ => return,
    };
    out.push(replace(
        span,
        format!("Use `{helper}()` instead of redirecting with `{method}()`."),
        call(helper, &argument_texts(ctx, list)),
    ));
}
