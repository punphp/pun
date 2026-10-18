//! Port of `PhpCsFixer\Fixer\ControlStructure\EmptyLoopBodyFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Proto, Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::path::Path;

struct EmptyLoopBodyFixer {
    braces: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(EmptyLoopBodyFixer { braces: false })
}

const TOKEN_LOOP_KINDS: [pun_tokens::Kind; 3] = [T_FOR, T_FOREACH, T_WHILE];

impl Fixer for EmptyLoopBodyFixer {
    fn name(&self) -> &'static str {
        "empty_loop_body"
    }
    fn priority(&self) -> i32 {
        39
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["style"])?;
        self.braces = c.string("style", "semicolon", &["braces", "semicolon"])? == "braces";
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&TOKEN_LOOP_KINDS)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (1..tokens.count()).rev() {
            if tokens[index].is_given_kind(&TOKEN_LOOP_KINDS) {
                let open = tokens
                    .get_next_token_of_kind(index, &[Proto::Char(b"(")], true)
                    .unwrap();
                let end_index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open);
                if self.braces {
                    fix_loop_braces(tokens, index, end_index);
                } else {
                    fix_loop_semicolon(tokens, end_index);
                }
            }
        }
    }
}

fn fix_loop_braces(tokens: &mut Tokens, index: usize, end_index: usize) {
    if tokens[index].is_kind(T_WHILE)
        && TokensAnalyzer::new(tokens).is_while_part_of_do_while(index)
    {
        return;
    }
    let semi_colon_index = tokens.get_next_meaningful_token(end_index).unwrap();
    if !tokens[semi_colon_index].equals(";", true) {
        return;
    }
    tokens.set(semi_colon_index, Token::char("{"));
    tokens.insert_at(semi_colon_index + 1, vec![Token::char("}")]);
}

fn fix_loop_semicolon(tokens: &mut Tokens, end_index: usize) {
    let brace_open_index = tokens.get_next_meaningful_token(end_index).unwrap();
    if !tokens[brace_open_index].equals("{", true) {
        return;
    }
    let brace_close_index = tokens
        .get_next_non_whitespace(brace_open_index, None)
        .unwrap();
    if !tokens[brace_close_index].equals("}", true) {
        return;
    }
    tokens.set(brace_open_index, Token::char(";"));
    tokens.clear_token_and_merge_surrounding_whitespace(brace_close_index);
}
