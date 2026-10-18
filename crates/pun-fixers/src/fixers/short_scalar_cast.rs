//! Port of `PhpCsFixer\Fixer\CastNotation\ShortScalarCastFixer`.
use pun_tokens::php_compat::str_replace;
use pun_format::Fixer;
use pun_tokens::php_compat::{strtolower, trim};
use pun_tokens::token::CAST_KINDS;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct ShortScalarCastFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(ShortScalarCastFixer)
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
            let content = tokens[index].get_content();
            let cast_from = trim(&content[1..content.len() - 1]);
            let cast_to: &[u8] = match &strtolower(cast_from)[..] {
                b"boolean" => b"bool",
                b"integer" => b"int",
                b"double" | b"real" => b"float",
                b"binary" => b"string",
                _ => continue,
            };
            let new = Token::new(
                tokens[index].get_id().unwrap(),
                str_replace(cast_from, cast_to, content),
            );
            tokens.set(index, new);
        }
    }
}
