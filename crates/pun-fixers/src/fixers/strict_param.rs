//! Port of `PhpCsFixer\Fixer\Strict\StrictParamFixer`.
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Kind::*, Token, Tokens};
use std::path::Path;

struct StrictParamFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(StrictParamFixer)
}

/// `$map[$name]`: each parameter's default, `None` where the call must already pass it.
fn params(name: &[u8]) -> Option<Vec<Option<Vec<Token>>>> {
    let true_token = || Some(vec![Token::new(T_STRING, "true")]);
    Some(match name {
        b"array_keys" | b"array_search" | b"in_array" => vec![None, None, true_token()],
        b"base64_decode" => vec![None, true_token()],
        b"mb_detect_encoding" => vec![
            None,
            Some(vec![
                Token::new(T_STRING, "mb_detect_order"),
                Token::char("("),
                Token::char(")"),
            ]),
            true_token(),
        ],
        _ => return None,
    })
}

impl Fixer for StrictParamFixer {
    fn name(&self) -> &'static str {
        "strict_param"
    }
    fn priority(&self) -> i32 {
        31
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let functions_analyzer = FunctionsAnalyzer::new();
        for index in (0..tokens.count()).rev() {
            if let Some(next_index) = tokens.get_next_meaningful_token(index)
                && !tokens[next_index].equals("(", true)
            {
                continue;
            }
            if let Some(function_params) = params(&strtolower(tokens[index].get_content()))
                && functions_analyzer.is_global_function_call(tokens, index)
            {
                fix_function(tokens, index, function_params);
            }
        }
    }
}

fn fix_function(
    tokens: &mut Tokens,
    function_index: usize,
    function_params: Vec<Option<Vec<Token>>>,
) {
    let start_brace_index = tokens
        .get_next_token_of_kind(function_index, &["(".into()], true)
        .unwrap();
    let end_brace_index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, start_brace_index);
    let mut params_quantity = 0;
    let mut expect_param = true;
    let mut index = start_brace_index + 1;
    while index < end_brace_index {
        let token = &tokens[index];
        if expect_param && !token.is_whitespace(None) && !token.is_comment() {
            params_quantity += 1;
            expect_param = false;
        }
        if token.equals("(", true) {
            index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, index);
        } else if token.is_kind(CT_ARRAY_BRACKET_OPEN) {
            index = tokens.find_block_end(Tokens::BLOCK_TYPE_ARRAY_BRACKET, index);
        } else if token.equals(",", true) {
            expect_param = true;
        }
        index += 1;
    }
    if params_quantity == function_params.len() {
        return;
    }
    let mut tokens_to_insert = Vec::new();
    for param in function_params.into_iter().skip(params_quantity) {
        // function call do not have all params that are required to set useStrict flag, exit from method!
        let Some(param) = param else {
            return;
        };
        tokens_to_insert.push(Token::char(","));
        tokens_to_insert.push(Token::new(T_WHITESPACE, " "));
        tokens_to_insert.extend(param);
    }
    let before_end_brace_index = tokens.get_prev_meaningful_token(end_brace_index).unwrap();
    if tokens[before_end_brace_index].equals(",", true) {
        tokens_to_insert.remove(0);
        tokens_to_insert.push(Token::char(","));
    }
    tokens.insert_at(before_end_brace_index + 1, tokens_to_insert);
}
