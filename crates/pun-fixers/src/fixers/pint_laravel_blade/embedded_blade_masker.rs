//! Port of `App\PrettierFormatters\EmbeddedBladeMasker`: masks Blade constructs inside
//! `<style>`/`<script>` before prettier, restores them after.
use pun_tokens::php_compat::{str_replace, substr_count};
use super::php::{contains, ctype_alnum, stripos, strpos};
use crate::analyzers::preg::RawPreg;
use std::cell::RefCell;

const BLOCK_OPENERS: &[&[u8]] = &[
    b"if", b"unless", b"foreach", b"for", b"while", b"isset", b"switch",
];
const BLOCK_CLOSERS: &[&[u8]] = &[
    b"endif",
    b"endunless",
    b"endforeach",
    b"endfor",
    b"endwhile",
    b"endisset",
    b"endempty",
    b"endswitch",
];

#[derive(Clone, Copy)]
enum Context {
    Value,
    JsExpression,
    JsStatement,
    Statement,
}

#[derive(Default)]
pub(super) struct EmbeddedBladeMasker {
    map: Vec<(Vec<u8>, Vec<u8>)>,
    original: Vec<u8>,
    counter: usize,
}

fn is_word(c: u8) -> bool {
    c == b'_' || ctype_alnum(c)
}

impl EmbeddedBladeMasker {
    /// `Err` where Pint's `preg_replace_callback` returns `null` (e.g. the backtrack limit on an unclosed
    /// `<script>` with quoted attributes): Pint casts it to `""` and empties the file; Pun reports the file instead.
    pub(super) fn pre_format(&mut self, content: &[u8]) -> Result<Vec<u8>, String> {
        self.map = Vec::new();
        self.original = content.to_vec();
        self.counter = 0;
        let this = RefCell::new(std::mem::take(self));
        let result = RawPreg::replace_callback(
            "/(<(style|script)\\b(?:\"[^\"]*\"|'[^']*'|[^>])*>)(.*?)(<\\/\\2>)/is",
            |m| {
                let is_css = m.str(2).eq_ignore_ascii_case(b"style");
                [
                    m.str(1),
                    &this.borrow_mut().mask_region(m.str(3), is_css),
                    m.str(4),
                ]
                .concat()
            },
            content,
        );
        *self = this.into_inner();
        result.map_err(|e| format!("preg_replace_callback(): {e}"))
    }

    pub(super) fn post_format(&self, content: &[u8]) -> Vec<u8> {
        if self.map.is_empty() {
            return content.to_vec();
        }
        // Every token must appear exactly once; otherwise fall back to the original to avoid corrupting the file.
        for (token, _) in &self.map {
            if substr_count(content, token) != 1 {
                return self.original.clone();
            }
        }
        let mut result = content.to_vec();
        for (token, original) in &self.map {
            result = str_replace(token, original, &result);
        }
        result
    }

    fn mask_region(&mut self, region: &[u8], is_css: bool) -> Vec<u8> {
        let length = region.len();
        let mut offset = 0;
        let mut result = Vec::with_capacity(length);
        while offset < length {
            let c = region[offset];
            if is_string_delimiter(c, is_css) {
                let end = scan_string_literal(region, length, offset, c);
                let literal = &region[offset..end];
                if contains_echo(literal) {
                    let masked = self.mask(
                        literal,
                        if is_css {
                            Context::Value
                        } else {
                            Context::JsExpression
                        },
                    );
                    result.extend_from_slice(&masked);
                } else {
                    result.extend_from_slice(literal);
                }
                offset = end;
                continue;
            }
            let Some(span) = match_construct(region, offset, length) else {
                result.push(c);
                offset += 1;
                continue;
            };
            let context = if is_css {
                css_context(region, offset)
            } else {
                js_context(region, offset)
            };
            let masked = self.mask(&region[offset..(offset + span).min(length)], context);
            result.extend_from_slice(&masked);
            offset += span;
        }
        result
    }

    fn mask(&mut self, original: &[u8], context: Context) -> Vec<u8> {
        let token = self.make_token(context);
        // `$this->map[$token] = $original`: tokens are unique, so this always appends.
        self.map.push((token.clone(), original.to_vec()));
        token
    }

    fn make_token(&mut self, context: Context) -> Vec<u8> {
        let index = self.unique_index();
        match context {
            Context::Value | Context::JsExpression => format!("__PINT_BLADE_{index}__"),
            Context::JsStatement => format!("__PINT_BLADE_{index}__;"),
            Context::Statement => format!("--pint-blade-{index}: 1;"),
        }
        .into_bytes()
    }

    fn unique_index(&mut self) -> usize {
        loop {
            let index = self.counter;
            self.counter += 1;
            let collides = contains(&self.original, format!("__PINT_BLADE_{index}__").as_bytes())
                || contains(&self.original, format!("--pint-blade-{index}").as_bytes());
            if !collides {
                return index;
            }
        }
    }
}

