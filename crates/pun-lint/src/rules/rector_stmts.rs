//! php-parser's view of Mago's tree for the dead-code and early-return rules ported from Rector (lint 7 batch 3):
//! statement lists as php-parser keeps them (`stmts`, with the `Nop` it adds for comments at the end of a list), the
//! parts of an `if`, what Rector's format-preserving printer writes when a rule removes, moves or replaces statements,
//! and the checks several of these rules share (`TerminatedNodeAnalyzer`'s last-statement checks, `NodeComparator`
//! with and without comments, `VariableAnalyzer`'s static, global and by-reference variables).
//!
//! The list, removal and re-indent helpers are a local copy of batch 1's `php_parser.rs` (in flight when this batch
//! was written); PLAN.md has a follow-up row to merge the two.

use mago_database::file::FileId;
use mago_span::HasSpan;
use mago_syntax::cst::*;
use mago_syntax::lexer::Lexer;
use mago_syntax::settings::LexerSettings;
use mago_syntax::token::TokenKind;
use mago_syntax_core::input::Input;
use pun_problem::{Edit, Span};

use crate::syntax::{node_span, unparenthesize};

/// One token of a region of the file: its kind and byte range in the file.
#[derive(Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
}

/// The tokens of `source[from..to]`, which starts between two statements (or at the start of the file, where it is
/// read as PHP reads a file: inline HTML first).
pub fn tokens(source: &str, from: usize, to: usize) -> Vec<Token> {
    let input = Input::new(FileId::zero(), &source.as_bytes()[from..to]);
    let mut lexer = if from == 0 {
        Lexer::new(input, LexerSettings::default())
    } else {
        Lexer::scripting(input, LexerSettings::default())
    };
    let mut out = Vec::new();
    while let Some(Ok(token)) = lexer.advance() {
        let start = from + token.start.offset as usize;
        out.push(Token {
            kind: token.kind,
            start,
            end: start + token.value.len(),
        });
    }
    out
}

/// The end of the last token in `[from, pos)` that is not whitespace or a comment, or `from` when there is none.
/// php-parser's `T_OPEN_TAG` takes one newline after `<?php`, so that newline counts as part of the tag.
pub fn previous_token_end(source: &str, from: usize, pos: usize) -> usize {
    let Some(token) = tokens(source, from, pos)
        .into_iter()
        .rev()
        .find(|t| !t.kind.is_trivia())
    else {
        return from;
    };
    let mut end = token.end;
    if token.kind == TokenKind::OpenTag && source.as_bytes().get(end) == Some(&b'\n') && end < pos {
        end += 1;
    }
    end
}

/// Whether a comment comes in `[from, to)` after the last token there that is not trivia: php-parser's
/// `maybeCreateZeroLengthNop`, which turns the comments before a list's closing token into a `Nop` statement.
pub fn trailing_comment(source: &str, from: usize, to: usize) -> bool {
    let mut found = false;
    for token in tokens(source, from, to) {
        if token.kind.is_comment() {
            found = true;
        } else if !token.kind.is_trivia() {
            found = false;
        }
    }
    found
}

/// The start of the first comment in `[from, to)`, the comments php-parser keeps with the statement at `to`.
pub fn first_comment(source: &str, from: usize, to: usize) -> Option<usize> {
    tokens(source, from, to)
        .into_iter()
        .find(|t| t.kind.is_comment())
        .map(|t| t.start)
}

/// The whitespace at the start of the line that holds `pos`.
pub fn line_indent(source: &str, pos: usize) -> &str {
    let line_start = source[..pos].rfind('\n').map_or(0, |i| i + 1);
    let rest = &source[line_start..];
    let len = rest.len() - rest.trim_start_matches([' ', '\t']).len();
    &rest[..len]
}

