//! Port of `PhpCsFixer\Fixer\Operator\StandardizeNotEqualsFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct StandardizeNotEqualsFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(StandardizeNotEqualsFixer)
}

impl Fixer for StandardizeNotEqualsFixer {
    fn name(&self) -> &'static str {
        "standardize_not_equals"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_IS_NOT_EQUAL)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.len() {
            if tokens[index].is_kind(T_IS_NOT_EQUAL) {
                tokens.set(index, Token::new(T_IS_NOT_EQUAL, "!="));
            }
        }
    }
}
