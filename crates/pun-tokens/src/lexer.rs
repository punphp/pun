//! A port of PHP 8.5's `Zend/zend_language_scanner.l` as driven by `token_get_all()`
//! (ext/tokenizer, no `TOKEN_PARSE`): same states, same longest-match rules, same
//! `__halt_compiler` handling. `short_open_tag` is assumed off (PHP's php.ini default).
//! See context/reference/tokens.md for why this is not built on mago-syntax's lexer.

use crate::kind::Kind::{self, *};

/// One element of `token_get_all()`'s result. `kind == None` is PHP's plain-string token
/// (`';'`, `'('`, `'"'`, also `'b"'` for a binary interpolated string).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawToken<'a> {
    pub kind: Option<Kind>,
    pub text: &'a [u8],
    pub line: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum State {
    Initial,
    Scripting,
    LookingForProperty,
    DoubleQuotes,
    Backquote,
    Heredoc,
    Nowdoc,
    EndHeredoc,
    LookingForVarname,
    VarOffset,
}

#[derive(Clone)]
pub(crate) struct Label {
    start: usize,
    end: usize,
    pub(crate) indentation: usize,
}

#[derive(Clone)]
pub(crate) struct Lexer<'a> {
    src: &'a [u8],
    pos: usize,
    pub(crate) state: State,
    stack: Vec<State>,
    pub(crate) labels: Vec<Label>,
    /// State that produced the last token.
    pub(crate) tok_state: State,
}

#[inline]
fn is_label_start(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_' || c >= 0x80
}

#[inline]
fn is_label_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_' || c >= 0x80
}

#[inline]
fn is_ws(c: u8) -> bool {
    matches!(c, b' ' | b'\n' | b'\r' | b'\t')
}

/// Tokenize like `token_get_all($src)`.
pub fn tokenize(src: &[u8]) -> Vec<RawToken<'_>> {
    let mut lx = Lexer::new(src);
    let mut spans: Vec<(Option<Kind>, usize, usize)> = Vec::with_capacity(src.len() / 4);
    // Tokens still to read after `__halt_compiler` (PHP's `need_tokens`, -1 there until it is seen).
    let mut need_tokens: Option<u32> = None;
    let mut halted = false;
    while let Some((kind, start, end)) = lx.next() {
        spans.push((kind, start, end));
        if let Some(need) = need_tokens.as_mut() {
            if !matches!(
                kind,
                Some(T_WHITESPACE | T_OPEN_TAG | T_COMMENT | T_DOC_COMMENT)
            ) {
                *need -= 1;
                if *need == 0 {
                    if end < src.len() {
                        spans.push((Some(T_INLINE_HTML), end, src.len()));
                        halted = true;
                    }
                    break;
                }
            }
        } else if kind == Some(T_HALT_COMPILER) {
            need_tokens = Some(3);
        }
    }

    // Line of a token = 1 + newlines before it; "\r\n" counts once, lone "\r" counts
    // (HANDLE_NEWLINES).
    let mut line = 1u32;
    let mut counted = 0usize;
    let mut out: Vec<RawToken> = spans
        .into_iter()
        .map(|(kind, start, end)| {
            for i in counted..start {
                let c = src[i];
                if c == b'\n' || (c == b'\r' && src.get(i + 1) != Some(&b'\n')) {
                    line += 1;
                }
            }
            counted = counted.max(start);
            RawToken {
                kind,
                text: &src[start..end],
                line,
            }
        })
        .collect();
    // The post-__halt_compiler T_INLINE_HTML reuses the third token's start line.
    if halted {
        let n = out.len();
        out[n - 1].line = out[n - 2].line;
    }
    out
}

