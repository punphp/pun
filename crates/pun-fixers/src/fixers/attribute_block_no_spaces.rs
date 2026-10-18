//! Port of `PhpCsFixer\Fixer\AttributeNotation\AttributeBlockNoSpacesFixer`.
use crate::analyzers::attribute_analyzer::AttributeAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Proto, Tokens};
use std::path::Path;

struct AttributeBlockNoSpacesFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(AttributeBlockNoSpacesFixer)
}

impl Fixer for AttributeBlockNoSpacesFixer {
    fn name(&self) -> &'static str {
        "attribute_block_no_spaces"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_ATTRIBUTE)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = 0;
        while let Some(found) =
            tokens.get_next_token_of_kind(index, &[Proto::Kind(T_ATTRIBUTE)], true)
        {
            let mut to_delete = Vec::new();
            let attribute_analysis = AttributeAnalyzer::collect_one(tokens, found);

            index = attribute_analysis.opening_bracket_index;
            let next_token_index = tokens.get_next_meaningful_token(index).unwrap();
            for i in index + 1..next_token_index {
                if !tokens[i].is_whitespace(None) {
                    to_delete.clear();
                    break;
                }
                to_delete.push(i);
            }

            index = attribute_analysis.closing_bracket_index;
            let prev_token_index = tokens.get_prev_meaningful_token(index).unwrap();
            for i in prev_token_index + 1..index {
                if !tokens[i].is_whitespace(None) {
                    to_delete.clear();
                    break;
                }
                to_delete.push(i);
            }

            for i in to_delete {
                tokens.clear_at(i);
            }
        }
    }
}
