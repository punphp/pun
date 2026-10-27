//! `lint/phpunit/add-mock-intersection-type`, ported from rector-phpunit's `AddIntersectionVarToMockObjectPropertyRector`
//! (rules/CodeQuality/Rector/Class_), with `MockObjectPropertyDetector`.
//!
//! In a test class with a `setUp()` method, a property typed `PHPUnit\Framework\MockObject\MockObject` that `setUp()`
//! assigns `createMock(X::class)` to gets the `@var` type `\PHPUnit\Framework\MockObject\MockObject&\X`, unless its
//! first `@var` tag has an intersection type already. Rector runs it only for PHPUnit 11 and later; Pun leaves that
//! to the rule's group.

use mago_syntax::cst::{
    Access, Call, ClassLikeMember, Expression, Hint, MethodBody, Node, NodeKind, Property,
};
use pun_problem::{Edit, Safety};

use super::annotations::Annotated;
use super::phpdoc_type::{TagType, typed_tag, with_tag_type};
use super::{AnyCall, arg_value, class_body, is_in_test_class, report, resolve};
use crate::Context;
use crate::rules::rector_stmts::{assign, block_list, expression};
use crate::syntax::unparenthesize;
use crate::{Report, Rule};

/// `PHPUnitClassName::MOCK_OBJECT`.
const MOCK_OBJECT: &str = "PHPUnit\\Framework\\MockObject\\MockObject";

pub struct AddMockIntersectionType;

impl Rule for AddMockIntersectionType {
    fn id(&self) -> &'static str {
        "lint/phpunit/add-mock-intersection-type"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class, NodeKind::AnonymousClass])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some((members, _, _)) = class_body(node) else {
            return;
        };
        let mut scope = ancestors.to_vec();
        scope.push(node);
        if !is_in_test_class(ctx, &scope, None) {
            return;
        }
        let Some(calls) = created_properties(ctx, members.iter(), "createMock") else {
            return;
        };
        let mut edits = Vec::new();
        let mut changed: Vec<usize> = Vec::new();
        for (name, call) in calls {
            let Some(at) = members
                .iter()
                .position(|m| matches!(m, ClassLikeMember::Property(p) if declares(p, name)))
            else {
                continue;
            };
            let ClassLikeMember::Property(property) = &members[at] else {
                unreachable!()
            };
            // only properties typed as a bare native MockObject
            if !is_typed(&scope, property, MOCK_OBJECT) {
                continue;
            }
            let Some(mocked) = mocked_class(&scope, &call) else {
                continue;
            };
            let ty = format!("\\{MOCK_OBJECT}&\\{mocked}");
            if changed.contains(&at) {
                continue;
            }
            if let Some(e) = set_var_type(ctx, &scope, property, |_| Some(ty)) {
                changed.push(at);
                edits.extend(e);
            }
        }
        if edits.is_empty() {
            return;
        }
        out.push(report(
            crate::syntax::node_span(&members[changed[0]]),
            "Add the mocked class to the mock property's @var as an intersection.",
            Safety::Safe,
            edits,
        ));
    }
}

/// `MockObjectPropertyDetector::collectFromClassMethod()` on the class's `setUp()`: the names of the properties
/// assigned a `method` call (`$this->` or `self::`), each with its last call, in the order of their first; `None`
/// without a `setUp()`.
pub(super) fn created_properties<'a>(
    ctx: &Context<'_>,
    mut members: impl Iterator<Item = &'a ClassLikeMember<'a>>,
    method: &str,
) -> Option<Vec<(&'a [u8], AnyCall<'a>)>> {
    let set_up = members.find_map(|m| match m {
        ClassLikeMember::Method(m) if m.name.value.eq_ignore_ascii_case(b"setUp") => Some(m),
        _ => None,
    })?;
    let mut calls: Vec<(&[u8], AnyCall<'_>)> = Vec::new();
    let MethodBody::Concrete(body) = &set_up.body else {
        return Some(calls);
    };
    for statement in block_list(ctx.source, body).stmts {
        let Some(assign) = expression(statement).and_then(assign) else {
            continue;
        };
        let Expression::Access(Access::Property(fetch)) = assign.lhs else {
            continue;
        };
        let mago_syntax::cst::ClassLikeMemberSelector::Identifier(name) = &fetch.property else {
            continue;
        };
        let call = match unparenthesize(assign.rhs) {
            Expression::Call(Call::Method(c)) => AnyCall::of(Node::MethodCall(c)),
            Expression::Call(Call::StaticMethod(c)) => AnyCall::of(Node::StaticMethodCall(c)),
            _ => None,
        };
        let Some(call) = call.filter(|c| c.is_name(&[method])) else {
            continue;
        };
        match calls.iter_mut().find(|(n, _)| *n == name.value) {
            Some(entry) => entry.1 = call,
            None => calls.push((name.value, call)),
        }
    }
    Some(calls)
}

/// php-parser's `Class_::getProperty()`: a property statement declares `name` (any of its items).
pub(super) fn declares(property: &Property<'_>, name: &[u8]) -> bool {
    match property {
        Property::Plain(p) => p.items.iter().any(|i| i.variable().name[1..] == *name),
        Property::Hooked(p) => p.item.variable().name[1..] == *name,
    }
}

/// `MockObjectPropertyDetector::detect()`: the property's type is the name `class`.
pub(super) fn is_typed(scope: &[Node<'_, '_>], property: &Property<'_>, class: &str) -> bool {
    let hint = match property {
        Property::Plain(p) => p.hint.as_ref(),
        Property::Hooked(p) => p.hint.as_ref(),
    };
    matches!(hint, Some(Hint::Identifier(id)) if resolve(scope, id) == class)
}

/// `resolveMockedClass()`: the class of an `X::...` first argument, as Rector names it (`self` and `static` as such).
pub(super) fn mocked_class(scope: &[Node<'_, '_>], call: &AnyCall<'_>) -> Option<String> {
    let arg = call.args().into_iter().next()?;
    let Expression::Access(Access::ClassConstant(fetch)) = arg_value(arg) else {
        return None;
    };
    match fetch.class {
        Expression::Identifier(id) => Some(resolve(scope, id)),
        Expression::Self_(_) => Some("self".to_string()),
        Expression::Static(_) => Some("static".to_string()),
        Expression::Parent(_) => Some("parent".to_string()),
        _ => None,
    }
}

/// The edits that set the type of the property's first `@var` tag (or add one) to what `ty` gives for the current
/// tag, unless that tag has an intersection type already; `None` where nothing changes or Pun does not follow.
pub(super) fn set_var_type(
    ctx: &Context<'_>,
    scope: &[Node<'_, '_>],
    property: &Property<'_>,
    ty: impl FnOnce(Option<&TagType<'_>>) -> Option<String>,
) -> Option<Vec<Edit>> {
    let annotated = Annotated::of(ctx.source, Node::Property(property), scope)?;
    let tag = match annotated.doc.as_ref() {
        Some(doc) => typed_tag(doc, "@var", None).ok()?,
        None => None,
    };
    // already has an intersection @var, skip
    if matches!(tag.as_ref().map(|t| &t.ty), Some(TagType::Intersection(_))) {
        return None;
    }
    let ty = ty(tag.as_ref().map(|t| &t.ty))?;
    let new_doc = with_tag_type(annotated.doc.as_ref(), "@var", None, &ty)?;
    annotated.edits(ctx.source, Some(&new_doc), &[])
}
