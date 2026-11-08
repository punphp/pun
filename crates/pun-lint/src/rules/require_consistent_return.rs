//! `lint/complexity/require-consistent-return`, ported from Rector 2.8.4's `ExplicitReturnNullRector`
//! (rules/CodeQuality/Rector/ClassMethod).
//!
//! A function or method (not a constructor) without a return type whose returns PHPStan infers as a union with `void`
//! (`ReturnTypeInferer`: a value's native type, `void` for a bare `return` or a reachable end) says `null` out loud:
//! each bare `return` becomes `return null`, and where the end of the body can be reached (a call PHPStan types
//! `never`, docblocks read, ends it) a `return null;` follows the last statement. A `@return` union with `void` then
//! has `null` there. Nothing changes with a `goto`, a `yield`, or a returned value whose native type is `mixed`.
//!
//! Rector prints the `@return` type anew (`PhpDocTypeChanger`): keywords in lower case, `boolean` as `bool`,
//! `true|false` as `bool` (none of them beside a `bool`), a repeat once, a class as its fully qualified name without
//! the leading `\` (an imported or partly qualified name as written, `self` as the class-like with it), the members
//! in `UnionTypeHelper::sortTypes()`'s order, and no blanks around a `|`. Only the first `@return` tag counts. Pun
//! leaves a function alone whose `@return` union with `void` holds another kind of type (generics, shapes, `static`,
//! `parent`), which Rector's type mapper rewrites.

use mago_syntax::cst::{Block, MethodBody, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety, Span};

use super::class_method::qualified;
use super::lint_b_doc::Doc;
use super::php_parser::{block_list, tokens};
use super::rector_conditions::{comments_between, indentation};
use super::types::facts::facts;
use super::types::names::resolve_written;
use super::types::silent_void::{Place, has_silent_void, has_silent_void_documented};
use super::types::{any_scoped, class_scope, returns_scoped};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct RequireConsistentReturn;

