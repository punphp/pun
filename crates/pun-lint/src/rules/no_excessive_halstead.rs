//! `lint/complexity/no-excessive-halstead`, ported from Mago 1.57.0's `halstead`
//! (mago-linter src/rule/maintainability/halstead.rs).
//!
//! Computes the Halstead volume, difficulty and effort of each function, method, closure, arrow function and
//! property hook body, and reports the ones above `volume-threshold` (default 1000), `difficulty-threshold` (12) or
//! `effort-threshold` (5000). Operators are binary operations, assignments, `if`, `elseif`, `for`, `switch`,
//! `catch`, `return`, `while`, `do`-`while` and casts, distinct by node kind; operands are variables and string,
//! integer and float literals, distinct by their text. Nested classes, functions and methods are skipped.

use std::collections::HashSet;

use mago_syntax::cst::{Node, NodeKind};

use super::mago_helpers::configure;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoExcessiveHalstead {
    pub volume_threshold: f64,
    pub difficulty_threshold: f64,
    pub effort_threshold: f64,
}

impl Default for NoExcessiveHalstead {
    fn default() -> Self {
        NoExcessiveHalstead {
            volume_threshold: 1000.0,
            difficulty_threshold: 12.0,
            effort_threshold: 5000.0,
        }
    }
}

impl Rule for NoExcessiveHalstead {
    fn id(&self) -> &'static str {
        "lint/complexity/no-excessive-halstead"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::PropertyHookConcreteBody,
            NodeKind::Method,
            NodeKind::Function,
            NodeKind::Closure,
            NodeKind::ArrowFunction,
        ])
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        configure(options, |key, value| {
            let value = value.as_f64()?;
            match key {
                "volume-threshold" => self.volume_threshold = value,
                "difficulty-threshold" => self.difficulty_threshold = value,
                "effort-threshold" => self.effort_threshold = value,
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
        let kind = match node {
            Node::PropertyHookConcreteBody(_) => "Hook",
            Node::Method(_) => "Method",
            Node::Function(_) => "Function",
            Node::Closure(_) => "Closure",
            _ => "Arrow function",
        };
        let (volume, difficulty, effort) = halstead(node);
        let mut metrics = Vec::new();
        if volume > self.volume_threshold {
            metrics.push("volume");
        }
        if difficulty > self.difficulty_threshold {
            metrics.push("difficulty");
        }
        if effort > self.effort_threshold {
            metrics.push("effort");
        }
        let label = match metrics.as_slice() {
            [] => return,
            [only] => (*only).to_string(),
            [first, second] => format!("{first} and {second}"),
            [rest @ .., last] => format!("{}, and {last}", rest.join(", ")),
        };
        out.push(Report {
            span: node_span(&node),
            message: format!("{kind} has a high halstead {label}"),
            help: Some("Consider breaking this into smaller, more focused functions.".into()),
            fix: None,
        });
    }
}

#[derive(Default)]
struct Counts<'arena> {
    operators: HashSet<NodeKind>,
    operands: HashSet<&'arena [u8]>,
    total_operators: usize,
    total_operands: usize,
}

/// `gather_and_compute_halstead`: (volume, difficulty, effort), each rounded to two decimals.
fn halstead<'arena>(node: Node<'_, 'arena>) -> (f64, f64, f64) {
    let mut counts = Counts::default();
    node.visit_children(|child| recurse(child, &mut counts));
    let (n1, n2) = (counts.operators.len() as f64, counts.operands.len() as f64);
    let (total_n1, total_n2) = (counts.total_operators as f64, counts.total_operands as f64);
    if counts.operands.is_empty() {
        return (0.0, 0.0, 0.0);
    }
    // n1 + n2 > 0, as there is an operand.
    let volume = (total_n1 + total_n2) * (n1 + n2).log2();
    let difficulty = (n1 / 2.0) * (total_n2 / n2.max(1.0));
    let effort = volume * difficulty;
    (round2(volume), round2(difficulty), round2(effort))
}

fn recurse<'arena>(node: Node<'_, 'arena>, counts: &mut Counts<'arena>) {
    if node.is_declaration() {
        return;
    }
    node.visit_children(|child| recurse(child, counts));
    match node {
        Node::Binary(_)
        | Node::Assignment(_)
        | Node::If(_)
        | Node::IfStatementBodyElseIfClause(_)
        | Node::IfColonDelimitedBodyElseIfClause(_)
        | Node::For(_)
        | Node::Switch(_)
        | Node::TryCatchClause(_)
        | Node::Return(_)
        | Node::While(_)
        | Node::DoWhile(_) => {
            counts.total_operators += 1;
            counts.operators.insert(node.kind());
        }
        Node::UnaryPrefix(unary) if unary.operator.is_cast() => {
            counts.total_operators += 1;
            counts.operators.insert(node.kind());
        }
        Node::DirectVariable(variable) => {
            counts.total_operands += 1;
            counts.operands.insert(variable.name);
        }
        Node::LiteralString(literal) => {
            counts.total_operands += 1;
            counts.operands.insert(literal.raw);
        }
        Node::LiteralInteger(literal) => {
            counts.total_operands += 1;
            counts.operands.insert(literal.raw);
        }
        Node::LiteralFloat(literal) => {
            counts.total_operands += 1;
            counts.operands.insert(literal.raw);
        }
        _ => {}
    }
}

fn round2(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}
