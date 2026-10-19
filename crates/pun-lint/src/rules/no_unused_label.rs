//! `lint/complexity/no-unused-label`, ported from Mago 1.57.0's `no-redundant-label`
//! (mago-linter src/rule/redundancy/no_redundant_label.rs).
//!
//! Reports a `goto` label that no `goto` in the file jumps to.

use mago_span::HasSpan;
use mago_syntax::cst::{Goto, Label, Node, NodeKind};
use mago_syntax::walker::Walker;

use crate::syntax::span;
use crate::{Context, Report, Rule};

pub struct NoUnusedLabel;

impl Rule for NoUnusedLabel {
    fn id(&self) -> &'static str {
        "lint/complexity/no-unused-label"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Program])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Program(program) = node else { return };
        let mut collected = LabelsAndGotos::default();
        LabelCollector.walk_program(program, &mut collected);
        for (name, label_span) in &collected.labels {
            if collected.gotos.contains(name) {
                continue;
            }
            out.push(Report {
                span: *label_span,
                message: format!("Redundant goto label `{}`.", String::from_utf8_lossy(name)),
                help: Some("Remove the redundant label.".to_string()),
                fix: None,
            });
        }
    }
}

#[derive(Default)]
struct LabelsAndGotos<'arena> {
    labels: Vec<(&'arena [u8], pun_problem::Span)>,
    gotos: Vec<&'arena [u8]>,
}

struct LabelCollector;

impl<'ast, 'arena> Walker<'ast, 'arena, LabelsAndGotos<'arena>> for LabelCollector {
    fn walk_in_label(&self, label: &'ast Label<'arena>, collected: &mut LabelsAndGotos<'arena>) {
        collected
            .labels
            .push((label.name.value, span(label.span())));
    }

    fn walk_in_goto(&self, goto: &'ast Goto<'arena>, collected: &mut LabelsAndGotos<'arena>) {
        collected.gotos.push(goto.label.value);
    }
}
