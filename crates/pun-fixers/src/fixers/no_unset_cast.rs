//! Port of `PhpCsFixer\Fixer\CastNotation\NoUnsetCastFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct NoUnsetCastFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoUnsetCastFixer)
}

impl Fixer for NoUnsetCastFixer {
    fn name(&self) -> &'static str {
        "no_unset_cast"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_UNSET_CAST)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = tokens.count() - 1;
        while index > 0 {
            if tokens[index].is_kind(T_UNSET_CAST) {
                fix_unset_cast(tokens, index);
            }
            index -= 1;
        }
    }
}

fn fix_unset_cast(tokens: &mut Tokens, index: usize) {
    let Some(mut assignment_index) = tokens.get_prev_meaningful_token(index).filter(|&i| tokens[i].equals("=", true)) else {
        return;
    };
    let Some(var_index) = tokens.get_next_meaningful_token(index).filter(|&i| tokens[i].is_kind(T_VARIABLE)) else {
        return;
    };
    let after_var = tokens.get_next_meaningful_token(var_index);
    if !after_var.is_some_and(|i| tokens[i].equals_any(&[";".into(), T_CLOSE_TAG.into()], true)) {
        return;
    }
    let next_is_white_space = tokens[assignment_index + 1].is_whitespace(None);
    tokens.clear_token_and_merge_surrounding_whitespace(index);
    tokens.clear_token_and_merge_surrounding_whitespace(var_index);
    assignment_index += 1;
    if !next_is_white_space {
        tokens.insert_at(assignment_index, vec![Token::new(T_WHITESPACE, " ")]);
    }
    assignment_index += 1;
    tokens.insert_at(assignment_index, vec![Token::new(T_STRING, "null")]);
}