/// How a statement list is delimited, which decides what the printer writes when the list ends up empty.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Frame {
    /// `{ ... }`: `open` is just after `{`, `close` is at `}`. An emptied block prints as `{`, a newline and `}`.
    Braces { open: usize, close: usize },
    /// The single statement of a control structure written without braces. Emptied, it prints as `{`, `}`.
    Bare,
    /// A file, a namespace without braces, a `case`, or an alternative-syntax body (`:` ... `endif;`).
    Open,
}

/// A statement list as php-parser keeps it.
pub struct List<'a> {
    /// The statements, without Mago's opening and closing tags and empty `;` statements, which php-parser drops.
    pub stmts: Vec<&'a Statement<'a>>,
    /// Comments after the last statement: php-parser adds a `Nop` statement for them.
    pub nop: bool,
    pub frame: Frame,
    /// The bytes of the list's contents, from after its opening token to its closing token.
    pub inner: Span,
}

impl<'a> List<'a> {
    /// php-parser's `stmts === []`.
    pub fn is_empty(&self) -> bool {
        self.stmts.is_empty() && !self.nop
    }

    /// The number of php-parser statements, the `Nop` included.
    pub fn len(&self) -> usize {
        self.stmts.len() + usize::from(self.nop)
    }

    /// php-parser's last statement, `None` when the list is empty or ends with the `Nop` of a comment.
    pub fn last(&self) -> Option<&'a Statement<'a>> {
        if self.nop {
            return None;
        }
        self.stmts.last().copied()
    }

    /// The span of the statement at `i` with the comments before it, which php-parser keeps with it.
    pub fn with_comments(&self, source: &str, i: usize) -> Span {
        let span = node_span(self.stmts[i]);
        let before = self.before(source, i);
        Span::new(
            first_comment(source, before, span.start).unwrap_or(span.start),
            span.end,
        )
    }

    /// The end of the token before the statement at `i` (its comments are after it).
    pub fn before(&self, source: &str, i: usize) -> usize {
        let from = if i == 0 {
            self.inner.start
        } else {
            node_span(self.stmts[i - 1]).end
        };
        previous_token_end(source, from, node_span(self.stmts[i]).start)
    }

    /// The span of the comments after the last statement (php-parser's trailing `Nop`).
    pub fn nop_span(&self, source: &str) -> Option<Span> {
        if !self.nop {
            return None;
        }
        let after = self
            .stmts
            .last()
            .map_or(self.inner.start, |s| node_span(*s).end);
        let start = first_comment(source, after, self.inner.end).unwrap_or(after);
        let end = self.inner.start + source[self.inner.start..self.inner.end].trim_end().len();
        Some(Span::new(start, end))
    }
}

fn kept(statement: &Statement<'_>) -> bool {
    match statement {
        Statement::OpeningTag(_) | Statement::ClosingTag(_) | Statement::Noop(_) => false,
        // php-parser's `T_CLOSE_TAG` takes the newline after `?>`, which Mago keeps as inline HTML of its own.
        Statement::Inline(inline) => {
            !(matches!(inline.value, b"\n" | b"\r\n") && inline.span.start.offset > 0)
        }
        _ => true,
    }
}

fn list<'a>(
    source: &str,
    all: impl IntoIterator<Item = &'a Statement<'a>>,
    frame: Frame,
    inner: Span,
    nop: bool,
) -> List<'a> {
    let stmts: Vec<&Statement> = all.into_iter().filter(|s| kept(s)).collect();
    let after = stmts.last().map_or(inner.start, |s| node_span(*s).end);
    List {
        nop: nop && trailing_comment(source, after, inner.end),
        stmts,
        frame,
        inner,
    }
}

fn start(s: mago_span::Span) -> usize {
    s.start.offset as usize
}

fn end(s: mago_span::Span) -> usize {
    s.end.offset as usize
}

/// The statements of a `{ ... }` block.
pub fn block_list<'a>(source: &str, block: &'a Block<'a>) -> List<'a> {
    let (open, close) = (end(block.left_brace), start(block.right_brace));
    list(
        source,
        block.statements.iter(),
        Frame::Braces { open, close },
        Span::new(open, close),
        true,
    )
}

