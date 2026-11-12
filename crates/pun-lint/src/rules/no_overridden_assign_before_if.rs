//! `lint/dead-code/no-overridden-assign-before-if`, ported from Rector 2.8.4's
//! `RemoveOverriddenAssignBeforeIfElseRector` (rules/DeadCode/Rector/If_).
//!
//! `$result = [];` right before an `if`/`else` whose two branches both assign `$result` before reading it goes: the
//! first value is never read. Left alone: an `if` with an `elseif` or without an `else`, a condition that reads the
//! variable, a first value with a side effect, a variable that is `global`, `static`, a superglobal or taken by
//! reference, and an `if` with `compact()`, `extract()`, `get_defined_vars()` or a dynamic variable in it.

use mago_syntax::cst::{
    Expression, Identifier, IndirectVariable, Node, NodeKind, Statement, Variable,
};
use pun_problem::Safety;

use super::no_unused_assignment::is_native_variable;
use super::rector_code_quality::named;
use super::rector_stmts::{
    HOLDERS, List, assign, expression, holder_list, if_view, is_static_or_global,
    is_used_by_reference, remove, variable_name,
};
use super::side_effect::has_side_effect;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoOverriddenAssignBeforeIf;

impl Rule for NoOverriddenAssignBeforeIf {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-overridden-assign-before-if"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(HOLDERS)
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let raw = crate::syntax::statements(node);
        if !raw
            .windows(2)
            .any(|w| matches!((w[0], w[1]), (Statement::Expression(_), Statement::If(_))))
        {
            return;
        }
        let source = ctx.source;
        let Some(list) = holder_list(source, node, ancestors) else {
            return;
        };
        let mut scope = ancestors.to_vec();
        scope.push(node);
        for (key, pair) in list.stmts.windows(2).enumerate() {
            let (stmt, Statement::If(next)) = (pair[0], pair[1]) else {
                continue;
            };
            let view = if_view(source, next);
            let Some((else_stmts, _)) = &view.else_ else {
                continue;
            };
            if !view.elseifs.is_empty() {
                continue;
            }
            let Some(name) = overridable_assign_variable_name(&scope, stmt) else {
                continue;
            };
            // the initial value is still read when the condition uses it
            if is_variable_used(Node::Expression(view.condition), name) {
                continue;
            }
            // compact()/get_defined_vars()/dynamic variables can read the value by name
            if should_skip_if(Node::If(next)) {
                continue;
            }
            if !is_overridden_first_in_branch(&view.stmts, name)
                || !is_overridden_first_in_branch(else_stmts, name)
            {
                continue;
            }
            out.push(Report::fixed(
                node_span(stmt),
                "Both branches of the `if` after this assignment assign the variable again; remove it.",
                Safety::Unsafe,
                remove(ctx, &list, &[key]),
            ));
        }
    }
}

/// `matchOverridableAssignVariableName`; `scope` ends with the node that holds `stmt`.
fn overridable_assign_variable_name<'a>(
    scope: &[Node<'a, 'a>],
    stmt: &'a Statement<'a>,
) -> Option<&'a [u8]> {
    let assign = assign(expression(stmt)?)?;
    let name = variable_name(assign.lhs)?;
    // removing the assign must not drop a side effect
    if has_side_effect(scope, assign.rhs) {
        return None;
    }
    let mut chain = scope.to_vec();
    chain.push(Node::Statement(stmt));
    if is_static_or_global(&chain, name)
        || is_used_by_reference(scope, name, node_span(assign.lhs).start)
        || is_native_variable(name)
    {
        return None;
    }
    Some(name)
}

/// `isOverriddenFirstInBranch`: the first statement that touches the variable assigns it without reading it.
fn is_overridden_first_in_branch(stmts: &List<'_>, name: &[u8]) -> bool {
    let Some(stmt) = stmts
        .stmts
        .iter()
        .find(|s| is_variable_used(Node::Statement(s), name))
    else {
        return false;
    };
    // first statement touching the variable must fully override it
    let Some(assign) = expression(stmt).and_then(assign) else {
        return false;
    };
    // e.g. $value = $value + 1 still reads the previous value
    variable_name(assign.lhs) == Some(name) && !is_variable_used(Node::Expression(assign.rhs), name)
}

fn should_skip_if(r#if: Node<'_, '_>) -> bool {
    any_variable_node(r#if, &|node| match node {
        Node::FunctionCall(call) => ["compact", "get_defined_vars", "extract"]
            .iter()
            .any(|name| named(call, name)),
        // dynamic variable access like $$name can read the value by name
        Node::IndirectVariable(v) => interpolated_name(v).is_none(),
        Node::NestedVariable(_) => true,
        _ => false,
    })
}

/// `isVariableUsedInNode`: a variable named `name` (without the `$`) anywhere in `node`.
fn is_variable_used(node: Node<'_, '_>, name: &[u8]) -> bool {
    any_variable_node(node, &|n| match n {
        Node::DirectVariable(v) => &v.name[1..] == name,
        Node::IndirectVariable(v) => interpolated_name(v) == Some(name),
        _ => false,
    })
}

/// The name in `"${name}"`, which php-parser reads as the variable `$name`; elsewhere `${...}` holds an expression.
fn interpolated_name<'a>(variable: &IndirectVariable<'a>) -> Option<&'a [u8]> {
    match variable.expression {
        Expression::Identifier(Identifier::Local(id)) => Some(id.value),
        _ => None,
    }
}

/// [`any_node`] over php-parser's nodes: the `$name` of `A::$name` and of a property declaration is a name there
/// (`VarLikeIdentifier`), not a variable.
fn any_variable_node(node: Node<'_, '_>, f: &dyn Fn(Node<'_, '_>) -> bool) -> bool {
    let mut stack = vec![node];
    while let Some(node) = stack.pop() {
        if f(node) {
            return true;
        }
        match node {
            Node::StaticPropertyAccess(access) => {
                stack.push(Node::Expression(access.class));
                match &access.property {
                    Variable::Direct(_) => {}
                    Variable::Indirect(v) => stack.push(Node::Expression(v.expression)),
                    Variable::Nested(v) => stack.push(Node::Variable(v.variable)),
                }
            }
            Node::PropertyAbstractItem(_) => {}
            Node::PropertyConcreteItem(item) => stack.push(Node::Expression(item.value)),
            _ => node.visit_children(|child| stack.push(child)),
        }
    }
    false
}
