//! `lint/phpunit/add-will-return-callback-param-type`, ported from rector-phpunit's
//! `TypeWillReturnCallableArrowFunctionRector` (rules/CodeQuality/Rector/Class_), with `SetUpAssignedMockTypesResolver`
//! and `MethodParametersAndReturnTypesResolver::resolveFromReflection`.
//!
//! In the methods of a test class, the closure or arrow function of `$mock->method('name')->willReturnCallback(fn)` or
//! `$mock->method('name')->with($this->callback(fn))` gets the native types of the mocked method: each untyped,
//! non-variadic parameter the type of the method's parameter at its position, and, when it has no return type, the
//! method's return type.
//!
//! The mocked class comes from the receiver's intersection type (`MockObject&SomeClass`), as
//! `lint/phpunit/prefer-will-return-callback` reads it, with the same fallback to the classes `setUp()` mocks. A
//! declared type that is one class other than the test class itself counts as `mixed` and is not written; neither is
//! a parameter without a type, or a return type left implicit. A closure with exactly one `return` of a value keeps
//! no return type unless that value's type takes in every value of the method's return type.

use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::metadata::ttype::TypeMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_syntax::cst::{
    Argument, Call, Expression, FunctionLikeParameterList, Literal, MethodCall, Node, NodeKind,
};
use pun_problem::{Edit, Safety, Span};

use super::prefer_will_return_callback::{
    is_named, mocked_classes, setup_assigned_mock_types, test_classes,
};
use super::{report, string_value};
use crate::rules::types::compare::classes::with_classes;
use crate::rules::types::compare::mapper::{expr_type, from_mago};
use crate::rules::types::compare::{RType, is_super};
use crate::rules::types::declare::{return_declaration, text_order};
use crate::rules::types::facts::{Facts, facts};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct AddWillReturnCallbackParamType;

impl Rule for AddWillReturnCallbackParamType {
    fn id(&self) -> &'static str {
        "lint/phpunit/add-will-return-callback-param-type"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodCall(call) = node else { return };
        let Some(inner) = match_inner_closure(call) else {
            return;
        };
        let Expression::Call(Call::Method(parent)) = call.object else {
            return;
        };
        if !is_named(&parent.method, "method") {
            return;
        }
        let Some(method) = parent
            .argument_list
            .arguments
            .first()
            .map(argument_value)
            .filter(|v| matches!(v, Expression::Literal(Literal::String(_))))
            .and_then(string_value)
        else {
            return;
        };
        // each test class whose methods hold the call, as Rector visits them: the outermost first, whose changes the
        // inner ones then find made
        for at in test_classes(ctx, ancestors) {
            let class = &ancestors[..=at];
            let edits = decorate(ctx, class, parent, method.as_bytes(), &inner);
            if !edits.is_empty() {
                out.push(report(
                    edits[0].span,
                    "Type the callback from the mocked method's native types.",
                    Safety::Unsafe,
                    edits,
                ));
                return;
            }
        }
    }
}

/// The closure or arrow function `refactor()` decorates.
enum Inner<'a> {
    Closure(&'a mago_syntax::cst::Closure<'a>),
    Arrow(&'a mago_syntax::cst::ArrowFunction<'a>),
}

impl<'a> Inner<'a> {
    fn of(expr: &'a Expression<'a>) -> Option<Self> {
        match expr {
            Expression::Closure(c) => Some(Inner::Closure(c)),
            Expression::ArrowFunction(a) => Some(Inner::Arrow(a)),
            _ => None,
        }
    }

    fn parameters(&self) -> &'a FunctionLikeParameterList<'a> {
        match self {
            Inner::Closure(c) => &c.parameter_list,
            Inner::Arrow(a) => &a.parameter_list,
        }
    }

    fn has_return_type(&self) -> bool {
        match self {
            Inner::Closure(c) => c.return_type_hint.is_some(),
            Inner::Arrow(a) => a.return_type_hint.is_some(),
        }
    }

    /// Where `: type` goes: after the `use` clause, or the parameter list.
    fn return_type_at(&self) -> usize {
        match self {
            Inner::Closure(c) => match &c.use_clause {
                Some(u) => node_span(u).end,
                None => node_span(&c.parameter_list).end,
            },
            Inner::Arrow(a) => node_span(&a.parameter_list).end,
        }
    }

    fn node(&self) -> Node<'a, 'a> {
        match self {
            Inner::Closure(c) => Node::Closure(c),
            Inner::Arrow(a) => Node::ArrowFunction(a),
        }
    }
}

fn argument_value<'a>(arg: &'a Argument<'a>) -> &'a Expression<'a> {
    match arg {
        Argument::Positional(a) => a.value,
        Argument::Named(a) => a.value,
    }
}

/// `matchInnerClosure()`: the closure of `with($this->callback(fn))` or `willReturnCallback(fn)`.
fn match_inner_closure<'a>(call: &'a MethodCall<'a>) -> Option<Inner<'a>> {
    let first = call.argument_list.arguments.first().map(argument_value)?;
    if is_named(&call.method, "with")
        && let Expression::Call(Call::Method(nested)) = first
        && is_named(&nested.method, "callback")
        && let Some(inner) = nested
            .argument_list
            .arguments
            .first()
            .map(argument_value)
            .and_then(Inner::of)
    {
        return Some(inner);
    }
    if is_named(&call.method, "willReturnCallback") {
        return Inner::of(first);
    }
    None
}

