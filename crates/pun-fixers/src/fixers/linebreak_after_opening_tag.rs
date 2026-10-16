//! Port of `PhpCsFixer\Fixer\PhpTag\LinebreakAfterOpeningTagFixer`.
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind::*, Token, Tokens, php_compat::rtrim};
use std::path::Path;

#[derive(Default)]
struct LinebreakAfterOpeningTagFixer {
    whitespaces_config: WhitespacesFixerConfig,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(LinebreakAfterOpeningTagFixer::default())
}

impl Fixer for LinebreakAfterOpeningTagFixer {
    fn name(&self) -> &'static str {
        "linebreak_after_opening_tag"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_monolithic_php() && !tokens.is_token_kind_found(T_OPEN_TAG_WITH_ECHO)
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let open_tag_index = if tokens[0].is_kind(T_INLINE_HTML) { 1 } else { 0 };
        // ignore if linebreak already present
        if tokens[open_tag_index].get_content().contains(&b'\n') {
            return;
        }
        let newline_found = tokens.iter().any(|t| (t.is_whitespace(None) || t.is_kind(T_OPEN_TAG)) && t.get_content().contains(&b'\n'));
        // ignore one-line files
        if !newline_found {
            return;
        }
        let content = [rtrim(tokens[open_tag_index].get_content()), self.whitespaces_config.get_line_ending().as_bytes()].concat();
        tokens.set(open_tag_index, Token::new(T_OPEN_TAG, content));
    }
}