fn match_construct(content: &[u8], offset: usize, length: usize) -> Option<usize> {
    let c = content[offset];
    // Raw PHP open tags ("<?php" / short echo "<?=") through their close tag.
    if c == b'<' && offset + 1 < length && content[offset + 1] == b'?' {
        let end = strpos(content, b"?>", offset + 2).map_or(length, |e| e + 2);
        return Some(end - offset);
    }
    // Blade echoes outside string literals.
    if let Some(echo) = echo_span(content, offset) {
        return Some(echo);
    }
    if c != b'@' {
        return None;
    }
    // Escaped "@@" is a literal "@", not a directive.
    if offset + 1 < length && content[offset + 1] == b'@' {
        return None;
    }
    // "@" preceded by a word character (e.g. an email) is not a directive.
    if offset > 0 && is_word(content[offset - 1]) {
        return None;
    }
    let mut cursor = offset + 1;
    while cursor < length && is_word(content[cursor]) {
        cursor += 1;
    }
    if cursor == offset + 1 {
        return None;
    }
    let lower = content[offset + 1..cursor].to_ascii_lowercase();
    if lower == b"php" {
        if cursor < length && content[cursor] == b'(' {
            return Some(skip_parens(content, cursor, length) - offset);
        }
        return stripos(content, b"@endphp", cursor).map(|end| end + 7 - offset);
    }
    if is_block_opener(content, &lower, cursor, length) {
        return scan_block(content, offset, length).map(|end| end - offset);
    }
    None
}

fn is_block_opener(content: &[u8], lower: &[u8], after_name: usize, length: usize) -> bool {
    if BLOCK_OPENERS.contains(&lower) {
        return true;
    }
    lower == b"empty" && next_non_space_is_paren(content, after_name, length)
}

fn scan_block(content: &[u8], start: usize, length: usize) -> Option<usize> {
    let mut depth: isize = 0;
    let mut offset = start;
    while offset < length {
        let (name, after_name) = next_directive(content, offset, length)?;
        let lower = name.to_ascii_lowercase();
        if lower.starts_with(b"end") && BLOCK_CLOSERS.contains(&&lower[..]) {
            depth -= 1;
            if depth == 0 {
                return Some(after_name);
            }
            offset = after_name;
            continue;
        }
        if is_block_opener(content, &lower, after_name, length) {
            depth += 1;
        }
        // Skip any "(...)" argument list so its contents are never re-scanned.
        offset = skip_optional_parens(content, after_name, length);
    }
    None
}

fn next_directive(content: &[u8], from: usize, length: usize) -> Option<(&[u8], usize)> {
    let mut offset = from;
    while offset < length {
        if content[offset] != b'@' {
            offset += 1;
            continue;
        }
        if offset + 1 < length && content[offset + 1] == b'@' {
            offset += 2;
            continue;
        }
        if offset > 0 && is_word(content[offset - 1]) {
            offset += 1;
            continue;
        }
        let mut cursor = offset + 1;
        while cursor < length && is_word(content[cursor]) {
            cursor += 1;
        }
        if cursor > offset + 1 {
            return Some((&content[offset + 1..cursor], cursor));
        }
        offset += 1;
    }
    None
}

fn next_non_space_is_paren(content: &[u8], mut offset: usize, length: usize) -> bool {
    while offset < length && (content[offset] == b' ' || content[offset] == b'\t') {
        offset += 1;
    }
    offset < length && content[offset] == b'('
}

fn skip_optional_parens(content: &[u8], offset: usize, length: usize) -> usize {
    let mut cursor = offset;
    while cursor < length && (content[cursor] == b' ' || content[cursor] == b'\t') {
        cursor += 1;
    }
    if cursor < length && content[cursor] == b'(' {
        return skip_parens(content, cursor, length);
    }
    offset
}

fn skip_parens(content: &[u8], mut offset: usize, length: usize) -> usize {
    let mut depth: isize = 0;
    while offset < length {
        let c = content[offset];
        if c == b'(' {
            depth += 1;
        } else if c == b')' {
            depth -= 1;
            offset += 1;
            if depth == 0 {
                return offset;
            }
            continue;
        }
        offset += 1;
    }
    offset
}

fn css_context(content: &[u8], offset: usize) -> Context {
    for &c in content[..offset].iter().rev() {
        if c == b':' {
            return Context::Value;
        }
        if c == b';' || c == b'{' || c == b'}' {
            return Context::Statement;
        }
    }
    Context::Statement
}

fn js_context(content: &[u8], offset: usize) -> Context {
    for &c in content[..offset].iter().rev() {
        if c == b' ' || c == b'\t' {
            continue;
        }
        if c == b';' || c == b'{' || c == b'}' || c == b'\n' || c == b'\r' {
            return Context::JsStatement;
        }
        return Context::JsExpression;
    }
    Context::JsStatement
}

fn is_string_delimiter(c: u8, is_css: bool) -> bool {
    if c == b'"' || c == b'\'' {
        return true;
    }
    !is_css && c == b'`'
}

fn scan_string_literal(content: &[u8], length: usize, offset: usize, quote: u8) -> usize {
    let mut cursor = offset + 1;
    while cursor < length {
        let c = content[cursor];
        if c == b'\\' {
            cursor += 2;
            continue;
        }
        if let Some(echo) = echo_span(content, cursor) {
            cursor += echo;
            continue;
        }
        if c == quote {
            return cursor + 1;
        }
        cursor += 1;
    }
    length
}

fn echo_span(content: &[u8], offset: usize) -> Option<usize> {
    if content[offset] != b'{' {
        return None;
    }
    let rest = &content[offset..];
    if rest.starts_with(b"{{--") {
        return strpos(content, b"--}}", offset + 4).map(|end| end + 4 - offset);
    }
    if rest.starts_with(b"{!!") {
        return strpos(content, b"!!}", offset + 3).map(|end| end + 3 - offset);
    }
    if rest.starts_with(b"{{") {
        return strpos(content, b"}}", offset + 2).map(|end| end + 2 - offset);
    }
    None
}

fn contains_echo(value: &[u8]) -> bool {
    contains(value, b"{{") || contains(value, b"{!!")
}
