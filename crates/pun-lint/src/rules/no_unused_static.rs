//! `lint/dead-code/no-unused-static`, ported from Mago 1.57.0's `no-unused-static`
//! (mago-linter src/rule/correctness/no_unused_static.rs).
//!
//! Reports a `static` variable of a function, method or closure that its body never reads or writes. Names that
//! start with `$_` are left alone, and so is a body with a variable variable or an `extract()` call.

use mago_syntax::cst::{Node, NodeKind};

use super::mago_usage::{declarations, function_like_body, is_silenced_name, used_names};
use crate::syntax::span;
use crate::{Context, Report, Rule};

pub struct NoUnusedStatic;

impl Rule for NoUnusedStatic {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-unused-static"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Function, NodeKind::Method, NodeKind::Closure])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(body) = function_like_body(node) else {
            return;
        };
        let declared = declarations(body, true);
        if declared.is_empty() {
            return;
        }
        let interest: Vec<&[u8]> = declared.iter().map(|(name, _)| *name).collect();
        let Some(used) = used_names(body, &interest) else {
            return;
        };
        for (name, declared_at) in declared {
            if is_silenced_name(name) || used.contains(&name) {
                continue;
            }
            let name = String::from_utf8_lossy(name);
            out.push(Report {
                span: span(declared_at),
                message: format!("Static variable `{name}` is declared but never used."),
                help: Some(
                    "Remove this `static` declaration, or reference the variable below."
                        .to_string(),
                ),
                fix: None,
            });
        }
    }
}
