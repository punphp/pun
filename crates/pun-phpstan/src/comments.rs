//! `@phpstan-ignore`, `@phpstan-ignore-line` and `@phpstan-ignore-next-line` comments: PHPStan 2.2.7's
//! `RichParser::getLinesToIgnore()` and `IgnoreLexer`, over the file's tokens as nikic/php-parser's lexer produces
//! them (token_get_all plus its sentinel token).

use indexmap::IndexMap;
use pcre2::bytes::Regex;
use pun_tokens::Kind;
use std::sync::LazyLock;

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
    /// The text in parentheses after the identifier.
    pub comment: Option<String>,
}

/// Line => identifiers to ignore there (`None`: every error on the line), in PHPStan's key order.
pub type LinesToIgnore = IndexMap<i64, Option<Vec<Identifier>>>;

/// Line => `@phpstan-ignore` parse errors (reported as "Parse error in @phpstan-ignore: ...").
pub type ParseErrors = IndexMap<i64, Vec<String>>;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(usize)]
enum T {
    Whitespace,
    End,
    Identifier,
    Comma,
    Open,
    Close,
    Other,
}

impl T {
    fn label(self) -> &'static str {
        // IgnoreLexer::LABELS, in TYPES order (whitespace is always allowed, so never named in a message)
        [
            "T_WHITESPACE",
            "end",
            "identifier",
            "comma (,)",
            "T_OPEN_PARENTHESIS",
            "T_CLOSE_PARENTHESIS",
            "T_OTHER",
        ][self as usize]
    }
}

const TYPES: [T; 7] = [
    T::Whitespace,
    T::End,
    T::Identifier,
    T::Comma,
    T::Open,
    T::Close,
    T::Other,
];

static LEXER: LazyLock<Regex> = LazyLock::new(|| {
    crate::php::preg(concat!(
        r"~\G(?:([\x09\x20]++)|(\r?+\n[\x09\x20]*+(?:\*(?!/)\x20?+)?|\*/)|",
        r"([a-zA-Z0-9](?:[a-zA-Z0-9\.]*[a-zA-Z0-9])?)|(,)|(\()|(\))|((?:[^\s\)\(])++))~si"
    ))
    .expect("IgnoreLexer pattern")
});

static PHPDOC_TAG: LazyLock<Regex> = LazyLock::new(|| {
    crate::php::preg(r"~(@(?:[a-z][a-z0-9-\\]+:)?[a-z][a-z0-9-\\]*+)|(@[a-z_\\][a-z0-9_\:\\]*[a-z_][a-z0-9_]*)~si")
        .expect("PHPDoc tag pattern")
});

/// `IgnoreLexer::tokenize()`
fn tokenize(input: &[u8]) -> Vec<(String, T, i64)> {
    let mut tokens = Vec::new();
    let mut line = 1;
    let mut at = 0;
    let mut locs = LEXER.capture_locations();
    while at < input.len() {
        let Ok(Some(m)) = LEXER.captures_read_at(&mut locs, input, at) else {
            break;
        };
        let t = (1..=TYPES.len())
            .find(|&g| locs.get(g).is_some())
            .map_or(T::Other, |g| TYPES[g - 1]);
        tokens.push((
            String::from_utf8_lossy(&input[at..m.end()]).into_owned(),
            t,
            line,
        ));
        at = m.end();
        if t == T::End {
            line += 1;
        }
    }
    if tokens.last().is_none_or(|t| t.1 != T::End) {
        tokens.push((String::new(), T::End, line));
    }
    tokens
}

/// `RichParser::parseIdentifiers()`: `Err((message, line within the comment))`.
fn parse_identifiers(text: &[u8], ignore_pos: usize) -> Result<Vec<Identifier>, (String, i64)> {
    let tokens = tokenize(&text[ignore_pos + "@phpstan-ignore".len()..]);
    let mut identifiers: Vec<Identifier> = Vec::new();
    let mut comment: Option<String> = None;
    let mut open = 0;
    let mut expected: Option<&[T]> = Some(&[T::Identifier]);
    let mut last_label = "@phpstan-ignore";
    let mut previous: Option<T> = None;
    let mut token_line = 1;
    for (content, t, line) in tokens {
        if let Some(p) = previous.filter(|&p| p != T::Whitespace) {
            last_label = p.label();
        }
        previous = Some(t);
        token_line = line;
        if let Some(expected) = expected
            && t != T::Whitespace
            && !expected.contains(&t)
        {
            let other = if t == T::Other {
                format!(" '{content}'")
            } else {
                String::new()
            };
            let labels: Vec<&str> = expected.iter().map(|e| e.label()).collect();
            return Err((
                format!(
                    "Unexpected {}{other} after {last_label}, expected {}",
                    t.label(),
                    labels.join(" or ")
                ),
                line,
            ));
        }
        match t {
            T::Open => {
                if open > 0 {
                    comment.get_or_insert_default().push_str(&content);
                }
                open += 1;
                expected = None;
            }
            T::Close => {
                open -= 1;
                if open == 0 {
                    if let Some(last) = identifiers.last_mut() {
                        last.comment = comment.take();
                    }
                    expected = Some(&[T::Comma, T::End]);
                } else {
                    comment.get_or_insert_default().push_str(&content);
                }
            }
            _ if open > 0 => comment.get_or_insert_default().push_str(&content),
            T::Identifier => {
                identifiers.push(Identifier {
                    name: content,
                    comment: None,
                });
                expected = Some(&[T::Comma, T::End, T::Open]);
            }
            T::Comma => expected = Some(&[T::Identifier]),
            _ => {}
        }
    }
    if open > 0 {
        return Err((
            "Unexpected end, unclosed opening parenthesis".into(),
            token_line,
        ));
    }
    // (PHPStan's "Missing identifier" cannot happen: the first token must already be an identifier.)
    Ok(identifiers)
}

