//! `lint/style/prefer-class-name-constant`, ported from Rector 2.8.4's `StringClassNameToClassConstantRector`
//! (rules/Php55/Rector/String_).
//!
//! A string whose value (without leading `\`) names a known class, interface, trait or enum with a namespace, and
//! does not start with a lowercase letter, becomes `\Name::class`, the name as the string writes it. php-parser's
//! `String_` is a quoted string without interpolation or a heredoc or nowdoc with only text. A string passed straight
//! to a call written `is_a(...)` (Rector compares the call's name as written, case included) is left alone. A class is
//! known when the codebase (PHP's built-in symbols and the project's) has it, as `ReflectionProvider::hasClass`.
//!
//! Options: `items = ["Nette\\*", "Error"]`, the names to leave alone: a pattern with `*` matched as `fnmatch` with
//! `FNM_NOESCAPE` matches it, any other name without regard to case.

use mago_syntax::cst::{
    CompositeString, Expression, Identifier, Literal, Node, NodeKind, StringPart,
};
use pun_problem::{Edit, Fix, Safety};

use super::refactor::{is_name, items};
use super::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

#[derive(Default)]
pub struct PreferClassNameConstant {
    pub skip: Vec<String>,
}

/// php-parser's value of a heredoc or nowdoc with text only (Mago's literal parts hold it, the closing label's
/// indentation and the newline before it taken out).
fn document_value(doc: &mago_syntax::cst::DocumentString<'_>) -> Option<Vec<u8>> {
    let mut value = Vec::new();
    for part in doc.parts.iter() {
        let StringPart::Literal(literal) = part else {
            return None;
        };
        value.extend_from_slice(literal.value.unwrap_or(literal.raw));
    }
    Some(value)
}

/// `fnmatch($pattern, $name, FNM_NOESCAPE)`: `*` and `?` as wildcards (they cross `\`), `[...]` a class.
fn fnmatch(pattern: &[u8], name: &[u8]) -> bool {
    match pattern.first() {
        None => name.is_empty(),
        Some(b'*') => (0..=name.len()).any(|i| fnmatch(&pattern[1..], &name[i..])),
        Some(b'?') => !name.is_empty() && fnmatch(&pattern[1..], &name[1..]),
        Some(b'[') => {
            let negate = matches!(pattern.get(1), Some(b'!' | b'^'));
            let first = 1 + usize::from(negate);
            // A `]` right after `[` or `[!` is a member of the set; without a closing `]` the `[` is literal.
            let close = (pattern.get(first + 1..))
                .and_then(|rest| rest.iter().position(|b| *b == b']'))
                .map(|i| i + first + 1);
            let Some(close) = close else {
                return name.first() == Some(&b'[') && fnmatch(&pattern[1..], &name[1..]);
            };
            let Some(&c) = name.first() else { return false };
            let set = &pattern[first..close];
            let mut hit = false;
            let mut i = 0;
            while i < set.len() {
                if i + 2 < set.len() && set[i + 1] == b'-' {
                    hit |= set[i] <= c && c <= set[i + 2];
                    i += 3;
                } else {
                    hit |= set[i] == c;
                    i += 1;
                }
            }
            hit != negate && fnmatch(&pattern[close + 1..], &name[1..])
        }
        Some(&p) => name.first() == Some(&p) && fnmatch(&pattern[1..], &name[1..]),
    }
}

/// Whether the string at `span` is an argument value of a call written `is_a(...)` (`FROM_FUNC_CALL_NAME`).
fn is_a_argument(ancestors: &[Node<'_, '_>], span: pun_problem::Span) -> bool {
    let Some(Node::FunctionCall(call)) = ancestors
        .iter()
        .rev()
        .find(|n| matches!(n, Node::FunctionCall(_)))
    else {
        return false;
    };
    let Expression::Identifier(name) = call.function else {
        return false;
    };
    let written = match name {
        Identifier::FullyQualified(id) => &id.value[1..],
        _ => name.value(),
    };
    written == b"is_a"
        && call
            .argument_list
            .arguments
            .iter()
            .any(|a| node_span(unparenthesize(a.value())) == span)
}

impl Rule for PreferClassNameConstant {
    fn id(&self) -> &'static str {
        "lint/style/prefer-class-name-constant"
    }

    /// `PhpVersionFeature::CLASSNAME_CONSTANT`.
    fn min_php_version(&self) -> u32 {
        50500
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        self.skip = items(options)?
            .into_iter()
            .map(|v| {
                v.as_str()
                    .map(str::to_string)
                    .ok_or("each of `items` must be a string")
            })
            .collect::<Result<_, _>>()?;
        Ok(())
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Literal, NodeKind::CompositeString])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (span, value) = match node {
            Node::Literal(Literal::String(s)) => {
                (node_span(s), s.value.unwrap_or_default().to_vec())
            }
            Node::CompositeString(CompositeString::Document(d)) => {
                let Some(value) = document_value(d) else {
                    return;
                };
                (node_span(d), value)
            }
            _ => return,
        };
        if is_a_argument(ancestors, span) {
            return;
        }
        let start = value
            .iter()
            .position(|b| *b != b'\\')
            .unwrap_or(value.len());
        // A name that is not UTF-8 names no class: the codebase's names come from UTF-8 sources.
        let name = String::from_utf8_lossy(&value[start..]);
        if name.is_empty() || !name.contains('\\') || name.as_bytes()[0].is_ascii_lowercase() {
            return;
        }
        if !facts(ctx).codebase.class_like_exists(name.as_bytes()) {
            return;
        }
        if self.skip.iter().any(|skip| match skip.contains('*') {
            true => fnmatch(skip.as_bytes(), name.as_bytes()),
            false => is_name(&name, skip),
        }) {
            return;
        }
        out.push(Report {
            span,
            message: format!("Use `\\{name}::class` instead of the class name in a string."),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits: vec![Edit::new(span, format!("\\{name}::class"))],
            }),
        });
    }
}
