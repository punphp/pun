//! Helpers that rules share for reading Mago's syntax tree.

use mago_database::file::FileId;
use mago_span::HasSpan;
use mago_syntax::cst::{Expression, Node, Statement};
use mago_syntax::lexer::Lexer;
use mago_syntax::settings::LexerSettings;
use mago_syntax_core::input::Input;
use pun_problem::Span;

/// A Mago span as a byte range of the file.
pub fn span(s: mago_span::Span) -> Span {
    Span::new(s.start.offset as usize, s.end.offset as usize)
}

/// The expression inside any number of parentheses. php-parser has no node for parentheses, so a Rector rule never
/// sees them.
pub fn unparenthesize<'a>(mut expr: &'a Expression<'a>) -> &'a Expression<'a> {
    while let Expression::Parenthesized(p) = expr {
        expr = p.expression;
    }
    expr
}

/// Whether two spans of `source` hold the same tokens, ignoring whitespace and comments. This stands in for Rector's
/// `NodeComparator::areNodesEqual`, which compares the two nodes printed without comments.
pub fn same_code(source: &str, a: Span, b: Span) -> bool {
    tokens(&source[a.start..a.end]) == tokens(&source[b.start..b.end])
}

fn tokens(code: &str) -> Vec<(mago_syntax::token::TokenKind, &[u8])> {
    let mut lexer = Lexer::scripting(
        Input::new(FileId::zero(), code.as_bytes()),
        LexerSettings::default(),
    );
    let mut out = Vec::new();
    while let Some(Ok(token)) = lexer.advance() {
        if !token.kind.is_trivia() {
            out.push((token.kind, token.value));
        }
    }
    out
}

/// The statements directly under `node`, the list php-parser keeps in a node's `stmts`: a file, a block, a namespace
/// body, a case, the body of an `if` or a loop.
pub fn statements<'ast, 'arena>(node: Node<'ast, 'arena>) -> Vec<&'ast Statement<'arena>> {
    let mut out = Vec::new();
    node.visit_children(|child| {
        if let Node::Statement(s) = child {
            out.push(s);
        }
    });
    out
}

/// The span of a node as a byte range.
pub fn node_span(node: &impl HasSpan) -> Span {
    span(node.span())
}
