//! `lint/types/add-param-type-array-dim-fetch`, ported from Rector 2.6.1's `StrictArrayParamDimFetchRector`
//! (rules/TypeDeclaration/Rector/ClassMethod): an untyped parameter of a method, function or closure that its body
//! reads with a key (`$param['key']`) gets the `array` type, unless the body first uses it in a way an array would
//! not allow, or its type may be a string, a union or an `ArrayAccess` object.
//!
//! PHPStan types a closure's parameter from the call it is passed to; Pun skips where that type is a union Mago does
//! not infer: `debug_backtrace()`'s frames passed alongside, the match groups of `preg_replace_callback()` with a
//! digits-only group, and items of constant shapes whose literal values differ (a `@template-covariant` value keeps
//! them apart). An offset Mago has no type for may be an integer: Pun skips there (Mago
//! gives no types for ModelInspectorTest.php after the method-call rule's fix to it, where Rector adds `array`).

use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{
    Argument, AssignmentOperator, BinaryOperator, Call, ClassLikeMemberSelector, Expression, FunctionLikeParameterList,
    Instantiation, MethodBody, Modifier, Node, NodeKind, UnaryPrefixOperator, Variable,
};

use super::class_scope;
use super::facts::{Facts, facts};
use super::has_parent_class_method;
use super::param::{Flow, add_hint, report, traverse};
use crate::rules::class_method::type_of;
use crate::rules::lint_b_helpers::value;
use crate::rules::phpunit::string_value;
use crate::rules::rector_code_quality::named;
use crate::syntax::{node_span, unparenthesize};
use pun_problem::Span;
use crate::{Context, Report, Rule};

pub struct AddParamTypeArrayDimFetch;

impl Rule for AddParamTypeArrayDimFetch {
    fn id(&self) -> &'static str {
        "lint/types/add-param-type-array-dim-fetch"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method, NodeKind::Function, NodeKind::Closure])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (params, body): (&FunctionLikeParameterList<'_>, Node<'_, '_>) = match node {
            Node::Method(m) => {
                let MethodBody::Concrete(body) = &m.body else {
                    return;
                };
                (&m.parameter_list, Node::Block(body))
            }
            Node::Function(f) => (&f.parameter_list, Node::Block(&f.body)),
            _ => {
                let Node::Closure(c) = node else { return };
                (&c.parameter_list, Node::Block(&c.body))
            }
        };
        let facts = facts(ctx);
        if let Node::Method(m) = node
            && !m.modifiers.iter().any(|m| matches!(m, Modifier::Private(_)))
            && class_scope(ancestors)
                .is_some_and(|scope| has_parent_class_method(&facts, &scope, m.name.value))
        {
            return;
        }
        let doc = crate::rules::lint_b_doc::Doc::of(ctx.source, ancestors, node_span(&node).start);
        // PHPStan gives a closure passed to a call its parameters' types from the callable the call takes.
        let passed = matches!(node, Node::Closure(_))
            && ancestors
                .iter()
                .rev()
                .find(|a| !matches!(a, Node::Expression(_) | Node::Parenthesized(_)))
                .is_some_and(|a| {
                    matches!(
                        a,
                        Node::Argument(_) | Node::PositionalArgument(_) | Node::NamedArgument(_)
                    )
                });
        // the match groups `preg_replace_callback()` passes for a pattern with a digits-only group are a union
        let call = ancestors.iter().rev().find(|a| {
            !matches!(
                a,
                Node::Expression(_)
                    | Node::Parenthesized(_)
                    | Node::Argument(_)
                    | Node::PositionalArgument(_)
                    | Node::NamedArgument(_)
                    | Node::ArgumentList(_)
            )
        });
        let numeric_matches = matches!(call, Some(Node::FunctionCall(call)) if named(call, "preg_replace_callback")
            && call.argument_list.arguments.first().and_then(|a| string_value(value(a))).is_some_and(|p| numeric_group(&p)));
        // PHPStan's `debug_backtrace()` is a list of frame shapes with optional keys (Mago's is `array`), so a callback
        // passed alongside it gets a union
        let frames = matches!(ancestors.iter().rev().find(|a| matches!(a, Node::ArgumentList(_))),
            Some(Node::ArgumentList(list)) if list.arguments.iter().any(|a| matches!(unparenthesize(value(a)),
                Expression::Call(Call::Function(f)) if named(f, "debug_backtrace"))));
        let mut edits = Vec::new();
        for (index, param) in params.parameters.iter().enumerate() {
            if param.hint.is_some() || param.ellipsis.is_some() || (numeric_matches || frames) && index == 0 {
                continue;
            }
            if let Some(default) = &param.default_value
                && !is_array(&facts, default.value)
            {
                continue;
            }
            let tagged = doc.as_ref().is_some_and(|d| {
                (0..d.len()).any(|i| {
                    let words = d.words(i);
                    words.first() == Some(&"@param")
                        && words.iter().any(|w| w.as_bytes() == param.variable.name)
                })
            });
            if is_accessed_array_dim_fetch(&facts, body, param.variable.name, tagged || passed) {
                edits.push(add_hint(param, "array"));
            }
        }
        if edits.is_empty() {
            return;
        }
        out.push(report(
            node_span(&node),
            "Add the `array` type to parameters read with a key.",
            edits,
        ));
    }
}

