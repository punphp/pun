//! `lint/dead-code/no-identical-branches`, ported from Rector 2.6.1's `SimplifyIfElseWithSameContentRector`
//! (rules/DeadCode/Rector/If_).
//!
//! An `if` with an `else` whose branches all hold the same code (comments included) runs that code whatever the
//! conditions say; the `if` is replaced by the code, or removed when the branches are empty. The conditions are not
//! kept, so a condition with side effects goes too.

use mago_syntax::cst::{Node, NodeKind, Statement};
use pun_problem::{Edit, Fix, Safety};

use super::rector_stmts::{
    Frame, enclosing, if_view, inserted_statements, line_indent, moved_statements, remove,
    same_with_comments,
};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoIdenticalBranches;

impl Rule for NoIdenticalBranches {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-identical-branches"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Statement])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Statement(s @ Statement::If(r#if)) = node else {
            return;
        };
        let source = ctx.source;
        let view = if_view(source, r#if);
        let Some((else_stmts, _)) = &view.else_ else {
            return;
        };
        // isIfWithConstantReturns: one content for all branches, printed with their comments.
        if !view
            .elseifs
            .iter()
            .all(|(_, list, _)| same_with_comments(ctx, &view.stmts, list))
            || !same_with_comments(ctx, &view.stmts, else_stmts)
        {
            return;
        }
        let (list, index) = enclosing(source, s, ancestors);
        let span = node_span(list.stmts[index]);
        let edits = if view.stmts.is_empty() {
            remove(ctx, &list, &[index])
        } else {
            let indent = line_indent(source, span.start);
            let text = if list.frame == Frame::Bare {
                // php-parser prints the statements the single statement of a control structure becomes in braces.
                let inner = format!("{indent}    ");
                format!(
                    "{{\n{inner}{}\n{indent}}}",
                    moved_statements(ctx, &view.stmts, &inner)
                )
            } else {
                inserted_statements(ctx, &view.stmts, indent)
            };
            vec![Edit::new(span, text)]
        };
        out.push(Report {
            span,
            message: "Every branch of this `if` holds the same code.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}
