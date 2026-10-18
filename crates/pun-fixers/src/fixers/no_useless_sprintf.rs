//! Port of `PhpCsFixer\Fixer\FunctionNotation\NoUselessSprintfFixer`.
use crate::analyzers::arguments_analyzer::ArgumentsAnalyzer;
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::Tokens;
use pun_tokens::php_compat::strtolower;
use std::path::Path;

struct NoUselessSprintfFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoUselessSprintfFixer)
}

impl Fixer for NoUselessSprintfFixer {
    fn name(&self) -> &'static str {
        "no_useless_sprintf"
    }
    fn priority(&self) -> i32 {
        42
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let function_analyzer = FunctionsAnalyzer::new();
        let arguments_analyzer = ArgumentsAnalyzer;
        for index in (1..tokens.count()).rev() {
            if !tokens[index].is_kind(T_STRING)
                || strtolower(tokens[index].get_content()) != b"sprintf"
                || !function_analyzer.is_global_function_call(tokens, index)
            {
                continue;
            }
            let open_parenthesis_index = tokens
                .get_next_token_of_kind(index, &["(".into()], true)
                .unwrap();
            let first = tokens
                .get_next_meaningful_token(open_parenthesis_index)
                .unwrap();
            if tokens[first].is_kind(T_ELLIPSIS) {
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
        }
    }
}
