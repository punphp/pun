//! `lint/complexity/no-single-case-switch`, ported from Rector 2.8.4's `SingularSwitchToIfRector`
//! (rules/CodeQuality/Rector/Switch_) with `SwitchManipulator::removeBreakNodes`.
//!
//! A `switch` with one `case` becomes an `if` that compares with `===`, its `break`s dropped (`break 2` becomes
//! `break`). A `switch` with only `default` becomes the default's statements without any `break`. A `break` or `continue` nested in
//! the case's statements keeps the `switch`. Rector stops with an error on a `default` with no statement left; Pun
//! leaves that `switch` alone.
//!
//! The new code is laid out as php-parser's format-preserving printer lays it out: the new `if` printed anew with
//! the statements moved to its indent, and the comments before the `switch` dropped; the statements of a `default`
//! put in the `switch`'s place in the list, each on a line of its own.

use mago_syntax::cst::{Expression, Literal, Node, NodeKind, Statement, SwitchBody, SwitchCase};
use pun_problem::{Edit, Fix, Safety, Span};

use super::php_parser::{Frame, Token, enclosing, holder_list, previous_token_end, tokens};
use super::rector_code_quality::infix;
use super::rector_conditions::{comments_between, indentation, print_comments, reprint};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoSingleCaseSwitch;

impl Rule for NoSingleCaseSwitch {
    fn id(&self) -> &'static str {
        "lint/complexity/no-single-case-switch"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Switch])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Switch(switch) = node else { return };
        let cases = match &switch.body {
            SwitchBody::BraceDelimited(b) => &b.cases,
            SwitchBody::ColonDelimited(b) => &b.cases,
        };
        let [case] = cases.as_slice() else { return };
        // nested break/continue would lose the switch to target and cause a fatal error
        if has_nested_break_or_continue(case) {
            return;
        }
        let source = ctx.source;
        let tokens = tokens(ctx);
        let (case_node, condition) = match case {
            SwitchCase::Expression(c) => (Node::SwitchExpressionCase(c), Some(c.expression)),
            SwitchCase::Default(c) => (Node::SwitchDefaultCase(c), None),
        };
        let items = items(source, &tokens, case_node, condition.is_some());
        let span = node_span(switch);
        // The comments php-parser keeps with the `switch`: from the token before it.
        let before = previous_token_end(source, &tokens, span.start);
        let start = comments_between(&tokens, before, span.start)
            .first()
            .map_or(span.start, |c| c.start);
        let outer = indentation(source, span.start);
        let edit = match condition {
            Some(condition) => {
                let inner = outer + 4;
                let nl = format!("\n{}", " ".repeat(inner));
                let body: String = items
                    .iter()
                    .map(|item| format!("{nl}{}", item.print(source, &tokens, inner)))
                    .collect();
                let cond = infix(ctx, switch.expression, "===", condition, (80, 80));
                let code = format!("if ({cond}) {{{body}\n{}}}", " ".repeat(outer));
                Edit::new(Span::new(start, span.end), code)
            }
            None => {
                if items.is_empty() {
                    return;
                }
                default_edit(source, &tokens, ancestors, &items, span, start, outer)
            }
        };
        out.push(Report {
            span,
            message: "A `switch` with a single case is an `if`.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![edit],
            }),
        });
    }
}

/// `hasNestedBreakOrContinue`: a `break` or `continue` below the case's statements (a top-level `break` goes with the
/// `switch`), outside nested functions and classes.
fn has_nested_break_or_continue(case: &SwitchCase<'_>) -> bool {
    fn find(node: Node<'_, '_>) -> bool {
        let mut found = false;
        node.visit_children(|child| {
            found = found
                || match child {
                    Node::Function(_)
                    | Node::Method(_)
                    | Node::Closure(_)
                    | Node::ArrowFunction(_)
                    | Node::Class(_)
                    | Node::AnonymousClass(_) => false,
                    Node::Break(_) | Node::Continue(_) => true,
                    _ => find(child),
                }
        });
        found
    }
    let statements = match case {
        SwitchCase::Expression(c) => &c.statements,
        SwitchCase::Default(c) => &c.statements,
    };
    statements.iter().any(|s| match s {
        Statement::Break(_) => false,
        Statement::Continue(_) => true,
        s => find(Node::Statement(s)),
    })
}