/// php-parser's `stmts` of a control structure whose body is one statement: the block's statements, nothing for `;`,
/// or the statement itself.
pub fn body_list<'a>(source: &str, statement: &'a Statement<'a>) -> List<'a> {
    match statement {
        Statement::Block(block) => block_list(source, block),
        _ => list(
            source,
            [statement],
            Frame::Bare,
            node_span(statement),
            false,
        ),
    }
}

pub fn open_list<'a>(
    source: &str,
    statements: &'a Sequence<'a, Statement<'a>>,
    from: usize,
    to: usize,
) -> List<'a> {
    list(
        source,
        statements.iter(),
        Frame::Open,
        Span::new(from, to),
        true,
    )
}

/// The node kinds that hold a list of statements, php-parser's `StmtsAware` nodes (Rector's `NodeGroup::STMTS_AWARE`)
/// as Mago has them. A control structure whose body is one statement keeps it as a list of one, which no rule of this
/// batch changes.
pub const HOLDERS: &[NodeKind] = &[
    NodeKind::Program,
    NodeKind::Block,
    NodeKind::NamespaceImplicitBody,
    NodeKind::SwitchExpressionCase,
    NodeKind::SwitchDefaultCase,
    NodeKind::IfColonDelimitedBody,
    NodeKind::IfColonDelimitedBodyElseIfClause,
    NodeKind::IfColonDelimitedBodyElseClause,
    NodeKind::WhileColonDelimitedBody,
    NodeKind::ForColonDelimitedBody,
    NodeKind::ForeachColonDelimitedBody,
    NodeKind::DeclareColonDelimitedBody,
];

/// The statement list of a node of [`HOLDERS`], or `None` for a block php-parser has no `StmtsAware` node for (a
/// property hook's body).
pub fn holder_list<'a>(
    source: &str,
    holder: Node<'a, 'a>,
    ancestors: &[Node<'a, 'a>],
) -> Option<List<'a>> {
    let open =
        |s: &'a Sequence<'a, Statement<'a>>, from: usize, to: usize| open_list(source, s, from, to);
    Some(match holder {
        Node::Block(block) => {
            if matches!(ancestors.last(), Some(Node::PropertyHookConcreteBody(_))) {
                return None;
            }
            block_list(source, block)
        }
        Node::Program(program) => open(&program.statements, 0, source.len()),
        Node::NamespaceImplicitBody(body) => {
            let to = body
                .statements
                .last()
                .map_or(end(body.span()), |s| end(s.span()));
            open(&body.statements, end(body.terminator.span()), to)
        }
        // A `case` has no trailing `Nop`: comments before the next `case` are that case's.
        Node::SwitchExpressionCase(case) => {
            case_list(source, &case.statements, end(case.separator.span()))
        }
        Node::SwitchDefaultCase(case) => {
            case_list(source, &case.statements, end(case.separator.span()))
        }
        Node::IfColonDelimitedBody(body) => {
            let to = body
                .else_if_clauses
                .first()
                .map(|c| start(c.span()))
                .or_else(|| body.else_clause.as_ref().map(|c| start(c.span())))
                .unwrap_or(start(body.endif.span()));
            open(&body.statements, end(body.colon), to)
        }
        Node::IfColonDelimitedBodyElseIfClause(clause) => open(
            &clause.statements,
            end(clause.colon),
            clause_end(source, clause.span()),
        ),
        Node::IfColonDelimitedBodyElseClause(clause) => open(
            &clause.statements,
            end(clause.colon),
            clause_end(source, clause.span()),
        ),
        Node::WhileColonDelimitedBody(body) => open(
            &body.statements,
            end(body.colon),
            start(body.end_while.span()),
        ),
        Node::ForColonDelimitedBody(body) => open(
            &body.statements,
            end(body.colon),
            start(body.end_for.span()),
        ),
        Node::ForeachColonDelimitedBody(body) => open(
            &body.statements,
            end(body.colon),
            start(body.end_foreach.span()),
        ),
        Node::DeclareColonDelimitedBody(body) => open(
            &body.statements,
            end(body.colon),
            start(body.end_declare.span()),
        ),
        _ => return None,
    })
}

