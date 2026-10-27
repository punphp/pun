//! `lint/refactor/replace-array-access`, ported from Rector 2.6.1's `ArrayDimFetchToMethodCallRector`
//! (rules/Transform/Rector/ArrayDimFetch).
//!
//! On an object of a configured type (`NodeTypeResolver::isObjectType`: the class, a class extending or implementing
//! it, or a class the codebase does not know written with the same name; `null` and `false` left out of a union), an
//! array access becomes a call of the configured methods, the first configuration with a method for the access
//! deciding:
//!
//! - `$object['key']` read: `$object->get('key')`.
//! - `$object['key'] = $value`: `$object->set('key', $value)`.
//! - `isset(...)`: each matching variable becomes `$object->has('key')`, joined with `&&` after an `isset()` of the
//!   others, with parentheses where php-parser's printer adds them (`!`, a cast, an operator binding tighter than
//!   `&&`, the right of `&&`).
//! - `unset(...)`: each matching variable becomes a statement `$object->unset('key');` after an `unset()` of the
//!   others.
//!
//! An access with no key (`$object[] = ...`), one a compound assignment or `=&` writes, one a destructuring list
//! assigns, a foreach target, and an `isset()` or `unset()` variable without a method stays. The key and the value keep
//! their text, as the format-preserving printer keeps the nodes they were. Where Rector writes code PHP rejects or
//! that means something else, Pun does not follow it: an access `++` or `--` changes, or one `=&` assigns to, stays
//! (Rector writes a call in a write context); a bare-word key in a string (`"$object[key]"`) is quoted
//! (`{$object->get('key')}`, where Rector writes the undefined constant `key`). When an `unset()` that is the only
//! statement of its list becomes several, php-parser reprints the node that holds the list; Pun keeps that node's
//! header as written and writes the list as the printer does, on lines of its own.
//!
//! Options: `items = [{ object-type = { class-name = "SomeClass" }, method = "get", set-method = "set",
//! exists-method = "has", unset-method = "unset" }]`, the `ArrayDimFetchToMethodCall` value objects as
//! `pun migrate` writes them (an `ObjectType`'s other, empty, properties may come along).

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::reference::TReference;
use mago_syntax::cst::{
    ArrayAccess, Construct, Expression, IssetConstruct, Node, NodeKind, Statement, StringPart,
};
use pun_problem::{Edit, Fix, Safety, Span};
use serde_json::Value;

use super::super::php_parser::precedence;
use super::super::rector_batch5::{list_removal, merge};
use super::super::rector_stmts::{Frame, enclosing, line_indent, replace_statement};
use super::super::lint_b_helpers::replace_run;
use super::super::types::facts::facts;
use super::{is_class, items, object, string};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// One `ArrayDimFetchToMethodCall`.
#[derive(Clone, Debug)]
pub struct ArrayAccessCall {
    pub class: String,
    pub get: String,
    pub set: Option<String>,
    pub exists: Option<String>,
    pub unset: Option<String>,
}

#[derive(Default)]
pub struct ReplaceArrayAccess {
    pub calls: Vec<ArrayAccessCall>,
}

#[derive(Clone, Copy)]
enum Handler {
    Get,
    Set,
    Exists,
    Unset,
}

/// An `ObjectType` as `pun migrate` writes it: its `class-name`, the empty caches it holds on construction too.
fn object_type(value: &Value) -> Result<String, String> {
    let Value::Object(map) = value else {
        return Err("`object-type` must be a table with a `class-name`".into());
    };
    for (key, v) in map {
        match key.as_str() {
            "class-name" => {}
            "current-ancestors" | "method-cache" if v.as_array().is_some_and(Vec::is_empty) => {}
            _ => return Err(format!("unknown key `{key}` in `object-type`; it takes `class-name`")),
        }
    }
    match map.get("class-name").and_then(Value::as_str) {
        Some(name) => Ok(name.trim_start_matches('\\').to_string()),
        None => Err("`object-type` must be a table with a `class-name`".into()),
    }
}

