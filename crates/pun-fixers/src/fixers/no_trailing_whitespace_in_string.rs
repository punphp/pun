//! Port of `PhpCsFixer\Fixer\StringNotation\NoTrailingWhitespaceInStringFixer`.
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct NoTrailingWhitespaceInStringFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(NoTrailingWhitespaceInStringFixer)
}

const KINDS: &[pun_tokens::Kind] = &[
    T_CONSTANT_ENCAPSED_STRING,
    T_ENCAPSED_AND_WHITESPACE,
    T_INLINE_HTML,
];

impl Fixer for NoTrailingWhitespaceInStringFixer {
    fn name(&self) -> &'static str {
        "no_trailing_whitespace_in_string"
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
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut last = true;
        for index in (0..tokens.count()).rev() {
            let token = &tokens[index];
            let is_last = last;
            last = false;
            if !token.is_given_kind(KINDS) {
                continue;
            }
            let is_inline_html = token.is_kind(T_INLINE_HTML);
            let regex = if is_inline_html && is_last {
                r"/\h+(?=\R|$)/"
            } else {
                r"/\h+(?=\R)/"
            };
            let mut content = Preg::replace(regex, "", token.get_content());
            if token.get_content() == content {
                continue;
            }
            if !is_inline_html || index == 0 {
                update_content(tokens, index, content);
                continue;
            }
            let prev = index - 1;
            if tokens[prev].is_kind(T_CLOSE_TAG)
                && let Some(m) = Preg::match_groups(r"/^\R/", &content)
            {
                let line_break = m.str(0).to_vec();
                let mut close = tokens[prev].get_content().to_vec();
                close.extend_from_slice(&line_break);
                tokens.set(prev, Token::new(T_CLOSE_TAG, close));
                content = content[line_break.len()..].to_vec();
            }
            update_content(tokens, index, content);
        }
    }
}

fn update_content(tokens: &mut Tokens, index: usize, content: Vec<u8>) {
    if content.is_empty() {
        tokens.clear_at(index);
        return;
    }
    let id = tokens[index].get_id();
    tokens.set(index, Token::with_id(id, content));
}
