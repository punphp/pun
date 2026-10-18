//! Port of `PhpCsFixer\Fixer\Alias\ModernizeStrposFixer` (v3 default: `modernize_stripos` false).
use crate::analyzers::arguments_analyzer::ArgumentsAnalyzer;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Proto, Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct ModernizeStrposFixer {
    modernize_stripos: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(ModernizeStrposFixer {
        modernize_stripos: false,
    })
}

/// `REPLACEMENTS`: operator, operand, replacement function, negate.
const REPLACEMENTS: &[(
    (pun_tokens::Kind, &str),
    (pun_tokens::Kind, &str),
    &str,
    bool,
)] = &[
    (
        (T_IS_IDENTICAL, "==="),
        (T_LNUMBER, "0"),
        "str_starts_with",
        false,
    ),
    (
        (T_IS_NOT_IDENTICAL, "!=="),
        (T_LNUMBER, "0"),
        "str_starts_with",
        true,
    ),
    (
        (T_IS_NOT_IDENTICAL, "!=="),
        (T_STRING, "false"),
        "str_contains",
        false,
    ),
    (
        (T_IS_IDENTICAL, "==="),
        (T_STRING, "false"),
        "str_contains",
        true,
    ),
];

/// `getCompareTokens`' result.
struct CompareTokens {
    operator_index: usize,
    operand_index: usize,
}

impl Fixer for ModernizeStrposFixer {
    fn name(&self) -> &'static str {
        "modernize_strpos"
    }
    fn priority(&self) -> i32 {
        37
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
            && tokens.is_any_token_kinds_found(&[T_IS_IDENTICAL, T_IS_NOT_IDENTICAL])
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["modernize_stripos"])?;
        self.modernize_stripos = o.bool("modernize_stripos", false)?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let functions_analyzer = FunctionsAnalyzer::new();
        let arguments_analyzer = ArgumentsAnalyzer;
        let mut modernize_candidates: Vec<Proto> = vec![(T_STRING, "strpos").into()];
        if self.modernize_stripos {
            modernize_candidates.push((T_STRING, "stripos").into());
        }
        let mut index = tokens.count() - 1;
        while index > 0 {
            if !tokens[index].equals_any(&modernize_candidates, false)
                || !functions_analyzer.is_global_function_call(tokens, index)
            {
                index -= 1;
                continue;
            }
            let open_index = tokens.get_next_meaningful_token(index).unwrap();
            let close_index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_index);
            let arguments = arguments_analyzer.get_arguments(tokens, open_index, close_index);
            if arguments.len() != 2 {
                index -= 1;
                continue;
            }
            let compare_tokens =
                get_compare_tokens(tokens, index, -1) // look behind
                    .or_else(|| get_compare_tokens(tokens, close_index, 1)); // look ahead
            if let Some(compare_tokens) = compare_tokens {
                let is_case_insensitive = tokens[index].equals((T_STRING, "stripos"), false);
                fix_call(tokens, index, &compare_tokens, is_case_insensitive);
            }
            index -= 1;
        }
    }
}

fn fix_call(
    tokens: &mut Tokens,
    mut function_index: usize,
    operator_indices: &CompareTokens,
    is_case_insensitive: bool,
) {
    for &(operator, operand, replacement, negate) in REPLACEMENTS {
        if !tokens[operator_indices.operator_index].equals(operator, true) {
            continue;
        }
        if !tokens[operator_indices.operand_index].equals(operand, false) {
            continue;
        }
        tokens.clear_token_and_merge_surrounding_whitespace(operator_indices.operator_index);
        tokens.clear_token_and_merge_surrounding_whitespace(operator_indices.operand_index);
        tokens.clear_token_and_merge_surrounding_whitespace(function_index);
        if negate {
            let mut negate_insert_index = function_index;
            let prev_function_index = tokens.get_prev_meaningful_token(function_index).unwrap();
            if tokens[prev_function_index].is_kind(T_NS_SEPARATOR) {
                negate_insert_index = prev_function_index;
            }
            tokens.insert_at(negate_insert_index, vec![Token::char("!")]);
            function_index += 1;
        }
        tokens.insert_at(function_index, vec![Token::new(T_STRING, replacement)]);
        if is_case_insensitive {
            wrap_arguments_with_str_to_lower(tokens, function_index);
        }
        break;
    }
}

