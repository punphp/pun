//! UnreachableStatementRule: NodeScopeResolver's UnreachableStatementNode, the first statement after one that always
//! terminates (getNextUnreachableStatements). What follows it is never processed, so nothing inside reports again.

use std::collections::HashSet;

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use super::super::Checker;
use super::finally;

/// The statements UnreachableStatementRule reports in `program`, by span.
pub(in super::super) fn find(c: &Checker, program: &Program) -> HashSet<Span> {
    let mut finder = Finder {
        c,
        dead: Vec::new(),
        out: HashSet::new(),
    };
    if c.level >= 4 {
        finder.walk_program(program, &mut ());
    }
    finder.out
}

struct Finder<'c, 'a, 'arena> {
    c: &'c Checker<'a, 'arena>,
    /// The statements never processed: from an unreachable one to the end of its list.
    dead: Vec<(u32, u32)>,
    out: HashSet<Span>,
}

impl Finder<'_, '_, '_> {
    fn list(&mut self, span: Span, statements: &[Statement], early_binding: bool) {
        if self
            .dead
            .iter()
            .any(|(a, b)| *a <= span.start.offset && span.end.offset <= *b)
        {
            return;
        }
        let mut terminated = false;
        for (i, s) in statements.iter().enumerate() {
            // ponytail: a label always ends the dead part, as if a goto reached it
            if matches!(s, Statement::Label(_)) {
                terminated = false;
                continue;
            }
            if terminated || !finally::always_terminates(self.c, s) {
                continue;
            }
            terminated = true;
            let rest = &statements[i + 1..];
            let next = rest
                .iter()
                .take_while(|s| !matches!(s, Statement::Label(_)))
                .find(|s| {
                    !matches!(
                        s,
                        Statement::Noop(_)
                            | Statement::Inline(_)
                            | Statement::OpeningTag(_)
                            | Statement::ClosingTag(_)
                    ) && !(early_binding
                        && matches!(
                            s,
                            Statement::Function(_)
                                | Statement::Class(_)
                                | Statement::Interface(_)
                                | Statement::Trait(_)
                                | Statement::Enum(_)
                                | Statement::HaltCompiler(_)
                        ))
                });
            if let Some(next) = next {
                self.out.insert(next.span());
                let end = rest
                    .iter()
                    .take_while(|s| !matches!(s, Statement::Label(_)))
                    .last()
                    .map_or(next.span().end.offset, |s| s.span().end.offset);
                self.dead.push((next.span().start.offset, end));
            }
        }
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Finder<'_, '_, '_> {
    fn walk_in_program(&mut self, p: &'ast Program<'arena>, _: &mut ()) {
        self.list(p.span(), p.statements.as_slice(), true);
    }
    fn walk_in_namespace(&mut self, n: &'ast Namespace<'arena>, _: &mut ()) {
        self.list(n.span(), n.statements().as_slice(), true);
    }
    fn walk_in_block(&mut self, b: &'ast Block<'arena>, _: &mut ()) {
        self.list(b.span(), b.statements.as_slice(), false);
    }
    fn walk_in_if_colon_delimited_body(
        &mut self,
        b: &'ast IfColonDelimitedBody<'arena>,
        _: &mut (),
    ) {
        self.list(b.span(), b.statements.as_slice(), false);
    }
    fn walk_in_if_colon_delimited_body_else_if_clause(
        &mut self,
        b: &'ast IfColonDelimitedBodyElseIfClause<'arena>,
        _: &mut (),
    ) {
        self.list(b.span(), b.statements.as_slice(), false);
    }
    fn walk_in_if_colon_delimited_body_else_clause(
        &mut self,
        b: &'ast IfColonDelimitedBodyElseClause<'arena>,
        _: &mut (),
    ) {
        self.list(b.span(), b.statements.as_slice(), false);
    }
    fn walk_in_while_colon_delimited_body(
        &mut self,
        b: &'ast WhileColonDelimitedBody<'arena>,
        _: &mut (),
    ) {
        self.list(b.span(), b.statements.as_slice(), false);
    }
    fn walk_in_for_colon_delimited_body(
        &mut self,
        b: &'ast ForColonDelimitedBody<'arena>,
        _: &mut (),
    ) {
        self.list(b.span(), b.statements.as_slice(), false);
    }
    fn walk_in_foreach_colon_delimited_body(
        &mut self,
        b: &'ast ForeachColonDelimitedBody<'arena>,
        _: &mut (),
    ) {
        self.list(b.span(), b.statements.as_slice(), false);
    }
    fn walk_in_declare_colon_delimited_body(
        &mut self,
        b: &'ast DeclareColonDelimitedBody<'arena>,
        _: &mut (),
    ) {
        self.list(b.span(), b.statements.as_slice(), false);
    }
    fn walk_in_switch_case(&mut self, b: &'ast SwitchCase<'arena>, _: &mut ()) {
        self.list(b.span(), b.statements(), false);
    }
}
