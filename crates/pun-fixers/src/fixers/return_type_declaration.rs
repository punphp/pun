//! Port of `PhpCsFixer\Fixer\FunctionNotation\ReturnTypeDeclarationFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct ReturnTypeDeclarationFixer {
    one_space_before: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(ReturnTypeDeclarationFixer {
        one_space_before: false,
    })
}

impl Fixer for ReturnTypeDeclarationFixer {
    fn name(&self) -> &'static str {
        "return_type_declaration"
    }
    fn priority(&self) -> i32 {
        -17
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["space_before"])?;
        self.one_space_before = c.string("space_before", "none", &["one", "none"])? == "one";
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(CT_TYPE_COLON)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = 0;
        let mut limit = tokens.count();
        while index < limit {
            if !tokens[index].is_kind(CT_TYPE_COLON) {
                index += 1;
                continue;
            }
            let previous_index = index - 1;
            if tokens[previous_index].is_whitespace(None) {
                let p = tokens.get_prev_non_whitespace(index - 1, None).unwrap();
                if !tokens[p].is_comment() {
                    if self.one_space_before {
                        tokens.set(previous_index, Token::new(T_WHITESPACE, " "));
                    } else {
                        tokens.clear_at(previous_index);
                    }
                }
            } else if self.one_space_before {
                if tokens.ensure_whitespace_at_index(index, 0, b" ") {
                    limit += 1;
                }
                index += 1;
            }
            index += 1;
            if tokens.ensure_whitespace_at_index(index, 0, b" ") {
                limit += 1;
            }
            index += 1;
        }
    }
}
