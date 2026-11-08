//! AssignToByRefExprFromForeachRule (level 1, `featureToggles.assignToByRefForeachExpr`): a VariableAssignNode of a
//! variable a `foreach (... as &$var)` left bound to the array's last element.
//!
//! ForeachHandler assigns the virtual `ForeachValueByRefExpr($valueVar)` after the loop, UnsetHandler invalidates it
//! and nothing else does (assigning the variable keeps it: the virtual node has no sub-nodes). The rule reports when
//! the scope may have it. Loop bodies are analysed with it only from their entry (the convergence passes run in a
//! deep context, which assigns nothing), so it only flows forward: [`entries()`] gives each statement the variables
//! that may hold it when the statement starts.

use std::collections::BTreeSet;
use std::sync::Arc;

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use super::Checker;
use super::types::flow::unparen;

pub(super) const IDENTIFIERS: &[&str] = &["assign.byRefForeachExpr"];

type Set = BTreeSet<Vec<u8>>;

/// Each statement's span with the variables that may be bound by reference at its start.
pub(super) type Entries = Vec<(Span, Arc<Set>)>;

pub(super) fn entries(program: &Program) -> Entries {
    let mut flow = Flow::default();
    flow.statements(program.statements.as_slice(), Set::new());
    let mut bodies = Bodies(&mut flow);
    bodies.walk_program(program, &mut ());
    flow.entries
}

/// The variable assignment `v` (a VariableAssignNode).
pub(super) fn variable_assign(c: &mut Checker, v: &DirectVariable) {
    let span = v.span();
    let Some((_, set)) = c
        .by_ref_foreach
        .iter()
        .filter(|(s, _)| s.start.offset <= span.start.offset && span.end.offset <= s.end.offset)
        .min_by_key(|(s, _)| s.end.offset - s.start.offset)
    else {
        return;
    };
    if !set.contains(v.name) {
        return;
    }
    let message = format!(
        "Assign to {} overwrites the last element from array.",
        String::from_utf8_lossy(v.name)
    );
    c.report(
        v,
        "assign.byRefForeachExpr",
        message,
        Some("Unset it right after foreach to avoid this problem."),
    );
}

/// Every function-like body starts without the virtual expression.
struct Bodies<'f>(&'f mut Flow);

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Bodies<'_> {
    fn walk_in_function(&mut self, f: &'ast Function<'arena>, (): &mut ()) {
        self.0.statements(f.body.statements.as_slice(), Set::new());
    }
    fn walk_in_method(&mut self, m: &'ast Method<'arena>, (): &mut ()) {
        if let MethodBody::Concrete(b) = &m.body {
            self.0.statements(b.statements.as_slice(), Set::new());
        }
    }
    fn walk_in_closure(&mut self, f: &'ast Closure<'arena>, (): &mut ()) {
        self.0.statements(f.body.statements.as_slice(), Set::new());
    }
    fn walk_in_property_hook(&mut self, h: &'ast PropertyHook<'arena>, (): &mut ()) {
        if let PropertyHookBody::Concrete(PropertyHookConcreteBody::Block(b)) = &h.body {
            self.0.statements(b.statements.as_slice(), Set::new());
        }
    }
}

#[derive(Default)]
struct Flow {
    entries: Entries,
    /// The scopes of the break and continue exit points of each enclosing loop or switch.
    exits: Vec<Set>,
}

fn union(a: Option<Set>, b: Option<Set>) -> Option<Set> {
    match (a, b) {
        (Some(mut a), Some(b)) => {
            a.extend(b);
            Some(a)
        }
        (a, b) => a.or(b),
    }
}

impl Flow {
    /// The statements from `flags`; `None` when their end is unreachable.
    fn statements(&mut self, stmts: &[Statement], flags: Set) -> Option<Set> {
        let mut flags = Some(flags);
        for s in stmts {
            match flags {
                Some(f) => flags = self.statement(s, f),
                // processStmtNodes() skips the statements after an always terminating one
                None => self.entries.push((s.span(), Arc::new(Set::new()))),
            }
        }
        flags
    }

    fn body(&mut self, s: &Statement, flags: Set) -> Option<Set> {
        self.statements(std::slice::from_ref(s), flags)
    }

    /// A loop body: its end with its continue and break exit points.
    fn loop_body(
        &mut self,
        run: impl FnOnce(&mut Flow) -> Option<Set>,
    ) -> (Option<Set>, Option<Set>) {
        self.exits.push(Set::new());
        let end = run(self);
        let exits = self.exits.pop().filter(|e| !e.is_empty());
        (end, exits)
    }

    fn exit(&mut self, flags: &Set, level: Option<&Expression>) {
        let n = match level.map(unparen) {
            Some(Expression::Literal(Literal::Integer(i))) => i.value.unwrap_or(1).max(1) as usize,
            _ => 1,
        };
        if let Some(i) = self.exits.len().checked_sub(n) {
            self.exits[i].extend(flags.iter().cloned());
        }
    }

