//! Port of `PhpCsFixer\Fixer\StringNotation\StringLengthToEmptyFixer`.
use crate::analyzers::abstract_function_reference_fixer::{
    FunctionReferenceFinder, IS_RISKY, is_candidate,
};
use crate::analyzers::arguments_analyzer::ArgumentsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::{self, *};
use pun_tokens::{Proto, Token, Tokens};
use std::path::Path;

struct StringLengthToEmptyFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(StringLengthToEmptyFixer)
}

fn get_replacement_yoda(operator: &Token, operand: &Token) -> Option<Kind> {
    if operand.get_content() == b"0" {
        if operator.is_given_kind(&[T_IS_IDENTICAL, T_IS_GREATER_OR_EQUAL]) {
            return Some(T_IS_IDENTICAL);
        }
        if operator.is_kind(T_IS_NOT_IDENTICAL) || operator.equals("<", true) {
            return Some(T_IS_NOT_IDENTICAL);
        }
        return None;
    }
    if operator.is_kind(T_IS_SMALLER_OR_EQUAL) {
        return Some(T_IS_NOT_IDENTICAL);
    }
    if operator.equals(">", true) {
        return Some(T_IS_IDENTICAL);
    }
    None
}

fn get_replacement_not_yoda(operator: &Token, operand: &Token) -> Option<Kind> {
    if operand.get_content() == b"0" {
        if operator.is_given_kind(&[T_IS_IDENTICAL, T_IS_SMALLER_OR_EQUAL]) {
            return Some(T_IS_IDENTICAL);
        }
        if operator.is_kind(T_IS_NOT_IDENTICAL) || operator.equals(">", true) {
            return Some(T_IS_NOT_IDENTICAL);
        }
        return None;
    }
    if operator.is_kind(T_IS_GREATER_OR_EQUAL) {
        return Some(T_IS_NOT_IDENTICAL);
    }
    if operator.equals("<", true) {
        return Some(T_IS_IDENTICAL);
    }
    None
}

fn is_operand_of_interest(token: &Token) -> bool {
    token.is_kind(T_LNUMBER) && matches!(token.get_content(), b"0" | b"1")
}

fn is_operator_of_interest(token: &Token) -> bool {
    token.is_given_kind(&[
        T_IS_IDENTICAL,
        T_IS_NOT_IDENTICAL,
        T_IS_SMALLER_OR_EQUAL,
        T_IS_GREATER_OR_EQUAL,
    ]) || token.equals("<", true)
        || token.equals(">", true)
}

fn is_of_higher_precedence(token: &Token) -> bool {
    const CHARS: &[Proto<'static>] = &[
        Proto::Char(b"!"),
        Proto::Char(b"%"),
        Proto::Char(b"*"),
        Proto::Char(b"+"),
        Proto::Char(b"-"),
        Proto::Char(b"."),
        Proto::Char(b"/"),
        Proto::Char(b"~"),
        Proto::Char(b"?"),
    ];
    token.is_given_kind(&[T_INSTANCEOF, T_POW, T_SL, T_SR]) || token.equals_any(CHARS, true)
}

fn keep_parentheses(
    tokens: &Tokens,
    open_parenthesis_index: usize,
    close_parenthesis_index: usize,
) -> bool {
    let mut i = tokens
        .get_next_meaningful_token(open_parenthesis_index)
        .unwrap();
    if tokens[i].is_cast() {
        i = tokens.get_next_meaningful_token(i).unwrap();
    }
    while i < close_parenthesis_index {
        let token = &tokens[i];
        if token.is_given_kind(&[T_VARIABLE, T_STRING])
            || token.is_object_operator()
            || token.is_whitespace(None)
            || token.is_comment()
        {
            i += 1;
            continue;
        }
        if let Some((block_type, true)) = Tokens::detect_block_type(token) {
            i = tokens.find_block_end(block_type, i) + 1;
            continue;
        }
        return true;
    }
    false
}

