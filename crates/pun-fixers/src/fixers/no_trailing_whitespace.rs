//! Port of `PhpCsFixer\Fixer\Whitespace\NoTrailingWhitespaceFixer`.
use crate::analyzers::preg::{PREG_SPLIT_DELIM_CAPTURE, Preg};
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens, php_compat::rtrim_chars};
use std::path::Path;

struct NoTrailingWhitespaceFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoTrailingWhitespaceFixer)
}

impl Fixer for NoTrailingWhitespaceFixer {
    fn name(&self) -> &'static str {
        "no_trailing_whitespace"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, _tokens: &Tokens) -> bool {
        true
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = tokens.count();
        while index > 0 {
            index -= 1;
            if !tokens[index].is_kind(T_OPEN_TAG)
                && !(tokens[index].is_whitespace(None)
                    && may_change(&tokens[index].content, index + 1 >= tokens.count()))
            {
                continue;
            }
            let token = tokens[index].clone();
            if token.is_kind(T_OPEN_TAG)
                && index + 1 < tokens.count()
                && tokens[index + 1].is_whitespace(None)
            {
                if let Some(open_tag_matches) = Preg::match_groups("/(.*)\\h$/", &token.content, 0)
                {
                    let next = tokens[index + 1].content.clone();
                    if let Some(whitespace_matches) = Preg::match_groups("/^(\\R)(.*)$/s", &next, 0)
                    {
                        let content = [open_tag_matches.str(1), whitespace_matches.str(1)].concat();
                        let rest = whitespace_matches.str(2).to_vec();
                        tokens.set(index, Token::new(T_OPEN_TAG, content));
                        tokens.ensure_whitespace_at_index(index + 1, 0, &rest);
                        continue;
                    }
                }
            }

            if !token.is_whitespace(None) {
                continue;
            }

            let mut lines: Vec<Vec<u8>> =
                Preg::split("/(\\R+)/", &token.content, -1, PREG_SPLIT_DELIM_CAPTURE)
                    .into_iter()
                    .map(|p| p.text)
                    .collect();
            let lines_size = lines.len();

            // fix only multiline whitespaces or singleline whitespaces at the end of file: may_change() lets no other through
            debug_assert!(lines_size > 1 || index + 1 >= tokens.count());
            if !tokens[index - 1].is_kind(T_OPEN_TAG)
                || !Preg::r#match("/(.*)\\R$/", &tokens[index - 1].content)
            {
                lines[0] = rtrim_chars(&lines[0], b" \t").to_vec();
            }
            for line in lines.iter_mut().skip(1) {
                let trimmed_line = rtrim_chars(line, b" \t");
                if !trimmed_line.is_empty() {
                    *line = trimmed_line.to_vec();
                }
            }
            let content = lines.concat();
            if !content.is_empty() {
                tokens.set(
                    index,
                    Token {
                        id: token.id,
                        content,
                    },
                );
            } else {
                tokens.clear_at(index);
            }
        }
    }
}

/// Whether trimming can change a whitespace token (only " \t\n\r\0\x0B" in it): a space or tab right before a line
/// break, or at the very end of the file (where an empty token takes the full path too). `\0` is never trimmed.
fn may_change(c: &[u8], last: bool) -> bool {
    let h = |b: &u8| matches!(b, b' ' | b'\t');
    (last && (c.is_empty() || c.last().is_some_and(h)))
        || c.windows(2)
            .any(|w| h(&w[0]) && matches!(w[1], b'\n' | b'\r' | 0x0b))
}
