//! `lint/complexity/no-excessive-nesting`, ported from Mago 1.57.0's `excessive-nesting`
//! (mago-linter src/rule/maintainability/excessive_nesting.rs).
//!
//! Counts block nesting from the file root and reports the first block deeper than `threshold` (default 7) on each
//! path, without looking inside it. With `function-like-threshold` set, each function, method, closure and property
//! hook body is also counted on its own, from zero for the body's statements, against that threshold: a body is
//! walked once for each count, as Mago does, so one block can be reported twice.

use mago_span::HasSpan;
use mago_syntax::cst::*;
use mago_syntax::walker::{MutWalker, walk_block_mut};

use super::mago_helpers::{configure, uint};
use crate::syntax::span;
use crate::{Context, Report, Rule};

pub struct NoExcessiveNesting {
    pub threshold: usize,
    pub function_like_threshold: Option<usize>,
}

impl Default for NoExcessiveNesting {
    fn default() -> Self {
        NoExcessiveNesting {
            threshold: 7,
            function_like_threshold: None,
        }
    }
}

impl Rule for NoExcessiveNesting {
    fn id(&self) -> &'static str {
        "lint/complexity/no-excessive-nesting"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Program])
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        configure(options, |key, value| {
            match key {
                "threshold" => self.threshold = uint(value, u64::MAX)?,
                "function-like-threshold" => {
                    self.function_like_threshold = Some(uint(value, u64::MAX)?);
                }
                _ => return None,
            }
            Some(())
        })
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Program(program) = node else { return };
        let mut walker = NestingWalker {
            threshold: self.threshold,
            function_like_threshold: self.function_like_threshold,
            level: 0,
        };
        walker.walk_program(program, out);
    }
}

struct NestingWalker {
    threshold: usize,
    function_like_threshold: Option<usize>,
    level: usize,
}

impl NestingWalker {
    fn enter_function_like_body(&mut self, body: &Block<'_>, out: &mut Vec<Report>) {
        let Some(threshold) = self.function_like_threshold else {
            return;
        };
        let saved = (self.threshold, self.level);
        (self.threshold, self.level) = (threshold, 0);
        walk_block_mut(self, body, out);
        (self.threshold, self.level) = saved;
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, Vec<Report>> for NestingWalker {
    fn walk_block(&mut self, block: &'ast Block<'arena>, out: &mut Vec<Report>) {
        self.level += 1;
        if self.level > self.threshold {
            out.push(Report {
                span: span(block.span()),
                message: "Excessive block nesting.".into(),
                help: Some(
                    "Refactor your code to reduce nesting (e.g. use early returns, guard clauses, or helper functions)."
                        .into(),
                ),
                fix: None,
            });
        } else {
            walk_block_mut(self, block, out);
        }
        self.level -= 1;
    }

    fn walk_in_function(&mut self, function: &'ast Function<'arena>, out: &mut Vec<Report>) {
        self.enter_function_like_body(&function.body, out);
    }

    fn walk_in_closure(&mut self, closure: &'ast Closure<'arena>, out: &mut Vec<Report>) {
        self.enter_function_like_body(&closure.body, out);
    }

    fn walk_in_method(&mut self, method: &'ast Method<'arena>, out: &mut Vec<Report>) {
        if let MethodBody::Concrete(body) = &method.body {
            self.enter_function_like_body(body, out);
        }
    }

    fn walk_in_property_hook_concrete_body(
        &mut self,
        body: &'ast PropertyHookConcreteBody<'arena>,
        out: &mut Vec<Report>,
    ) {
        if let PropertyHookConcreteBody::Block(block) = body {
            self.enter_function_like_body(block, out);
        }
    }
}
