//! Port of `PhpCsFixer\Fixer\Basic\NumericLiteralSeparatorFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct NumericLiteralSeparatorFixer {
    override_existing: bool,
    use_separator: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(NumericLiteralSeparatorFixer {
        override_existing: false,
        use_separator: true,
    })
}

impl Fixer for NumericLiteralSeparatorFixer {
    fn name(&self) -> &'static str {
        "numeric_literal_separator"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_DNUMBER, T_LNUMBER])
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["override_existing", "strategy"])?;
        self.override_existing = o.bool("override_existing", false)?;
        self.use_separator = o.choice(
            "strategy",
            "use_separator",
            &[("use_separator", true), ("no_separator", false)],
        )?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            let token = &tokens[index];
            if !token.is_given_kind(&[T_DNUMBER, T_LNUMBER]) {
                continue;
            }
            let new_content = self.format_value(token.get_content());
            if token.get_content() == new_content {
                continue;
            }
            let id = token.id.unwrap();
            tokens.set(index, Token::new(id, new_content));
        }
    }
}

/// `str_contains($v, '-') ? 1 : 0`: a negative value needs an offset.
fn negative_offset(v: &[u8]) -> usize {
    usize::from(v.contains(&b'-'))
}

impl NumericLiteralSeparatorFixer {
    fn format_value(&self, value: &[u8]) -> Vec<u8> {
        let mut value = value.to_vec();
        if !self.use_separator {
            value.retain(|&c| c != b'_');
            return value;
        }
        if self.override_existing {
            value.retain(|&c| c != b'_');
        } else if value.contains(&b'_') {
            // Keep already underscored literals untouched.
            return value;
        }
        let lower = strtolower(&value);
        if lower.starts_with(b"0b") {
            return insert_every_right(value, 8, 2);
        }
        if lower.starts_with(b"0x") {
            return insert_every_right(value, 2, 2);
        }
        if lower.starts_with(b"0o") {
            return insert_every_right(value, 3, 2);
        }
        if lower.starts_with(b"0") && !lower.contains(&b'.') {
            // Octal notation prior PHP 8.1 but still valid
            return insert_every_right(value, 3, 1);
        }

        // All other types
        let m =
            Preg::match_groups(r"/([0-9-_]+)?((\.)([0-9_]*))?((e)([0-9-_]+))?/i", &value).unwrap();
        let group = |i| m.get(i).unwrap_or(b"");
        let integer = group(1);
        let mut joined = insert_every_right(integer.to_vec(), 3, negative_offset(integer));
        let dot = group(3);
        if !dot.is_empty() {
            let integer = group(4);
            joined.extend_from_slice(dot);
            joined.extend(insert_every_left(
                integer.to_vec(),
                3,
                negative_offset(integer),
            ));
        }
        let tim = group(6);
        if !tim.is_empty() {
            let integer = group(7);
            joined.extend_from_slice(tim);
            joined.extend(insert_every_right(
                integer.to_vec(),
                3,
                negative_offset(integer),
            ));
        }
        joined
    }
}

/// `insertEveryRight`: `substr_replace($value, '_', $position, 0)` from the right, every `length` characters.
fn insert_every_right(mut value: Vec<u8>, length: usize, offset: usize) -> Vec<u8> {
    let mut position = length;
    while (position as isize) < value.len() as isize - offset as isize {
        value.insert(value.len() - position, b'_');
        position += length + 1;
    }
    value
}

/// `insertEveryLeft`: `substr_replace($value, '_', $position, $offset)` from the left, every `length` characters.
fn insert_every_left(mut value: Vec<u8>, length: usize, offset: usize) -> Vec<u8> {
    let mut position = length;
    while position < value.len() {
        let end = (position + offset).min(value.len());
        value.splice(position..end, [b'_']);
        position += length + 1;
    }
    value
}
