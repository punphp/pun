//! `lint/php-8.4/prefer-array-find`, ported from Rector 2.6.1's `ForeachToArrayFindRector` (rules/Php84/Rector/Foreach_); the shared code and what Pun does
//! differently are in `lint_b_foreach`.

use mago_syntax::cst::{Node, NodeKind, Statement};
use pun_problem::{Fix, Safety};

use super::lint_b_foreach::{Kind, refactor};
use super::rector_stmts::{HOLDERS, holder_list};
use crate::{Context, Report, Rule};

pub struct PreferArrayFind;

impl Rule for PreferArrayFind {
    fn id(&self) -> &'static str {
        "lint/php-8.4/prefer-array-find"
    }

    fn min_php_version(&self) -> u32 {
        // PhpVersionFeature::ARRAY_FIND
        80400
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(HOLDERS)
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        if !crate::syntax::statements(node)
            .iter()
            .any(|s| matches!(s, Statement::Foreach(_)))
        {
            return;
        }
        let Some(list) = holder_list(ctx.source, node, ancestors) else {
            return;
        };
        let Some((edits, span)) = refactor(ctx, ancestors, &list, Kind::Find) else {
            return;
        };
        out.push(Report {
            span,
            message: "Use `array_find()` instead of a `foreach` that looks for an item."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}