/// Where a colon clause's statements end: at the next `elseif`, `else` or `endif` after it.
fn clause_end(source: &str, span: mago_span::Span) -> usize {
    let from = end(span);
    tokens(source, from, source.len())
        .into_iter()
        .find(|t| !t.kind.is_trivia())
        .map_or(source.len(), |t| t.start)
}

fn case_list<'a>(
    source: &str,
    statements: &'a Sequence<'a, Statement<'a>>,
    from: usize,
) -> List<'a> {
    let to = statements.last().map_or(from, |s| end(s.span()));
    list(
        source,
        statements.iter(),
        Frame::Open,
        Span::new(from, to),
        false,
    )
}

/// The edits for Rector's removal of the statements at `indices` from `list`, as php-parser's format-preserving
/// printer removes a list item: a statement goes with the comments before it (php-parser keeps them with the node
/// after them) and the blanks before it, back to the end of the token before it; the list's first statement goes
/// with the blanks after it instead. A list left empty prints as `{`, a newline and `}` at the indent of the line
/// the list opens on, or as `{}` around nothing where a control structure had one statement without braces.
pub fn remove(source: &str, list: &List<'_>, indices: &[usize]) -> Vec<Edit> {
    if indices.len() == list.stmts.len() && !list.nop {
        match list.frame {
            Frame::Braces { open, close } => {
                let indent = line_indent(source, open - 1);
                return vec![Edit::new(Span::new(open, close), format!("\n{indent}"))];
            }
            Frame::Bare => {
                let span = node_span(list.stmts[0]);
                let indent = line_indent(source, span.start);
                return vec![Edit::new(span, format!("{{\n{indent}}}"))];
            }
            Frame::Open => {}
        }
    }
    indices
        .iter()
        .map(|&i| {
            let span = node_span(list.stmts[i]);
            let before = list.before(source, i);
            if i > 0 {
                return Edit::new(Span::new(before, span.end), "");
            }
            let start = first_comment(source, before, span.start).unwrap_or(span.start);
            let rest = &source[span.end..list.inner.end];
            let blanks = rest.len() - rest.trim_start().len();
            Edit::new(Span::new(start, span.end + blanks), "")
        })
        .collect()
}

/// `source[span]` with up to `cut` bytes of leading blanks taken off every line after the first (or `add` put before
/// it), except lines that start inside a token (a string, a heredoc, inline HTML).
pub fn reindent(source: &str, span: Span, from: &str, to: &str) -> String {
    let text = &source[span.start..span.end];
    if from == to {
        return text.to_string();
    }
    let toks = tokens(source, span.start, span.end);
    // Heredocs and nowdocs, from `<<<` to the closing label: their lines are the string's bytes.
    let mut documents = Vec::new();
    let mut open = None;
    for t in &toks {
        match t.kind {
            TokenKind::DocumentStart(_) => open = Some(t.start),
            TokenKind::DocumentEnd => documents.extend(open.take().map(|s| (s, t.end))),
            _ => {}
        }
    }
    let protected = |pos: usize| {
        documents.iter().any(|&(s, e)| s < pos && pos <= e)
            || toks
                .iter()
                .any(|t| !t.kind.is_trivia() && t.start < pos && pos < t.end)
    };
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    let mut offset = span.start;
    while let Some(i) = rest.find('\n') {
        out.push_str(&rest[..=i]);
        offset += i + 1;
        rest = &rest[i + 1..];
        if protected(offset) || rest.starts_with('\n') {
            continue;
        }
        if let Some(stripped) = rest.strip_prefix(from) {
            out.push_str(to);
            offset += from.len();
            rest = stripped;
        }
    }
    out.push_str(rest);
    out
}

