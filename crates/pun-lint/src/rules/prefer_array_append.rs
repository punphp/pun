//! `lint/arrays/prefer-array-append`, ported from Rector 2.8.4's `ChangeArrayPushToArrayAssignRector`
//! (rules/CodeQuality/Rector/FuncCall).
//!
//! `array_push($items, $a, $b);` as a statement becomes `$items[] = $a;` and `$items[] = $b;`, one statement per
//! pushed value. The fix is unsafe: `array_push()` on a value that is not an array throws a `TypeError`, where
//! `$items[] =` turns `null` into an array, and each value is evaluated after the appends before it.

use mago_syntax::cst::{ExpressionStatement, Node, NodeKind};
use pun_problem::{Fix, Safety};

use super::rector_code_quality::{args, call_to, operand, statements_edit, text};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferArrayAppend;

impl Rule for PreferArrayAppend {
    fn id(&self) -> &'static str {
        "lint/arrays/prefer-array-append"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::ExpressionStatement])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        if let Node::ExpressionStatement(statement) = node {
            check(statement, ancestors, ctx, out);
        }
    }
}

fn check(
    statement: &ExpressionStatement<'_>,
    ancestors: &[Node<'_, '_>],
    ctx: &Context<'_>,
    out: &mut Vec<Report>,
) {
    let Some(call) = call_to(statement.expression, "array_push") else {
        return;
    };
    let args = args(call);
    // hasArraySpread, then the array and at least one value.
    if args.iter().any(|a| a.is_unpacked()) || args.len() < 2 {
        return;
    }
    // `$items[] = $value` is printed by `pPrefixOp` for `Assign` (precedence 160), which wraps a value that binds
    // looser (`and`, `xor`, `or`). Rector's value is the original node, which its printer never wraps, so
    // `array_push($items, $a and $b)` would become `$items[] = $a and $b`; Pun keeps the parentheses.
    let array = text(ctx, args[0].value());
    let parts: Vec<String> = args[1..]
        .iter()
        .map(|a| format!("{array}[] = {}", operand(ctx, a.value(), 160, None)))
        .collect();
    let Some(edit) = statements_edit(ctx, ancestors, statement, &parts) else {
        return;
    };
    out.push(Report {
        span: node_span(statement),
        message: "Use `$array[] =` instead of `array_push()`.".to_string(),
        help: None,
        fix: Some(Fix {
            safety: Safety::Unsafe,
            edits: vec![edit],
        }),
    });
}
