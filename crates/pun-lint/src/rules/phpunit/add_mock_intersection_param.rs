//! `lint/phpunit/add-mock-intersection-param`, ported from rector-phpunit's
//! `AddIntersectionParamToMockObjectParamRector` (rules/CodeQuality/Rector/Class_).
//!
//! In a test class, a private method's parameter typed `PHPUnit\Framework\MockObject\MockObject` (not variadic) gets
//! a `@param \X&\PHPUnit\Framework\MockObject\MockObject $name` tag when every `$this->method(...)` call in the
//! class's methods passes at its position a `createMock(X::class)` call, or a variable assigned one once in the
//! calling method; not when a `@param` tag names it already.

use mago_syntax::cst::{
    Argument, ClassLikeMember, Expression, Hint, Method, MethodBody, Modifier, Node, NodeKind,
    Variable,
};
use pun_problem::Safety;

use super::add_mock_intersection_type::mocked_class;
use super::annotations::Annotated;
use super::no_bare_mock_assign::scoped;
use super::phpdoc_type::typed_tag;
use super::{AnyCall, class_body, is_in_test_class, resolve};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// `PHPUnitClassName::MOCK_OBJECT`.
const MOCK_OBJECT: &str = "PHPUnit\\Framework\\MockObject\\MockObject";

pub struct AddMockIntersectionParam;

impl Rule for AddMockIntersectionParam {
    fn id(&self) -> &'static str {
        "lint/phpunit/add-mock-intersection-param"
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
        if !is_in_test_class(ctx, &scope, None) {
            return;
        }
        let methods: Vec<&Method<'_>> = members
            .iter()
            .filter_map(|m| match m {
                ClassLikeMember::Method(m) => Some(m),
                _ => None,
            })
            .collect();
        // collectMockObjectParamClassMethods: only private methods, as those can only be called from the very same
        // class
        let targets: Vec<&Method<'_>> = methods
            .iter()
            .copied()
            .filter(|m| {
                m.modifiers
                    .iter()
                    .any(|m| matches!(m, Modifier::Private(_)))
                    && matches!(m.body, MethodBody::Concrete(_))
                    && m.parameter_list
                        .parameters
                        .iter()
                        .any(|p| is_bare_mock_object_param(&scope, p))
            })
            .collect();
        if targets.is_empty() {
            return;
        }
        // resolveMockedClassesFromCallSites: per target and position, what each call passes
        let mut passed: Vec<Vec<Vec<Option<String>>>> =
            targets.iter().map(|_| Vec::new()).collect();
        for method in &methods {
            let MethodBody::Concrete(body) = &method.body else {
                continue;
            };
            let roots: Vec<Node<'_, '_>> = body.statements.iter().map(Node::Statement).collect();
            let variables = mocked_classes_by_variable_name(&scope, &roots);
            scoped(&roots, &mut |n| {
                let Node::MethodCall(call) = n else {
                    return;
                };
                if !matches!(unparenthesize(call.object), Expression::Variable(Variable::Direct(v)) if v.name.eq_ignore_ascii_case(b"$this"))
                {
                    return;
                }
                let Some(name) = AnyCall::of(n).and_then(|c| c.name()) else {
                    return;
                };
                let Some(at) = targets.iter().position(|t| t.name.value == name) else {
                    return;
                };
                for (position, arg) in call.argument_list.arguments.iter().enumerate() {
                    if passed[at].len() <= position {
                        passed[at].resize(position + 1, Vec::new());
                    }
                    passed[at][position].push(arg_mocked_class(&scope, arg, &variables));
                }
            });
        }
        let mut edits = Vec::new();
        for (at, method) in targets.iter().enumerate() {
            let annotated = Annotated::of(ctx.source, Node::Method(method), &scope).unwrap();
            let mut added: Vec<String> = Vec::new();
            for (position, param) in method.parameter_list.parameters.iter().enumerate() {
                if !is_bare_mock_object_param(&scope, param) {
                    continue;
                }
                // every call site must pass a mock of the very same class
                let Some(Some(class)) = passed[at]
                    .get(position)
                    .filter(|classes| {
                        !classes.is_empty() && classes.iter().all(|c| *c == classes[0])
                    })
                    .map(|classes| classes[0].clone())
                else {
                    continue;
                };
                let name = String::from_utf8_lossy(param.variable.name);
                // do not contradict an existing docblock type
                if let Some(doc) = &annotated.doc {
                    match typed_tag(doc, "@param", Some(&name)) {
                        Ok(None) => {}
                        Ok(Some(_)) => continue,
                        Err(()) => return,
                    }
                }
                added.push(format!("@param \\{class}&\\{MOCK_OBJECT} {name}"));
            }
            if added.is_empty() {
                continue;
            }
            let added: Vec<&str> = added.iter().map(String::as_str).collect();
            let new_doc = match &annotated.doc {
                Some(doc) => doc.print_appending(&vec![false; doc.len()], &added),
                None => format!("/**\n * {}\n */", added.join("\n * ")),
            };
            let Some(e) = annotated.edits(ctx.source, Some(&new_doc), &[]) else {
                return;
            };
            edits.extend(e);
        }
        if edits.is_empty() {
            return;
        }
        out.push(Report::fixed(
            node_span(&targets[0].name),
            "Add the mocked class to the mock parameter's @param as an intersection.",
            Safety::Safe,
            edits,
        ));
    }
}

