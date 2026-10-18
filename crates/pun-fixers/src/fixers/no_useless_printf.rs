//! Port of `PhpCsFixer\Fixer\FunctionNotation\NoUselessPrintfFixer`.
use crate::analyzers::arguments_analyzer::ArgumentsAnalyzer;
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Token, Tokens};
use std::collections::BTreeMap;
use std::path::Path;

struct NoUselessPrintfFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoUselessPrintfFixer)
}

impl Fixer for NoUselessPrintfFixer {
    fn name(&self) -> &'static str {
        "no_useless_printf"
    }
    fn priority(&self) -> i32 {
        10
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let functions_analyzer = FunctionsAnalyzer::new();
        let arguments_analyzer = ArgumentsAnalyzer;
        let mut printf_indices = BTreeMap::new();
        for index in (1..tokens.count()).rev() {
            if !tokens[index].is_kind(T_STRING) {
                continue;
            }
            if strtolower(tokens[index].get_content()) != b"printf" {
                continue;
            }
            if !functions_analyzer.is_global_function_call(tokens, index) {
                continue;
            }
            let open_parenthesis_index = tokens
                .get_next_token_of_kind(index, &["(".into()], true)
                .unwrap();
            let first = tokens
                .get_next_meaningful_token(open_parenthesis_index)
                .unwrap();
            if tokens[first].is_given_kind(&[T_ELLIPSIS, CT_FIRST_CLASS_CALLABLE]) {
                continue;
            }
            let close_parenthesis_index =
                tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_parenthesis_index);
            if arguments_analyzer.count_arguments(
                tokens,
                open_parenthesis_index,
                close_parenthesis_index,
            ) != 1
            {
                continue;
            }
            tokens.clear_token_and_merge_surrounding_whitespace(close_parenthesis_index);
            let prev = tokens
                .get_prev_meaningful_token(close_parenthesis_index)
                .unwrap();
            if tokens[prev].equals(",", true) {
                tokens.clear_token_and_merge_surrounding_whitespace(prev);
            }
            tokens.clear_token_and_merge_surrounding_whitespace(open_parenthesis_index);
            tokens.clear_token_and_merge_surrounding_whitespace(index);
            let prev = tokens.get_prev_meaningful_token(index).unwrap();
            if tokens[prev].is_kind(T_NS_SEPARATOR) {
                tokens.clear_token_and_merge_surrounding_whitespace(prev);
            }
            printf_indices.insert(
                index,
                vec![Token::new(T_PRINT, "print"), Token::new(T_WHITESPACE, " ")],
            );
        }
        if printf_indices.is_empty() {
            return;
        }
        tokens.insert_slices(printf_indices);
    }
}
