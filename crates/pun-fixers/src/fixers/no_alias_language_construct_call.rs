//! Port of `PhpCsFixer\Fixer\Alias\NoAliasLanguageConstructCallFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct NoAliasLanguageConstructCallFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(NoAliasLanguageConstructCallFixer)
}

impl Fixer for NoAliasLanguageConstructCallFixer {
    fn name(&self) -> &'static str {
        "no_alias_language_construct_call"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_EXIT)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.len() {
            let token = &tokens[index];
            if !token.is_kind(T_EXIT) || token.content.eq_ignore_ascii_case(b"exit") {
                continue;
            }
            tokens.set(index, Token::new(T_EXIT, "exit"));
        }
    }
}
