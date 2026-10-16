//! Port of `PhpCsFixer\Fixer\ClassUsage\DateTimeImmutableFixer`.
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::{php_compat, Kind::*, Token, Tokens};
use std::path::Path;

struct DateTimeImmutableFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(DateTimeImmutableFixer)
}

impl Fixer for DateTimeImmutableFixer {
    fn name(&self) -> &'static str {
        "date_time_immutable"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let functions_analyzer = FunctionsAnalyzer::new();
        let function_map: [(&[u8], &str); 2] =
            [(b"date_create", "date_create_immutable"), (b"date_create_from_format", "date_create_immutable_from_format")];
        let mut is_in_namespace = false;
        let mut is_imported = false; // e.g. use DateTime;

        let mut limit = tokens.count();
        let mut index = 0;
        while index < limit {
            let token = &tokens[index];
            if token.is_kind(T_NAMESPACE) {
                is_in_namespace = true;
                index += 1;
                continue;
            }
            if is_in_namespace && token.is_kind(T_USE) {
                let next_index = tokens.get_next_meaningful_token(index).unwrap();
                if php_compat::strtolower(&tokens[next_index].content) != b"datetime" {
                    index += 1;
                    continue;
                }
                let next_next_index = tokens.get_next_meaningful_token(next_index).unwrap();
                if tokens[next_next_index].equals(";", true) {
                    is_imported = true;
                }
                index = next_next_index + 1;
                continue;
            }
            if !token.is_kind(T_STRING) {
                index += 1;
                continue;
            }
            let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
            if tokens[prev_index].is_kind(T_FUNCTION) {
                index += 1;
                continue;
            }
            let lowercase_content = php_compat::strtolower(&token.content);
            if lowercase_content == b"datetime" {
                fix_class_usage(tokens, index, is_in_namespace, is_imported);
                limit = tokens.count(); // update limit, as fixing class usage may insert new token
                index += 1;
                continue;
            }
            if let Some((_, new)) = function_map.iter().find(|(k, _)| *k == &lowercase_content[..]) {
                if functions_analyzer.is_global_function_call(tokens, index) {
                    tokens.set(index, Token::new(T_STRING, *new));
                }
            }
            index += 1;
        }
    }
}

fn fix_class_usage(tokens: &mut Tokens, index: usize, is_in_namespace: bool, is_imported: bool) {
    let next_index = tokens.get_next_meaningful_token(index).unwrap();
    if tokens[next_index].is_kind(T_DOUBLE_COLON) {
        let next_next_index = tokens.get_next_meaningful_token(next_index).unwrap();
        if tokens[next_next_index].is_kind(T_STRING) {
            let n3 = tokens.get_next_meaningful_token(next_next_index).unwrap();
            if !tokens[n3].equals("(", true) {
                return;
            }
        }
    }
    let mut is_used_alone = false; // e.g. new DateTime();
    let mut is_used_with_leading_backslash = false; // e.g. new \DateTime();
    let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
    if tokens[prev_index].is_kind(T_NS_SEPARATOR) {
        let prev_prev_index = tokens.get_prev_meaningful_token(prev_index).unwrap();
        if !tokens[prev_prev_index].is_kind(T_STRING) {
            is_used_with_leading_backslash = true;
        }
    } else if !tokens[prev_index].is_kind(T_DOUBLE_COLON) && !tokens[prev_index].is_object_operator() {
        is_used_alone = true;
    }
    if is_used_with_leading_backslash || (is_used_alone && ((is_in_namespace && is_imported) || !is_in_namespace)) {
        tokens.set(index, Token::new(T_STRING, "DateTimeImmutable"));
        if is_in_namespace && is_used_alone {
            tokens.insert_at(index, vec![Token::new(T_NS_SEPARATOR, "\\")]);
        }
    }
}
