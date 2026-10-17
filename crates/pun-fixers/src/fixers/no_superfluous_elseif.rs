//! Port of `PhpCsFixer\Fixer\ControlStructure\NoSuperfluousElseifFixer`.
use crate::analyzers::abstract_no_useless_else_fixer::{PRIORITY, is_superfluous_else};
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct NoSuperfluousElseifFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoSuperfluousElseifFixer)
}

impl Fixer for NoSuperfluousElseifFixer {
    fn name(&self) -> &'static str {
        "no_superfluous_elseif"
    }
    fn priority(&self) -> i32 {
        PRIORITY
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_ELSE, T_ELSEIF])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = 0;
        while index < tokens.len() {
            if is_elseif(tokens, index) && is_superfluous_else(tokens, index) {
                convert_elseif_to_if(tokens, index);
            }
            index += 1;
        }
    }
}

fn is_elseif(tokens: &Tokens, index: usize) -> bool {
    tokens[index].is_kind(T_ELSEIF)
        || (tokens[index].is_kind(T_ELSE)
            && tokens[tokens.get_next_meaningful_token(index).unwrap()].is_kind(T_IF))
}

fn convert_elseif_to_if(tokens: &mut Tokens, index: usize) {
    if tokens[index].is_kind(T_ELSE) {
        tokens.clear_token_and_merge_surrounding_whitespace(index);
    } else {
        tokens.set(index, Token::new(T_IF, "if"));
    }
    let mut whitespace = Vec::new();
    for previous in (1..index).rev() {
        let token = &tokens[previous];
        if token.is_whitespace(None) {
            if let Some(m) = Preg::match_groups("/(\\R\\N*)$/", &token.content, 0) {
                whitespace = m.str(1).to_vec();
                break;
            }
        }
    }
    if whitespace.is_empty() {
        return;
    }
    let previous_token = &tokens[index - 1];
    if !previous_token.is_whitespace(None) {
        tokens.insert_at(index, vec![Token::new(T_WHITESPACE, whitespace)]);
    } else if !Preg::r#match("/\\R/", &previous_token.content) {
        tokens.set(index - 1, Token::new(T_WHITESPACE, whitespace));
    }
}
