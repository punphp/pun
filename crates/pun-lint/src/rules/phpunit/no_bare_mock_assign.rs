//! `lint/phpunit/no-bare-mock-assign`, ported from rector-phpunit's `BareCreateMockAssignToDirectUseRector`
//! (rules/CodeQuality/Rector/ClassMethod), with `AssignedMocksCollector`, `VariableFinder` and
//! `AssertMethodAnalyzer::detectTestCaseCall`.
//!
//! In a test class, a method or `foreach` whose statements (two or more) assign `$x = $any->createMock(X::class)` of
//! an existing, non-abstract class, where `$x` is used once at most and only as a direct argument of a method call,
//! static call or `new`, loses the assignment and passes the `createMock()` call there instead. A variable that is
//! the receiver of a method call, used in a closure's `use`, or passed to a `TestCase`'s assertion stays.
//! Variable names compare ASCII case-insensitively, as Rector's `isName` does.

use mago_codex::symbol::SymbolKind;
use mago_syntax::cst::{
    Call, Expression, ForeachBody, MethodBody, Node, NodeKind, PartialArgument, Variable,
};
use pun_problem::{Edit, Safety};

use super::prefer_this_call::NON_ASSERT;
use super::{
    ASSERT, AnyCall, TEST_CASE, arg_value, class_string, expr_name, is_in_test_class,
    own_class_reflection, receiver_is, report, text,
};
use crate::rules::rector_batch5::remove_statements;
use crate::rules::rector_stmts::{List, assign, block_list, body_list, expression, holder_list};
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoBareMockAssign;

impl Rule for NoBareMockAssign {
    fn id(&self) -> &'static str {
        "lint/phpunit/no-bare-mock-assign"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method, NodeKind::Foreach])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (list, method) = match node {
            Node::Method(method) => {
                let MethodBody::Concrete(block) = &method.body else {
                    return;
                };
                (block_list(ctx.source, block), Some(method))
            }
            _ => {
                let Node::Foreach(each) = node else { return };
                match &each.body {
                    ForeachBody::Statement(statement) => (body_list(ctx.source, statement), None),
                    // a property hook's body aside, a holder has a list
                    ForeachBody::ColonDelimited(body) => {
                        let holder = Node::ForeachColonDelimitedBody(body);
                        (holder_list(ctx.source, holder, ancestors).unwrap(), None)
                    }
                }
            }
        };
        if !is_in_test_class(ctx, ancestors, method) || list.len() < 2 {
            return;
        }
        // The nodes `findInstancesOfScoped([$node])` walks: a method's statements, or the `foreach` itself.
        let roots: Vec<Node<'_, '_>> = match node {
            Node::Method(_) => list.stmts.iter().map(|s| Node::Statement(s)).collect(),
            _ => vec![node],
        };
        let mut scope = ancestors.to_vec();
        scope.push(node);
        let names = collect(ctx, ancestors, &list);
        let mut removed = vec![false; list.stmts.len()];
        let mut edits = Vec::new();
        for name in names {
            if is_variable_used_as_part_of_method_call(&roots, &name)
                || is_used_more_often_than_in_call_like_args(&roots, &name)
                || is_used_in_closure(node, &name)
                || is_used_in_assert_call(ctx, &scope, &roots, &name)
            {
                continue;
            }
            // 1. remove initial assign
            let mut mock: Option<&str> = None;
            for (at, statement) in list.stmts.iter().enumerate() {
                if let Some(assign) = expression(statement).and_then(assign)
                    && create_mock_arg(assign.rhs).is_some()
                    && is_variable(assign.lhs, &name)
                {
                    removed[at] = true;
                    mock = Some(text(ctx, assign.rhs));
                    continue;
                }
                // nothing to process yet
                let Some(mock) = mock else {
                    continue;
                };
                // 2. replace variable with call-like args of new instance
                for value in call_like_args(&[Node::Statement(statement)]) {
                    if is_variable(value, &name) {
                        edits.push(Edit::new(node_span(value), mock));
                    }
                }
            }
        }
        if !removed.contains(&true) {
            return;
        }
        edits.extend(remove_statements(ctx.source, &list, &removed));
        let first = removed.iter().position(|r| *r).unwrap();
        out.push(report(
            node_span(list.stmts[first]),
            "Pass the mock directly instead of assigning it to a variable first.",
            Safety::Safe,
            edits,
        ));
    }
}

