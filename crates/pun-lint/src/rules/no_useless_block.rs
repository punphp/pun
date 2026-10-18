//! `lint/complexity/no-useless-block`, ported from Rector 2.6.1's `ReplaceBlockToItsStmtsRector`
//! (rules/DeadCode/Rector/Block).
//!
//! A `{ ... }` block that stands as a statement of its own (php-parser's `Stmt\Block`, not the body of a control
//! structure, a function or a `try`) is replaced by its statements, or removed when it has none.
//!
//! Mago 1.57.0's `no-redundant-block` (mapped "same") reports the same blocks on the fixtures and on blocks in
//! functions, `if` bodies and `case`s, and has no fix.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::php_parser::{block_list, enclosing, line_indent, moved_text, remove, tokens};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoUselessBlock;

impl Rule for NoUselessBlock {
    fn id(&self) -> &'static str {
        "lint/complexity/no-useless-block"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Block])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Block(block) = node else { return };
        // A cheap look before the file is read into tokens: the block is a statement in a list, not a body.
        let n = ancestors.len();
        if n < 2 || !matches!(ancestors[n - 1], Node::Statement(_)) {
            return;
        }
        if !matches!(
            ancestors[n - 2],
            Node::Block(_)
                | Node::Program(_)
                | Node::NamespaceImplicitBody(_)
                | Node::SwitchExpressionCase(_)
                | Node::SwitchDefaultCase(_)
                | Node::IfColonDelimitedBody(_)
                | Node::IfColonDelimitedBodyElseIfClause(_)
                | Node::IfColonDelimitedBodyElseClause(_)
                | Node::WhileColonDelimitedBody(_)
                | Node::ForColonDelimitedBody(_)
                | Node::ForeachColonDelimitedBody(_)
                | Node::DeclareColonDelimitedBody(_)
        ) {
            return;
        }
        let source = ctx.source;
        let tokens = tokens(source);
        let (list, index) = enclosing(source, &tokens, ancestors);
        let inner = block_list(&tokens, block);
        let span = node_span(block);
        let edits = if inner.is_empty() {
            remove(source, &tokens, &list, &[index])
        } else {
            let mut text = moved_text(source, &tokens, &inner, line_indent(source, span.start));
            // php-parser prints the Nop of a trailing comment as an empty line.
            if inner.nop && index + 1 < list.stmts.len() {
                text.push('\n');
            }
            vec![Edit::new(span, text)]
        };
        out.push(Report {
            span,
            message: "This block groups nothing; use its statements directly.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}
