//! Port of `PhpCsFixer\Fixer\ListNotation\ListSyntaxFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::Fixer;
use pun_tokens::Kind::{self, *};
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct ListSyntaxFixer {
    candidate_token_kind: Kind,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(ListSyntaxFixer { candidate_token_kind: T_LIST })
}

impl Fixer for ListSyntaxFixer {
    fn name(&self) -> &'static str {
        "list_syntax"
    }
    fn priority(&self) -> i32 {
        2
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["syntax"])?;
        self.candidate_token_kind =
            if c.string("syntax", "short", &["long", "short"])? == "long" { CT_DESTRUCTURING_BRACKET_OPEN } else { T_LIST };
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(self.candidate_token_kind)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (0..tokens.count()).rev() {
            if tokens[index].is_kind(self.candidate_token_kind) {
                if self.candidate_token_kind == T_LIST {
                    fix_to_short_syntax(tokens, index);
                } else {
                    fix_to_long_syntax(tokens, index);
                }
            }
        }
    }
}

fn fix_to_long_syntax(tokens: &mut Tokens, index: usize) {
    let close_index = tokens.get_next_token_of_kind(index, &[CT_DESTRUCTURING_BRACKET_CLOSE.into(), "[".into()], true).unwrap();
    if !tokens[close_index].is_kind(CT_DESTRUCTURING_BRACKET_CLOSE) {
        return;
    }
    tokens.set(index, Token::char("("));
    tokens.set(close_index, Token::char(")"));
    tokens.insert_at(index, vec![Token::new(T_LIST, "list")]);
}

fn fix_to_short_syntax(tokens: &mut Tokens, index: usize) {
    let open_index = tokens.get_next_token_of_kind(index, &["(".into()], true).unwrap();
    let close_index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_index);
    tokens.set(open_index, Token::new(CT_DESTRUCTURING_BRACKET_OPEN, "["));
    tokens.set(close_index, Token::new(CT_DESTRUCTURING_BRACKET_CLOSE, "]"));
    tokens.clear_token_and_merge_surrounding_whitespace(index);
}