/// `AssignedMocksCollector::collect`: the names of the variables assigned a mock of an existing, non-abstract class,
/// in the order of their first assignment.
fn collect(ctx: &Context<'_>, ancestors: &[Node<'_, '_>], list: &List<'_>) -> Vec<String> {
    let facts = facts(ctx);
    let mut names: Vec<String> = Vec::new();
    for statement in &list.stmts {
        let Some(assign) = expression(statement).and_then(assign) else {
            continue;
        };
        let (Some(first_arg), Some(name)) =
            (create_mock_arg(assign.rhs), variable_name(assign.lhs))
        else {
            continue;
        };
        let Some(class) = class_string(ctx, ancestors, first_arg) else {
            continue;
        };
        let Some(metadata) = facts.codebase.get_class_like(class.as_bytes()) else {
            continue;
        };
        // these cannot be instantiated (BetterReflection's `isAbstract`: an `abstract` class, not an interface)
        if metadata.kind == SymbolKind::Class && metadata.flags.is_abstract() {
            continue;
        }
        let name = String::from_utf8_lossy(name).into_owned();
        if !names.contains(&name) {
            names.push(name);
        }
    }
    names
}

/// `matchCreateMockArgAssignedToVariable` without the variable check: the first argument of a `createMock()` method
/// call.
fn create_mock_arg<'a>(expr: &'a Expression<'a>) -> Option<&'a Expression<'a>> {
    let Expression::Call(Call::Method(call)) = unparenthesize(expr) else {
        return None;
    };
    let call = AnyCall::of(Node::MethodCall(call))?;
    if !call.is_name(&["createMock"]) {
        return None;
    }
    Some(arg_value(call.args().into_iter().next()?))
}

/// A plain variable's name, without the `$`.
fn variable_name<'a>(expr: &'a Expression<'a>) -> Option<&'a [u8]> {
    match unparenthesize(expr) {
        Expression::Variable(Variable::Direct(v)) => Some(&v.name[1..]),
        _ => None,
    }
}

/// `$expr instanceof Variable && isName($expr, $name)`.
fn is_variable(expr: &Expression<'_>, name: &str) -> bool {
    variable_name(expr).is_some_and(|n| n.eq_ignore_ascii_case(name.as_bytes()))
}

/// `BetterNodeFinder::findInstancesOfScoped`: each node under `roots` (the roots included), not entering a class, a
/// closure or a function (an arrow function is entered; an anonymous class's arguments are too).
pub(super) fn scoped<'a>(roots: &[Node<'a, 'a>], found: &mut dyn FnMut(Node<'a, 'a>)) {
    let mut stack: Vec<Node<'a, 'a>> = roots.iter().rev().copied().collect();
    while let Some(node) = stack.pop() {
        match node {
            Node::Class(_) | Node::Closure(_) | Node::Function(_) => continue,
            Node::AnonymousClass(class) => {
                found(node);
                if let Some(list) = &class.argument_list {
                    stack.push(Node::PartialArgumentList(list));
                }
                continue;
            }
            _ => {}
        }
        found(node);
        let mut children = Vec::new();
        node.visit_children(|child| children.push(child));
        stack.extend(children.into_iter().rev());
    }
}

/// `isVariableUsedAsPartOfMethodCall`.
fn is_variable_used_as_part_of_method_call(roots: &[Node<'_, '_>], name: &str) -> bool {
    let mut used = false;
    scoped(roots, &mut |node| {
        if let Node::MethodCall(call) = node
            && expr_name(call.object).is_some_and(|n| n.eq_ignore_ascii_case(name.as_bytes()))
        {
            used = true;
        }
    });
    used
}

