//! php-parser's format-preserving printer on a statement list in which Rector replaced one statement by others
//! (`refactor()` returning an array of nodes): `pArray` with the `Differ`'s edit script.
//!
//! The old statement and the new ones are a run of one removal and several additions, which the `Differ` turns into
//! a replacement when there is one new statement. A replacement keeps everything before the old statement's comments
//! and writes the new statement's comments (the old ones go). A removal takes the old statement with its comments;
//! after a statement it also takes the blanks back to that statement, and the new statements follow it a line each
//! at its indent; at the start of the list the blanks before the old statement stay, the blanks after it go, and the
//! new statements are written a line each before the next statement, at its indent. With no next statement, or where
//! a control structure's single statement without braces becomes several, php-parser gives up on the list and prints
//! the node holding it anew; [`splice`] gives `None` there. Rector changes every statement of a list in one pass, so
//! the statements a rule changes next to each other are one run of removals and additions.

use mago_span::HasSpan;
use mago_syntax::cst::{Expression, ForeachTarget, Node, Statement};
use pun_problem::{Edit, Span};

use super::no_redundant_null_check::reprint_function_like;

use super::php_parser::{Frame, List, Token, enclosing, previous_token_end, remove};
use super::rector_conditions::{comments_between, indentation, print_comments, reprint};
use crate::syntax::node_span;

/// A statement of the new list that is not in the old one.
pub(crate) struct NewStatement {
    /// The comments php-parser keeps with it.
    pub comments: Vec<Span>,
    pub code: Code,
}

#[derive(Clone)]
pub(crate) enum Code {
    /// A statement of the file, printed with its code moved to the new indent.
    Moved(Span),
    /// A statement printed anew, on one line.
    Text(String),
    /// php-parser's `Nop` for comments after the last statement of a list: only its comments print.
    Nop,
}

impl NewStatement {
    /// The statement at `span`, with the comments before it.
    pub(crate) fn moved(source: &str, tokens: &[Token], span: Span) -> Self {
        let before = previous_token_end(source, tokens, span.start);
        NewStatement {
            comments: comments_between(tokens, before, span.start),
            code: Code::Moved(span),
        }
    }

    /// The statements of `list`, each with its comments, and its `Nop`.
    pub(crate) fn all(source: &str, tokens: &[Token], list: &List<'_, '_>) -> Vec<Self> {
        let mut out: Vec<Self> = list
            .stmts
            .iter()
            .map(|s| Self::moved(source, tokens, node_span(*s)))
            .collect();
        if list.nop {
            let after = list
                .stmts
                .last()
                .map_or(list.inner.start, |s| node_span(*s).end);
            out.push(NewStatement {
                comments: comments_between(tokens, after, list.inner.end),
                code: Code::Nop,
            });
        }
        out
    }

    /// The statement as `pStmts` prints it in a list printed anew at the indent `level`: its comments after a line
    /// break, then (unless it is a `Nop`) its code after another.
    pub(crate) fn print_in_list(
        &self,
        source: &str,
        tokens: &[Token],
        level: usize,
        nl: &str,
    ) -> String {
        let mut out = String::new();
        if !self.comments.is_empty() {
            out.push_str(nl);
            out.push_str(&print_comments(source, &self.comments, nl));
        }
        if !matches!(self.code, Code::Nop) {
            out.push_str(nl);
            out.push_str(
                &Self {
                    comments: Vec::new(),
                    code: self.code.clone(),
                }
                .print(source, tokens, level, nl),
            );
        }
        out
    }

    /// `p($node)` at the indent `level`, after its comments, lines joined by `nl`.
    fn print(&self, source: &str, tokens: &[Token], level: usize, nl: &str) -> String {
        let mut out = String::new();
        if !self.comments.is_empty() {
            out.push_str(&print_comments(source, &self.comments, nl));
            out.push_str(nl);
        }
        match &self.code {
            Code::Moved(span) => out.push_str(&reprint(
                source,
                tokens,
                *span,
                level as isize - indentation(source, span.start) as isize,
            )),
            Code::Text(text) => out.push_str(text),
            Code::Nop => {}
        }
        out
    }
}

/// Where the statement at `index` of `list` starts with the comments php-parser keeps with it.
fn comment_start(source: &str, tokens: &[Token], list: &List<'_, '_>, index: usize) -> usize {
    let span = node_span(list.stmts[index]);
    let before = previous_token_end(source, tokens, span.start).max(list.inner.start);
    comments_between(tokens, before, span.start)
        .first()
        .map_or(span.start, |c| c.start)
}

