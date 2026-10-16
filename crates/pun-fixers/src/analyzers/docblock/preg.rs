//! Thin adapter over the shared `analyzers::preg::Preg` for the call shapes the DocBlock port uses,
//! plus PHP's `preg_quote`.

pub use crate::analyzers::preg::Captures as Caps;
use crate::analyzers::preg::{PREG_SPLIT_DELIM_CAPTURE, PREG_SPLIT_NO_EMPTY, Preg};

pub const SPLIT_NO_EMPTY: u32 = PREG_SPLIT_NO_EMPTY;
pub const SPLIT_DELIM_CAPTURE: u32 = PREG_SPLIT_DELIM_CAPTURE;

pub fn is_match(pattern: impl AsRef<[u8]>, subject: &[u8]) -> bool {
    Preg::r#match(pattern, subject)
}

pub fn captures<'s>(pattern: impl AsRef<[u8]>, subject: &'s [u8]) -> Option<Caps<'s>> {
    Preg::match_groups(pattern, subject, 0)
}

pub fn match_at<'s>(pattern: impl AsRef<[u8]>, subject: &'s [u8], offset: usize) -> Option<Caps<'s>> {
    Preg::match_at(pattern, subject, 0, offset)
}

pub fn match_all<'s>(pattern: impl AsRef<[u8]>, subject: &'s [u8]) -> Vec<Caps<'s>> {
    Preg::match_all(pattern, subject, 0)
}

pub fn replace(pattern: impl AsRef<[u8]>, replacement: &[u8], subject: &[u8], limit: isize) -> Vec<u8> {
    Preg::replace_limit(pattern, replacement, subject, limit).0
}

pub fn split(pattern: impl AsRef<[u8]>, subject: &[u8], flags: u32) -> Vec<Vec<u8>> {
    Preg::split(pattern, subject, -1, flags).into_iter().map(|p| p.text).collect()
}

/// PHP `preg_quote`.
pub fn quote(s: &[u8], delimiter: Option<u8>) -> Vec<u8> {
    let mut o = Vec::with_capacity(s.len());
    for &c in s {
        if c == 0 {
            o.extend_from_slice(b"\\000");
            continue;
        }
        if b".\\+*?[^]$(){}=!<>|:-#".contains(&c) || Some(c) == delimiter {
            o.push(b'\\');
        }
        o.push(c);
    }
    o
}
