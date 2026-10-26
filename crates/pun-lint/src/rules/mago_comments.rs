//! Report rules on comments, ported from Mago 1.57.0's linter (mago-linter src/rule/clarity/):
//!
//! - `lint/style/require-todo-reference` (`tagged-todo`, tagged_todo.rs, and `tagged-fixme`, tagged_fixme.rs): a
//!   comment with a line that starts with `TODO` (or `FIXME`), in any case, but has no `todo(` (`fixme(`) followed
//!   by a non-space character anywhere on that line. One report per comment and tag; on one comment the FIXME
//!   report comes first, as in Mago.
//! - `lint/style/no-invalid-phpdoc` (`valid-docblock`, valid_docblock.rs): each syntax error mago-phpdoc-syntax finds
//!   in a docblock.

use mago_phpdoc_syntax::PHPDocParser;
use mago_span::HasSpan;
use mago_syntax::comments::comment_lines;
use mago_syntax::cst::{Node, NodeKind, Trivia, TriviaKind};

use crate::syntax::span;
use crate::{Context, Report, Rule};

pub struct RequireTodoReference;

impl Rule for RequireTodoReference {
    fn id(&self) -> &'static str {
        "lint/style/require-todo-reference"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Program])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Program(program) = node else { return };
        for (tag, message, help) in [
            (
                "fixme",
                "FIXME comment should be tagged with (@username) or (#issue).",
                "Add a user tag or issue reference to the FIXME comment, e.g. FIXME(@azjezz), FIXME(azjezz), FIXME(#123).",
            ),
            (
                "todo",
                "TODO should be tagged with (@username) or (#issue).",
                "Add a user tag or issue reference to the TODO comment, e.g. TODO(@azjezz), TODO(azjezz), TODO(#123).",
            ),
        ] {
            for trivia in program.trivia.iter().filter(|t| t.kind.is_comment()) {
                if is_untagged(trivia, tag) {
                    out.push(Report {
                        span: span(trivia.span),
                        message: message.into(),
                        help: Some(help.into()),
                        fix: None,
                    });
                }
            }
        }
    }
}

/// Whether a line of the comment starts with `tag` and is not tagged.
fn is_untagged(trivia: &Trivia<'_>, tag: &str) -> bool {
    comment_lines(trivia).into_iter().any(|(_, line)| {
        let start = line
            .iter()
            .position(|b| !b.is_ascii_whitespace())
            .unwrap_or(line.len());
        let stripped = &line[start..];
        if !stripped
            .get(..tag.len())
            .is_some_and(|p| p.eq_ignore_ascii_case(tag.as_bytes()))
        {
            return false;
        }
        let Ok(lower) = String::from_utf8(stripped.to_ascii_lowercase()) else {
            return false;
        };
        !is_tagged(&lower, tag)
    })
}

/// The regex `<tag>\((#|@)?\S+`: `<tag>(` followed by a non-whitespace character.
fn is_tagged(line: &str, tag: &str) -> bool {
    let open = format!("{tag}(");
    line.match_indices(&open).any(|(i, _)| {
        line[i + open.len()..]
            .chars()
            .next()
            .is_some_and(|c| !c.is_whitespace())
    })
}

pub struct NoInvalidPhpdoc;

impl Rule for NoInvalidPhpdoc {
    fn id(&self) -> &'static str {
        "lint/style/no-invalid-phpdoc"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Program])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Program(program) = node else { return };
        let arena = mago_allocator::LocalArena::new();
        for trivia in program
            .trivia
            .iter()
            .filter(|t| t.kind == TriviaKind::DocBlockComment)
        {
            let document = PHPDocParser::parse_without_trivia(&arena, trivia.value, trivia.span);
            for error in document.errors {
                out.push(Report {
                    span: span(error.span()),
                    message: error.to_string(),
                    help: Some(error.help().into()),
                    fix: None,
                });
            }
        }
    }
}
