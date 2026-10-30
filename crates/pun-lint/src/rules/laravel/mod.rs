//! The `laravel` group: rules ported from rector-laravel 2.5.0 (driftingly/rector-laravel, src/Rector/...), each in
//! its own file, and what they share.
//!
//! rector-laravel builds new nodes with Rector's `NodeFactory` and prints them with php-parser's printer: a new call is
//! pretty-printed (`name(arg, arg)`), a class it names is fully qualified (`\Illuminate\Support\Env`), and the nodes it
//! keeps from the old code keep their own text. The rules here write the same text.

mod add_factory_extends_phpdoc;
mod add_has_factory_trait;
mod add_query_closure_type;
mod add_relation_generics;
mod add_relation_query_closure_type;
mod add_tap_closure_type;
mod conditional_helpers;
mod names;
mod no_app_array_access;
mod no_collection_aliases;
mod no_controller_middleware;
mod no_negated_blank_filled;
mod no_negated_contains;
mod no_negated_filter;
mod no_negated_when;
mod no_request_all;
mod no_useless_value_call;
mod no_useless_with_call;
mod object_type;
mod prefer_abort_if;
mod prefer_anonymous_migration;
mod prefer_back_helper;
mod prefer_casts_method;
mod prefer_collected_by_attribute;
mod prefer_class_container_key;
mod prefer_closure_only_bind;
mod prefer_collection_all;
mod prefer_constructor_injection;
mod prefer_date_facade;
mod prefer_default_argument;
mod prefer_dispatch_helper;
mod prefer_env_helper;
mod prefer_environment_check;
mod prefer_exception_class_string;
mod prefer_explicit_json_call;
mod prefer_facade_over_app;
mod prefer_latest_oldest;
mod prefer_nullsafe;
mod prefer_protected_model_methods;
mod prefer_query_builder_call;
mod prefer_report_if;
mod prefer_request_facade;
mod prefer_request_server;
mod prefer_request_validate;
mod prefer_resolve_helper;
mod prefer_scope_attribute;
mod prefer_session_facade;
mod prefer_sleep_facade;
mod prefer_status_assertion_method;
mod prefer_str_starts_with;
mod prefer_table_attribute;
mod prefer_throw_if;
mod prefer_to_route_helper;
mod prefer_today;
mod prefer_travel_to;
pub(crate) mod property_attribute;
mod property_rules;
mod prefer_typed_assert_closure;
mod prefer_validation_rule_array;
mod prefer_view_array;
mod print;
mod query_builder;

