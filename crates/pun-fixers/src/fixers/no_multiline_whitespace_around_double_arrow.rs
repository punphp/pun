//! Port of `PhpCsFixer\Fixer\ArrayNotation\NoMultilineWhitespaceAroundDoubleArrowFixer`.
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens, php_compat::rtrim};
use std::path::Path;

struct NoMultilineWhitespaceAroundDoubleArrowFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoMultilineWhitespaceAroundDoubleArrowFixer)
}

fn fix_whitespace(tokens: &mut Tokens, index: usize) {
    let token = &tokens[index];
    if token.is_whitespace(None) && !token.is_whitespace(Some(b" \t")) {
        let t = Token::new(T_WHITESPACE, [rtrim(token.get_content()), b" "].concat());
        tokens.set(index, t);
    }
}

impl Fixer for NoMultilineWhitespaceAroundDoubleArrowFixer {
    fn name(&self) -> &'static str {
        "no_multiline_whitespace_around_double_arrow"
    }
    fn priority(&self) -> i32 {
        // Must run before BinaryOperatorSpacesFixer, MethodArgumentSpaceFixer.
        31
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOUBLE_ARROW)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].is_kind(T_DOUBLE_ARROW) {
                continue;
            }
            if !tokens[index - 2].is_comment() || tokens[index - 2].get_content().starts_with(b"/*") {
                fix_whitespace(tokens, index - 1);
            }
            // do not move anything about if there is a comment following the whitespace
            if !tokens[index + 2].is_comment() {
                fix_whitespace(tokens, index + 1);
            }
        }
    }
}
