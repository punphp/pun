//! `lex_scan()` in parser mode, the way `token_get_all($code, TOKEN_PARSE)` drives it: whitespace,
//! comments and open tags are skipped, `?>` is `';'`, `<?=` is `T_ECHO`, and the scanner's own
//! errors (Zend/zend_language_scanner.l `zend_throw_exception(zend_ce_parse_error, ...)`) are raised
//! when the offending token is lexed. Tokens are produced on demand, so an error after the point where
//! the parser fails (or after `__halt_compiler();`) is never reached, as in PHP.
//!
//! `lineno()` is `CG(zend_lineno)`, the line PHP reports errors on.

use crate::kind::Kind::{self, *};
use crate::lexer::{Lexer, State};

/// A token for the parser. `kind == None` is the single-character token `ch` (`';'` for `?>`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParseToken {
    pub kind: Option<Kind>,
    pub ch: u8,
    pub start: usize,
    pub end: usize,
}

/// A `ParseError` thrown by the scanner.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LexError {
    pub message: String,
    pub line: u32,
}

pub struct ParseLexer<'a> {
    src: &'a [u8],
    lx: Lexer<'a>,
    lineno: u32,
    increment_lineno: bool,
    /// zend_nest_location stack: opening char and its line.
    nesting: Vec<(u8, u32)>,
    /// Per open heredoc/nowdoc: indentation and "uses spaces" of its closing label, as body stripping sees it.
    docs: Vec<(usize, bool)>,
    stopped: bool,
    /// Running as the heredoc scan-ahead (`SCNG(heredoc_scan_ahead)`, not parser mode).
    scan_ahead: bool,
    /// `SCNG(heredoc_indentation)`/`heredoc_indentation_uses_spaces`, set by closing labels during the scan-ahead.
    ahead_indentation: (usize, bool),
}

fn newlines(s: &[u8]) -> u32 {
    (0..s.len()).filter(|&i| s[i] == b'\n' || (s[i] == b'\r' && s.get(i + 1) != Some(&b'\n'))).count() as u32
}

const MIXED: &str = "Invalid indentation - tabs and spaces cannot be mixed";

fn body_indentation_error(indentation: usize) -> String {
    format!("Invalid body indentation level (expecting an indentation level of at least {indentation})")
}

/// Indentation characters: (mixed tabs and spaces, spaces only).
fn spacing(s: &[u8]) -> (bool, bool) {
    let tabs = s.contains(&b'\t');
    let spaces = s.contains(&b' ');
    (tabs && spaces, spaces && !tabs)
}

/// `zend_scan_escape_string`'s errors (`\u{...}` empty, not hex, unterminated, or above U+10FFFF), with the
/// number of newlines before the bad escape.
fn escape_error(s: &[u8]) -> Option<(&'static str, u32)> {
    let mut i = 0;
    while i < s.len() {
        if s[i] == b'\\' && i + 1 < s.len() {
            i += 1;
            if s[i] == b'u' && s.get(i + 1) == Some(&b'{') {
                let digits = s[i + 2..].iter().take_while(|c| c.is_ascii_hexdigit()).count();
                if digits == 0 || s.get(i + 2 + digits) != Some(&b'}') {
                    return Some(("Invalid UTF-8 codepoint escape sequence", newlines(&s[..i])));
                }
                let hex = &s[i + 2..i + 2 + digits];
                let hex = &hex[hex.iter().position(|&c| c != b'0').unwrap_or(hex.len())..];
                if hex.len() > 6 || hex.iter().fold(0u32, |v, &c| v * 16 + (c as char).to_digit(16).unwrap()) > 0x10FFFF {
                    return Some(("Invalid UTF-8 codepoint escape sequence: Codepoint too large", newlines(&s[..i])));
                }
                i += 2 + digits;
            }
        }
        i += 1;
    }
    None
}

fn next_newline(s: &[u8], from: usize) -> Option<(usize, usize)> {
    (from..s.len()).find(|&i| s[i] == b'\r' || s[i] == b'\n').map(|i| (i, if s[i] == b'\r' && s.get(i + 1) == Some(&b'\n') { 2 } else { 1 }))
}

/// `strip_multiline_string_indentation`'s errors, with the number of newlines before the bad line.
fn strip_error(s: &[u8], indentation: usize, using_spaces: bool, newline_at_start: bool, newline_at_end: bool) -> Option<(String, u32)> {
    let end = s.len();
    let mut str = 0;
    let mut count = 0;
    let mut nl = Some((0, 0));
    if !newline_at_start {
        let (p, len) = next_newline(s, 0)?;
        str = p + len;
        count += 1;
    }
    while str <= end && nl.is_some() {
        nl = next_newline(s, str).or(if newline_at_end { Some((end, 0)) } else { None });
        for _ in 0..indentation {
            if nl.is_some_and(|(p, _)| p == str) {
                break;
            }
            if str == end || (s[str] != b' ' && s[str] != b'\t') {
                return Some((body_indentation_error(indentation), count));
            }
            if (s[str] == b' ') != using_spaces {
                return Some((MIXED.to_string(), count));
            }
            str += 1;
        }
        if str == end {
            break;
        }
        str = nl.map_or(end, |(p, len)| p + len);
        count += 1;
    }
    None
}