/// The statements of `list` (comments included, the trailing `Nop` too) as one text at the indent `to`, a line each:
/// what Rector's printer writes when the statements move to a list at another depth.
pub fn moved_statements(source: &str, list: &List<'_>, to: &str) -> String {
    let mut parts: Vec<String> = (0..list.stmts.len())
        .map(|i| {
            let span = list.with_comments(source, i);
            reindent(source, span, line_indent(source, span.start), to)
        })
        .collect();
    if let Some(span) = list.nop_span(source) {
        parts.push(reindent(source, span, line_indent(source, span.start), to));
    }
    parts.join(&format!("\n{to}"))
}

/// [`moved_statements`] inserted into an existing list by the format-preserving printer: it writes a moved `Nop`'s
/// comments and then a new line for the empty `Nop`, whose indent `BetterStandardPrinter::pArray` strips.
pub fn inserted_statements(source: &str, list: &List<'_>, to: &str) -> String {
    let mut text = moved_statements(source, list, to);
    if list.nop {
        text.push('\n');
    }
    text
}

/// The parts of an `if` as php-parser has them: `If_` with its `stmts`, its `elseifs` and its `else`. php-parser reads
/// `else if` as an `else` whose only statement is an `if`.
pub struct IfView<'a> {
    pub condition: &'a Expression<'a>,
    pub stmts: List<'a>,
    pub elseifs: Vec<(&'a Expression<'a>, List<'a>, Span)>,
    /// The `else` clause's statements and the clause's span.
    pub else_: Option<(List<'a>, Span)>,
    /// Whether the `if` is written with `:` and `endif;`.
    pub colon: bool,
}

pub fn if_view<'a>(source: &str, r#if: &'a If<'a>) -> IfView<'a> {
    match &r#if.body {
        IfBody::Statement(body) => IfView {
            condition: r#if.condition,
            stmts: body_list(source, body.statement),
            elseifs: body
                .else_if_clauses
                .iter()
                .map(|c| (c.condition, body_list(source, c.statement), node_span(c)))
                .collect(),
            else_: body
                .else_clause
                .as_ref()
                .map(|c| (body_list(source, c.statement), node_span(c))),
            colon: false,
        },
        IfBody::ColonDelimited(body) => {
            let to = body
                .else_if_clauses
                .first()
                .map(|c| start(c.span()))
                .or_else(|| body.else_clause.as_ref().map(|c| start(c.span())))
                .unwrap_or(start(body.endif.span()));
            IfView {
                condition: r#if.condition,
                stmts: open_list(source, &body.statements, end(body.colon), to),
                elseifs: body
                    .else_if_clauses
                    .iter()
                    .map(|c| {
                        let list = open_list(
                            source,
                            &c.statements,
                            end(c.colon),
                            clause_end(source, c.span()),
                        );
                        (c.condition, list, node_span(c))
                    })
                    .collect(),
                else_: body.else_clause.as_ref().map(|c| {
                    let list = open_list(
                        source,
                        &c.statements,
                        end(c.colon),
                        clause_end(source, c.span()),
                    );
                    (list, node_span(c))
                }),
                colon: true,
            }
        }
    }
}

/// The expression of an expression statement, without its parentheses (php-parser has no node for them).
pub fn expression<'a>(statement: &'a Statement<'a>) -> Option<&'a Expression<'a>> {
    match statement {
        Statement::Expression(e) => Some(unparenthesize(e.expression)),
        _ => None,
    }
}

/// php-parser's `Assign` (`=`, not `=&`, which is `AssignRef`, nor a compound assignment).
pub fn assign<'a>(expr: &'a Expression<'a>) -> Option<&'a Assignment<'a>> {
    match unparenthesize(expr) {
        Expression::Assignment(a) if is_plain_assign(a) => Some(a),
        _ => None,
    }
}

