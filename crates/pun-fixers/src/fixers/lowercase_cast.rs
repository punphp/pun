//! Port of `PhpCsFixer\Fixer\CastNotation\LowercaseCastFixer`.
use pun_format::Fixer;
use pun_tokens::token::CAST_KINDS;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct LowercaseCastFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(LowercaseCastFixer)
}

impl Fixer for LowercaseCastFixer {
    fn name(&self) -> &'static str {
        "lowercase_cast"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(CAST_KINDS)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].is_cast() || !tokens[index].get_content().iter().any(u8::is_ascii_uppercase) {
                continue;
            }
            let t = &tokens[index];
            let new = Token::new(t.get_id().unwrap(), t.get_content().to_ascii_lowercase());
            tokens.set(index, new);
        }
    }
}