impl Rule for RequireConsistentReturn {
    fn id(&self) -> &'static str {
        "lint/complexity/require-consistent-return"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method, NodeKind::Function])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let body: &Block<'_> = match node {
            Node::Method(m) => match &m.body {
                MethodBody::Concrete(b)
                    if m.return_type_hint.is_none()
                        && !m.name.value.eq_ignore_ascii_case(b"__construct") =>
                {
                    b
                }
                _ => return,
            },
            Node::Function(f) if f.return_type_hint.is_none() => &f.body,
            _ => return,
        };
        let returns = returns_scoped(body);
        let bare: Vec<_> = returns.iter().filter(|r| r.value.is_none()).collect();
        if returns.len() == bare.len() {
            return;
        }
        let facts = facts(ctx);
        // every returned value of a known native type other than `mixed`, which a union with it would be
        if !returns.iter().filter_map(|r| r.value).all(|v| {
            facts
                .native_type(node_span(v))
                .is_some_and(|t| !t.types.is_empty() && !t.is_mixed())
        }) {
            return;
        }
        let scope = class_scope(ancestors);
        let place = Place {
            facts: &facts,
            ancestors,
            scope: scope.as_ref(),
        };
        if bare.is_empty() && !has_silent_void(&place, body.statements.iter()) {
            return;
        }
        if any_scoped(Node::Block(body), &mut |n| matches!(n, Node::Goto(_))) {
            return;
        }
        let source = ctx.source;
        let mut edits: Vec<Edit> = bare
            .iter()
            .map(|r| {
                let at = r.r#return.span.end.offset as usize;
                Edit::new(Span::new(at, at), " null")
            })
            .collect();
        if has_silent_void_documented(&place, body.statements.iter()) {
            let tokens = tokens(ctx);
            let list = block_list(&tokens, body);
            let last = node_span(*list.stmts.last().expect("a body that returns"));
            // after the comments php-parser keeps in a `Nop` at the end, at their indent
            let (end, from) = match comments_between(&tokens, last.end, list.inner.end).last() {
                Some(c) if list.nop => (c.end, c.start),
                _ => (last.end, last.start),
            };
            let indent = " ".repeat(indentation(source, from));
            edits.push(Edit::new(
                Span::new(end, end),
                format!("\n{indent}return null;"),
            ));
        } else if bare.is_empty() {
            return;
        }
        match doc_edit(ctx, node, ancestors) {
            Some(Some(edit)) => edits.push(edit),
            Some(None) => {}
            None => return,
        }
        edits.sort_by_key(|e| e.span.start);
        out.push(Report {
            span: node_span(&node),
            message: "This function returns a value on some paths; return `null` explicitly on the others."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// `transformDocUnionVoidToUnionNull`: the edit of the `@return` type, `Some(None)` when it stays, `None` when Pun
/// cannot print it as Rector does.
fn doc_edit(ctx: &crate::Context<'_>, node: Node<'_, '_>, ancestors: &[Node<'_, '_>]) -> Option<Option<Edit>> {
    let source = ctx.source;
    let Some(doc) = Doc::of(ctx, ancestors, node_span(&node).start) else {
        return Some(None);
    };
    let text = &source[doc.span.start..doc.span.end];
    let tags: Vec<usize> = text
        .match_indices("@return")
        .filter(|(i, _)| {
            let before = text[..*i].chars().next_back();
            let after = text[i + 7..].chars().next();
            !before.is_some_and(|c| c.is_alphanumeric() || c == '-')
                && after.is_some_and(char::is_whitespace)
        })
        .map(|(i, _)| i + 7)
        .collect();
    // `getReturnType()` reads the first tag
    let Some(&at) = tags.first() else {
        return Some(None);
    };
    let start = at + text[at..].len() - text[at..].trim_start_matches([' ', '\t']).len();
    // a type ends at a blank outside brackets (`array<int, string>` is one)
    let word_end = |from: usize| {
        let mut depth = 0i32;
        let rest = &text[from..];
        from + rest
            .char_indices()
            .find(|&(_, c)| {
                match c {
                    '<' | '(' | '{' | '[' => depth += 1,
                    '>' | ')' | '}' | ']' => depth -= 1,
                    _ => {}
                }
                depth <= 0 && c.is_whitespace()
            })
            .map_or(rest.len(), |(i, _)| i)
    };
    // a union may have blanks around its `|` (`int | void`)
    let mut end = word_end(start);
    while let Some(next) = text[end..]
        .trim_start_matches([' ', '\t'])
        .strip_prefix('|')
    {
        end = word_end(text.len() - next.trim_start_matches([' ', '\t']).len());
    }
    let end = text[..end].strip_suffix("*/").map_or(end, str::len);
    let written: String = text[start..end]
        .chars()
        .filter(|c| !matches!(c, ' ' | '\t'))
        .collect();
    let len = end - start;
    let members: Vec<&str> = written.split('|').collect();
    if members.len() < 2 || !members.iter().any(|m| m.eq_ignore_ascii_case("void")) {
        return Some(None);
    }
    let mut printed: Vec<String> = Vec::new();
    for member in &members {
        let lower = member.to_ascii_lowercase();
        let name = match lower.as_str() {
            "void" | "null" => "null".to_string(),
            "boolean" | "bool" => "bool".to_string(),
            "integer" | "int" => "int".to_string(),
            "double" | "float" => "float".to_string(),
            "string" | "true" | "false" | "array" => lower,
            // a union with `mixed` is `mixed`, no union
            "mixed" => return Some(None),
            // the class-like around (an anonymous class's name is Rector's own hash); outside one the tag stays
            "self" => {
                let class = ancestors.iter().enumerate().rev().find_map(|(i, a)| {
                    let name = match a {
                        Node::Class(c) => c.name.value,
                        Node::Trait(t) => t.name.value,
                        Node::Enum(e) => e.name.value,
                        Node::AnonymousClass(_) => return Some(None),
                        _ => return None,
                    };
                    Some(Some(qualified(&ancestors[..i], name)))
                });
                // PHPStan's `ObjectType`, a `FullyQualifiedNameNode`
                match class {
                    Some(class) => format!("\\{}", class?),
                    None => return Some(None),
                }
            }
            _ if is_class_name(member) && !is_keyword(&lower) => {
                if member.starts_with('\\') {
                    member[1..].to_string()
                } else if member.contains('\\') {
                    member.to_string()
                } else {
                    match resolve_written(ancestors, member) {
                        (_, true) => member.to_string(),
                        (resolved, false) => resolved,
                    }
                }
            }
            _ => return None,
        };
        printed.push(name);
    }
    // `true|false` is `bool`
    if let (Some(t), Some(f)) = (
        printed.iter().position(|p| p == "true"),
        printed.iter().position(|p| p == "false"),
    ) {
        if printed.iter().any(|p| p == "bool") {
            printed.retain(|p| p != "true" && p != "false");
        } else {
            printed[t.min(f)] = "bool".to_string();
            printed.remove(t.max(f));
        }
    }
    let mut unique: Vec<String> = Vec::new();
    for p in printed {
        if !unique.iter().any(|u| u.eq_ignore_ascii_case(&p)) {
            unique.push(p);
        }
    }
    // `UnionType::toPhpDocNode()` prints the members in `UnionTypeHelper::sortTypes()`'s order: `null` last, `true`
    // and `false` before it, the rest by their name (`describe()`, a class without the leading `\`), case-insensitively
    let key = |p: &String| -> (u8, String, String) {
        let rank = match p.as_str() {
            "null" => 2,
            "true" | "false" => 1,
            _ => 0,
        };
        let name = if is_class_name(p) && !matches!(p.as_str(), "int" | "float" | "string" | "bool" | "array") {
            resolve_written(ancestors, p).0
        } else {
            p.clone()
        };
        (rank, name.to_ascii_lowercase(), name)
    };
    unique.sort_by_key(key);
    if unique.len() < 2 {
        return Some(None);
    }
    let from = doc.span.start + start;
    Some(Some(Edit::new(
        Span::new(from, from + len),
        unique.join("|"),
    )))
}

fn is_class_name(member: &str) -> bool {
    let name = member.strip_prefix('\\').unwrap_or(member);
    !name.is_empty()
        && name.split('\\').all(|part| {
            part.chars()
                .next()
                .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
                && part.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        })
}

/// A phpdoc keyword Rector maps to a type of its own.
fn is_keyword(lower: &str) -> bool {
    matches!(
        lower,
        "static"
            | "parent"
            | "never"
            | "object"
            | "iterable"
            | "callable"
            | "resource"
            | "scalar"
            | "numeric"
            | "number"
            | "list"
            | "non-empty-string"
            | "class-string"
            | "positive-int"
            | "negative-int"
            | "array-key"
            | "noreturn"
            | "never-return"
            | "never-returns"
    ) || lower.contains('-')
}
