//! UnreachableStatementRule: NodeScopeResolver's UnreachableStatementNode, the first statement after one that always
//! terminates (getNextUnreachableStatements). What follows it is never processed, so nothing inside reports again.

use std::collections::HashSet;

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use super::super::Checker;
use super::finally;

#[derive(Default)]
pub(in super::super) struct Statements {
    pub(in super::super) reported: HashSet<Span>,
    pub(in super::super) skipped: HashSet<Span>,
    /// The original statement of each UnreachableStatementNode, built at every level.
    pub(in super::super) nodes: HashSet<Span>,
}

/// UnreachableStatementNode positions and statements the source walker never processes.
pub(in super::super) fn find(c: &Checker, program: &Program) -> Statements {
    let mut finder = Finder {
        c,
        out: Statements::default(),
    };
    finder.walk_program(program, &mut ());
    finder.out
}

struct Finder<'c, 'a, 'arena> {
    c: &'c Checker<'a, 'arena>,
    out: Statements,
}

impl Finder<'_, '_, '_> {
    fn list(&mut self, statements: &[Statement], early_binding: bool) {
        let mut terminated = false;
        for (i, s) in statements.iter().enumerate() {
            // ponytail: a label always ends the dead part, as if a goto reached it
            if matches!(s, Statement::Label(_)) {
                terminated = false;
                continue;
            }
            if terminated {
                // StatementsHandler::processStatementStep still analyses declarations after termination.
                if !matches!(
                    s,
                    Statement::Function(_)
                        | Statement::Class(_)
                        | Statement::Interface(_)
                        | Statement::Trait(_)
                        | Statement::Enum(_)
                ) {
                    self.out.skipped.insert(s.span());
                }
                continue;
            }
            if !finally::always_terminates(self.c, s) {
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
                self.out.nodes.insert(next.span());
                if self.c.level >= 4 {
                    self.out.reported.insert(next.span());
                }
            }
        }
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Finder<'_, '_, '_> {
    fn walk_statement(&mut self, s: &'ast Statement<'arena>, context: &mut ()) {
        if !self.out.skipped.contains(&s.span()) {
            mago_syntax::walker::walk_statement_mut(self, s, context);
        }
    }

    fn walk_in_program(&mut self, p: &'ast Program<'arena>, (): &mut ()) {
        self.list(p.statements.as_slice(), true);
    }
    fn walk_in_namespace(&mut self, n: &'ast Namespace<'arena>, (): &mut ()) {
        self.list(n.statements().as_slice(), true);
    }
    fn walk_in_block(&mut self, b: &'ast Block<'arena>, (): &mut ()) {
        self.list(b.statements.as_slice(), false);
    }
    fn walk_in_if_colon_delimited_body(
        &mut self,
        b: &'ast IfColonDelimitedBody<'arena>,
        (): &mut (),
    ) {
        self.list(b.statements.as_slice(), false);
    }
    fn walk_in_if_colon_delimited_body_else_if_clause(
        &mut self,
        b: &'ast IfColonDelimitedBodyElseIfClause<'arena>,
        (): &mut (),
    ) {
        self.list(b.statements.as_slice(), false);
    }
    fn walk_in_if_colon_delimited_body_else_clause(
        &mut self,
        b: &'ast IfColonDelimitedBodyElseClause<'arena>,
        (): &mut (),
    ) {
        self.list(b.statements.as_slice(), false);
    }
    fn walk_in_while_colon_delimited_body(
        &mut self,
        b: &'ast WhileColonDelimitedBody<'arena>,
        (): &mut (),
    ) {
        self.list(b.statements.as_slice(), false);
    }
    fn walk_in_for_colon_delimited_body(
        &mut self,
        b: &'ast ForColonDelimitedBody<'arena>,
        (): &mut (),
    ) {
        self.list(b.statements.as_slice(), false);
    }
    fn walk_in_foreach_colon_delimited_body(
        &mut self,
        b: &'ast ForeachColonDelimitedBody<'arena>,
        (): &mut (),
    ) {
        self.list(b.statements.as_slice(), false);
    }
    fn walk_in_declare_colon_delimited_body(
        &mut self,
        b: &'ast DeclareColonDelimitedBody<'arena>,
        (): &mut (),
    ) {
        self.list(b.statements.as_slice(), false);
    }
    fn walk_in_switch_case(&mut self, b: &'ast SwitchCase<'arena>, (): &mut ()) {
        self.list(b.statements(), false);
    }
}
