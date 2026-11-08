//! Rector 2.8.4's `SilentVoidResolver::hasSilentVoid` (rules/TypeDeclaration/TypeInferer): whether the end of a
//! function-like's body can be reached, so that it returns nothing there.

use mago_syntax::cst::{
    Construct, DoWhile, Expression, IfBody, Literal, Node, Statement, SwitchBody, SwitchCase,
    While, WhileBody,
};

use super::ClassScope;
use super::facts::Facts;
use super::never::is_native_never;

/// What the statements are looked at with: the codebase, and where they are.
pub(crate) struct Place<'a, 'ast, 'arena> {
    pub facts: &'a Facts,
    pub ancestors: &'a [Node<'ast, 'arena>],
    pub scope: Option<&'a ClassScope>,
}

/// `hasSilentVoid` for a body's statements.
pub(crate) fn has_silent_void<'s>(
    place: &Place<'_, '_, '_>,
    statements: impl IntoIterator<Item = &'s Statement<'s>>,
) -> bool {
    !always_returns_or_exits(place, false, statements)
}

/// `hasSilentVoid($functionLike, false)`: the `never` of a statement read with docblocks (`getType()`).
pub(crate) fn has_silent_void_documented<'s>(
    place: &Place<'_, '_, '_>,
    statements: impl IntoIterator<Item = &'s Statement<'s>>,
) -> bool {
    !always_returns_or_exits(place, true, statements)
}

/// The statements a statement body holds, as php-parser lists them: a block's statements, or the statement alone.
fn body<'s>(statement: &'s Statement<'s>) -> Vec<&'s Statement<'s>> {
    match statement {
        Statement::Block(block) => block.statements.iter().collect(),
        other => vec![other],
    }
}

/// `hasStmtsAlwaysReturnOrExit`.
fn always_returns_or_exits<'s>(
    place: &Place<'_, '_, '_>,
    documented: bool,
    statements: impl IntoIterator<Item = &'s Statement<'s>>,
) -> bool {
    statements.into_iter().any(|statement| match statement {
        Statement::Expression(e) => {
            matches!(
                e.expression,
                Expression::Throw(_)
                    | Expression::Yield(_)
                    | Expression::Construct(Construct::Exit(_) | Construct::Die(_))
            ) || is_native_never(place.facts, place.ancestors, place.scope, e.expression)
                || (documented
                    && (place
                        .facts
                        .full_type(crate::syntax::node_span(e.expression)))
                    .is_some_and(mago_codex::ttype::union::TUnion::is_never))
        }
        Statement::Return(r) => r.value.is_some(),
        Statement::Switch(switch) => switch_always_returns(place, documented, switch),
        Statement::Try(r#try) => {
            let finally = r#try.finally_clause.as_ref().is_some_and(|f| {
                always_returns_or_exits(place, documented, f.block.statements.iter())
            });
            if !always_returns_or_exits(place, documented, r#try.block.statements.iter()) {
                return finally;
            }
            r#try.catch_clauses.iter().all(|c| {
                finally || always_returns_or_exits(place, documented, c.block.statements.iter())
            })
        }
        Statement::If(r#if) => if_always_returns(place, documented, &r#if.body),
        Statement::While(r#while) => while_always_returns(r#while),
        Statement::DoWhile(r#do) => do_always_returns(place, documented, r#do),
        _ => false,
    })
}

/// `isIfReturn`: every branch, with an `else`, always returns or exits.
fn if_always_returns(place: &Place<'_, '_, '_>, documented: bool, body: &IfBody<'_>) -> bool {
    match body {
        IfBody::Statement(b) => {
            b.else_if_clauses
                .iter()
                .all(|c| always_returns_or_exits(place, documented, self::body(c.statement)))
                && b.else_clause.as_ref().is_some_and(|e| {
                    always_returns_or_exits(place, documented, self::body(b.statement))
                        && always_returns_or_exits(place, documented, self::body(e.statement))
                })
        }
        IfBody::ColonDelimited(b) => {
            b.else_if_clauses
                .iter()
                .all(|c| always_returns_or_exits(place, documented, c.statements.iter()))
                && b.else_clause.as_ref().is_some_and(|e| {
                    always_returns_or_exits(place, documented, b.statements.iter())
                        && always_returns_or_exits(place, documented, e.statements.iter())
                })
        }
    }
}

/// `isSwitchWithAlwaysReturnOrExit`: a `default` with statements, and every case with statements always returns or
/// exits.
fn switch_always_returns(
    place: &Place<'_, '_, '_>,
    documented: bool,
    switch: &mago_syntax::cst::Switch<'_>,
) -> bool {
    let cases = match &switch.body {
        SwitchBody::BraceDelimited(b) => &b.cases,
        SwitchBody::ColonDelimited(b) => &b.cases,
    };
    let statements = |case: &SwitchCase<'_>| match case {
        SwitchCase::Expression(c) => c.statements.len(),
        SwitchCase::Default(c) => c.statements.len(),
    };
    let has_default = cases
        .iter()
        .find(|c| matches!(c, SwitchCase::Default(_)))
        .is_some_and(|c| statements(c) > 0);
    if !has_default {
        return false;
    }
    let filled = cases.iter().filter(|c| statements(c) > 0).count();
    let returning = cases
        .iter()
        .filter(|c| match c {
            SwitchCase::Expression(c) => {
                always_returns_or_exits(place, documented, c.statements.iter())
            }
            SwitchCase::Default(c) => {
                always_returns_or_exits(place, documented, c.statements.iter())
            }
        })
        .count();
    filled == returning
}

/// `ValueResolver::isTrue` for a loop condition.
fn is_true(condition: &Expression<'_>) -> bool {
    matches!(condition, Expression::Literal(Literal::True(_)))
}

/// `isDoOrWhileWithAlwaysReturnOrExit` for a `while`: an endless one without `break`, `continue` or `goto`.
fn while_always_returns(r#while: &While<'_>) -> bool {
    let statements: Vec<&Statement<'_>> = match &r#while.body {
        WhileBody::Statement(s) => body(s),
        WhileBody::ColonDelimited(b) => b.statements.iter().collect(),
    };
    is_true(r#while.condition) && !has_loop_control(&statements)
}

/// `isDoOrWhileWithAlwaysReturnOrExit` for a `do`-`while`.
fn do_always_returns(place: &Place<'_, '_, '_>, documented: bool, r#do: &DoWhile<'_>) -> bool {
    let statements = body(r#do.statement);
    if !is_true(r#do.condition)
        && !always_returns_or_exits(place, documented, statements.iter().copied())
    {
        return false;
    }
    !has_loop_control(&statements)
}

/// `isFoundLoopControl`: a `break`, `continue` or `goto` outside nested classes, functions and closures.
fn has_loop_control(statements: &[&Statement<'_>]) -> bool {
    fn walk(node: Node<'_, '_>) -> bool {
        match node {
            Node::Break(_) | Node::Continue(_) | Node::Goto(_) => true,
            Node::Class(_) | Node::AnonymousClass(_) | Node::Function(_) | Node::Closure(_) => {
                false
            }
            _ => {
                let mut found = false;
                node.visit_children(|child| found = found || walk(child));
                found
            }
        }
    }
    statements.iter().any(|s| walk(Node::Statement(s)))
}
