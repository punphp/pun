//! `lint/phpunit/prefer-create-stub`, ported from rector-phpunit's `PropertyCreateMockToCreateStubRector`
//! (rules/PHPUnit120/Rector/Class_), with `MockObjectPropertyDetector::collectFromClassMethod` and
//! `MockObjectExprDetector::isPropertyUsedForMocking`.
//!
//! In a test class that is not abstract, whose short name neither ends in `TestCase` nor starts with `Abstract`, and
//! that has a `setUp()` method, a property `setUp()` assigns `createMock()` to becomes a stub when no method call in
//! the class (closures included) is made on a fetch of it: the call becomes `createStub()`, and the property's type
//! `\PHPUnit\Framework\MockObject\Stub`, or, in an intersection, that type in place of `MockObject`. Rector runs it
//! only for PHPUnit 11 and later; Pun leaves that to the rule's group.

use mago_syntax::cst::{
    Access, Call, ClassLikeMember, ClassLikeMemberSelector, Expression, Hint, MethodBody, Modifier,
    Node, NodeKind, Property,
};
use pun_problem::{Edit, Safety};

use super::{AnyCall, class_body, is_in_test_class, report, resolve, text};
use crate::rules::rector_stmts::{assign, block_list, expression};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// `PHPUnitClassName::MOCK_OBJECT`.
const MOCK_OBJECT: &str = "PHPUnit\\Framework\\MockObject\\MockObject";
/// `PHPUnitClassName::STUB`, as php-parser prints a new `FullyQualified`.
const STUB: &str = "\\PHPUnit\\Framework\\MockObject\\Stub";

pub struct PreferCreateStub;

impl Rule for PreferCreateStub {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-create-stub"
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
        let (members, _, _) = class_body(node).unwrap();
        let mut scope = ancestors.to_vec();
        scope.push(node);
        // shouldSkipClass
        if !is_in_test_class(ctx, &scope, None) {
            return;
        }
        if let Node::Class(class) = node {
            // skip abstract/base test classes, as property can be mocked in child classes
            if class
                .modifiers
                .iter()
                .any(|m| matches!(m, Modifier::Abstract(_)))
            {
                return;
            }
            let short = class.name.value;
            if short.ends_with(b"TestCase") || short.starts_with(b"Abstract") {
                return;
            }
        }
        // the setup class method must be here, so we have a place where the createMock() is used
        let Some(set_up) = members.iter().find_map(|m| match m {
            ClassLikeMember::Method(m) if m.name.value.eq_ignore_ascii_case(b"setUp") => Some(m),
            _ => None,
        }) else {
            return;
        };
        let MethodBody::Concrete(body) = &set_up.body else {
            return;
        };
        // collectFromClassMethod: the last createMock() assigned to each property, in the order of the first
        let mut calls: Vec<(&[u8], AnyCall<'_>)> = Vec::new();
        for statement in block_list(ctx.source, body).stmts {
            let Some(assign) = expression(statement).and_then(assign) else {
                continue;
            };
            let Some(name) = property_fetch_name(assign.lhs) else {
                continue;
            };
            let call = match unparenthesize(assign.rhs) {
                Expression::Call(Call::Method(c)) => AnyCall::of(Node::MethodCall(c)),
                Expression::Call(Call::StaticMethod(c)) => AnyCall::of(Node::StaticMethodCall(c)),
                _ => None,
            };
            let Some(call) = call.filter(|c| c.is_name(&["createMock"])) else {
                continue;
            };
            match calls.iter_mut().find(|(n, _)| *n == name) {
                Some(entry) => entry.1 = call,
                None => calls.push((name, call)),
            }
        }
        let mut edits: Vec<Edit> = Vec::new();
        let mut first = None;
        for (name, call) in calls {
            // isPropertyUsedForMocking: the property must exist, with no method call on a fetch of it
            let Some(property) = members.iter().find_map(|m| match m {
                ClassLikeMember::Property(p) if declares(p, name) => Some(p),
                _ => None,
            }) else {
                continue;
            };
            if is_called_on(node, name) {
                continue;
            }
            first.get_or_insert(call.name_span());
            edits.push(Edit::new(call.name_span(), "createStub"));
            let at = edits.len();
            let hint = match property {
                Property::Plain(p) => p.hint.as_ref(),
                Property::Hooked(p) => p.hint.as_ref(),
            };
            match hint {
                Some(Hint::Intersection(_)) => {
                    let mut parts = Vec::new();
                    flatten(hint.unwrap(), &mut parts);
                    let matched = |h: &Hint<'_>| matches!(h, Hint::Identifier(id) if resolve(&scope, id) == MOCK_OBJECT);
                    if parts.iter().any(|h| matched(h)) {
                        let printed: Vec<&str> = parts
                            .iter()
                            .map(|h| if matched(h) { STUB } else { text(ctx, *h) })
                            .collect();
                        edits.push(Edit::new(node_span(hint.unwrap()), printed.join("&")));
                    }
                }
                Some(hint) => edits.push(Edit::new(node_span(hint), STUB)),
                None => {
                    let variable = match property {
                        Property::Plain(p) => p.items.first().unwrap().variable(),
                        Property::Hooked(p) => p.item.variable(),
                    };
                    let at = node_span(variable).start;
                    edits.push(Edit::new(
                        pun_problem::Span::new(at, at),
                        format!("{STUB} "),
                    ));
                }
            }
            // a property statement of two items gets its type once
            if edits[..at].contains(&edits[edits.len() - 1]) && edits.len() > at {
                edits.pop();
            }
        }
        let Some(first) = first else {
            return;
        };
        out.push(report(
            first,
            "Use createStub() for a mock property that expects no calls.",
            Safety::Safe,
            edits,
        ));
    }
}

/// php-parser's `Class_::getProperty()`: a property statement declares `name` (any of its items).
fn declares(property: &Property<'_>, name: &[u8]) -> bool {
    match property {
        Property::Plain(p) => p.items.iter().any(|i| i.variable().name[1..] == *name),
        Property::Hooked(p) => p.item.variable().name[1..] == *name,
    }
}

/// The name of a `PropertyFetch` (`$any->name`, not `?->`) with an identifier name.
fn property_fetch_name<'a>(expr: &'a Expression<'a>) -> Option<&'a [u8]> {
    match unparenthesize(expr) {
        Expression::Access(Access::Property(a)) => match &a.property {
            ClassLikeMemberSelector::Identifier(id) => Some(id.value),
            _ => None,
        },
        _ => None,
    }
}

/// Whether a method call anywhere in the class's methods is made on a property fetch (`$any->name`) of `name`.
fn is_called_on(class: Node<'_, '_>, name: &[u8]) -> bool {
    let mut stack: Vec<Node<'_, '_>> = Vec::new();
    if let Some((members, _, _)) = class_body(class) {
        stack.extend(members.iter().filter_map(|m| match m {
            ClassLikeMember::Method(m) => Some(Node::Method(m)),
            _ => None,
        }));
    }
    while let Some(node) = stack.pop() {
        if let Node::MethodCall(call) = node
            && property_fetch_name(call.object).is_some_and(|p| p.eq_ignore_ascii_case(name))
        {
            return true;
        }
        node.visit_children(|child| stack.push(child));
    }
    false
}

/// The types of an intersection, as php-parser lists them in its `IntersectionType`.
fn flatten<'a>(hint: &'a Hint<'a>, out: &mut Vec<&'a Hint<'a>>) {
    match hint {
        Hint::Intersection(i) => {
            flatten(i.left, out);
            flatten(i.right, out);
        }
        _ => out.push(hint),
    }
}
