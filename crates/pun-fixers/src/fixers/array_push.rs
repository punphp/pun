//! Port of `PhpCsFixer\Fixer\Alias\ArrayPushFixer`.
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens};
use std::path::Path;

struct ArrayPushFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(ArrayPushFixer)
}

impl Fixer for ArrayPushFixer {
    fn name(&self) -> &'static str {
        "array_push"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING) && tokens.count() > 7
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let functions_analyzer = FunctionsAnalyzer::new();
        let mut index = tokens.count() as isize - 7;
        while index > 0 {
            let i = index as usize;
            if !tokens[i].equals((T_STRING, "array_push"), false) || !functions_analyzer.is_global_function_call(tokens, i) {
                index -= 1;
                continue;
            }
            // meaningful before must be `<?php`, `{`, `}` or `;`
            let call_index = i;
            let mut i = tokens.get_prev_meaningful_token(i).unwrap();
            let mut namespace_separator_index = None;
            if tokens[i].is_kind(T_NS_SEPARATOR) {
                namespace_separator_index = Some(i);
                i = tokens.get_prev_meaningful_token(i).unwrap();
            }
            index = i as isize - 1;
            if !tokens[i].equals_any(&[";".into(), "{".into(), "}".into(), ")".into(), T_OPEN_TAG.into()], true) {
                continue;
            }

            // figure out where the arguments list opens
            let open_brace_index = tokens.get_next_meaningful_token(call_index).unwrap();
            // figure out where the arguments list closes
            let close_brace_index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_brace_index);
            // meaningful after `)` must be `;`, `? >` or nothing
            if tokens
                .get_next_meaningful_token(close_brace_index)
                .is_some_and(|after| !tokens[after].equals_any(&[";".into(), T_CLOSE_TAG.into()], true))
            {
                continue;
            }

            // must have 2 arguments
            // first argument must be a variable (with possibly array indexing etc.),
            // after that nothing meaningful should be there till the next `,` or `)`
            // if `)` than we cannot fix it (it is a single argument call)
            let first_argument_stop = get_first_argument_end(tokens, open_brace_index);
            let first_argument_stop = tokens.get_next_meaningful_token(first_argument_stop).unwrap();
            if !tokens[first_argument_stop].equals(",", true) {
                continue;
            }

            // second argument can be about anything but ellipsis, we must make sure there is not
            // a third argument (or more) passed to `array_push`
            let second_argument_start = tokens.get_next_meaningful_token(first_argument_stop).unwrap();
            if get_second_argument_end(tokens, second_argument_start, close_brace_index).is_none() {
                continue;
            }

            // candidate is valid, replace tokens
            tokens.clear_token_and_merge_surrounding_whitespace(close_brace_index);
            tokens.clear_token_and_merge_surrounding_whitespace(first_argument_stop);
            tokens.insert_at(
                first_argument_stop,
                vec![Token::char("["), Token::char("]"), Token::new(T_WHITESPACE, " "), Token::char("=")],
            );
            tokens.clear_token_and_merge_surrounding_whitespace(open_brace_index);
            tokens.clear_token_and_merge_surrounding_whitespace(call_index);
            if let Some(ns) = namespace_separator_index {
                tokens.clear_token_and_merge_surrounding_whitespace(ns);
            }
        }
    }
}

fn get_first_argument_end(tokens: &Tokens, mut index: usize) -> usize {
    let mut next_index = tokens.get_next_meaningful_token(index).unwrap();
    loop {
        let next_token = &tokens[next_index];
        if !next_token.equals_any(
            &[
                "$".into(), "[".into(), "(".into(), CT_ARRAY_INDEX_BRACE_OPEN.into(), CT_DYNAMIC_PROP_BRACE_OPEN.into(),
                CT_DYNAMIC_VAR_BRACE_OPEN.into(), CT_NAMESPACE_OPERATOR.into(), T_NS_SEPARATOR.into(), T_STATIC.into(),
                T_STRING.into(), T_VARIABLE.into(),
            ],
            true,
        ) {
            break;
        }
        if let Some((block_type, _)) = Tokens::detect_block_type(next_token) {
            next_index = tokens.find_block_end(block_type, next_index);
        }
        index = next_index;
        next_index = tokens.get_next_meaningful_token(next_index).unwrap();
    }
    let next_token = &tokens[next_index];
    if next_token.is_kind(T_OBJECT_OPERATOR) {
        return get_first_argument_end(tokens, next_index);
    }
    if next_token.is_kind(T_DOUBLE_COLON) {
        return get_first_argument_end(tokens, tokens.get_next_meaningful_token(next_index).unwrap());
    }
    index
}

fn get_second_argument_end(tokens: &Tokens, mut index: usize, end_index: usize) -> Option<usize> {
    if tokens[index].is_kind(T_ELLIPSIS) {
        return None;
    }
    while index <= end_index {
        let mut block_type = Tokens::detect_block_type(&tokens[index]);
        while let Some((t, true)) = block_type {
            index = tokens.find_block_end(t, index);
            index = tokens.get_next_meaningful_token(index).unwrap();
            block_type = Tokens::detect_block_type(&tokens[index]);
        }
        if tokens[index].equals(",", true)
            || tokens[index].is_given_kind(&[T_YIELD, T_YIELD_FROM, T_LOGICAL_AND, T_LOGICAL_OR, T_LOGICAL_XOR])
        {
            return None;
        }
        index += 1;
    }
    Some(end_index)
}
