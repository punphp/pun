//! Mago 1.57.0's linter helper for the early-exit rules (rule/utils/early_exit.rs), used by lint-b's port of
//! `prefer-early-continue`.

use mago_syntax::cst::{
    BinaryOperator, Block, Expression, If, IfBody, Statement, UnaryPrefixOperator,
};
use pun_problem::{Edit, Fix, Safety, Span};

use crate::Report;
use crate::syntax::node_span;

pub struct EarlyExitPattern {
    pub keyword: &'static str,
    pub title: &'static str,
    pub help: &'static str,
    pub is_early_exit_statement: fn(&Statement<'_>) -> bool,
}

/// `check_early_exit`: an `if` without `else`/`elseif` whose body has more than `max_allowed_statements` statements
/// and is not itself an early exit is reported on the `if`, with a fix that negates the condition, exits in the
/// `if` and moves the body after it.
pub fn check_early_exit(
    source: &str,
    if_stmt: &If<'_>,
    max_allowed_statements: usize,
    pattern: &EarlyExitPattern,
    out: &mut Vec<Report>,
) {
    let (has_else, body_len, is_early_exit) = match &if_stmt.body {
        IfBody::Statement(body) => (
            !body.else_if_clauses.is_empty() || body.else_clause.is_some(),
            statement_len(body.statement),
            (pattern.is_early_exit_statement)(body.statement),
        ),
        IfBody::ColonDelimited(body) => (
            !body.else_if_clauses.is_empty() || body.else_clause.is_some(),
            body.statements.len(),
            body.statements.len() == 1
                && (pattern.is_early_exit_statement)(&body.statements.nodes[0]),
        ),
    };
    if has_else || body_len <= max_allowed_statements || is_early_exit {
        return;
    }

    let mut edits = Vec::new();
    let replace = |span: Span, text: &str| Edit::new(span, text);
    let condition = if_stmt.condition;
    let (start, end) = (node_span(condition).start, node_span(condition).end);
    let negate = |edits: &mut Vec<Edit>| {
        edits.push(replace(Span::new(start, start), "!("));
        edits.push(replace(Span::new(end, end), ")"));
    };
    match condition {
        Expression::UnaryPrefix(unary) if matches!(unary.operator, UnaryPrefixOperator::Not(_)) => {
            let operand = node_span(unary.operand);
            edits.push(replace(
                Span::new(start, end),
                &source[operand.start..operand.end],
            ));
        }
        Expression::Binary(binary) => {
            let negated = match binary.operator {
                BinaryOperator::Equal(_) => Some("!="),
                BinaryOperator::NotEqual(_) => Some("=="),
                BinaryOperator::Identical(_) => Some("!=="),
                BinaryOperator::NotIdentical(_) => Some("==="),
                BinaryOperator::AngledNotEqual(_) => Some("=="),
                BinaryOperator::LessThan(_) => Some(">="),
                BinaryOperator::LessThanOrEqual(_) => Some(">"),
                BinaryOperator::GreaterThan(_) => Some("<="),
                BinaryOperator::GreaterThanOrEqual(_) => Some("<"),
                _ => None,
            };
            match negated {
                Some(op) => edits.push(replace(node_span(&binary.operator), op)),
                None => negate(&mut edits),
            }
        }
        _ => negate(&mut edits),
    }

    let gap_has_comment = |from: usize, to: usize| {
        source.as_bytes()[from..to]
            .iter()
            .any(|b| !b.is_ascii_whitespace())
    };
    let exit_block = format!("{{ {}; }}", pattern.keyword);
    // The body's statements, the end of what opens them (`{` or `:`) and the span of what closes them (`}`, or
    // `endif;`), or for a body that is one statement without braces, that statement.
    let (statements, open, close) = match &if_stmt.body {
        IfBody::Statement(body) => match body.statement {
            Statement::Block(block) => (
                block.statements.as_slice(),
                node_span(&block.left_brace),
                node_span(&block.right_brace),
            ),
            statement => {
                let paren_end = node_span(&if_stmt.right_parenthesis).end;
                let stmt_start = node_span(statement).start;
                edits.push(if gap_has_comment(paren_end, stmt_start) {
                    replace(Span::new(paren_end, paren_end), &format!(" {exit_block}\n"))
                } else {
                    replace(
                        Span::new(paren_end, stmt_start),
                        &format!(" {exit_block}\n\n"),
                    )
                });
                return push(if_stmt, pattern, edits, out);
            }
        },
        IfBody::ColonDelimited(body) => {
            let endif = node_span(&body.endif);
            let terminator = node_span(&body.terminator);
            (
                body.statements.as_slice(),
                node_span(&body.colon),
                Span::new(endif.start, terminator.end),
            )
        }
    };
    // The body is not empty: `body_len` is above `max_allowed_statements` (Mago's fix for an empty body only runs
    // when that option is below 0, which it cannot be).
    let (first, last) = (&statements[0], &statements[statements.len() - 1]);
    let first_start = node_span(first).start;
    edits.push(if gap_has_comment(open.end, first_start) {
        replace(open, &format!("{exit_block}\n"))
    } else {
        replace(
            Span::new(open.start, first_start),
            &format!("{exit_block}\n\n"),
        )
    });
    let last_end = node_span(last).end;
    edits.push(if gap_has_comment(last_end, close.start) {
        replace(close, "")
    } else {
        replace(Span::new(last_end, close.end), "")
    });
    push(if_stmt, pattern, edits, out);
}

fn push(if_stmt: &If<'_>, pattern: &EarlyExitPattern, edits: Vec<Edit>, out: &mut Vec<Report>) {
    out.push(Report {
        span: node_span(if_stmt),
        message: pattern.title.to_string(),
        help: Some(pattern.help.to_string()),
        fix: Some(Fix {
            safety: Safety::Safe,
            edits,
        }),
    });
}

/// `extract_single_if_from_statement`.
pub fn single_if<'ast, 'arena>(stmt: &'ast Statement<'arena>) -> Option<&'ast If<'arena>> {
    match stmt {
        Statement::If(r#if) => Some(r#if),
        Statement::Block(block) => single_if_in(block.statements.as_slice()),
        _ => None,
    }
}

/// `extract_single_if_from_statements`: the `if` that is the only statement that is not empty.
pub fn single_if_in<'ast, 'arena>(stmts: &'ast [Statement<'arena>]) -> Option<&'ast If<'arena>> {
    let mut non_empty = stmts.iter().filter(|s| statement_len(s) > 0);
    match (non_empty.next(), non_empty.next()) {
        (Some(only), None) => single_if(only),
        _ => None,
    }
}

fn statement_len(stmt: &Statement<'_>) -> usize {
    match stmt {
        Statement::Noop(_) => 0,
        Statement::Block(Block { statements, .. }) => statements.len(),
        _ => 1,
    }
}
