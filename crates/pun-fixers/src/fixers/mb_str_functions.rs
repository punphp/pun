//! Port of `PhpCsFixer\Fixer\Alias\MbStrFunctionsFixer` (PHP 8.5 map; alternatives filtered by
//! existence in the oracle PHP like upstream's `ReflectionFunction::isInternal`).
use crate::analyzers::arguments_analyzer::ArgumentsAnalyzer;
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use crate::analyzers::php_functions::is_internal_function;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct MbStrFunctionsFixer {
    functions: Vec<(&'static str, &'static str, &'static [usize])>,
}

const FUNCTIONS_MAP: &[(&str, &str, &[usize])] = &[
    ("str_split", "mb_str_split", &[1, 2, 3]),
    ("stripos", "mb_stripos", &[2, 3]),
    ("stristr", "mb_stristr", &[2, 3]),
    ("strlen", "mb_strlen", &[1]),
    ("strpos", "mb_strpos", &[2, 3]),
    ("strrchr", "mb_strrchr", &[2]),
    ("strripos", "mb_strripos", &[2, 3]),
    ("strrpos", "mb_strrpos", &[2, 3]),
    ("strstr", "mb_strstr", &[2, 3]),
    ("strtolower", "mb_strtolower", &[1]),
    ("strtoupper", "mb_strtoupper", &[1]),
    ("substr", "mb_substr", &[2, 3]),
    ("substr_count", "mb_substr_count", &[2, 3, 4]),
    ("str_pad", "mb_str_pad", &[1, 2, 3, 4]),
    ("trim", "mb_trim", &[1, 2]),
    ("ltrim", "mb_ltrim", &[1, 2]),
    ("rtrim", "mb_rtrim", &[1, 2]),
];

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(MbStrFunctionsFixer {
        functions: FUNCTIONS_MAP
            .iter()
            .copied()
            .filter(|(_, alt, _)| is_internal_function(alt.as_bytes()))
            .collect(),
    })
}

impl Fixer for MbStrFunctionsFixer {
    fn name(&self) -> &'static str {
        "mb_str_functions"
    }
    fn priority(&self) -> i32 {
        2
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let arguments_analyzer = ArgumentsAnalyzer;
        let functions_analyzer = FunctionsAnalyzer::new();
        for index in (1..tokens.count()).rev() {
            if !tokens[index].is_kind(T_STRING) {
                continue;
            }
            let content = &tokens[index].content;
            let Some(&(_, alternative_name, argument_count)) = self
                .functions
                .iter()
                .find(|(n, _, _)| n.as_bytes().eq_ignore_ascii_case(content))
            else {
                continue;
            };
            if functions_analyzer.is_global_function_call(tokens, index) {
                let open_parenthesis = tokens.get_next_meaningful_token(index).unwrap();
                let close_parenthesis =
                    tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_parenthesis);
                let number_of_arguments =
                    arguments_analyzer.count_arguments(tokens, open_parenthesis, close_parenthesis);
                if !argument_count.contains(&number_of_arguments) {
                    continue;
                }
                tokens.set(index, Token::new(T_STRING, alternative_name));
                continue;
            }
            let mut function_index = tokens.get_prev_meaningful_token(index).unwrap();
            if tokens[function_index].is_kind(T_NS_SEPARATOR) {
                function_index = tokens.get_prev_meaningful_token(function_index).unwrap();
            }
            if !tokens[function_index].is_kind(CT_FUNCTION_IMPORT) {
                continue;
            }
            tokens.set(index, Token::new(T_STRING, alternative_name));
        }
    }
}
