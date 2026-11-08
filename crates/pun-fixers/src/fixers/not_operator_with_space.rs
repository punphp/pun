//! Port of `PhpCsFixer\Fixer\Operator\NotOperatorWithSpaceFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct NotOperatorWithSpaceFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(NotOperatorWithSpaceFixer)
}

impl Fixer for NotOperatorWithSpaceFixer {
    fn name(&self) -> &'static str {
        "not_operator_with_space"
    }
    fn priority(&self) -> i32 {
        -10
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found("!")
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (0..tokens.count()).rev() {
            if tokens[index].equals("!", true) {
                if !tokens[index + 1].is_whitespace(None) {
                    tokens.insert_at(index + 1, vec![Token::new(T_WHITESPACE, " ")]);
                }
                if !tokens[index - 1].is_whitespace(None) {
                    tokens.insert_at(index, vec![Token::new(T_WHITESPACE, " ")]);
                }
            }
        }
    }
}
