//! Port of `PhpCsFixer\Tokenizer\Analyzer\WhitespacesAnalyzer`.
use pun_tokens::{Kind::*, Tokens};

pub struct WhitespacesAnalyzer;

impl WhitespacesAnalyzer {
    pub fn detect_indent(tokens: &Tokens, mut index: usize) -> Vec<u8> {
        let whitespace_index = loop {
            let Some(whitespace_index) = tokens.get_prev_token_of_kind(index, &[T_WHITESPACE.into()], true) else {
                return Vec::new();
            };
            if tokens[whitespace_index].get_content().contains(&b'\n') {
                break whitespace_index;
            }
            let prev_token = &tokens[whitespace_index - 1];
            if prev_token.is_given_kind(&[T_OPEN_TAG, T_COMMENT]) && prev_token.get_content().ends_with(b"\n") {
                break whitespace_index;
            }
            index = whitespace_index;
        };
        let content = tokens[whitespace_index].get_content();
        content[content.iter().rposition(|&b| b == b'\n').map_or(0, |p| p + 1)..].to_vec()
    }
}
