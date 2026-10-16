//! Port of `PhpCsFixer\Fixer\PhpTag\BlankLineAfterOpeningTagFixer`.
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind::*, Token, Tokens, php_compat::rtrim};
use std::path::Path;

#[derive(Default)]
struct BlankLineAfterOpeningTagFixer {
    whitespaces_config: WhitespacesFixerConfig,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(BlankLineAfterOpeningTagFixer::default())
}

impl Fixer for BlankLineAfterOpeningTagFixer {
    fn name(&self) -> &'static str {
        "blank_line_after_opening_tag"
    }
    fn priority(&self) -> i32 {
        // Must run before BlankLinesBeforeNamespaceFixer, NoBlankLinesBeforeNamespaceFixer.
        // Must run after DeclareStrictTypesFixer.
        1
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_monolithic_php() && !tokens.is_token_kind_found(T_OPEN_TAG_WITH_ECHO)
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let line_ending = self.whitespaces_config.get_line_ending().as_bytes();
        let newline_found = tokens.iter().any(|t| (t.is_whitespace(None) || t.is_kind(T_OPEN_TAG)) && t.get_content().contains(&b'\n'));
        // ignore one-line files
        if !newline_found {
            return;
        }
        let open_tag_index = if tokens[0].is_kind(T_INLINE_HTML) { 1 } else { 0 };
        let token = &tokens[open_tag_index];
        if !token.get_content().contains(&b'\n') {
            let t = Token { id: token.get_id(), content: [rtrim(token.get_content()), line_ending].concat() };
            tokens.set(open_tag_index, t);
        }
        let new_line_index = open_tag_index + 1;
        if new_line_index >= tokens.count() {
            return;
        }
        if tokens[new_line_index].is_whitespace(None) {
            if !tokens[new_line_index].get_content().contains(&b'\n') {
                let t = Token::new(T_WHITESPACE, [line_ending, tokens[new_line_index].get_content()].concat());
                tokens.set(new_line_index, t);
            }
        } else {
            tokens.insert_at(new_line_index, vec![Token::new(T_WHITESPACE, line_ending)]);
        }
    }
}
