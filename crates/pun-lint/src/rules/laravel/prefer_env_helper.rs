//! `lint/laravel/prefer-env-helper`, ported from rector-laravel 2.5.0's `EnvVariableToEnvHelperRector`
//! (src/Rector/ArrayDimFetch).
//!
//! Reading `$_ENV['NAME']` becomes `\Illuminate\Support\Env::get('NAME')`. An assigned item is left alone
//! (`AttributeKey::IS_BEING_ASSIGNED`), and so is any other place a call cannot be written to (Rector rewrites those
//! into code PHP rejects).

use mago_syntax::cst::{Node, NodeKind};

use super::{in_string, is_variable, is_written, replace};
use crate::rules::rector_code_quality::text;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferEnvHelper;

impl Rule for PreferEnvHelper {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-env-helper"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::ArrayAccess])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::ArrayAccess(access) = node else {
            return;
        };
        if !is_variable(access.array, "_ENV") {
            return;
        }
        let span = node_span(access);
        if is_written(span, ancestors) || in_string(ancestors) {
            return;
        }
        out.push(replace(
            span,
            "Use `Env::get()` instead of reading `$_ENV`.".to_string(),
            format!(
                "\\Illuminate\\Support\\Env::get({})",
                text(ctx, access.index)
            ),
        ));
    }
}
