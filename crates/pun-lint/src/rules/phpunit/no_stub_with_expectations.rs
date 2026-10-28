//! `lint/phpunit/no-stub-with-expectations`, ported from rector-phpunit's `MockObjectArgCreateStubToCreateMockRector`
//! (rules/PHPUnit110/Rector/ClassMethod).
//!
//! In a test class, the last top-level `$x = $any->createStub(...)` of each variable becomes `createMock()` when `$x`
//! (by name, ASCII case-insensitively) is an argument of a method or static call in the method (not inside a closure
//! or a nested function) whose parameter at that position, or of that name, is a `MockObject` (its type with the
//! docblock read, as PHPStan's reflection gives it). The called method is found as `ReflectionResolver` finds it: on
//! the one class of the receiver's type (`$this` is the class itself), or on each class of a static call's class.

use mago_codex::metadata::parameter::FunctionLikeParameterMetadata;
use mago_syntax::cst::{
    Argument, Call, ClassLikeMemberSelector, Expression, MethodBody, Node, NodeKind, Variable,
};
use pun_problem::{Edit, Safety};

use super::no_bare_mock_assign::scoped;
use super::prefer_create_stub_expression::is_mock_object;
use super::{AnyCall, class_parents, is_in_test_class, object_class, own_class, report, resolve};
use crate::rules::rector_stmts::{assign, block_list, expression};
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoStubWithExpectations;

impl Rule for NoStubWithExpectations {
    fn id(&self) -> &'static str {
        "lint/phpunit/no-stub-with-expectations"
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
        let MethodBody::Concrete(block) = &method.body else {
            return;
        };
        if !is_in_test_class(ctx, ancestors, Some(method)) {
            return;
        }
        let list = block_list(ctx.source, block);
        // `collectCreateStubAssigns`: the last `createStub()` call of each variable name
        let mut stubs: Vec<(&[u8], &ClassLikeMemberSelector<'_>)> = Vec::new();
        for statement in &list.stmts {
            let Some(assignment) = expression(statement).and_then(assign) else {
                continue;
            };
            let (Expression::Variable(Variable::Direct(var)), Expression::Call(Call::Method(call))) =
                (assignment.lhs, assignment.rhs)
            else {
                continue;
            };
            let is_create_stub = matches!(&call.method, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(b"createStub"));
            if !is_create_stub {
                continue;
            }
            let name = &var.name[1..];
            match stubs.iter_mut().find(|(n, _)| *n == name) {
                Some(entry) => entry.1 = &call.method,
                None => stubs.push((name, &call.method)),
            }
        }
        if stubs.is_empty() {
            return;
        }
        let roots: Vec<Node<'_, '_>> = list.stmts.iter().map(|s| Node::Statement(s)).collect();
        let mut scope = ancestors.to_vec();
        scope.push(node);
        let mut edits = Vec::new();
        for (name, selector) in stubs {
            if is_passed_as_mock_object_arg(ctx, &scope, &roots, name) {
                edits.push(Edit::new(node_span(selector), "createMock"));
            }
        }
        let Some(first) = edits.first().map(|e| e.span) else {
            return;
        };
        out.push(report(
            first,
            "Create a mock, not a stub, for a value passed where a MockObject is expected.",
            Safety::Safe,
            edits,
        ));
    }
}

/// `isPassedAsMockObjectArg`.
fn is_passed_as_mock_object_arg(
    ctx: &Context<'_>,
    scope: &[Node<'_, '_>],
    roots: &[Node<'_, '_>],
    name: &[u8],
) -> bool {
    let mut found = false;
    scoped(roots, &mut |node| {
        if found {
            return;
        }
        let Some(call) = AnyCall::of(node) else {
            return;
        };
        for (index, arg) in call.args().into_iter().enumerate() {
            let value = match arg {
                Argument::Positional(p) => p.value,
                Argument::Named(n) => n.value,
            };
            let Expression::Variable(Variable::Direct(v)) = value else {
                continue;
            };
            if !v.name[1..].eq_ignore_ascii_case(name) {
                continue;
            }
            if is_mock_object_param(ctx, scope, &call, arg, index) {
                found = true;
                return;
            }
        }
    });
    found
}

/// `isMockObjectParam`: the parameter `arg` fills (by its name, or else by `index`) has a `MockObject` type.
fn is_mock_object_param(
    ctx: &Context<'_>,
    scope: &[Node<'_, '_>],
    call: &AnyCall<'_>,
    arg: &Argument<'_>,
    index: usize,
) -> bool {
    let Some(method) = call.name() else {
        return false;
    };
    let classes = called_classes(ctx, scope, call);
    let facts = facts(ctx);
    let codebase = facts.documented();
    let Some(metadata) = classes
        .iter()
        .find_map(|class| codebase.get_declaring_method(class.as_bytes(), method))
    else {
        return false;
    };
    let mock_object = |parameter: &FunctionLikeParameterMetadata| {
        parameter.type_metadata.as_ref().is_some_and(|t| {
            !t.type_union.types.is_empty()
                && t.type_union
                    .types
                    .iter()
                    .all(|a| is_mock_object(codebase, a))
        })
    };
    if let Argument::Named(named) = arg {
        return metadata
            .parameters
            .iter()
            .any(|p| p.name.0.as_bytes()[1..] == *named.name.value && mock_object(p));
    }
    metadata.parameters.get(index).is_some_and(mock_object)
}

/// The classes `ReflectionResolver` looks a call's method up on, in order: the one class of the receiver's type
/// (`$this` is the class itself), or each class of a static call's class.
pub(super) fn called_classes(
    ctx: &Context<'_>,
    scope: &[Node<'_, '_>],
    call: &AnyCall<'_>,
) -> Vec<String> {
    if call.is_static {
        match call.receiver {
            Expression::Self_(_) | Expression::Static(_) => own_class(scope).into_iter().collect(),
            Expression::Identifier(id) => vec![resolve(scope, id)],
            Expression::Parent(_) => own_class(scope)
                .and_then(|c| class_parents(ctx, &c).into_iter().next())
                .into_iter()
                .collect(),
            other => object_class(ctx, other).into_iter().collect(),
        }
    } else {
        match unparenthesize(call.receiver) {
            Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => {
                own_class(scope).into_iter().collect()
            }
            other => object_class(ctx, other).into_iter().collect(),
        }
    }
}
