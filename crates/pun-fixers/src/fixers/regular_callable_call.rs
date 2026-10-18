//! Port of `PhpCsFixer\Fixer\FunctionNotation\RegularCallableCallFixer`.
use crate::analyzers::arguments_analyzer::ArgumentsAnalyzer;
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::{str_replace, trim};
use pun_tokens::{Token, Tokens};
use std::collections::BTreeMap;
use std::path::Path;

struct RegularCallableCallFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(RegularCallableCallFixer)
}

impl Fixer for RegularCallableCallFixer {
    fn name(&self) -> &'static str {
        "regular_callable_call"
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
        let functions_analyzer = FunctionsAnalyzer::new();
        let arguments_analyzer = ArgumentsAnalyzer;
        for index in (1..tokens.count()).rev() {
            if !tokens[index].equals_any(
                &[
                    (T_STRING, "call_user_func").into(),
                    (T_STRING, "call_user_func_array").into(),
                ],
                false,
            ) {
                continue;
            }
            if !functions_analyzer.is_global_function_call(tokens, index) {
                continue; // redeclare/override
            }
            let open_parenthesis = tokens.get_next_meaningful_token(index).unwrap();
            let close_parenthesis =
                tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_parenthesis);
            let arguments =
                arguments_analyzer.get_arguments(tokens, open_parenthesis, close_parenthesis);
            if arguments.is_empty() {
                return; // no arguments!
            }
            process_call(tokens, index, &arguments);
        }
    }
}

fn process_call(tokens: &mut Tokens, index: usize, arguments: &BTreeMap<usize, usize>) {
    let first_arg_index = tokens
        .get_next_meaningful_token(tokens.get_next_meaningful_token(index).unwrap())
        .unwrap();
    let first_arg_token = tokens[first_arg_index].clone();
    if first_arg_token.is_kind(T_CONSTANT_ENCAPSED_STRING) {
        let after_first_arg_index = tokens.get_next_meaningful_token(first_arg_index).unwrap();
        if !tokens[after_first_arg_index].equals_any(&[",".into(), ")".into()], true) {
            return; // first argument is an expression like `call_user_func("foo"."bar", ...)`, not supported!
        }
        let content = first_arg_token.get_content();
        if !is_valid_function_invoke(content) {
            return;
        }
        let unescaped = str_replace(b"\\\\", b"\\", content);
        let mut code = b"<?php ".to_vec();
        code.extend_from_slice(&unescaped[1..unescaped.len() - 1]);
        code.extend_from_slice(b"();");
        let new_call_tokens = Tokens::from_code(&code);
        let size = new_call_tokens.count();
        let new_call_tokens: Vec<Token> = new_call_tokens.as_slice()[1..size - 3]
            .iter()
            .filter(|t| !t.get_content().is_empty())
            .cloned()
            .collect();
        replace_call_user_func_with_callback(
            tokens,
            index,
            new_call_tokens,
            first_arg_index,
            first_arg_index,
        );
    } else if first_arg_token.is_kind(T_FUNCTION)
        || (first_arg_token.is_kind(T_STATIC)
            && tokens[tokens.get_next_meaningful_token(first_arg_index).unwrap()]
                .is_kind(T_FUNCTION))
    {
        let first_arg_end_index = tokens.find_block_end(
            Tokens::BLOCK_TYPE_BRACE,
            tokens
                .get_next_token_of_kind(first_arg_index, &["{".into()], true)
                .unwrap(),
        );
        let mut new_call_tokens = vec![Token::char("(")];
        new_call_tokens
            .extend_from_slice(&tokens.as_slice()[first_arg_index..=first_arg_end_index]);
        new_call_tokens.push(Token::char(")"));
        replace_call_user_func_with_callback(
            tokens,
            index,
            new_call_tokens,
            first_arg_index,
            first_arg_end_index,
        );
    } else if first_arg_token.is_kind(T_VARIABLE) {
        // `reset($arguments)`: the end of the first argument.
        let first_arg_end_index = *arguments.values().next().unwrap();
        for (&argument_start, &argument_end) in arguments {
            if first_arg_end_index == argument_end {
                continue;
            }
            for i in argument_start..=argument_end {
                if tokens[i].equals_token(&first_arg_token, true) {
                    return;
                }
            }
        }
        let new_call_tokens =
            Tokens::from_array(tokens.as_slice()[first_arg_index..=first_arg_end_index].to_vec());
        let mut complex = false;
        let mut new_call_index = new_call_tokens.count() as isize - 1;
        while new_call_index >= 0 {
            let token = &new_call_tokens[new_call_index as usize];
            if token.is_given_kind(&[T_WHITESPACE, T_COMMENT, T_DOC_COMMENT, T_VARIABLE]) {
                new_call_index -= 1;
                continue;
            }
            if let Some((block_type, _)) = Tokens::detect_block_type(token)
                && (block_type == Tokens::BLOCK_TYPE_INDEX_BRACE
                    || block_type == Tokens::BLOCK_TYPE_INDEX_BRACKET)
            {
                new_call_index =
                    new_call_tokens.find_block_start(block_type, new_call_index as usize) as isize
                        - 1;
                continue;
            }
            complex = true;
            break;
        }
        let mut new_call_tokens = new_call_tokens.as_slice().to_vec();
        if complex {
            new_call_tokens.push(Token::char(")"));
            new_call_tokens.insert(0, Token::char("("));
        }
        replace_call_user_func_with_callback(
            tokens,
            index,
            new_call_tokens,
            first_arg_index,
            first_arg_end_index,
        );
    }
}

fn replace_call_user_func_with_callback(
    tokens: &mut Tokens,
    call_index: usize,
    new_call_tokens: Vec<Token>,
    first_arg_start_index: usize,
    first_arg_end_index: usize,
) {
    tokens.clear_range(first_arg_start_index, first_arg_end_index);
    let after_first_arg_index = tokens
        .get_next_meaningful_token(first_arg_end_index)
        .unwrap();
    if tokens[after_first_arg_index].equals(",", true) {
        let use_ellipsis = tokens[call_index].equals((T_STRING, "call_user_func_array"), false);
        if use_ellipsis {
            let second_arg_index = tokens
                .get_next_meaningful_token(after_first_arg_index)
                .unwrap();
            tokens.insert_at(second_arg_index, vec![Token::new(T_ELLIPSIS, "...")]);
        }
        tokens.clear_at(after_first_arg_index);
        tokens.remove_trailing_whitespace(after_first_arg_index, None);
    }
    tokens.override_range(call_index, call_index, new_call_tokens);
    let prev_index = tokens.get_prev_meaningful_token(call_index).unwrap();
    if tokens[prev_index].is_kind(T_NS_SEPARATOR) {
        tokens.clear_token_and_merge_surrounding_whitespace(prev_index);
    }
}

fn is_valid_function_invoke(name: &[u8]) -> bool {
    if name.len() < 3 || name[0] == b'b' || name[0] == b'B' {
        return false;
    }
    let name = &name[1..name.len() - 1];
    name == trim(name)
}