/// Whether the regex `pattern` has a capture group that matches digits only (`(\d+)`, `(?<id>[0-9]+)`): PHPStan's
/// `RegexArrayShapeMatcher` types it `numeric-string`, which Rector's type corrector keeps, so the match groups are
/// an array shape of more than one type.
fn numeric_group(pattern: &str) -> bool {
    static GROUP: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"(?:^|[^\\])\((?:\?(?:P?<\w+>|'\w+'))?(?:(?:\\d|\[0-9\])(?:[+*?]|\{\d+(?:,\d*)?\})?)+\)")
            .unwrap()
    });
    GROUP.is_match(pattern)
}

/// `getType($default)->isArray()->yes()`.
fn is_array(facts: &Facts, expr: &Expression<'_>) -> bool {
    if matches!(
        unparenthesize(expr),
        Expression::Array(_) | Expression::LegacyArray(_)
    ) {
        return true;
    }
    type_of(facts, expr).is_some_and(|t| {
        !t.types.is_empty() && t.types.iter().all(|a| matches!(a, TAtomic::Array(_)))
    })
}

/// `isParamAccessedArrayDimFetch()`: walking the body (not nested functions and classes), the last offset read on
/// the parameter before anything that stops the walk. With `known` (a `@param` tag names the parameter, or a call
/// gives a closure its type), the type the analyser gives is PHPStan's, and a value whose type Pun does not read stops
/// it; without, the parameter is `mixed` until assigned (Mago's narrowing of it to an array by `isset()` or `empty()`
/// on an offset is not PHPStan's).
fn is_accessed_array_dim_fetch(
    facts: &Facts,
    body: Node<'_, '_>,
    name: &[u8],
    known: bool,
) -> bool {
    // Where the parameter is assigned: until then PHPStan's type is the parameter's own (`isset()` and `empty()` on an
    // offset narrow it with accessories, where Mago makes it an array).
    let mut assigned: Vec<usize> = Vec::new();
    // The type the parameter has where the body first names it: until it is assigned, PHPStan keeps it (Mago narrows
    // a parameter read with an offset).
    let mut initial: Option<&TUnion> = None;
    let mut seen = false;
    // The assignment targets, whose type is the one after the assignment.
    let mut targets: Vec<Span> = Vec::new();
    // Where a call whose `@phpstan-assert` narrows its argument (`$this->assertIsArray($param)`) ends: after it,
    // PHPStan's type is the asserted one, as Mago's is.
    let mut asserted: Vec<usize> = Vec::new();
    traverse(body, &mut |node| {
        if let Node::MethodCall(c) = node
            && let ClassLikeMemberSelector::Identifier(method) = &c.method
            && c.argument_list.arguments.iter().any(|a| is_variable(value(a), name))
            && type_of(facts, c.object).is_some_and(|t| asserts(facts, t, method.value))
        {
            asserted.push(node_span(c).end);
        }
        if let Node::Assignment(a) = node
            && is_variable(a.lhs, name)
        {
            assigned.push(node_span(a).end);
            targets.push(node_span(unparenthesize(a.lhs)));
        }
        if let Node::Expression(e) = node
            && !seen
            && is_variable(e, name)
            && !targets.contains(&node_span(e))
        {
            seen = true;
            initial = type_of(facts, e);
        }
        Flow::Continue
    });
    // The offsets PHPStan reads in an `isset()` scope, where the variable is not `null`: the left of `??` and the
    // values of `isset()` and `empty()`.
    let mut guarded: Vec<Span> = Vec::new();
    let mut accessed = false;
    traverse(body, &mut |node| {
        match node {
            Node::Binary(b) if matches!(b.operator, BinaryOperator::NullCoalesce(_)) => {
                guarded.push(node_span(unparenthesize(b.lhs)));
            }
            Node::IssetConstruct(i) => {
                guarded.extend(i.values.iter().map(|v| node_span(unparenthesize(v))));
            }
            Node::EmptyConstruct(e) => guarded.push(node_span(unparenthesize(e.value))),
            _ => {}
        }
        if should_stop(node, name) {
            accessed = false;
            return Flow::Stop;
        }
        let Node::ArrayAccess(access) = node else {
            return Flow::Continue;
        };
        if !is_variable(access.array, name) {
            return Flow::Continue;
        }
        let mut variable = type_of(facts, access.array);
        let at = node_span(access).start;
        let unassigned = !assigned.iter().any(|&end| end <= at);
        let narrowed;
        if known && unassigned && !asserted.iter().any(|&end| end <= at) {
            let here = variable;
            variable = initial;
            // A check that removes `null` (`if ($param)`) narrows it in both analysers.
            let has_null = |t: &TUnion| t.types.iter().any(|a| matches!(a, TAtomic::Null));
            if let (Some(i), Some(h)) = (initial, here)
                && i.types.len() > 1
                && has_null(i)
                && !has_null(h)
            {
                narrowed = TUnion::from_vec(
                    i.types.iter().filter(|a| !matches!(a, TAtomic::Null)).cloned().collect(),
                );
                variable = Some(&narrowed);
            }
        }
        if !known
            && unassigned
            && variable.is_some_and(|t| t.types.iter().all(|a| matches!(a, TAtomic::Array(_))))
        {
            variable = None;
        }
        let without_null;
        if guarded.contains(&node_span(access))
            && let Some(t) = variable
            && t.types.len() > 1
            && t.types.iter().any(|a| matches!(a, TAtomic::Null))
        {
            without_null = TUnion::from_vec(
                t.types.iter().filter(|a| !matches!(a, TAtomic::Null)).cloned().collect(),
            );
            variable = Some(&without_null);
        }
        if variable.is_none() && known {
            accessed = false;
            return Flow::Stop;
        }
        // skip possible strings
        if variable.is_some_and(is_string) {
            accessed = false;
            return Flow::Stop;
        }
        // skip integer in possibly string type as string can be accessed via int
        // an offset Mago has no type for (it gives up on some code) may be an integer: then Pun leaves the parameter
        let integer = type_of(facts, access.index).is_none_or(is_integer);
        if integer && variable.is_none_or(maybe_string) {
            return Flow::Continue;
        }
        if variable.is_some_and(|t| is_union(t) || is_array_access(facts, t)) {
            accessed = false;
            return Flow::Stop;
        }
        accessed = true;
        Flow::Continue
    });
    accessed
}

