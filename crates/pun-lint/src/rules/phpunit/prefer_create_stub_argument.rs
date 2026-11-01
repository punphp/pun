//! `lint/phpunit/prefer-create-stub-argument`, ported from rector-phpunit's `CreateStubOverCreateMockArgRector`
//! (rules/PHPUnit120/Rector/CallLike), with `MockObjectExprDetector::isUsedForMocking`.
//!
//! In a test class, `$any->createMock(...)` becomes `createStub()` where it is:
//! - an argument of a method call, a static call or a `new` whose method (found as `ReflectionResolver` finds it; a
//!   `new` reads the constructor) takes no `MockObject` there: the parameter at that position, or of that name, has
//!   no type that is a `MockObject` (read with its docblock, as PHPStan's reflection gives it). A call whose method
//!   is not found is left alone, as is `new parent(...)`, for which Rector finds none;
//! - an array item's value;
//! - the value of a top-level `$x = ...` statement (not a property's) of a test method
//!   (`TestsNodeAnalyzer::isTestClassMethod()`) when `$x` is not the receiver of a method call in the method and is
//!   not passed to a method whose parameter there is a `MockObject`.
//!
//! Rector runs it only for PHPUnit 11 and later; Pun leaves that to the rule's group.

use mago_syntax::cst::{
    Access, Argument, ArgumentList, ArrayElement, Call, ClassLikeMemberSelector, Expression,
    Method, MethodBody, Node, NodeKind, PartialArgument, Variable,
};
use pun_problem::{Edit, Safety};

use super::no_stub_with_expectations::called_classes;
use super::prefer_create_stub_expression::{
    is_super_type_of_mock_object, is_test_class_method, is_used_for_mocking,
};
use super::{AnyCall, is_in_test_class, object_class, own_class, resolve};
use crate::rules::rector_stmts::{assign, block_list, expression};
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferCreateStubArgument;

const MESSAGE: &str = "Use createStub() for a mock that expects no calls.";

impl Rule for PreferCreateStubArgument {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-create-stub-argument"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::MethodCall,
            NodeKind::StaticMethodCall,
            NodeKind::Instantiation,
            NodeKind::AnonymousClass,
            NodeKind::ArrayElement,
            NodeKind::Method,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let method = match node {
            Node::Method(method) => Some(method),
            _ => None,
        };
        if !is_in_test_class(ctx, ancestors, method) {
            return;
        }
        let edits = match node {
            Node::Method(method) => class_method(ctx, ancestors, method),
            Node::ArrayElement(element) => {
                let value = match element {
                    ArrayElement::KeyValue(kv) => kv.value,
                    ArrayElement::Value(v) => v.value,
                    ArrayElement::Variadic(v) => v.value,
                    ArrayElement::Missing(_) => return,
                };
                create_mock(value).into_iter().collect()
            }
            _ => call_like(ctx, ancestors, node),
        };
        let Some(first) = edits.first().map(|e: &Edit| e.span) else {
            return;
        };
        out.push(Report::fixed(first, MESSAGE, Safety::Safe, edits));
    }
}

/// The edit renaming `expr` when it is a `->createMock(...)` call (`matchCreateMockMethodCall`).
fn create_mock(expr: &Expression<'_>) -> Option<Edit> {
    let Expression::Call(Call::Method(call)) = unparenthesize(expr) else {
        return None;
    };
    let ClassLikeMemberSelector::Identifier(name) = &call.method else {
        return None;
    };
    name.value
        .eq_ignore_ascii_case(b"createMock")
        .then(|| Edit::new(node_span(name), "createStub"))
}

