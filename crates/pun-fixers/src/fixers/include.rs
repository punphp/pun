//! Port of `PhpCsFixer\Fixer\ControlStructure\IncludeFixer`.
use crate::analyzers::blocks_analyzer::BlocksAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct IncludeFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(IncludeFixer)
}

struct Includy {
    begin: usize,
    braces: Option<(usize, usize)>,
    end: usize,
}

impl Fixer for IncludeFixer {
    fn name(&self) -> &'static str {
        "include"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_REQUIRE, T_REQUIRE_ONCE, T_INCLUDE, T_INCLUDE_ONCE])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let includies = find_includies(tokens);
        clear_includies(tokens, includies);
    }
}

fn clear_includies(tokens: &mut Tokens, includies: Vec<Includy>) {
    let blocks_analyzer = BlocksAnalyzer;
    for includy in includies {
        if !tokens[includy.end].is_kind(T_CLOSE_TAG) {
            let after_end_index = tokens.get_next_non_whitespace(includy.end, None);
            if after_end_index.is_none_or(|i| !tokens[i].is_comment()) {
                tokens.remove_leading_whitespace(includy.end, None);
            }
        }
        let braces = includy.braces;
        if let Some((open, close)) = braces {
            let prev_index = tokens.get_prev_meaningful_token(includy.begin).unwrap();
            let next_index = tokens.get_next_meaningful_token(close).unwrap();
            // Include is also legal as function parameter or condition statement but requires being wrapped then.
            if !tokens[next_index].equals_any(&[";".into(), T_CLOSE_TAG.into()], true)
                && !blocks_analyzer.is_block(tokens, prev_index, next_index)
            {
                continue;
            }
            remove_whitespace_around_if_possible(tokens, open);
            remove_whitespace_around_if_possible(tokens, close);
            tokens.clear_token_and_merge_surrounding_whitespace(open);
            tokens.clear_token_and_merge_surrounding_whitespace(close);
        }
        let next_index = tokens.get_non_empty_sibling(includy.begin, 1).unwrap();
        if tokens[next_index].is_whitespace(None) {
            tokens.set(next_index, Token::new(T_WHITESPACE, " "));
        } else if braces.is_some()
            || tokens[next_index].is_given_kind(&[
                T_VARIABLE,
                T_CONSTANT_ENCAPSED_STRING,
                T_COMMENT,
            ])
        {
            tokens.insert_at(includy.begin + 1, vec![Token::new(T_WHITESPACE, " ")]);
        }
    }
}

fn find_includies(tokens: &Tokens) -> Vec<Includy> {
    let mut includies = Vec::new();
    for (_, indices) in tokens.find_given_kinds(
        &[T_REQUIRE, T_REQUIRE_ONCE, T_INCLUDE, T_INCLUDE_ONCE],
        0,
        None,
    ) {
        for index in indices {
            let end = tokens
                .get_next_token_of_kind(index, &[";".into(), T_CLOSE_TAG.into()], true)
                .unwrap();
            let brace_open_index = tokens.get_next_meaningful_token(index).unwrap();
            let braces = tokens[brace_open_index].equals("(", true).then(|| {
                (
                    brace_open_index,
                    tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, brace_open_index),
                )
            });
            includies.push(Includy {
                begin: index,
                braces,
                end,
            });
        }
    }
    // krsort
    includies.sort_by(|a, b| b.begin.cmp(&a.begin));
    includies
}

fn remove_whitespace_around_if_possible(tokens: &mut Tokens, index: usize) {
    let next_index = tokens.get_next_non_whitespace(index, None);
    if next_index.is_none_or(|i| !tokens[i].is_comment()) {
        tokens.remove_leading_whitespace(index, None);
    }
    let prev_index = tokens.get_prev_non_whitespace(index, None);
    if prev_index.is_none_or(|i| !tokens[i].is_comment()) {
        tokens.remove_trailing_whitespace(index, None);
    }
}
