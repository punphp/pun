//! Port of `PhpCsFixer\Fixer\Whitespace\NoWhitespaceInBlankLineFixer`.
use crate::analyzers::preg::Preg;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind::*, Tokens};
use std::path::Path;

#[derive(Default)]
struct NoWhitespaceInBlankLineFixer {
    whitespaces_config: WhitespacesFixerConfig,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoWhitespaceInBlankLineFixer::default())
}

impl Fixer for NoWhitespaceInBlankLineFixer {
    fn name(&self) -> &'static str {
        "no_whitespace_in_blank_line"
    }
    fn priority(&self) -> i32 {
        -99
    }
    fn is_candidate(&self, _tokens: &Tokens) -> bool {
        true
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        // skip first as it cannot be a white space token
        let count = tokens.count();
        for i in 1..count {
            if tokens[i].is_whitespace(None) {
                self.fix_whitespace_token(tokens, i);
            }
        }
    }
}

impl NoWhitespaceInBlankLineFixer {
    fn fix_whitespace_token(&self, tokens: &mut Tokens, index: usize) {
        let content = tokens[index].content.clone();
        let mut lines = Preg::split_str("/(\r\n|\n)/", &content);
        let line_count = lines.len();
        let has_next = index + 1 < tokens.count();

        // fix T_WHITESPACES with at least 3 lines (eg `\n   \n`)
        // and T_WHITESPACES with at least 2 lines at the end of file or after open tag with linebreak
        if line_count > 2 || (line_count > 0 && (!has_next || tokens[index - 1].is_kind(T_OPEN_TAG))) {
            let l_max = if has_next { line_count - 1 } else { line_count };
            let mut l_start = 1;
            if tokens[index - 1].is_kind(T_OPEN_TAG) && tokens[index - 1].content.ends_with(b"\n") {
                l_start = 0;
            }
            for line in lines.iter_mut().take(l_max).skip(l_start) {
                *line = Preg::replace("/^\\h+$/", "", &line);
            }
            let content = lines.join(self.whitespaces_config.get_line_ending().as_bytes());
            tokens.ensure_whitespace_at_index(index, 0, &content);
        }
    }
}
