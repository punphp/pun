//! Port of `PhpCsFixer\Fixer\ArrayNotation\YieldFromArrayToYieldsFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::collections::BTreeMap;
use std::path::Path;

struct YieldFromArrayToYieldsFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(YieldFromArrayToYieldsFixer)
}

impl Fixer for YieldFromArrayToYieldsFixer {
    fn name(&self) -> &'static str {
        "yield_from_array_to_yields"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[T_YIELD_FROM])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let yield_tokens = || vec![Token::new(T_YIELD, "yield"), Token::new(T_WHITESPACE, " ")];
        let mut inserts: BTreeMap<usize, Vec<Token>> = BTreeMap::new();
        let tokens_count = tokens.count();
        // getYieldsFromToUnpack is a generator: each candidate is found on the tokens as fixed so far.
        for index in 1..tokens_count {
            let Some((start_index, end_index)) = yield_from_to_unpack(tokens, index) else {
                continue;
            };
            tokens.clear_token_and_merge_surrounding_whitespace(index);
            if tokens[start_index].equals("(", true) {
                let prev_start_index = tokens.get_prev_meaningful_token(start_index).unwrap();
                tokens.clear_token_and_merge_surrounding_whitespace(prev_start_index);
            }
            tokens.clear_token_and_merge_surrounding_whitespace(start_index);
            tokens.clear_token_and_merge_surrounding_whitespace(end_index);
            let mut array_has_trailing_comma = false;
            let start_index = tokens.get_next_meaningful_token(start_index).unwrap();
            inserts.insert(start_index, yield_tokens());
            let last = tokens.get_prev_meaningful_token(end_index).unwrap();
            let mut i = start_index;
            while i <= last {
                if let Some((block_type, true)) = Tokens::detect_block_type(&tokens[i]) {
                    i = tokens.find_block_end(block_type, i) + 1;
                    continue;
                }
                if tokens[i].equals(",", true) {
                    let next_item_index = tokens.get_next_meaningful_token(i).unwrap();
                    if next_item_index < end_index {
                        inserts.insert(next_item_index, yield_tokens());
                    } else {
                        array_has_trailing_comma = true;
                    }
                    tokens.set(i, Token::char(";"));
                }
                i += 1;
            }
            if array_has_trailing_comma {
                let next = tokens.get_next_meaningful_token(end_index).unwrap();
                tokens.clear_token_and_merge_surrounding_whitespace(next);
            }
        }
        tokens.insert_slices(inserts);
    }
}

fn yield_from_to_unpack(tokens: &Tokens, index: usize) -> Option<(usize, usize)> {
    if !tokens[index].is_kind(T_YIELD_FROM) {
        return None;
    }
    let prev_index = tokens.get_prev_meaningful_token(index)?;
    if !tokens[prev_index].equals_any(
        &[";".into(), "{".into(), "}".into(), T_OPEN_TAG.into()],
        true,
    ) {
        return None;
    }
    let array_start_index = tokens.get_next_meaningful_token(index)?;
    let (start_index, end_index) = if tokens[array_start_index].is_kind(T_ARRAY) {
        let start = tokens
            .get_next_token_of_kind(array_start_index, &["(".into()], true)
            .unwrap();
        (
            start,
            tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, start),
        )
    } else if tokens[array_start_index].is_kind(CT_ARRAY_BRACKET_OPEN) {
        (
            array_start_index,
            tokens.find_block_end(Tokens::BLOCK_TYPE_ARRAY_BRACKET, array_start_index),
        )
    } else {
        return None;
    };
    if Some(end_index) == tokens.get_next_meaningful_token(start_index) {
        return None;
    }
    if !tokens
        .find_given_kind(T_YIELD_FROM, start_index, Some(end_index))
        .is_empty()
    {
        return None;
    }
    Some((start_index, end_index))
}
