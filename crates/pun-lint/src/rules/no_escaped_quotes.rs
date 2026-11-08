//! `lint/style/no-escaped-quotes`, ported from Rector 2.8.4's `SimplifyQuoteEscapeRector`
//! (rules/CodingStyle/Rector/String_).
//!
//! A single-quoted string that holds `'` but no `"` becomes double-quoted, and a double-quoted one that holds `"` but
//! no `'` becomes single-quoted, so the quotes inside need no escape. A string with a backslash, a `$`, a control
//! byte (a newline, a tab) or bytes above ASCII that are not valid UTF-8 stays, and so does a double-quoted one written
//! with an escape other than `\"` (`"\u{00A0}"`).

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
        let count = |q: u8| value.iter().filter(|&&b| b == q).count();
        let (double, single) = (count(b'"'), count(b'\''));
        // no quotes to simplify
        if double == 0 && single == 0 {
            return;
        }
        // `hasNonPrintableChars`: a control byte, or a byte above ASCII outside valid UTF-8 text.
        if value.iter().any(|&b| b < 0x20)
            || (value.iter().any(|&b| b >= 0x80) && std::str::from_utf8(value).is_err())
        {
            return;
        }
        // ESCAPED_CHAR_REGEX: chars that would be newly escaped (`\n` and `\t` are control bytes above).
        if value.iter().any(|&b| b == b'\\' || b == b'$') {
            return;
        }
        // php-parser prints the value again in the other kind (`escapeString()`, `pSingleQuotedString()`), which
        // escapes none of these bytes.
        let new = match string.kind {
            LiteralStringKind::SingleQuoted if double == 0 && single > 0 => b'"',
            // `hasEscapeSequence`: any escape other than `\"` (`"\u{00A0}"`) would be printed as the raw char.
            LiteralStringKind::DoubleQuoted if single == 0 && double > 0 => {
                let raw = string.raw;
                let inner = String::from_utf8_lossy(&raw[1..raw.len() - 1]).replace("\\\"", "\"");
                if inner.as_bytes() != value {
                    return;
                }
                b'\''
            }
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