/// Whether the method `method` of the object `t` asserts the type of an argument (`@phpstan-assert`).
fn asserts(facts: &Facts, t: &TUnion, method: &[u8]) -> bool {
    let [TAtomic::Object(TObject::Named(object))] = t.types.as_ref() else { return false };
    facts.documented().get_declaring_method(object.name.as_bytes(), method).is_some_and(|m| !m.assertions.is_empty())
}

/// `shouldStop()`.
fn should_stop(node: Node<'_, '_>, name: &[u8]) -> bool {
    let checked: Option<&Expression<'_>> = match node {
        Node::FunctionCall(call)
            if ["is_array", "is_string", "is_int", "is_bool", "is_float"]
                .iter()
                .any(|n| named(call, n)) =>
        {
            call.argument_list.arguments.first().map(value)
        }
        Node::ExpressionStatement(statement) => Some(statement.expression),
        Node::Binary(binary) if matches!(binary.operator, BinaryOperator::NullCoalesce(_)) => {
            Some(binary.lhs)
        }
        Node::Assignment(assign) if matches!(assign.operator, AssignmentOperator::Coalesce(_)) => {
            Some(assign.lhs)
        }
        _ => None,
    };
    if let Some(checked) = checked.map(unparenthesize) {
        if let Expression::Call(Call::Method(call)) = checked
            && is_variable(call.object, name)
        {
            return true;
        }
        if is_variable(checked, name) {
            return true;
        }
    }
    match node {
        // isEmptyOrEchoedOrCasted
        Node::EmptyConstruct(empty) => is_variable(empty.value, name),
        Node::Echo(echo) => echo.values.iter().any(|v| is_variable(v, name)),
        Node::UnaryPrefix(cast) if matches!(cast.operator, UnaryPrefixOperator::ArrayCast(..)) => {
            is_variable(cast.operand, name)
        }
        // isPropertyFetchedOnArrayDimFetch
        Node::PropertyAccess(fetch) => is_dim_fetch_on(fetch.object, name),
        Node::StaticPropertyAccess(fetch) => is_dim_fetch_on(fetch.class, name),
        // isInstanceofParam
        Node::Binary(binary) if matches!(binary.operator, BinaryOperator::Instanceof(_)) => {
            is_variable(binary.lhs, name)
        }
        // isReassignAndUseAsArg
        Node::Assignment(assign)
            if matches!(assign.operator, AssignmentOperator::Assign(_))
                && is_variable(assign.lhs, name) =>
        {
            call_args(assign.rhs).is_some_and(|args| {
                args.iter()
                    .any(|a| matches!(a, Argument::Positional(p) if p.ellipsis.is_none() && is_variable(p.value, name))
                        || matches!(a, Argument::Named(n) if is_variable(n.value, name)))
            })
        }
        _ => false,
    }
}