/// The edits for one call, read in the test class `class` ends in.
fn decorate(
    ctx: &Context<'_>,
    class: &[Node<'_, '_>],
    parent: &MethodCall<'_>,
    method: &[u8],
    inner: &Inner<'_>,
) -> Vec<Edit> {
    let Some(mocked) = setup_assigned_mock_types(ctx, class) else {
        return Vec::new();
    };
    // we need mocks
    let Some(classes) = mocked_classes(ctx, parent, &mocked) else {
        return Vec::new();
    };
    let facts = facts(ctx);
    let current = super::own_class_reflection(ctx, class).expect("a test class");
    let Some(resolved) = resolve_from_reflection(&facts, &classes, method, &current) else {
        return Vec::new();
    };
    let mut edits = Vec::new();
    for (key, param) in inner.parameters().parameters.iter().enumerate() {
        // avoid typing variadic parameters; already filled, lets skip it
        if param.ellipsis.is_some() || param.hint.is_some() {
            continue;
        }
        // we need specific non-mixed type
        let Some(Some(t)) = resolved.params.get(key) else {
            continue;
        };
        let Some(text) =
            declaration(&facts, t, ctx.php_version).filter(|_| !t.type_union.is_mixed())
        else {
            continue;
        };
        let at = param
            .ampersand
            .map_or(node_span(&param.variable).start, |a| {
                a.start.offset as usize
            });
        edits.push(Edit::new(Span::new(at, at), format!("{text} ")));
    }
    if !inner.has_return_type()
        && let Some(t) = &resolved.return_type
        && !conflicts_with_returned(&facts, class, inner, t)
        && let Some(text) = declaration(&facts, t, ctx.php_version)
    {
        let at = inner.return_type_at();
        edits.push(Edit::new(Span::new(at, at), format!(": {text}")));
    }
    edits
}

/// The native types `resolveFromReflection()` gives: a parameter's or the return's type, `None` for `mixed` (none
/// written, a class other than the test class, or no type).
struct Resolved<'c> {
    params: Vec<Option<&'c TypeMetadata>>,
    return_type: Option<&'c TypeMetadata>,
}

/// `MethodParametersAndReturnTypesResolver::resolveFromReflection()` on the first of `classes` that has the method
/// natively. Mago keeps an intersection's members in an order of its own, so where several have it and their types
/// differ, `None`.
fn resolve_from_reflection<'c>(
    facts: &'c Facts,
    classes: &[String],
    method: &[u8],
    current: &str,
) -> Option<Resolved<'c>> {
    let mut found: Option<(&FunctionLikeMetadata, Resolved<'c>)> = None;
    for class in classes {
        let Some(metadata) = facts
            .codebase
            .get_declaring_method(class.as_bytes(), method)
        else {
            continue;
        };
        let resolved = Resolved {
            params: metadata
                .parameters
                .iter()
                .map(|p| resolve_type(p.type_declaration_metadata.as_ref(), current))
                .collect(),
            return_type: resolve_type(metadata.return_type_declaration_metadata.as_ref(), current),
        };
        match &found {
            Some((other, _)) if !std::ptr::eq(*other, metadata) => return None,
            Some(_) => {}
            None => found = Some((metadata, resolved)),
        }
    }
    found.map(|(_, r)| r)
}

/// `resolveObjectType()` and the check after it: a type that is one class (`Closure` included) other than the test
/// class is `mixed`, and so is no type.
fn resolve_type<'c>(t: Option<&'c TypeMetadata>, current: &str) -> Option<&'c TypeMetadata> {
    let t = t?;
    if let [TAtomic::Object(TObject::Named(n))] = t.type_union.types.as_ref()
        && n.intersection_types.is_none()
        && !n.name.as_bytes().eq_ignore_ascii_case(current.as_bytes())
    {
        return None;
    }
    if let [TAtomic::Object(TObject::Enum(e))] = t.type_union.types.as_ref()
        && !e.name.as_bytes().eq_ignore_ascii_case(current.as_bytes())
    {
        return None;
    }
    // `Closure` is a class to PHPStan (Mago reads it as a callable)
    if let [TAtomic::Callable(c)] = t.type_union.types.as_ref()
        && c.is_closure()
    {
        return None;
    }
    Some(t)
}

/// `StaticTypeMapper::mapPHPStanTypeToPhpParserNode()` of a declared type, in its written order; an explicit `mixed`
/// is `mixed`.
fn declaration(facts: &Facts, t: &TypeMetadata, php_version: u32) -> Option<String> {
    if t.type_union.is_mixed() {
        return Some("mixed".to_string());
    }
    let order = facts
        .source_of(t.span.file_id)
        .and_then(|s| {
            s.get(t.span.start.offset as usize..t.span.end.offset as usize)
                .map(text_order)
        })
        .unwrap_or_default();
    return_declaration(
        t.type_union.types.iter(),
        &facts.codebase,
        php_version,
        &order,
        true,
    )
}

/// `shouldSkipReturnForConflictWithReturnedNodeType()`: the function holds exactly one `return` of a value (nested
/// functions' included), and that value's type does not take in every value of `t`.
fn conflicts_with_returned(
    facts: &Facts,
    class: &[Node<'_, '_>],
    inner: &Inner<'_>,
    t: &TypeMetadata,
) -> bool {
    let mut values = Vec::new();
    let mut stack = vec![inner.node()];
    while let Some(node) = stack.pop() {
        if let Node::Return(r) = node
            && let Some(value) = r.value
        {
            values.push(value);
        }
        node.visit_children(|c| stack.push(c));
    }
    let [value] = values[..] else {
        return false;
    };
    with_classes(facts.documented(), || {
        let returned = RType::T(expr_type(facts, class, value, false));
        let native = if t.type_union.is_mixed() {
            RType::mixed()
        } else {
            RType::T(from_mago(&t.type_union))
        };
        !is_super(&returned, &native).yes()
    })
}