/// `isBareMockObjectParam()`: not variadic, typed with the name `MockObject`.
fn is_bare_mock_object_param(
    scope: &[Node<'_, '_>],
    param: &mago_syntax::cst::FunctionLikeParameter<'_>,
) -> bool {
    param.ellipsis.is_none()
        && matches!(&param.hint, Some(Hint::Identifier(id)) if resolve(scope, id).eq_ignore_ascii_case(MOCK_OBJECT))
}

/// `resolveMockedClassesByVariableName()`: each variable assigned (outside closures) with the class of its
/// `createMock()` call; a variable assigned again has none.
fn mocked_classes_by_variable_name(
    scope: &[Node<'_, '_>],
    roots: &[Node<'_, '_>],
) -> Vec<(Vec<u8>, Option<String>)> {
    let mut variables: Vec<(Vec<u8>, Option<String>)> = Vec::new();
    scoped(roots, &mut |node| {
        let Node::Assignment(assign) = node else {
            return;
        };
        if !matches!(
            assign.operator,
            mago_syntax::cst::AssignmentOperator::Assign(_)
        ) {
            return;
        }
        let Expression::Variable(Variable::Direct(v)) = assign.lhs else {
            return;
        };
        let name = v.name[1..].to_vec();
        // re-assigned variable, the type is not reliable
        if let Some(entry) = variables.iter_mut().find(|(n, _)| *n == name) {
            entry.1 = None;
            return;
        }
        let class = create_mock_class(scope, assign.rhs);
        variables.push((name, class));
    });
    variables
}

/// `resolveArgMockedClass()`.
fn arg_mocked_class(
    scope: &[Node<'_, '_>],
    arg: &Argument<'_>,
    variables: &[(Vec<u8>, Option<String>)],
) -> Option<String> {
    // named args make the position unreliable
    let Argument::Positional(arg) = arg else {
        return None;
    };
    if arg.ellipsis.is_some() {
        return None;
    }
    if let Expression::Variable(variable) = arg.value {
        let Variable::Direct(v) = variable else {
            return None;
        };
        return variables
            .iter()
            .find(|(n, _)| *n == v.name[1..])
            .and_then(|(_, c)| c.clone());
    }
    create_mock_class(scope, arg.value)
}

/// `resolveCreateMockClass()`: the class of a `createMock(X::...)` method or static call.
fn create_mock_class(scope: &[Node<'_, '_>], expr: &Expression<'_>) -> Option<String> {
    let call = match expr {
        Expression::Call(mago_syntax::cst::Call::Method(c)) => AnyCall::of(Node::MethodCall(c)),
        Expression::Call(mago_syntax::cst::Call::StaticMethod(c)) => {
            AnyCall::of(Node::StaticMethodCall(c))
        }
        _ => None,
    }?;
    if !call.is_name(&["createMock"]) {
        return None;
    }
    mocked_class(scope, &call)
}