impl ReplaceArrayAccess {
    fn method(&self, ctx: &Context<'_>, access: &ArrayAccess<'_>, handler: Handler) -> Option<&str> {
        self.calls.iter().find_map(|call| {
            if !is_object_type(ctx, access.array, &call.class) {
                return None;
            }
            match handler {
                Handler::Get => Some(call.get.as_str()),
                Handler::Set => call.set.as_deref(),
                Handler::Exists => call.exists.as_deref(),
                Handler::Unset => call.unset.as_deref(),
            }
        })
    }

    /// `$var->method(key)` for an access a statement or an expression is rebuilt around.
    fn call_text(&self, ctx: &Context<'_>, e: &Expression<'_>, handler: Handler) -> Option<String> {
        let Expression::ArrayAccess(access) = e else {
            return None;
        };
        let method = self.method(ctx, access, handler)?;
        Some(format!(
            "{}->{method}({})",
            text(ctx, node_span(access.array)),
            text(ctx, node_span(access.index))
        ))
    }
}

fn text<'s>(ctx: &Context<'s>, span: Span) -> &'s str {
    &ctx.source[span.start..span.end]
}

/// `NodeTypeResolver::isObjectType($expr, new ObjectType($class))` on the type Mago gives the expression.
fn is_object_type(ctx: &Context<'_>, expr: &Expression<'_>, class: &str) -> bool {
    let facts = facts(ctx);
    let Some(t) = facts.full_type(node_span(unparenthesize(expr))) else {
        return false;
    };
    let kept: Vec<&TAtomic> = (t.types.iter())
        .filter(|a| !a.is_null() && !matches!(a, TAtomic::Scalar(s) if s.is_false()))
        .collect();
    !kept.is_empty()
        && kept.iter().all(|a| match a {
            TAtomic::Object(TObject::Named(n)) => {
                is_class(ctx, &n.name.to_string(), class)
            }
            TAtomic::Reference(TReference::Symbol { name, .. }) => name.to_string() == class,
            _ => false,
        })
}

/// The nodes above `ancestors`' last one, nearest first, without the `Expression` wrappers of Mago's walk.
fn above<'a, 'ast, 'arena: 'ast>(
    ancestors: &'a [Node<'ast, 'arena>],
) -> impl Iterator<Item = Node<'ast, 'arena>> + 'a {
    ancestors
        .iter()
        .rev()
        .copied()
        .filter(|n| !matches!(n, Node::Expression(_)))
}

/// Whether the access at `span` is one the read handler leaves to another (or to no) handler.
fn skipped_read(ancestors: &[Node<'_, '_>], span: Span) -> bool {
    if in_foreach_target(ancestors) {
        return true;
    }
    let mut chain = above(ancestors);
    match chain.next() {
        // Any assignment's variable: `=` is the set handler's (`IS_BEING_ASSIGNED`), a compound one's
        // `IS_ASSIGN_OP_VAR`, and `=&` Rector would turn into a call in a write context.
        Some(Node::Assignment(a)) => node_span(a.lhs) == span,
        Some(Node::IssetConstruct(_) | Node::Unset(_)) => true,
        // `++`, `--`: a call in a write context.
        Some(Node::UnaryPrefix(u)) => u.operator.is_increment_or_decrement(),
        Some(Node::UnaryPostfix(_)) => true,
        // The value of an item of a list an assignment destructures into (`IS_BEING_ASSIGNED` on `$item->value`).
        Some(Node::ValueArrayElement(_)) => is_destructured(chain),
        Some(Node::KeyValueArrayElement(kv)) => node_span(kv.value) == span && is_destructured(chain),
        _ => false,
    }
}

/// Whether the list above an item (`chain` starting at the item's `ArrayElement`) is what an assignment writes.
fn is_destructured<'ast, 'arena: 'ast>(mut chain: impl Iterator<Item = Node<'ast, 'arena>>) -> bool {
    let (Some(Node::ArrayElement(_)), Some(list @ (Node::Array(_) | Node::List(_)))) = (chain.next(), chain.next())
    else {
        return false;
    };
    matches!(chain.next(), Some(Node::Assignment(a)) if node_span(a.lhs) == node_span(&list))
}

/// PHPStan gives the nodes of a foreach target no scope, so Rector resolves no type there.
fn in_foreach_target(ancestors: &[Node<'_, '_>]) -> bool {
    ancestors.iter().any(|n| {
        matches!(n, Node::ForeachValueTarget(_) | Node::ForeachKeyValueTarget(_))
    })
}

