//! Port of `PhpCsFixer\Fixer\Operator\AssignNullCoalescingToCoalesceEqualFixer`.
use crate::analyzers::abstract_short_operator_fixer::{ShortOperator, apply_fix};
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct AssignNullCoalescingToCoalesceEqualFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(AssignNullCoalescingToCoalesceEqualFixer)
}

impl ShortOperator for AssignNullCoalescingToCoalesceEqualFixer {
    fn is_operator_token_candidate(&self, tokens: &Tokens, index: usize) -> bool {
        if !tokens[index].is_kind(T_COALESCE) {
            return false;
        }
        // make sure after '??' does not contain '? :'
        let next_index = tokens
            .get_next_token_of_kind(index, &["?".into(), ";".into(), T_CLOSE_TAG.into()], true)
            .unwrap();
        !tokens[next_index].equals("?", true)
    }
    fn get_replacement_token(&self, _token: &Token) -> Token {
        Token::new(T_COALESCE_EQUAL, "??=")
    }
}

impl Fixer for AssignNullCoalescingToCoalesceEqualFixer {
    fn name(&self) -> &'static str {
        "assign_null_coalescing_to_coalesce_equal"
    }
    fn priority(&self) -> i32 {
        -1
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_COALESCE)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        apply_fix(self, tokens);
    }
}
