//! Port of `PhpCsFixer\Fixer\StringNotation\StringLineEndingFixer`.
use crate::analyzers::preg::Preg;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

#[derive(Default)]
struct StringLineEndingFixer {
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(StringLineEndingFixer::default())
}

const KINDS: &[pun_tokens::Kind] = &[
    T_CONSTANT_ENCAPSED_STRING,
    T_ENCAPSED_AND_WHITESPACE,
    T_INLINE_HTML,
];

impl Fixer for StringLineEndingFixer {
    fn name(&self) -> &'static str {
        "string_line_ending"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(KINDS)
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let ending = self.whitespaces_config.get_line_ending();
        for index in 0..tokens.count() {
            let token = &tokens[index];
            if !token.is_given_kind(KINDS) {
                continue;
            }
            let content = Preg::replace("#\\R#u", ending, token.get_content());
            tokens.set(index, Token::with_id(token.get_id(), content));
        }
    }
}
