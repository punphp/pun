//! Port of `PhpCsFixer\Fixer\Alias\SetTypeToCastFixer`.
use crate::analyzers::abstract_function_reference_fixer::{FunctionReferenceFinder, IS_RISKY};
use crate::analyzers::arguments_analyzer::ArgumentsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::{self, *};
use pun_tokens::php_compat::{ltrim_chars, rtrim_chars, strtolower};
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct SetTypeToCastFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(SetTypeToCastFixer)
}

/// `$map`; `'null'` is dealt with separately.
fn map(type_: &[u8]) -> Option<(Kind, &'static str)> {
    Some(match type_ {
        b"array" => (T_ARRAY_CAST, "(array)"),
        b"bool" | b"boolean" => (T_BOOL_CAST, "(bool)"),
        b"double" | b"float" => (T_DOUBLE_CAST, "(float)"),
        b"int" | b"integer" => (T_INT_CAST, "(int)"),
        b"object" => (T_OBJECT_CAST, "(object)"),
        b"string" => (T_STRING_CAST, "(string)"),
        _ => return None,
    })
}

fn find_settype_calls(tokens: &Tokens) -> Vec<(usize, usize, usize)> {
    let finder = FunctionReferenceFinder::new();
    let mut candidates = Vec::new();
    let end = tokens.count();
    let mut i = 1;
    while i < end {
        let Some(candidate) = finder.find("settype", tokens, i, Some(end)) else {
            break;
        };
        i = candidate.1 + 1; // proceed to openParenthesisIndex
        candidates.push(candidate);
    }
    candidates
}

fn remove_settype_call(
    tokens: &mut Tokens,
    function_name_index: usize,
    open_parenthesis_index: usize,
    first_argument_start: usize,
    comma_index: usize,
    second_argument_start: usize,
    close_parenthesis_index: usize,
) {
    tokens.clear_token_and_merge_surrounding_whitespace(close_parenthesis_index);
    let prev_index = tokens
        .get_prev_meaningful_token(close_parenthesis_index)
        .unwrap();
    if tokens[prev_index].equals(",", true) {
        tokens.clear_token_and_merge_surrounding_whitespace(prev_index);
    }
    tokens.clear_token_and_merge_surrounding_whitespace(second_argument_start);
    tokens.clear_token_and_merge_surrounding_whitespace(comma_index);
    tokens.clear_token_and_merge_surrounding_whitespace(first_argument_start);
    tokens.clear_token_and_merge_surrounding_whitespace(open_parenthesis_index);
    tokens.clear_at(function_name_index); // we'll be inserting here so no need to merge the space tokens
    tokens.clear_empty_tokens();
}

impl Fixer for SetTypeToCastFixer {
    fn name(&self) -> &'static str {
        "set_type_to_cast"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        IS_RISKY
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[T_CONSTANT_ENCAPSED_STRING, T_STRING, T_VARIABLE])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let arguments_analyzer = ArgumentsAnalyzer;
        for (function_name_index, open, close) in find_settype_calls(tokens).into_iter().rev() {
            let arguments: Vec<(usize, usize)> = arguments_analyzer
                .get_arguments(tokens, open, close)
                .into_iter()
                .collect();
            if arguments.len() != 2 {
                continue; // function must be overridden or used incorrectly
            }
            let prev = tokens
                .get_prev_meaningful_token(function_name_index)
                .unwrap();
            if !tokens[prev].equals_any(
                &[";".into(), "{".into(), "}".into(), T_OPEN_TAG.into()],
                true,
            ) {
                continue; // return value of the function is used
            }
            // --- Test first argument --------------------
            let mut first_argument_start = arguments[0].0;
            if tokens[first_argument_start].is_comment()
                || tokens[first_argument_start].is_whitespace(None)
            {
                first_argument_start = tokens
                    .get_next_meaningful_token(first_argument_start)
                    .unwrap();
            }
            if !tokens[first_argument_start].is_kind(T_VARIABLE) {
                continue; // settype only works with variables pass by reference, function must be overridden
            }
            let comma_index = tokens
                .get_next_meaningful_token(first_argument_start)
                .unwrap();
            if !tokens[comma_index].equals(",", true) {
                continue; // first argument is complex statement; function must be overridden
            }
            // --- Test second argument -------------------
            let (mut second_argument_start, second_argument_end) = arguments[1];
            if tokens[second_argument_start].is_comment()
                || tokens[second_argument_start].is_whitespace(None)
            {
                second_argument_start = tokens
                    .get_next_meaningful_token(second_argument_start)
                    .unwrap();
            }
            if !tokens[second_argument_start].is_kind(T_CONSTANT_ENCAPSED_STRING)
                || tokens
                    .get_next_meaningful_token(second_argument_start)
                    .unwrap()
                    < second_argument_end
            {
                continue; // second argument is of the wrong type or is a (complex) statement of some sort (function is overridden)
            }
            // --- Test type ------------------------------
            let type_ = strtolower(ltrim_chars(
                rtrim_chars(tokens[second_argument_start].get_content(), b"\"'"),
                b"\"'",
            ));
            let cast = map(&type_);
            if type_ != b"null" && cast.is_none() {
                continue; // we don't know how to map
            }
            // --- Fixing ---------------------------------
            let argument_token = tokens[first_argument_start].clone();
            remove_settype_call(
                tokens,
                function_name_index,
                open,
                first_argument_start,
                comma_index,
                second_argument_start,
                close,
            );
            let ws = || Token::new(T_WHITESPACE, " ");
            match cast {
                None => {
                    tokens.insert_at(
                        function_name_index,
                        vec![
                            argument_token,
                            ws(),
                            Token::char("="),
                            ws(),
                            Token::new(T_STRING, "null"),
                        ],
                    );
                    tokens.remove_trailing_whitespace(function_name_index + 4, None);
                }
                Some((kind, content)) => {
                    tokens.insert_at(
                        function_name_index,
                        vec![
                            argument_token.clone(),
                            ws(),
                            Token::char("="),
                            ws(),
                            Token::new(kind, content),
                            ws(),
                            argument_token,
                        ],
                    );
                    tokens.remove_trailing_whitespace(function_name_index + 6, None);
                }
            }
        }
    }
}
