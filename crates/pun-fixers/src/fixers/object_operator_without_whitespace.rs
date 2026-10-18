//! Port of `PhpCsFixer\Fixer\Operator\ObjectOperatorWithoutWhitespaceFixer`.
use pun_format::Fixer;
use pun_tokens::{Tokens, token::OBJECT_OPERATOR_KINDS};
use std::path::Path;

struct ObjectOperatorWithoutWhitespaceFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(ObjectOperatorWithoutWhitespaceFixer)
}

impl Fixer for ObjectOperatorWithoutWhitespaceFixer {
    fn name(&self) -> &'static str {
        "object_operator_without_whitespace"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(OBJECT_OPERATOR_KINDS)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.len() {
            if !tokens[index].is_object_operator() {
                continue;
            }
            if tokens[index - 1].is_whitespace(Some(b" \t")) && !tokens[index - 2].is_comment() {
                tokens.clear_at(index - 1);
            }
            if tokens[index + 1].is_whitespace(Some(b" \t")) && !tokens[index + 2].is_comment() {
                tokens.clear_at(index + 1);
            }
        }
    }
}
