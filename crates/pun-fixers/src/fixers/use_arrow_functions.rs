//! Port of `PhpCsFixer\Fixer\FunctionNotation\UseArrowFunctionsFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::tokens_analyzer::TokensAnalyzer;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct UseArrowFunctionsFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(UseArrowFunctionsFixer)
}

impl Fixer for UseArrowFunctionsFixer {
    fn name(&self) -> &'static str {
        "use_arrow_functions"
    }
    fn priority(&self) -> i32 {
        32
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[T_FUNCTION, T_RETURN])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = tokens.count() - 1;
        while index > 0 {
            let current = index;
            index -= 1;
            if tokens[current].is_kind(CT_ATTRIBUTE_CLOSE) {
                index = tokens.find_block_start(Tokens::BLOCK_TYPE_ATTRIBUTE, current) - 1;
                continue;
            }
            if !tokens[current].is_kind(T_FUNCTION)
                || !TokensAnalyzer::new(tokens).is_lambda(current)
            {
                continue;
            }
            fix_lambda(tokens, current);
        }
    }
}

fn fix_lambda(tokens: &mut Tokens, index: usize) {
    let mut parameters_start = tokens.get_next_meaningful_token(index).unwrap();
    if tokens[parameters_start].is_kind(CT_RETURN_REF) {
        parameters_start = tokens.get_next_meaningful_token(parameters_start).unwrap();
    }
    let parameters_end = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, parameters_start);

    let mut next = tokens.get_next_meaningful_token(parameters_end).unwrap();
    let mut use_range = None;
    if tokens[next].is_kind(CT_USE_LAMBDA) {
        let mut use_start = next;
        if tokens[use_start - 1].is_kind(T_WHITESPACE) {
            use_start -= 1;
        }
        next = tokens.get_next_meaningful_token(next).unwrap();
        while !tokens[next].equals(")", true) {
            if tokens[next].equals("&", true) {
                // variables used by reference are not supported by arrow functions
                return;
            }
            next = tokens.get_next_meaningful_token(next).unwrap();
        }
        use_range = Some((use_start, next));
        next = tokens.get_next_meaningful_token(next).unwrap();
    }

    let brace_open = if tokens[next].equals("{", true) {
        next
    } else {
        tokens
            .get_next_token_of_kind(next, &["{".into()], true)
            .unwrap()
    };
    let mut ret = brace_open + 1;
    if tokens[ret].is_kind(T_WHITESPACE) {
        ret += 1;
    }
    if !tokens[ret].is_kind(T_RETURN) {
        return;
    }

    let semicolon = tokens
        .get_next_token_of_kind(ret, &["{".into(), ";".into()], true)
        .unwrap();
    if !tokens[semicolon].equals(";", true) {
        return;
    }

    let mut brace_close = semicolon + 1;
    if tokens[brace_close].is_kind(T_WHITESPACE) {
        brace_close += 1;
    }
    if !tokens[brace_close].equals("}", true) {
        return;
    }

    // Abort if closure has `use()` clause and return statement includes external files.
    if use_range.is_some()
        && (ret..semicolon).any(|i| {
            tokens[i].is_given_kind(&[T_INCLUDE, T_INCLUDE_ONCE, T_REQUIRE, T_REQUIRE_ONCE])
        })
    {
        return;
    }

    // transform
    let mut tokens_to_insert = vec![Token::new(T_DOUBLE_ARROW, "=>")];
    if tokens.get_next_meaningful_token(ret) == Some(semicolon) {
        tokens_to_insert.push(Token::new(T_WHITESPACE, " "));
        tokens_to_insert.push(Token::new(T_STRING, "null"));
    }
    tokens.clear_range(semicolon, brace_close - 1);
    tokens.clear_token_and_merge_surrounding_whitespace(brace_close);
    tokens.clear_range(brace_open + 1, ret);
    tokens.override_range(brace_open, brace_open, tokens_to_insert);
    if let Some((use_start, use_end)) = use_range {
        tokens.clear_range(use_start, use_end);
    }
    tokens.set(index, Token::new(T_FN, "fn"));
}