fn keyword(lower: &[u8]) -> Option<Kind> {
    Some(match lower {
        b"exit" | b"die" => T_EXIT,
        b"fn" => T_FN,
        b"function" => T_FUNCTION,
        b"const" => T_CONST,
        b"return" => T_RETURN,
        b"try" => T_TRY,
        b"catch" => T_CATCH,
        b"finally" => T_FINALLY,
        b"throw" => T_THROW,
        b"if" => T_IF,
        b"elseif" => T_ELSEIF,
        b"endif" => T_ENDIF,
        b"else" => T_ELSE,
        b"while" => T_WHILE,
        b"endwhile" => T_ENDWHILE,
        b"do" => T_DO,
        b"for" => T_FOR,
        b"endfor" => T_ENDFOR,
        b"foreach" => T_FOREACH,
        b"endforeach" => T_ENDFOREACH,
        b"declare" => T_DECLARE,
        b"enddeclare" => T_ENDDECLARE,
        b"instanceof" => T_INSTANCEOF,
        b"as" => T_AS,
        b"switch" => T_SWITCH,
        b"match" => T_MATCH,
        b"endswitch" => T_ENDSWITCH,
        b"case" => T_CASE,
        b"default" => T_DEFAULT,
        b"break" => T_BREAK,
        b"continue" => T_CONTINUE,
        b"goto" => T_GOTO,
        b"echo" => T_ECHO,
        b"print" => T_PRINT,
        b"class" => T_CLASS,
        b"interface" => T_INTERFACE,
        b"trait" => T_TRAIT,
        b"extends" => T_EXTENDS,
        b"implements" => T_IMPLEMENTS,
        b"new" => T_NEW,
        b"clone" => T_CLONE,
        b"var" => T_VAR,
        b"eval" => T_EVAL,
        b"include" => T_INCLUDE,
        b"include_once" => T_INCLUDE_ONCE,
        b"require" => T_REQUIRE,
        b"require_once" => T_REQUIRE_ONCE,
        b"namespace" => T_NAMESPACE,
        b"use" => T_USE,
        b"insteadof" => T_INSTEADOF,
        b"global" => T_GLOBAL,
        b"isset" => T_ISSET,
        b"empty" => T_EMPTY,
        b"__halt_compiler" => T_HALT_COMPILER,
        b"static" => T_STATIC,
        b"abstract" => T_ABSTRACT,
        b"final" => T_FINAL,
        b"private" => T_PRIVATE,
        b"protected" => T_PROTECTED,
        b"public" => T_PUBLIC,
        b"readonly" => T_READONLY,
        b"unset" => T_UNSET,
        b"list" => T_LIST,
        b"array" => T_ARRAY,
        b"callable" => T_CALLABLE,
        b"or" => T_LOGICAL_OR,
        b"and" => T_LOGICAL_AND,
        b"xor" => T_LOGICAL_XOR,
        b"__class__" => T_CLASS_C,
        b"__trait__" => T_TRAIT_C,
        b"__function__" => T_FUNC_C,
        b"__property__" => T_PROPERTY_C,
        b"__method__" => T_METHOD_C,
        b"__line__" => T_LINE,
        b"__file__" => T_FILE,
        b"__dir__" => T_DIR,
        b"__namespace__" => T_NS_C,
        _ => return None,
    })
}

fn cast(lower: &[u8]) -> Option<Kind> {
    Some(match lower {
        b"int" | b"integer" => T_INT_CAST,
        b"float" | b"double" | b"real" => T_DOUBLE_CAST,
        b"string" | b"binary" => T_STRING_CAST,
        b"array" => T_ARRAY_CAST,
        b"object" => T_OBJECT_CAST,
        b"bool" | b"boolean" => T_BOOL_CAST,
        b"unset" => T_UNSET_CAST,
        b"void" => T_VOID_CAST,
        _ => return None,
    })
}

type Tok = (Option<Kind>, usize, usize);

impl<'a> Lexer<'a> {
    pub(crate) fn new(src: &'a [u8]) -> Self {
        Lexer {
            src,
            pos: 0,
            state: State::Initial,
            stack: Vec::new(),
            labels: Vec::new(),
            tok_state: State::Initial,
        }
    }

    #[inline]
    fn at(&self, i: usize) -> u8 {
        // Like the scanner's NUL padding past the end.
        *self.src.get(i).unwrap_or(&0)
    }

    fn starts_with_ci(&self, i: usize, lit: &[u8]) -> bool {
        self.src.len() >= i + lit.len() && self.src[i..i + lit.len()].eq_ignore_ascii_case(lit)
    }

    fn label_end(&self, mut i: usize) -> usize {
        while i < self.src.len() && is_label_char(self.src[i]) {
            i += 1;
        }
        i
    }

    fn push(&mut self, s: State) {
        self.stack.push(self.state);
        self.state = s;
    }

    fn pop(&mut self) {
        // yy_pop_state on an empty stack never happens in valid scanner flows except `}`,
        // which guards it; keep the current state in that case.
        if let Some(s) = self.stack.pop() {
            self.state = s;
        }
    }

    pub(crate) fn next(&mut self) -> Option<Tok> {
        loop {
            let p = self.pos;
            if p >= self.src.len() {
                return None;
            }
            self.tok_state = self.state;
            let r = match self.state {
                State::Initial => Some(self.initial(p)),
                State::Scripting => Some(self.scripting(p)),
                State::LookingForProperty => self.looking_for_property(p),
                State::DoubleQuotes => Some(self.quoted(p, b'"')),
                State::Backquote => Some(self.quoted(p, b'`')),
                State::Heredoc => Some(self.heredoc(p)),
                State::Nowdoc => Some(self.nowdoc(p)),
                State::EndHeredoc => {
                    let l = self.labels.pop().expect("heredoc label");
                    self.state = State::Scripting;
                    let end = (p + l.indentation + l.end - l.start).min(self.src.len());
                    Some((Some(T_END_HEREDOC), end))
                }
                State::LookingForVarname => {
                    let e = self.label_end(p);
                    self.pop();
                    self.push(State::Scripting);
                    if is_label_start(self.at(p)) && matches!(self.at(e), b'[' | b'}') {
                        Some((Some(T_STRING_VARNAME), e))
                    } else {
                        None
                    }
                }
                State::VarOffset => Some(self.var_offset(p)),
            };
            if let Some((kind, end)) = r {
                self.pos = end;
                return Some((kind, p, end));
            }
        }
    }