    fn statement(&mut self, s: &Statement, flags: Set) -> Option<Set> {
        self.entries.push((s.span(), Arc::new(flags.clone())));
        match s {
            Statement::Return(_) | Statement::Goto(_) => None,
            Statement::Break(b) => {
                self.exit(&flags, b.level);
                None
            }
            Statement::Continue(k) => {
                self.exit(&flags, k.level);
                None
            }
            Statement::Expression(e)
                if matches!(
                    unparen(e.expression),
                    Expression::Throw(_)
                        | Expression::Construct(Construct::Exit(_) | Construct::Die(_))
                ) =>
            {
                None
            }
            Statement::Block(b) => self.statements(b.statements.as_slice(), flags),
            Statement::Namespace(n) => match &n.body {
                NamespaceBody::Implicit(b) => self.statements(b.statements.as_slice(), flags),
                NamespaceBody::BraceDelimited(b) => self.statements(b.statements.as_slice(), flags),
            },
            Statement::Declare(d) => match &d.body {
                DeclareBody::Statement(s) => self.body(s, flags),
                DeclareBody::ColonDelimited(b) => self.statements(b.statements.as_slice(), flags),
            },
            Statement::If(i) => {
                let (mut out, has_else) = match &i.body {
                    IfBody::Statement(b) => {
                        let mut out = self.body(b.statement, flags.clone());
                        for e in &b.else_if_clauses {
                            out = union(out, self.body(e.statement, flags.clone()));
                        }
                        if let Some(e) = &b.else_clause {
                            out = union(out, self.body(e.statement, flags.clone()));
                        }
                        (out, b.else_clause.is_some())
                    }
                    IfBody::ColonDelimited(b) => {
                        let mut out = self.statements(b.statements.as_slice(), flags.clone());
                        for e in &b.else_if_clauses {
                            out =
                                union(out, self.statements(e.statements.as_slice(), flags.clone()));
                        }
                        if let Some(e) = &b.else_clause {
                            out =
                                union(out, self.statements(e.statements.as_slice(), flags.clone()));
                        }
                        (out, b.else_clause.is_some())
                    }
                };
                if !has_else {
                    out = union(out, Some(flags));
                }
                out
            }
            Statement::Foreach(f) => {
                let (end, exits) = self.loop_body(|w| match &f.body {
                    ForeachBody::Statement(s) => w.body(s, flags.clone()),
                    ForeachBody::ColonDelimited(b) => {
                        w.statements(b.statements.as_slice(), flags.clone())
                    }
                });
                let mut out = union(union(Some(flags), end), exits).unwrap_or_default();
                let value = match &f.target {
                    ForeachTarget::Value(t) => t.value,
                    ForeachTarget::KeyValue(t) => t.value,
                };
                if let Expression::UnaryPrefix(u) = unparen(value)
                    && u.operator.is_reference()
                    && let Expression::Variable(Variable::Direct(v)) = unparen(u.operand)
                {
                    out.insert(v.name.to_vec());
                }
                Some(out)
            }
            Statement::While(w) => {
                let (end, exits) = self.loop_body(|f| match &w.body {
                    WhileBody::Statement(s) => f.body(s, flags.clone()),
                    WhileBody::ColonDelimited(b) => {
                        f.statements(b.statements.as_slice(), flags.clone())
                    }
                });
                union(union(Some(flags), end), exits)
            }
            Statement::For(l) => {
                let (end, exits) = self.loop_body(|f| match &l.body {
                    ForBody::Statement(s) => f.body(s, flags.clone()),
                    ForBody::ColonDelimited(b) => {
                        f.statements(b.statements.as_slice(), flags.clone())
                    }
                });
                union(union(Some(flags), end), exits)
            }
            Statement::DoWhile(d) => {
                let (end, exits) = self.loop_body(|f| f.body(d.statement, flags.clone()));
                union(end, exits)
            }
            Statement::Switch(sw) => {
                let cases = match &sw.body {
                    SwitchBody::BraceDelimited(b) => b.cases.as_slice(),
                    SwitchBody::ColonDelimited(b) => b.cases.as_slice(),
                };
                let has_default = cases.iter().any(|c| matches!(c, SwitchCase::Default(_)));
                let (end, exits) = self.loop_body(|f| {
                    let mut fall: Option<Set> = None;
                    for case in cases {
                        let stmts = match case {
                            SwitchCase::Expression(c) => c.statements.as_slice(),
                            SwitchCase::Default(d) => d.statements.as_slice(),
                        };
                        let entry = union(Some(flags.clone()), fall.take()).unwrap_or_default();
                        fall = f.statements(stmts, entry);
                    }
                    fall
                });
                let out = union(end, exits);
                if has_default {
                    out
                } else {
                    union(out, Some(flags))
                }
            }
            Statement::Try(t) => {
                let tried = self.statements(t.block.statements.as_slice(), flags.clone());
                let catch_entry = union(Some(flags.clone()), tried.clone()).unwrap_or_default();
                let mut out = tried;
                for c in &t.catch_clauses {
                    out = union(
                        out,
                        self.statements(c.block.statements.as_slice(), catch_entry.clone()),
                    );
                }
                match &t.finally_clause {
                    Some(f) => {
                        let entry = union(Some(catch_entry), out.clone()).unwrap_or_default();
                        let finally = self.statements(f.block.statements.as_slice(), entry);
                        out.and(finally)
                    }
                    None => out,
                }
            }
            Statement::Unset(u) => {
                let mut flags = flags;
                for value in &u.values {
                    if let Expression::Variable(Variable::Direct(v)) = unparen(value) {
                        flags.remove(v.name);
                    }
                }
                Some(flags)
            }
            _ => Some(flags),
        }
    }
}
