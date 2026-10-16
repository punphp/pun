//! Port of `PhpCsFixer\Fixer\Whitespace\IndentationTypeFixer`.
use crate::analyzers::preg::Preg;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind::*, Token, Tokens};
use std::path::Path;

#[derive(Default)]
struct IndentationTypeFixer {
    whitespaces_config: WhitespacesFixerConfig,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(IndentationTypeFixer::default())
}

/// `str_replace($search, $replace, $s)`.
fn str_replace(search: &[u8], replace: &[u8], s: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(s.len());
    let mut i = 0;
    while i < s.len() {
        if s[i..].starts_with(search) {
            out.extend_from_slice(replace);
            i += search.len();
        } else {
            out.push(s[i]);
            i += 1;
        }
    }
    out
}

impl IndentationTypeFixer {
    fn fix_indent_in_comment(&self, tokens: &Tokens, index: usize) -> Token {
        let (mut content, mut count) = Preg::replace_limit("/^(?:(?<! ) {1,3})?\\t/m", "\\1    ", tokens[index].get_content(), -1);
        // Also check for more tabs.
        while count != 0 {
            (content, count) = Preg::replace_limit("/^(\\ +)?\\t/m", "\\1    ", &content, -1);
        }
        let indent = self.whitespaces_config.get_indent();
        // change indent to expected one
        let content = Preg::replace_callback("/^(?:    )+/m", |m| Self::get_expected_indent(m.str(0), indent), &content);
        Token { id: tokens[index].get_id(), content }
    }

    fn fix_indent_token(&self, tokens: &Tokens, index: usize) -> Token {
        let mut content = tokens[index].get_content().to_vec();
        let mut previous_token_has_trailing_linebreak = false;
        // @TODO this can be removed when we have a transformer for "T_OPEN_TAG" to "T_OPEN_TAG + T_WHITESPACE"
        if index > 0 && tokens[index - 1].get_content().contains(&b'\n') {
            content.insert(0, b'\n');
            previous_token_has_trailing_linebreak = true;
        }
        let indent = self.whitespaces_config.get_indent();
        let mut new_content = Preg::replace_callback(
            "/(\\R)(\\h+)/", // find indent
            |m| {
                // normalize mixed indent
                let content = Preg::replace("/(?:(?<! ) {1,3})?\\t/", "    ", m.str(2));
                // change indent to expected one
                [m.str(1), &Self::get_expected_indent(&content, indent)].concat()
            },
            &content,
        );
        if previous_token_has_trailing_linebreak {
            new_content.remove(0);
        }
        Token::new(T_WHITESPACE, new_content)
    }

    fn get_expected_indent(content: &[u8], indent: &str) -> Vec<u8> {
        if indent == "\t" {
            return str_replace(b"    ", indent.as_bytes(), content);
        }
        content.to_vec()
    }
}

impl Fixer for IndentationTypeFixer {
    fn name(&self) -> &'static str {
        "indentation_type"
    }
    fn priority(&self) -> i32 {
        // Must run before PhpdocIndentFixer.
        // Must run after ClassAttributesSeparationFixer.
        50
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_COMMENT, T_DOC_COMMENT, T_WHITESPACE])
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            let token = &tokens[index];
            if token.is_comment() {
                let t = self.fix_indent_in_comment(tokens, index);
                tokens.set(index, t);
                continue;
            }
            if token.is_whitespace(None) {
                let t = self.fix_indent_token(tokens, index);
                tokens.set(index, t);
            }
        }
    }
}
