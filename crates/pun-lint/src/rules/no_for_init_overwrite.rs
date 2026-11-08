//! `lint/control-flow/no-for-init-overwrite`, ported from phpstan-strict-rules 2.0.12's
//! `OverwriteVariablesWithForLoopInitRule` (identifier `for.variableOverwrite`).
//!
//! A `for` loop's init assignment (`for ($i = 0; ...)`, or a destructuring one) is reported for each variable the
//! scope already has for sure (`$scope->hasVariableType()->yes()`). Pun reads that from the syntax, as PHPStan's
//! `NodeScopeResolver` merges scopes: a variable is defined when a parameter, a `use`d variable, an enclosing
//! `foreach`, `catch`, condition or `for` init, or a statement before the loop in the same function assigns it on
//! every path. Loops (their bodies may not run), blocks (`polluteScopeWithBlock` is off) and `unset()` leave it
//! undefined, as do a by-reference argument and a `try` without `finally` whose `catch` does not assign it, which
//! PHPStan resolves from function signatures and throw points.

use mago_syntax::cst::*;

use super::side_effect::any_node;
use crate::syntax::{node_span, statements, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoForInitOverwrite;

impl Rule for NoForInitOverwrite {
    fn id(&self) -> &'static str {
        "lint/control-flow/no-for-init-overwrite"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::For])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::For(r#for) = node else { return };
        for init in &r#for.initializations {
            if let Expression::Assignment(assign) = unparenthesize(init)
                && matches!(assign.operator, AssignmentOperator::Assign(_))
                && !matches!(assign.rhs, Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Reference(_)))
            {
                check_value_var(node, ancestors, assign.lhs, out);
            }
        }
    }
}

