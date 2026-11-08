//! `lint/complexity/prefer-func-num-args`, ported from Rector 2.8.4's `SimplifyFuncGetArgsCountRector`
//! (rules/CodeQuality/Rector/FuncCall).
//!
//! `count(func_get_args())` becomes `func_num_args()`.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::rector_code_quality::{args, call_to, named};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferFuncNumArgs;

impl Rule for PreferFuncNumArgs {
    fn id(&self) -> &'static str {
        "lint/complexity/prefer-func-num-args"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(c) = node else { return };
        if !named(c, "count") {
            return;
        }
        // `$node->getArgs()[0]` on a `count()` without arguments is null, which is no `FuncCall`.
        let Some(first) = args(c).first().map(|a| a.value()) else {
            return;
        };
        if call_to(first, "func_get_args").is_none() {
            return;
        }
        let span = node_span(c);
        out.push(Report {
            span,
            message: "Use `func_num_args()` instead of counting `func_get_args()`.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, "func_num_args()")],
            }),
        });
    }
}
