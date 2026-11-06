//! `lint/complexity/no-redundant-null-check`, ported from Rector 2.6.1's `SimplifyIfNotNullReturnRector`
//! (rules/CodeQuality/Rector/If_) with `IfManipulator::matchIfNotNullReturnValue`.
//!
//! ```php
//! if ($value !== null) {
//!     return $value;
//! }
//!
//! return null;
//! ```
//!
//! becomes `return $value;`: returning the value when it is not null and null otherwise returns the value. The `if`
//! (one `return` in it, no `elseif` or `else`) and the `return null;` right after it make way for the inner `return`,
//! moved to the `if`'s indent with the comments before it. The comments before the `if` and between the two go, as
//! php-parser's printer drops them with the removed statements.

use mago_syntax::cst::{Node, NodeKind, Statement};
use pun_problem::{Edit, Fix, Safety, Span};

use super::no_redundant_readonly_property::print_modifiers;
use super::php_parser::{Token, enclosing, previous_token_end, tokens};
use super::rector_code_quality::is_constant;
use super::rector_conditions::{
    comments_between, if_not_null_return_value, if_parts, indentation, is_stmts_aware,
    print_comments, reprint,
};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoRedundantNullCheck;

impl Rule for NoRedundantNullCheck {
    fn id(&self) -> &'static str {
        "lint/complexity/no-redundant-null-check"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::If])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::If(r#if) = node else { return };
        let n = ancestors.len();
        if !is_stmts_aware(ancestors[n - 2], &ancestors[..n - 2]) {
            return;
        }
        let source = ctx.source;
        let tokens = tokens(ctx);
        let parts = if_parts(&tokens, r#if);
        if parts.has_else || parts.has_elseifs {
            return;
        }
        let (list, index) = enclosing(source, &tokens, ancestors);
        let Some(Statement::Return(next)) = list.stmts.get(index + 1).copied() else {
            return;
        };
        if if_not_null_return_value(ctx, r#if, &parts.stmts).is_none() {
            return;
        }
        if !next.value.is_some_and(|v| is_constant(v, "null")) {
            return;
        }
        let inner = node_span(parts.stmts.stmts[0]);
        let if_span = node_span(r#if);
        let level = indentation(source, if_span.start);
        let nl = format!("\n{}", " ".repeat(level));
        // The inner `return` with the comments php-parser keeps with it, at the `if`'s indent.
        let before_inner = previous_token_end(source, &tokens, inner.start);
        let comments = comments_between(&tokens, before_inner, inner.start);
        let adjust = level as isize - indentation(source, inner.start) as isize;
        let mut code = reprint(source, &tokens, inner, adjust);
        if !comments.is_empty() {
            code = format!("{}{nl}{code}", print_comments(source, &comments, &nl));
        }
        let end = node_span(next).end;
        let edit = if index > 0 {
            // Both statements removed and the inner one inserted after the statement before them.
            let before = previous_token_end(source, &tokens, if_span.start).max(list.inner.start);
            Edit::new(Span::new(before, end), format!("{nl}{code}"))
        } else {
            // Removed from the start of the list, with the comments before the `if`: the inner statement goes before
            // the next one, which follows on its own line.
            let start = comments_between(&tokens, list.inner.start, if_span.start)
                .first()
                .map_or(if_span.start, |c| c.start);
            if list.stmts.len() == 2 && !list.nop {
                // Nothing left in the list to insert before: php-parser prints the function holding it anew.
                if let Some(edit) = function_like_reprint(source, &tokens, ancestors, inner) {
                    out.push(report(if_span, edit));
                    return;
                }
            }
            match list.stmts.get(index + 2) {
                Some(after) => {
                    let after = node_span(*after).start;
                    let after = comments_between(&tokens, end, after)
                        .first()
                        .map_or(after, |c| c.start);
                    Edit::new(Span::new(start, after), format!("{code}{nl}"))
                }
                None => Edit::new(Span::new(start, end), code),
            }
        };
        out.push(report(if_span, edit));
    }
}

fn report(span: Span, edit: Edit) -> Report {
    Report {
        span,
        message: "Return the value directly; the null check returns null for null anyway."
            .to_string(),
        help: None,
        fix: Some(Fix {
            safety: Safety::Safe,
            edits: vec![edit],
        }),
    }
}

/// The function, method or closure whose body is the list, printed as php-parser's `pStmt_ClassMethod`,
/// `pStmt_Function` and `pExpr_Closure` print it with `inner` (the `return` that stays, with its comments) as its only
/// statement: see [`reprint_function_like`].
fn function_like_reprint(
    source: &str,
    tokens: &[Token],
    ancestors: &[Node<'_, '_>],
    inner: Span,
) -> Option<Edit> {
    let n = ancestors.len();
    let holder = match ancestors[n.checked_sub(3)?] {
        Node::MethodBody(_) => ancestors[n - 4],
        other => other,
    };
    reprint_function_like(source, tokens, holder, &|level| {
        let nl_in = format!("\n{}", " ".repeat(level));
        let mut code = nl_in.clone();
        let before = previous_token_end(source, tokens, inner.start);
        let comments = comments_between(tokens, before, inner.start);
        if !comments.is_empty() {
            code.push_str(&print_comments(source, &comments, &nl_in));
            code.push_str(&nl_in);
        }
        code.push_str(&reprint(
            source,
            tokens,
            inner,
            level as isize - indentation(source, inner.start) as isize,
        ));
        code
    })
}

/// The function, method or closure `holder` printed anew, as php-parser's `pStmt_ClassMethod`, `pStmt_Function` and
/// `pExpr_Closure` print it when its statement list cannot keep its layout: attributes and modifiers in their order,
/// the parameters on one line (one per line when one of them has comments), and the body at its own indent, with the
/// statements `body` prints at the level it is given (each after a line break). Anything else gets no new layout.
pub(crate) fn reprint_function_like(
    source: &str,
    tokens: &[Token],
    holder: Node<'_, '_>,
    body: &dyn Fn(usize) -> String,
) -> Option<Edit> {
    let (span, attributes, inline, head, parameters, uses, return_type, new_line) = match holder {
        Node::Method(m) => {
            let reference = if m.ampersand.is_some() { "&" } else { "" };
            let head = format!(
                "{}function {reference}{}",
                print_modifiers(&m.modifiers, false),
                text(source, &m.name)
            );
            (
                node_span(m),
                &m.attribute_lists,
                false,
                head,
                &m.parameter_list,
                None,
                &m.return_type_hint,
                true,
            )
        }
        Node::Function(f) => {
            let reference = if f.ampersand.is_some() { "&" } else { "" };
            let head = format!("function {reference}{}", text(source, &f.name));
            (
                node_span(f),
                &f.attribute_lists,
                false,
                head,
                &f.parameter_list,
                None,
                &f.return_type_hint,
                true,
            )
        }
        Node::Closure(c) => {
            let head = format!(
                "{}function {}",
                if c.r#static.is_some() { "static " } else { "" },
                if c.ampersand.is_some() { "&" } else { "" }
            );
            let uses = c.use_clause.as_ref();
            (
                node_span(c),
                &c.attribute_lists,
                true,
                head,
                &c.parameter_list,
                uses,
                &c.return_type_hint,
                false,
            )
        }
        _ => return None,
    };
    let level = indentation(source, span.start);
    let nl = format!("\n{}", " ".repeat(level));
    let nl_in = format!("\n{}", " ".repeat(level + 4));
    let moved = |s: Span, to: usize| {
        reprint(
            source,
            tokens,
            s,
            to as isize - indentation(source, s.start) as isize,
        )
    };
    let mut code = String::new();
    for list in attributes.iter() {
        code.push_str(text(source, list));
        code.push_str(if inline { " " } else { &nl });
    }
    code.push_str(&head);
    // pParams: one line, or one parameter per line when one has comments, without a trailing comma (the printer
    // targets PHP 7.4).
    let parameters: Vec<(Vec<Span>, Span)> = parameters
        .parameters
        .iter()
        .map(|p| {
            let s = node_span(p);
            let before = previous_token_end(source, tokens, s.start);
            (comments_between(tokens, before, s.start), s)
        })
        .collect();
    code.push('(');
    if parameters.iter().all(|(c, _)| c.is_empty()) {
        let printed: Vec<String> = parameters.iter().map(|(_, s)| moved(*s, level)).collect();
        code.push_str(&printed.join(", "));
    } else {
        let printed: Vec<String> = parameters
            .iter()
            .map(|(comments, s)| {
                let mut p = String::new();
                if !comments.is_empty() {
                    p.push_str(&nl_in);
                    p.push_str(&print_comments(source, comments, &nl_in));
                }
                p.push_str(&nl_in);
                p.push_str(&moved(*s, level + 4));
                p
            })
            .collect();
        code.push_str(&printed.join(","));
        code.push_str(&nl);
    }
    code.push(')');
    if let Some(uses) = uses {
        let printed: Vec<&str> = uses.variables.iter().map(|v| text(source, v)).collect();
        code.push_str(&format!(" use ({})", printed.join(", ")));
    }
    if let Some(r) = return_type {
        code.push_str(&format!(": {}", text(source, &r.hint)));
    }
    code.push_str(if new_line { &nl } else { " " });
    code.push('{');
    code.push_str(&body(level + 4));
    code.push_str(&nl);
    code.push('}');
    Some(Edit::new(span, code))
}

fn text<'s>(source: &'s str, node: &impl mago_span::HasSpan) -> &'s str {
    let s = node_span(node);
    &source[s.start..s.end]
}