fn find_str_length_calls(tokens: &Tokens) -> Vec<(usize, usize, usize)> {
    let finder = FunctionReferenceFinder::new();
    let mut candidates = Vec::new();
    let count = tokens.count();
    let mut i = 0;
    while i < count {
        let Some(candidate) = finder.find("strlen", tokens, i, Some(count)) else {
            break;
        };
        i = candidate.1 + 1; // proceed to openParenthesisIndex
        candidates.push(candidate);
    }
    candidates.reverse();
    candidates
}

impl Fixer for StringLengthToEmptyFixer {
    fn name(&self) -> &'static str {
        "string_length_to_empty"
    }
    fn priority(&self) -> i32 {
        1
    }
    fn is_risky(&self) -> bool {
        IS_RISKY
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let arguments_analyzer = ArgumentsAnalyzer;
        for (function_name_index, open_parenthesis_index, close_parenthesis_index) in
            find_str_length_calls(tokens)
        {
            if arguments_analyzer.count_arguments(
                tokens,
                open_parenthesis_index,
                close_parenthesis_index,
            ) != 1
            {
                continue; // must be one argument
            }
            // test for leading `\` before `strlen` call
            let next_index = tokens
                .get_next_meaningful_token(close_parenthesis_index)
                .unwrap();
            let mut previous_index = tokens
                .get_prev_meaningful_token(function_name_index)
                .unwrap();
            let mut namespace_separator_index = None;
            if tokens[previous_index].is_kind(T_NS_SEPARATOR) {
                namespace_separator_index = Some(previous_index);
                previous_index = tokens.get_prev_meaningful_token(previous_index).unwrap();
            }
            let (operator_index, operand_index, replacement);
            // test for yoda vs non-yoda fix case
            if is_operator_of_interest(&tokens[previous_index]) {
                operator_index = previous_index;
                operand_index = tokens.get_prev_meaningful_token(previous_index).unwrap();
                if !is_operand_of_interest(&tokens[operand_index]) {
                    continue;
                }
                let Some(r) = get_replacement_yoda(&tokens[operator_index], &tokens[operand_index])
                else {
                    continue;
                };
                replacement = r;
                if is_of_higher_precedence(&tokens[next_index]) {
                    continue;
                }
                let left = tokens.get_prev_meaningful_token(operand_index).unwrap();
                if is_of_higher_precedence(&tokens[left]) {
                    continue;
                }
            } else if is_operator_of_interest(&tokens[next_index]) {
                operator_index = next_index;
                operand_index = tokens.get_next_meaningful_token(next_index).unwrap();
                if !is_operand_of_interest(&tokens[operand_index]) {
                    continue;
                }
                let Some(r) =
                    get_replacement_not_yoda(&tokens[operator_index], &tokens[operand_index])
                else {
                    continue;
                };
                replacement = r;
                let right = tokens.get_next_meaningful_token(operand_index).unwrap();
                if is_of_higher_precedence(&tokens[right]) {
                    continue;
                }
                if is_of_higher_precedence(&tokens[previous_index]) {
                    continue;
                }
            } else {
                continue;
            }
            // prepare for fixing
            let keep = keep_parentheses(tokens, open_parenthesis_index, close_parenthesis_index);
            let operand_content = if replacement == T_IS_IDENTICAL {
                "==="
            } else {
                "!=="
            };
            // apply fixing
            tokens.set(operand_index, Token::new(T_CONSTANT_ENCAPSED_STRING, "''"));
            tokens.set(operator_index, Token::new(replacement, operand_content));
            if !keep {
                tokens.clear_token_and_merge_surrounding_whitespace(close_parenthesis_index);
                tokens.clear_token_and_merge_surrounding_whitespace(open_parenthesis_index);
            }
            tokens.clear_token_and_merge_surrounding_whitespace(function_name_index);
            if let Some(ns) = namespace_separator_index {
                tokens.clear_token_and_merge_surrounding_whitespace(ns);
            }
        }
    }
}