pub use add_factory_extends_phpdoc::AddFactoryExtendsPhpdoc;
pub use add_has_factory_trait::AddHasFactoryTrait;
pub use add_query_closure_type::AddQueryClosureType;
pub use add_relation_generics::AddRelationGenerics;
pub use add_relation_query_closure_type::AddRelationQueryClosureType;
pub use add_tap_closure_type::AddTapClosureType;
pub use no_app_array_access::NoAppArrayAccess;
pub use no_collection_aliases::NoCollectionAliases;
pub use no_controller_middleware::NoControllerMiddleware;
pub use no_negated_blank_filled::NoNegatedBlankFilled;
pub use no_negated_contains::NoNegatedContains;
pub use no_negated_filter::NoNegatedFilter;
pub use no_negated_when::NoNegatedWhen;
pub use no_request_all::NoRequestAll;
pub use no_useless_value_call::NoUselessValueCall;
pub use no_useless_with_call::NoUselessWithCall;
pub use prefer_abort_if::PreferAbortIf;
pub use prefer_anonymous_migration::PreferAnonymousMigration;
pub use prefer_back_helper::PreferBackHelper;
pub use prefer_casts_method::PreferCastsMethod;
pub use prefer_collected_by_attribute::PreferCollectedByAttribute;
pub use prefer_class_container_key::{ContainerService, PreferClassContainerKey};
pub use prefer_closure_only_bind::PreferClosureOnlyBind;
pub use prefer_collection_all::PreferCollectionAll;
pub use prefer_constructor_injection::{Injection, PreferConstructorInjection};
pub use prefer_date_facade::PreferDateFacade;
pub use prefer_default_argument::PreferDefaultArgument;
pub use prefer_dispatch_helper::PreferDispatchHelper;
pub use prefer_env_helper::PreferEnvHelper;
pub use prefer_environment_check::PreferEnvironmentCheck;
pub use prefer_exception_class_string::PreferExceptionClassString;
pub use prefer_explicit_json_call::PreferExplicitJsonCall;
pub use prefer_facade_over_app::PreferFacadeOverApp;
pub use prefer_latest_oldest::PreferLatestOldest;
pub use prefer_nullsafe::PreferNullsafe;
pub use prefer_protected_model_methods::PreferProtectedModelMethods;
pub use prefer_query_builder_call::PreferQueryBuilderCall;
pub use prefer_report_if::PreferReportIf;
pub use prefer_request_facade::PreferRequestFacade;
pub use prefer_request_server::PreferRequestServer;
pub use prefer_request_validate::PreferRequestValidate;
pub use prefer_resolve_helper::PreferResolveHelper;
pub use prefer_scope_attribute::PreferScopeAttribute;
pub use prefer_session_facade::PreferSessionFacade;
pub use prefer_sleep_facade::PreferSleepFacade;
pub use prefer_status_assertion_method::PreferStatusAssertionMethod;
pub use prefer_str_starts_with::PreferStrStartsWith;
pub use prefer_table_attribute::PreferTableAttribute;
pub use prefer_throw_if::PreferThrowIf;
pub use prefer_to_route_helper::PreferToRouteHelper;
pub use prefer_today::PreferToday;
pub use prefer_travel_to::PreferTravelTo;
pub use property_rules::{
    PreferAliasesAttribute, PreferAppendsAttribute, PreferBackoffAttribute,
    PreferCollectsAttribute, PreferCommandHiddenAttribute, PreferConnectionAttribute,
    PreferDateFormatAttribute, PreferDelayAttribute, PreferDeleteWhenMissingModelsAttribute,
    PreferDescriptionAttribute, PreferErrorBagAttribute, PreferFailOnTimeoutAttribute,
    PreferFillableAttribute, PreferGuardedAttribute, PreferHelpAttribute, PreferHiddenAttribute,
    PreferJobConnectionAttribute, PreferMaxExceptionsAttribute, PreferPreserveKeysAttribute,
    PreferQueueAttribute, PreferSignatureAttribute, PreferStopOnFirstFailureAttribute,
    PreferTimeoutAttribute, PreferTouchesAttribute, PreferTriesAttribute, PreferUniqueForAttribute,
    PreferVisibleAttribute, PreferWithoutIncrementingAttribute, PreferWithoutTimestampsAttribute,
};
pub use prefer_typed_assert_closure::PreferTypedAssertClosure;
pub use prefer_validation_rule_array::PreferValidationRuleArray;
pub use prefer_view_array::PreferViewArray;