    fn initial(&mut self, p: usize) -> (Option<Kind>, usize) {
        let n = self.src.len();
        let mut cur = p + 1;
        if self.at(p) == b'<' && self.at(p + 1) == b'?' {
            if self.at(p + 2) == b'=' {
                self.state = State::Scripting;
                return (Some(T_OPEN_TAG_WITH_ECHO), p + 3);
            }
            if self.starts_with_ci(p + 2, b"php") {
                let e = p + 5;
                match self.at(e) {
                    b' ' | b'\t' | b'\n' => {
                        self.state = State::Scripting;
                        return (Some(T_OPEN_TAG), e + 1);
                    }
                    b'\r' => {
                        self.state = State::Scripting;
                        return (
                            Some(T_OPEN_TAG),
                            if self.at(e + 1) == b'\n' {
                                e + 2
                            } else {
                                e + 1
                            },
                        );
                    }
                    _ if e == n => {
                        self.state = State::Scripting;
                        return (Some(T_OPEN_TAG), e);
                    }
                    _ => cur = e,
                }
            } else {
                cur = p + 2;
            }
        }
        // inline_char_handler
        loop {
            match self.src[cur.min(n)..].iter().position(|&c| c == b'<') {
                Some(off) => cur = cur + off + 1,
                None => cur = n,
            }
            if cur >= n {
                break;
            }
            if self.src[cur] == b'?'
                && (self.at(cur + 1) == b'='
                    || (self.starts_with_ci(cur + 1, b"php")
                        && (cur + 4 == n
                            || matches!(self.at(cur + 4), b' ' | b'\t' | b'\n' | b'\r'))))
            {
                cur -= 1;
                break;
            }
        }
        (Some(T_INLINE_HTML), cur.min(n))
    }

    /// End positions reachable by `{WHITESPACE_OR_COMMENTS}` from `i` (excluding `i`).
    fn woc_ends(&self, i: usize) -> Vec<usize> {
        let n = self.src.len();
        let mut seen = vec![i];
        let mut out = Vec::new();
        // `out` as a set: a whitespace run of n bytes has n ends, so `out.contains` made this quadratic.
        let mut found = std::collections::HashSet::new();
        while let Some(q) = seen.pop() {
            let mut next = Vec::new();
            let c = self.at(q);
            if q < n && is_ws(c) {
                next.push(q + 1);
            } else if c == b'/' && self.at(q + 1) == b'*' {
                // "/*" ... first "*/", no NUL.
                let mut k = q + 2;
                while k + 1 < n
                    && !(self.src[k] == b'*' && self.src[k + 1] == b'/')
                    && self.src[k] != 0
                {
                    k += 1;
                }
                if k + 1 < n && self.src[k] == b'*' && self.src[k + 1] == b'/' {
                    next.push(k + 2);
                }
            } else if c == b'/' && self.at(q + 1) == b'/' {
                let mut k = q + 2;
                while k < n && !matches!(self.src[k], 0 | b'\n' | b'\r') {
                    k += 1;
                }
                if k < n && matches!(self.src[k], b'\n' | b'\r') {
                    next.push(k + 1);
                }
            } else if c == b'#' && q < n {
                let d = self.at(q + 1);
                if q + 1 < n && matches!(d, b'\n' | b'\r') {
                    next.push(q + 2);
                }
                if q + 1 < n && d != b'[' && d != 0 {
                    let mut k = q + 2;
                    while k < n && !matches!(self.src[k], 0 | b'\n' | b'\r') {
                        k += 1;
                    }
                    if k < n && matches!(self.src[k], b'\n' | b'\r') {
                        next.push(k + 1);
                    }
                }
            }
            for e in next {
                if found.insert(e) {
                    out.push(e);
                    seen.push(e);
                }
            }
        }
        out
    }

