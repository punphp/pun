//! Port of `PhpCsFixer\Fixer\Semicolon\SemicolonAfterInstructionFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Proto, Token, Tokens};
use std::path::Path;

struct SemicolonAfterInstructionFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(SemicolonAfterInstructionFixer)
}

impl Fixer for SemicolonAfterInstructionFixer {
    fn name(&self) -> &'static str {
        "semicolon_after_instruction"
    }
    fn priority(&self) -> i32 {
        // Must run before SimplifiedIfReturnFixer.
        2
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_CLOSE_TAG)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (2..tokens.count()).rev() {
            if !tokens[index].is_kind(T_CLOSE_TAG) {
                continue;
            }
            let prev = tokens.get_prev_meaningful_token(index).unwrap();
            if tokens[prev].equals_any(
                &[
                    Proto::Char(b";"),
                    Proto::Char(b"{"),
                    Proto::Char(b"}"),
                    Proto::Char(b":"),
                    Proto::Kind(T_OPEN_TAG),
                ],
                true,
            ) {
                continue;
            }
            tokens.insert_at(prev + 1, vec![Token::char(";")]);
        }
    }
}