/// The arguments of a call (`CallLike`), not a first-class callable.
fn call_args<'a>(expr: &'a Expression<'a>) -> Option<Vec<&'a Argument<'a>>> {
    let list = match unparenthesize(expr) {
        Expression::Call(Call::Function(c)) => &c.argument_list,
        Expression::Call(Call::Method(c)) => &c.argument_list,
        Expression::Call(Call::NullSafeMethod(c)) => &c.argument_list,
        Expression::Call(Call::StaticMethod(c)) => &c.argument_list,
        Expression::Instantiation(Instantiation {
            argument_list: Some(list),
            ..
        }) => list,
        _ => return None,
    };
    Some(list.arguments.iter().collect())
}

/// Whether `expr` is `$name[...]`.
fn is_dim_fetch_on(expr: &Expression<'_>, name: &[u8]) -> bool {
    matches!(unparenthesize(expr), Expression::ArrayAccess(a) if is_variable(a.array, name))
}

/// Whether `expr` is the variable `name` (`$` included).
pub(crate) fn is_variable(expr: &Expression<'_>, name: &[u8]) -> bool {
    matches!(unparenthesize(expr), Expression::Variable(Variable::Direct(v)) if v.name == name)
}

/// `isString()->yes()`.
fn is_string(t: &TUnion) -> bool {
    !t.types.is_empty()
        && t.types
            .iter()
            .all(|a| matches!(a, TAtomic::Scalar(s) if s.is_any_string()))
}

