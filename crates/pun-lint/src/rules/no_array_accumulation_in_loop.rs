//! `lint/style/no-array-accumulation-in-loop`, ported from Mago 1.57.0's linter rule `no-array-accumulation-in-loop`
//! (rule/best_practices/no_array_accumulation_in_loop.rs).
//!
//! In the body of a loop (not in a nested loop, function, closure, arrow function or method), an assignment whose
//! right side is a call to `array_merge`/`array_merge_recursive` or `array_unique`/`array_values`, or an array with
//! a spread element, copies the whole array on every pass. The call (or, for a spread, the assignment) is reported.

use mago_syntax::cst::{
    ArrowFunction, Assignment, Call, Closure, DoWhile, Expression, For, ForBody, Foreach,
    ForeachBody, Function, Method, Node, NodeKind, Statement, While, WhileBody,
};
use mago_syntax::walker::MutWalker;

use super::lint_b_names::function_call_matches_any;
use crate::{Context, Report, Rule};

pub struct NoArrayAccumulationInLoop;

const MERGE_FUNCTIONS: &[&str] = &["array_merge", "array_merge_recursive"];
const ACCUMULATOR_FUNCTIONS: &[&str] = &["array_unique", "array_values"];

impl Rule for NoArrayAccumulationInLoop {
    fn id(&self) -> &'static str {
        "lint/style/no-array-accumulation-in-loop"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::For,
            NodeKind::Foreach,
            NodeKind::While,
            NodeKind::DoWhile,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let statements: &[Statement<'arena>] = match node {
            Node::For(l) => match &l.body {
                ForBody::Statement(s) => std::slice::from_ref(*s),
                ForBody::ColonDelimited(b) => b.statements.as_slice(),
            },
            Node::Foreach(l) => match &l.body {
                ForeachBody::Statement(s) => std::slice::from_ref(*s),
                ForeachBody::ColonDelimited(b) => b.statements.as_slice(),
            },
            Node::While(l) => match &l.body {
                WhileBody::Statement(s) => std::slice::from_ref(*s),
                WhileBody::ColonDelimited(b) => b.statements.as_slice(),
            },
            _ => {
                let Node::DoWhile(l) = node else { return };
                std::slice::from_ref(l.statement)
            }
        };
        let mut collector = Collector { ancestors, out };
        for statement in statements {
            collector.walk_statement(statement, &mut ());
        }
    }
}

struct Collector<'a, 'ast, 'arena> {
    ancestors: &'a [Node<'ast, 'arena>],
    out: &'a mut Vec<Report>,
}

impl Collector<'_, '_, '_> {
    fn report(&mut self, span: mago_span::Span, message: String, help: String) {
        self.out.push(Report {
            span: crate::syntax::span(span),
            message,
            help: Some(help),
            fix: None,
        });
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Collector<'_, '_, 'arena> {
    fn walk_in_assignment(&mut self, assignment: &'ast Assignment<'arena>, (): &mut ()) {
        if let Expression::Call(Call::Function(call)) = assignment.rhs {
            if let Some(name) = function_call_matches_any(self.ancestors, call, MERGE_FUNCTIONS) {
                let span = mago_span::HasSpan::span(call);
                self.report(
                    span,
                    format!("`{name}()` called inside a loop causes O(n²) array copying."),
                    format!("Collect items into an array and call `{name}()` once after the loop."),
                );
                return;
            }
            if let Some(name) =
                function_call_matches_any(self.ancestors, call, ACCUMULATOR_FUNCTIONS)
            {
                let span = mago_span::HasSpan::span(call);
                self.report(
                    span,
                    format!(
                        "`{name}()` called inside a loop reprocesses the entire array every iteration."
                    ),
                    format!("Move `{name}()` outside the loop and call it once after accumulation."),
                );
                return;
            }
        }
        if let Expression::Array(array) = assignment.rhs
            && array
                .elements
                .iter()
                .any(mago_syntax::cst::ArrayElement::is_variadic)
        {
            let span = mago_span::HasSpan::span(assignment);
            self.report(
                span,
                "Array spread in reassignment inside a loop causes O(n²) array copying."
                    .to_string(),
                "Collect items with `$array[] = $item` and merge once after the loop.".to_string(),
            );
        }
    }

    fn walk_for(&mut self, _: &'ast For<'arena>, (): &mut ()) {}
    fn walk_foreach(&mut self, _: &'ast Foreach<'arena>, (): &mut ()) {}
    fn walk_while(&mut self, _: &'ast While<'arena>, (): &mut ()) {}
    fn walk_do_while(&mut self, _: &'ast DoWhile<'arena>, (): &mut ()) {}
    fn walk_function(&mut self, _: &'ast Function<'arena>, (): &mut ()) {}
    fn walk_closure(&mut self, _: &'ast Closure<'arena>, (): &mut ()) {}
    fn walk_arrow_function(&mut self, _: &'ast ArrowFunction<'arena>, (): &mut ()) {}
    fn walk_method(&mut self, _: &'ast Method<'arena>, (): &mut ()) {}
}