fn invalid_octal(text: &[u8]) -> bool {
    text[0] == b'0' && text.iter().all(|c| c.is_ascii_digit() || *c == b'_') && text.iter().any(|c| matches!(c, b'8' | b'9'))
}

fn unterminated_comment(text: &[u8]) -> bool {
    text.starts_with(b"/*") && !(text.len() >= 4 && text.ends_with(b"*/"))
}

impl<'a> ParseLexer<'a> {
    pub fn new(src: &'a [u8]) -> Self {
        ParseLexer {
            src,
            lx: Lexer::new(src),
            lineno: 1,
            increment_lineno: false,
            nesting: Vec::new(),
            docs: Vec::new(),
            stopped: false,
            scan_ahead: false,
            ahead_indentation: (0, false),
        }
    }

    /// `CG(zend_lineno)`.
    pub fn lineno(&self) -> u32 {
        self.lineno
    }

    /// `zend_stop_lexing()` (after `__halt_compiler();`): every later token is the end of file.
    pub fn stop(&mut self) {
        self.stopped = true;
    }

    fn err<T>(&self, message: impl Into<String>, line: u32) -> Result<T, LexError> {
        Err(LexError { message: message.into(), line })
    }

    fn bad_nesting(&self, opening: u8, line: u32, closing: Option<u8>) -> LexError {
        let mut m = format!("Unclosed '{}'", opening as char);
        if line != self.lineno {
            m += &format!(" on line {line}");
        }
        if let Some(c) = closing {
            m += &format!(" does not match '{}'", c as char);
        }
        LexError { message: m, line: self.lineno }
    }

    /// The next token, `Ok(None)` at the end of file. Mirrors `zendlex()` + `lex_scan()`.
    pub fn next_token(&mut self) -> Result<Option<ParseToken>, LexError> {
        if self.increment_lineno && !self.scan_ahead {
            self.lineno += 1;
            self.increment_lineno = false;
        }
        loop {
            let Some((kind, start, end)) = (if self.stopped { None } else { self.lx.next() }) else {
                if let Some(&(c, line)) = self.nesting.last() {
                    return Err(self.bad_nesting(c, line, None));
                }
                return Ok(None);
            };
            let state = self.lx.tok_state;
            let text = &self.src[start..end];
            let ch = text.last().copied().unwrap_or(0);
            let tok = ParseToken { kind, ch, start, end };
            match kind {
                Some(T_WHITESPACE | T_OPEN_TAG | T_COMMENT | T_DOC_COMMENT) => {
                    if unterminated_comment(text) {
                        return self.err(format!("Unterminated comment starting line {}", self.lineno), self.lineno);
                    }
                    self.lineno += newlines(text);
                    continue;
                }
                Some(T_CLOSE_TAG) => {
                    self.increment_lineno = text.len() > 2;
                    return Ok(Some(ParseToken { kind: None, ch: b';', ..tok }));
                }
                Some(T_OPEN_TAG_WITH_ECHO) => return Ok(Some(ParseToken { kind: Some(T_ECHO), ..tok })),
                Some(T_LNUMBER | T_DNUMBER) if invalid_octal(text) => return self.err("Invalid numeric literal", self.lineno),
                Some(T_DOUBLE_CAST) if !self.scan_ahead && text[1..].iter().filter(|c| c.is_ascii_alphabetic()).map(u8::to_ascii_lowercase).eq(*b"real") => {
                    return self.err("The (real) cast has been removed, use (float) instead", self.lineno);
                }
                Some(T_CONSTANT_ENCAPSED_STRING) if text[0] == b'"' || (text[0] | 0x20 == b'b' && text[1] == b'"') => {
                    let q = (text[0] != b'"') as usize;
                    if let Some((m, at)) = escape_error(&text[q + 1..text.len() - 1]) {
                        return self.err(m, self.lineno + at);
                    }
                }
                Some(T_ENCAPSED_AND_WHITESPACE) => match state {
                    State::DoubleQuotes | State::Backquote => {
                        if let Some((m, at)) = escape_error(text) {
                            return self.err(m, self.lineno + at);
                        }
                    }
                    // Unterminated single-quoted string: the scanner counts no newlines.
                    State::Scripting => return Ok(Some(tok)),
                    State::Heredoc | State::Nowdoc => return self.doc_body(tok, state),
                    _ => {}
                },
                Some(T_START_HEREDOC) => return self.start_heredoc(tok),
                Some(T_END_HEREDOC) => {
                    self.docs.pop();
                }
                Some(T_ATTRIBUTE) => self.nesting.push((b'[', self.lineno)),
                Some(T_CURLY_OPEN | T_DOLLAR_OPEN_CURLY_BRACES) => self.nesting.push((b'{', self.lineno)),
                None if state == State::Scripting => match ch {
                    b'(' | b'[' | b'{' => self.nesting.push((ch, self.lineno)),
                    b')' | b']' | b'}' => match self.nesting.pop() {
                        None => return self.err(format!("Unmatched '{}'", ch as char), self.lineno),
                        Some((o, line)) if (o == b'{') != (ch == b'}') || (o == b'[') != (ch == b']') => {
                            return Err(self.bad_nesting(o, line, Some(ch)));
                        }
                        Some(_) => {}
                    },
                    _ => {}
                },
                _ => {}
            }
            self.lineno += newlines(text);
            return Ok(Some(tok));
        }
    }

