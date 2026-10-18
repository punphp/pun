//! Port of `PhpCsFixer\Fixer\Strict\StrictComparisonFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct StrictComparisonFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(StrictComparisonFixer)
}

impl Fixer for StrictComparisonFixer {
    fn name(&self) -> &'static str {
        "strict_comparison"
    }
    fn priority(&self) -> i32 {
        38
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_IS_EQUAL, T_IS_NOT_EQUAL])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for (kind, indices) in tokens.find_given_kinds(&[T_IS_EQUAL, T_IS_NOT_EQUAL], 0, None) {
            let new = if kind == T_IS_EQUAL {
                (T_IS_IDENTICAL, "===")
            } else {
                (T_IS_NOT_IDENTICAL, "!==")
            };
            for index in indices {
                tokens.set(index, Token::new(new.0, new.1));
            }
        }
    }
}