fn wrap_arguments_with_str_to_lower(tokens: &mut Tokens, function_index: usize) {
    let should_add_namespace = tokens[function_index - 1].is_kind(T_NS_SEPARATOR);
    let ns = usize::from(should_add_namespace);
    let open_index = tokens.get_next_meaningful_token(function_index).unwrap();
    let close_index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_index);
    let arguments = ArgumentsAnalyzer.get_arguments(tokens, open_index, close_index);
    let (&first_start, &first_end) = arguments.first_key_value().unwrap();
    let (&last_start, &last_end) = arguments.last_key_value().unwrap();
    let mut first_argument_index_start = first_start;
    let first_argument_index_end = first_end + 3 + ns;
    let is_second_argument_token_white_space = tokens[last_start].is_kind(T_WHITESPACE);
    let mut second_argument_index_start = if is_second_argument_token_white_space {
        tokens.get_next_meaningful_token(last_start).unwrap()
    } else {
        last_start
    };
    second_argument_index_start += 3 + ns;
    let second_argument_index_end =
        last_end + 6 + ns + usize::from(is_second_argument_token_white_space);
    if should_add_namespace {
        tokens.insert_at(
            first_argument_index_start,
            vec![Token::new(T_NS_SEPARATOR, "\\")],
        );
        first_argument_index_start += 1;
    }
    tokens.insert_at(
        first_argument_index_start,
        vec![Token::new(T_STRING, "strtolower"), Token::char("(")],
    );
    tokens.insert_at(first_argument_index_end, vec![Token::char(")")]);
    if should_add_namespace {
        tokens.insert_at(
            second_argument_index_start,
            vec![Token::new(T_NS_SEPARATOR, "\\")],
        );
        second_argument_index_start += 1;
    }
    tokens.insert_at(
        second_argument_index_start,
        vec![Token::new(T_STRING, "strtolower"), Token::char("(")],
    );
    tokens.insert_at(second_argument_index_end, vec![Token::char(")")]);
}

fn get_compare_tokens(
    tokens: &Tokens,
    offset_index: usize,
    direction: isize,
) -> Option<CompareTokens> {
    let mut operator_index = tokens.get_meaningful_token_sibling(offset_index, direction);
    if let Some(i) = operator_index
        && tokens[i].is_kind(T_NS_SEPARATOR)
    {
        operator_index = tokens.get_meaningful_token_sibling(i, direction);
    }
    let operator_index = operator_index?;
    if !tokens[operator_index].is_given_kind(&[T_IS_IDENTICAL, T_IS_NOT_IDENTICAL]) {
        return None;
    }
    let operand_index = tokens
        .get_meaningful_token_sibling(operator_index, direction)
        .unwrap();
    let operand = &tokens[operand_index];
    if !operand.equals((T_LNUMBER, "0"), true) && !operand.equals((T_STRING, "false"), false) {
        return None;
    }
    let precedence_token_index = tokens.get_meaningful_token_sibling(operand_index, direction);
    if precedence_token_index.is_some_and(|i| is_of_higher_precedence(&tokens[i])) {
        return None;
    }
    Some(CompareTokens {
        operator_index,
        operand_index,
    })
}

fn is_of_higher_precedence(token: &Token) -> bool {
    token.is_given_kind(&[
        T_DEC,
        T_INC,
        T_INSTANCEOF,
        T_IS_GREATER_OR_EQUAL,
        T_IS_SMALLER_OR_EQUAL,
        T_POW,
        T_SL,
        T_SR,
    ]) || token.equals_any(
        &[
            "!".into(),
            "%".into(),
            "*".into(),
            "+".into(),
            "-".into(),
            ".".into(),
            "/".into(),
            "<".into(),
            ">".into(),
            "~".into(),
        ],
        true,
    )
}