    fn start_heredoc(&mut self, tok: ParseToken) -> Result<Option<ParseToken>, LexError> {
        self.lineno += 1;
        if self.lx.state == State::EndHeredoc {
            let indent = self.lx.labels.last().unwrap().indentation;
            if spacing(&self.src[tok.end..tok.end + indent]).0 {
                return self.err(MIXED, self.lineno);
            }
            self.docs.push((indent, false));
            return Ok(Some(tok));
        }
        if self.scan_ahead || self.src[tok.start..tok.end].contains(&b'\'') {
            self.docs.push((0, false));
            return Ok(Some(tok));
        }
        // Scan ahead (not in parser mode, fresh nesting stack) to the matching T_END_HEREDOC for the
        // closing label's indentation; a scanner exception or the end of file stops the scan.
        let mut ahead = ParseLexer { lx: self.lx.clone(), lineno: self.lineno, scan_ahead: true, ..ParseLexer::new(self.src) };
        let (mut level, mut first) = (1, None);
        while level > 0 {
            let Ok(Some(t)) = ahead.next_token() else {
                break;
            };
            first.get_or_insert(t.kind);
            match t.kind {
                Some(T_START_HEREDOC) => level += 1,
                Some(T_END_HEREDOC) => level -= 1,
                _ => {}
            }
        }
        let (indentation, spaces) = ahead.ahead_indentation;
        if matches!(first, Some(Some(T_VARIABLE | T_DOLLAR_OPEN_CURLY_BRACES | T_CURLY_OPEN))) && indentation > 0 {
            return self.err(body_indentation_error(indentation), self.lineno);
        }
        self.docs.push((indentation, spaces));
        Ok(Some(tok))
    }

    fn doc_body(&mut self, tok: ParseToken, state: State) -> Result<Option<ParseToken>, LexError> {
        let text = &self.src[tok.start..tok.end];
        let closed = self.lx.state == State::EndHeredoc;
        let mut body = text;
        if closed {
            let indent = self.lx.labels.last().unwrap().indentation;
            let (mixed, spaces) = spacing(&self.src[tok.end..tok.end + indent]);
            // The heredoc rule records the label for the scan-ahead even when it throws on mixed indentation.
            if state == State::Heredoc && self.scan_ahead {
                self.ahead_indentation = (indent, spaces);
            }
            if mixed {
                return self.err(MIXED, self.lineno);
            }
            if state == State::Nowdoc {
                *self.docs.last_mut().unwrap() = (indent, spaces);
            }
            body = &text[..text.len() - if text.ends_with(b"\r\n") { 2 } else { 1 }];
            self.increment_lineno = true;
        }
        // The heredoc rule returns early (no stripping) at the end of file after a newline and indentation.
        let eof_after_newline = !closed && tok.end == self.src.len() && text.iter().rev().find(|&&c| c != b' ' && c != b'\t').is_some_and(|&c| c == b'\n' || c == b'\r');
        if !self.scan_ahead && (closed || (state == State::Heredoc && !eof_after_newline)) {
            let (indentation, spaces) = *self.docs.last().unwrap();
            let at_start = matches!(self.src[tok.start - 1], b'\n' | b'\r');
            if let Some((m, at)) = strip_error(body, indentation, spaces, at_start, closed) {
                return self.err(m, self.lineno + at);
            }
            if state == State::Heredoc {
                if let Some((m, at)) = escape_error(body) {
                    return self.err(m, self.lineno + at);
                }
            }
        }
        self.lineno += newlines(body);
        Ok(Some(tok))
    }
}
