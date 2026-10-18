//! Port of `PhpCsFixer\Fixer\Operator\StandardizeIncrementFixer`.
use crate::analyzers::abstract_increment_operator_fixer::find_start;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Proto, Token, Tokens};
use std::path::Path;

struct StandardizeIncrementFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(StandardizeIncrementFixer)
}

const EXPRESSION_END_TOKENS: [Proto<'static>; 8] = [
    Proto::Char(b";"),
    Proto::Char(b")"),
    Proto::Char(b"]"),
    Proto::Char(b","),
    Proto::Char(b":"),
    Proto::Kind(CT_DYNAMIC_PROP_BRACE_CLOSE),
    Proto::Kind(CT_DYNAMIC_VAR_BRACE_CLOSE),
    Proto::Kind(T_CLOSE_TAG),
];

impl Fixer for StandardizeIncrementFixer {
    fn name(&self) -> &'static str {
        "standardize_increment"
    }
    fn priority(&self) -> i32 {
        16
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_PLUS_EQUAL, T_MINUS_EQUAL])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (1..tokens.count()).rev() {
            if !tokens[index].equals_any(&EXPRESSION_END_TOKENS, true) {
                continue;
            }
            let number_index = tokens.get_prev_meaningful_token(index).unwrap();
            let number = &tokens[number_index];
            if !number.is_kind(T_LNUMBER) || number.get_content() != b"1" {
                continue;
            }
            let operator_index = tokens.get_prev_meaningful_token(number_index).unwrap();
            let operator = &tokens[operator_index];
            if !operator.is_given_kind(&[T_PLUS_EQUAL, T_MINUS_EQUAL]) {
                continue;
            }
            let token = if operator.is_kind(T_PLUS_EQUAL) {
                Token::new(T_INC, "++")
            } else {
                Token::new(T_DEC, "--")
            };
            let start_index = find_start(tokens, operator_index);
            let clear_from = tokens.get_prev_meaningful_token(operator_index).unwrap() + 1;
            clear_range_leave_comments(tokens, clear_from, number_index);
            tokens.insert_at(start_index, vec![token]);
        }
    }
}

/// Clear tokens in the given range unless they are comments.
fn clear_range_leave_comments(tokens: &mut Tokens, index_start: usize, index_end: usize) {
    for i in index_start..=index_end {
        let token = &tokens[i];
        if token.is_comment() || token.is_whitespace(Some(b"\n\r")) {
            continue;
        }
        tokens.clear_at(i);
    }
}
