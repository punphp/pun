//! The PHP string functions Pint's Blade formatters use, byte-oriented like PHP.

pub use pun_tokens::php_compat::{
    isspace as ctype_space, ltrim_chars, rtrim, stripos, strpos, strrpos, substr, trim,
};

pub fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    strpos(haystack, needle, 0).is_some()
}

pub fn ltrim(s: &[u8]) -> &[u8] {
    pun_tokens::php_compat::ltrim_chars(s, pun_tokens::php_compat::TRIM_CHARS)
}

pub fn trim_chars<'a>(s: &'a [u8], chars: &[u8]) -> &'a [u8] {
    pun_tokens::php_compat::ltrim_chars(pun_tokens::php_compat::rtrim_chars(s, chars), chars)
}

/// `explode("\n", $s)`.
pub fn lines(s: &[u8]) -> Vec<Vec<u8>> {
    pun_tokens::php_compat::explode(b"\n", s)
        .into_iter()
        .map(<[u8]>::to_vec)
        .collect()
}

/// `implode("\n", $lines)`.
pub fn join_lines(lines: &[Vec<u8>]) -> Vec<u8> {
    lines.join(&b'\n')
}

pub fn ctype_alpha(c: u8) -> bool {
    c.is_ascii_alphabetic()
}

pub fn ctype_alnum(c: u8) -> bool {
    c.is_ascii_alphanumeric()
}