    fn scripting(&mut self, p: usize) -> (Option<Kind>, usize) {
        let n = self.src.len();
        let c = self.src[p];
        if is_ws(c) {
            let mut e = p + 1;
            while e < n && is_ws(self.src[e]) {
                e += 1;
            }
            return (Some(T_WHITESPACE), e);
        }
        if is_label_start(c) {
            return self.identifier(p);
        }
        if c.is_ascii_digit() || (c == b'.' && self.at(p + 1).is_ascii_digit()) {
            return self.number(p);
        }
        match c {
            b'$' if is_label_start(self.at(p + 1)) => (Some(T_VARIABLE), self.label_end(p + 1)),
            b'\\' => {
                if is_label_start(self.at(p + 1)) {
                    (
                        Some(T_NAME_FULLY_QUALIFIED),
                        self.qualified_end(self.label_end(p + 1)),
                    )
                } else {
                    (Some(T_NS_SEPARATOR), p + 1)
                }
            }
            b'#' if self.at(p + 1) == b'[' => (Some(T_ATTRIBUTE), p + 2),
            b'#' => (Some(T_COMMENT), self.line_comment_end(p + 1)),
            b'/' if self.at(p + 1) == b'/' => (Some(T_COMMENT), self.line_comment_end(p + 2)),
            b'/' if self.at(p + 1) == b'*' => self.block_comment(p),
            b'?' if self.at(p + 1) == b'>' => {
                self.state = State::Initial;
                let e = p + 2;
                let e = match self.at(e) {
                    b'\n' if e < n => e + 1,
                    b'\r' if e < n => {
                        if self.at(e + 1) == b'\n' {
                            e + 2
                        } else {
                            e + 1
                        }
                    }
                    _ => e,
                };
                (Some(T_CLOSE_TAG), e)
            }
            b'(' => {
                let mut i = p + 1;
                while matches!(self.at(i), b' ' | b'\t') {
                    i += 1;
                }
                let ws = i;
                while self.at(i).is_ascii_alphabetic() {
                    i += 1;
                }
                if i > ws && i - ws <= 7 {
                    let lower = self.src[ws..i].to_ascii_lowercase();
                    if let Some(k) = cast(&lower) {
                        while matches!(self.at(i), b' ' | b'\t') {
                            i += 1;
                        }
                        if self.at(i) == b')' && i < n {
                            return (Some(k), i + 1);
                        }
                    }
                }
                (None, p + 1)
            }
            b'\'' => self.single_quoted(p, 0),
            b'"' => self.double_quoted(p, 0),
            b'`' => {
                self.state = State::Backquote;
                (None, p + 1)
            }
            b'<' if self.at(p + 1) == b'<' && self.at(p + 2) == b'<' => {
                if let Some(e) = self.heredoc_start(p, 0) {
                    return (Some(T_START_HEREDOC), e);
                }
                self.operator(p)
            }
            b'&' => {
                if matches!(self.at(p + 1), b'&' | b'=') {
                    return self.operator(p);
                }
                let followed = |q: usize| {
                    self.at(q) == b'$'
                        || (self.at(q) == b'.' && self.at(q + 1) == b'.' && self.at(q + 2) == b'.')
                };
                if followed(p + 1) || self.woc_ends(p + 1).into_iter().any(followed) {
                    (Some(T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG), p + 1)
                } else {
                    (Some(T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG), p + 1)
                }
            }
            b'{' => {
                self.push(State::Scripting);
                (None, p + 1)
            }
            b'}' => {
                if !self.stack.is_empty() {
                    self.pop();
                }
                (None, p + 1)
            }
            _ => self.operator(p),
        }
    }

    fn operator(&mut self, p: usize) -> (Option<Kind>, usize) {
        let a = self.at(p);
        let b = self.at(p + 1);
        let c = self.at(p + 2);
        let three = match (a, b, c) {
            (b'=', b'=', b'=') => Some(T_IS_IDENTICAL),
            (b'!', b'=', b'=') => Some(T_IS_NOT_IDENTICAL),
            (b'<', b'=', b'>') => Some(T_SPACESHIP),
            (b'*', b'*', b'=') => Some(T_POW_EQUAL),
            (b'.', b'.', b'.') => Some(T_ELLIPSIS),
            (b'<', b'<', b'=') => Some(T_SL_EQUAL),
            (b'>', b'>', b'=') => Some(T_SR_EQUAL),
            (b'?', b'?', b'=') => Some(T_COALESCE_EQUAL),
            (b'?', b'-', b'>') => Some(T_NULLSAFE_OBJECT_OPERATOR),
            _ => None,
        };
        if let Some(k) = three {
            if k == T_NULLSAFE_OBJECT_OPERATOR {
                self.push(State::LookingForProperty);
            }
            return (Some(k), p + 3);
        }
        let two = match (a, b) {
            (b'-', b'>') => Some(T_OBJECT_OPERATOR),
            (b':', b':') => Some(T_DOUBLE_COLON),
            (b'?', b'?') => Some(T_COALESCE),
            (b'=', b'>') => Some(T_DOUBLE_ARROW),
            (b'+', b'+') => Some(T_INC),
            (b'-', b'-') => Some(T_DEC),
            (b'=', b'=') => Some(T_IS_EQUAL),
            (b'!', b'=') | (b'<', b'>') => Some(T_IS_NOT_EQUAL),
            (b'<', b'=') => Some(T_IS_SMALLER_OR_EQUAL),
            (b'>', b'=') => Some(T_IS_GREATER_OR_EQUAL),
            (b'+', b'=') => Some(T_PLUS_EQUAL),
            (b'-', b'=') => Some(T_MINUS_EQUAL),
            (b'*', b'=') => Some(T_MUL_EQUAL),
            (b'*', b'*') => Some(T_POW),
            (b'/', b'=') => Some(T_DIV_EQUAL),
            (b'.', b'=') => Some(T_CONCAT_EQUAL),
            (b'%', b'=') => Some(T_MOD_EQUAL),
            (b'&', b'=') => Some(T_AND_EQUAL),
            (b'|', b'=') => Some(T_OR_EQUAL),
            (b'^', b'=') => Some(T_XOR_EQUAL),
            (b'|', b'>') => Some(T_PIPE),
            (b'|', b'|') => Some(T_BOOLEAN_OR),
            (b'&', b'&') => Some(T_BOOLEAN_AND),
            (b'<', b'<') => Some(T_SL),
            (b'>', b'>') => Some(T_SR),
            _ => None,
        };
        if let Some(k) = two {
            if k == T_OBJECT_OPERATOR {
                self.push(State::LookingForProperty);
            }
            return (Some(k), p + 2);
        }
        if b";:,.|^&+-/*=%!~$<>?@[]()".contains(&a) {
            (None, p + 1)
        } else {
            (Some(T_BAD_CHARACTER), p + 1)
        }
    }

