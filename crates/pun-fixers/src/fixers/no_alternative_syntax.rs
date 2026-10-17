//! Port of `PhpCsFixer\Fixer\ControlStructure\NoAlternativeSyntaxFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct NoAlternativeSyntaxFixer {
    fix_non_monolithic_code: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoAlternativeSyntaxFixer {
        fix_non_monolithic_code: true,
    })
}

impl Fixer for NoAlternativeSyntaxFixer {
    fn name(&self) -> &'static str {
        "no_alternative_syntax"
    }
    fn priority(&self) -> i32 {
        42
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["fix_non_monolithic_code"])?;
        // Future::getV4OrV3(false, true): v3 default.
        self.fix_non_monolithic_code = c.bool("fix_non_monolithic_code", true)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.has_alternative_syntax()
            && (self.fix_non_monolithic_code || tokens.is_monolithic_php())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (0..tokens.count()).rev() {
            let token = tokens[index].clone();
            fix_elseif(index, &token, tokens);
            fix_else(index, &token, tokens);
            fix_open_close_controls(index, &token, tokens);
        }
    }
}

/// Upstream returns the structure index when no `(` follows; only `elseif` calls this, and it always has one.
fn find_parenthesis_end(tokens: &Tokens, structure_token_index: usize) -> usize {
    let next_index = tokens
        .get_next_meaningful_token(structure_token_index)
        .unwrap();
    tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, next_index)
}

fn fix_open_close_controls(index: usize, token: &Token, tokens: &mut Tokens) {
    if token.is_given_kind(&[T_IF, T_FOREACH, T_WHILE, T_FOR, T_SWITCH, T_DECLARE]) {
        let open_index = tokens
            .get_next_token_of_kind(index, &["(".into()], true)
            .unwrap();
        let close_index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_index);
        let after_parenthesis_index = tokens.get_next_meaningful_token(close_index).unwrap();
        if !tokens[after_parenthesis_index].equals(":", true) {
            return;
        }
        let mut items = Vec::new();
        if !tokens[after_parenthesis_index - 1].is_whitespace(None) {
            items.push(Token::new(T_WHITESPACE, " "));
        }
        items.push(Token::char("{"));
        if !tokens[after_parenthesis_index + 1].is_whitespace(None) {
            items.push(Token::new(T_WHITESPACE, " "));
        }
        tokens.clear_at(after_parenthesis_index);
        tokens.insert_at(after_parenthesis_index, items);
    }
    if !token.is_given_kind(&[
        T_ENDIF,
        T_ENDFOREACH,
        T_ENDWHILE,
        T_ENDFOR,
        T_ENDSWITCH,
        T_ENDDECLARE,
    ]) {
        return;
    }
    let next_token_index = tokens.get_next_meaningful_token(index).unwrap();
    let next_is_semicolon = tokens[next_token_index].equals(";", true);
    tokens.set(index, Token::char("}"));
    if next_is_semicolon {
        tokens.clear_at(next_token_index);
    }
}

fn fix_else(index: usize, token: &Token, tokens: &mut Tokens) {
    if !token.is_kind(T_ELSE) {
        return;
    }
    let token_after_else_index = tokens.get_next_meaningful_token(index).unwrap();
    if !tokens[token_after_else_index].equals(":", true) {
        return;
    }
    add_braces(
        tokens,
        Token::new(T_ELSE, "else"),
        index,
        token_after_else_index,
    );
}

fn fix_elseif(index: usize, token: &Token, tokens: &mut Tokens) {
    if !token.is_kind(T_ELSEIF) {
        return;
    }
    let parenthesis_end_index = find_parenthesis_end(tokens, index);
    let token_after_parenthesis_index = tokens
        .get_next_meaningful_token(parenthesis_end_index)
        .unwrap();
    if !tokens[token_after_parenthesis_index].equals(":", true) {
        return;
    }
    add_braces(
        tokens,
        Token::new(T_ELSEIF, "elseif"),
        index,
        token_after_parenthesis_index,
    );
}

fn add_braces(tokens: &mut Tokens, token: Token, index: usize, mut colon_index: usize) {
    let mut items = vec![Token::char("}"), Token::new(T_WHITESPACE, " "), token];
    if !tokens[index + 1].is_whitespace(None) {
        items.push(Token::new(T_WHITESPACE, " "));
    }
    // increment the position of the colon by number of items inserted
    colon_index += items.len();
    tokens.clear_at(index);
    tokens.insert_at(index, items);
    let mut items = vec![Token::char("{")];
    if !tokens[colon_index + 1].is_whitespace(None) {
        items.push(Token::new(T_WHITESPACE, " "));
    }
    tokens.clear_at(colon_index);
    tokens.insert_at(colon_index, items);
}