/// Whether an assignment is php-parser's `Assign`: `=`, not `=&` nor a compound assignment.
pub fn is_plain_assign(a: &Assignment<'_>) -> bool {
    matches!(a.operator, AssignmentOperator::Assign(_))
        && !matches!(a.rhs, Expression::UnaryPrefix(u) if u.operator.is_reference())
}

/// The name of a plain variable (`$name`, without the `$`), php-parser's `Variable` with a string name.
pub fn variable_name<'a>(expr: &'a Expression<'a>) -> Option<&'a [u8]> {
    match unparenthesize(expr) {
        Expression::Variable(Variable::Direct(v)) => Some(&v.name[1..]),
        _ => None,
    }
}

/// Whether an expression is php-parser's `Exit_` (`exit`, `die`) or `Throw_`.
pub fn is_exit_or_throw(expr: &Expression<'_>) -> bool {
    matches!(
        unparenthesize(expr),
        Expression::Construct(Construct::Exit(_) | Construct::Die(_)) | Expression::Throw(_)
    )
}

/// The tokens of `source[span]` without whitespace, comments included (a comment's lines without their leading
/// blanks, as php-parser's `Comment::getReformattedText` re-indents them): Rector's `BetterStandardPrinter::print`,
/// which prints the comments of the nodes it prints.
fn tokens_with_comments(source: &str, span: Span) -> Vec<(TokenKind, String)> {
    tokens(source, span.start, span.end)
        .into_iter()
        .filter(|t| t.kind != TokenKind::Whitespace)
        .map(|t| {
            let text = &source[t.start..t.end];
            let text = if t.kind.is_comment() {
                text.lines().map(str::trim).collect::<Vec<_>>().join("\n")
            } else {
                text.to_string()
            };
            (t.kind, text)
        })
        .collect()
}

/// Whether two lists print the same with their comments (Rector's `BetterStandardPrinter::print` on two `stmts`).
pub fn same_with_comments(source: &str, a: &List<'_>, b: &List<'_>) -> bool {
    tokens_with_comments(source, printed(source, a))
        == tokens_with_comments(source, printed(source, b))
}

/// The span php-parser prints for a list: the statements with the comments before the first and the trailing `Nop`.
pub fn printed(source: &str, list: &List<'_>) -> Span {
    let start = if list.stmts.is_empty() {
        list.nop_span(source).map_or(list.inner.start, |s| s.start)
    } else {
        list.with_comments(source, 0).start
    };
    let end = list
        .nop_span(source)
        .map(|s| s.end)
        .or_else(|| list.stmts.last().map(|s| node_span(*s).end))
        .unwrap_or(start);
    Span::new(start, end.max(start))
}

/// The list that holds `statement` (the node `ancestors` ends with is its parent), and the statement's index in it.
/// A control structure's single statement is a list of one.
pub fn enclosing<'a>(
    source: &str,
    statement: &'a Statement<'a>,
    ancestors: &[Node<'a, 'a>],
) -> (List<'a>, usize) {
    let n = ancestors.len();
    let list = match ancestors[n - 1] {
        Node::Block(block) => block_list(source, block),
        holder => match holder_list(source, holder, &ancestors[..n - 1]) {
            Some(list) => list,
            None => body_list(source, statement),
        },
    };
    // php-parser's list drops no `if` nor expression statement.
    let this = node_span(statement);
    let index = list
        .stmts
        .iter()
        .position(|s| node_span(*s) == this)
        .unwrap();
    (list, index)
}

/// The edit that puts the statements `text(indent)` builds in place of the statement at `index` of `list` (Rector's
/// `refactor()` returning several nodes). A control structure's single statement cannot hold several, so php-parser
/// prints them in braces there.
pub fn replace_statement(
    source: &str,
    list: &List<'_>,
    index: usize,
    text: impl Fn(&str) -> String,
) -> Edit {
    let span = node_span(list.stmts[index]);
    let indent = line_indent(source, span.start);
    if list.frame == Frame::Bare {
        let inner = format!("{indent}    ");
        return Edit::new(span, format!("{{\n{inner}{}\n{indent}}}", text(&inner)));
    }
    Edit::new(span, text(indent))
}