    fn qualified_end(&self, mut e: usize) -> usize {
        while self.at(e) == b'\\' && is_label_start(self.at(e + 1)) {
            e = self.label_end(e + 1);
        }
        e
    }

    fn identifier(&mut self, p: usize) -> (Option<Kind>, usize) {
        let e = self.label_end(p);
        let word = &self.src[p..e];
        if word.len() == 1 && (word[0] | 0x20) == b'b' {
            match self.at(e) {
                b'\'' => return self.single_quoted(p, 1),
                b'"' => return self.double_quoted(p, 1),
                b'<' if self.at(e + 1) == b'<' && self.at(e + 2) == b'<' => {
                    if let Some(end) = self.heredoc_start(p, 1) {
                        return (Some(T_START_HEREDOC), end);
                    }
                }
                _ => {}
            }
        }
        if self.at(e) == b'\\' && is_label_start(self.at(e + 1)) {
            let q = self.qualified_end(e);
            let kind = if word.eq_ignore_ascii_case(b"namespace") {
                T_NAME_RELATIVE
            } else {
                T_NAME_QUALIFIED
            };
            return (Some(kind), q);
        }
        if word.len() > 15 {
            return (Some(T_STRING), e);
        }
        let mut buf = [0u8; 15];
        buf[..word.len()].copy_from_slice(word);
        buf[..word.len()].make_ascii_lowercase();
        let lower = &buf[..word.len()];
        match lower {
            b"yield" => {
                let best = self
                    .woc_ends(e)
                    .into_iter()
                    .filter(|&q| self.starts_with_ci(q, b"from") && !is_label_char(self.at(q + 4)))
                    .max();
                if let Some(q) = best {
                    return (Some(T_YIELD_FROM), q + 4);
                }
                return (Some(T_YIELD), e);
            }
            b"enum" => {
                let (mut as_string, mut as_enum) = (None, None);
                for q in self.woc_ends(e) {
                    for kw in [&b"extends"[..], b"implements"] {
                        if self.starts_with_ci(q, kw) {
                            as_string = as_string.max(Some(q + kw.len()));
                        }
                    }
                    if q < self.src.len()
                        && (self.src[q].is_ascii_alphabetic()
                            || self.src[q] == b'_'
                            || self.src[q] >= 0x80)
                    {
                        as_enum = as_enum.max(Some(q + 1));
                    }
                }
                return match (as_string, as_enum) {
                    (Some(s), en) if Some(s) >= en => (Some(T_STRING), e),
                    (_, Some(_)) => (Some(T_ENUM), e),
                    _ => (Some(T_STRING), e),
                };
            }
            b"public" | b"protected" | b"private" if self.starts_with_ci(e, b"(set)") => {
                let k = match lower[2] {
                    b'b' => T_PUBLIC_SET,
                    b'o' => T_PROTECTED_SET,
                    _ => T_PRIVATE_SET,
                };
                return (Some(k), e + 5);
            }
            _ => {}
        }
        (Some(keyword(lower).unwrap_or(T_STRING)), e)
    }

    fn lnum_end(&self, i: usize) -> Option<usize> {
        self.digits_end(i, |c| c.is_ascii_digit())
    }

    /// `[d]+(_[d]+)*`
    fn digits_end(&self, mut i: usize, d: impl Fn(u8) -> bool) -> Option<usize> {
        if !d(self.at(i)) || i >= self.src.len() {
            return None;
        }
        loop {
            while i < self.src.len() && d(self.src[i]) {
                i += 1;
            }
            if self.at(i) == b'_' && i + 1 < self.src.len() && d(self.src[i + 1]) {
                i += 1;
            } else {
                return Some(i);
            }
        }
    }

    /// Longest of HNUM / BNUM / ONUM starting at `p`, if any: (end, base).
    fn prefixed_number(&self, p: usize) -> Option<(usize, u32)> {
        if self.at(p) != b'0' {
            return None;
        }
        let (base, d): (u32, fn(u8) -> bool) = match self.at(p + 1) | 0x20 {
            b'x' => (16, |c: u8| c.is_ascii_hexdigit()),
            b'b' => (2, |c: u8| c == b'0' || c == b'1'),
            b'o' => (8, |c: u8| (b'0'..=b'7').contains(&c)),
            _ => return None,
        };
        self.digits_end(p + 2, d).map(|e| (e, base))
    }

