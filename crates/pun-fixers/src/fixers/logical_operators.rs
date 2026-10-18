//! Port of `PhpCsFixer\Fixer\Operator\LogicalOperatorsFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct LogicalOperatorsFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(LogicalOperatorsFixer)
}

impl Fixer for LogicalOperatorsFixer {
    fn name(&self) -> &'static str {
        "logical_operators"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_LOGICAL_AND, T_LOGICAL_OR])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.len() {
            if tokens[index].is_kind(T_LOGICAL_AND) {
                tokens.set(index, Token::new(T_BOOLEAN_AND, "&&"));
            } else if tokens[index].is_kind(T_LOGICAL_OR) {
                tokens.set(index, Token::new(T_BOOLEAN_OR, "||"));
            }
        }
    }
}