/// The comments in `[from, to)`, from the start of the first to the end of the last.
pub fn comments(source: &str, from: usize, to: usize) -> Option<Span> {
    let comments: Vec<Token> = tokens(source, from, to)
        .into_iter()
        .filter(|t| t.kind.is_comment())
        .collect();
    Some(Span::new(comments.first()?.start, comments.last()?.end))
}

/// A block as php-parser's pretty printer writes it around `list` at the indent `indent`: `{`, the statements a line
/// each one level deeper, and `}` on a line of its own.
pub fn braced(source: &str, list: &List<'_>, indent: &str) -> String {
    if list.is_empty() {
        return format!("{{\n{indent}}}");
    }
    let inner = format!("{indent}    ");
    format!(
        "{{\n{inner}{}\n{indent}}}",
        moved_statements(source, list, &inner)
    )
}

/// Whether `node` or a node under it is a variable that prints as `target` (`BetterNodeFinder::findFirst` with
/// `NodeComparator::areNodesEqual` against a variable): a variable anywhere, in a closure, an interpolated string or a
/// parameter too.
pub fn has_equal_variable(source: &str, node: Node<'_, '_>, target: Span) -> bool {
    if matches!(
        node,
        Node::DirectVariable(_) | Node::IndirectVariable(_) | Node::NestedVariable(_)
    ) && crate::syntax::same_code(source, node_span(&node), target)
    {
        return true;
    }
    let mut found = false;
    node.visit_children(|child| found = found || has_equal_variable(source, child, target));
    found
}

/// `VariableAnalyzer::isStaticOrGlobal` for the variable `name` inside the statement `chain` ends with (as a
/// `Node::Statement`): Rector's `GlobalVariableNodeVisitor` and `StaticVariableNodeVisitor` mark every variable of
/// that name in the statements after a `global` or `static` statement of the same list, at any depth but not inside
/// a class.
pub fn is_static_or_global(chain: &[Node<'_, '_>], name: &[u8]) -> bool {
    for j in 0..chain.len().saturating_sub(1) {
        // A holder's child is a statement.
        let (holder, Node::Statement(statement)) = (chain[j], chain[j + 1]) else {
            continue;
        };
        if !HOLDERS.contains(&holder.kind())
            || chain[j + 1..]
                .iter()
                .any(|n| matches!(n, Node::Class(_) | Node::AnonymousClass(_)))
        {
            continue;
        }
        let this = node_span(statement);
        let before = crate::syntax::statements(holder);
        let declared = before
            .iter()
            .take_while(|s| node_span(**s) != this)
            .any(|s| match s {
                Statement::Global(g) => g
                    .variables
                    .iter()
                    .any(|v| matches!(v, Variable::Direct(d) if &d.name[1..] == name)),
                Statement::Static(s) => s.items.iter().any(|i| &i.variable().name[1..] == name),
                _ => false,
            });
        if declared {
            return true;
        }
    }
    false
}

