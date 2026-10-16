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
            if tokens[i].is_whitespace(None) && self.may_change(tokens, i) {
                self.fix_whitespace_token(tokens, i);
            }
        }
    }
}

impl NoWhitespaceInBlankLineFixer {
    /// Whether fix_whitespace_token can change the token, mirroring its line selection without allocating: only a
    /// "\r" (a "\r\n" is rejoined with "\n") or a selected line made only of spaces and tabs (`^\h+$`). Otherwise
    /// the lines rejoin to the same content, and setting an identical token is a no-op.
    fn may_change(&self, tokens: &Tokens, index: usize) -> bool {
        let c = &tokens[index].content;
        if self.whitespaces_config.get_line_ending() != "\n" || c.contains(&b'\r') {
            return true;
        }
        let line_count = c.iter().filter(|&&b| b == b'\n').count() + 1;
        let has_next = index + 1 < tokens.count();
        let prev = &tokens[index - 1];
        if !(line_count > 2 || !has_next || prev.is_kind(T_OPEN_TAG)) {
            return false;
        }
        let l_max = if has_next { line_count - 1 } else { line_count };
        let l_start = usize::from(!(prev.is_kind(T_OPEN_TAG) && prev.content.ends_with(b"\n")));
        c.split(|&b| b == b'\n').take(l_max).skip(l_start).any(|l| !l.is_empty() && l.iter().all(|b| matches!(b, b' ' | b'\t')))
    }

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
