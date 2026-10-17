//! Port of `PhpCsFixer\Tokenizer\Analyzer\GotoLabelAnalyzer`.
use pun_tokens::{Kind::*, Tokens};

pub struct GotoLabelAnalyzer;

impl GotoLabelAnalyzer {
    pub fn belongs_to_go_to_label(&self, tokens: &Tokens, index: usize) -> bool {
        if !tokens[index].equals(":", true) {
            return false;
        }
        let prev = match tokens.get_prev_meaningful_token(index) {
            Some(prev) if tokens[prev].is_kind(T_STRING) => prev,
            _ => return false,
        };
        tokens.get_prev_meaningful_token(prev).is_some_and(|prev| {
            tokens[prev].equals_any(
                &[
                    ":".into(),
                    ";".into(),
                    "{".into(),
                    "}".into(),
                    T_OPEN_TAG.into(),
                ],
                true,
            )
        })
    }
}
