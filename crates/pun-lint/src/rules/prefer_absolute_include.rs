//! `lint/complexity/prefer-absolute-include`, ported from Rector 2.6.1's `AbsolutizeRequireAndIncludePathRector`
//! (rules/CodeQuality/Rector/Include_).
//!
//! `require 'autoload.php'` becomes `require __DIR__ . '/autoload.php'`, and `require './a.php'` becomes
//! `require __DIR__ . '/a.php'`: a relative path resolves against the include path and the working directory, an
//! absolute one does not. Paths that are absolute already, `phar://` paths and paths with `config/` stay. A path that
//! starts a concatenation (`'path/' . $name`) gets `__DIR__` too, with only the `phar://` check.
//!
//! The changed string prints again in its own quotes, escaped as php-parser escapes it.

use mago_syntax::cst::{
    BinaryOperator, Construct, Expression, IncludeConstruct, IncludeOnceConstruct, Literal,
    LiteralString, LiteralStringKind, Node, NodeKind, RequireConstruct, RequireOnceConstruct,
};
use pun_problem::{Edit, Fix, Safety};

use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferAbsoluteInclude;

impl Rule for PreferAbsoluteInclude {
    fn id(&self) -> &'static str {
        "lint/complexity/prefer-absolute-include"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Construct])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Construct(c) = node else { return };
        let (Construct::Include(IncludeConstruct { value, .. })
        | Construct::IncludeOnce(IncludeOnceConstruct { value, .. })
        | Construct::Require(RequireConstruct { value, .. })
        | Construct::RequireOnce(RequireOnceConstruct { value, .. })) = c
        else {
            return;
        };
        let (string, path) = match unparenthesize(value) {
            Expression::Binary(concat)
                if matches!(concat.operator, BinaryOperator::StringConcat(_))
                    && string_of(concat.lhs).is_some_and(|(_, v)| !v.starts_with(b"phar://")) =>
            {
                let (string, v) = string_of(concat.lhs).expect("checked above");
                (string, prefix_with_dir_constant(v.to_vec()))
            }
            expr => {
                let Some((string, v)) = string_of(expr) else {
                    return;
                };
                let Some(path) = absolutized(v) else { return };
                (string, path)
            }
        };
        let Some(printed) = print_string(string.kind, &path) else {
            return;
        };
        let span = node_span(string);
        out.push(Report {
            span,
            message: "Make the included path absolute with `__DIR__`.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, format!("__DIR__ . {printed}"))],
            }),
        });
    }
}

/// A php-parser `String_` written in quotes, and its value.
fn string_of<'a>(expr: &'a Expression<'a>) -> Option<(&'a LiteralString<'a>, &'a [u8])> {
    match unparenthesize(expr) {
        Expression::Literal(Literal::String(s)) => Some((s, s.value?)),
        _ => None,
    }
}

/// `refactor()` for a whole string path: the new value, or none for a path that stays.
fn absolutized(value: &[u8]) -> Option<Vec<u8>> {
    // isRefactorableStringPath, then "skip phar"
    if value.starts_with(b"phar://") {
        return None;
    }
    // skip absolute paths
    if value.starts_with(b"/") || value.starts_with(b"\\") {
        return None;
    }
    if value.windows(7).any(|w| w == b"config/") {
        return None;
    }
    // WINDOWS_DRIVE_REGEX `#^[a-zA-z]\:[\/\\\]#`: `A-z` spans the six characters between `Z` and `a` too.
    if let [drive, b':', b'/' | b'\\', ..] = value
        && (b'A'..=b'z').contains(drive)
    {
        return None;
    }
    // add preslash to string
    let value = match value.strip_prefix(b".") {
        Some(rest) if rest.starts_with(b"/") => rest.to_vec(),
        _ => [b"/", value].concat(),
    };
    Some(prefix_with_dir_constant(value))
}

/// `prefixWithDirConstant`'s change to the string: a leading `./` becomes `/`, and a missing leading `/` is added.
fn prefix_with_dir_constant(mut value: Vec<u8>) -> Vec<u8> {
    if value.starts_with(b"./") {
        value.remove(0);
    }
    if !value.starts_with(b"/") {
        value.insert(0, b'/');
    }
    value
}

/// php-parser's `pScalar_String` for a string of `kind` with `value`: `pSingleQuotedString` or `escapeString` in
/// double quotes. A value that is not UTF-8 is left alone (php-parser escapes its bytes one by one).
pub(crate) fn print_string(kind: LiteralStringKind, value: &[u8]) -> Option<String> {
    let value = std::str::from_utf8(value).ok()?;
    let mut out = String::with_capacity(value.len() + 2);
    if kind == LiteralStringKind::SingleQuoted {
        // `'`, a `\` before `'`, `\` or the end, and a `\` after a `\` get a `\` before them.
        let bytes = value.as_bytes();
        out.push('\'');
        for (i, c) in value.char_indices() {
            let escape = c == '\''
                || (c == '\\'
                    && (matches!(bytes.get(i + 1), None | Some(b'\'' | b'\\'))
                        || (i > 0 && bytes[i - 1] == b'\\')));
            if escape {
                out.push('\\');
            }
            out.push(c);
        }
        out.push('\'');
        return Some(out);
    }
    out.push('"');
    for c in value.chars() {
        match c {
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\x0C' => out.push_str("\\f"),
            '\x0B' => out.push_str("\\v"),
            '$' | '"' | '\\' => {
                out.push('\\');
                out.push(c);
            }
            '\0'..='\x08' | '\x0E'..='\x1F' => out.push_str(&format!("\\x{:02x}", c as u32)),
            _ => out.push(c),
        }
    }
    out.push('"');
    Some(out)
}
