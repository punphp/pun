//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitSetUpTearDownVisibilityFixer`.
use crate::analyzers::abstract_php_unit_fixer as base;
use pun_format::Fixer;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Kind, Token, Tokens, TokensAnalyzer};
use std::collections::BTreeMap;
use std::path::Path;

struct PhpUnitSetUpTearDownVisibilityFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpUnitSetUpTearDownVisibilityFixer)
}

impl Fixer for PhpUnitSetUpTearDownVisibilityFixer {
    fn name(&self) -> &'static str {
        "php_unit_set_up_tear_down_visibility"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        base::is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        base::apply_fix(tokens, apply_php_unit_class_fix);
    }
}

fn apply_php_unit_class_fix(tokens: &mut Tokens, start_index: usize, end_index: usize) {
    let mut counter = 0;
    let mut slices_to_insert = BTreeMap::new();
    let mut index = start_index + 1;
    while index < end_index {
        if counter == 2 {
            break;
        }
        if tokens[index].equals("{", true) {
            index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, index) + 1;
            continue;
        }
        if !tokens[index].is_kind(Kind::T_FUNCTION) {
            index += 1;
            continue;
        }
        let function_name_index = tokens.get_next_meaningful_token(index).unwrap();
        let function_name = strtolower(&tokens[function_name_index].content);
        if function_name != b"setup" && function_name != b"teardown" {
            index += 1;
            continue;
        }
        counter += 1;
        let visibility = TokensAnalyzer::new(tokens)
            .get_method_attributes(index)
            .visibility;
        if visibility == Some(Kind::T_PUBLIC) {
            let visibility_index = tokens
                .get_prev_token_of_kind(index, &[Kind::T_PUBLIC.into()], true)
                .unwrap();
            tokens.set(
                visibility_index,
                Token::new(Kind::T_PROTECTED, b"protected".to_vec()),
            );
        } else if visibility.is_none() {
            slices_to_insert.insert(
                index,
                vec![
                    Token::new(Kind::T_PROTECTED, b"protected".to_vec()),
                    Token::new(Kind::T_WHITESPACE, b" ".to_vec()),
                ],
            );
        }
        index += 1;
    }
    tokens.insert_slices(slices_to_insert);
}