/// `checkValueVar()`: a variable the scope has, or the items of a `list()` or `[]` destructuring.
fn check_value_var(
    r#for: Node<'_, '_>,
    ancestors: &[Node<'_, '_>],
    expr: &Expression<'_>,
    out: &mut Vec<Report>,
) {
    match unparenthesize(expr) {
        Expression::Variable(Variable::Direct(v)) => {
            if defined(r#for, ancestors, v.name) {
                out.push(Report {
                    span: node_span(v),
                    message: format!(
                        "For loop initial assignment overwrites variable {}.",
                        String::from_utf8_lossy(v.name)
                    ),
                    help: None,
                    fix: None,
                });
            }
        }
        Expression::Array(Array { elements, .. }) | Expression::List(List { elements, .. }) => {
            for element in elements {
                match element {
                    ArrayElement::KeyValue(e) => check_value_var(r#for, ancestors, e.value, out),
                    ArrayElement::Value(e) => check_value_var(r#for, ancestors, e.value, out),
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

const SUPERGLOBALS: &[&[u8]] = &[
    b"$GLOBALS",
    b"$_SERVER",
    b"$_GET",
    b"$_POST",
    b"$_FILES",
    b"$_COOKIE",
    b"$_SESSION",
    b"$_REQUEST",
    b"$_ENV",
];

/// `$scope->hasVariableType($name)->yes()` before the `for` loop `node`.
pub(super) fn defined(node: Node<'_, '_>, ancestors: &[Node<'_, '_>], name: &[u8]) -> bool {
    if SUPERGLOBALS.contains(&name) {
        return true;
    }
    let start = node_span(&node).start;
    for (i, ancestor) in ancestors.iter().enumerate().rev() {
        let child = ancestors.get(i + 1).copied().unwrap_or(node);
        let found = match ancestor {
            Node::Function(f) => return has_parameter(&f.parameter_list, name),
            Node::Method(m) => return has_parameter(&m.parameter_list, name),
            Node::Closure(c) => {
                return has_parameter(&c.parameter_list, name)
                    || c.use_clause
                        .as_ref()
                        .is_some_and(|u| u.variables.iter().any(|v| v.variable.name == name));
            }
            Node::PropertyHook(h) => {
                return match &h.parameter_list {
                    Some(list) => has_parameter(list, name),
                    None => h.name.value == b"set" && name == b"$value",
                };
            }
            Node::If(x) => expr_defines(x.condition, name),
            Node::IfStatementBodyElseIfClause(x) => expr_defines(x.condition, name),
            Node::IfColonDelimitedBodyElseIfClause(x) => {
                expr_defines(x.condition, name) || preceding(*ancestor, child, start, name)
            }
            Node::While(x) => expr_defines(x.condition, name),
            Node::Switch(x) => expr_defines(x.expression, name),
            Node::For(x) => x
                .initializations
                .iter()
                .chain(x.conditions.iter())
                .any(|e| expr_defines(e, name)),
            Node::Foreach(x) => {
                expr_defines(x.expression, name)
                    || match &x.target {
                        ForeachTarget::Value(t) => assigns(t.value, name),
                        ForeachTarget::KeyValue(t) => {
                            assigns(t.key, name) || assigns(t.value, name)
                        }
                    }
            }
            Node::TryCatchClause(c) => c.variable.as_ref().is_some_and(|v| v.name == name),
            // A list of statements that holds the loop: the statements before it.
            Node::Program(_)
            | Node::Block(_)
            | Node::NamespaceImplicitBody(_)
            | Node::SwitchExpressionCase(_)
            | Node::SwitchDefaultCase(_)
            | Node::IfColonDelimitedBody(_)
            | Node::IfColonDelimitedBodyElseClause(_)
            | Node::WhileColonDelimitedBody(_)
            | Node::ForColonDelimitedBody(_)
            | Node::ForeachColonDelimitedBody(_)
            | Node::DeclareColonDelimitedBody(_) => preceding(*ancestor, child, start, name),
            _ => false,
        };
        if found {
            return true;
        }
    }
    false
}

/// Whether the statements of the list `holder` before `start` define `name`, when `child`, the node on the way to the
/// loop, is one of them.
fn preceding(holder: Node<'_, '_>, child: Node<'_, '_>, start: usize, name: &[u8]) -> bool {
    if !matches!(child, Node::Statement(_)) {
        return false;
    }
    let before = statements(holder);
    list_flow(
        before.into_iter().filter(|s| node_span(*s).end <= start),
        name,
    ) == Flow::Defines
}

fn has_parameter(list: &FunctionLikeParameterList<'_>, name: &[u8]) -> bool {
    list.parameters.iter().any(|p| p.variable.name == name)
}

/// What running code does to the variable: assigns it on every path that goes on, never goes on, or neither.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Flow {
    Defines,
    Exits,
    No,
}

/// Statements run in order: the last `unset()` or assignment wins, and a statement that never goes on ends the list.
fn list_flow<'a>(stmts: impl Iterator<Item = &'a Statement<'a>>, name: &[u8]) -> Flow {
    let mut state = Flow::No;
    for stmt in stmts {
        if unsets(stmt, name) {
            state = Flow::No;
        }
        match stmt_flow(stmt, name) {
            Flow::Exits => return Flow::Exits,
            Flow::Defines => state = Flow::Defines,
            Flow::No => {}
        }
    }
    state
}

/// The scopes of branches merged: defined when every branch that goes on defines it.
fn merge(flows: impl IntoIterator<Item = Flow>) -> Flow {
    let mut all_exit = true;
    for flow in flows {
        match flow {
            Flow::No => return Flow::No,
            Flow::Defines => all_exit = false,
            Flow::Exits => {}
        }
    }
    if all_exit { Flow::Exits } else { Flow::Defines }
}

/// The body of a control structure: a block's statements, or the one statement.
fn body_flow(stmt: &Statement<'_>, name: &[u8]) -> Flow {
    match stmt {
        Statement::Block(b) => list_flow(b.statements.iter(), name),
        _ => stmt_flow(stmt, name),
    }
}

fn stmt_flow(stmt: &Statement<'_>, name: &[u8]) -> Flow {
    match stmt {
        Statement::Expression(e) => match unparenthesize(e.expression) {
            Expression::Throw(_)
            | Expression::Construct(Construct::Exit(_) | Construct::Die(_)) => Flow::Exits,
            expr => defines_if(expr_defines(expr, name)),
        },
        Statement::Return(_) | Statement::Continue(_) | Statement::Break(_) => Flow::Exits,
        Statement::Echo(e) => defines_if(e.values.iter().any(|v| expr_defines(v, name))),
        Statement::Global(g) => defines_if(
            g.variables
                .iter()
                .any(|v| matches!(v, Variable::Direct(d) if d.name == name)),
        ),
        Statement::Static(s) => defines_if(s.items.iter().any(|item| item.variable().name == name)),
        Statement::Block(b) => match list_flow(b.statements.iter(), name) {
            Flow::Exits => Flow::Exits,
            _ => Flow::No,
        },
        Statement::If(x) => {
            if expr_defines(x.condition, name) {
                return Flow::Defines;
            }
            match &x.body {
                IfBody::Statement(b) => merge(
                    std::iter::once(body_flow(b.statement, name))
                        .chain(
                            b.else_if_clauses
                                .iter()
                                .map(|c| body_flow(c.statement, name)),
                        )
                        .chain([b
                            .else_clause
                            .as_ref()
                            .map_or(Flow::No, |c| body_flow(c.statement, name))]),
                ),
                IfBody::ColonDelimited(b) => merge(
                    std::iter::once(list_flow(b.statements.iter(), name))
                        .chain(
                            b.else_if_clauses
                                .iter()
                                .map(|c| list_flow(c.statements.iter(), name)),
                        )
                        .chain([b
                            .else_clause
                            .as_ref()
                            .map_or(Flow::No, |c| list_flow(c.statements.iter(), name))]),
                ),
            }
        }
        Statement::Switch(x) => {
            if expr_defines(x.expression, name) {
                return Flow::Defines;
            }
            if !x.body.has_default_case() {
                return Flow::No;
            }
            let cases = x.body.cases();
            merge((0..cases.len()).map(|i| case_path(&cases[i..], name)))
        }
        Statement::Try(x) => {
            if x.finally_clause
                .as_ref()
                .is_some_and(|f| list_flow(f.block.statements.iter(), name) == Flow::Defines)
            {
                return Flow::Defines;
            }
            merge(
                std::iter::once(list_flow(x.block.statements.iter(), name)).chain(
                    x.catch_clauses
                        .iter()
                        .map(|c| list_flow(c.block.statements.iter(), name)),
                ),
            )
        }
        Statement::DoWhile(x) => defines_if(
            body_flow(x.statement, name) == Flow::Defines || expr_defines(x.condition, name),
        ),
        _ => Flow::No,
    }
}

/// One way through a `switch`: from a case on, falling through the ones after it, to the first `break`.
fn case_path(cases: &[SwitchCase<'_>], name: &[u8]) -> Flow {
    let mut state = Flow::No;
    for case in cases {
        for stmt in case.statements() {
            if let Statement::Break(_) = stmt {
                return state;
            }
            // A `break` deeper down leaves the switch with whatever the path has assigned so far.
            if any_node(Node::Statement(stmt), &|n| matches!(n, Node::Break(_))) {
                return Flow::No;
            }
            if unsets(stmt, name) {
                state = Flow::No;
            }
            match stmt_flow(stmt, name) {
                Flow::Exits => return Flow::Exits,
                Flow::Defines => state = Flow::Defines,
                Flow::No => {}
            }
        }
    }
    state
}

fn defines_if(yes: bool) -> Flow {
    if yes { Flow::Defines } else { Flow::No }
}

/// Whether `stmt` holds `unset($name)`.
fn unsets(stmt: &Statement<'_>, name: &[u8]) -> bool {
    any_node(
        Node::Statement(stmt),
        &|n| matches!(n, Node::Unset(u) if u.values.iter().any(|v| matches!(unparenthesize(v), Expression::Variable(Variable::Direct(d)) if d.name == name))),
    )
}

/// Whether evaluating `expr` assigns `name` on every path: an assignment to it, or one inside an operand that always
/// runs (not the right side of `&&`, `||`, `??`, `and`, `or`, a ternary's branches, a `match` arm or a closure).
fn expr_defines(expr: &Expression<'_>, name: &[u8]) -> bool {
    node_defines(Node::Expression(expr), name)
}

fn node_defines(node: Node<'_, '_>, name: &[u8]) -> bool {
    match node {
        Node::Closure(_) | Node::ArrowFunction(_) | Node::AnonymousClass(_) => false,
        Node::Assignment(a) => assigns(a.lhs, name) || expr_defines(a.rhs, name),
        Node::Binary(b)
            if matches!(
                b.operator,
                BinaryOperator::And(_)
                    | BinaryOperator::Or(_)
                    | BinaryOperator::LowAnd(_)
                    | BinaryOperator::LowOr(_)
                    | BinaryOperator::NullCoalesce(_)
            ) =>
        {
            expr_defines(b.lhs, name)
        }
        Node::Conditional(c) => expr_defines(c.condition, name),
        Node::Match(m) => expr_defines(m.expression, name),
        _ => {
            let mut found = false;
            node.visit_children(|child| found = found || node_defines(child, name));
            found
        }
    }
}

/// Whether the assignment target `target` writes the variable `name`: the variable, an element of it, or a
/// destructuring that holds one of those.
fn assigns(target: &Expression<'_>, name: &[u8]) -> bool {
    match unparenthesize(target) {
        Expression::Variable(Variable::Direct(v)) => v.name == name,
        Expression::ArrayAccess(a) => assigns(a.array, name),
        Expression::ArrayAppend(a) => assigns(a.array, name),
        Expression::Array(Array { elements, .. }) | Expression::List(List { elements, .. }) => {
            elements.iter().any(|e| match e {
                ArrayElement::KeyValue(e) => assigns(e.value, name),
                ArrayElement::Value(e) => assigns(e.value, name),
                _ => false,
            })
        }
        _ => false,
    }
}
