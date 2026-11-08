//! `lint/laravel/prefer-resolve-helper`, ported from rector-laravel 2.7.3's `AppToResolveRector` (src/Rector/FuncCall).
//!
//! `app('foo')` becomes `resolve('foo')`. A call without the `abstract` argument, or with one that is null, is left
//! alone: `app()` returns the container.

use mago_syntax::cst::{Node, NodeKind};

use super::{get_arg, replace};
use crate::rules::lint_b_helpers::{Constant, constant, value};
use crate::rules::rector_code_quality::named;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferResolveHelper;

impl Rule for PreferResolveHelper {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-resolve-helper"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(f) = node else {
            return;
        };
        if !named(f, "app") {
            return;
        }
        let Some(arg) = get_arg(&f.argument_list, "abstract", 0) else {
            return;
        };
        if constant(ancestors, value(arg), node_span(f).start) == Some(Constant::Null) {
            return;
        }
        out.push(replace(
            node_span(f.function),
            "Use `resolve()` instead of `app()`.".to_string(),
            "resolve".to_string(),
        ));
    }
}
