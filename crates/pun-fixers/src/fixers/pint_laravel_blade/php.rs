//! The PHP string functions Pint's Blade formatters use, byte-oriented like PHP.

pub use pun_tokens::php_compat::{ltrim_chars, rtrim, trim};

/// `strpos($haystack, $needle, $offset)` (callers pass a non-empty needle and `offset <= len`).
pub fn strpos(haystack: &[u8], needle: &[u8], offset: usize) -> Option<usize> {
    haystack[offset..]
        .windows(needle.len())
        .position(|w| w == needle)
        .map(|p| p + offset)
}

/// `stripos($haystack, $needle, $offset)` (ASCII case folding).
pub fn stripos(haystack: &[u8], needle: &[u8], offset: usize) -> Option<usize> {
    haystack[offset..]
        .windows(needle.len())
        .position(|w| w.eq_ignore_ascii_case(needle))
        .map(|p| p + offset)
}

/// `strrpos($haystack, $needle)`.
pub fn strrpos(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.len() > haystack.len() {
        return None;
    }
    (0..=haystack.len() - needle.len())
        .rev()
        .find(|&i| &haystack[i..i + needle.len()] == needle)
}

/// `substr($s, $start, $length)` for `start < len` (callers guarantee it).
pub fn substr(s: &[u8], start: usize, length: Option<usize>) -> &[u8] {
    let end = length.map_or(s.len(), |l| (start + l).min(s.len()));
    &s[start..end]
}

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
    s.split(|&b| b == b'\n').map(<[u8]>::to_vec).collect()
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

pub fn ctype_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}