/// The edits that replace the `count` statements of `list` from `index` on (a run of changed statements, each
/// replaced or removed) by `new`, or `None` where php-parser prints the list's holder anew.
pub(crate) fn splice(
    source: &str,
    tokens: &[Token],
    list: &List<'_, '_>,
    index: usize,
    count: usize,
    new: &[NewStatement],
) -> Option<Vec<Edit>> {
    let has_nop = list.nop || new.iter().any(|n| matches!(n.code, Code::Nop));
    let last = node_span(list.stmts[index + count - 1]);
    if new.is_empty() {
        let indices: Vec<usize> = (index..index + count).collect();
        return Some(remove(source, tokens, list, &indices));
    }
    if new.len() == count {
        // the `Differ` pairs the removals with the additions: each a replacement
        return Some(
            new.iter()
                .enumerate()
                .map(|(j, one)| {
                    let old = node_span(list.stmts[index + j]);
                    let start = comment_start(source, tokens, list, index + j);
                    let level = indentation(source, old.start);
                    let nl = format!("\n{}", " ".repeat(level));
                    let text = one.print(source, tokens, level, &nl);
                    strip_nop_blanks(source, Span::new(start, old.end), text, has_nop)
                })
                .collect(),
        );
    }
    if list.frame == Frame::Bare {
        return None;
    }
    let start = comment_start(source, tokens, list, index);
    let (span, text) = if index > 0 {
        // removed after a kept statement: the new ones follow that statement, at its indent
        let previous = node_span(list.stmts[index - 1]);
        let level = indentation(source, previous.start);
        let nl = format!("\n{}", " ".repeat(level));
        let before = previous_token_end(source, tokens, start).max(list.inner.start);
        let text: String = new
            .iter()
            .map(|n| format!("{nl}{}", n.print(source, tokens, level, &nl)))
            .collect();
        (Span::new(before, last.end), text)
    } else {
        // removed from the start: the new ones go before the next statement (or the `Nop`), at its indent
        let next = if index + count < list.stmts.len() {
            comment_start(source, tokens, list, index + count)
        } else if list.nop {
            comments_between(tokens, last.end, list.inner.end)
                .first()?
                .start
        } else {
            return None;
        };
        let level = indentation(source, next);
        let nl = format!("\n{}", " ".repeat(level));
        let text: String = new
            .iter()
            .map(|n| format!("{}{nl}", n.print(source, tokens, level, &nl)))
            .collect();
        (Span::new(start, next), text)
    };
    Some(vec![strip_nop_blanks(source, span, text, has_nop)])
}

/// The edit of `text` at `span`, with `BetterStandardPrinter::pArray`'s clean-up: when the new list holds a `Nop`,
/// lines of only blanks lose their blanks.
fn strip_nop_blanks(source: &str, span: Span, text: String, has_nop: bool) -> Edit {
    if !has_nop {
        return Edit::new(span, text);
    }
    let line_start = source[..span.start].rfind('\n').map_or(0, |i| i + 1);
    let line_end = source[span.end..]
        .find('\n')
        .map_or(source.len(), |i| span.end + i);
    let whole = format!(
        "{}{text}{}",
        &source[line_start..span.start],
        &source[span.end..line_end]
    );
    let cleaned: Vec<&str> = whole
        .split('\n')
        .map(|line| if line.trim().is_empty() { "" } else { line })
        .collect();
    Edit::new(Span::new(line_start, line_end), cleaned.join("\n"))
}

/// What a rule's `refactor()` does to one statement of a list.
pub(crate) enum Change {
    /// Edits inside the statement, which stays in the list.
    Edits(Vec<Edit>),
    /// `NodeVisitor::REMOVE_NODE`.
    Remove,
    /// The statement replaced by these.
    Replace(Vec<NewStatement>),
}

/// The edits for the statement `ancestors` ends with (a `Node::Statement`), given what `change` does to a statement
/// of its list (called with the chain to it): Rector changes the statements of a list in one pass, so the statement
/// a run of removed or replaced statements starts with gets the run's edits, and the others none.
pub(crate) fn run_edits<'ast, 'arena>(
    source: &str,
    tokens: &[Token],
    ancestors: &[Node<'ast, 'arena>],
    change: &dyn Fn(&[Node<'ast, 'arena>]) -> Option<Change>,
) -> Option<Vec<Edit>> {
    let n = ancestors.len();
    let at = |statement: &'ast Statement<'arena>| {
        let mut chain = ancestors.to_vec();
        chain[n - 1] = Node::Statement(statement);
        change(&chain)
    };
    let replaced = |c: &Option<Change>| matches!(c, Some(Change::Remove | Change::Replace(_)));
    let own = change(ancestors)?;
    if let Change::Edits(edits) = own {
        return Some(edits);
    }
    let (list, index) = enclosing(source, tokens, ancestors);
    if index > 0 && replaced(&at(list.stmts[index - 1])) {
        return None;
    }
    let mut new = Vec::new();
    let mut count = 0;
    let mut next = Some(own);
    while let Some(Change::Remove | Change::Replace(_)) = &next {
        if let Some(Change::Replace(statements)) = next {
            new.extend(statements);
        }
        count += 1;
        next = list.stmts.get(index + count).and_then(|s| at(s));
    }
    match splice(source, tokens, &list, index, count, &new) {
        Some(edits) => Some(edits),
        None => holder_reprint(source, tokens, ancestors, &new),
    }
}

