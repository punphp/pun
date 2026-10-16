//! Port of `PhpCsFixer\Fixer\ArrayNotation\ArraySyntaxFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::{Kind, Kind::*, Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct ArraySyntaxFixer {
    short: bool,
    candidate_token_kind: Kind,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(ArraySyntaxFixer { short: true, candidate_token_kind: T_ARRAY })
}

impl Fixer for ArraySyntaxFixer {
    fn name(&self) -> &'static str {
        "array_syntax"
    }
    fn priority(&self) -> i32 {
        37
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(self.candidate_token_kind)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["syntax"])?;
        self.short = o.string("syntax", "short", &["long", "short"])? == "short";
        self.candidate_token_kind = if self.short { T_ARRAY } else { CT_ARRAY_BRACKET_OPEN };
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (0..tokens.count()).rev() {
            if tokens[index].is_kind(self.candidate_token_kind) {
                if self.short {
                    fix_to_short_array_syntax(tokens, index);
                } else {
                    fix_to_long_array_syntax(tokens, index);
                }
            }
        }
    }
}

fn fix_to_long_array_syntax(tokens: &mut Tokens, index: usize) {
    let close_index = tokens.find_block_end(Tokens::BLOCK_TYPE_ARRAY_BRACKET, index);
    tokens.set(index, Token::char("("));
    tokens.set(close_index, Token::char(")"));
    tokens.insert_at(index, vec![Token::new(T_ARRAY, "array")]);
}

fn fix_to_short_array_syntax(tokens: &mut Tokens, index: usize) {
    let open_index = tokens.get_next_token_of_kind(index, &["(".into()], true).unwrap();
    let close_index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_index);
    tokens.set(open_index, Token::new(CT_ARRAY_BRACKET_OPEN, "["));
    tokens.set(close_index, Token::new(CT_ARRAY_BRACKET_CLOSE, "]"));
    tokens.clear_token_and_merge_surrounding_whitespace(index);
}