/// `findCallLikes`, as the values of their arguments: the method calls, static calls and `new`s (first-class
/// callables are not calls in Mago).
fn call_like_args<'a>(roots: &[Node<'a, 'a>]) -> Vec<&'a Expression<'a>> {
    let mut values = Vec::new();
    scoped(roots, &mut |node| match node {
        Node::MethodCall(call) => values.extend(call.argument_list.arguments.iter().map(arg_value)),
        Node::StaticMethodCall(call) => {
            values.extend(call.argument_list.arguments.iter().map(arg_value))
        }
        Node::Instantiation(new) => values.extend(
            new.argument_list
                .iter()
                .flat_map(|l| l.arguments.iter())
                .map(arg_value),
        ),
        Node::AnonymousClass(new) => values.extend(
            new.argument_list
                .iter()
                .flat_map(|l| l.arguments.iter())
                .filter_map(PartialArgument::value),
        ),
        _ => {}
    });
    values
}

/// `isUsedMoreOftenThanInCallLikeArgs`, with `VariableFinder::find`.
fn is_used_more_often_than_in_call_like_args(roots: &[Node<'_, '_>], name: &str) -> bool {
    let mut found_variables = 0;
    scoped(roots, &mut |node| {
        if let Node::DirectVariable(v) = node
            && v.name[1..].eq_ignore_ascii_case(name.as_bytes())
        {
            found_variables += 1;
        }
    });
    let call_like_variable_use_count = call_like_args(roots)
        .into_iter()
        .filter(|value| is_variable(value, name))
        .count();
    call_like_variable_use_count > 1 || found_variables - 1 > call_like_variable_use_count
}

/// `isUsedInClosure`: any closure's `use` of the variable, however deep.
fn is_used_in_closure(node: Node<'_, '_>, name: &str) -> bool {
    let mut stack = vec![node];
    while let Some(node) = stack.pop() {
        if let Node::ClosureUseClauseVariable(v) = node
            && v.variable.name[1..].eq_ignore_ascii_case(name.as_bytes())
        {
            return true;
        }
        node.visit_children(|child| stack.push(child));
    }
    false
}

/// `isUsedInAssertCall`: the variable is a direct argument of a call `detectTestCaseCall` accepts.
fn is_used_in_assert_call(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    roots: &[Node<'_, '_>],
    name: &str,
) -> bool {
    let mut used = false;
    scoped(roots, &mut |node| {
        if used || !matches!(node, Node::MethodCall(_) | Node::StaticMethodCall(_)) {
            return;
        }
        let call = AnyCall::of(node).unwrap();
        used = detect_test_case_call(ctx, ancestors, &call)
            && call
                .args()
                .into_iter()
                .any(|arg| is_variable(arg_value(arg), name));
    });
    used
}

/// `AssertMethodAnalyzer::detectTestCaseCall`: the receiver is a `TestCase`, the method an assertion (or one of
/// `NonAssertNonStaticMethods`) that the scope's class has from `TestCase` or `Assert`, and a static call's class is
/// `self` or `static`.
fn detect_test_case_call(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    call: &AnyCall<'_>,
) -> bool {
    if !receiver_is(ctx, ancestors, call, &[TEST_CASE]) {
        return false;
    }
    let Some(method) = call.name() else {
        return false;
    };
    if !method.starts_with(b"assert") && !NON_ASSERT.iter().any(|n| n.as_bytes() == method) {
        return false;
    }
    if call.is_static
        && !matches!(
            unparenthesize(call.receiver),
            Expression::Self_(_) | Expression::Static(_)
        )
    {
        return false;
    }
    let facts = facts(ctx);
    own_class_reflection(ctx, ancestors)
        .and_then(|class| facts.codebase.get_class_like(class.as_bytes()))
        .and_then(|metadata| {
            metadata
                .declaring_method_ids
                .iter()
                .find(|(m, _)| m.as_bytes().eq_ignore_ascii_case(method))
        })
        .is_some_and(|(_, id)| {
            let declaring = id.get_class_name();
            [TEST_CASE, ASSERT]
                .iter()
                .any(|c| c.as_bytes().eq_ignore_ascii_case(declaring.as_bytes()))
        })
}
