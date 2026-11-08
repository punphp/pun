//! `lint/strings/no-single-argument-sprintf`, ported from Rector 2.8.4's `UnwrapSprintfOneArgumentRector`
//! (rules/CodeQuality/Rector/FuncCall).
//!
//! `sprintf('value')` becomes `'value'`. The fix is unsafe: `sprintf('100%%')` gives `100%`, and a format with a
//! placeholder but no value throws an `ArgumentCountError`.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::rector_code_quality::{args, moved, named};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoSingleArgumentSprintf;

impl Rule for NoSingleArgumentSprintf {
    fn id(&self) -> &'static str {
        "lint/strings/no-single-argument-sprintf"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(c) = node else { return };
        if !named(c, "sprintf") {
            return;
        }
        // One argument, not spread. Without arguments Rector reads a missing `getArgs()[0]` and changes nothing.
        let args = args(c);
        let [format] = args.as_slice() else { return };
        if format.is_unpacked() {
            return;
        }
        let span = node_span(c);
        let code = moved(ctx, span, ancestors, format.value());
        out.push(Report {
            span,
            message: "`sprintf()` with only a format does nothing; use the format itself."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits: vec![Edit::new(span, code)],
            }),
        });
    }
}
