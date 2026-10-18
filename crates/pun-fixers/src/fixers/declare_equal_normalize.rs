//! Port of `PhpCsFixer\Fixer\LanguageConstruct\DeclareEqualNormalizeFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens};
use serde_json::Value;
use std::path::Path;

#[derive(Default)]
struct DeclareEqualNormalizeFixer {
    space_none: bool,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(DeclareEqualNormalizeFixer::default())
}

impl Fixer for DeclareEqualNormalizeFixer {
    fn name(&self) -> &'static str {
        "declare_equal_normalize"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DECLARE)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["space"])?;
        self.space_none = o.string("space", "none", &["single", "none"])? == "none";
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let count = tokens.count();
        let mut index = 0;
        while index + 6 < count {
            if tokens[index].is_kind(T_DECLARE) {
                let open_parenthesis_index = tokens.get_next_meaningful_token(index).unwrap();
                let close_parenthesis_index =
                    tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_parenthesis_index);
                let mut i = close_parenthesis_index;
                while i > open_parenthesis_index {
                    if tokens[i].equals("=", true) {
                        if self.space_none {
                            remove_whitespace_around_token(tokens, i);
                        } else {
                            ensure_whitespace_around_token(tokens, i);
                        }
                    }
                    i -= 1;
                }
            }
            index += 1;
        }
    }
}

fn ensure_whitespace_around_token(tokens: &mut Tokens, index: usize) {
    if tokens[index + 1].is_whitespace(None) {
        if tokens[index + 1].content != b" " {
            tokens.set(index + 1, Token::new(T_WHITESPACE, " "));
        }
    } else {
        tokens.insert_at(index + 1, vec![Token::new(T_WHITESPACE, " ")]);
    }
    if tokens[index - 1].is_whitespace(None) {
        let prev = tokens.get_prev_non_whitespace(index - 1, None).unwrap();
        if tokens[index - 1].content != b" " && !tokens[prev].is_comment() {
            tokens.set(index - 1, Token::new(T_WHITESPACE, " "));
        }
    } else {
        tokens.insert_at(index, vec![Token::new(T_WHITESPACE, " ")]);
    }
}

fn remove_whitespace_around_token(tokens: &mut Tokens, index: usize) {
    let prev = tokens.get_prev_non_whitespace(index, None).unwrap();
    if !tokens[prev].is_comment() {
        tokens.remove_leading_whitespace(index, None);
    }
    tokens.remove_trailing_whitespace(index, None);
}
