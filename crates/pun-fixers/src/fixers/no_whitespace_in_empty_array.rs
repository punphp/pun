//! Port of `PhpCsFixer\Fixer\ArrayNotation\NoWhitespaceInEmptyArrayFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::Tokens;
use std::path::Path;

struct NoWhitespaceInEmptyArrayFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoWhitespaceInEmptyArrayFixer)
}

impl Fixer for NoWhitespaceInEmptyArrayFixer {
    fn name(&self) -> &'static str {
        "no_whitespace_in_empty_array"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(CT_ARRAY_BRACKET_OPEN)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (1..tokens.count()).rev() {
            if !tokens[index].is_kind(CT_ARRAY_BRACKET_CLOSE) {
                continue;
            }
            if tokens.get_prev_non_whitespace(index, None) != Some(index - 2) {
                continue;
            }
            if !tokens[index - 2].is_kind(CT_ARRAY_BRACKET_OPEN) {
                continue;
            }
            tokens.clear_at(index - 1);
        }
    }
}
