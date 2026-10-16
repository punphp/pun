//! Port of `PhpCsFixer\Fixer\StringNotation\NoBinaryStringFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct NoBinaryStringFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoBinaryStringFixer)
}

impl Fixer for NoBinaryStringFixer {
    fn name(&self) -> &'static str {
        "no_binary_string"
    }
    fn priority(&self) -> i32 {
        40
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_CONSTANT_ENCAPSED_STRING, T_START_HEREDOC]) || tokens.is_token_kind_found("b\"")
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = 0;
        while index < tokens.len() {
            let token = &tokens[index];
            if token.is_given_kind(&[T_CONSTANT_ENCAPSED_STRING, T_START_HEREDOC]) {
                if token.content[0].eq_ignore_ascii_case(&b'b') {
                    let new = Token::new(token.id.unwrap(), &token.content[1..]);
                    tokens.set(index, new);
                }
            } else if token.equals("b\"", true) {
                tokens.set(index, Token::char("\""));
            }
            index += 1;
        }
    }
}