    fn number(&mut self, p: usize) -> (Option<Kind>, usize) {
        let lnum = self.lnum_end(p);
        let mut best: (usize, Kind) = (p, T_LNUMBER);
        if let Some((e, base)) = self.prefixed_number(p) {
            best = (e, prefixed_kind(&self.src[p + 2..e], base));
        }
        if let Some(e) = lnum
            && e > best.0
        {
            best = (e, decimal_kind(&self.src[p..e]));
        }
        // DNUM: ({LNUM}?"."{LNUM}) | ({LNUM}"."{LNUM}?)
        let dot = lnum.unwrap_or(p);
        let mut dnum = None;
        if self.at(dot) == b'.' && dot < self.src.len() {
            if let Some(e) = self.lnum_end(dot + 1) {
                dnum = Some(e);
            } else if lnum.is_some() {
                dnum = Some(dot + 1);
            }
        }
        if let Some(e) = dnum
            && e > best.0
        {
            best = (e, T_DNUMBER);
        }
        // EXPONENT_DNUM: ({LNUM}|{DNUM})[eE][+-]?{LNUM}
        for base_end in [lnum, dnum].into_iter().flatten() {
            if self.at(base_end) | 0x20 == b'e' && base_end < self.src.len() {
                let mut i = base_end + 1;
                if matches!(self.at(i), b'+' | b'-') {
                    i += 1;
                }
                if let Some(e) = self.lnum_end(i)
                    && e > best.0
                {
                    best = (e, T_DNUMBER);
                }
            }
        }
        (Some(best.1), best.0)
    }

    fn line_comment_end(&self, mut i: usize) -> usize {
        let n = self.src.len();
        while i < n {
            match self.src[i] {
                b'\r' | b'\n' => break,
                b'?' if self.at(i + 1) == b'>' && i + 1 < n => break,
                _ => i += 1,
            }
        }
        i
    }

    fn block_comment(&self, p: usize) -> (Option<Kind>, usize) {
        let n = self.src.len();
        let mut cur = p + 2;
        let mut doc = false;
        if self.at(cur) == b'*' && cur < n && is_ws(self.at(cur + 1)) && cur + 1 < n {
            doc = true;
            cur += 1;
            while cur < n && is_ws(self.src[cur]) {
                cur += 1;
            }
        }
        while cur < n {
            let c = self.src[cur];
            cur += 1;
            if c == b'*' && self.at(cur) == b'/' && cur < n {
                break;
            }
        }
        if cur < n {
            cur += 1;
        }
        (Some(if doc { T_DOC_COMMENT } else { T_COMMENT }), cur)
    }

    fn single_quoted(&self, p: usize, bprefix: usize) -> (Option<Kind>, usize) {
        let n = self.src.len();
        let mut cur = p + bprefix + 1;
        loop {
            if cur < n {
                if self.src[cur] == b'\'' {
                    return (Some(T_CONSTANT_ENCAPSED_STRING), cur + 1);
                }
                let c = self.src[cur];
                cur += 1;
                if c == b'\\' && cur < n {
                    cur += 1;
                }
            } else {
                return (Some(T_ENCAPSED_AND_WHITESPACE), n);
            }
        }
    }

    fn double_quoted(&mut self, p: usize, bprefix: usize) -> (Option<Kind>, usize) {
        let n = self.src.len();
        let mut cur = p + bprefix + 1;
        while cur < n {
            let c = self.src[cur];
            cur += 1;
            match c {
                b'"' => return (Some(T_CONSTANT_ENCAPSED_STRING), cur),
                b'$' if is_label_start(self.at(cur)) || self.at(cur) == b'{' => break,
                b'{' if self.at(cur) == b'$' => break,
                b'\\' if cur < n => cur += 1,
                _ => {}
            }
        }
        self.state = State::DoubleQuotes;
        (None, p + bprefix + 1)
    }

