//! Port of `PhpCsFixer\Fixer\Alias\PowToExponentiationFixer`.
use crate::analyzers::abstract_function_reference_fixer::{FunctionReferenceFinder, IS_RISKY};
use crate::analyzers::arguments_analyzer::ArgumentsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::{self, *};
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct PowToExponentiationFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(PowToExponentiationFixer)
}

/// `getAllowedKinds`.
const ALLOWED_KINDS: &[Kind] = &[
    T_DNUMBER,
    T_LNUMBER,
    T_VARIABLE,
    T_STRING,
    T_CONSTANT_ENCAPSED_STRING,
    T_DOUBLE_CAST,
    T_INT_CAST,
    T_INC,
    T_DEC,
    T_NS_SEPARATOR,
    T_WHITESPACE,
    T_DOUBLE_COLON,
    T_LINE,
    T_COMMENT,
    T_DOC_COMMENT,
    CT_NAMESPACE_OPERATOR,
    T_OBJECT_OPERATOR,
    T_NULLSAFE_OBJECT_OPERATOR,
];

impl Fixer for PowToExponentiationFixer {
    fn name(&self) -> &'static str {
        "pow_to_exponentiation"
    }
    fn priority(&self) -> i32 {
        32
    }
    fn is_risky(&self) -> bool {
        IS_RISKY
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.count() > 7 && tokens.is_token_kind_found(T_STRING)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let candidates = find_pow_calls(tokens);
        let arguments_analyzer = ArgumentsAnalyzer;
        let mut number_of_tokens_added = 0;
        let mut previous_close_parenthesis_index = tokens.count();
        for mut candidate in candidates.into_iter().rev() {
            if previous_close_parenthesis_index < candidate.2 {
                previous_close_parenthesis_index = candidate.2;
                candidate.2 += number_of_tokens_added;
            } else {
                previous_close_parenthesis_index = candidate.2;
                number_of_tokens_added = 0;
            }
            let arguments = arguments_analyzer.get_arguments(tokens, candidate.1, candidate.2);
            if arguments.len() != 2 {
                continue;
            }
            if (candidate.1..candidate.2).any(|i| tokens[i].is_kind(T_ELLIPSIS)) {
                continue;
            }
            number_of_tokens_added += fix_pow_to_exponentiation(
                tokens,
                candidate.0,
                candidate.1,
                candidate.2,
                &arguments.into_iter().collect::<Vec<_>>(),
            );
        }
    }
}

fn find_pow_calls(tokens: &Tokens) -> Vec<(usize, usize, usize)> {
    let finder = FunctionReferenceFinder::new();
    let mut candidates = vec![];
    let end = tokens.count() - 6;
    let mut i = 1;
    while i < end {
        let Some(candidate) = finder.find("pow", tokens, i, Some(end)) else {
            break;
        };
        i = candidate.1 + 1; // proceed to openParenthesisIndex
        candidates.push(candidate);
    }
    candidates
}

fn fix_pow_to_exponentiation(
    tokens: &mut Tokens,
    function_name_index: usize,
    open_parenthesis_index: usize,
    close_parenthesis_index: usize,
    arguments: &[(usize, usize)],
) -> usize {
    // `reset($arguments)`: the end of the first argument.
    let comma = tokens
        .get_next_token_of_kind(arguments[0].1, &[",".into()], true)
        .unwrap();
    tokens.set(comma, Token::new(T_POW, "**"));
    tokens.clear_at(close_parenthesis_index);
    let previous_index = tokens
        .get_prev_meaningful_token(close_parenthesis_index)
        .unwrap();
    if tokens[previous_index].equals(",", true) {
        tokens.clear_at(previous_index); // trailing ',' in function call (PHP 7.3)
    }
    let mut added = 0;
    for &(argument_start_index, argument_end_index) in arguments.iter().rev() {
        if is_parenthesis_needed(tokens, argument_start_index, argument_end_index) {
            tokens.insert_at(argument_end_index + 1, vec![Token::char(")")]);
            tokens.insert_at(argument_start_index, vec![Token::char("(")]);
            added += 2;
        }
    }
    tokens.clear_at(open_parenthesis_index);
    tokens.clear_token_and_merge_surrounding_whitespace(function_name_index);
    let prev_meaningful_token_index = tokens
        .get_prev_meaningful_token(function_name_index)
        .unwrap();
    if tokens[prev_meaningful_token_index].is_kind(T_NS_SEPARATOR) {
        tokens.clear_at(prev_meaningful_token_index);
    }
    added
}

fn is_parenthesis_needed(
    tokens: &Tokens,
    argument_start_index: usize,
    argument_end_index: usize,
) -> bool {
    let mut i = argument_start_index;
    while i <= argument_end_index {
        if tokens[i].is_given_kind(ALLOWED_KINDS) || tokens.is_empty_at(i) {
            i += 1;
            continue;
        }
        if let Some((block_type, _)) = Tokens::detect_block_type(&tokens[i]) {
            i = tokens.find_block_end(block_type, i) + 1;
            continue;
        }
        if tokens[i].equals("$", true) {
            i = tokens.get_next_meaningful_token(i).unwrap();
            if tokens[i].is_kind(CT_DYNAMIC_VAR_BRACE_OPEN) {
                i = tokens.find_block_end(Tokens::BLOCK_TYPE_DYNAMIC_VAR_BRACE, i) + 1;
                continue;
            }
        }
        if tokens[i].equals("+", true)
            && tokens.get_prev_meaningful_token(i).unwrap() < argument_start_index
        {
            i += 1;
            continue;
        }
        return true;
    }
    false
}
