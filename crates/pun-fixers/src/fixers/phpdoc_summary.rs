//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocSummaryFixer`.
use crate::analyzers::docblock::{DocBlock, ShortDescription};
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::php_compat::{rtrim, rtrim_chars, strtolower};
use pun_tokens::{Kind, Token, Tokens};
use std::path::Path;

struct PhpdocSummaryFixer {
    line_ending: Vec<u8>,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocSummaryFixer {
        line_ending: b"\n".to_vec(),
    })
}

impl Fixer for PhpdocSummaryFixer {
    fn name(&self) -> &'static str {
        "phpdoc_summary"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.line_ending = config.get_line_ending().as_bytes().to_vec();
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(Kind::T_DOC_COMMENT)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].is_kind(Kind::T_DOC_COMMENT) {
                continue;
            }
            let doc = DocBlock::new(tokens[index].get_content());
            let Some(end) = ShortDescription::new(&doc).get_end() else {
                continue;
            };
            let mut line = doc.get_line(end).unwrap().borrow_mut();
            let content = rtrim(line.get_content()).to_vec();
            // final line of Description is NOT properly formatted, and first line of Description,
            // if different than final line, does NOT indicate a list
            if !is_correctly_formatted(&content)
                && (end == 1
                    || (doc.is_multi_line()
                        && rtrim(doc.get_line(1).unwrap().borrow().get_content()).last()
                            != Some(&b':')))
            {
                line.set_content([&content[..], b".", &self.line_ending].concat());
                drop(line);
                tokens.set(index, Token::new(Kind::T_DOC_COMMENT, doc.get_content()));
            }
        }
    }
}

fn is_correctly_formatted(content: &[u8]) -> bool {
    if strtolower(content)
        .windows(13)
        .any(|w| w == b"{@inheritdoc}")
    {
        return true;
    }
    // rtrim's char list is bytes, so the multibyte marks count byte by byte, as upstream
    rtrim_chars(content, ".:。!?¡¿！？".as_bytes()).len() != content.len()
}
