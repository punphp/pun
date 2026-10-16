//! Port of `PhpCsFixer\Tokenizer\Analyzer\ArgumentsAnalyzer`.
use super::analysis::{ArgumentAnalysis, TypeAnalysis};
use pun_tokens::{Kind, Kind::*, Tokens};
use std::collections::BTreeMap;

const ARGUMENT_INFO_SKIP_TYPES: &[Kind] = &[
    T_ELLIPSIS, T_FINAL, CT_CONSTRUCTOR_PROPERTY_PROMOTION_PUBLIC, CT_CONSTRUCTOR_PROPERTY_PROMOTION_PROTECTED,
    CT_CONSTRUCTOR_PROPERTY_PROMOTION_PRIVATE, T_READONLY, T_PRIVATE_SET, T_PROTECTED_SET, T_PUBLIC_SET,
];

pub struct ArgumentsAnalyzer;

impl ArgumentsAnalyzer {
    pub fn count_arguments(&self, tokens: &Tokens, open_parenthesis: usize, close_parenthesis: usize) -> usize {
        self.get_arguments(tokens, open_parenthesis, close_parenthesis).len()
    }

    /// start index => end index of each argument.
    pub fn get_arguments(&self, tokens: &Tokens, open_parenthesis: usize, close_parenthesis: usize) -> BTreeMap<usize, usize> {
        let mut arguments = BTreeMap::new();
        let first_sensible_token = tokens.get_next_meaningful_token(open_parenthesis).unwrap();
        if tokens[first_sensible_token].equals(")", true) {
            return arguments;
        }
        let mut param_content_index = open_parenthesis + 1;
        let mut arguments_start = param_content_index;
        while param_content_index < close_parenthesis {
            let token = &tokens[param_content_index];
            // skip nested (), [], {} constructs
            if let Some((block_type, true)) = Tokens::detect_block_type(token) {
                param_content_index = tokens.find_block_end(block_type, param_content_index) + 1;
                continue;
            }
            // if comma matched, increase arguments counter
            if token.equals(",", true) {
                if tokens.get_next_meaningful_token(param_content_index) == Some(close_parenthesis) {
                    break; // trailing ',' in function call (PHP 7.3)
                }
                arguments.insert(arguments_start, param_content_index - 1);
                arguments_start = param_content_index + 1;
            }
            param_content_index += 1;
        }
        arguments.insert(arguments_start, param_content_index - 1);
        arguments
    }

    pub fn get_argument_info(&self, tokens: &Tokens, argument_start: usize, argument_end: usize) -> ArgumentAnalysis {
        let mut default: Option<Vec<u8>> = None;
        let mut name: Option<Vec<u8>> = None;
        let mut name_index = None;
        let mut type_: Option<Vec<u8>> = None;
        let mut type_index_start: Option<usize> = None;
        let mut type_index_end = None;
        let mut saw_name = false;

        let mut index = argument_start;
        while index <= argument_end {
            let token = &tokens[index];
            if token.is_kind(T_ATTRIBUTE) {
                index = tokens.find_block_end(Tokens::BLOCK_TYPE_ATTRIBUTE, index) + 1;
                continue;
            }
            if token.is_kind(CT_PROPERTY_HOOK_BRACE_OPEN) {
                index = tokens.find_block_end(Tokens::BLOCK_TYPE_PROPERTY_HOOK, index) + 1;
                continue;
            }
            if token.is_comment() || token.is_whitespace(None) || token.is_given_kind(ARGUMENT_INFO_SKIP_TYPES) || token.equals("&", true) {
                index += 1;
                continue;
            }
            if token.is_kind(T_VARIABLE) {
                if !saw_name {
                    saw_name = true;
                    name_index = Some(index);
                    name = Some(token.content.clone());
                }
                index += 1;
                continue;
            }
            if token.equals("=", true) {
                index += 1;
                continue;
            }
            if saw_name {
                default.get_or_insert_with(Vec::new).extend_from_slice(&token.content);
            } else {
                type_index_start = Some(type_index_start.filter(|&s| s > 0).unwrap_or(index));
                type_index_end = Some(index);
                type_.get_or_insert_with(Vec::new).extend_from_slice(&token.content);
            }
            index += 1;
        }
        if name.is_none() {
            type_ = None;
        }
        ArgumentAnalysis {
            name,
            name_index,
            default,
            type_analysis: type_.map(|t| TypeAnalysis::new(&t, type_index_start, type_index_end)),
        }
    }
}
