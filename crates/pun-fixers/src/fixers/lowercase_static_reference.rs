//! Port of `PhpCsFixer\Fixer\Casing\LowercaseStaticReferenceFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct LowercaseStaticReferenceFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(LowercaseStaticReferenceFixer)
}

impl Fixer for LowercaseStaticReferenceFixer {
    fn name(&self) -> &'static str {
        "lowercase_static_reference"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_STATIC, T_STRING])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.len() {
            let token = &tokens[index];
            if !token.equals_any(&[(T_STRING, "self").into(), (T_STATIC, "static").into(), (T_STRING, "parent").into()], false) {
                continue;
            }
            if !is_token_to_fix(tokens, index) {
                continue;
            }
            let token = &tokens[index];
            tokens.set(index, Token::new(token.id.unwrap(), token.content.to_ascii_lowercase()));
        }
    }
}

fn is_token_to_fix(tokens: &Tokens, index: usize) -> bool {
    if tokens[index].content == tokens[index].content.to_ascii_lowercase() {
        return false; // case is already correct
    }
    let next_index = tokens.get_next_meaningful_token(index).unwrap();
    if tokens[next_index].is_kind(T_DOUBLE_COLON) {
        return true;
    }
    if !tokens[next_index].is_given_kind(&[T_VARIABLE, CT_TYPE_ALTERNATION])
        && !tokens[next_index].equals_any(&["(".into(), ")".into(), "{".into(), ";".into()], true)
    {
        return false;
    }
    let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
    if tokens[prev_index].is_kind(T_INSTANCEOF) {
        return true;
    }
    if tokens[prev_index].is_kind(T_CASE) {
        return !tokens[next_index].equals(";", true);
    }
    if !tokens[prev_index].is_given_kind(&[T_NEW, T_PRIVATE, T_PROTECTED, T_PUBLIC, CT_NULLABLE_TYPE, CT_TYPE_COLON, CT_TYPE_ALTERNATION])
        && !tokens[prev_index].equals_any(&["(".into(), "{".into()], true)
    {
        return false;
    }
    if tokens[prev_index].equals("(", true) && tokens[next_index].equals(")", true) {
        return false;
    }
    if tokens[index].content.eq_ignore_ascii_case(b"static") && tokens[next_index].is_kind(T_VARIABLE) {
        return false;
    }
    true
}