/// The node whose statement list was the changed statements alone, printed anew with `new` as its statements:
/// php-parser's `pArray` gives up on a list whose only statement it removes, and the printer falls back to printing
/// the node holding it (`pStmt_ClassMethod`, `pStmt_Function`, `pExpr_Closure`, `pStmt_Foreach`, `pStmt_While`,
/// `pStmt_If`, `pStmt_ElseIf`, `pStmt_Else`, `pStmt_Case`). Pun prints those; under any other node it leaves the
/// code alone. `pStmt_If` keeps the code of its `elseif` and `else` clauses, one space before each.
fn holder_reprint(
    source: &str,
    tokens: &[Token],
    ancestors: &[Node<'_, '_>],
    new: &[NewStatement],
) -> Option<Vec<Edit>> {
    let n = ancestors.len();
    let parent = ancestors.get(n.checked_sub(2)?)?;
    let body = |level: usize| -> String {
        let nl = format!("\n{}", " ".repeat(level));
        new.iter()
            .map(|s| s.print_in_list(source, tokens, level, &nl))
            .collect()
    };
    let text = |s: mago_span::Span| &source[s.start.offset as usize..s.end.offset as usize];
    let case = |span: mago_span::Span, head: String| {
        let span = node_span(&span);
        let level = indentation(source, span.start);
        Some(vec![Edit::new(span, format!("{head}:{}", body(level + 4)))])
    };
    match parent {
        Node::SwitchExpressionCase(c) => {
            return case(c.span(), format!("case {}", text(c.expression.span())));
        }
        Node::SwitchDefaultCase(c) => return case(c.span(), "default".to_string()),
        Node::Block(_) => {}
        _ => return None,
    }
    let owner = ancestors[..n - 2].iter().rev().find(|a| {
        !matches!(
            a,
            Node::Statement(_) | Node::MethodBody(_) | Node::ForeachBody(_) | Node::WhileBody(_)
        )
    })?;
    // `head {` and the statements at the indent of the line `start` is on, then `}`
    let braced = |start: usize, end: usize, head: String| {
        let level = indentation(source, start);
        Edit::new(
            Span::new(start, end),
            format!("{head} {{{}\n{}}}", body(level + 4), " ".repeat(level)),
        )
    };
    match owner {
        Node::IfStatementBody(if_body) => {
            let r#if = ancestors[..n - 2].iter().rev().find_map(|a| match a {
                Node::If(r#if) => Some(*r#if),
                _ => None,
            })?;
            let start = node_span(r#if).start;
            let end = node_span(if_body.statement).end;
            let mut edits = vec![braced(
                start,
                end,
                format!("if ({})", text(r#if.condition.span())),
            )];
            let mut previous = end;
            let clauses = (if_body.else_if_clauses.iter().map(mago_span::HasSpan::span))
                .chain(if_body.else_clause.iter().map(mago_span::HasSpan::span));
            for clause in clauses {
                let clause = node_span(&clause);
                edits.push(Edit::new(
                    Span::new(previous, clause.start),
                    " ".to_string(),
                ));
                previous = clause.end;
            }
            Some(edits)
        }
        Node::IfStatementBodyElseIfClause(clause) => {
            let span = node_span(*clause);
            Some(vec![braced(
                span.start,
                span.end,
                format!("elseif ({})", text(clause.condition.span())),
            )])
        }
        Node::IfStatementBodyElseClause(clause) => {
            let span = node_span(*clause);
            Some(vec![braced(span.start, span.end, "else".to_string())])
        }
        Node::Foreach(foreach) => {
            let text = |e: &Expression<'_>| {
                let s = node_span(e);
                &source[s.start..s.end]
            };
            let target = match &foreach.target {
                ForeachTarget::Value(v) => text(v.value).to_string(),
                ForeachTarget::KeyValue(kv) => format!("{} => {}", text(kv.key), text(kv.value)),
            };
            let span = node_span(*foreach);
            let level = indentation(source, span.start);
            Some(vec![Edit::new(
                span,
                format!(
                    "foreach ({} as {target}) {{{}\n{}}}",
                    text(foreach.expression),
                    body(level + 4),
                    " ".repeat(level)
                ),
            )])
        }
        Node::While(r#while) => {
            let span = node_span(*r#while);
            let level = indentation(source, span.start);
            let condition = node_span(r#while.condition);
            Some(vec![Edit::new(
                span,
                format!(
                    "while ({}) {{{}\n{}}}",
                    &source[condition.start..condition.end],
                    body(level + 4),
                    " ".repeat(level)
                ),
            )])
        }
        holder => reprint_function_like(source, tokens, *holder, &body).map(|e| vec![e]),
    }
}