/// `isInteger()->yes()`.
fn is_integer(t: &TUnion) -> bool {
    !t.types.is_empty()
        && t.types
            .iter()
            .all(|a| matches!(a, TAtomic::Scalar(s) if s.is_int()))
}

/// `isString()->maybe()`: not surely a string, and not surely something else.
fn maybe_string(t: &TUnion) -> bool {
    !is_string(t)
        && (t.types.is_empty()
            || t.types.iter().any(|a| match a {
                TAtomic::Mixed(_) | TAtomic::Callable(_) => true,
                TAtomic::Scalar(s) => {
                    s.is_any_string()
                        || matches!(s, TScalar::Generic | TScalar::ArrayKey | TScalar::Numeric)
                }
                _ => false,
            }))
}

/// `TypeFactory::createMixedPassedOrUnionType([$type]) instanceof UnionType`: more than one type, `true` with `false`
/// counting as one `bool` and the arrays as one array (PHPStan unites `array<...>|list<...>`).
fn is_union(t: &TUnion) -> bool {
    // PHPStan's union with `mixed` is `mixed`.
    if t.types.iter().any(|a| matches!(a, TAtomic::Mixed(_))) {
        return false;
    }
    let count = |f: fn(&TAtomic) -> bool| t.types.iter().filter(|a| f(a)).count();
    let bools = count(|a| matches!(a, TAtomic::Scalar(TScalar::Bool(_))));
    let arrays = count(|a| matches!(a, TAtomic::Array(_)));
    if let [TAtomic::Array(array)] = t.types.as_ref() {
        return constant_array_is_union(array);
    }
    t.types.len() - bools.saturating_sub(1) - arrays.saturating_sub(1) > 1
}

/// `TypeFactory::unwrapConstantArrayTypes`: a constant array (an array shape) is one `array<key, item>` per type of
/// its items, so items of more than one type (literal values counting as their general type) make a union.
fn constant_array_is_union(array: &TArray) -> bool {
    let items: Vec<&(bool, TUnion)> = match array {
        TArray::Keyed(k) if k.parameters.is_none() => k.known_items.iter().flat_map(|i| i.values()).collect(),
        TArray::List(l) if l.element_type.is_never() => l.known_elements.iter().flat_map(|i| i.values()).collect(),
        _ => return false,
    };
    // `TypeUtils::flattenTypes` splits it into `getAllArrays()`: with every key optional, one of them is empty
    if !items.is_empty() && items.iter().all(|(optional, _)| *optional) {
        return true;
    }
    let values: Vec<&TUnion> = items.iter().map(|(_, v)| v).collect();
    let mut kinds: Vec<String> = Vec::new();
    for v in values {
        for a in v.types.iter() {
            let kind = match a {
                // `TypeHasher` tells `ArrayType`s apart by their item type's precise description: `4` is not `5`
                TAtomic::Scalar(s) if s.is_literal_value() && !s.is_bool() => a.get_id().to_string(),
                TAtomic::Scalar(s) if s.is_int() => "int".to_string(),
                TAtomic::Scalar(s) if s.is_float() => "float".to_string(),
                TAtomic::Scalar(s) if s.is_any_string() => "string".to_string(),
                TAtomic::Scalar(s) if s.is_bool() => "bool".to_string(),
                TAtomic::Array(_) => "array".to_string(),
                other => other.get_id().to_string(),
            };
            if !kinds.contains(&kind) {
                kinds.push(kind);
            }
        }
    }
    kinds.len() > 1
}

/// `isArrayAccess()`: an object of a class that is `ArrayAccess` or implements it.
fn is_array_access(facts: &Facts, t: &TUnion) -> bool {
    let [TAtomic::Object(TObject::Named(named))] = t.types.as_ref() else {
        return false;
    };
    let name = named.name.as_bytes();
    name.eq_ignore_ascii_case(b"ArrayAccess")
        || facts.codebase.get_class_like(name).is_some_and(|c| {
            c.all_parent_interfaces
                .iter()
                .any(|i| i.as_bytes().eq_ignore_ascii_case(b"ArrayAccess"))
        })
}
