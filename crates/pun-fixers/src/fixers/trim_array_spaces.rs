//! Port of `PhpCsFixer\Fixer\ArrayNotation\TrimArraySpacesFixer`.
use pun_format::Fixer;
use pun_tokens::{Kind::*, Tokens};
use std::path::Path;

struct TrimArraySpacesFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(TrimArraySpacesFixer)
}

impl Fixer for TrimArraySpacesFixer {
    fn name(&self) -> &'static str {
        "trim_array_spaces"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[
            T_ARRAY,
            CT_ARRAY_BRACKET_OPEN,
            CT_DESTRUCTURING_BRACKET_OPEN,
        ])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let c = tokens.count();
        for index in 0..c {
            if tokens[index].is_given_kind(&[
                T_ARRAY,
                CT_ARRAY_BRACKET_OPEN,
                CT_DESTRUCTURING_BRACKET_OPEN,
            ]) {
                fix_array(tokens, index);
            }
        }
    }
}

fn fix_array(tokens: &mut Tokens, index: usize) {
    let mut start_index = index;

    let end_index = if tokens[start_index].is_kind(T_ARRAY) {
        start_index = tokens.get_next_meaningful_token(start_index).unwrap();
        tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, start_index)
    } else if tokens[start_index].is_kind(CT_DESTRUCTURING_BRACKET_OPEN) {
        tokens.find_block_end(Tokens::BLOCK_TYPE_DESTRUCTURING_BRACKET, start_index)
    } else {
        tokens.find_block_end(Tokens::BLOCK_TYPE_ARRAY_BRACKET, start_index)
    };

    let next_index = start_index + 1;
    let next_non_whitespace_index = tokens.get_next_non_whitespace(start_index, None).unwrap();
    let prev_index = end_index - 1;
    let prev_non_whitespace_index = tokens.get_prev_non_whitespace(end_index, None).unwrap();

    let next_non_whitespace_token = &tokens[next_non_whitespace_index];
    if tokens[next_index].is_whitespace(Some(b" \t"))
        && (!next_non_whitespace_token.is_comment()
            || next_non_whitespace_index == prev_non_whitespace_index
            || tokens[next_non_whitespace_index + 1].is_whitespace(Some(b" \t"))
            || next_non_whitespace_token.content.starts_with(b"/*"))
    {
        tokens.clear_at(next_index);
    }

    if tokens[prev_index].is_whitespace(Some(b" \t"))
        && !tokens[prev_non_whitespace_index].equals(",", true)
    {
        tokens.clear_at(prev_index);
    }
}
