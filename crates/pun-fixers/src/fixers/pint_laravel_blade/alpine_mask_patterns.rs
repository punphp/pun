//! Port of `App\PrettierFormatters\AlpineMaskPatterns`: masks an Alpine `x-mask` pattern (a literal template such
//! as "9999-999", which prettier would format as a JS subtraction) across prettier, then restores it.
use super::embedded_blade_masker::Unrestorable;
use super::php::contains;
use crate::analyzers::preg::RawPreg;
use pun_tokens::php_compat::{str_replace, substr_count};
use std::cell::RefCell;

/// `PATTERN`: a trailing ".modifier" still masks; `x-mask:dynamic` is a real JS expression and is left alone.
const PATTERN: &str = r#"/(?<![-\w:.@])x-mask(?:\.[\w.-]+)?\s*=\s*(?:(["'])(.+?)\1|([^\s"'=<>`]+))/i"#;

#[derive(Default)]
pub(super) struct AlpineMaskPatterns {
    map: Vec<(Vec<u8>, Vec<u8>)>,
    original: Vec<u8>,
    counter: usize,
}

impl AlpineMaskPatterns {
    /// `Err` where Pint's `preg_replace_callback` returns `null` (cast to `""`, emptying the file); Pun reports the
    /// file instead, like `EmbeddedBladeMasker::pre_format`.
    pub(super) fn pre_format(&mut self, content: &[u8]) -> Result<Vec<u8>, String> {
        self.map = Vec::new();
        self.original = content.to_vec();
        self.counter = 0;
        let this = RefCell::new(std::mem::take(self));
        let result = RawPreg::replace_callback(
            PATTERN,
            |m| {
                let whole = m.str(0);
                let group = if m.offset(3).is_some() { 3 } else { 2 };
                let value = m.str(group);
                let token = this.borrow_mut().mask(value);
                // Splice the token in at the value's own position, so a value that also appears in the
                // attribute name is left alone.
                let position = m.offset(group).expect("the value group matched") - m.start();
                [&whole[..position], &token, &whole[position + value.len()..]].concat()
            },
            content,
        );
        *self = this.into_inner();
        result.map_err(|e| format!("preg_replace_callback(): {e}"))
    }

    /// `Err(Unrestorable)` when a token does not come back exactly once.
    pub(super) fn post_format(&self, content: &[u8]) -> Result<Vec<u8>, Unrestorable> {
        if self.map.is_empty() {
            return Ok(content.to_vec());
        }
        for (token, _) in &self.map {
            if substr_count(content, token) != 1 {
                return Err(Unrestorable);
            }
        }
        let mut result = content.to_vec();
        for (token, value) in &self.map {
            result = str_replace(token, value, &result);
        }
        Ok(result)
    }

    fn mask(&mut self, value: &[u8]) -> Vec<u8> {
        let token = self.make_token(value.len());
        self.map.push((token.clone(), value.to_vec()));
        token
    }

    /// `makeToken`: "pm<n>_" padded with "_" to the pattern's byte length, widened until the source lacks it.
    fn make_token(&mut self, length: usize) -> Vec<u8> {
        let mut token = format!("pm{}_", self.counter).into_bytes();
        self.counter += 1;
        if token.len() < length {
            token.resize(length, b'_');
        }
        while contains(&self.original, &token) {
            token.push(b'_');
        }
        token
    }
}
