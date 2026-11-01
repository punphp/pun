//! OverwrittenExitPointByFinallyRule and the exit points its virtual node reads.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use super::super::Checker;
use super::super::types::flow::unparen;

#[derive(Default)]
pub(super) struct Result {
    always: bool,
    /// `None`: a synthetic `break`/`continue` (getExitPointsForOuterLoop()), without a position.
    pub(super) exits: Vec<(Option<Span>, &'static str)>,
    pub(super) jumps: Vec<(Option<Span>, &'static str, u64)>,
}

/// PHPStan only builds FinallyExitPointsNode for an always-terminating finally block.
pub(super) fn check(c: &mut Checker, t: &Try) {
    if c.level < 4 {
        return;
    }
    let Some(finally) = &t.finally_clause else {
        return;
    };
    let mut after = statements(c, finally.block.statements.as_slice());
    after
        .exits
        .extend(after.jumps.iter().map(|(s, k, _)| (*s, *k)));
    if !after.always || after.exits.is_empty() {
        return;
    }
    let mut before = statements(c, t.block.statements.as_slice());
    for catch in &t.catch_clauses {
        let branch = statements(c, catch.block.statements.as_slice());
        before.exits.extend(branch.exits);
        before.jumps.extend(branch.jumps);
    }
    before
        .exits
        .extend(before.jumps.iter().map(|(s, k, _)| (*s, *k)));
    if before.exits.is_empty() {
        return;
    }
    let messages = before
        .exits
        .into_iter()
        .map(|(span, kind)| {
            (
                span,
                format!(
                    "This {kind} is overwritten by a different one in the finally block below."
                ),
            )
        })
        .chain(
            after
                .exits
                .into_iter()
                .map(|(span, kind)| (span, format!("The overwriting {kind} is on this line."))),
        );
    for (span, message) in messages {
        // PHPStan's line of a node without a position
        let line = span.map_or(-1, |span| c.line_of(span));
        c.report_at_line(line, "finally.exitPoint", message, None);
    }
}

/// `isAlwaysTerminating()` of an expression's result: a `throw`, `exit` or never-returning call that is evaluated
/// whatever happens (not in a closure, a short-circuited operand or a branch).
fn terminates(c: &Checker, e: &Expression) -> bool {
    match e {
        Expression::Throw(_) | Expression::Construct(Construct::Exit(_) | Construct::Die(_)) => {
            true
        }
        Expression::Closure(_) | Expression::ArrowFunction(_) => false,
        Expression::Binary(b)
            if matches!(
                b.operator,
                BinaryOperator::And(_)
                    | BinaryOperator::Or(_)
                    | BinaryOperator::LowAnd(_)
                    | BinaryOperator::LowOr(_)
            ) =>
        {
            terminates(c, b.lhs)
        }
        // the right side runs when the left is always null
        Expression::Binary(b) if matches!(b.operator, BinaryOperator::NullCoalesce(_)) => {
            terminates(c, b.lhs)
                || (c.type_of(b.lhs).is_some_and(|t| t.is_null()) && terminates(c, b.rhs))
        }
        Expression::Assignment(a) if matches!(a.operator, AssignmentOperator::Coalesce(_)) => {
            terminates(c, a.lhs)
                || (c.type_of(a.lhs).is_some_and(|t| t.is_null()) && terminates(c, a.rhs))
        }
        Expression::Conditional(t) => terminates(c, t.condition),
        Expression::Match(m) => terminates(c, m.expression),
        Expression::Call(Call::NullSafeMethod(_)) => false,
        Expression::Access(Access::NullSafeProperty(p)) => terminates(c, p.object),
        Expression::Call(_) if c.call_terminates(e) => true,
        _ => Node::Expression(e)
            .children()
            .into_iter()
            .any(|n| node_terminates(c, n)),
    }
}

fn node_terminates(c: &Checker, n: Node) -> bool {
    match n {
        Node::Expression(e) => terminates(c, e),
        Node::AnonymousClass(a) => a.argument_list.as_ref().is_some_and(|args| {
            Node::PartialArgumentList(args)
                .children()
                .into_iter()
                .any(|n| node_terminates(c, n))
        }),
        n => n.children().into_iter().any(|n| node_terminates(c, n)),
    }
}

/// `isAlwaysTerminating()` of a statement's result.
pub(super) fn always_terminates(c: &Checker, s: &Statement) -> bool {
    statement(c, s).always
}

pub(super) fn statements(c: &Checker, items: &[Statement]) -> Result {
    let mut result = Result::default();
    for item in items {
        let next = statement(c, item);
        result.exits.extend(next.exits);
        result.jumps.extend(next.jumps);
        if next.always {
            result.always = true;
            break;
        }
    }
    result
}

fn bool_type(c: &Checker, e: &Expression) -> Option<bool> {
    match unparen(e) {
        Expression::Literal(Literal::True(_)) => Some(true),
        Expression::Literal(Literal::False(_) | Literal::Null(_)) => Some(false),
        _ => c.type_of(e).and_then(|t| {
            if t.is_always_truthy() {
                Some(true)
            } else if t.is_always_falsy() {
                Some(false)
            } else {
                None
            }
        }),
    }
}

fn statement(c: &Checker, s: &Statement) -> Result {
    let kind = match s {
        Statement::Return(_) => Some("return"),
        Statement::Expression(e) => match unparen(e.expression) {
            Expression::Throw(_) => Some("throw"),
            _ if c.type_of(e.expression).is_some_and(|t| t.is_explicit_never()) => Some("exit point"),
            _ if terminates(c, e.expression) => Some("exit point"),
            _ => None,
        },
        Statement::Echo(e) if e.values.iter().any(|v| terminates(c, v)) => Some("exit point"),
        _ => None,
    };
    if let Some(kind) = kind {
        return Result {
            always: true,
            exits: vec![(Some(s.span()), kind)],
            ..Result::default()
        };
    }
    match s {
        Statement::Break(b) => Result {
            always: true,
            jumps: vec![(Some(s.span()), "break", level(b.level))],
            ..Result::default()
        },
        Statement::Continue(b) => Result {
            always: true,
            jumps: vec![(Some(s.span()), "continue", level(b.level))],
            ..Result::default()
        },
        Statement::Block(b) => statements(c, b.statements.as_slice()),
        Statement::Goto(_) => Result {
            always: true,
            ..Result::default()
        },
        Statement::If(i) => {
            let mut branches = vec![(Some(i.condition), i.body.statements())];
            branches.extend(
                i.body
                    .else_if_clauses()
                    .into_iter()
                    .map(|(e, s)| (Some(e), s)),
            );
            if let Some(s) = i.body.else_statements() {
                branches.push((None, s));
            }
            let mut result = Result {
                always: true,
                ..Result::default()
            };
            let mut exhaustive = false;
            for (condition, body) in branches {
                let known = condition.and_then(|e| bool_type(c, e));
                if known == Some(false) {
                    continue;
                }
                let branch = statements(c, body);
                result.always &= branch.always;
                result.exits.extend(branch.exits);
                result.jumps.extend(branch.jumps);
                if condition.is_none() || known == Some(true) {
                    exhaustive = true;
                    break;
                }
            }
            result.always &= exhaustive;
            result
        }
        Statement::Try(t) => {
            let mut result = statements(c, t.block.statements.as_slice());
            for catch in &t.catch_clauses {
                let branch = statements(c, catch.block.statements.as_slice());
                result.always &= branch.always;
                result.exits.extend(branch.exits);
                result.jumps.extend(branch.jumps);
            }
            if let Some(finally) = &t.finally_clause {
                let branch = statements(c, finally.block.statements.as_slice());
                result.always |= branch.always;
                result.exits.extend(branch.exits);
                result.jumps.extend(branch.jumps);
            }
            result
        }
        Statement::While(w) => loop_body(c, w.body.statements(), bool_type(c, w.condition), false),
        Statement::For(f) => loop_body(
            c,
            f.body.statements(),
            f.conditions
                .iter()
                .last()
                .map_or(Some(true), |e| bool_type(c, e)),
            false,
        ),
        Statement::Foreach(f) => loop_body(c, f.body.statements(), None, false),
        Statement::DoWhile(d) => loop_body(
            c,
            std::slice::from_ref(d.statement),
            bool_type(c, d.condition),
            true,
        ),
        Statement::Switch(s) => {
            let mut result = Result {
                always: s.body.has_default_case(),
                ..Result::default()
            };
            let mut fallthrough = false;
            for case in s.body.cases().iter().rev() {
                let mut branch = statements(c, case.statements());
                let breaks = branch.jumps.iter().any(|(_, _, n)| *n == 1);
                fallthrough = (branch.always || fallthrough) && !breaks;
                result.always &= fallthrough;
                outer_loop(&mut branch.jumps);
                result.exits.extend(branch.exits);
                result.jumps.extend(branch.jumps);
            }
            result
        }
        _ => Result::default(),
    }
}

fn loop_body(c: &Checker, s: &[Statement], condition: Option<bool>, do_while: bool) -> Result {
    if condition == Some(false) && !do_while {
        return Result::default();
    }
    let mut result = statements(c, s);
    let breaks = result
        .jumps
        .iter()
        .any(|(_, k, n)| *k == "break" && *n == 1);
    let continues = result
        .jumps
        .iter()
        .any(|(_, k, n)| *k == "continue" && *n == 1);
    result.always = !breaks && (condition == Some(true) || do_while && result.always && !continues);
    outer_loop(&mut result.jumps);
    result
}

/// getExitPointsForOuterLoop(): drop level 1; decrement above 2, otherwise retain a synthetic level 1.
fn outer_loop(jumps: &mut Vec<(Option<Span>, &'static str, u64)>) {
    jumps.retain_mut(|(span, _, n)| {
        if *n == 1 {
            return false;
        }
        *n = if *n > 2 { *n - 1 } else { 1 };
        *span = None;
        true
    });
}

fn level(e: Option<&Expression>) -> u64 {
    match e.map(unparen) {
        Some(Expression::Literal(Literal::Integer(i))) => i.value.unwrap_or(1),
        _ => 1,
    }
}

/// The expression types NodeScopeResolver reads to find never-returning calls and constant branches.
pub(super) fn typed(program: &Program) -> Vec<Span> {
    struct Wanted {
        spans: Vec<Span>,
    }
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Wanted {
        fn walk_in_call(&mut self, call: &'ast Call<'arena>, _: &mut ()) {
            self.spans.push(call.span());
        }
        fn walk_in_statement_expression(&mut self, s: &'ast ExpressionStatement<'arena>, _: &mut ()) {
            // StatementHandler also terminates on the completed value, independently
            // of a nested expression's isAlwaysTerminating flag (notably `new`).
            self.spans.push(s.expression.span());
        }
        fn walk_in_binary(&mut self, b: &'ast Binary<'arena>, _: &mut ()) {
            if matches!(b.operator, BinaryOperator::NullCoalesce(_)) {
                self.spans.push(b.lhs.span());
            }
        }
        fn walk_in_assignment(&mut self, a: &'ast Assignment<'arena>, _: &mut ()) {
            if matches!(a.operator, AssignmentOperator::Coalesce(_)) {
                self.spans.push(a.lhs.span());
            }
        }
        fn walk_in_if(&mut self, i: &'ast If<'arena>, _: &mut ()) {
            self.spans.push(i.condition.span());
            self.spans
                .extend(i.body.else_if_clauses().iter().map(|(e, _)| e.span()));
        }
        fn walk_in_while(&mut self, w: &'ast While<'arena>, _: &mut ()) {
            self.spans.push(w.condition.span());
        }
        fn walk_in_do_while(&mut self, w: &'ast DoWhile<'arena>, _: &mut ()) {
            self.spans.push(w.condition.span());
        }
        fn walk_in_for(&mut self, f: &'ast For<'arena>, _: &mut ()) {
            self.spans.extend(f.conditions.iter().map(|e| e.span()));
        }
    }
    let mut wanted = Wanted { spans: Vec::new() };
    wanted.walk_program(program, &mut ());
    wanted.spans
}
