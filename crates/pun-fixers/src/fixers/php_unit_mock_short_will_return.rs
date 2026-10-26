//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitMockShortWillReturnFixer`.
use crate::analyzers::abstract_php_unit_fixer::{apply_fix, is_candidate};
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct PhpUnitMockShortWillReturnFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpUnitMockShortWillReturnFixer)
}

const RETURN_METHODS_MAP: &[(&[u8], &str)] = &[
    (b"returnargument", "willReturnArgument"),
    (b"returncallback", "willReturnCallback"),
    (b"returnself", "willReturnSelf"),
    (b"returnvalue", "willReturn"),
    (b"returnvaluemap", "willReturnMap"),
];

impl Fixer for PhpUnitMockShortWillReturnFixer {
    fn name(&self) -> &'static str {
        "php_unit_mock_short_will_return"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        apply_fix(tokens, apply_php_unit_class_fix);
    }
}

fn apply_php_unit_class_fix(tokens: &mut Tokens, start_index: usize, end_index: usize) {
    let functions = FunctionsAnalyzer::new();
    for index in start_index..end_index {
        if !tokens[index].is_object_operator() {
            continue;
        }
        let replace = tokens.get_next_meaningful_token(index).unwrap();
        if !tokens[replace].equals((T_STRING, "will"), false) {
            continue;
        }
        let replace_open = tokens.get_next_meaningful_token(replace).unwrap();
        if !tokens[replace_open].equals("(", true) {
            continue;
        }
        let class_reference = tokens.get_next_meaningful_token(replace_open).unwrap();
        let object_operator = tokens.get_next_meaningful_token(class_reference).unwrap();
        let remove = tokens.get_next_meaningful_token(object_operator).unwrap();
        if !functions.is_the_same_class_call(tokens, remove) {
            continue;
        }
        let lowered = strtolower(tokens[remove].get_content());
        let Some((_, method)) = RETURN_METHODS_MAP
            .iter()
            .find(|(n, _)| *n == lowered.as_slice())
        else {
            continue;
        };
        let open = tokens.get_next_meaningful_token(remove).unwrap();
        if tokens[tokens.get_next_meaningful_token(open).unwrap()].is_kind(CT_FIRST_CLASS_CALLABLE)
        {
            continue;
        }
        let close = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open);
        tokens.set(replace, Token::new(T_STRING, method.as_bytes().to_vec()));
        for i in [class_reference, object_operator, remove, open, close] {
            tokens.clear_token_and_merge_surrounding_whitespace(i);
        }
        let comma = tokens.get_next_meaningful_token(close).unwrap();
        if tokens[comma].equals(",", true) {
            tokens.clear_token_and_merge_surrounding_whitespace(comma);
        }
    }
}
