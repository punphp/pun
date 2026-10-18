//! Port of `PhpCsFixer\Fixer\Basic\OctalNotationFixer`. Pun runs as PHP 8.5, so `PHP_VERSION_ID >= 8_01_00` holds.
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct OctalNotationFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(OctalNotationFixer)
}

impl Fixer for OctalNotationFixer {
    fn name(&self) -> &'static str {
        "octal_notation"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_LNUMBER)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].is_kind(T_LNUMBER) {
                continue;
            }
            let content = tokens[index].get_content();
            let new_content = Preg::replace("#^0_*+([0-7_]+)$#", "0o$1", content);
            if content == new_content {
                continue;
            }
            tokens.set(index, Token::new(T_LNUMBER, new_content));
        }
    }
}
