//! `lint/phpunit/require-strict-types-in-tests`, ported from rector-phpunit's `DeclareStrictTypesTestsRector`
//! (rules/CodeQuality/Rector/StmtsAwareInterface).
//!
//! A file without a top-level `declare(strict_types=...)` (`DeclareStrictTypeFinder`, which compares the key's case)
//! whose first named class (`BetterNodeFinder::findFirstNonAnonymousClass`, depth first) is a test class gets
//! `declare(strict_types=1);` and an empty statement ahead of its statements. A file with inline HTML is left alone
//! (`HTMLAverseRectorInterface`).
//!
//! php-parser's format-preserving printer writes the two statements at the first statement's comments, followed by
//! a blank line. A file of one statement falls back to printing the file anew: `<?php`, a blank line, the
//! declaration, a blank line, then the statement's comments and the statement, and nothing after it.

use mago_syntax::cst::{Namespace, NamespaceBody, Node, NodeKind, Sequence, Statement};
use pun_problem::{Edit, Safety, Span};

use super::{is_in_test_class, report};
use crate::rules::lint_b_helpers::contains_html;
use crate::rules::rector_stmts::{open_list, printed, tokens, trailing_comment};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

const DECLARE: &str = "declare(strict_types=1);";

pub struct RequireStrictTypesInTests;

impl Rule for RequireStrictTypesInTests {
    fn id(&self) -> &'static str {
        "lint/phpunit/require-strict-types-in-tests"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Program])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Program(program) = node else { return };
        let has_strict_types = program.statements.iter().any(|statement| {
            matches!(statement, Statement::Declare(d) if d.items.iter().any(|i| i.name.value == b"strict_types"))
        });
        if has_strict_types || contains_html(node, ancestors) || !first_class_is_test(ctx, node) {
            return;
        }
        let source = ctx.source;
        let list = open_list(source, &program.statements, 0, source.len());
        let first = list.with_comments(source, 0);
        let last_end = list.stmts.last().map_or(0, |s| node_span(*s).end);
        // php-parser moves the statements after a namespace without braces into it, a trailing `Nop` too
        let in_namespace = matches!(
            list.stmts.last(),
            Some(Statement::Namespace(Namespace {
                body: NamespaceBody::Implicit(_),
                ..
            }))
        );
        let nop = list.nop || closing_nop(source, &program.statements, last_end).is_some();
        let edit = if list.stmts.len() + usize::from(nop && !in_namespace) == 1 {
            let mut text = format!("<?php\n\n{DECLARE}\n");
            let start = node_span(list.stmts[0]).start;
            for comment in tokens(source, list.before(source, 0), start)
                .into_iter()
                .filter(|t| t.kind.is_comment())
            {
                text.push('\n');
                text.push_str(&source[comment.start..comment.end]);
            }
            // php-parser's namespace without braces ends with its last statement (a `?>` after it is not one)
            let end = match list.stmts[0] {
                Statement::Namespace(Namespace {
                    body: NamespaceBody::Implicit(body),
                    ..
                }) => {
                    let terminator = node_span(&body.terminator).end;
                    let inner = open_list(source, &body.statements, terminator, source.len());
                    let end = printed(source, &inner).end.max(terminator);
                    closing_nop(source, &body.statements, end).unwrap_or(end)
                }
                statement => node_span(statement).end,
            };
            text.push('\n');
            text.push_str(&source[start..end]);
            text.push('\n');
            Edit::new(Span::new(0, source.len()), text)
        } else {
            Edit::new(
                Span::new(first.start, first.start),
                format!("{DECLARE}\n\n"),
            )
        };
        out.push(report(
            Span::new(first.start, first.start),
            "Declare strict types in a test file.",
            Safety::Unsafe,
            vec![edit],
        ));
    }
}

/// The end of a `?>` among `statements` after `after` with comments before it: php-parser reads the tag as `;`, and
/// the comments make that an empty statement (`Nop`) ending with the tag.
fn closing_nop(
    source: &str,
    statements: &Sequence<'_, Statement<'_>>,
    after: usize,
) -> Option<usize> {
    statements
        .iter()
        .find_map(|s| match s {
            Statement::ClosingTag(tag) if node_span(tag).start >= after => Some(node_span(tag)),
            _ => None,
        })
        .filter(|tag| trailing_comment(source, after, tag.start))
        .map(|tag| tag.end)
}

/// Whether the first class declaration (depth first, not an anonymous class) under `program` is a test class. A
/// class declared inside a function has no class reflection in Rector's scope, so it is not one.
fn first_class_is_test(ctx: &Context<'_>, program: Node<'_, '_>) -> bool {
    let mut stack = vec![(program, 0usize)];
    let mut path: Vec<Node> = Vec::new();
    while let Some((node, depth)) = stack.pop() {
        path.truncate(depth);
        if matches!(node, Node::Class(_)) {
            let in_function = path.iter().any(|a| {
                matches!(
                    a,
                    Node::Function(_) | Node::Method(_) | Node::Closure(_) | Node::ArrowFunction(_)
                )
            });
            path.push(node);
            return !in_function && is_in_test_class(ctx, &path, None);
        }
        path.push(node);
        let mut children = Vec::new();
        node.visit_children(|child| children.push(child));
        stack.extend(children.into_iter().rev().map(|c| (c, depth + 1)));
    }
    false
}
