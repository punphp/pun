//! `lint/complexity/no-excessive-parameters`, ported from Mago 1.57.0's `excessive-parameter-list`
//! (mago-linter src/rule/maintainability/excessive_parameter_list.rs).
//!
//! Reports a parameter list longer than `threshold` (default 5); a constructor's is compared with
//! `constructor-threshold` when that is set.

use mago_syntax::cst::{Node, NodeKind};

use super::mago_helpers::{configure, function_like_scope, uint};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoExcessiveParameters {
    pub threshold: usize,
    pub constructor_threshold: Option<usize>,
}

impl Default for NoExcessiveParameters {
    fn default() -> Self {
        NoExcessiveParameters {
            threshold: 5,
            constructor_threshold: None,
        }
    }
}

impl Rule for NoExcessiveParameters {
    fn id(&self) -> &'static str {
        "lint/complexity/no-excessive-parameters"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionLikeParameterList])
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        configure(options, |key, value| {
            match key {
                "threshold" => self.threshold = uint(value, u8::MAX.into())?,
                "constructor-threshold" => {
                    self.constructor_threshold = Some(uint(value, u8::MAX.into())?);
                }
                _ => return None,
            }
            Some(())
        })
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionLikeParameterList(list) = node else {
            return;
        };
        let is_constructor = matches!(
            function_like_scope(ancestors),
            Some(Node::Method(m)) if m.name.value.eq_ignore_ascii_case(b"__construct")
        );
        let threshold = match is_constructor {
            true => self.constructor_threshold.unwrap_or(self.threshold),
            false => self.threshold,
        };
        if list.parameters.len() > threshold {
            out.push(Report {
                span: node_span(list),
                message: "Parameter list is too long.".into(),
                help: Some(
                    "Try reducing the number of parameters, or consider passing an object or a shape instead.".into(),
                ),
                fix: None,
            });
        }
    }
}
