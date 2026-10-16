//! Port of `PhpCsFixer\Fixer\Casing\LowercaseKeywordsFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct LowercaseKeywordsFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(LowercaseKeywordsFixer)
}

impl Fixer for LowercaseKeywordsFixer {
    fn name(&self) -> &'static str {
        "lowercase_keywords"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(pun_tokens::token::KEYWORDS)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.len() {
            let token = &tokens[index];
            if token.is_keyword() && !token.is_kind(T_HALT_COMPILER) {
                let new = Token::new(token.id.unwrap(), token.content.to_ascii_lowercase());
                tokens.set(index, new);
            }
        }
    }
}