/// `refactorClassMethod`.
fn class_method(ctx: &Context<'_>, ancestors: &[Node<'_, '_>], method: &Method<'_>) -> Vec<Edit> {
    let MethodBody::Concrete(block) = &method.body else {
        return Vec::new();
    };
    if !is_test_class_method(ctx, ancestors, method) {
        return Vec::new();
    }
    let list = block_list(ctx.source, block);
    let roots: Vec<Node<'_, '_>> = list.stmts.iter().map(|s| Node::Statement(s)).collect();
    let mut edits = Vec::new();
    for statement in &list.stmts {
        let Some(assignment) = expression(statement).and_then(assign) else {
            continue;
        };
        // handled in another rule
        if matches!(
            unparenthesize(assignment.lhs),
            Expression::Access(Access::Property(_))
        ) {
            continue;
        }
        let Some(edit) = create_mock(assignment.rhs) else {
            continue;
        };
        // no change, as we use the variable for mocking later
        let used = match unparenthesize(assignment.lhs) {
            Expression::Variable(Variable::Direct(v)) => {
                is_used_for_mocking(ctx, ancestors, &roots, &v.name[1..], true)
            }
            // a variable without a name, to be safe
            Expression::Variable(_) => true,
            _ => false,
        };
        if !used {
            edits.push(edit);
        }
    }
    edits
}

/// Each argument of `list`: its parameter's name when it is a named one, and its value.
fn arguments<'a>(list: &'a ArgumentList<'a>) -> Vec<(Option<&'a [u8]>, &'a Expression<'a>)> {
    list.arguments
        .iter()
        .map(|arg| match arg {
            Argument::Named(named) => (Some(named.name.value), named.value),
            Argument::Positional(p) => (None, p.value),
        })
        .collect()
}

/// The method call, static call or `new` part of `refactor()`.
fn call_like(ctx: &Context<'_>, ancestors: &[Node<'_, '_>], node: Node<'_, '_>) -> Vec<Edit> {
    let (class, method, args) = match node {
        Node::Instantiation(new) => {
            let Some(list) = &new.argument_list else {
                return Vec::new();
            };
            let class = match unparenthesize(new.class) {
                Expression::Identifier(id) => Some(resolve(ancestors, id)),
                Expression::Self_(_) | Expression::Static(_) => own_class(ancestors),
                // Rector finds no constructor for `new parent` (tests/rector/CreateStubOverCreateMockArgRector/Oracle)
                Expression::Parent(_) => None,
                other => object_class(ctx, other),
            };
            (
                class.into_iter().collect(),
                &b"__construct"[..],
                arguments(list),
            )
        }
        Node::AnonymousClass(new) => {
            let Some(list) = &new.argument_list else {
                return Vec::new();
            };
            let args = list
                .arguments
                .iter()
                .filter_map(|arg| match arg {
                    PartialArgument::Named(named) => Some((Some(named.name.value), named.value)),
                    other => other.value().map(|v| (None, v)),
                })
                .collect();
            let class = facts(ctx).anonymous_class_name(mago_span::HasSpan::span(new));
            (vec![class], &b"__construct"[..], args)
        }
        _ => {
            let call = AnyCall::of(node).unwrap();
            let Some(name) = call.name() else {
                return Vec::new();
            };
            (
                called_classes(ctx, ancestors, &call),
                name,
                arguments(call.args),
            )
        }
    };
    let facts = facts(ctx);
    let codebase = facts.documented();
    let Some(metadata) = class
        .iter()
        .find_map(|class| codebase.get_declaring_method(class.as_bytes(), method))
    else {
        return Vec::new();
    };
    let mut edits = Vec::new();
    for (index, (name, value)) in args.into_iter().enumerate() {
        let parameter = match name {
            Some(name) => metadata
                .parameters
                .iter()
                .find(|p| p.name.0.as_bytes()[1..] == *name),
            None => metadata.parameters.get(index),
        };
        let parameter_type = parameter.and_then(|p| p.type_metadata.as_ref());
        // PHPStan reads a template in the type as it resolves for this call, which Pun does not follow
        if parameter_type.is_some_and(|t| {
            t.type_union.has_template_types()
                || is_super_type_of_mock_object(codebase, &t.type_union)
        }) {
            continue;
        }
        edits.extend(create_mock(value));
    }
    edits
}
