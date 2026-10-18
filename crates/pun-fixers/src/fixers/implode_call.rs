//! Port of `PhpCsFixer\Fixer\FunctionNotation\ImplodeCallFixer`.
use crate::analyzers::arguments_analyzer::ArgumentsAnalyzer;
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct ImplodeCallFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(ImplodeCallFixer)
}

impl Fixer for ImplodeCallFixer {
    fn name(&self) -> &'static str {
        "implode_call"
    }
    fn priority(&self) -> i32 {
        37
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let functions_analyzer = FunctionsAnalyzer::new();
        for index in (1..tokens.count()).rev() {
            if !tokens[index].equals((T_STRING, "implode"), false) {
                continue;
            }
            if !functions_analyzer.is_global_function_call(tokens, index) {
                continue;
            }
            let arguments_indices = get_argument_indices(tokens, index);
            if arguments_indices.len() == 1 {
                tokens.insert_at(
                    arguments_indices[0].0,
                    vec![
                        Token::new(T_CONSTANT_ENCAPSED_STRING, "''"),
                        Token::char(","),
                        Token::new(T_WHITESPACE, " "),
                    ],
                );
                continue;
            }
            if arguments_indices.len() == 2 {
                let (first_argument_index, first_argument_end_index) = arguments_indices[0];
                let mut second_argument_index = arguments_indices[1].0;
                if tokens[first_argument_index].is_kind(T_CONSTANT_ENCAPSED_STRING) {
                    continue;
                }
                if !tokens[second_argument_index].is_kind(T_CONSTANT_ENCAPSED_STRING) {
                    continue;
                }
                let mut new_second_argument_tokens = vec![];
                for i in first_argument_index..=first_argument_end_index {
                    new_second_argument_tokens.push(tokens[i].clone());
                    tokens.clear_at(i);
                }
                let second = tokens[second_argument_index].clone();
                tokens.insert_at(first_argument_index, vec![second]);
                second_argument_index += 1;
                tokens.clear_at(second_argument_index);
                tokens.insert_at(second_argument_index, new_second_argument_tokens);
            }
        }
    }
}

/// First meaningful token => last meaningful token of each argument, in order.
fn get_argument_indices(tokens: &Tokens, function_name_index: usize) -> Vec<(usize, usize)> {
    let open_parenthesis = tokens
        .get_next_token_of_kind(function_name_index, &["(".into()], true)
        .unwrap();
    let close_parenthesis = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_parenthesis);
    let mut indices: Vec<(usize, usize)> = vec![];
    for (start_index_candidate, end_index) in
        ArgumentsAnalyzer.get_arguments(tokens, open_parenthesis, close_parenthesis)
    {
        let start = tokens
            .get_next_meaningful_token(start_index_candidate - 1)
            .unwrap();
        let end = tokens.get_prev_meaningful_token(end_index + 1).unwrap();
        indices.push((start, end));
    }
    indices
}