use mago_syntax::cst::{
    Argument, ArgumentList, Assignment, AssignmentOperator, Call, ClassLikeMemberSelector,
    CompositeString, Expression, Node, UnaryPrefixOperator, Variable,
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
        // Before PreferFacadeOverApp, which also rewrites the `resolve()` this writes: Rector's sorted rule list runs
        // AppToResolveRector first, so both report `app(X)`.
        Box::new(PreferResolveHelper),
        Box::new(PreferFacadeOverApp),
        Box::new(NoRequestAll),
        Box::new(PreferCastsMethod),
        Box::new(PreferAnonymousMigration),
        Box::new(PreferNullsafe::default()),
        Box::new(PreferDateFacade),
        Box::new(PreferRequestFacade),
        Box::new(PreferDefaultArgument),
        Box::new(PreferExplicitJsonCall),
        Box::new(PreferTravelTo),
        Box::new(PreferAliasesAttribute),
        Box::new(PreferAppendsAttribute),
        Box::new(PreferBackoffAttribute),
        Box::new(PreferCollectedByAttribute),
        Box::new(PreferCollectsAttribute),
        Box::new(PreferCommandHiddenAttribute),
        Box::new(PreferConnectionAttribute),
        Box::new(PreferDateFormatAttribute),
        Box::new(PreferDelayAttribute),
        Box::new(PreferDeleteWhenMissingModelsAttribute),
        Box::new(PreferDescriptionAttribute),
        Box::new(PreferErrorBagAttribute),
        Box::new(PreferFailOnTimeoutAttribute),
        Box::new(PreferFillableAttribute),
        Box::new(PreferGuardedAttribute),
        Box::new(PreferHelpAttribute),
        Box::new(PreferHiddenAttribute),
        Box::new(PreferJobConnectionAttribute),
        Box::new(PreferMaxExceptionsAttribute),
        Box::new(PreferPreserveKeysAttribute),
        Box::new(PreferQueueAttribute),
        Box::new(PreferSignatureAttribute),
        Box::new(PreferStopOnFirstFailureAttribute),
        Box::new(PreferTimeoutAttribute),
        Box::new(PreferTableAttribute),
        Box::new(PreferTouchesAttribute),
        Box::new(PreferTriesAttribute),
        Box::new(PreferUniqueForAttribute),
        Box::new(PreferVisibleAttribute),
        Box::new(PreferWithoutIncrementingAttribute),
        Box::new(PreferWithoutTimestampsAttribute),
        Box::new(PreferScopeAttribute),
        Box::new(NoCollectionAliases),
        Box::new(NoNegatedContains),
        Box::new(NoNegatedFilter),
        Box::new(NoNegatedWhen),
        Box::new(PreferStatusAssertionMethod),
        Box::new(PreferEnvironmentCheck),
        Box::new(PreferTypedAssertClosure),
        Box::new(NoUselessValueCall),
        Box::new(NoUselessWithCall),
        Box::new(PreferDispatchHelper),
        Box::new(PreferValidationRuleArray),
        Box::new(NoControllerMiddleware),
        Box::new(PreferViewArray),
        Box::new(AddQueryClosureType),
        Box::new(AddRelationQueryClosureType),
        Box::new(PreferProtectedModelMethods),
        Box::new(PreferLatestOldest::default()),
        Box::new(PreferRequestValidate),
        Box::new(PreferCollectionAll),
        Box::new(PreferClosureOnlyBind),
        Box::new(AddHasFactoryTrait::default()),
        Box::new(PreferQueryBuilderCall::default()),
        Box::new(PreferClassContainerKey::default()),
        Box::new(AddFactoryExtendsPhpdoc),
        Box::new(AddRelationGenerics),
        Box::new(AddTapClosureType),
        Box::new(NoAppArrayAccess),
        Box::new(PreferConstructorInjection::default()),
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

/// `CallLike::getArg($name, $position)`: the argument named `name`, or the unnamed one at `position`; unpacked
/// arguments are skipped.
pub(crate) fn get_arg<'a>(
    list: &'a ArgumentList<'a>,
    name: &str,
    position: usize,
) -> Option<&'a Argument<'a>> {
    list.arguments
        .iter()
        .enumerate()
        .find(|(i, a)| match a {
            Argument::Named(n) => n.name.value == name.as_bytes(),
            Argument::Positional(p) => p.ellipsis.is_none() && *i == position,
        })
        .map(|(_, a)| a)
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

/// PHPStan's `ObjectType::isSuperTypeOf` on a class name: `class` is one of `targets` (ignoring ASCII case), or a
/// class the codebase knows that extends, implements or uses (itself or through a parent) one of them.
pub(crate) fn is_a(ctx: &Context<'_>, class: &str, targets: &[&str]) -> bool {
    let target = |name: &[u8]| {
        targets
            .iter()
            .any(|t| t.as_bytes().eq_ignore_ascii_case(name))
    };
    if target(class.as_bytes()) {
        return true;
    }
    let facts = crate::rules::types::facts::facts(ctx);
    let codebase = &facts.codebase;
    codebase.get_class_like(class.as_bytes()).is_some_and(|c| {
        c.all_parent_classes
            .iter()
            .chain(c.all_parent_interfaces.iter())
            .any(|p| target(p.as_bytes()))
            || std::iter::once(&c.name)
                .chain(c.all_parent_classes.iter())
                .filter_map(|p| codebase.get_class_like(p.as_bytes()))
                .any(|p| p.used_traits.iter().any(|t| target(t.as_bytes())))
    })
}

/// Whether the expression holds an object of `class` (or a subclass the codebase knows): `$this` in such a class, or
/// what [`is_object_of`](crate::rules::lint_b_helpers::is_object_of) proves from the syntax.
pub(crate) fn is_object(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    expr: &Expression<'_>,
    class: &str,
) -> bool {
    match unparenthesize(expr) {
        Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => {
            // The class-like `$this` stands for; none in a trait, which Rector leaves alone.
            let own = ancestors.iter().rev().find_map(|a| {
                let name = match a {
                    Node::Class(c) => c.name.value,
                    Node::Enum(e) => e.name.value,
                    Node::AnonymousClass(c) => {
                        let span = mago_span::HasSpan::span(*c);
                        return Some(
                            crate::rules::types::facts::facts(ctx).anonymous_class_name(span),
                        );
                    }
                    _ => return None,
                };
                let name = String::from_utf8_lossy(name);
                Some(match crate::rules::types::namespace_of(ancestors) {
                    Some(ns) => format!("{ns}\\{name}"),
                    None => name.into_owned(),
                })
            });
            own.is_some_and(|own| is_a(ctx, &own, &[class]))
        }
        e => crate::rules::lint_b_helpers::is_object_of(ancestors, e, class, node_span(e).start),
    }
}
