//! Port of `PhpCsFixer\Fixer\AttributeNotation\AttributeEmptyParenthesesFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{BlockType, Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct AttributeEmptyParenthesesFixer {
    use_parentheses: bool,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(AttributeEmptyParenthesesFixer {
        use_parentheses: false,
    })
}

impl Fixer for AttributeEmptyParenthesesFixer {
    fn name(&self) -> &'static str {
        "attribute_empty_parentheses"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_ATTRIBUTE)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["use_parentheses"])?;
        self.use_parentheses = o.bool("use_parentheses", false)?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let stops = ["(".into(), ",".into(), CT_ATTRIBUTE_CLOSE.into()];
        let mut index = 0;
        while let Some(found) = tokens.get_next_token_of_kind(index, &[T_ATTRIBUTE.into()], true) {
            index = found;
            let mut next_index = index;
            loop {
                let parentheses_index = tokens
                    .get_next_token_of_kind(next_index, &stops, true)
                    .unwrap();
                if self.use_parentheses {
                    Self::ensure_parentheses_at(tokens, parentheses_index);
                } else {
                    Self::ensure_no_parentheses_at(tokens, parentheses_index);
                }
                next_index = tokens
                    .get_next_token_of_kind(next_index, &stops, true)
                    .unwrap();
                // Find closing parentheses, we need to do this in case there's a comma inside the parentheses
                if tokens[next_index].equals("(", true) {
                    next_index = tokens.find_block_end(BlockType::Parenthesis, next_index);
                    next_index = tokens
                        .get_next_token_of_kind(
                            next_index,
                            &[",".into(), CT_ATTRIBUTE_CLOSE.into()],
                            true,
                        )
                        .unwrap();
                }
                // In case there's a comma right before T_ATTRIBUTE_CLOSE
                if !tokens[next_index].is_kind(CT_ATTRIBUTE_CLOSE) {
                    next_index = tokens.get_next_meaningful_token(next_index).unwrap();
                }
                if tokens[next_index].is_kind(CT_ATTRIBUTE_CLOSE) {
                    break;
                }
            }
        }
    }
}

impl AttributeEmptyParenthesesFixer {
    fn ensure_parentheses_at(tokens: &mut Tokens, index: usize) {
        if tokens[index].equals("(", true) {
            return;
        }
        let at = tokens.get_prev_meaningful_token(index).unwrap() + 1;
        tokens.insert_at(at, vec![Token::char("("), Token::char(")")]);
    }

    fn ensure_no_parentheses_at(tokens: &mut Tokens, index: usize) {
        if !tokens[index].equals("(", true) {
            return;
        }
        let closing_index = tokens.get_next_meaningful_token(index).unwrap();
        // attribute has arguments - parentheses can not be removed
        if !tokens[closing_index].equals(")", true) {
            return;
        }
        tokens.clear_token_and_merge_surrounding_whitespace(closing_index);
        tokens.clear_token_and_merge_surrounding_whitespace(index);
    }
}