fn newlines(s: &[u8]) -> i64 {
    s.iter().filter(|&&c| c == b'\n').count() as i64
}

fn find(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    // the needle's first byte, then the rest: lines_to_ignore searches every file
    let mut at = from;
    loop {
        at += haystack.get(at..)?.iter().position(|&c| c == needle[0])?;
        if haystack[at..].starts_with(needle) {
            return Some(at);
        }
        at += 1;
    }
}

/// `$lines += [$line => null]`
fn add_null(lines: &mut LinesToIgnore, line: i64) {
    lines.entry(line).or_insert(None);
}

fn add_identifiers(lines: &mut LinesToIgnore, line: i64, ids: Vec<Identifier>) {
    lines
        .entry(line)
        .or_insert(None)
        .get_or_insert_default()
        .extend(ids);
}

/// `getLinesToIgnoreForTokenByIgnoreComment()`, added with `+=`.
fn by_ignore_comment(
    lines: &mut LinesToIgnore,
    text: &[u8],
    line: i64,
    comment: &str,
    next_line: bool,
) {
    let mut offset = 0;
    while let Some(pos) = find(text, comment.as_bytes(), offset) {
        add_null(lines, line + newlines(&text[..pos]) + i64::from(next_line));
        offset = pos + 1;
    }
}

/// The ignore comments of a PHP file that parsed (PHPStan only reads them from a successful parse).
pub fn lines_to_ignore(code: &[u8]) -> (LinesToIgnore, ParseErrors) {
    // every line and error below comes from an `@phpstan-ignore` in a comment; most files have none and skip the lexer
    if find(code, b"@phpstan-ignore", 0).is_none() {
        return (LinesToIgnore::new(), ParseErrors::new());
    }
    let mut tokens: Vec<(Option<Kind>, &[u8], i64)> = pun_tokens::tokenize(code)
        .into_iter()
        .map(|t| (t.kind, t.text, i64::from(t.line)))
        .collect();
    // php-parser's sentinel token, on the last token's end line.
    let end_line = tokens
        .last()
        .map_or(1, |&(_, text, line)| line + newlines(text));
    tokens.push((None, b"\0", end_line));

    let mut lines = LinesToIgnore::new();
    let mut errors: Vec<(i64, String)> = Vec::new();
    let mut previous_line: Option<i64> = None;
    // (comment text, ignore position, line of the tag, line the comment ends on)
    let mut pending: Option<(&[u8], usize, i64, i64)> = None;
    for (kind, text, token_line) in tokens {
        let mut line = token_line;
        if kind != Some(Kind::T_COMMENT) && kind != Some(Kind::T_DOC_COMMENT) {
            if kind != Some(Kind::T_WHITESPACE) {
                if let Some((p_text, p_pos, p_tag_line, p_line)) = pending.take() {
                    match parse_identifiers(p_text, p_pos) {
                        Err((message, at)) => {
                            errors.push((p_tag_line + at, message));
                            continue;
                        }
                        Ok(ids) => add_identifiers(
                            &mut lines,
                            if line != p_line + 1 { p_line } else { line },
                            ids,
                        ),
                    }
                }
                previous_line = Some(line);
            }
            continue;
        }

        let mut text = text;
        let is_next_line = find(text, b"@phpstan-ignore-next-line", 0).is_some();
        let is_current_line = find(text, b"@phpstan-ignore-line", 0).is_some();
        if kind == Some(Kind::T_DOC_COMMENT) {
            by_ignore_comment(&mut lines, text, line, "@phpstan-ignore-line", false);
            if is_next_line {
                if let Some(last) = PHPDOC_TAG.find_iter(text).filter_map(Result::ok).last()
                    && last.as_bytes() == b"@phpstan-ignore-next-line"
                {
                    // Ignores the line after the PHPDoc, and cuts the PHPDoc before the tag.
                    lines.insert(line + 1 + newlines(text), None);
                    text = &text[..last.start()];
                }
                by_ignore_comment(&mut lines, text, line, "@phpstan-ignore-next-line", true);
            }
            if is_next_line || is_current_line {
                continue;
            }
        } else {
            if is_next_line {
                line += 1;
            }
            if is_next_line || is_current_line {
                line += newlines(text);
                lines.insert(line, None);
                continue;
            }
        }

        let Some(ignore_pos) = find(text, b"@phpstan-ignore", 0) else {
            continue;
        };
        let ignore_line = newlines(&text[..ignore_pos]) - 1;
        if previous_line == Some(line) {
            match parse_identifiers(text, ignore_pos) {
                Ok(ids) => add_identifiers(&mut lines, line, ids),
                Err((message, at)) => errors.push((token_line + at + ignore_line, message)),
            }
            continue;
        }
        line += newlines(text);
        pending = Some((text, ignore_pos, token_line + ignore_line, line));
    }

    let mut processed = ParseErrors::new();
    for (line, message) in errors {
        processed.entry(line).or_default().push(message);
    }
    (lines, processed)
}
