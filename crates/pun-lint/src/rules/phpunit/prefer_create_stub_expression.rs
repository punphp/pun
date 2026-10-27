//! `lint/phpunit/prefer-create-stub-expression`, ported from rector-phpunit's
//! `ExpressionCreateMockToCreateStubRector` (rules/PHPUnit120/Rector/ClassMethod), with
//! `AssignedMocksCollector::matchCreateMockArgAssignedToVariable` and `MockObjectExprDetector::isUsedForMocking`.
//!
//! In a test method (`TestsNodeAnalyzer::isTestClassMethod()`) of two or more statements, a statement
//! `$x = $any->createMock(...)` becomes `createStub()` unless `$x` is the receiver of a method call in the method, or
//! is passed to a method whose parameter at that position (or of that name) is a `MockObject`. Rector runs it only for
//! PHPUnit 11 and later; Pun leaves that to the rule's group.
//!
//! Rector reads a parameter's `@param` type too. Pun sees native declarations only, so a parameter without one counts
//! as a `MockObject` and the mock stays (tests/rector/ExpressionCreateMockToCreateStubRector/Pun).

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{
    Argument, Call, Expression, MethodBody, Node, NodeKind, Statement, Variable,
};
use pun_problem::{Edit, Safety};

use super::annotations::named_tags;
use super::no_bare_mock_assign::scoped;
use super::{AnyCall, is_public, object_class, own_class, report};
use crate::rules::lint_b_doc::Doc;
use crate::rules::rector_batch5::has_attribute;
use crate::rules::rector_stmts::{assign, expression};
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// `PHPUnitClassName::MOCK_OBJECT`.
const MOCK_OBJECT: &str = "PHPUnit\\Framework\\MockObject\\MockObject";

pub struct PreferCreateStubExpression;

impl Rule for PreferCreateStubExpression {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-create-stub-expression"
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
        // isTestClassMethod
        let is_test = is_public(method)
            && (method.name.value.starts_with(b"test")
                || has_attribute(
                    ancestors,
                    &method.attribute_lists,
                    "PHPUnit\\Framework\\Attributes\\Test",
                )
                || Doc::of(ctx.source, ancestors, node_span(method).start).is_some_and(|doc| {
                    named_tags(ctx, ancestors, &doc, "test").is_some_and(|t| !t.is_empty())
                }));
        // Mago keeps a `;` alone as a statement, php-parser drops it
        let statements: Vec<&Statement<'_>> = block
            .statements
            .iter()
            .filter(|s| !matches!(s, Statement::Noop(_)))
            .collect();
        if !is_test || statements.len() < 2 {
            return;
        }
        let roots: Vec<Node<'_, '_>> = statements.iter().map(|s| Node::Statement(s)).collect();
        for statement in &statements {
            let Some(assign) = expression(statement).and_then(assign) else {
                continue;
            };
            // matchCreateMockArgAssignedToVariable
            let Expression::Variable(Variable::Direct(variable)) = unparenthesize(assign.lhs)
            else {
                continue;
            };
            let Expression::Call(Call::Method(call)) = unparenthesize(assign.rhs) else {
                continue;
            };
            let call = AnyCall::of(Node::MethodCall(call)).unwrap();
            if !call.is_name(&["createMock"]) || call.args().is_empty() {
                continue;
            }
            if is_used_for_mocking(ctx, ancestors, &roots, &variable.name[1..]) {
                continue;
            }
            out.push(report(
                node_span(statement),
                "Use createStub() for a mock that expects no calls.",
                Safety::Safe,
                vec![Edit::new(call.name_span(), "createStub")],
            ));
        }
    }
}

/// `MockObjectExprDetector::isUsedForMocking` for the variable `name` of the method whose statements are `roots`.
fn is_used_for_mocking(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    roots: &[Node<'_, '_>],
    name: &[u8],
) -> bool {
    let is_name = |expr: &Expression<'_>| matches!(unparenthesize(expr), Expression::Variable(Variable::Direct(v)) if v.name[1..].eq_ignore_ascii_case(name));
    let mut related_variables = 0;
    let mut method_calls = Vec::new();
    scoped(roots, &mut |node| match node {
        Node::DirectVariable(v) if v.name[1..].eq_ignore_ascii_case(name) => related_variables += 1,
        Node::MethodCall(call) => method_calls.push(call),
        _ => {}
    });
    // only self variable found, nothing to mock
    if related_variables == 1 {
        return false;
    }
    for call in method_calls {
        if !matches!(unparenthesize(call.object), Expression::Variable(_)) {
            continue;
        }
        if is_name(call.object) {
            // variable is being called on, most like mocking, lets skip
            return true;
        }
        // check if variable is passed as arg to a method that declares MockObject type parameter
        for (index, arg) in call.argument_list.arguments.iter().enumerate() {
            let value = match arg {
                Argument::Positional(a) => a.value,
                Argument::Named(a) => a.value,
            };
            if !is_name(value) {
                continue;
            }
            if passes_mock_object(ctx, ancestors, call.object, call, index, arg) {
                return true;
            }
        }
    }
    false
}

/// Whether the parameter of the called method that `arg` (at `index`) fills is a `MockObject`: by the argument's name,
/// or by its position.
fn passes_mock_object(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    receiver: &Expression<'_>,
    call: &mago_syntax::cst::MethodCall<'_>,
    index: usize,
    arg: &Argument<'_>,
) -> bool {
    // resolveMethodReflectionFromMethodCall
    let class = match unparenthesize(receiver) {
        Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => own_class(ancestors),
        other => object_class(ctx, other),
    };
    let (Some(class), mago_syntax::cst::ClassLikeMemberSelector::Identifier(method)) =
        (class, &call.method)
    else {
        return false;
    };
    let facts = facts(ctx);
    let Some(metadata) = facts
        .codebase
        .get_declaring_method(class.as_bytes(), method.value)
    else {
        return false;
    };
    // Pun reads native declarations only: a parameter without one may have a `@param MockObject`, so it counts as one
    let mock_object =
        |parameter: &mago_codex::metadata::parameter::FunctionLikeParameterMetadata| {
            parameter
                .type_declaration_metadata
                .as_ref()
                .is_none_or(|t| is_super_type_of_mock_object(&facts.codebase, &t.type_union))
        };
    if let Argument::Named(named) = arg
        && metadata
            .parameters
            .iter()
            .any(|p| p.name.0.as_bytes()[1..] == *named.name.value && mock_object(p))
    {
        return true;
    }
    metadata.parameters.get(index).is_some_and(mock_object)
}

/// `(new ObjectType(MockObject))->isSuperTypeOf($type)->yes()`.
fn is_super_type_of_mock_object(codebase: &CodebaseMetadata, t: &TUnion) -> bool {
    !t.types.is_empty() && t.types.iter().all(|a| is_mock_object(codebase, a))
}

/// A named object that is a `MockObject`, or an intersection with one.
pub(super) fn is_mock_object(codebase: &CodebaseMetadata, atomic: &TAtomic) -> bool {
    let TAtomic::Object(TObject::Named(named)) = atomic else {
        return false;
    };
    let is_mock = |c: &[u8]| {
        c.eq_ignore_ascii_case(MOCK_OBJECT.as_bytes())
            || codebase.is_instance_of(c, MOCK_OBJECT.as_bytes())
    };
    is_mock(named.name.as_bytes())
        || named
            .intersection_types
            .iter()
            .flat_map(|i| i.iter())
            .any(|a| matches!(a, TAtomic::Object(TObject::Named(n)) if is_mock(n.name.as_bytes())))
}
