//! Port of `PhpCsFixer\Fixer\ControlStructure\NoUselessElseFixer`.
use crate::analyzers::abstract_no_useless_else_fixer::is_superfluous_else;
use pun_format::Fixer;
use pun_tokens::{Kind, Tokens, TokensAnalyzer};
use std::path::Path;

struct NoUselessElseFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoUselessElseFixer)
}

impl Fixer for NoUselessElseFixer {
    fn name(&self) -> &'static str {
        "no_useless_else"
    }
    fn priority(&self) -> i32 {
        39
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(Kind::T_ELSE)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].is_kind(Kind::T_ELSE) {
                continue;
            }
            if tokens[tokens.get_next_meaningful_token(index).unwrap()].equals_any(&[":".into(), Kind::T_IF.into()], true) {
                continue;
            }
            fix_empty_else(tokens, index);
            if tokens.is_empty_at(index) {
                continue;
            }
            if is_superfluous_else(tokens, index) && !contains_named_symbol_declaration(tokens, index) {
                clear_else(tokens, index);
            }
        }
    }
}

fn fix_empty_else(tokens: &mut Tokens, index: usize) {
    let next = tokens.get_next_meaningful_token(index).unwrap();
    if tokens[next].equals("{", true) {
        let close = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, next);
        if close - next == 1 || tokens.get_next_meaningful_token(next) == Some(close) {
            clear_else(tokens, index);
        }
        return;
    }
    let end = tokens.get_next_token_of_kind(index, &[";".into(), Kind::T_CLOSE_TAG.into()], true);
    if Some(next) == end {
        clear_else(tokens, index);
    }
}

fn clear_else(tokens: &mut Tokens, index: usize) {
    tokens.clear_token_and_merge_surrounding_whitespace(index);
    let next = tokens.get_next_meaningful_token(index).unwrap();
    if !tokens[next].equals("{", true) {
        return;
    }
    let close = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, next);
    tokens.clear_token_and_merge_surrounding_whitespace(close);
    tokens.clear_token_and_merge_surrounding_whitespace(next);
}

fn contains_named_symbol_declaration(tokens: &Tokens, index: usize) -> bool {
    let next = tokens.get_next_meaningful_token(index).unwrap();
    if !tokens[next].equals("{", true) {
        return false;
    }
    let a = TokensAnalyzer::new(tokens);
    let close = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, next);
    (next + 1..close).any(|i| (tokens[i].is_kind(Kind::T_FUNCTION) && !a.is_lambda(i)) || (tokens[i].is_classy() && !a.is_anonymous_class(i)))
}
