//! Port of `PhpCsFixer\Fixer\Whitespace\SingleBlankLineAtEofFixer`.
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind::*, Tokens};
use std::path::Path;

#[derive(Default)]
struct SingleBlankLineAtEofFixer {
    whitespaces_config: WhitespacesFixerConfig,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(SingleBlankLineAtEofFixer::default())
}

impl Fixer for SingleBlankLineAtEofFixer {
    fn name(&self) -> &'static str {
        "single_blank_line_at_eof"
    }
    fn priority(&self) -> i32 {
        // must run last to be sure the file is properly formatted before it runs
        -100
    }
    fn is_candidate(&self, _tokens: &Tokens) -> bool {
        true
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let count = tokens.count();
        if count > 0 && !tokens[count - 1].is_given_kind(&[T_INLINE_HTML, T_CLOSE_TAG, T_OPEN_TAG]) {
            let le = self.whitespaces_config.get_line_ending().as_bytes().to_vec();
            tokens.ensure_whitespace_at_index(count - 1, 1, &le);
        }
    }
}
