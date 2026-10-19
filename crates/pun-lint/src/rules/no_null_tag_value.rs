//! `lint/dead-code/no-null-tag-value`, ported from Rector 2.6.1's `RemoveNullTagValueNodeRector`
//! (rules/DeadCode/Rector/ClassMethod).
//!
//! A doc comment's `@param null $name` (of a parameter the function or method has), `@return null` and `@var null`
//! (on a property or an expression statement), with no description, go. Every `@param`-like tag (`@phpstan-param`,
//! ...) of such a parameter goes with it; `@return` and `@var` take their `@phan-` variants along, not the
//! `@phpstan-` and `@psalm-` ones (`PhpDocInfo::removeByType`). A doc comment left empty goes.

use mago_syntax::cst::{ClassLikeMember, Node, NodeKind, Statement};
use pun_problem::{Edit, Fix, Safety, Span};

use mago_syntax::token::TokenKind;

use super::rector_batch5::doc_without_tags;
use super::rector_stmts::{previous_token_end, tokens};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoNullTagValue;

impl Rule for NoNullTagValue {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-null-tag-value"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::Function,
            NodeKind::Method,
            NodeKind::Statement,
            NodeKind::ClassLikeMember,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let params = match node {
            Node::Function(f) => Some(&f.parameter_list),
            Node::Method(m) => Some(&m.parameter_list),
            Node::Statement(Statement::Expression(_))
            | Node::ClassLikeMember(ClassLikeMember::Property(_)) => None,
            _ => return,
        };
        let span = node_span(&node);
        let from = ancestors
            .iter()
            .rev()
            .map(|a| node_span(a).start)
            .find(|start| *start < span.start)
            .unwrap_or(0);
        // php-parser's `getDocComment`: the last doc comment before the node, comments after it or not.
        let Some(doc) = tokens(
            ctx.source,
            previous_token_end(ctx.source, from, span.start),
            span.start,
        )
        .into_iter()
        .rfind(|t| t.kind == TokenKind::DocBlockComment)
        .map(|t| Span::new(t.start, t.end)) else {
            return;
        };
        let text = &ctx.source[doc.start..doc.end];
        let tags = std::cell::RefCell::new(Vec::new());
        doc_without_tags(text, |tag| {
            tags.borrow_mut().push(tag.to_string());
            false
        });
        let tags = tags.into_inner();
        let removed = match params {
            Some(params) => {
                // `getParamTagValueByName`: the first `@param` of each parameter.
                let names: Vec<&str> = params
                    .parameters
                    .iter()
                    .filter_map(|p| {
                        let name = std::str::from_utf8(p.variable.name).ok()?;
                        let first = tags
                            .iter()
                            .find_map(|t| param(t, "@param").filter(|(n, _)| *n == name))?;
                        first.1.then_some(name)
                    })
                    .collect();
                // `getReturnTagValue`: the first `@return`.
                let returns_null = tags
                    .iter()
                    .find_map(|t| value(t, &["@return"]))
                    .is_some_and(is_null);
                doc_without_tags(text, |tag| {
                    ["@param", "@phpstan-param", "@psalm-param", "@phan-param"]
                        .iter()
                        .any(|kind| param(tag, kind).is_some_and(|(n, _)| names.contains(&n)))
                        || (returns_null
                            && value(tag, &["@return", "@phan-return", "@phan-real-return"])
                                .is_some())
                })
            }
            None => {
                // `getVarTagValueNode`: the first `@var`.
                let var_null = tags
                    .iter()
                    .find_map(|t| value(t, &["@var"]))
                    .is_some_and(|v| is_null(strip_variable(v)));
                if !var_null {
                    return;
                }
                doc_without_tags(text, |tag| value(tag, &["@var", "@phan-var"]).is_some())
            }
        };
        let Some(new) = removed else {
            return;
        };
        let edit = if new.is_empty() {
            let blank = ctx.source[doc.end..].len() - ctx.source[doc.end..].trim_start().len();
            Edit::new(Span::new(doc.start, doc.end + blank), "")
        } else {
            Edit::new(doc, new)
        };
        out.push(Report {
            span: doc,
            message: "Remove the `null` tag, which tells nothing the code does not.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![edit],
            }),
        });
    }
}

/// What follows the tag `tag` names, when it is one of `kinds` and has a value (php-parser's tag value node).
fn value<'a>(tag: &'a str, kinds: &[&str]) -> Option<&'a str> {
    let (name, rest) = tag.split_once(char::is_whitespace)?;
    let rest = rest.trim();
    (kinds.contains(&name) && !rest.is_empty()).then_some(rest)
}

/// The parameter a `kind` tag names (without `&` or `...`, with its `$`), and whether its type is `null` without a
/// description.
fn param<'a>(tag: &'a str, kind: &str) -> Option<(&'a str, bool)> {
    let rest = value(tag, &[kind])?;
    let (ty, rest) = rest.split_once(char::is_whitespace)?;
    let rest = rest.trim_start();
    let variable = rest.trim_start_matches('&').trim_start_matches("...");
    let end = variable.find(char::is_whitespace).unwrap_or(variable.len());
    let (name, description) = variable.split_at(end);
    (name.len() > 1 && name.starts_with('$'))
        .then_some((name, ty == "null" && description.trim().is_empty()))
}

/// A `@var` value without the variable it names.
fn strip_variable(value: &str) -> &str {
    match value.split_once(char::is_whitespace) {
        Some((ty, rest))
            if rest.trim().starts_with('$') && !rest.trim().contains(char::is_whitespace) =>
        {
            ty
        }
        _ => value,
    }
}

/// `isNull`: the type is the identifier `null` and there is no description.
fn is_null(value: &str) -> bool {
    value == "null"
}
