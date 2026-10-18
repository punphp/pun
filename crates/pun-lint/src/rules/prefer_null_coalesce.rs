//! `lint/complexity/prefer-null-coalesce`, ported from Rector 2.6.1's
//! `ArrayKeyExistsTernaryThenValueToCoalescingRector` (rules/CodeQuality/Rector/Ternary).
//!
//! `array_key_exists($key, $values) ? $values[$key] : null` becomes `$values[$key] ?? null`. The two differ only when
//! the key holds `null`, and then both give `null`.

use mago_syntax::cst::{Expression, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::rector_code_quality::{args, call_to, equal, is_constant, text};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferNullCoalesce;

impl Rule for PreferNullCoalesce {
    fn id(&self) -> &'static str {
        "lint/complexity/prefer-null-coalesce"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Conditional])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Conditional(t) = node else { return };
        let Some(call) = call_to(t.condition, "array_key_exists") else {
            return;
        };
        let Some(Expression::ArrayAccess(fetch)) = t.then.map(unparenthesize) else {
            return;
        };
        // areArrayKeysExistsArgsMatchingDimFetch: the array is the second argument and the key the first.
        let args = args(call);
        if args.len() < 2
            || !equal(ctx, fetch.array, args[1].value())
            || !equal(ctx, fetch.index, args[0].value())
        {
            return;
        }
        if !is_constant(t.r#else, "null") {
            return;
        }
        let code = format!(
            "{} ?? {}",
            text(ctx, fetch),
            text(ctx, unparenthesize(t.r#else))
        );
        out.push(Report {
            span: node_span(t),
            message: "Use `??` instead of an `array_key_exists()` ternary.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(node_span(t), code)],
            }),
        });
    }
}