    /// `b?"<<<"{TABS_AND_SPACES}({LABEL}|'{LABEL}'|"{LABEL}"){NEWLINE}`; on match pushes the
    /// label, switches state and returns the token end.
    fn heredoc_start(&mut self, p: usize, bprefix: usize) -> Option<usize> {
        let n = self.src.len();
        let mut i = p + bprefix + 3;
        while matches!(self.at(i), b' ' | b'\t') {
            i += 1;
        }
        let quote = match self.at(i) {
            q @ (b'\'' | b'"') => {
                i += 1;
                Some(q)
            }
            _ => None,
        };
        if !is_label_start(self.at(i)) || i >= n {
            return None;
        }
        let ls = i;
        let le = self.label_end(i);
        i = le;
        if let Some(q) = quote {
            if self.at(i) != q {
                return None;
            }
            i += 1;
        }
        let end = match self.at(i) {
            b'\n' if i < n => i + 1,
            b'\r' if i < n => {
                if self.at(i + 1) == b'\n' {
                    i + 2
                } else {
                    i + 1
                }
            }
            _ => return None,
        };
        self.state = if quote == Some(b'\'') {
            State::Nowdoc
        } else {
            State::Heredoc
        };
        let label = &self.src[ls..le];
        let mut cur = end;
        while cur < n && matches!(self.src[cur], b' ' | b'\t') {
            cur += 1;
        }
        let mut indentation = 0;
        if cur < n
            && label.len() < n - cur
            && &self.src[cur..cur + label.len()] == label
            && !is_label_char(self.src[cur + label.len()])
        {
            indentation = cur - end;
            self.state = State::EndHeredoc;
        }
        self.labels.push(Label {
            start: ls,
            end: le,
            indentation,
        });
        Some(end)
    }

    /// Rules shared by double quotes, backquotes and heredoc: `${`, `{$`, `$name...`.
    fn interpolation(&mut self, p: usize) -> Option<(Option<Kind>, usize)> {
        match (self.at(p), self.at(p + 1)) {
            (b'$', b'{') => {
                self.push(State::LookingForVarname);
                Some((Some(T_DOLLAR_OPEN_CURLY_BRACES), p + 2))
            }
            (b'{', b'$') => {
                self.push(State::Scripting);
                Some((Some(T_CURLY_OPEN), p + 1))
            }
            (b'$', c) if is_label_start(c) => {
                let e = self.label_end(p + 1);
                match (self.at(e), self.at(e + 1), self.at(e + 2), self.at(e + 3)) {
                    (b'-', b'>', c, _) if is_label_start(c) && e + 2 < self.src.len() => {
                        self.push(State::LookingForProperty);
                    }
                    (b'?', b'-', b'>', c) if is_label_start(c) && e + 3 < self.src.len() => {
                        self.push(State::LookingForProperty);
                    }
                    (b'[', ..) => self.push(State::VarOffset),
                    _ => {}
                }
                Some((Some(T_VARIABLE), e))
            }
            _ => None,
        }
    }

    fn quoted(&mut self, p: usize, quote: u8) -> (Option<Kind>, usize) {
        let n = self.src.len();
        if self.src[p] == quote {
            self.state = State::Scripting;
            return (None, p + 1);
        }
        if let Some(t) = self.interpolation(p) {
            return t;
        }
        let mut cur = p + 1;
        if self.src[p] == b'\\' && cur < n {
            cur += 1;
        }
        while cur < n {
            let c = self.src[cur];
            cur += 1;
            let stop = match c {
                _ if c == quote => true,
                b'$' => is_label_start(self.at(cur)) || self.at(cur) == b'{',
                b'{' => self.at(cur) == b'$',
                b'\\' => {
                    if cur < n {
                        cur += 1;
                    }
                    false
                }
                _ => false,
            };
            if stop {
                cur -= 1;
                break;
            }
        }
        (Some(T_ENCAPSED_AND_WHITESPACE), cur)
    }

    /// Body of a heredoc (`interpolate`) or nowdoc. Returns the end and switches to
    /// ST_END_HEREDOC when the closing label is found.
    fn doc_body(&mut self, p: usize, interpolate: bool) -> (Option<Kind>, usize) {
        let n = self.src.len();
        let (ls, le) = {
            let l = self.labels.last().expect("heredoc label");
            (l.start, l.end)
        };
        let llen = le - ls;
        let mut cur = p;
        while cur < n {
            let c = self.src[cur];
            cur += 1;
            match c {
                b'\r' | b'\n' => {
                    if c == b'\r' && self.at(cur) == b'\n' && cur < n {
                        cur += 1;
                    }
                    let mut indentation = 0;
                    while cur < n && matches!(self.src[cur], b' ' | b'\t') {
                        cur += 1;
                        indentation += 1;
                    }
                    if cur == n {
                        return (Some(T_ENCAPSED_AND_WHITESPACE), n);
                    }
                    if is_label_start(self.src[cur])
                        && llen < n - cur
                        && self.src[cur..cur + llen] == self.src[ls..le]
                    {
                        if is_label_char(self.src[cur + llen]) {
                            continue;
                        }
                        cur -= indentation;
                        self.labels.last_mut().unwrap().indentation = indentation;
                        self.state = State::EndHeredoc;
                        break;
                    }
                }
                b'$' if interpolate && (is_label_start(self.at(cur)) || self.at(cur) == b'{') => {
                    cur -= 1;
                    break;
                }
                b'{' if interpolate && self.at(cur) == b'$' => {
                    cur -= 1;
                    break;
                }
                b'\\'
                    if interpolate
                        && cur < n
                        && self.src[cur] != b'\n'
                        && self.src[cur] != b'\r' =>
                {
                    cur += 1;
                }
                _ => {}
            }
        }
        (Some(T_ENCAPSED_AND_WHITESPACE), cur)
    }

