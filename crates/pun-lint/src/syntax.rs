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

/// Whether `expr` is `$this`, compared like `NodeNameResolver::isName` (ignoring case), through parentheses.
pub fn is_this(expr: &Expression<'_>) -> bool {
    matches!(unparenthesize(expr), Expression::Variable(mago_syntax::cst::Variable::Direct(v)) if v.name.eq_ignore_ascii_case(b"$this"))
}

/// Whether two spans of `source` hold the same tokens, ignoring whitespace and comments. This stands in for Rector's
/// `NodeComparator::areNodesEqual`, which compares the two nodes printed without comments.
pub fn same_code(source: &str, a: Span, b: Span) -> bool {
    tokens(&source[a.start..a.end]) == tokens(&source[b.start..b.end])
}

/// Rector's `NodeComparator::areNodesEqual` on Mago's tree: the two nodes are the same code once printed without
/// comments. php-parser keeps no node for parentheses and its printer only writes the ones precedence needs, so the
/// parentheses of every parenthesized expression inside either node are left out, as are whitespace and comments.
pub fn same_node(source: &str, a: Node<'_, '_>, b: Node<'_, '_>) -> bool {
    printed_tokens(source, a) == printed_tokens(source, b)
}

/// [`same_node`] for two expressions.
pub fn same_expression(source: &str, a: &Expression<'_>, b: &Expression<'_>) -> bool {
    same_node(source, Node::Expression(a), Node::Expression(b))
}

fn printed_tokens<'s>(
    source: &'s str,
    node: Node<'_, '_>,
) -> Vec<(mago_syntax::token::TokenKind, &'s [u8])> {
    let span = node_span(&node);
    let mut parentheses = Vec::new();
    collect_parentheses(node, &mut parentheses);
    let code = &source[span.start..span.end];
    let mut lexer = Lexer::scripting(
        Input::new(FileId::zero(), code.as_bytes()),
        LexerSettings::default(),
    );
    let mut out = Vec::new();
    while let Some(Ok(token)) = lexer.advance() {
        let at = span.start + token.start.offset as usize;
        if !token.kind.is_trivia() && !parentheses.contains(&at) {
            out.push((token.kind, &source.as_bytes()[at..at + token.value.len()]));
        }
    }
    out
}

fn collect_parentheses(node: Node<'_, '_>, out: &mut Vec<usize>) {
    if let Node::Parenthesized(p) = node {
        out.push(p.left_parenthesis.start.offset as usize);
        out.push(p.right_parenthesis.start.offset as usize);
    }
    node.visit_children(|child| collect_parentheses(child, out));
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
