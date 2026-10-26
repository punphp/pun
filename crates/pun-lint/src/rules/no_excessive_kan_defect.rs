//! `lint/complexity/no-excessive-kan-defect`, ported from Mago 1.57.0's `kan-defect`
//! (mago-linter src/rule/maintainability/kan_defect.rs).
//!
//! phpmetrics' Kan defect score, `0.15 + 0.23 * loops + 0.22 * (switch + match) + 0.07 * if`, of each class-like,
//! function and closure (nested code included); reported on the header when above `threshold` (default 1.6).

use mago_syntax::cst::*;
use mago_syntax::walker::Walker;

use super::mago_helpers::{class_like_header_span, configure};
use crate::{Context, Report, Rule};

pub struct NoExcessiveKanDefect {
    pub threshold: f64,
}

impl Default for NoExcessiveKanDefect {
    fn default() -> Self {
        NoExcessiveKanDefect { threshold: 1.6 }
    }
}

impl Rule for NoExcessiveKanDefect {
    fn id(&self) -> &'static str {
        "lint/complexity/no-excessive-kan-defect"
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
                "threshold" => self.threshold = value.as_f64()?,
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
        let mut factors = (0, 0, 0);
        let kind = match node {
            Node::Class(n) => {
                Collector.walk_class(n, &mut factors);
                "Class"
            }
            Node::Trait(n) => {
                Collector.walk_trait(n, &mut factors);
                "Trait"
            }
            Node::AnonymousClass(n) => {
                Collector.walk_anonymous_class(n, &mut factors);
                "Class"
            }
            Node::Enum(n) => {
                Collector.walk_enum(n, &mut factors);
                "Enum"
            }
            Node::Interface(n) => {
                Collector.walk_interface(n, &mut factors);
                "Interface"
            }
            Node::Function(n) => {
                Collector.walk_function(n, &mut factors);
                "Function"
            }
            Node::Closure(n) => {
                Collector.walk_closure(n, &mut factors);
                "Closure"
            }
            _ => return,
        };
        let (select, r#while, r#if) = factors;
        let score = 0.07f64.mul_add(
            r#if as f64,
            0.22f64.mul_add(select as f64, 0.23f64.mul_add(r#while as f64, 0.15)),
        );
        if score > self.threshold {
            out.push(Report {
                span: class_like_header_span(node),
                message: format!("{kind} has a high kan defect score."),
                help: Some(
                    "Try reducing the number of loops, switch statements, or if statements.".into(),
                ),
                fix: None,
            });
        }
    }
}

/// (switch and match, loops other than `for`, if).
struct Collector;

impl<'ast, 'arena> Walker<'ast, 'arena, (usize, usize, usize)> for Collector {
    fn walk_in_switch(&self, _: &'ast Switch<'arena>, counts: &mut (usize, usize, usize)) {
        counts.0 += 1;
    }

    fn walk_in_match(&self, _: &'ast Match<'arena>, counts: &mut (usize, usize, usize)) {
        counts.0 += 1;
    }

    fn walk_in_do_while(&self, _: &'ast DoWhile<'arena>, counts: &mut (usize, usize, usize)) {
        counts.1 += 1;
    }

    fn walk_in_while(&self, _: &'ast While<'arena>, counts: &mut (usize, usize, usize)) {
        counts.1 += 1;
    }

    fn walk_in_foreach(&self, _: &'ast Foreach<'arena>, counts: &mut (usize, usize, usize)) {
        counts.1 += 1;
    }

    fn walk_in_if(&self, _: &'ast If<'arena>, counts: &mut (usize, usize, usize)) {
        counts.2 += 1;
    }
}