    fn heredoc(&mut self, p: usize) -> (Option<Kind>, usize) {
        if let Some(t) = self.interpolation(p) {
            return t;
        }
        self.doc_body(p, true)
    }

    fn nowdoc(&mut self, p: usize) -> (Option<Kind>, usize) {
        self.doc_body(p, false)
    }

    fn looking_for_property(&mut self, p: usize) -> Option<(Option<Kind>, usize)> {
        let n = self.src.len();
        let c = self.src[p];
        Some(if is_ws(c) {
            let mut e = p + 1;
            while e < n && is_ws(self.src[e]) {
                e += 1;
            }
            (Some(T_WHITESPACE), e)
        } else if c == b'-' && self.at(p + 1) == b'>' {
            (Some(T_OBJECT_OPERATOR), p + 2)
        } else if c == b'?' && self.at(p + 1) == b'-' && self.at(p + 2) == b'>' {
            (Some(T_NULLSAFE_OBJECT_OPERATOR), p + 3)
        } else if is_label_start(c) {
            self.pop();
            (Some(T_STRING), self.label_end(p))
        } else if c == b'#' {
            (Some(T_COMMENT), self.line_comment_end(p + 1))
        } else if c == b'/' && self.at(p + 1) == b'/' {
            (Some(T_COMMENT), self.line_comment_end(p + 2))
        } else if c == b'/' && self.at(p + 1) == b'*' {
            self.block_comment(p)
        } else {
            self.pop();
            return None;
        })
    }

    fn var_offset(&mut self, p: usize) -> (Option<Kind>, usize) {
        let c = self.src[p];
        if c.is_ascii_digit() {
            let mut end = self.lnum_end(p).unwrap_or(p + 1);
            if let Some((e, _)) = self.prefixed_number(p) {
                end = end.max(e);
            }
            return (Some(T_NUM_STRING), end);
        }
        match c {
            b'$' if is_label_start(self.at(p + 1)) => (Some(T_VARIABLE), self.label_end(p + 1)),
            b']' => {
                self.pop();
                (None, p + 1)
            }
            b' ' | b'\n' | b'\r' | b'\t' | b'\\' | b'\'' | b'#' => {
                self.pop();
                (Some(T_ENCAPSED_AND_WHITESPACE), p)
            }
            _ if is_label_start(c) => (Some(T_STRING), self.label_end(p)),
            _ if b";:,.|^&+-/*=%!~$<>?@[(){}\"`".contains(&c) => (None, p + 1),
            _ => (Some(T_BAD_CHARACTER), p + 1),
        }
    }
}

fn strip(digits: &[u8]) -> Vec<u8> {
    digits.iter().copied().filter(|&c| c != b'_').collect()
}

/// T_LNUMBER unless the value overflows a 64-bit zend_long (then T_DNUMBER).
fn decimal_kind(text: &[u8]) -> Kind {
    let mut d = strip(text);
    let octal = d[0] == b'0';
    if octal && let Some(i) = d.iter().position(|&c| c == b'8' || c == b'9') {
        d.truncate(i);
    }
    if d.len() < 19 {
        return T_LNUMBER;
    }
    let base = if octal { 8 } else { 10 };
    if fits_i64(&d, base) {
        T_LNUMBER
    } else {
        T_DNUMBER
    }
}

fn prefixed_kind(digits: &[u8], base: u32) -> Kind {
    let d = strip(digits);
    let d: &[u8] = &d[d.iter().position(|&c| c != b'0').unwrap_or(d.len())..];
    let fits = match base {
        16 => d.len() < 16 || (d.len() == 16 && d[0] <= b'7'),
        2 => d.len() < 64,
        _ => fits_i64(d, 8),
    };
    if fits { T_LNUMBER } else { T_DNUMBER }
}

fn fits_i64(d: &[u8], base: u32) -> bool {
    let mut v: i64 = 0;
    for &c in d {
        let digit = i64::from((c as char).to_digit(base).unwrap_or(0));
        match v
            .checked_mul(i64::from(base))
            .and_then(|v| v.checked_add(digit))
        {
            Some(x) => v = x,
            None => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(src: &str) -> Vec<(String, String)> {
        tokenize(src.as_bytes())
            .iter()
            .map(|t| {
                (
                    t.kind.map_or(String::new(), |k| k.name().to_string()),
                    String::from_utf8_lossy(t.text).into_owned(),
                )
            })
            .collect()
    }

    #[test]
    fn basics() {
        let t = kinds("<?php echo $a->class; ?>\nx");
        assert_eq!(t[0], ("T_OPEN_TAG".into(), "<?php ".into()));
        assert_eq!(t[5], ("T_STRING".into(), "class".into()));
        assert_eq!(t[8], ("T_CLOSE_TAG".into(), "?>\n".into()));
        assert_eq!(t[9], ("T_INLINE_HTML".into(), "x".into()));
    }
}
