//! Port of `PhpCsFixer\Fixer\ArrayNotation\NormalizeIndexBraceFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct NormalizeIndexBraceFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(NormalizeIndexBraceFixer)
}

impl Fixer for NormalizeIndexBraceFixer {
    fn name(&self) -> &'static str {
        "normalize_index_brace"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(CT_ARRAY_INDEX_BRACE_OPEN)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.len() {
            if tokens[index].is_kind(CT_ARRAY_INDEX_BRACE_OPEN) {
                tokens.set(index, Token::char("["));
            } else if tokens[index].is_kind(CT_ARRAY_INDEX_BRACE_CLOSE) {
                tokens.set(index, Token::char("]"));
            }
        }
    }
}
