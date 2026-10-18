//! Port of `PhpCsFixer\Fixer\Operator\NotOperatorWithSuccessorSpaceFixer`.
use pun_format::Fixer;
use pun_tokens::Tokens;
use std::path::Path;

struct NotOperatorWithSuccessorSpaceFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(NotOperatorWithSuccessorSpaceFixer)
}

impl Fixer for NotOperatorWithSuccessorSpaceFixer {
    fn name(&self) -> &'static str {
        "not_operator_with_successor_space"
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
                tokens.ensure_whitespace_at_index(index + 1, 0, b" ");
            }
        }
    }
}