/// The key of an access in a string written without braces (`"$a[key]"`) as PHP reads it: a bare word or a number
/// PHP keeps as a string quoted, a canonical integer and a variable as they are.
fn interpolated_key(key: &str) -> String {
    let digits = key.strip_prefix('-').unwrap_or(key);
    let canonical = !digits.is_empty()
        && digits.bytes().all(|b| b.is_ascii_digit())
        && (digits == "0" || !digits.starts_with('0'))
        && key.parse::<i64>().is_ok()
        && key != "-0";
    if key.starts_with('$') || canonical {
        key.to_string()
    } else {
        format!("'{key}'")
    }
}

fn report(span: Span, method: &str, edits: Vec<Edit>) -> Report {
    Report {
        span,
        message: format!("Call `{method}()` instead of the array access."),
        help: None,
        fix: Some(Fix {
            safety: Safety::Unsafe,
            edits,
        }),
    }
}

impl Rule for ReplaceArrayAccess {
    fn id(&self) -> &'static str {
        "lint/refactor/replace-array-access"
    }

    fn configure(&mut self, options: &Value) -> Result<(), String> {
        const KEYS: &[&str] = &["object-type", "method", "set-method", "exists-method", "unset-method"];
        let mut calls = Vec::new();
        for item in items(options)? {
            let map = object(item, KEYS, &["object-type", "method"])?;
            let optional = |key: &str| -> Result<Option<String>, String> {
                map.get(key).map(|_| string(map, key)).transpose()
            };
            calls.push(ArrayAccessCall {
                class: object_type(&map["object-type"])?,
                get: string(map, "method")?,
                set: optional("set-method")?,
                exists: optional("exists-method")?,
                unset: optional("unset-method")?,
            });
        }
        self.calls = calls;
        Ok(())
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::ArrayAccess,
            NodeKind::Assignment,
            NodeKind::IssetConstruct,
            NodeKind::Statement,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        if self.calls.is_empty() {
            return;
        }
        match node {
            Node::ArrayAccess(access) => self.read(access, ancestors, ctx, out),
            Node::Assignment(a) => {
                let Expression::ArrayAccess(access) = a.lhs else {
                    return;
                };
                if !super::super::rector_stmts::is_plain_assign(a) || in_foreach_target(ancestors) {
                    return;
                }
                let Some(method) = self.method(ctx, access, Handler::Set) else {
                    return;
                };
                let rhs = node_span(a.rhs);
                let edits = vec![
                    Edit::new(
                        Span::new(node_span(access.array).end, node_span(access.index).start),
                        format!("->{method}("),
                    ),
                    Edit::new(Span::new(node_span(access.index).end, rhs.start), ", "),
                    Edit::new(Span::new(rhs.end, rhs.end), ")"),
                ];
                out.push(report(node_span(a), method, edits));
            }
            Node::IssetConstruct(isset) => self.isset(isset, ancestors, ctx, out),
            Node::Statement(stmt @ Statement::Unset(_)) => self.unset(stmt, ancestors, ctx, out),
            _ => {}
        }
    }
}