/// One statement of the new code: its comments and its text. Comments after the last statement go, as in Rector.
struct Item {
    comments: Vec<Span>,
    code: Code,
}

enum Code {
    /// A statement as it was.
    Statement(Span),
    /// A `break` with a lower level.
    Break(String),
}

impl Item {
    /// The item at indent `level`: `pComments` and a newline, then the statement moved to that indent.
    fn print(&self, source: &str, tokens: &[Token], level: usize) -> String {
        let nl = format!("\n{}", " ".repeat(level));
        let comments = print_comments(source, &self.comments, &nl);
        let code = match &self.code {
            Code::Statement(s) => {
                let adjust = level as isize - indentation(source, s.start) as isize;
                reprint(source, tokens, *s, adjust)
            }
            Code::Break(b) => b.clone(),
        };
        if self.comments.is_empty() {
            code
        } else {
            format!("{comments}{nl}{code}")
        }
    }
}

/// The statements of the case, without the `break`s `removeBreakNodes` (with a condition) or `array_filter` (for a
/// `default`) drops.
fn items(source: &str, tokens: &[Token], case: Node<'_, '_>, with_condition: bool) -> Vec<Item> {
    let list = holder_list(source, tokens, case);
    let mut items = Vec::new();
    let mut previous = list.inner.start;
    for statement in &list.stmts {
        let span = node_span(*statement);
        let comments = comments_between(tokens, previous, span.start);
        previous = span.end;
        let code = match statement {
            Statement::Break(_) if !with_condition => continue,
            Statement::Break(b) => match b.level.map(unparenthesize) {
                Some(Expression::Literal(Literal::Integer(i)))
                    if i.value.is_some_and(|v| v > 1) =>
                {
                    let level = i.value.expect("checked") - 1;
                    Code::Break(if level == 1 {
                        "break;".to_string()
                    } else {
                        format!("break {level};")
                    })
                }
                _ => continue,
            },
            _ => Code::Statement(span),
        };
        items.push(Item { comments, code });
    }
    items
}

/// The edit for a `default` alone: its statements in the `switch`'s place, as php-parser's `pArray` prints a list
/// item replaced by one (a replacement) or by several (a removal and insertions).
fn default_edit(
    source: &str,
    tokens: &[Token],
    ancestors: &[Node<'_, '_>],
    items: &[Item],
    span: Span,
    start: usize,
    outer: usize,
) -> Edit {
    let nl = format!("\n{}", " ".repeat(outer));
    if let [item] = items {
        return Edit::new(
            Span::new(start, span.end),
            item.print(source, tokens, outer),
        );
    }
    let (list, index) = enclosing(source, tokens, ancestors);
    if list.frame == Frame::Bare {
        // php-parser prints the parent again with braces.
        let inner = format!("\n{}", " ".repeat(outer + 4));
        let body: String = items
            .iter()
            .map(|item| format!("{inner}{}", item.print(source, tokens, outer + 4)))
            .collect();
        return Edit::new(span, format!("{{{body}{nl}}}"));
    }
    if index > 0 {
        let before = previous_token_end(source, tokens, start).max(list.inner.start);
        let code: String = items
            .iter()
            .map(|item| format!("{nl}{}", item.print(source, tokens, outer)))
            .collect();
        return Edit::new(Span::new(before, span.end), code);
    }
    // Removed from the start of the list: the statements go before the next one, each followed by a newline.
    let next = list.stmts.get(index + 1).map(|s| {
        let s = node_span(*s);
        comments_between(tokens, span.end, s.start)
            .first()
            .map_or(s.start, |c| c.start)
    });
    let code: Vec<String> = items
        .iter()
        .map(|item| item.print(source, tokens, outer))
        .collect();
    match next {
        Some(next) => Edit::new(Span::new(start, next), format!("{}{nl}", code.join(&nl))),
        None => Edit::new(Span::new(start, span.end), code.join(&nl)),
    }
}
