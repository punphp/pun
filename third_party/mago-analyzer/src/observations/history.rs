//! Index only explicitly requested expressions outside their asking call.

use std::rc::Rc;

use foldhash::HashMap;
use mago_allocator::Arena;
use mago_span::{HasSpan, Span};
use mago_syntax::{
    cst::{Call, ClassLikeMemberSelector, Expression, Program, Variable},
    walker::MutWalker,
};

use super::CallScopeRequest;

pub(super) fn contains(root: Span, span: Span) -> bool {
    root.file_id == span.file_id
        && root.start.offset <= span.start.offset
        && root.end.offset >= span.end.offset
}

#[derive(Debug)]
pub(super) struct History<'arena> {
    pub queries: HashMap<Span, Vec<(Span, Option<&'arena Expression<'arena>>)>>,
    pub expressions: Rc<HashMap<Span, Expression<'arena>>>,
}

impl<'arena> History<'arena> {
    pub fn new<A: Arena>(
        program: &'arena Program<'arena>,
        requests: &[CallScopeRequest],
        arena: &'arena A,
    ) -> Self {
        let mut index = Index {
            expressions: requests
                .iter()
                .flat_map(|r| {
                    r.expressions
                        .iter()
                        .filter(|span| !contains(r.call, **span))
                })
                .map(|span| (*span, None))
                .collect(),
            selectors: requests.iter().map(|r| (r.call, None)).collect(),
        };
        index.walk_program(program, &mut ());
        let selectors: HashMap<_, _> = index
            .selectors
            .into_iter()
            .map(|(call, selector)| (call, selector.map(|expression| &*arena.alloc(expression))))
            .collect();
        let mut queries: HashMap<_, Vec<_>> = HashMap::default();
        for request in requests {
            for span in &request.expressions {
                if !contains(request.call, *span) {
                    queries
                        .entry(*span)
                        .or_default()
                        .push((request.call, selectors[&request.call]));
                }
            }
        }
        Self {
            queries,
            expressions: Rc::new(
                index
                    .expressions
                    .into_iter()
                    .filter_map(|(span, e)| e.map(|e| (span, e)))
                    .collect(),
            ),
        }
    }
}

struct Index<'arena> {
    expressions: HashMap<Span, Option<Expression<'arena>>>,
    selectors: HashMap<Span, Option<Expression<'arena>>>,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Index<'arena> {
    fn walk_in_expression(&mut self, expression: &'ast Expression<'arena>, _: &mut ()) {
        let span = expression.span();
        if let Some(slot) = self.expressions.get_mut(&span) {
            *slot = Some(expression.clone());
        }
        if let Expression::Call(Call::Method(call)) = expression
            && let Some(selector) = self.selectors.get_mut(&span)
        {
            *selector = match &call.method {
                ClassLikeMemberSelector::Variable(v) => Some(Expression::Variable(v.clone())),
                ClassLikeMemberSelector::Expression(e) => Some(e.expression.clone()),
                _ => None,
            };
        }
    }

    fn walk_in_variable(&mut self, variable: &'ast Variable<'arena>, _: &mut ()) {
        if let Some(slot) = self.expressions.get_mut(&variable.span()) {
            *slot = Some(Expression::Variable(variable.clone()));
        }
    }
}
