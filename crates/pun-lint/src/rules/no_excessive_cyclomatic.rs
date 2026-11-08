//! `lint/complexity/no-excessive-cyclomatic`, ported from Mago 1.57.0's `cyclomatic-complexity`
//! (mago-linter src/rule/maintainability/cyclomatic_complexity.rs).
//!
//! A class-like's complexity is the sum, over its methods that are not plain setters or getters, of each method's
//! complexity less one; a function's or closure's counts its body. Every `if`, `elseif`, loop, `catch`, ternary,
//! `default:` and logical or `??` operator adds one, `<=>` adds two. Options: `threshold` (default 15) and
//! `method-threshold` (default none), which also checks each method on its own.

use mago_span::HasSpan;
use mago_syntax::cst::*;
use mago_syntax::walker::Walker;

use super::mago_helpers::{class_like_header_span, configure, is_method_setter_or_getter, uint};
use crate::syntax::span;
use crate::{Context, Report, Rule};

pub struct NoExcessiveCyclomatic {
    pub threshold: usize,
    pub method_threshold: Option<usize>,
}

impl Default for NoExcessiveCyclomatic {
    fn default() -> Self {
        NoExcessiveCyclomatic {
            threshold: 15,
            method_threshold: None,
        }
    }
}

impl Rule for NoExcessiveCyclomatic {
    fn id(&self) -> &'static str {
        "lint/complexity/no-excessive-cyclomatic"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::Class,
            NodeKind::Trait,
            NodeKind::AnonymousClass,
            NodeKind::Enum,
            NodeKind::Interface,
            NodeKind::Function,
            NodeKind::Closure,
        ])
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        configure(options, |key, value| {
            match key {
                "threshold" => self.threshold = uint(value, u64::MAX)?,
                "method-threshold" => self.method_threshold = Some(uint(value, u64::MAX)?),
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
        let header = class_like_header_span(node);
        match node {
            Node::Class(n) => self.check_class_like("Class", n.members.as_slice(), header, out),
            Node::Trait(n) => self.check_class_like("Trait", n.members.as_slice(), header, out),
            Node::AnonymousClass(n) => {
                self.check_class_like("Class", n.members.as_slice(), header, out);
            }
            Node::Enum(n) => self.check_class_like("Enum", n.members.as_slice(), header, out),
            Node::Interface(n) => {
                self.check_class_like("Interface", n.members.as_slice(), header, out);
            }
            Node::Function(n) => self.check_function_like("Function", &n.body, header, out),
            Node::Closure(n) => self.check_function_like("Closure", &n.body, header, out),
            _ => {}
        }
    }
}

impl NoExcessiveCyclomatic {
    fn check_class_like(
        &self,
        kind: &str,
        members: &[ClassLikeMember<'_>],
        header: pun_problem::Span,
        out: &mut Vec<Report>,
    ) {
        let methods = members.iter().filter_map(|m| match m {
            ClassLikeMember::Method(method) => Some(method),
            _ => None,
        });
        let complexity: usize = methods
            .clone()
            .filter_map(method_complexity)
            .map(|c| c - 1)
            .sum();
        if complexity > self.threshold {
            out.push(report(format!("{kind} has high complexity."), header));
        }
        let Some(method_threshold) = self.method_threshold else {
            return;
        };
        for method in methods {
            if method_complexity(method).is_some_and(|c| c > method_threshold) {
                out.push(report(
                    format!(
                        "Method `{}` has high complexity.",
                        String::from_utf8_lossy(method.name.value)
                    ),
                    span(method.name.span()),
                ));
            }
        }
    }

    fn check_function_like(
        &self,
        kind: &str,
        body: &Block<'_>,
        header: pun_problem::Span,
        out: &mut Vec<Report>,
    ) {
        let mut complexity = 0;
        ComplexityCounter.walk_block(body, &mut complexity);
        if complexity > self.threshold {
            out.push(report(format!("{kind} has high complexity."), header));
        }
    }
}

fn report(message: String, at: pun_problem::Span) -> Report {
    Report {
        span: at,
        message,
        help: None,
        fix: None,
    }
}

/// `get_cyclomatic_complexity_of_method`: none for a setter or getter.
fn method_complexity(method: &Method<'_>) -> Option<usize> {
    if is_method_setter_or_getter(method) {
        return None;
    }
    let mut complexity = 1;
    if !method.is_abstract() {
        ComplexityCounter.walk_method(method, &mut complexity);
    }
    Some(complexity)
}

struct ComplexityCounter;

impl<'ast, 'arena> Walker<'ast, 'arena, usize> for ComplexityCounter {
    fn walk_in_if(&self, _: &'ast If<'arena>, count: &mut usize) {
        *count += 1;
    }

    fn walk_in_if_statement_body_else_if_clause(
        &self,
        _: &'ast IfStatementBodyElseIfClause<'arena>,
        count: &mut usize,
    ) {
        *count += 1;
    }

    fn walk_in_if_colon_delimited_body_else_if_clause(
        &self,
        _: &'ast IfColonDelimitedBodyElseIfClause<'arena>,
        count: &mut usize,
    ) {
        *count += 1;
    }

    fn walk_in_for(&self, _: &'ast For<'arena>, count: &mut usize) {
        *count += 1;
    }

    fn walk_in_foreach(&self, _: &'ast Foreach<'arena>, count: &mut usize) {
        *count += 1;
    }

    fn walk_in_while(&self, _: &'ast While<'arena>, count: &mut usize) {
        *count += 1;
    }

    fn walk_in_do_while(&self, _: &'ast DoWhile<'arena>, count: &mut usize) {
        *count += 1;
    }

    fn walk_in_try_catch_clause(&self, _: &'ast TryCatchClause<'arena>, count: &mut usize) {
        *count += 1;
    }

    fn walk_in_conditional(&self, _: &'ast Conditional<'arena>, count: &mut usize) {
        *count += 1;
    }

    fn walk_in_switch_default_case(&self, _: &'ast SwitchDefaultCase<'arena>, count: &mut usize) {
        *count += 1;
    }

    fn walk_in_binary(&self, binary: &'ast Binary<'arena>, count: &mut usize) {
        match binary.operator {
            operator if operator.is_logical() || operator.is_null_coalesce() => *count += 1,
            BinaryOperator::Spaceship(_) => *count += 2,
            _ => {}
        }
    }
}
