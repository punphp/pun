//! Port of `PhpCsFixer\Fixer\ArrayNotation\ReturnToYieldFromFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct ReturnToYieldFromFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(ReturnToYieldFromFixer)
}

impl Fixer for ReturnToYieldFromFixer {
    fn name(&self) -> &'static str {
        "return_to_yield_from"
    }
    fn priority(&self) -> i32 {
        1
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[T_FUNCTION, T_RETURN])
            && tokens.is_any_token_kinds_found(&[T_ARRAY, CT_ARRAY_BRACKET_OPEN])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in tokens.find_given_kind(T_RETURN, 0, None) {
            if should_be_fixed(tokens, index) {
                tokens.set(index, Token::new(T_YIELD_FROM, "yield from"));
            }
        }
    }
}

fn should_be_fixed(tokens: &Tokens, return_index: usize) -> bool {
    // A `return` is always followed by an expression or `;`.
    let array_start_index = tokens.get_next_meaningful_token(return_index).unwrap();
    if !tokens[array_start_index].is_given_kind(&[T_ARRAY, CT_ARRAY_BRACKET_OPEN]) {
        return false;
    }
    let array_end_index = if tokens[array_start_index].is_kind(CT_ARRAY_BRACKET_OPEN) {
        tokens.find_block_end(Tokens::BLOCK_TYPE_ARRAY_BRACKET, array_start_index)
    } else {
        let open = tokens
            .get_next_token_of_kind(array_start_index, &["(".into()], true)
            .unwrap();
        tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open)
    };
    let mut function_end_index = Some(array_end_index);
    loop {
        function_end_index = tokens.get_next_meaningful_token(function_end_index.unwrap());
        match function_end_index {
            Some(i) if tokens[i].equals(";", true) => continue,
            _ => break,
        }
    }
    let Some(function_end_index) = function_end_index.filter(|&i| tokens[i].equals("}", true))
    else {
        return false;
    };
    let function_start_index =
        tokens.find_block_start(Tokens::BLOCK_TYPE_BRACE, function_end_index);
    let return_type_index = tokens
        .get_prev_meaningful_token(function_start_index)
        .unwrap();
    if !tokens[return_type_index].is_kind(T_STRING) {
        return false;
    }
    if strtolower(tokens[return_type_index].get_content()) != b"iterable" {
        return false;
    }
    let before_return_type_index = tokens.get_prev_meaningful_token(return_type_index).unwrap();
    tokens[before_return_type_index].is_kind(CT_TYPE_COLON)
}
