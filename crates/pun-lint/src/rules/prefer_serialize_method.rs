//! `lint/classes/prefer-serialize-method`, ported from Rector 2.8.4's `SleepToSerializeRector`
//! (rules/Php85/Rector/Class_).
//!
//! PHP 8.5 deprecates `__sleep()`. In a class without `__serialize()`, a `__sleep()` whose every return gives an array
//! literal (and whose end cannot be reached) becomes `__serialize(): array`, each string item `'name'` turning into
//! `'name' => $this->name` and the other items going. A `__sleep()` already typed `array` stays.
//!
//! Rector's `SilentVoidResolver` also counts a `switch`, a loop and a call to a `never` function as ends of the body;
//! Pun counts a return, a throw, an exit, and an `if`/`else` or a `try` whose branches all end, and leaves the
//! method alone in the other cases.

use mago_syntax::cst::{
    ArrayElement, Construct, Expression, Hint, Literal, MethodBody, Node, NodeKind, Statement,
    TokenSeparatedSequence,
};
use pun_problem::{Edit, Fix, Safety};

use super::lint_b_serialize::{class_method, rename, set_return_type};
use super::no_null_debug_info::returns;
use super::rector_batch5::{list_removal, merge};
use super::rector_stmts::{List, block_list, if_view};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferSerializeMethod;

impl Rule for PreferSerializeMethod {
    fn id(&self) -> &'static str {
        "lint/classes/prefer-serialize-method"
    }

    fn min_php_version(&self) -> u32 {
        // PhpVersionFeature::DEPRECATED_METHOD_SLEEP
        80500
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class, NodeKind::AnonymousClass])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        if class_method(node, "__serialize").is_some() {
            return;
        }
        let Some(method) = class_method(node, "__sleep") else {
            return;
        };
        if method
            .return_type_hint
            .as_ref()
            .is_some_and(|h| matches!(h.hint, Hint::Array(_)))
        {
            return;
        }
        let MethodBody::Concrete(block) = &method.body else {
            return;
        };
        let source = ctx.source;
        // `ReturnAnalyzer::hasOnlyReturnWithExpr()`; `findReturnsScoped()` finds none in a generator.
        let mut values = Vec::new();
        let mut bare = false;
        returns(Node::Block(block), &mut |r| match r.value {
            Some(value) => values.push(unparenthesize(value)),
            None => bare = true,
        });
        if values.is_empty()
            || bare
            || has_yield(Node::Block(block))
            || !always_ends(source, &block_list(source, block))
        {
            return;
        }
        let mut edits = Vec::new();
        for value in values {
            let elements = match value {
                Expression::Array(array) => &array.elements,
                Expression::LegacyArray(array) => &array.elements,
                _ => return,
            };
            let Some(array_edits) = items(source, elements) else {
                return;
            };
            edits.extend(array_edits);
        }
        if edits.is_empty() {
            return;
        }
        edits.push(rename(method, "__serialize"));
        edits.push(set_return_type(method, "array"));
        out.push(Report {
            span: node_span(&method.name),
            message: "Replace the deprecated `__sleep()` with `__serialize()`.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits,
            }),
        });
    }
}

/// The edits that turn an array's string items into `'name' => $this->name` and drop its other items, none when it
/// has no string item. `None` (no fix at all) for a string Pun does not print as Rector does: one that is no
/// property name.
fn items(
    source: &str,
    elements: &TokenSeparatedSequence<'_, ArrayElement<'_>>,
) -> Option<Vec<Edit>> {
    let mut spans = Vec::new();
    let mut names = Vec::new();
    for element in elements.iter() {
        let value = match element {
            ArrayElement::KeyValue(e) => Some(e.value),
            ArrayElement::Value(e) => Some(e.value),
            ArrayElement::Variadic(_) | ArrayElement::Missing(_) => None,
        };
        let name = match value {
            Some(Expression::Literal(Literal::String(s))) => {
                let name = std::str::from_utf8(s.value?).ok()?;
                // Rector prints any name after `$this->`; `'a-b'` would read as a subtraction and `'it\'s'` would
                // not parse, so Pun leaves those methods alone.
                if !is_identifier(name) {
                    return None;
                }
                Some((node_span(s), name))
            }
            _ => None,
        };
        spans.push(node_span(element));
        names.push(name);
    }
    if names.iter().all(Option::is_none) {
        return Some(Vec::new());
    }
    let removed: Vec<bool> = names.iter().map(Option::is_none).collect();
    let mut edits = list_removal(&spans, &removed).unwrap_or_default();
    for (span, name) in spans.iter().zip(&names) {
        if let Some((string, name)) = name {
            let key = &source[string.start..string.end];
            edits.push(Edit::new(*span, format!("{key} => $this->{name}")));
        }
    }
    Some(merge(edits))
}

/// `SilentVoidResolver::hasStmtsAlwaysReturnOrExit()`, for the statements Pun follows.
fn always_ends(source: &str, list: &List<'_>) -> bool {
    list.stmts.iter().any(|statement| match statement {
        Statement::Return(r) => r.value.is_some(),
        Statement::Expression(e) => matches!(
            unparenthesize(e.expression),
            Expression::Throw(_) | Expression::Construct(Construct::Exit(_) | Construct::Die(_))
        ),
        Statement::If(r#if) => {
            let view = if_view(source, r#if);
            view.else_
                .as_ref()
                .is_some_and(|(list, _)| always_ends(source, list))
                && always_ends(source, &view.stmts)
                && view
                    .elseifs
                    .iter()
                    .all(|(_, list, _)| always_ends(source, list))
        }
        Statement::Try(r#try) => {
            let ends = |block| always_ends(source, &block_list(source, block));
            r#try
                .finally_clause
                .as_ref()
                .is_some_and(|f| ends(&f.block))
                || (ends(&r#try.block) && r#try.catch_clauses.iter().all(|c| ends(&c.block)))
        }
        _ => false,
    })
}

/// A name `$this->name` can take: a letter, `_` or a byte from 0x80, then those and digits.
pub(super) fn is_identifier(name: &str) -> bool {
    let mut bytes = name.bytes();
    bytes
        .next()
        .is_some_and(|b| b == b'_' || b.is_ascii_alphabetic() || b >= 0x80)
        && bytes.all(|b| b == b'_' || b.is_ascii_alphanumeric() || b >= 0x80)
}

/// Whether a `yield` comes under `node`, outside nested classes, functions and closures.
fn has_yield(node: Node<'_, '_>) -> bool {
    let mut found = false;
    node.visit_children(|child| match child {
        Node::Class(_) | Node::AnonymousClass(_) | Node::Function(_) | Node::Closure(_) => {}
        Node::Yield(_) => found = true,
        _ => found |= has_yield(child),
    });
    found
}