impl ReplaceArrayAccess {
    fn read(
        &self,
        access: &ArrayAccess<'_>,
        ancestors: &[Node<'_, '_>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let span = node_span(access);
        if skipped_read(ancestors, span) {
            return;
        }
        let Some(method) = self.method(ctx, access, Handler::Get) else {
            return;
        };
        let (array, index) = (node_span(access.array), node_span(access.index));
        let edits = match above(ancestors).next() {
            // `"$a[key]"`: the printer wraps the call in braces (`FIXUP_ENCAPSED`).
            Some(Node::StringPart(StringPart::Expression(_))) => vec![Edit::new(
                span,
                format!(
                    "{{{}->{method}({})}}",
                    text(ctx, array),
                    interpolated_key(text(ctx, index))
                ),
            )],
            _ => vec![
                Edit::new(Span::new(array.end, index.start), format!("->{method}(")),
                Edit::new(Span::new(index.end, span.end), ")"),
            ],
        };
        out.push(report(span, method, edits));
    }

    fn isset(
        &self,
        isset: &IssetConstruct<'_>,
        ancestors: &[Node<'_, '_>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let values: Vec<&Expression<'_>> = isset.values.iter().copied().collect();
        let calls: Vec<Option<String>> = values
            .iter()
            .map(|v| self.call_text(ctx, v, Handler::Exists))
            .collect();
        if calls.iter().all(Option::is_none) {
            return;
        }
        let span = node_span(isset);
        let mut parts = Vec::new();
        let removed: Vec<bool> = calls.iter().map(Option::is_some).collect();
        let spans: Vec<Span> = values.iter().map(|v| node_span(*v)).collect();
        if let Some(edits) = list_removal(&spans, &removed) {
            let mut kept = text(ctx, span).to_string();
            for edit in merge(edits).iter().rev() {
                kept.replace_range(edit.span.start - span.start..edit.span.end - span.start, &edit.text);
            }
            parts.push(kept);
        }
        parts.extend(calls.into_iter().flatten());
        let mut new = parts.join(" && ");
        if parts.len() > 1 && needs_parentheses(ancestors, span) {
            new = format!("({new})");
        }
        out.push(report(span, "isset", vec![Edit::new(span, new)]));
    }

    fn unset(
        &self,
        stmt: &Statement<'_>,
        ancestors: &[Node<'_, '_>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Statement::Unset(unset) = stmt else { return };
        let values: Vec<&Expression<'_>> = unset.values.iter().copied().collect();
        let calls: Vec<Option<String>> = values
            .iter()
            .map(|v| self.call_text(ctx, v, Handler::Unset))
            .collect();
        if calls.iter().all(Option::is_none) {
            return;
        }
        let span = node_span(stmt);
        let indent = line_indent(ctx.source, span.start).to_string();
        let lines: Vec<String> = calls.iter().flatten().map(|c| format!("{c};")).collect();
        let removed: Vec<bool> = calls.iter().map(Option::is_some).collect();
        let spans: Vec<Span> = values.iter().map(|v| node_span(*v)).collect();
        let edits = match list_removal(&spans, &removed) {
            // The `unset()` stays with the variables left; the calls follow it, a line each.
            Some(edits) => {
                let mut edits = merge(edits);
                let added: String = lines.iter().map(|l| format!("\n{indent}{l}")).collect();
                edits.push(Edit::new(Span::new(span.end, span.end), added));
                edits
            }
            None => {
                let (list, index) = enclosing(ctx.source, stmt, ancestors);
                if lines.len() == 1 {
                    vec![Edit::new(span, lines[0].clone())]
                } else if list.frame == Frame::Bare {
                    vec![replace_statement(ctx.source, &list, index, |indent| {
                        lines.join(&format!("\n{indent}"))
                    })]
                } else if list.len() == 1 && list.nop_span(ctx.source).is_none()
                    && let Some(Node::Block(block)) = ancestors.last()
                {
                    // php-parser reprints the list's holder; its block is a line per statement.
                    let open = block.left_brace.start.offset as usize;
                    let close = block.right_brace.end.offset as usize;
                    let outer = line_indent(ctx.source, open);
                    let inner = format!("{outer}    ");
                    let body: String = lines.iter().map(|l| format!("\n{inner}{l}")).collect();
                    vec![Edit::new(Span::new(open, close), format!("{{{body}\n{outer}}}"))]
                } else {
                    vec![replace_run(
                        ctx.source,
                        &list,
                        index,
                        index,
                        &lines.join(&format!("\n{indent}")),
                    )]
                }
            }
        };
        out.push(report(span, "unset", edits));
    }
}

/// Whether php-parser's printer puts the `isset() && ...` that replaced the `isset()` at `span` in parentheses:
/// where its parent's fixup asks for a precedence `&&` does not meet, and the source has none around it.
fn needs_parentheses(ancestors: &[Node<'_, '_>], span: Span) -> bool {
    let parent = above(ancestors).find(|n| !matches!(n, Node::Construct(Construct::Isset(_))));
    match parent {
        Some(Node::Binary(b)) => {
            let on_right = node_span(b.rhs).start <= span.start;
            precedence(&b.operator) > 7 || (precedence(&b.operator) == 7 && on_right)
        }
        Some(Node::UnaryPrefix(_) | Node::Clone(_) | Node::Pipe(_)) => true,
        _ => false,
    }
}
