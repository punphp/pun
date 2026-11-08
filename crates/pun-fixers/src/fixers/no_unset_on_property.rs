//! Port of `PhpCsFixer\Fixer\LanguageConstruct\NoUnsetOnPropertyFixer`.
use crate::analyzers::arguments_analyzer::ArgumentsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct NoUnsetOnPropertyFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(NoUnsetOnPropertyFixer)
}

/// `_UnsetInfo`.
struct UnsetInfo {
    start_index: usize,
    end_index: usize,
    is_to_transform: bool,
    is_first: bool,
}

fn get_unsets_info(tokens: &Tokens, index: usize) -> Vec<UnsetInfo> {
    let unset_start = tokens
        .get_next_token_of_kind(index, &["(".into()], true)
        .unwrap();
    let unset_end = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, unset_start);
    let mut is_first = true;
    let mut unsets = Vec::new();
    for (start_index, end_index) in ArgumentsAnalyzer.get_arguments(tokens, unset_start, unset_end)
    {
        let start_index = tokens.get_next_meaningful_token(start_index - 1).unwrap();
        let end_index = tokens.get_prev_meaningful_token(end_index + 1).unwrap();
        unsets.push(UnsetInfo {
            start_index,
            end_index,
            is_to_transform: is_property(tokens, start_index, end_index),
            is_first,
        });
        is_first = false;
    }
    unsets
}

fn is_property(tokens: &Tokens, index: usize, end_index: usize) -> bool {
    if tokens[index].is_kind(T_VARIABLE) {
        // An argument of `unset(` is always followed by `,` or `)`.
        let next_index = tokens.get_next_meaningful_token(index).unwrap();
        if !tokens[next_index].is_kind(T_OBJECT_OPERATOR) {
            return false;
        }
        // An object operator is never the last token of a valid argument.
        let next_index = tokens.get_next_meaningful_token(next_index).unwrap();
        if tokens
            .get_next_meaningful_token(next_index)
            .is_some_and(|n| n < end_index)
        {
            return false;
        }
        return tokens[next_index].is_kind(T_STRING);
    }
    if tokens[index].is_given_kind(&[T_NS_SEPARATOR, T_STRING]) {
        let next_index = tokens
            .get_token_not_of_kinds_sibling(index, 1, &[T_DOUBLE_COLON, T_NS_SEPARATOR, T_STRING])
            .unwrap();
        if tokens
            .get_next_meaningful_token(next_index)
            .is_some_and(|n| n < end_index)
        {
            return false;
        }
        return tokens[next_index].is_kind(T_VARIABLE);
    }
    false
}

fn update_tokens(tokens: &mut Tokens, unset_info: &UnsetInfo, is_last_unset: bool) {
    // if entry is first and to be transformed we remove leading "unset("
    if unset_info.is_first && unset_info.is_to_transform {
        let brace_index = tokens
            .get_prev_token_of_kind(unset_info.start_index, &["(".into()], true)
            .unwrap();
        let unset_index = tokens
            .get_prev_token_of_kind(brace_index, &[T_UNSET.into()], true)
            .unwrap();
        tokens.clear_token_and_merge_surrounding_whitespace(brace_index);
        tokens.clear_token_and_merge_surrounding_whitespace(unset_index);
    }
    // if entry is last and to be transformed we remove trailing ")"
    if is_last_unset && unset_info.is_to_transform {
        let brace_index = tokens
            .get_next_token_of_kind(unset_info.end_index, &[")".into()], true)
            .unwrap();
        let previous_index = tokens.get_prev_meaningful_token(brace_index).unwrap();
        if tokens[previous_index].equals(",", true) {
            tokens.clear_token_and_merge_surrounding_whitespace(previous_index); // trailing ',' in function call (PHP 7.3)
        }
        tokens.clear_token_and_merge_surrounding_whitespace(brace_index);
    }
    // if entry is not last we replace comma with semicolon (last entry already has semicolon - from original unset)
    if !is_last_unset {
        let comma_index = tokens
            .get_next_token_of_kind(unset_info.end_index, &[",".into()], true)
            .unwrap();
        tokens.set(comma_index, Token::char(";"));
    }
    // if entry is to be unset and is not last we add trailing ")"
    if !unset_info.is_to_transform && !is_last_unset {
        tokens.insert_at(unset_info.end_index + 1, vec![Token::char(")")]);
    }
    // if entry is to be unset and is not first we add leading "unset("
    if !unset_info.is_to_transform && !unset_info.is_first {
        tokens.insert_at(
            unset_info.start_index,
            vec![Token::new(T_UNSET, "unset"), Token::char("(")],
        );
    }
    // and finally
    // if entry is to be transformed we add trailing " = null"
    if unset_info.is_to_transform {
        tokens.insert_at(
            unset_info.end_index + 1,
            vec![
                Token::new(T_WHITESPACE, " "),
                Token::char("="),
                Token::new(T_WHITESPACE, " "),
                Token::new(T_STRING, "null"),
            ],
        );
    }
}

impl Fixer for NoUnsetOnPropertyFixer {
    fn name(&self) -> &'static str {
        "no_unset_on_property"
    }
    fn priority(&self) -> i32 {
        25
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_UNSET)
            && tokens.is_any_token_kinds_found(&[T_OBJECT_OPERATOR, T_DOUBLE_COLON])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (0..tokens.count()).rev() {
            if !tokens[index].is_kind(T_UNSET) {
                continue;
            }
            let unsets_info = get_unsets_info(tokens, index);
            if !unsets_info.iter().any(|u| u.is_to_transform) {
                continue;
            }
            let mut is_last_unset = true; // "last" as we reverse the array below
            for unset_info in unsets_info.iter().rev() {
                update_tokens(tokens, unset_info, is_last_unset);
                is_last_unset = false;
            }
        }
    }
}