/// `VariableAnalyzer::isUsedByReference` for a variable `name` at `at` with these ancestors: Rector's
/// `ByRefVariableNodeVisitor` marks, in each function-like, every variable named like one of its parameters taken by
/// reference, a variable it uses by reference (a closure), or a variable a closure met earlier in its body uses by
/// reference.
pub fn is_used_by_reference(ancestors: &[Node<'_, '_>], name: &[u8], at: usize) -> bool {
    let mut outermost = None;
    for node in ancestors {
        let (params, uses) = match node {
            Node::Function(f) => (Some(&f.parameter_list), None),
            Node::Method(m) => (Some(&m.parameter_list), None),
            Node::Closure(c) => (Some(&c.parameter_list), c.use_clause.as_ref()),
            Node::ArrowFunction(a) => (Some(&a.parameter_list), None),
            _ => continue,
        };
        outermost.get_or_insert(*node);
        let by_ref_param = params.is_some_and(|p| {
            p.parameters
                .iter()
                .any(|p| p.ampersand.is_some() && &p.variable.name[1..] == name)
        });
        if by_ref_param || uses.is_some_and(|u| by_ref_use(u, name)) {
            return true;
        }
    }
    // The closures the outermost function-like's walk meets before the variable.
    fn closures_before(node: Node<'_, '_>, name: &[u8], at: usize) -> bool {
        if node_span(&node).start >= at {
            return false;
        }
        if let Node::Closure(c) = node
            && c.use_clause.as_ref().is_some_and(|u| by_ref_use(u, name))
        {
            return true;
        }
        let mut found = false;
        node.visit_children(|child| found = found || closures_before(child, name, at));
        found
    }
    let Some(outermost) = outermost else {
        return false;
    };
    let mut found = false;
    outermost.visit_children(|child| found = found || closures_before(child, name, at));
    found
}

fn by_ref_use(clause: &ClosureUseClause<'_>, name: &[u8]) -> bool {
    clause
        .variables
        .iter()
        .any(|v| v.ampersand.is_some() && &v.variable.name[1..] == name)
}

/// Whether the doc comment Rector reads for a statement (the last `/** */` among the comments before it) has a `@var`
/// tag with a type: `PhpDocInfo::getVarTagValueNode()`.
pub fn has_var_tag(source: &str, comments_start: usize, statement_start: usize) -> bool {
    let Some(doc) = tokens(source, comments_start, statement_start)
        .into_iter()
        .filter(|t| t.kind == TokenKind::DocBlockComment)
        .last()
    else {
        return false;
    };
    source[doc.start..doc.end].lines().any(|line| {
        let line = line
            .trim_start()
            .trim_start_matches("/**")
            .trim_start_matches('*')
            .trim_start()
            .trim_end_matches("*/");
        let Some(rest) = line.strip_prefix("@var") else {
            return false;
        };
        let Some(first) = rest.chars().next() else {
            return false;
        };
        let value = rest.trim_start();
        first.is_whitespace()
            && !value.is_empty()
            && (!value.starts_with('$') || value.starts_with("$this"))
    })
}

/// Whether `node` or a node under it is a binary operation `op` matches (`BetterNodeFinder::findInstanceOf`).
pub fn has_binary(node: Node<'_, '_>, op: &dyn Fn(&BinaryOperator<'_>) -> bool) -> bool {
    if let Node::Binary(b) = node
        && op(&b.operator)
    {
        return true;
    }
    let mut found = false;
    node.visit_children(|child| found = found || has_binary(child, op));
    found
}

/// `matchComparedAndDesiredValues` of `RepeatedOrEqualToInArrayRector` and `RepeatedAndNotEqualToNotInArrayRector`:
/// the comparisons a chain of `chain` operators joins, left to right, as (compared, value) pairs, or `None` when an
/// operand is not a comparison `compare` matches.
pub fn compared_values<'a>(
    binary: &'a Binary<'a>,
    chain: &dyn Fn(&BinaryOperator<'_>) -> bool,
    compare: &dyn Fn(&BinaryOperator<'_>) -> bool,
) -> Option<Vec<(&'a Expression<'a>, &'a Expression<'a>)>> {
    let comparison = |e: &'a Expression<'a>| match unparenthesize(e) {
        Expression::Binary(b) if compare(&b.operator) => Some((b.lhs, b.rhs)),
        _ => None,
    };
    let mut out = vec![comparison(binary.rhs)?];
    let mut current = binary;
    while let Expression::Binary(left) = unparenthesize(current.lhs)
        && chain(&left.operator)
    {
        out.push(comparison(left.rhs)?);
        current = left;
    }
    out.push(comparison(current.lhs)?);
    out.reverse();
    Some(out)
}
