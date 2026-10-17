//! Port of `PhpCsFixer\Fixer\CastNotation\ShortScalarCastFixer`.
use pun_format::Fixer;
use pun_tokens::php_compat::{strtolower, trim};
use pun_tokens::token::CAST_KINDS;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct ShortScalarCastFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(ShortScalarCastFixer)
}

/// PHP `str_replace($search, $replace, $subject)` for one non-empty needle.
fn str_replace(search: &[u8], replace: &[u8], subject: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(subject.len());
    let mut i = 0;
    while i < subject.len() {
        if subject[i..].starts_with(search) {
            out.extend_from_slice(replace);
            i += search.len();
        } else {
            out.push(subject[i]);
            i += 1;
        }
    }
    out
}

impl Fixer for ShortScalarCastFixer {
    fn name(&self) -> &'static str {
        "short_scalar_cast"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(CAST_KINDS)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].is_cast() {
                continue;
            }
            let content = &tokens[index].content;
            let cast_from = trim(&content[1..content.len() - 1]);
            let cast_to: &[u8] = match &strtolower(cast_from)[..] {
                b"boolean" => b"bool",
                b"integer" => b"int",
                b"double" | b"real" => b"float",
                b"binary" => b"string",
                _ => continue,
            };
            let new = Token::new(
                tokens[index].id.unwrap(),
                str_replace(cast_from, cast_to, content),
            );
            tokens.set(index, new);
        }
    }
}
