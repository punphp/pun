//! Port of `PhpCsFixer\Fixer\LanguageConstruct\GetClassToClassKeywordFixer`. Upstream's candidate check also needs
//! PHP >= 8.0 at run time, which Pun's PHP version always is.
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Token, Tokens};
use std::collections::BTreeMap;
use std::path::Path;

struct GetClassToClassKeywordFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(GetClassToClassKeywordFixer)
}

impl Fixer for GetClassToClassKeywordFixer {
    fn name(&self) -> &'static str {
        "get_class_to_class_keyword"
    }
    fn priority(&self) -> i32 {
        1
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[T_STRING, T_VARIABLE])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let functions_analyzer = FunctionsAnalyzer::new();
        // Upstream's `$indicesToClear`, filled from the end: its foreach runs in that order.
        let mut indices_to_clear = vec![];
        'outer: for index in (1..tokens.count()).rev() {
            if !tokens[index].equals((T_STRING, "get_class"), false) {
                continue;
            }
            if !functions_analyzer.is_global_function_call(tokens, index) {
                continue;
            }
            let brace_open_index = tokens.get_next_meaningful_token(index).unwrap();
            let brace_close_index =
                tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, brace_open_index);
            if Some(brace_close_index) == tokens.get_next_meaningful_token(brace_open_index) {
                continue; // get_class with no arguments
            }
            let mut meaningful_tokens_count = 0;
            let mut variable_tokens_indices = vec![];
            for i in brace_open_index + 1..brace_close_index {
                if !tokens[i].equals_any(
                    &[
                        T_WHITESPACE.into(),
                        T_COMMENT.into(),
                        T_DOC_COMMENT.into(),
                        "(".into(),
                        ")".into(),
                    ],
                    true,
                ) {
                    meaningful_tokens_count += 1;
                }
                if !tokens[i].is_kind(T_VARIABLE) {
                    continue;
                }
                if strtolower(tokens[i].get_content()) == b"$this" {
                    continue 'outer; // get_class($this)
                }
                variable_tokens_indices.push(i);
            }
            if meaningful_tokens_count > 1 || variable_tokens_indices.len() != 1 {
                continue; // argument contains more logic, or more arguments, or no variable argument
            }
            indices_to_clear.push((
                index,
                brace_open_index,
                variable_tokens_indices[0],
                brace_close_index,
            ));
        }
        let mut token_slices = BTreeMap::new();
        for (index, brace_open_index, variable_index, brace_close_index) in indices_to_clear {
            token_slices.insert(
                index,
                vec![
                    Token::new(T_VARIABLE, tokens[variable_index].get_content()),
                    Token::new(T_DOUBLE_COLON, "::"),
                    Token::new(CT_CLASS_CONSTANT, "class"),
                ],
            );
            clear_get_class_call(tokens, index, brace_open_index, brace_close_index);
        }
        tokens.insert_slices(token_slices);
    }
}

fn clear_get_class_call(
    tokens: &mut Tokens,
    index: usize,
    brace_open_index: usize,
    brace_close_index: usize,
) {
    for i in brace_open_index..=brace_close_index {
        if tokens[i].is_given_kind(&[T_WHITESPACE, T_COMMENT, T_DOC_COMMENT]) {
            continue;
        }
        tokens.clear_token_and_merge_surrounding_whitespace(i);
    }
    let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
    if tokens[prev_index].is_kind(T_NS_SEPARATOR) {
        tokens.clear_at(prev_index);
    }
    tokens.clear_at(index);
}
