//! `lint/style/prefer-class-keyword`, ported from Rector 2.8.4's `UseClassKeywordForClassNameResolutionRector`
//! (rules/CodingStyle/Rector/String_).
//!
//! A string that names a known class before `::` (`'App\Models\User::find()'`) is split around each occurrence of
//! that name, which becomes `\App\Models\User::class`: `\App\Models\User::class . '::find()'`. Only names with a
//! namespace count; heredoc and nowdoc strings are left alone. A class is known when the codebase (the built-in
//! symbols and the project's) has it, as PHPStan's `ReflectionProvider::hasClass`.

use mago_syntax::cst::{Literal, LiteralStringKind, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::prefer_absolute_include::print_string;
use super::rector_code_quality::{Prec, in_parent};
use super::types::facts::facts;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferClassKeyword;

impl Rule for PreferClassKeyword {
    fn id(&self) -> &'static str {
        "lint/style/prefer-class-keyword"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Literal])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Literal(Literal::String(string)) = node else {
            return;
        };
        let value = string.value.unwrap_or_default();
        let names = before_double_colon(value);
        if names.is_empty() {
            return;
        }
        let facts = facts(ctx);
        let has_class = |name: &[u8]| {
            let name = name.strip_prefix(b"\\").unwrap_or(name);
            !name.is_empty() && facts.codebase.get_class_like(name).is_some()
        };
        let classes: Vec<&[u8]> = names
            .into_iter()
            .filter(|n| has_class(n) && n.contains(&b'\\'))
            .collect();
        if classes.is_empty() {
            return;
        }
        let mut exprs = Vec::new();
        for part in split(value, &classes) {
            if has_class(part) {
                let name = part.strip_prefix(b"\\").unwrap_or(part);
                exprs.push(format!("\\{}::class", String::from_utf8_lossy(name)));
            } else {
                let Some(printed) = print_string(LiteralStringKind::SingleQuoted, part) else {
                    return;
                };
                exprs.push(printed);
            }
        }
        let span = node_span(string);
        let code = in_parent(span, ancestors, exprs.join(" . "), Prec::Infix(50, 51, 50));
        out.push(Report {
            span,
            message: "Use `::class` instead of a class name in a string.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits: vec![Edit::new(span, code)],
            }),
        });
    }
}

/// The `class_name` group of each match of `#(?<class_name>[\\a-zA-Z0-9_\x80-\xff]*)::#`, in order.
fn before_double_colon(value: &[u8]) -> Vec<&[u8]> {
    let class_char = |b: u8| b == b'\\' || b == b'_' || b.is_ascii_alphanumeric() || b >= 0x80;
    let mut names = Vec::new();
    let mut pos = 0;
    while let Some(k) = value[pos..]
        .windows(2)
        .position(|w| w == b"::")
        .map(|k| k + pos)
    {
        let mut start = k;
        while start > pos && class_char(value[start - 1]) {
            start -= 1;
        }
        names.push(&value[start..k]);
        pos = k + 2;
    }
    names
}

/// `Strings::split($value, '#(name|name...)#')` without the empty pieces: the text between the names and the names.
fn split<'a>(value: &'a [u8], names: &[&[u8]]) -> Vec<&'a [u8]> {
    let mut parts = Vec::new();
    let (mut from, mut i) = (0, 0);
    while i < value.len() {
        match names.iter().find(|n| value[i..].starts_with(n)) {
            Some(name) => {
                parts.push(&value[from..i]);
                parts.push(&value[i..i + name.len()]);
                i += name.len();
                from = i;
            }
            None => i += 1,
        }
    }
    parts.push(&value[from..]);
    parts.retain(|p| !p.is_empty());
    parts
}
