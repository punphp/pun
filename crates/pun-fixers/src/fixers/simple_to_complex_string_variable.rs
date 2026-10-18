//! Port of `PhpCsFixer\Fixer\StringNotation\SimpleToComplexStringVariableFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct SimpleToComplexStringVariableFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(SimpleToComplexStringVariableFixer)
}

impl Fixer for SimpleToComplexStringVariableFixer {
    fn name(&self) -> &'static str {
        "simple_to_complex_string_variable"
    }
    fn priority(&self) -> i32 {
        // Must run after ExplicitStringVariableFixer.
        -10
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOLLAR_OPEN_CURLY_BRACES)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        // `${` sits inside a string, so at least `"` comes before it and `}"` after it.
        let mut index = tokens.count() - 3;
        while index > 0 {
            if tokens[index].is_kind(T_DOLLAR_OPEN_CURLY_BRACES)
                && tokens[index + 1].is_kind(T_STRING_VARNAME)
            {
                fix_variable(tokens, index);
            }
            index -= 1;
        }
    }
}

fn fix_variable(tokens: &mut Tokens, index: usize) {
    let varname = tokens[index + 1].content.clone();
    let dollar_close_token =
        tokens.find_block_end(Tokens::BLOCK_TYPE_COMPLEX_STRING_VARIABLE, index);
    let prev_token_content = &tokens[index - 1].content;
    if prev_token_content.ends_with(b"$") && !prev_token_content.ends_with(b"\\$") {
        let mut content = prev_token_content[..prev_token_content.len() - 1].to_vec();
        content.extend_from_slice(b"\\$");
        tokens.set(index - 1, Token::new(T_ENCAPSED_AND_WHITESPACE, content));
    }
    tokens.set(index, Token::new(T_CURLY_OPEN, "{"));
    tokens.set(index + 1, Token::new(T_VARIABLE, [b"$", &varname[..]].concat()));
    tokens.set(dollar_close_token, Token::new(CT_CURLY_CLOSE, "}"));
}
