//! `lint/style/no-escaped-quotes`, ported from Rector 2.6.1's `SimplifyQuoteEscapeRector`
//! (rules/CodingStyle/Rector/String_).
//!
//! A single-quoted string that holds `'` but no `"` becomes double-quoted, and a double-quoted one that holds `"` but
//! no `'` becomes single-quoted, so the quotes inside need no escape. A string with a backslash, a `$`, a newline, a
//! tab or any byte outside printable ASCII stays.

use mago_syntax::cst::{LiteralStringKind, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoEscapedQuotes;

impl Rule for NoEscapedQuotes {
    fn id(&self) -> &'static str {
        "lint/style/no-escaped-quotes"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::LiteralString])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::LiteralString(s) = node else { return };
        let string = s;
        let Some(value) = string.value else { return };
        // HAS_NON_PRINTABLE_CHARS and ESCAPED_CHAR_REGEX (`\n` and `\t` are among the former).
        if value
            .iter()
            .any(|&b| !(0x20..0x80).contains(&b) || b == b'\\' || b == b'$')
        {
            return;
        }
        let count = |q: u8| value.iter().filter(|&&b| b == q).count();
        let (double, single) = (count(b'"'), count(b'\''));
        // php-parser prints the value again in the other kind (`escapeString()`, `pSingleQuotedString()`), which
        // escapes none of these bytes.
        let new = match string.kind {
            LiteralStringKind::SingleQuoted if double == 0 && single > 0 => b'"',
            LiteralStringKind::DoubleQuoted if single == 0 && double > 0 => b'\'',
            _ => return,
        } as char;
        let value = String::from_utf8_lossy(value);
        out.push(Report {
            span: node_span(string),
            message: "Use the other quotes so the string needs no escaped quotes.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(node_span(string), format!("{new}{value}{new}"))],
            }),
        });
    }
}
